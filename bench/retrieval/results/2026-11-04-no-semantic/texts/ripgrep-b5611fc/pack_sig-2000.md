## Task
# ripgrep-b5611fc · ripgrep-b5611fc-2 · open
Goal: ignore: reuse parent repository detection

PR #3556
Workspace: /root/bench/ws/rebf815-ripgrep-b5611fc/ripgrep-b5611fc-2
Changed (0): nothing yet

## Map
- repo map (226 files):
  .cargo/ (1 files)
    config.toml
  .github/ (6 files)
    FUNDING.yml
    ISSUE_TEMPLATE/ (3 files)
    workflows/ (2 files)
  .gitignore
  .ignore
  .nvim.lua
  AI_POLICY.md
  CHANGELOG.md
  CONTRIBUTING.md
  COPYING
  Cargo.lock
  Cargo.toml
  FAQ.md
  GUIDE.md
  HomebrewFormula
  LICENSE-MIT
  README.md
  RELEASE-CHECKLIST.md
  UNLICENSE
  benchsuite/ (30 files)
    benchsuite
    runs/ (29 files)
  build.rs
  ci/ (4 files)
    sha256-releases
    test-complete
    ubuntu-install-packages
    utils.sh
  crates/ (139 files)
    cli/ (12 files)
    core/ (27 files)
    globset/ (11 files)
    grep/ (7 files)
    ignore/ (21 files)
    matcher/ (9 files)
    pcre2/ (7 files)
    printer/ (17 files)
    regex/ (13 files)
    searcher/ (15 files)
  fuzz/ (5 files)
    .gitignore
    Cargo.lock
    Cargo.toml
    README.md
    fuzz_targets/ (1 files)
  pkg/ (3 files)
    brew/ (1 files)
    windows/ (2 files)
  rustfmt.toml
  scripts/ (1 files)
    copy-examples
  tests/ (19 files)
    binary.rs
    data/ (9 files)
    feature.rs
    hay.rs
    json.rs
    macros.rs
    misc.rs
    multiline.rs
    regression.rs
    tests.rs
    util.rs

## Code
- sym:19645 IgnoreOptions (crates/ignore/src/dir.rs:72-90)
  struct IgnoreOptions
- sym:18771 NoRequireGit.doc_long (crates/core/flags/defs.rs:4850-4863)
  fn doc_long(&self) -> &'static str
- sym:19650 Ignore.parent (crates/ignore/src/dir.rs:181-186)
  pub(crate) fn parent(&self) -> Option<Ignore>
- sym:19146 generate_long_flag (crates/core/flags/doc/help.rs:131-208)
  fn generate_long_flag(flag: &dyn Flag, out: &mut String)
- sym:19150 generate_flag (crates/core/flags/doc/man.rs:46-110)
  fn generate_flag(flag: &'static dyn Flag, out: &mut String)
- sym:19390 Glob.as_ref (crates/globset/src/glob.rs:84-86)
  fn as_ref(&self) -> &Glob
- sym:19591 Workspace.write (crates/ignore/benches/workloads.rs:88-92)
  fn write(&self, relative: &str, contents: &str)
- sym:19812 respects_git_repository_boundaries (crates/ignore/src/incremental.rs:788-806)
  fn respects_git_repository_boundaries()
- sym:20121 test_dirs_in_deep (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:193-318)
  fn test_dirs_in_deep()
- sym:18737 test_no_ignore_parent (crates/core/flags/defs.rs:4667-4677)
  fn test_no_ignore_parent()
- sym:19817 compiled_multi_matchers_are_reused (crates/ignore/src/incremental.rs:881-898)
  fn compiled_multi_matchers_are_reused()
- sym:20119 test_files_in_deep (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:57-86)
  fn test_files_in_deep()

---
tokens: 888/2000
