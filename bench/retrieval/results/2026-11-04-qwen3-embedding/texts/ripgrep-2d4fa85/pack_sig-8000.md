## Task
# ripgrep-2d4fa85 · ripgrep-2d4fa85 · open
Goal: ignore: skip empty absolute parent matchers (#3557)

When no ancestor above the walk root has ignore rules, we now skip
constructing an absolute path and checking parent matchers. Repository
boundaries still affect Git ignore rules, even when the repository has
no rules of its own.

On Linux x86-64 against 3fce3b5, warm incremental matching improves
13.4% and replay allocates or reallocates about half as often.
Warm-cache walks of the Ruff and uv checkouts improve 1–4%; the
cached ignored-path control is 3.3% slower.

PR #3557
Workspace: /root/bench/ws/rebf815-ripgrep-2d4fa85/ripgrep-2d4fa85
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
- sym:16277 Ignore (crates/ignore/src/dir.rs:94-112)
  pub(crate) struct Ignore
- sym:16278 IgnoreInner (crates/ignore/src/dir.rs:115-166)
  struct IgnoreInner
- sym:16247 incremental_ignored_parent (crates/ignore/benches/workloads.rs:380-388)
  fn incremental_ignored_parent(b: &mut test::Bencher)
- sym:16262 incremental_gitignore_only (crates/ignore/benches/workloads.rs:563-576)
  fn incremental_gitignore_only(b: &mut test::Bencher)
- sym:16367 Gitignore.matched_path_or_any_parents (crates/ignore/src/gitignore.rs:234-256)
  pub fn matched_path_or_any_parents<P: AsRef<Path>>( &self, path: P, is_dir: bool, ) -> Match<&Glob>
- sym:16282 Ignore.add_parents (crates/ignore/src/dir.rs:192-254)
  pub(crate) fn add_parents<P: AsRef<Path>>( &self, path: P, ) -> (Ignore, Option<Error>)
- sym:16703 check_symlink_loop (crates/ignore/src/walk.rs:2038-2059)
  fn check_symlink_loop( ig_parent: &Ignore, child_path: &Path, child_depth: usize, ) -> Result<(), Error>
- sym:16291 Ignore.matched_ignore (crates/ignore/src/dir.rs:544-681)
  pub(crate) fn matched_ignore<'a>( &'a self, path: &Path, is_dir: bool, ) -> Match<IgnoreMatch<'a>>
- sym:16654 Walk.next (crates/ignore/src/walk.rs:1190-1261)
  fn next(&mut self) -> Option<Result<DirEntry, Error>>
- sym:16399 IncrementalIgnore (crates/ignore/src/incremental.rs:57-68)
  pub struct IncrementalIgnore
- sym:16682 Work.add_parents (crates/ignore/src/walk.rs:1591-1600)
  fn add_parents(&mut self) -> Option<Error>
- sym:16409 IncrementalIgnore.matched_with_errors_ignore (crates/ignore/src/incremental.rs:310-391)
  fn matched_with_errors_ignore( &mut self, relative: &Path, is_dir: bool, errs: &mut PartialErrorBuilder, ) -> Option<IncrementalMatch>
- sym:16693 Worker.generate_work (crates/ignore/src/walk.rs:1874-1936)
  fn generate_work( &mut self, ig: &Ignore, depth: usize, root_device: Option<u64>, fs_dent: fs::DirEntry, ) -> WalkState
- sym:16621 WalkBuilder.build_parallel (crates/ignore/src/walk.rs:701-715)
  pub fn build_parallel(&self) -> WalkParallel
- sym:16276 IgnoreOptions (crates/ignore/src/dir.rs:72-90)
  struct IgnoreOptions
- sym:16649 Walk (crates/ignore/src/walk.rs:1117-1126)
  pub struct Walk
- sym:16246 incremental_ignored_nested (crates/ignore/benches/workloads.rs:366-377)
  fn incremental_ignored_nested(b: &mut test::Bencher)
- sym:16239 incremental_warm_noncanonical (crates/ignore/benches/workloads.rs:271-290)
  fn incremental_warm_noncanonical(b: &mut test::Bencher)
- sym:16240 incremental_ignored (crates/ignore/benches/workloads.rs:293-301)
  fn incremental_ignored(b: &mut test::Bencher)
- sym:16410 IncrementalIgnore.match_path (crates/ignore/src/incremental.rs:393-411)
  fn match_path( &self, ignore: &Ignore, relative: &Path, is_dir: bool, ) -> IncrementalMatch
- sym:16253 incremental_dense_ignore_warm (crates/ignore/benches/workloads.rs:436-447)
  fn incremental_dense_ignore_warm(b: &mut test::Bencher)
- sym:16304 IgnoreBuilder.build_with_cwd (crates/ignore/src/dir.rs:813-858)
  pub(crate) fn build_with_cwd(&self, cwd: Option<PathBuf>) -> Ignore
- sym:16673 WalkParallel (crates/ignore/src/walk.rs:1411-1422)
  pub struct WalkParallel
- sym:16346 absolute_parent_matchers_are_cached_across_roots (crates/ignore/src/dir.rs:1427-1447)
  fn absolute_parent_matchers_are_cached_across_roots()
- sym:16443 respects_git_repository_boundaries (crates/ignore/src/incremental.rs:788-806)
  fn respects_git_repository_boundaries()
- sym:16449 compiled_multi_child_matchers_are_not_reused (crates/ignore/src/incremental.rs:901-936)
  fn compiled_multi_child_matchers_are_not_reused()
