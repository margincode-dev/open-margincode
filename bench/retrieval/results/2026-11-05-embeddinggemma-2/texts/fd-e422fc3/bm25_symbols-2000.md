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
        "test2/Download (1).tar.gz",
    );

    // Combine with --case-sensitive (unmatched).
    te.assert_output(&["--fixed-strings", "--case-sensitive", "download (1)"], "");

    // Combine with --case-sensitive (matched).
    te.assert_output(
        &["--fixed-strings", "--case-sensitive", "Download (1)"],
        "test2/Download (1).tar.gz",
    );
}
```
- hir_has_uppercase_char (src/regex_helper.rs:15-37)
```rust
fn hir_has_uppercase_char(hir: &Hir) -> bool {
    use regex_syntax::hir::*;

    match hir.kind() {
        HirKind::Literal(Literal(bytes)) => match std::str::from_utf8(bytes) {
            Ok(s) => s.chars().any(|c| c.is_uppercase()),
            Err(_) => bytes.iter().any(|b| char::from(*b).is_uppercase()),
        },
        HirKind::Class(Class::Unicode(ranges)) => ranges
            .iter()
            .any(|r| r.start().is_uppercase() || r.end().is_uppercase()),
        HirKind::Class(Class::Bytes(ranges)) => ranges
            .iter()
            .any(|r| char::from(r.start()).is_uppercase() || char::from(r.end()).is_uppercase()),
        HirKind::Capture(Capture { sub, .. }) | HirKind::Repetition(Repetition { sub, .. }) => {
            hir_has_uppercase_char(sub)
        }
        HirKind::Concat(hirs) | HirKind::Alternation(hirs) => {
            hirs.iter().any(hir_has_uppercase_char)
        }
        _ => false,
    }
}
```
- test_exact_literal_nonsubstring (tests/tests.rs:2448-2481)
```rust
fn test_exact_literal_nonsubstring() {
    let dirs = &["test1", "test2"];
    let files = &[
        "test1/a.foo",
        "test1/aa.foo",
        "test1/a.food",
        "test1/ca.food",
        "test1/a_foo",
        "test2/Download (1).tar.gz",
    ];
    let te = TestEnv::new(dirs, files);

    // Literal search, dot is treated as character. Should match only the exact name,
    // not "aa.foo", "a.food", or "ca.food".
    te.assert_output(&["--exact", "a.foo"], "test1/a.foo");

    // Literal search, parens are treated as characters. Substring should not match.
    te.assert_output(&["--exact", "download (1)"], "");

    // Literal search, parens are treated as characters. Case-insensitive by default.
    te.assert_output(
        &["--exact", "download (1).tar.gz"],
        "test2/Download (1).tar.gz",
    );

    // Combine with --case-sensitive, should not match due to case mismatch.
    te.assert_output(&["--exact", "--case-sensitive", "download (1).tar.gz"], "");

    // Combine with --case-sensitive, exact match should match.
    te.assert_output(
        &["--exact", "--case-sensitive", "Download (1).tar.gz"],
        "test2/Download (1).tar.gz",
    );
}
```
- pattern_has_uppercase_char (src/regex_helper.rs:5-12)
```rust
pub fn pattern_has_uppercase_char(pattern: &str) -> bool {
    let mut parser = ParserBuilder::new().utf8(false).build();

    parser
        .parse(pattern)
        .map(|hir| hir_has_uppercase_char(&hir))
        .unwrap_or(false)
}
```
- test_opposing (tests/tests.rs:2913-2928)
```rust
fn test_opposing(flag: &str, opposing_flags: &[&str]) {
    let te = TestEnv::new(DEFAULT_DIRS, DEFAULT_FILES);

    let mut flags = vec![flag];
    flags.extend_from_slice(opposing_flags);
    let out_no_flags = te.assert_success_and_get_normalized_output(".", &[]);
    let out_opposing_flags = te.assert_success_and_get_normalized_output(".", &flags);

    assert_eq!(
        out_no_flags,
        out_opposing_flags,
        "{} should override {}",
        opposing_flags.join(" "),
        flag
    );
}
```
- test_case_insensitive (tests/tests.rs:556-570)
```rust
fn test_case_insensitive() {
    let te = TestEnv::new(DEFAULT_DIRS, DEFAULT_FILES);

    te.assert_output(
        &["--ignore-case", "C.Foo"],
        "one/two/c.foo
        one/two/C.Foo2",
    );

    te.assert_output(
        &["--case-sensitive", "--ignore-case", "C.Foo"],
        "one/two/c.foo
        one/two/C.Foo2",
    );
}
```
- construct_config (src/main.rs:253-374)
```rust
fn construct_config(mut opts: Opts, pattern_regexps: &[String]) -> Result<Config> {
    // The search will be case-sensitive if the command line flag is set or
    // if any of the patterns has an uppercase character (smart case).
    let case_sensitive = !opts.ignore_case
        && (opts.case_sensitive
            || pattern_regexps
                .iter()
                .any(|pat| pattern_has_uppercase_char(pat)));

    let path_separator = opts
        .path_separator
        .take()
        .or_else(filesystem::default_path_separator);
    let actual_path_separator = path_separator
        .clone()
