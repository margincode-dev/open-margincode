use scholion_pack::lexical::{goal_terms, words};

#[test]
fn short_camel_case_parts_are_kept() {
    let w = words("readUrl parseHTTPRequest getId");
    for want in ["readurl", "read", "url", "http", "request", "parse", "get", "id"] {
        assert!(w.contains(&want.to_owned()), "{want} missing from {w:?}");
    }
}

#[test]
fn goal_terms_find_the_url_part() {
    assert!(goal_terms("Fix readUrl").contains(&"url".to_owned()));
}
