use scholion_core::{Anchor, SymbolId};
use scholion_index::{AnchorResolution, CodeIndex, PgIndex};
use scholion_store::PgStore;
use scholion_testutil::TestDb;
use sqlx::PgPool;

async fn setup() -> (TestDb, PgIndex, PgPool) {
    let db = TestDb::new().await;
    let pool = PgStore::connect(&db.url).await.unwrap().pool().clone();
    (db, PgIndex::new(pool.clone()), pool)
}

fn f(path: &str, src: &str) -> (String, Vec<u8>) {
    (path.to_owned(), src.as_bytes().to_vec())
}

async fn sym(idx: &PgIndex, scope: &str, path: &str, qname: &str) -> SymbolId {
    let syms = idx.symbols_in("demo", scope, path).await.unwrap();
    syms.iter()
        .find(|s| s.symbol.qname == qname)
        .unwrap_or_else(|| panic!("{path}:{qname} not indexed"))
        .id
}

fn sorted(mut v: Vec<(SymbolId, u8)>) -> Vec<(i64, u8)> {
    v.sort_by_key(|(id, h)| (*h, id.0));
    v.into_iter().map(|(id, h)| (id.0, h)).collect()
}

#[tokio::test]
async fn unchanged_content_is_not_reparsed() {
    let (_db, idx, _) = setup().await;
    let files = vec![f("a.py", "def a():\n    pass\n")];
    let first = idx
        .index_paths("demo", "main", files.clone())
        .await
        .unwrap();
    assert_eq!((first.parsed, first.unchanged), (1, 0));
    let second = idx
        .index_paths("demo", "main", files.clone())
        .await
        .unwrap();
    assert_eq!((second.parsed, second.cached, second.unchanged), (0, 0, 1));
    // same bytes in another scope: parse cache hit, no new parse
    let other = idx.index_paths("demo", "task-1", files).await.unwrap();
    assert_eq!((other.parsed, other.cached), (0, 1));
}

