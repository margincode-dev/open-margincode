## Task
# ripgrep-6892bc2 · ripgrep-6892bc2 · open
Goal: ignore: use byte keys for incremental parent lookups

We now store incremental directory-cache keys as OsString values to
avoid path-component hashing. On Unix, we find a cached parent by
scanning path bytes before parsing components. Cache misses fall back
to the component walk, preserving alternate path spellings and cached
ignore-file snapshots.

On Linux x86-64 against 3fce3b5, warm incremental replay over Ruff
and uv path sets improves 12–13% and 15–18%, respectively, across
default and fat-LTO builds; ignored-parent checks improve about 68%.
Paths with an internal ./ are 0.5–2.2% slower.

PR #3559
Workspace: /root/bench/ws/rebf815-ripgrep-6892bc2/ripgrep-6892bc2
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
- sym:13040 IncrementalIgnore.matched_with_errors_ignore (crates/ignore/src/incremental.rs:310-391)
  fn matched_with_errors_ignore( &mut self, relative: &Path, is_dir: bool, errs: &mut PartialErrorBuilder, ) -> Option<IncrementalMatch>
- sym:12876 incremental_ignored_parent (crates/ignore/benches/workloads.rs:380-388)
  fn incremental_ignored_parent(b: &mut test::Bencher)
- sym:13030 IncrementalIgnore (crates/ignore/src/incremental.rs:57-68)
  pub struct IncrementalIgnore
- sym:12907 IgnoreInner (crates/ignore/src/dir.rs:115-169)
  struct IgnoreInner
- sym:13033 CachedDir (crates/ignore/src/incremental.rs:94-103)
  enum CachedDir
- sym:13034 IncrementalIgnore.new (crates/ignore/src/incremental.rs:106-131)
  pub(crate) fn new( root: PathBuf, ignore: Ignore, options: IncrementalIgnoreOptions, ) -> IncrementalIgnore
- sym:13042 IncrementalIgnore.root_ignore (crates/ignore/src/incremental.rs:413-433)
  fn root_ignore( &mut self, errs: &mut PartialErrorBuilder, ) -> Option<Ignore>
- sym:13055 normalize_absolute (crates/ignore/src/incremental.rs:542-555)
  fn normalize_absolute(path: &Path) -> Option<PathBuf>
- sym:13224 DirEntryRaw (crates/ignore/src/walk.rs:238-256)
  struct DirEntryRaw
- sym:12906 Ignore (crates/ignore/src/dir.rs:94-112)
  pub(crate) struct Ignore
- sym:13039 IncrementalIgnore.matched_with_errors_impl (crates/ignore/src/incremental.rs:224-308)
  fn matched_with_errors_impl( &mut self, relative: &Path, is_dir: bool, errs: &mut PartialErrorBuilder, ) -> IncrementalMatch
- sym:13037 IncrementalIgnore.matched (crates/ignore/src/incremental.rs:194-204)
  pub fn matched<P: AsRef<Path>>( &mut self, path: P, is_dir: bool, ) -> IncrementalMatch
- sym:13041 IncrementalIgnore.match_path (crates/ignore/src/incremental.rs:393-411)
  fn match_path( &self, ignore: &Ignore, relative: &Path, is_dir: bool, ) -> IncrementalMatch
- sym:13162 is_file_name (crates/ignore/src/pathutil.rs:123-136)
  pub(crate) fn is_file_name<P: AsRef<Path>>(path: P) -> bool
- sym:13038 IncrementalIgnore.matched_with_errors (crates/ignore/src/incremental.rs:213-222)
  pub fn matched_with_errors<P: AsRef<Path>>( &mut self, path: P, is_dir: bool, ) -> (IncrementalMatch, Option<Error>)
- sym:13163 file_name (crates/ignore/src/pathutil.rs:142-171)
  pub(crate) fn file_name<'a, P: AsRef<Path> + ?Sized>( path: &'a P, ) -> Option<&'a OsStr>
- sym:12891 incremental_gitignore_only (crates/ignore/benches/workloads.rs:563-576)
  fn incremental_gitignore_only(b: &mut test::Bencher)
