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
Workspace: /root/bench/av_gemma/ws/rec728b-ripgrep-2d4fa85/ripgrep-2d4fa85
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
- sym:16367 Gitignore.matched_path_or_any_parents (crates/ignore/src/gitignore.rs:234-256)
  pub fn matched_path_or_any_parents<P: AsRef<Path>>( &self, path: P, is_dir: bool, ) -> Match<&Glob>
- sym:16277 Ignore (crates/ignore/src/dir.rs:94-112)
  pub(crate) struct Ignore
- sym:16703 check_symlink_loop (crates/ignore/src/walk.rs:2038-2059)
  fn check_symlink_loop( ig_parent: &Ignore, child_path: &Path, child_depth: usize, ) -> Result<(), Error>
- sym:16278 IgnoreInner (crates/ignore/src/dir.rs:115-166)
  struct IgnoreInner
- sym:16409 IncrementalIgnore.matched_with_errors_ignore (crates/ignore/src/incremental.rs:310-391)
  fn matched_with_errors_ignore( &mut self, relative: &Path, is_dir: bool, errs: &mut PartialErrorBuilder, ) -> Option<IncrementalMatch>
- sym:16654 Walk.next (crates/ignore/src/walk.rs:1190-1261)
  fn next(&mut self) -> Option<Result<DirEntry, Error>>
- sym:16399 IncrementalIgnore (crates/ignore/src/incremental.rs:57-68)
  pub struct IncrementalIgnore
- sym:16682 Work.add_parents (crates/ignore/src/walk.rs:1591-1600)
  fn add_parents(&mut self) -> Option<Error>
- sym:16410 IncrementalIgnore.match_path (crates/ignore/src/incremental.rs:393-411)
  fn match_path( &self, ignore: &Ignore, relative: &Path, is_dir: bool, ) -> IncrementalMatch
- sym:16282 Ignore.add_parents (crates/ignore/src/dir.rs:192-254)
  pub(crate) fn add_parents<P: AsRef<Path>>( &self, path: P, ) -> (Ignore, Option<Error>)
- sym:16402 CachedDir (crates/ignore/src/incremental.rs:94-103)
  enum CachedDir
- sym:16509 Override.matched (crates/ignore/src/overrides.rs:97-110)
  pub fn matched<'a, P: AsRef<Path>>( &'a self, path: P, is_dir: bool, ) -> Match<Glob<'a>>
- sym:16247 incremental_ignored_parent (crates/ignore/benches/workloads.rs:380-388)
  fn incremental_ignored_parent(b: &mut test::Bencher)
- sym:16300 Parents.next (crates/ignore/src/dir.rs:732-744)
  fn next(&mut self) -> Option<IgnoreRef<'a>>
- sym:16693 Worker.generate_work (crates/ignore/src/walk.rs:1874-1936)
  fn generate_work( &mut self, ig: &Ignore, depth: usize, root_device: Option<u64>, fs_dent: fs::DirEntry, ) -> WalkState
- sym:16223 Workspace.builder (crates/ignore/benches/workloads.rs:94-99)
  fn builder(&self) -> WalkBuilder
- sym:16621 WalkBuilder.build_parallel (crates/ignore/src/walk.rs:701-715)
  pub fn build_parallel(&self) -> WalkParallel
- sym:16276 IgnoreOptions (crates/ignore/src/dir.rs:72-90)
  struct IgnoreOptions
- sym:16648 WalkBuilder.build_ignore (crates/ignore/src/walk.rs:1104-1108)
  fn build_ignore(&self) -> Ignore
- sym:16649 Walk (crates/ignore/src/walk.rs:1117-1126)
  pub struct Walk
- sym:16262 incremental_gitignore_only (crates/ignore/benches/workloads.rs:563-576)
  fn incremental_gitignore_only(b: &mut test::Bencher)
- sym:16620 WalkBuilder.build_matchers (crates/ignore/src/walk.rs:675-694)
  pub fn build_matchers(&self) -> Vec<IncrementalIgnore>
- sym:16413 IncrementalMatch.none (crates/ignore/src/incremental.rs:456-462)
  fn none(is_dir: bool) -> IncrementalMatch
- sym:16647 WalkBuilder.get_or_set_current_dir (crates/ignore/src/walk.rs:1080-1101)
  fn get_or_set_current_dir(&self) -> Option<&Path>
- sym:16304 IgnoreBuilder.build_with_cwd (crates/ignore/src/dir.rs:813-858)
  pub(crate) fn build_with_cwd(&self, cwd: Option<PathBuf>) -> Ignore
- sym:16673 WalkParallel (crates/ignore/src/walk.rs:1411-1422)
  pub struct WalkParallel
- sym:16346 absolute_parent_matchers_are_cached_across_roots (crates/ignore/src/dir.rs:1427-1447)
  fn absolute_parent_matchers_are_cached_across_roots()
- sym:16449 compiled_multi_child_matchers_are_not_reused (crates/ignore/src/incremental.rs:901-936)
  fn compiled_multi_child_matchers_are_not_reused()
- sym:16443 respects_git_repository_boundaries (crates/ignore/src/incremental.rs:788-806)
  fn respects_git_repository_boundaries()
