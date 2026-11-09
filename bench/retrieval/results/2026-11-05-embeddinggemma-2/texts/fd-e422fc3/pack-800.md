## Task
# fd-e422fc3 · fd-e422fc3 · open

## Map
- repo map (60 files):
  .cargo/ (1 files)
  .github/ (7 files)
  .gitignore
  CHANGELOG.md
  CONTRIBUTING.md
  Cargo.lock
  Cargo.toml
  Cross.toml
  LICENSE-APACHE
  LICENSE-MIT
  Makefile
  README.md
  SECURITY.md
  contrib/ (4 files)
  doc/ (8 files)
  rustfmt.toml
  scripts/ (4 files)
  src/ (22 files)
  tests/ (2 files)

## Code
- sym:38653 hir_has_uppercase_char (src/regex_helper.rs:15-37)
  fn hir_has_uppercase_char(hir: &Hir) -> bool
- sym:38655 hir_matches_strings_with_leading_dot (src/regex_helper.rs:50-79)
  fn hir_matches_strings_with_leading_dot(hir: &Hir) -> bool
- sym:38643 build_regex (src/main.rs:522-536)
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
- sym:38652 pattern_has_uppercase_char (src/regex_helper.rs:5-12)
  pub fn pattern_has_uppercase_char(pattern: &str) -> bool
- sym:38654 pattern_matches_strings_with_leading_dot (src/regex_helper.rs:40-47)
  pub fn pattern_matches_strings_with_leading_dot(pattern: &str) -> bool
- sym:38768 test_smart_case (tests/tests.rs:517-537)
  fn test_smart_case()
- sym:38531 i32.from (src/exit_codes.rs:15-22)
  fn from(code: ExitCode) -> Self
- sym:38828 test_fixed_strings (tests/tests.rs:2412-2444)
  fn test_fixed_strings()
- sym:38829 test_exact_literal_nonsubstring (tests/tests.rs:2448-2481)
  fn test_exact_literal_nonsubstring()
- sym:38657 pattern_has_uppercase_char_advanced (src/regex_helper.rs:91-96)
  fn pattern_has_uppercase_char_advanced()
- sym:38770 test_case_insensitive (tests/tests.rs:556-570)
  fn test_case_insensitive()
- sym:38848 test_opposing (tests/tests.rs:2913-2928)
  fn test_opposing(flag: &str, opposing_flags: &[&str])
- sym:38773 test_smart_case_glob_searches (tests/tests.rs:626-636)

---
tokens: 682/800
