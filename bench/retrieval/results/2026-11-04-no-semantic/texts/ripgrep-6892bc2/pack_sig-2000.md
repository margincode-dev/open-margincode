## Task
# ripgrep-6892bc2 · ripgrep-6892bc2-2 · open
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
Workspace: /root/bench/ws/rebf815-ripgrep-6892bc2/ripgrep-6892bc2-2
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
- sym:13055 normalize_absolute (crates/ignore/src/incremental.rs:542-555)
  fn normalize_absolute(path: &Path) -> Option<PathBuf>
- sym:13224 DirEntryRaw (crates/ignore/src/walk.rs:238-256)
  struct DirEntryRaw
- sym:12907 IgnoreInner (crates/ignore/src/dir.rs:115-169)
  struct IgnoreInner
- sym:13039 IncrementalIgnore.matched_with_errors_impl (crates/ignore/src/incremental.rs:224-308)
  fn matched_with_errors_impl( &mut self, relative: &Path, is_dir: bool, errs: &mut PartialErrorBuilder, ) -> IncrementalMatch
- sym:12876 incremental_ignored_parent (crates/ignore/benches/workloads.rs:380-388)
  fn incremental_ignored_parent(b: &mut test::Bencher)
- sym:11293 DecompressionMatcherBuilder.new (crates/cli/src/decompress.rs:42-44)
  pub fn new() -> DecompressionMatcherBuilder
- sym:11300 DecompressionMatcher.new (crates/cli/src/decompress.rs:167-171)
  pub fn new() -> DecompressionMatcher
- sym:11304 DecompressionReaderBuilder.new (crates/cli/src/decompress.rs:205-207)
  pub fn new() -> DecompressionReaderBuilder
- sym:11310 DecompressionReader.new (crates/cli/src/decompress.rs:352-356)
  pub fn new<P: AsRef<Path>>( path: P, ) -> Result<DecompressionReader, CommandError>
- sym:13068 parent_ignore_and_custom_ignore (crates/ignore/src/incremental.rs:679-695)
  fn parent_ignore_and_custom_ignore()
- sym:13078 cached_parent_matcher_is_used_for_directory (crates/ignore/src/incremental.rs:863-877)
  fn cached_parent_matcher_is_used_for_directory()

---
tokens: 1084/2000
