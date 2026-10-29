//! Lexical code search (Plan 6): goal words find code chunks without the semantic layer.

use scholion_core::SymbolId;
use scholion_index::{CodeIndex, PgIndex};
use scholion_store::PgStore;
use scholion_testutil::TestDb;

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

fn terms(t: &[&str]) -> Vec<String> {
    t.iter().map(|s| (*s).to_owned()).collect()
}

#[tokio::test]
async fn snake_case_parts_find_their_symbol() {
    let (_db, idx) = setup().await;
    let hits = idx
        .search_code_lexical("demo", "main", &terms(&["readable"]), 10)
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
            .search_code_lexical("demo", "main", &terms(&[t]), 10)
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
        .search_code_lexical("demo", "main", &terms(&["name", "human"]), 10)
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
            .search_code_lexical("demo", scope, &t, 10)
            .await
            .unwrap();
        assert!(hits.is_empty(), "{scope} {t:?}: {hits:?}");
    }
    let hits = idx
        .search_code_lexical("demo", "main", &terms(&["name"]), 2)
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

/// Okapi BM25 (as E1's `bm25_symbols` baseline): a term in a short chunk counts for more than the
/// same term in a long one.
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
        .search_code_lexical("demo", "main", &terms(&["retry"]), 10)
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
        .search_code_lexical("demo", "main", &terms(&["cache", "expire"]), 10)
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
            .search_code_lexical("demo", "main", &terms(&["token"]), 10)
            .await
            .unwrap();
        assert_eq!(hits.len(), 2);
        assert!(
            hits[0].0.0 < hits[1].0.0 || hits[0].1 > hits[1].1,
            "{hits:?}"
        );
    }
}

/// Unlike `ts_rank`, BM25 counts how often a term occurs (up to saturation): a function that is
/// about the term beats a tiny one that mentions it once (E1: ripgrep's `Ignore.add_parents`
/// ranked 43rd under `ts_rank`, behind one-line flags).
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
        .search_code_lexical("demo", "main", &terms(&["parent"]), 10)
        .await
        .unwrap();
    assert_eq!(qnames(&idx, &hits).await, vec!["often", "once"], "{hits:?}");
}
