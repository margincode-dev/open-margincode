## Task
# ripgrep-b5611fc · ripgrep-b5611fc · open
Goal: ignore: reuse parent repository detection

PR #3556
Workspace: /root/bench/av_gemma/ws/rec728b-ripgrep-b5611fc/ripgrep-b5611fc
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
- sym:19650 Ignore.parent (crates/ignore/src/dir.rs:181-186)
  pub(crate) fn parent(&self) -> Option<Ignore>
- sym:19666 IgnoreRef.is_absolute_parent (crates/ignore/src/dir.rs:725-727)
  pub(crate) fn is_absolute_parent(&self) -> bool
- sym:18729 NoIgnoreParent (crates/core/flags/defs.rs:4632-4632)
  struct NoIgnoreParent
- sym:19616 incremental_ignored_parent (crates/ignore/benches/workloads.rs:380-388)
  fn incremental_ignored_parent(b: &mut test::Bencher)
- sym:19736 Gitignore.matched_path_or_any_parents (crates/ignore/src/gitignore.rs:234-256)
  pub fn matched_path_or_any_parents<P: AsRef<Path>>( &self, path: P, is_dir: bool, ) -> Match<&Glob>
- sym:20072 check_symlink_loop (crates/ignore/src/walk.rs:2038-2059)
  fn check_symlink_loop( ig_parent: &Ignore, child_path: &Path, child_depth: usize, ) -> Result<(), Error>
- sym:18732 NoIgnoreParent.name_negated (crates/core/flags/defs.rs:4641-4643)
  fn name_negated(&self) -> Option<&'static str>
- sym:19645 IgnoreOptions (crates/ignore/src/dir.rs:72-90)
  struct IgnoreOptions
- sym:19669 Parents.next (crates/ignore/src/dir.rs:736-748)
  fn next(&mut self) -> Option<IgnoreRef<'a>>
- sym:18771 NoRequireGit.doc_long (crates/core/flags/defs.rs:4850-4863)
  fn doc_long(&self) -> &'static str
- sym:19646 Ignore (crates/ignore/src/dir.rs:94-112)
  pub(crate) struct Ignore
- sym:19668 Parents.Item (crates/ignore/src/dir.rs:734-734)
  type Item = IgnoreRef<'a>
- sym:19661 Ignore.parents (crates/ignore/src/dir.rs:688-690)
  pub(crate) fn parents(&self) -> Parents<'_>
- sym:19592 Workspace.builder (crates/ignore/benches/workloads.rs:94-99)
  fn builder(&self) -> WalkBuilder
- sym:19667 Parents (crates/ignore/src/dir.rs:731-731)
  pub(crate) struct Parents<'a>
