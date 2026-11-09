## Task
# ripgrep-b5611fc · ripgrep-b5611fc · open
Goal: ignore: reuse parent repository detection

PR #3556
Workspace: /root/bench/av_tei/ws/rec09fc-ripgrep-b5611fc/ripgrep-b5611fc
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
- sym:19616 incremental_ignored_parent (crates/ignore/benches/workloads.rs:380-388)
  fn incremental_ignored_parent(b: &mut test::Bencher)
- sym:19645 IgnoreOptions (crates/ignore/src/dir.rs:72-90)
  struct IgnoreOptions
- sym:19651 Ignore.add_parents (crates/ignore/src/dir.rs:192-258)
  pub(crate) fn add_parents<P: AsRef<Path>>( &self, path: P, ) -> (Ignore, Option<Error>)
- sym:18771 NoRequireGit.doc_long (crates/core/flags/defs.rs:4850-4863)
  fn doc_long(&self) -> &'static str
- sym:19650 Ignore.parent (crates/ignore/src/dir.rs:181-186)
  pub(crate) fn parent(&self) -> Option<Ignore>
- sym:19666 IgnoreRef.is_absolute_parent (crates/ignore/src/dir.rs:725-727)
  pub(crate) fn is_absolute_parent(&self) -> bool
- sym:20072 check_symlink_loop (crates/ignore/src/walk.rs:2038-2059)
  fn check_symlink_loop( ig_parent: &Ignore, child_path: &Path, child_depth: usize, ) -> Result<(), Error>
- sym:19668 Parents.Item (crates/ignore/src/dir.rs:734-734)
  type Item = IgnoreRef<'a>
- sym:18729 NoIgnoreParent (crates/core/flags/defs.rs:4632-4632)
  struct NoIgnoreParent
- sym:19667 Parents (crates/ignore/src/dir.rs:731-731)
  pub(crate) struct Parents<'a>
- sym:19812 respects_git_repository_boundaries (crates/ignore/src/incremental.rs:788-806)
  fn respects_git_repository_boundaries()
- sym:19806 parent_ignore_and_custom_ignore (crates/ignore/src/incremental.rs:679-695)
  fn parent_ignore_and_custom_ignore()
- sym:19820 parent_loading_can_be_disabled (crates/ignore/src/incremental.rs:958-976)
  fn parent_loading_can_be_disabled()
- sym:19713 absolute_parent (crates/ignore/src/dir.rs:1388-1408)
  fn absolute_parent()
- sym:18030 DecompressionMatcherBuilder.new (crates/cli/src/decompress.rs:42-44)
  pub fn new() -> DecompressionMatcherBuilder
- sym:18037 DecompressionMatcher.new (crates/cli/src/decompress.rs:167-171)
  pub fn new() -> DecompressionMatcher
- sym:18041 DecompressionReaderBuilder.new (crates/cli/src/decompress.rs:205-207)
  pub fn new() -> DecompressionReaderBuilder
- sym:18047 DecompressionReader.new (crates/cli/src/decompress.rs:352-356)
  pub fn new<P: AsRef<Path>>( path: P, ) -> Result<DecompressionReader, CommandError>
- sym:18113 CommandReaderBuilder.new (crates/cli/src/process.rs:90-92)
  pub fn new() -> CommandReaderBuilder
- sym:18117 CommandReader.new (crates/cli/src/process.rs:195-199)
  pub fn new( cmd: &mut process::Command, ) -> Result<CommandReader, CommandError>
- sym:19146 generate_long_flag (crates/core/flags/doc/help.rs:131-208)
  fn generate_long_flag(flag: &dyn Flag, out: &mut String)
- sym:19803 nested_parent_gitignores (crates/ignore/src/incremental.rs:629-643)
  fn nested_parent_gitignores()
- sym:20121 test_dirs_in_deep (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:193-318)
  fn test_dirs_in_deep()
- sym:18737 test_no_ignore_parent (crates/core/flags/defs.rs:4667-4677)
  fn test_no_ignore_parent()
- sym:19714 absolute_parent_anchored (crates/ignore/src/dir.rs:1411-1428)
  fn absolute_parent_anchored()
- sym:19814 cached_allowed_parent_matches_children (crates/ignore/src/incremental.rs:833-845)
  fn cached_allowed_parent_matches_children()
- sym:19817 compiled_multi_matchers_are_reused (crates/ignore/src/incremental.rs:881-898)
  fn compiled_multi_matchers_are_reused()
- sym:20100 gitignore_parent (crates/ignore/src/walk.rs:2436-2446)
  fn gitignore_parent()
- sym:20119 test_files_in_deep (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:57-86)
  fn test_files_in_deep()
- sym:19822 multiple_roots_keep_their_own_context (crates/ignore/src/incremental.rs:1009-1032)
  fn multiple_roots_keep_their_own_context()
- sym:19816 cached_parent_matcher_is_used_for_directory (crates/ignore/src/incremental.rs:863-877)
  fn cached_parent_matcher_is_used_for_directory()
- sym:18683 test_no_ignore (crates/core/flags/defs.rs:4368-4389)
  fn test_no_ignore()
- sym:19810 directly_ignored_directory_is_not_descended (crates/ignore/src/incremental.rs:751-761)
  fn directly_ignored_directory_is_not_descended()
- sym:19715 absolute_parent_matchers_are_cached_across_roots (crates/ignore/src/dir.rs:1431-1451)
  fn absolute_parent_matchers_are_cached_across_roots()
- sym:18058 b (crates/cli/src/escape.rs:85-87)
  fn b(bytes: &'static [u8]) -> Vec<u8>

---
tokens: 1717/4000
