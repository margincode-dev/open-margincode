## Task
# ripgrep-6892bc2 · ripgrep-6892bc2 · open

## Map
- repo map: .cargo/ .github/ .gitignore .ignore .nvim.lua AI_POLICY.md CHANGELOG.md CONTRIBUTING.md COPYING Cargo.lock Cargo.toml FAQ.md (226 files)

## Code
- sym:13040 IncrementalIgnore.matched_with_errors_ignore (crates/ignore/src/incremental.rs:310-391)
  fn matched_with_errors_ignore( &mut self, relative: &Path, is_dir: bool, errs: &mut PartialErrorBuilder, ) -> Option<IncrementalMatch>
- sym:13055 normalize_absolute (crates/ignore/src/incremental.rs:542-555)
  fn normalize_absolute(path: &Path) -> Option<PathBuf>
- sym:13033 CachedDir (crates/ignore/src/incremental.rs:94-103)
  enum CachedDir
- sym:13224 DirEntryRaw (crates/ignore/src/walk.rs:238-256)
  struct DirEntryRaw
- sym:12907 IgnoreInner (crates/ignore/src/dir.rs:115-169)
  struct IgnoreInner
- sym:13030 IncrementalIgnore (crates/ignore/src/incremental.rs:57-68)
  pub struct IncrementalIgnore
- sym:13039 IncrementalIgnore.matched_with_errors_impl (crates/ignore/src/incremental.rs:224-308)
  fn matched_with_errors_impl( &mut self, relative: &Path, is_dir: bool, errs: &mut PartialErrorBuilder, ) -> IncrementalMatch
- sym:12876 incremental_ignored_parent (crates/ignore/benches/workloads.rs:380-388)
  fn incremental_ignored_parent(b: &mut test::Bencher)
- sym:12906 Ignore (crates/ignore/src/dir.rs:94-112)
  pub(crate) struct Ignore
- sym:13078 cached_parent_matcher_is_used_for_directory (crates/ignore/src/incremental.rs:863-877)
  fn cached_parent_matcher_is_used_for_directory()
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
- sym:13077 cached_ignored_parent_matches_children (crates/ignore/src/incremental.rs:848-860)
- sym:13076 cached_allowed_parent_matches_children (crates/ignore/src/incremental.rs:833-845)

---
tokens: 687/800
