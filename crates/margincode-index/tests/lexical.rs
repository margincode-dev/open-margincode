//! Lexical code search: goal words find code chunks without the semantic layer.

use margincode_core::SymbolId;
use margincode_index::{CodeIndex, PgIndex, ScopeView};
use margincode_store::PgStore;
use margincode_testutil::TestDb;

async fn setup() -> (TestDb, PgIndex) {
    let db = TestDb::new().await;
    let pool = PgStore::connect(&db.url).await.unwrap().pool().clone();
    let idx = PgIndex::new(pool);
    let files = vec![
        (
            "app/params.py",
            "class Parameter:\n    def human_readable_name(self):\n        return self.name\n",
        ),
        (
            "app/net.ts",
            "export function readUrl(name: string) {\n  return fetch(name);\n}\n",
        ),
        (
            "app/names.py",
            "def name_one(name):\n    return name\n\n\ndef name_two(name):\n    return name\n\n\ndef name_three(name):\n    return name\n",
        ),
    ];
    idx.index_paths(
        "demo",
        "main",
        files
            .into_iter()
            .map(|(p, s)| (p.to_owned(), s.as_bytes().to_vec()))
            .collect(),
    )
    .await
    .unwrap();
    (db, idx)
}

async fn qnames(idx: &PgIndex, hits: &[(SymbolId, f32)]) -> Vec<String> {
    let ids: Vec<SymbolId> = hits.iter().map(|(id, _)| *id).collect();
    let syms = idx.get_symbols(&ids).await.unwrap();
    ids.iter()
        .map(|id| {
            syms.iter()
                .find(|s| s.id == *id)
                .unwrap()
                .symbol
                .qname
                .clone()
        })
        .collect()
}

async fn paths(idx: &PgIndex, hits: &[(SymbolId, f32)]) -> Vec<String> {
    let mut out = vec![];
    for (id, _) in hits {
        out.push(
            idx.get_symbols(&[*id]).await.unwrap()[0]
                .symbol
                .path
                .clone(),
        );
    }
    out
}

fn terms(t: &[&str]) -> Vec<String> {
    t.iter().map(|s| (*s).to_owned()).collect()
}

#[tokio::test]
async fn snake_case_parts_find_their_symbol() {
    let (_db, idx) = setup().await;
    let hits = idx
        .search_code_lexical(&ScopeView::new("demo", "main"), &terms(&["readable"]), 10)
        .await
        .unwrap();
    assert_eq!(
        qnames(&idx, &hits).await[0],
        "Parameter.human_readable_name",
        "{hits:?}"
    );
}

#[tokio::test]
async fn camel_case_parts_and_whole_words_both_match() {
    let (_db, idx) = setup().await;
    for t in ["url", "readurl"] {
        let hits = idx
            .search_code_lexical(&ScopeView::new("demo", "main"), &terms(&[t]), 10)
            .await
            .unwrap();
        assert_eq!(qnames(&idx, &hits).await, vec!["readUrl"], "{t}");
    }
}

#[tokio::test]
async fn rare_terms_outweigh_common_ones() {
    let (_db, idx) = setup().await;
    // "name" is in almost every chunk; "human" in one.
    let hits = idx
        .search_code_lexical(
            &ScopeView::new("demo", "main"),
            &terms(&["name", "human"]),
            10,
        )
        .await
        .unwrap();
    let q = qnames(&idx, &hits).await;
    assert_eq!(q[0], "Parameter.human_readable_name", "{q:?}");
    assert!(hits.windows(2).all(|w| w[0].1 >= w[1].1), "best first");
    assert!(q.len() > 1, "common-term matches still come after: {q:?}");
}

#[tokio::test]
async fn no_terms_or_no_match_or_other_scope_is_empty() {
    let (_db, idx) = setup().await;
    for (scope, t) in [
        ("main", vec![]),
        ("main", terms(&["zebra"])),
        ("task-1", terms(&["human"])),
    ] {
        let hits = idx
            .search_code_lexical(&ScopeView::new("demo", scope), &t, 10)
            .await
            .unwrap();
        assert!(hits.is_empty(), "{scope} {t:?}: {hits:?}");
    }
    let hits = idx
        .search_code_lexical(&ScopeView::new("demo", "main"), &terms(&["name"]), 2)
        .await
        .unwrap();
    assert_eq!(hits.len(), 2, "limit");
}

async fn index(files: &[(&str, &str)]) -> (TestDb, PgIndex) {
    let db = TestDb::new().await;
    let pool = PgStore::connect(&db.url).await.unwrap().pool().clone();
    let idx = PgIndex::new(pool);
    idx.index_paths(
        "demo",
        "main",
        files
            .iter()
            .map(|(p, s)| ((*p).to_owned(), s.as_bytes().to_vec()))
            .collect(),
    )
    .await
    .unwrap();
    (db, idx)
}