- sym:16442 ignored_ancestor_wins (crates/ignore/src/incremental.rs:767-783)
  fn ignored_ancestor_wins()
- sym:16731 gitignore_parent (crates/ignore/src/walk.rs:2436-2446)
  fn gitignore_parent()
- sym:16347 multi_root_gitignore_order_independent (crates/ignore/src/dir.rs:1460-1516)
  fn multi_root_gitignore_order_independent()
- sym:16448 compiled_multi_matchers_are_reused (crates/ignore/src/incremental.rs:881-898)
  fn compiled_multi_matchers_are_reused()
- sym:16451 parent_loading_can_be_disabled (crates/ignore/src/incremental.rs:958-976)
  fn parent_loading_can_be_disabled()
- sym:16434 nested_parent_gitignores (crates/ignore/src/incremental.rs:629-643)
  fn nested_parent_gitignores()
- sym:16437 parent_ignore_and_custom_ignore (crates/ignore/src/incremental.rs:679-695)
  fn parent_ignore_and_custom_ignore()
- sym:16345 absolute_parent_anchored (crates/ignore/src/dir.rs:1407-1424)
  fn absolute_parent_anchored()
- sym:14661 DecompressionMatcherBuilder.new (crates/cli/src/decompress.rs:42-44)
  pub fn new() -> DecompressionMatcherBuilder
- sym:14668 DecompressionMatcher.new (crates/cli/src/decompress.rs:167-171)
  pub fn new() -> DecompressionMatcher
- sym:14672 DecompressionReaderBuilder.new (crates/cli/src/decompress.rs:205-207)
  pub fn new() -> DecompressionReaderBuilder
- sym:14678 DecompressionReader.new (crates/cli/src/decompress.rs:352-356)
  pub fn new<P: AsRef<Path>>( path: P, ) -> Result<DecompressionReader, CommandError>
- sym:14744 CommandReaderBuilder.new (crates/cli/src/process.rs:90-92)
  pub fn new() -> CommandReaderBuilder
- sym:14748 CommandReader.new (crates/cli/src/process.rs:195-199)
  pub fn new( cmd: &mut process::Command, ) -> Result<CommandReader, CommandError>
- sym:15817 State.new (crates/core/flags/hiargs.rs:950-960)
  fn new() -> anyhow::Result<State>
- sym:15863 ContextSeparator.new (crates/core/flags/lowargs.rs:483-491)
  pub(crate) fn new(os: &OsStr) -> anyhow::Result<ContextSeparator>
- sym:15870 FieldContextSeparator.new (crates/core/flags/lowargs.rs:561-569)
  pub(crate) fn new(os: &OsStr) -> anyhow::Result<FieldContextSeparator>
- sym:15874 FieldMatchSeparator.new (crates/core/flags/lowargs.rs:595-603)
  pub(crate) fn new(os: &OsStr) -> anyhow::Result<FieldMatchSeparator>
- sym:15910 Parser.new (crates/core/flags/parse.rs:176-216)
  fn new() -> &'static Parser
- sym:15919 FlagMap.new (crates/core/flags/parse.rs:374-387)
  fn new(infos: &[FlagInfo]) -> FlagMap
- sym:15927 HaystackBuilder.new (crates/core/haystack.rs:22-24)
  pub(crate) fn new() -> HaystackBuilder
- sym:15932 Haystack.path (crates/core/haystack.rs:108-114)
  pub(crate) fn path(&self) -> &Path
- sym:15968 SearchWorkerBuilder.new (crates/core/search.rs:54-59)
  pub(crate) fn new() -> SearchWorkerBuilder
- sym:16740 no_read_permissions (crates/ignore/src/walk.rs:2658-2673)
  fn no_read_permissions()
- sym:16446 cached_ignored_parent_matches_children (crates/ignore/src/incremental.rs:848-860)
  fn cached_ignored_parent_matches_children()
- sym:16447 cached_parent_matcher_is_used_for_directory (crates/ignore/src/incremental.rs:863-877)
  fn cached_parent_matcher_is_used_for_directory()
- sym:16752 test_dirs_in_deep (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:193-318)
  fn test_dirs_in_deep()
- sym:16348 multi_root_custom_ignore_order_independent (crates/ignore/src/dir.rs:1523-1563)
  fn multi_root_custom_ignore_order_independent()
- sym:16444 compiled_matchers_are_reused (crates/ignore/src/incremental.rs:811-830)
  fn compiled_matchers_are_reused()
- sym:16445 cached_allowed_parent_matches_children (crates/ignore/src/incremental.rs:833-845)
  fn cached_allowed_parent_matches_children()
- sym:16750 test_files_in_deep (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:57-86)
  fn test_files_in_deep()
- sym:16441 directly_ignored_directory_is_not_descended (crates/ignore/src/incremental.rs:751-761)
  fn directly_ignored_directory_is_not_descended()
- sym:16439 glob_overrides_apply_to_ancestors (crates/ignore/src/incremental.rs:717-729)
  fn glob_overrides_apply_to_ancestors()
- sym:16435 anchored_child_rule_uses_child_root (crates/ignore/src/incremental.rs:648-657)
  fn anchored_child_rule_uses_child_root()
- sym:16450 partial_errors_keep_valid_rules (crates/ignore/src/incremental.rs:941-953)
  fn partial_errors_keep_valid_rules()
- sym:16344 absolute_parent (crates/ignore/src/dir.rs:1384-1404)
  fn absolute_parent()
- sym:14689 b (crates/cli/src/escape.rs:85-87)
  fn b(bytes: &'static [u8]) -> Vec<u8>

---
tokens: 2982/8000
