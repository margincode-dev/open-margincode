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
