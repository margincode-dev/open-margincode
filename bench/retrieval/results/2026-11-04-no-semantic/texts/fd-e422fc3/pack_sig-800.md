## Task
# fd-e422fc3 · fd-e422fc3-2 · open
Goal: Fix smart-case detection with inline case-insensitive flags

Inspect literal characters in the regex AST rather than the case-folded HIR. Otherwise an inline (?i) flag introduces uppercase alternatives and incorrectly makes other --and patterns case-sensitive.

Add unit and CLI regressions, including scoped flags and explicit case-sensitivity options.

Assisted-by: OpenAI Codex (investigation, implementation, tests and validation)
Workspace: /root/bench/ws/rebf815-fd-e422fc3/fd-e422fc3-2
Changed (0): nothing yet

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
- sym:38655 hir_matches_strings_with_leading_dot (src/regex_helper.rs:50-79)
  fn hir_matches_strings_with_leading_dot(hir: &Hir) -> bool
- sym:38643 build_regex (src/main.rs:522-536)
  fn build_regex(pattern_regex: String, config: &Config) -> Result<regex::bytes::Regex>
- sym:38653 hir_has_uppercase_char (src/regex_helper.rs:15-37)
  fn hir_has_uppercase_char(hir: &Hir) -> bool
- sym:38454 Config (src/config.rs:14-136)
  pub struct Config
- sym:38490 OutputBuffer.new (src/exec/command.rs:18-23)
  pub fn new(null_separator: bool) -> Self
- sym:38499 CommandSet.new (src/exec/mod.rs:36-49)
  pub fn new<I, T, S>(input: I) -> Result<CommandSet> where I: IntoIterator<Item = T>, T: IntoIterator<Item = S>, S: AsRef<str>,
- sym:38505 CommandBuilder.new (src/exec/mod.rs:136-162)
  fn new(template: &CommandTemplate, limit: usize) -> io::Result<Self>
- sym:38828 test_fixed_strings (tests/tests.rs:2412-2444)
  fn test_fixed_strings()
- sym:38829 test_exact_literal_nonsubstring (tests/tests.rs:2448-2481)
  fn test_exact_literal_nonsubstring()
- sym:38768 test_smart_case (tests/tests.rs:517-537)
  fn test_smart_case()
- sym:38848 test_opposing (tests/tests.rs:2913-2928)
  fn test_opposing(flag: &str, opposing_flags: &[&str])
- sym:38773 test_smart_case_glob_searches (tests/tests.rs:626-636)
  fn test_smart_case_glob_searches()

---
tokens: 680/800