#[tokio::test]
async fn neighbors_two_hops_both_directions() {
    let (_db, idx, _) = setup().await;
    idx.index_paths(
        "demo",
        "main",
        vec![
            f("a.py", "def a():\n    b()\n"),
            f("b.py", "def b():\n    c()\n"),
            f("c.py", "def c():\n    pass\n"),
        ],
    )
    .await
    .unwrap();
    let (a, b, c) = (
        sym(&idx, "main", "a.py", "a").await,
        sym(&idx, "main", "b.py", "b").await,
        sym(&idx, "main", "c.py", "c").await,
    );
    assert_eq!(
        sorted(idx.neighbors("demo", "main", &[b], 1).await.unwrap()),
        vec![(a.0.min(c.0), 1), (a.0.max(c.0), 1)]
    );
    assert_eq!(
        sorted(idx.neighbors("demo", "main", &[a], 2).await.unwrap()),
        vec![(b.0, 1), (c.0, 2)]
    );
    assert_eq!(
        sorted(idx.neighbors("demo", "main", &[a], 1).await.unwrap()),
        vec![(b.0, 1)]
    );
    assert!(
        idx.neighbors("demo", "main", &[], 2)
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn ambiguous_name_links_all_candidates_with_split_weight() {
    let (_db, idx, pool) = setup().await;
    idx.index_paths(
        "demo",
        "main",
        vec![
            f("x.py", "def run():\n    go()\n"),
            f("p.py", "def go():\n    pass\n"),
            f("q.py", "def go():\n    pass\n"),
        ],
    )
    .await
    .unwrap();
    let run = sym(&idx, "main", "x.py", "run").await;
    let weights: Vec<(f32,)> =
        sqlx::query_as("SELECT weight FROM symbol_refs WHERE src = $1 ORDER BY dst")
            .bind(run.0)
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(weights, vec![(0.5,), (0.5,)]);
}

#[tokio::test]
async fn edges_follow_cross_file_changes() {
    let (_db, idx, pool) = setup().await;
    idx.index_paths("demo", "main", vec![f("a.py", "def a():\n    b()\n")])
        .await
        .unwrap();
    let a = sym(&idx, "main", "a.py", "a").await;
    let edge_count = |pool: PgPool| async move {
        sqlx::query_as::<_, (i64,)>("SELECT count(*) FROM symbol_refs")
            .fetch_one(&pool)
            .await
            .unwrap()
            .0
    };
    assert_eq!(edge_count(pool.clone()).await, 0);
    idx.index_paths("demo", "main", vec![f("b.py", "def b():\n    pass\n")])
        .await
        .unwrap();
    let b = sym(&idx, "main", "b.py", "b").await;
    assert_eq!(
        sorted(idx.neighbors("demo", "main", &[a], 1).await.unwrap()),
        vec![(b.0, 1)]
    );
    idx.remove_paths("demo", "main", &["b.py".into()])
        .await
        .unwrap();
    assert_eq!(edge_count(pool).await, 0);
    assert!(
        idx.symbols_in("demo", "main", "b.py")
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn resolve_returns_stale_when_symbol_removed_and_path_gone() {
    let (_db, idx, _) = setup().await;
    idx.index_paths("demo", "main", vec![f("a.py", "def f():\n    pass\n")])
        .await
        .unwrap();
    let anchor = Anchor {
        path: "a.py".into(),
        symbol: Some("f".into()),
    };
    assert!(matches!(
        idx.resolve("demo", "main", &anchor).await.unwrap(),
        AnchorResolution::Symbol(_)
    ));
    idx.remove_paths("demo", "main", &["a.py".into()])
        .await
        .unwrap();
    assert_eq!(
        idx.resolve("demo", "main", &anchor).await.unwrap(),
        AnchorResolution::Stale
    );
}

#[tokio::test]
async fn resolve_returns_pathonly_when_symbol_renamed_but_path_exists() {
    let (_db, idx, _) = setup().await;
    idx.index_paths("demo", "main", vec![f("a.py", "def old():\n    pass\n")])
        .await
        .unwrap();
    idx.index_paths("demo", "main", vec![f("a.py", "def new():\n    pass\n")])
        .await
        .unwrap();
    let old = Anchor {
        path: "a.py".into(),
        symbol: Some("old".into()),
    };
    let new = Anchor {
        path: "a.py".into(),
        symbol: Some("new".into()),
    };
    let file = Anchor {
        path: "a.py".into(),
        symbol: None,
    };
    assert_eq!(
        idx.resolve("demo", "main", &old).await.unwrap(),
        AnchorResolution::PathOnly("a.py".into())
    );
    assert!(matches!(
        idx.resolve("demo", "main", &new).await.unwrap(),
        AnchorResolution::Symbol(_)
    ));
    assert_eq!(
        idx.resolve("demo", "main", &file).await.unwrap(),
        AnchorResolution::PathOnly("a.py".into())
    );
}

#[tokio::test]
async fn scopes_are_isolated() {
    let (_db, idx, _) = setup().await;
    idx.index_paths("demo", "main", vec![f("a.py", "def f():\n    pass\n")])
        .await
        .unwrap();
    idx.index_paths("demo", "task-1", vec![f("a.py", "def g():\n    pass\n")])
        .await
        .unwrap();
    let names = |s: Vec<scholion_index::IndexedSymbol>| {
        s.into_iter().map(|x| x.symbol.qname).collect::<Vec<_>>()
    };
    assert_eq!(
        names(idx.symbols_in("demo", "main", "a.py").await.unwrap()),
        vec!["f"]
    );
    assert_eq!(
        names(idx.symbols_in("demo", "task-1", "a.py").await.unwrap()),
        vec!["g"]
    );
}

#[tokio::test]
async fn ignored_dirs_never_indexed() {
    let (_db, idx, _) = setup().await;
    let stats = idx
        .index_paths(
            "demo",
            "main",
            vec![
                f("node_modules/x.py", "def x():\n    pass\n"),
                f("app/.venv/lib/y.py", "def y():\n    pass\n"),
                f("target/z.rs", "fn z() {}\n"),
                f("pkg/ok.py", "def ok():\n    pass\n"),
            ],
        )
        .await
        .unwrap();
    assert_eq!((stats.parsed, stats.skipped), (1, 3));
    assert!(
        idx.symbols_in("demo", "main", "node_modules/x.py")
            .await
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        idx.symbols_in("demo", "main", "pkg/ok.py")
            .await
            .unwrap()
            .len(),
        1
    );
}

#[tokio::test]
async fn unparseable_file_does_not_abort_the_batch() {
    let (_db, idx, _) = setup().await;
    let junk = ")))))) (((((( ===== class class def def ;;;;\n".repeat(20);
    let stats = idx
        .index_paths(
            "demo",
            "main",
            vec![
                f("junk.py", &junk),
                f("blob.py", "\u{0}\u{0}"),
                f("ok.py", "def ok():\n    pass\n"),
            ],
        )
        .await
        .unwrap();
    assert_eq!(
        idx.symbols_in("demo", "main", "ok.py").await.unwrap().len(),
        1
    );
    assert_eq!(stats.failed, 0);
}

#[tokio::test]
async fn get_symbols_returns_requested_ids_only() {
    let (_db, idx, _) = setup().await;
    idx.index_paths(
        "demo",
        "main",
        vec![f("a.py", "def a():\n    pass\n\ndef b():\n    pass\n")],
    )
    .await
    .unwrap();
    let a = sym(&idx, "main", "a.py", "a").await;
    let got = idx.get_symbols(&[a]).await.unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(
        (
            got[0].id,
            got[0].symbol.qname.as_str(),
            got[0].symbol.path.as_str()
        ),
        (a, "a", "a.py")
    );
    assert!(idx.get_symbols(&[]).await.unwrap().is_empty());
    assert!(idx.get_symbols(&[SymbolId(-5)]).await.unwrap().is_empty());
}

#[tokio::test]
async fn search_symbols_matches_qname_and_path_substring_case_insensitively() {
    let (_db, idx, _) = setup().await;
    idx.index_paths(
        "demo",
        "main",
        vec![
            f("auth/login.py", "def verify_password(p):\n    pass\n"),
            f(
                "util.py",
                "class PasswordHasher:\n    def hash(self):\n        pass\n",
            ),
            f("misc.py", "def zebra():\n    pass\n"),
        ],
    )
    .await
    .unwrap();
    let names = |v: Vec<scholion_index::IndexedSymbol>| {
        v.into_iter().map(|s| s.symbol.qname).collect::<Vec<_>>()
    };
    assert_eq!(
        names(
            idx.search_symbols("demo", "main", "PASSWORD", 10)
                .await
                .unwrap()
        ),
        vec!["verify_password", "PasswordHasher", "PasswordHasher.hash"]
    );
    assert_eq!(
        names(
            idx.search_symbols("demo", "main", "auth/", 10)
                .await
                .unwrap()
        ),
        vec!["verify_password"]
    );
    assert_eq!(
        idx.search_symbols("demo", "main", "password", 1)
            .await
            .unwrap()
            .len(),
        1
    );
    assert!(
        idx.search_symbols("demo", "main", "100%_never", 10)
            .await
            .unwrap()
            .is_empty(),
        "wildcards are literal"
    );
    assert!(
        idx.search_symbols("demo", "other-scope", "password", 10)
            .await
            .unwrap()
            .is_empty()
    );
    assert!(
        idx.search_symbols("demo", "main", "", 10)
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn in_degree_counts_the_symbols_that_reference_each_one() {
    let (_db, idx, _) = setup().await;
    let mut files = vec![f(
        "core.py",
        "def core():\n    pass\n\ndef lonely():\n    pass\n",
    )];
    for n in 0..6 {
        files.push(f(
            &format!("user{n}.py"),
            &format!("def user{n}():\n    core()\n"),
        ));
    }
    idx.index_paths("demo", "main", files).await.unwrap();
    let core = sym(&idx, "main", "core.py", "core").await;
    let lonely = sym(&idx, "main", "core.py", "lonely").await;
    let deg = idx.in_degree(&[core, lonely]).await.unwrap();
    assert_eq!(deg.get(&core), Some(&6));
    assert_eq!(deg.get(&lonely).copied().unwrap_or(0), 0);
}
