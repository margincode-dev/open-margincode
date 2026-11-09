- build_regex (src/main.rs:522-536)
```rust
fn build_regex(pattern_regex: String, config: &Config) -> Result<regex::bytes::Regex> {
    RegexBuilder::new(&pattern_regex)
        .case_insensitive(!config.case_sensitive)
        .dot_matches_new_line(true)
        .build()
        .map_err(|e| {
            anyhow!(
                "{}\n\nNote: You can search for literal substrings with '--fixed-strings' \
                 or literal strings with '--exact' options (instead of a regular expression). \
                 Alternatively, you can \
                 also use the '--glob' option to match on a glob pattern.",
                e
            )
        })
}
```
- hir_matches_strings_with_leading_dot (src/regex_helper.rs:50-79)
```rust
fn hir_matches_strings_with_leading_dot(hir: &Hir) -> bool {
    use regex_syntax::hir::*;

    // Note: this only really detects the simplest case where a regex starts with
    // "^\\.", i.e. a start text anchor and a literal dot character. There are a lot
    // of other patterns that ONLY match hidden files, e.g. ^(\\.foo|\\.bar) which are
    // not (yet) detected by this algorithm.
    match hir.kind() {
        HirKind::Concat(hirs) => {
            let mut hirs = hirs.iter();
            if let Some(hir) = hirs.next() {
                if hir.kind() != &HirKind::Look(Look::Start) {
                    return false;
                }
            } else {
                return false;
            }

            if let Some(hir) = hirs.next() {
                match hir.kind() {
                    HirKind::Literal(Literal(bytes)) => bytes.starts_with(b"."),
                    _ => false,
                }
            } else {
                false
            }
        }
        _ => false,
    }
}
```
- test_smart_case (tests/tests.rs:517-537)
```rust
fn test_smart_case() {
    let te = TestEnv::new(DEFAULT_DIRS, DEFAULT_FILES);

    te.assert_output(
        &["c.foo"],
        "one/two/c.foo
        one/two/C.Foo2",
    );

    te.assert_output(&["C.Foo"], "one/two/C.Foo2");

    te.assert_output(&["Foo"], "one/two/C.Foo2");

    // Only literal uppercase chars should trigger case sensitivity.
    te.assert_output(
        &["\\Ac"],
        "one/two/c.foo
        one/two/C.Foo2",
    );
    te.assert_output(&["\\AC"], "one/two/C.Foo2");
}
```
- test_fixed_strings (tests/tests.rs:2412-2444)
```rust
fn test_fixed_strings() {
    let dirs = &["test1", "test2"];
    let files = &["test1/a.foo", "test1/a_foo", "test2/Download (1).tar.gz"];
    let te = TestEnv::new(dirs, files);

    // Regex search, dot is treated as "any character"
    te.assert_output(
        &["a.foo"],
        "test1/a.foo
         test1/a_foo",
    );

    // Literal search, dot is treated as character
    te.assert_output(&["--fixed-strings", "a.foo"], "test1/a.foo");

    // Regex search, parens are treated as group
    te.assert_output(&["download (1)"], "");

    // Literal search, parens are treated as characters. Case-insensitive by default.
    te.assert_output(
        &["--fixed-strings", "download (1)"],
