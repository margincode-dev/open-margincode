## Task
# fd-e422fc3 · fd-e422fc3 · open
Goal: Fix smart-case detection with inline case-insensitive flags

Inspect literal characters in the regex AST rather than the case-folded HIR. Otherwise an inline (?i) flag introduces uppercase alternatives and incorrectly makes other --and patterns case-sensitive.

Add unit and CLI regressions, including scoped flags and explicit case-sensitivity options.

Assisted-by: OpenAI Codex (investigation, implementation, tests and validation)
Workspace: /root/bench/av_gemma/ws/rec728b-fd-e422fc3/fd-e422fc3
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
- sym:38654 pattern_matches_strings_with_leading_dot (src/regex_helper.rs:40-47)
  pub fn pattern_matches_strings_with_leading_dot(pattern: &str) -> bool
- sym:38638 construct_config (src/main.rs:253-374)
  fn construct_config(mut opts: Opts, pattern_regexps: &[String]) -> Result<Config>
- sym:38636 build_pattern_regex (src/main.rs:223-237)
  fn build_pattern_regex(pattern: &str, opts: &Opts) -> Result<String>
- sym:38431 Opts (src/cli.rs:31-694)
  pub struct Opts
- sym:38570 SIZE_CAPTURES (src/filter/size.rs:6-6)
  static SIZE_CAPTURES: OnceLock<Regex> = OnceLock::new()
- sym:38642 ensure_use_hidden_option_for_leading_dot_pattern (src/main.rs:502-520)
  fn ensure_use_hidden_option_for_leading_dot_pattern( config: &Config, pattern_regexps: &[String], ) -> Result<()>
- sym:38700 WorkerState.new (src/walk.rs:317-327)
  fn new(patterns: Vec<Regex>, config: Config) -> Self
- sym:38569 Check.parse (src/filter/owner.rs:85-102)
  fn parse<F>(s: Option<&str>, f: F) -> Result<Self> where F: Fn(&str) -> Result<T>,
- sym:38454 Config (src/config.rs:14-136)
  pub struct Config
- sym:38631 run (src/main.rs:80-117)
  fn run() -> Result<ExitCode>
- sym:38699 WorkerState (src/walk.rs:305-314)
  struct WorkerState
- sym:38581 SizeFilter.parse_opt (src/filter/size.rs:33-66)
  fn parse_opt(s: &str) -> Option<Self>
- sym:38691 ReceiverBuffer.new (src/walk.rs:152-170)
  fn new(state: &'a WorkerState, rx: Receiver<Batch>, stdout: W) -> Self
- sym:38659 needs_escape (src/sanitize.rs:8-23)
  fn needs_escape(c: char) -> bool
- sym:38705 WorkerState.scan (src/walk.rs:606-642)
  fn scan(&self, paths: &[PathBuf]) -> Result<ExitCode>
- sym:38629 DEFAULT_LS_COLORS (src/main.rs:60-63)
  const DEFAULT_LS_COLORS: &str = "
- sym:38433 Opts.normalize_path (src/cli.rs:726-739)
  fn normalize_path(&self, path: &Path) -> PathBuf
- sym:38456 DirEntryInner (src/dir_entry.rs:12-17)
  enum DirEntryInner
- sym:38634 ensure_search_pattern_is_not_a_path (src/main.rs:174-186)
  fn ensure_search_pattern_is_not_a_path(opts: &Opts) -> Result<()>
- sym:38828 test_fixed_strings (tests/tests.rs:2412-2444)
  fn test_fixed_strings()
- sym:38768 test_smart_case (tests/tests.rs:517-537)
  fn test_smart_case()
- sym:38770 test_case_insensitive (tests/tests.rs:556-570)
  fn test_case_insensitive()
- sym:38773 test_smart_case_glob_searches (tests/tests.rs:626-636)
  fn test_smart_case_glob_searches()
- sym:38829 test_exact_literal_nonsubstring (tests/tests.rs:2448-2481)
  fn test_exact_literal_nonsubstring()
- sym:38769 test_case_sensitive (tests/tests.rs:541-552)
  fn test_case_sensitive()
- sym:38767 test_regex_searches (tests/tests.rs:496-513)
  fn test_regex_searches()
- sym:38756 test_and_plus_ignore_case (tests/tests.rs:244-252)
  fn test_and_plus_ignore_case()
- sym:38757 test_and_plus_case_sensitive (tests/tests.rs:255-262)
  fn test_and_plus_case_sensitive()
- sym:38774 test_case_sensitive_glob_searches (tests/tests.rs:640-644)
  fn test_case_sensitive_glob_searches()
- sym:38658 matches_strings_with_leading_dot_simple (src/regex_helper.rs:99-105)
  fn matches_strings_with_leading_dot_simple()
- sym:38531 i32.from (src/exit_codes.rs:15-22)
  fn from(code: ExitCode) -> Self
- sym:38657 pattern_has_uppercase_char_advanced (src/regex_helper.rs:91-96)
  fn pattern_has_uppercase_char_advanced()
- sym:38848 test_opposing (tests/tests.rs:2913-2928)
  fn test_opposing(flag: &str, opposing_flags: &[&str])
- sym:38656 pattern_has_uppercase_char_simple (src/regex_helper.rs:82-88)
  fn pattern_has_uppercase_char_simple()
- sym:38520 tokens_with_literal_braces (src/exec/mod.rs:369-376)
  fn tokens_with_literal_braces()
- sym:38521 tokens_with_literal_braces_and_placeholder (src/exec/mod.rs:379-383)
  fn tokens_with_literal_braces_and_placeholder()
- sym:38791 test_no_global_ignore (tests/tests.rs:1049-1052)
  fn test_no_global_ignore(flag: &str, expected_output: &str)
- sym:38600 out_of_range_unix_timestamp_is_rejected (src/filter/time.rs:204-209)
  fn out_of_range_unix_timestamp_is_rejected()
- sym:38617 parse_only_brace_escapes (src/fmt/mod.rs:228-234)
  fn parse_only_brace_escapes()
- sym:38672 strips_c1_csi_and_osc_initiators (src/sanitize.rs:136-140)
  fn strips_c1_csi_and_osc_initiators()
- sym:38776 test_regex_overrides_glob (tests/tests.rs:659-663)
  fn test_regex_overrides_glob()

---
tokens: 1906/8000