- sym:19642 IgnoreMatch.gitignore (crates/ignore/src/dir.rs:56-58)
  fn gitignore(x: &'a gitignore::Glob) -> IgnoreMatch<'a>
- sym:19727 Gitignore.new (crates/ignore/src/gitignore.rs:103-118)
  pub fn new<P: AsRef<Path>>( gitignore_path: P, ) -> (Gitignore, Option<Error>)
- sym:19647 IgnoreInner (crates/ignore/src/dir.rs:115-166)
  struct IgnoreInner
- sym:19771 CachedDir (crates/ignore/src/incremental.rs:94-103)
  enum CachedDir
- sym:19641 IgnoreMatch.overrides (crates/ignore/src/dir.rs:52-54)
  fn overrides(x: overrides::Glob<'a>) -> IgnoreMatch<'a>
- sym:19651 Ignore.add_parents (crates/ignore/src/dir.rs:192-258)
  pub(crate) fn add_parents<P: AsRef<Path>>( &self, path: P, ) -> (Ignore, Option<Error>)
- sym:18731 NoIgnoreParent.name_long (crates/core/flags/defs.rs:4638-4640)
  fn name_long(&self) -> &'static str
- sym:19639 IgnoreMatch (crates/ignore/src/dir.rs:38-38)
  pub(crate) struct IgnoreMatch<'a>
- sym:19649 Ignore.is_root (crates/ignore/src/dir.rs:176-178)
  pub(crate) fn is_root(&self) -> bool
- sym:18734 NoIgnoreParent.doc_short (crates/core/flags/defs.rs:4647-4649)
  fn doc_short(&self) -> &'static str
- sym:20016 WalkBuilder.get_or_set_current_dir (crates/ignore/src/walk.rs:1080-1101)
  fn get_or_set_current_dir(&self) -> Option<&Path>
- sym:19660 Ignore.matched_ignore (crates/ignore/src/dir.rs:548-685)
  pub(crate) fn matched_ignore<'a>( &'a self, path: &Path, is_dir: bool, ) -> Match<IgnoreMatch<'a>>
- sym:19713 absolute_parent (crates/ignore/src/dir.rs:1388-1408)
  fn absolute_parent()
- sym:19820 parent_loading_can_be_disabled (crates/ignore/src/incremental.rs:958-976)
  fn parent_loading_can_be_disabled()
- sym:19714 absolute_parent_anchored (crates/ignore/src/dir.rs:1411-1428)
  fn absolute_parent_anchored()
- sym:19816 cached_parent_matcher_is_used_for_directory (crates/ignore/src/incremental.rs:863-877)
  fn cached_parent_matcher_is_used_for_directory()
- sym:19814 cached_allowed_parent_matches_children (crates/ignore/src/incremental.rs:833-845)
  fn cached_allowed_parent_matches_children()
- sym:19812 respects_git_repository_boundaries (crates/ignore/src/incremental.rs:788-806)
  fn respects_git_repository_boundaries()
- sym:19815 cached_ignored_parent_matches_children (crates/ignore/src/incremental.rs:848-860)
  fn cached_ignored_parent_matches_children()
- sym:19806 parent_ignore_and_custom_ignore (crates/ignore/src/incremental.rs:679-695)
  fn parent_ignore_and_custom_ignore()
- sym:20100 gitignore_parent (crates/ignore/src/walk.rs:2436-2446)
  fn gitignore_parent()
- sym:19715 absolute_parent_matchers_are_cached_across_roots (crates/ignore/src/dir.rs:1431-1451)
  fn absolute_parent_matchers_are_cached_across_roots()
- sym:19803 nested_parent_gitignores (crates/ignore/src/incremental.rs:629-643)
  fn nested_parent_gitignores()
- sym:19390 Glob.as_ref (crates/globset/src/glob.rs:84-86)
  fn as_ref(&self) -> &Glob
- sym:19591 Workspace.write (crates/ignore/benches/workloads.rs:88-92)
  fn write(&self, relative: &str, contents: &str)
- sym:19656 Ignore.add_child_path_with_found_ignore_files (crates/ignore/src/dir.rs:340-467)
  fn add_child_path_with_found_ignore_files( &self, dir: &Path, ignore_files_list: Option<&IgnoreFilesFound>, ) -> (IgnoreInner, Option<Error>)
- sym:19662 Ignore.absolute_base (crates/ignore/src/dir.rs:694-696)
  fn absolute_base(&self) -> Option<&Path>
- sym:19673 IgnoreBuilder.build_with_cwd (crates/ignore/src/dir.rs:817-862)
  pub(crate) fn build_with_cwd(&self, cwd: Option<PathBuf>) -> Ignore
- sym:19778 IncrementalIgnore.matched_with_errors_ignore (crates/ignore/src/incremental.rs:310-391)
  fn matched_with_errors_ignore( &mut self, relative: &Path, is_dir: bool, errs: &mut PartialErrorBuilder, ) -> Option<IncrementalMatch>
- sym:19837 Error.clone (crates/ignore/src/lib.rs:123-153)
  fn clone(&self) -> Error
- sym:19861 Match.inner (crates/ignore/src/lib.rs:464-470)
  pub fn inner(&self) -> Option<&T>
- sym:19862 Match.map (crates/ignore/src/lib.rs:475-481)
  pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Match<U>
- sym:19900 is_file_name (crates/ignore/src/pathutil.rs:123-136)
  pub(crate) fn is_file_name<P: AsRef<Path>>(path: P) -> bool
- sym:19917 Selection.map (crates/ignore/src/types.rs:205-214)
  fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Selection<U>
- sym:19918 Selection.inner (crates/ignore/src/types.rs:216-221)
  fn inner(&self) -> &T
- sym:20023 Walk.next (crates/ignore/src/walk.rs:1190-1261)
  fn next(&mut self) -> Option<Result<DirEntry, Error>>
- sym:20121 test_dirs_in_deep (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:193-318)
  fn test_dirs_in_deep()
- sym:18737 test_no_ignore_parent (crates/core/flags/defs.rs:4667-4677)
  fn test_no_ignore_parent()
- sym:19817 compiled_multi_matchers_are_reused (crates/ignore/src/incremental.rs:881-898)
  fn compiled_multi_matchers_are_reused()
- sym:20119 test_files_in_deep (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:57-86)
  fn test_files_in_deep()
- sym:18683 test_no_ignore (crates/core/flags/defs.rs:4368-4389)
  fn test_no_ignore()
- sym:19804 anchored_child_rule_uses_child_root (crates/ignore/src/incremental.rs:648-657)
  fn anchored_child_rule_uses_child_root()
- sym:19808 glob_overrides_apply_to_ancestors (crates/ignore/src/incremental.rs:717-729)
  fn glob_overrides_apply_to_ancestors()
- sym:20116 get_gitignore (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:8-13)
  fn get_gitignore() -> Gitignore
- sym:20109 no_read_permissions (crates/ignore/src/walk.rs:2658-2673)
  fn no_read_permissions()
- sym:21356 Dir.new (tests/util.rs:73-87)
  pub fn new(name: &str) -> Dir

---
tokens: 2743/8000