- sym:13338 path_equals (crates/ignore/src/walk.rs:2123-2142)
  fn path_equals(dent: &DirEntry, handle: &Handle) -> Result<bool, Error>
- sym:12868 incremental_warm_noncanonical (crates/ignore/benches/workloads.rs:271-290)
  fn incremental_warm_noncanonical(b: &mut test::Bencher)
- sym:12911 Ignore.add_parents (crates/ignore/src/dir.rs:195-262)
  pub(crate) fn add_parents<P: AsRef<Path>>( &self, path: P, ) -> (Ignore, Option<Error>)
- sym:12666 GlobStrategic.is_match_candidate (crates/globset/src/glob.rs:175-201)
  fn is_match_candidate(&self, candidate: &Candidate<'_>) -> bool
- sym:12766 LiteralStrategy (crates/globset/src/lib.rs:722-726)
  struct LiteralStrategy
- sym:13032 RootIgnore (crates/ignore/src/incremental.rs:82-87)
  enum RootIgnore
- sym:13046 IncrementalMatch.from_match (crates/ignore/src/incremental.rs:472-475)
  fn from_match(mat: Match<()>, is_dir: bool) -> IncrementalMatch
- sym:12867 incremental_warm (crates/ignore/benches/workloads.rs:257-268)
  fn incremental_warm(b: &mut test::Bencher)
- sym:12930 IgnoreBuilder (crates/ignore/src/dir.rs:758-781)
  pub(crate) struct IgnoreBuilder
- sym:13031 IncrementalIgnoreOptions (crates/ignore/src/incremental.rs:73-79)
  pub(crate) struct IncrementalIgnoreOptions
- sym:12920 Ignore.matched_ignore (crates/ignore/src/dir.rs:553-690)
  pub(crate) fn matched_ignore<'a>( &'a self, path: &Path, is_dir: bool, ) -> Match<IgnoreMatch<'a>>
- sym:12875 incremental_ignored_nested (crates/ignore/benches/workloads.rs:366-377)
  fn incremental_ignored_nested(b: &mut test::Bencher)
- sym:13078 cached_parent_matcher_is_used_for_directory (crates/ignore/src/incremental.rs:863-877)
  fn cached_parent_matcher_is_used_for_directory()
- sym:13076 cached_allowed_parent_matches_children (crates/ignore/src/incremental.rs:833-845)
  fn cached_allowed_parent_matches_children()
- sym:13077 cached_ignored_parent_matches_children (crates/ignore/src/incremental.rs:848-860)
  fn cached_ignored_parent_matches_children()
- sym:13068 parent_ignore_and_custom_ignore (crates/ignore/src/incremental.rs:679-695)
  fn parent_ignore_and_custom_ignore()
- sym:13082 parent_loading_can_be_disabled (crates/ignore/src/incremental.rs:958-976)
  fn parent_loading_can_be_disabled()
- sym:13073 ignored_ancestor_wins (crates/ignore/src/incremental.rs:767-783)
  fn ignored_ancestor_wins()
- sym:11293 DecompressionMatcherBuilder.new (crates/cli/src/decompress.rs:42-44)
  pub fn new() -> DecompressionMatcherBuilder
- sym:11300 DecompressionMatcher.new (crates/cli/src/decompress.rs:167-171)
  pub fn new() -> DecompressionMatcher
- sym:11304 DecompressionReaderBuilder.new (crates/cli/src/decompress.rs:205-207)
  pub fn new() -> DecompressionReaderBuilder
- sym:11310 DecompressionReader.new (crates/cli/src/decompress.rs:352-356)
  pub fn new<P: AsRef<Path>>( path: P, ) -> Result<DecompressionReader, CommandError>
- sym:11376 CommandReaderBuilder.new (crates/cli/src/process.rs:90-92)
  pub fn new() -> CommandReaderBuilder
- sym:11380 CommandReader.new (crates/cli/src/process.rs:195-199)
  pub fn new( cmd: &mut process::Command, ) -> Result<CommandReader, CommandError>
- sym:12449 State.new (crates/core/flags/hiargs.rs:950-960)
  fn new() -> anyhow::Result<State>