/// Okapi BM25: a term in a short chunk counts for more than the same term in a long one.
#[tokio::test]
async fn short_chunks_beat_long_ones_with_the_same_term() {
    let filler: String = (0..60).map(|i| format!("    w{i} = other_{i}\n")).collect();
    let long = format!("def long_one():\n    retry = 1\n{filler}");
    let (_db, idx) = index(&[
        ("app/long.py", &long),
        ("app/short.py", "def short_one():\n    retry = 1\n"),
        ("app/other.py", "def unrelated():\n    pass\n"),
    ])
    .await;
    let hits = idx
        .search_code_lexical(&ScopeView::new("demo", "main"), &terms(&["retry"]), 10)
        .await
        .unwrap();
    assert_eq!(
        qnames(&idx, &hits).await,
        vec!["short_one", "long_one"],
        "{hits:?}"
    );
}

/// Term frequency saturates: repeating one query word does not beat matching two of them.
#[tokio::test]
async fn repeating_one_term_does_not_beat_matching_two() {
    let (_db, idx) = index(&[
        (
            "app/a.py",
            "def spam():\n    cache = cache + cache + cache + cache + cache + cache\n",
        ),
        ("app/b.py", "def both():\n    cache = expire\n"),
        ("app/c.py", "def expire_only():\n    expire = 1\n"),
        ("app/d.py", "def cache_only():\n    cache = 1\n"),
    ])
    .await;
    let hits = idx
        .search_code_lexical(
            &ScopeView::new("demo", "main"),
            &terms(&["cache", "expire"]),
            10,
        )
        .await
        .unwrap();
    assert_eq!(qnames(&idx, &hits).await[0], "both", "{hits:?}");
}

#[tokio::test]
async fn equal_scores_keep_symbol_order() {
    let (_db, idx) = index(&[
        ("app/a.py", "def first():\n    token = 1\n"),
        ("app/b.py", "def secnd():\n    token = 1\n"),
    ])
    .await;
    for _ in 0..3 {
        let hits = idx
            .search_code_lexical(&ScopeView::new("demo", "main"), &terms(&["token"]), 10)
            .await
            .unwrap();
        assert_eq!(hits.len(), 2);
        assert!(
            hits[0].0.0 < hits[1].0.0 || hits[0].1 > hits[1].1,
            "{hits:?}"
        );
    }
}

/// A float sum depends on its order, and Postgres hands an aggregate its rows in no fixed order
/// (on a large repo a sort-based `GROUP BY` mixes each chunk's terms): two copies of one function
/// must still tie, and go by path. The filler makes the planner search term by term, as there.
#[tokio::test]
async fn copies_of_one_function_tie_whatever_order_the_rows_come_in() {
    let words = [
        "query", "event", "cookies", "encode", "headers", "parse", "result", "body",
    ];
    // each word a different number of times, and in a different number of files, so that every
    // term adds a different amount
    let body: String = words
        .iter()
        .enumerate()
        .map(|(i, w)| format!("    {w}_{i} = [{}]\n", vec![*w; i + 1].join(", ")))
        .collect();
    let body = format!("def handle():\n{body}");
    let mut files = vec![("src/lambda/handler.py".to_owned(), body.clone())];
    for (i, w) in words.iter().enumerate() {
        for k in 0..i {
            files.push((
                format!("src/f{i}_{k}.py"),
                format!("def f{i}_{k}():\n    return {w}\n"),
            ));
        }
    }
    for f in 0..40 {
        let defs: String = (0..50)
            .map(|j| {
                format!(
                    "def filler_{j}():\n    return {}\n\n\n",
                    words[(f + j) % words.len()]
                )
            })
            .collect();
        files.push((format!("lib/filler_{f}.py"), defs));
    }
    files.push(("adapters/lambda/handler.py".to_owned(), body));
    let files: Vec<(&str, &str)> = files
        .iter()
        .map(|(p, s)| (p.as_str(), s.as_str()))
        .collect();
    let (db, idx) = index(&files).await;
    let store = PgStore::connect(&db.url).await.unwrap();
    sqlx::query("ANALYZE").execute(store.pool()).await.unwrap();
    for _ in 0..3 {
        let hits = idx
            .search_code_lexical(&ScopeView::new("demo", "main"), &terms(&words), 10)
            .await
            .unwrap();
        assert_eq!(
            paths(&idx, &hits[..2]).await,
            ["adapters/lambda/handler.py", "src/lambda/handler.py"],
            "{hits:?}"
        );
        assert_eq!(hits[0].1, hits[1].1, "{hits:?}");
    }
}

/// Unlike `ts_rank`, BM25 counts how often a term occurs (up to saturation): a function that is
/// about the term beats a tiny one that mentions it once.
#[tokio::test]
async fn using_a_term_often_beats_mentioning_it_once() {
    let body: String = (0..8)
        .map(|i| format!("    parent{i} = parent.parent(step_{i}, other_{i})\n"))
        .collect();
    let often = format!("def often():\n{body}");
    let (_db, idx) = index(&[
        ("app/once.py", "def once():\n    parent = 1\n"),
        ("app/often.py", &often),
        ("app/none.py", "def unrelated():\n    pass\n"),
    ])
    .await;
    let hits = idx
        .search_code_lexical(&ScopeView::new("demo", "main"), &terms(&["parent"]), 10)
        .await
        .unwrap();
    assert_eq!(qnames(&idx, &hits).await, vec!["often", "once"], "{hits:?}");
}
