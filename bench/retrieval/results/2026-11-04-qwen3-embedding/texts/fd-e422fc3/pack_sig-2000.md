## Task
# fd-e422fc3 · fd-e422fc3 · open
Goal: Fix smart-case detection with inline case-insensitive flags

Inspect literal characters in the regex AST rather than the case-folded HIR. Otherwise an inline (?i) flag introduces uppercase alternatives and incorrectly makes other --and patterns case-sensitive.

Add unit and CLI regressions, including scoped flags and explicit case-sensitivity options.

Assisted-by: OpenAI Codex (investigation, implementation, tests and validation)
Workspace: /root/bench/ws/rebf815-fd-e422fc3/fd-e422fc3
Changed (0): nothing yet

## Map
- repo map (60 files):
  .cargo/ (1 files)
    config.toml
  .github/ (7 files)
    FUNDING.yml
    ISSUE_TEMPLATE/ (4 files)
    dependabot.yml
    workflows/ (1 files)
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
    completion/ (4 files)
  doc/ (8 files)
    .gitattributes
    fd.1
    logo.png
    logo.svg
    release-checklist.md
    screencast.sh
    screencast.svg
    sponsors.md
  rustfmt.toml
  scripts/ (4 files)
    create-archive.sh
    create-deb.sh
    update-help.awk
    version-bump.sh
  src/ (22 files)
    cli.rs
    config.rs
    dir_entry.rs
    error.rs
    exec/ (3 files)
    exit_codes.rs
    filesystem.rs
    filetypes.rs
    filter/ (4 files)
    fmt/ (2 files)
    hyperlink.rs
    main.rs
    output.rs
    regex_helper.rs
    sanitize.rs
    walk.rs
  tests/ (2 files)
    testenv/ (1 files)
    tests.rs

## Code
- sym:38653 hir_has_uppercase_char (src/regex_helper.rs:15-37)
  fn hir_has_uppercase_char(hir: &Hir) -> bool
- sym:38655 hir_matches_strings_with_leading_dot (src/regex_helper.rs:50-79)
  fn hir_matches_strings_with_leading_dot(hir: &Hir) -> bool
- sym:38643 build_regex (src/main.rs:522-536)
  fn build_regex(pattern_regex: String, config: &Config) -> Result<regex::bytes::Regex>
- sym:38652 pattern_has_uppercase_char (src/regex_helper.rs:5-12)
  pub fn pattern_has_uppercase_char(pattern: &str) -> bool
- sym:38768 test_smart_case (tests/tests.rs:517-537)
  fn test_smart_case()
- sym:38773 test_smart_case_glob_searches (tests/tests.rs:626-636)
  fn test_smart_case_glob_searches()
- sym:38531 i32.from (src/exit_codes.rs:15-22)
  fn from(code: ExitCode) -> Self
- sym:38654 pattern_matches_strings_with_leading_dot (src/regex_helper.rs:40-47)
  pub fn pattern_matches_strings_with_leading_dot(pattern: &str) -> bool
- sym:38828 test_fixed_strings (tests/tests.rs:2412-2444)
  fn test_fixed_strings()
- sym:38770 test_case_insensitive (tests/tests.rs:556-570)
  fn test_case_insensitive()
- sym:38829 test_exact_literal_nonsubstring (tests/tests.rs:2448-2481)
  fn test_exact_literal_nonsubstring()
- sym:38769 test_case_sensitive (tests/tests.rs:541-552)
  fn test_case_sensitive()
- sym:38657 pattern_has_uppercase_char_advanced (src/regex_helper.rs:91-96)
  fn pattern_has_uppercase_char_advanced()
- sym:38656 pattern_has_uppercase_char_simple (src/regex_helper.rs:82-88)
  fn pattern_has_uppercase_char_simple()
- sym:38848 test_opposing (tests/tests.rs:2913-2928)
  fn test_opposing(flag: &str, opposing_flags: &[&str])

---
tokens: 937/2000