- sym:12457 BinaryDetection.is_none (crates/core/flags/hiargs.rs:1179-1182)
  pub(crate) fn is_none(&self) -> bool
- sym:12492 ContextModeLimited.get (crates/core/flags/lowargs.rs:452-464)
  pub(crate) fn get(&self) -> (usize, usize)
- sym:12495 ContextSeparator.new (crates/core/flags/lowargs.rs:483-491)
  pub(crate) fn new(os: &OsStr) -> anyhow::Result<ContextSeparator>
- sym:12502 FieldContextSeparator.new (crates/core/flags/lowargs.rs:561-569)
  pub(crate) fn new(os: &OsStr) -> anyhow::Result<FieldContextSeparator>
- sym:12506 FieldMatchSeparator.new (crates/core/flags/lowargs.rs:595-603)
  pub(crate) fn new(os: &OsStr) -> anyhow::Result<FieldMatchSeparator>
- sym:12542 Parser.new (crates/core/flags/parse.rs:176-216)
  fn new() -> &'static Parser
- sym:12551 FlagMap.new (crates/core/flags/parse.rs:374-387)
  fn new(infos: &[FlagInfo]) -> FlagMap
- sym:12559 HaystackBuilder.new (crates/core/haystack.rs:22-24)
  pub(crate) fn new() -> HaystackBuilder
- sym:12977 absolute_parent_matchers_are_cached_across_roots (crates/ignore/src/dir.rs:1486-1506)
  fn absolute_parent_matchers_are_cached_across_roots()
- sym:13080 compiled_multi_child_matchers_are_not_reused (crates/ignore/src/incremental.rs:901-936)
  fn compiled_multi_child_matchers_are_not_reused()
- sym:13072 directly_ignored_directory_is_not_descended (crates/ignore/src/incremental.rs:751-761)
  fn directly_ignored_directory_is_not_descended()
- sym:13079 compiled_multi_matchers_are_reused (crates/ignore/src/incremental.rs:881-898)
  fn compiled_multi_matchers_are_reused()
- sym:13096 whitelisted_hidden_directory_allows_descendants (crates/ignore/src/incremental.rs:1253-1265)
  fn whitelisted_hidden_directory_allows_descendants()
- sym:13065 nested_parent_gitignores (crates/ignore/src/incremental.rs:629-643)
  fn nested_parent_gitignores()
- sym:12973 absolute_parent (crates/ignore/src/dir.rs:1394-1414)
  fn absolute_parent()
- sym:13087 symlink_path_stays_under_lexical_root (crates/ignore/src/incremental.rs:1064-1078)
  fn symlink_path_stays_under_lexical_root()
- sym:13075 compiled_matchers_are_reused (crates/ignore/src/incremental.rs:811-830)
  fn compiled_matchers_are_reused()
- sym:13070 glob_overrides_apply_to_ancestors (crates/ignore/src/incremental.rs:717-729)
  fn glob_overrides_apply_to_ancestors()
- sym:13064 matchedd (crates/ignore/src/incremental.rs:618-625)
  fn matchedd<P: AsRef<Path>>( matcher: &mut IncrementalIgnore, path: P, ) -> IncrementalMatch
- sym:13092 max_filesize_respects_follow_links (crates/ignore/src/incremental.rs:1173-1191)
  fn max_filesize_respects_follow_links()
- sym:13063 matchedf (crates/ignore/src/incremental.rs:609-616)
  fn matchedf<P: AsRef<Path>>( matcher: &mut IncrementalIgnore, path: P, ) -> IncrementalMatch
- sym:13348 normal_path (crates/ignore/src/walk.rs:2224-2226)
  fn normal_path(unix: &str) -> String
- sym:13383 test_dirs_in_deep (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:193-318)
  fn test_dirs_in_deep()
- sym:13381 test_files_in_deep (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:57-86)
  fn test_files_in_deep()
- sym:11321 b (crates/cli/src/escape.rs:85-87)
  fn b(bytes: &'static [u8]) -> Vec<u8>

---
tokens: 3260/8000
