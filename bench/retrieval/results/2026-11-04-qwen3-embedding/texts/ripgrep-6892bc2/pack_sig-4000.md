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
- sym:13055 normalize_absolute (crates/ignore/src/incremental.rs:542-555)
  fn normalize_absolute(path: &Path) -> Option<PathBuf>
- sym:13224 DirEntryRaw (crates/ignore/src/walk.rs:238-256)
  struct DirEntryRaw
- sym:12907 IgnoreInner (crates/ignore/src/dir.rs:115-169)
  struct IgnoreInner
- sym:12906 Ignore (crates/ignore/src/dir.rs:94-112)
  pub(crate) struct Ignore
- sym:13039 IncrementalIgnore.matched_with_errors_impl (crates/ignore/src/incremental.rs:224-308)
  fn matched_with_errors_impl( &mut self, relative: &Path, is_dir: bool, errs: &mut PartialErrorBuilder, ) -> IncrementalMatch
- sym:13037 IncrementalIgnore.matched (crates/ignore/src/incremental.rs:194-204)
  pub fn matched<P: AsRef<Path>>( &mut self, path: P, is_dir: bool, ) -> IncrementalMatch
- sym:13041 IncrementalIgnore.match_path (crates/ignore/src/incremental.rs:393-411)
  fn match_path( &self, ignore: &Ignore, relative: &Path, is_dir: bool, ) -> IncrementalMatch
- sym:13042 IncrementalIgnore.root_ignore (crates/ignore/src/incremental.rs:413-433)
  fn root_ignore( &mut self, errs: &mut PartialErrorBuilder, ) -> Option<Ignore>
- sym:13033 CachedDir (crates/ignore/src/incremental.rs:94-103)
  enum CachedDir
- sym:13034 IncrementalIgnore.new (crates/ignore/src/incremental.rs:106-131)
  pub(crate) fn new( root: PathBuf, ignore: Ignore, options: IncrementalIgnoreOptions, ) -> IncrementalIgnore
- sym:13162 is_file_name (crates/ignore/src/pathutil.rs:123-136)
  pub(crate) fn is_file_name<P: AsRef<Path>>(path: P) -> bool
- sym:13038 IncrementalIgnore.matched_with_errors (crates/ignore/src/incremental.rs:213-222)
  pub fn matched_with_errors<P: AsRef<Path>>( &mut self, path: P, is_dir: bool, ) -> (IncrementalMatch, Option<Error>)
- sym:13163 file_name (crates/ignore/src/pathutil.rs:142-171)
  pub(crate) fn file_name<'a, P: AsRef<Path> + ?Sized>( path: &'a P, ) -> Option<&'a OsStr>
- sym:13078 cached_parent_matcher_is_used_for_directory (crates/ignore/src/incremental.rs:863-877)
  fn cached_parent_matcher_is_used_for_directory()
- sym:13076 cached_allowed_parent_matches_children (crates/ignore/src/incremental.rs:833-845)
  fn cached_allowed_parent_matches_children()
- sym:13077 cached_ignored_parent_matches_children (crates/ignore/src/incremental.rs:848-860)
  fn cached_ignored_parent_matches_children()
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
- sym:13068 parent_ignore_and_custom_ignore (crates/ignore/src/incremental.rs:679-695)
  fn parent_ignore_and_custom_ignore()
- sym:12977 absolute_parent_matchers_are_cached_across_roots (crates/ignore/src/dir.rs:1486-1506)
  fn absolute_parent_matchers_are_cached_across_roots()
- sym:13080 compiled_multi_child_matchers_are_not_reused (crates/ignore/src/incremental.rs:901-936)
  fn compiled_multi_child_matchers_are_not_reused()
- sym:13082 parent_loading_can_be_disabled (crates/ignore/src/incremental.rs:958-976)
  fn parent_loading_can_be_disabled()
- sym:13072 directly_ignored_directory_is_not_descended (crates/ignore/src/incremental.rs:751-761)
  fn directly_ignored_directory_is_not_descended()
- sym:13079 compiled_multi_matchers_are_reused (crates/ignore/src/incremental.rs:881-898)
  fn compiled_multi_matchers_are_reused()
- sym:13096 whitelisted_hidden_directory_allows_descendants (crates/ignore/src/incremental.rs:1253-1265)
  fn whitelisted_hidden_directory_allows_descendants()
- sym:13065 nested_parent_gitignores (crates/ignore/src/incremental.rs:629-643)
  fn nested_parent_gitignores()
- sym:11321 b (crates/cli/src/escape.rs:85-87)
  fn b(bytes: &'static [u8]) -> Vec<u8>

---
tokens: 2013/4000