- sym:16345 absolute_parent_anchored (crates/ignore/src/dir.rs:1407-1424)
  fn absolute_parent_anchored()
- sym:16451 parent_loading_can_be_disabled (crates/ignore/src/incremental.rs:958-976)
  fn parent_loading_can_be_disabled()
- sym:16344 absolute_parent (crates/ignore/src/dir.rs:1384-1404)
  fn absolute_parent()
- sym:16442 ignored_ancestor_wins (crates/ignore/src/incremental.rs:767-783)
  fn ignored_ancestor_wins()
- sym:16437 parent_ignore_and_custom_ignore (crates/ignore/src/incremental.rs:679-695)
  fn parent_ignore_and_custom_ignore()
- sym:16434 nested_parent_gitignores (crates/ignore/src/incremental.rs:629-643)
  fn nested_parent_gitignores()
- sym:14710 Error.from (crates/cli/src/human.rs:66-68)
  fn from(size_err: ParseSizeError) -> std::io::Error
- sym:14727 Error.from (crates/cli/src/pattern.rs:40-42)
  fn from(paterr: InvalidPatternError) -> io::Error
- sym:14739 CommandError.is_empty (crates/cli/src/process.rs:33-38)
  pub(crate) fn is_empty(&self) -> bool
- sym:14741 CommandError.from (crates/cli/src/process.rs:66-68)
  fn from(ioerr: io::Error) -> CommandError
- sym:14742 Error.from (crates/cli/src/process.rs:72-79)
  fn from(cmderr: CommandError) -> io::Error
- sym:15053 Glob (crates/core/flags/defs.rs:2504-2504)
  struct Glob
- sym:15932 Haystack.path (crates/core/haystack.rs:108-114)
  pub(crate) fn path(&self) -> &Path
- sym:15935 Haystack.is_dir (crates/core/haystack.rs:143-154)
  fn is_dir(&self) -> bool
- sym:16020 Glob (crates/globset/src/glob.rs:76-81)
  pub struct Glob
- sym:16021 Glob.as_ref (crates/globset/src/glob.rs:84-86)
  fn as_ref(&self) -> &Glob
- sym:16026 Glob.Err (crates/globset/src/glob.rs:124-124)
  type Err = Error
- sym:16094 Error (crates/globset/src/lib.rs:157-162)
  pub struct Error
- sym:16107 GlobSet.is_empty (crates/globset/src/lib.rs:331-333)
  pub fn is_empty(&self) -> bool
- sym:16176 MultiStrategyBuilder.is_empty (crates/globset/src/lib.rs:1063-1065)
  fn is_empty(&self) -> bool
- sym:16266 DirEntry.path (crates/ignore/examples/walk.rs:58-63)
  fn path(&self) -> &Path
- sym:16445 cached_allowed_parent_matches_children (crates/ignore/src/incremental.rs:833-845)
  fn cached_allowed_parent_matches_children()
- sym:16447 cached_parent_matcher_is_used_for_directory (crates/ignore/src/incremental.rs:863-877)
  fn cached_parent_matcher_is_used_for_directory()
- sym:16740 no_read_permissions (crates/ignore/src/walk.rs:2658-2673)
  fn no_read_permissions()
- sym:16446 cached_ignored_parent_matches_children (crates/ignore/src/incremental.rs:848-860)
  fn cached_ignored_parent_matches_children()
- sym:16731 gitignore_parent (crates/ignore/src/walk.rs:2436-2446)
  fn gitignore_parent()
- sym:16347 multi_root_gitignore_order_independent (crates/ignore/src/dir.rs:1460-1516)
  fn multi_root_gitignore_order_independent()
- sym:16452 paths_are_relative_to_the_root (crates/ignore/src/incremental.rs:981-1004)
  fn paths_are_relative_to_the_root()
- sym:16752 test_dirs_in_deep (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:193-318)
  fn test_dirs_in_deep()
- sym:16348 multi_root_custom_ignore_order_independent (crates/ignore/src/dir.rs:1523-1563)
  fn multi_root_custom_ignore_order_independent()
- sym:16750 test_files_in_deep (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:57-86)
  fn test_files_in_deep()
- sym:16435 anchored_child_rule_uses_child_root (crates/ignore/src/incremental.rs:648-657)
  fn anchored_child_rule_uses_child_root()
- sym:16436 leading_dot_slash_impacts_matching (crates/ignore/src/incremental.rs:662-675)
  fn leading_dot_slash_impacts_matching()
- sym:16431 matchers (crates/ignore/src/incremental.rs:605-607)
  fn matchers(builder: &WalkBuilder) -> Vec<IncrementalIgnore>
- sym:16448 compiled_multi_matchers_are_reused (crates/ignore/src/incremental.rs:881-898)
  fn compiled_multi_matchers_are_reused()
- sym:16450 partial_errors_keep_valid_rules (crates/ignore/src/incremental.rs:941-953)
  fn partial_errors_keep_valid_rules()
- sym:16439 glob_overrides_apply_to_ancestors (crates/ignore/src/incremental.rs:717-729)
  fn glob_overrides_apply_to_ancestors()
- sym:16279 Ignore.path (crates/ignore/src/dir.rs:171-173)
  pub(crate) fn path(&self) -> &Path

---
tokens: 3041/8000
