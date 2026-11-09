## Task
# ripgrep-6892bc2 · ripgrep-6892bc2 · open

## Map
- repo map: .cargo/ .github/ .gitignore .ignore .nvim.lua AI_POLICY.md CHANGELOG.md CONTRIBUTING.md COPYING Cargo.lock Cargo.toml FAQ.md (226 files)

## Code
- sym:13040 IncrementalIgnore.matched_with_errors_ignore (crates/ignore/src/incremental.rs:310-391)
  fn matched_with_errors_ignore( &mut self, relative: &Path, is_dir: bool, errs: &mut PartialErrorBuilder, ) -> Option<IncrementalMatch>
- sym:12876 incremental_ignored_parent (crates/ignore/benches/workloads.rs:380-388)
  fn incremental_ignored_parent(b: &mut test::Bencher)
- sym:13055 normalize_absolute (crates/ignore/src/incremental.rs:542-555)
  fn normalize_absolute(path: &Path) -> Option<PathBuf>
- sym:13041 IncrementalIgnore.match_path (crates/ignore/src/incremental.rs:393-411)
  fn match_path( &self, ignore: &Ignore, relative: &Path, is_dir: bool, ) -> IncrementalMatch
- sym:13224 DirEntryRaw (crates/ignore/src/walk.rs:238-256)
  struct DirEntryRaw
- sym:12907 IgnoreInner (crates/ignore/src/dir.rs:115-169)
  struct IgnoreInner
- sym:13033 CachedDir (crates/ignore/src/incremental.rs:94-103)
  enum CachedDir
- sym:13039 IncrementalIgnore.matched_with_errors_impl (crates/ignore/src/incremental.rs:224-308)
  fn matched_with_errors_impl( &mut self, relative: &Path, is_dir: bool, errs: &mut PartialErrorBuilder, ) -> IncrementalMatch
- sym:13037 IncrementalIgnore.matched (crates/ignore/src/incremental.rs:194-204)
  pub fn matched<P: AsRef<Path>>( &mut self, path: P, is_dir: bool, ) -> IncrementalMatch
- sym:13078 cached_parent_matcher_is_used_for_directory (crates/ignore/src/incremental.rs:863-877)
  fn cached_parent_matcher_is_used_for_directory()
- sym:11293 DecompressionMatcherBuilder.new (crates/cli/src/decompress.rs:42-44)
  pub fn new() -> DecompressionMatcherBuilder
- sym:11300 DecompressionMatcher.new (crates/cli/src/decompress.rs:167-171)
- sym:11304 DecompressionReaderBuilder.new (crates/cli/src/decompress.rs:205-207)
- sym:11310 DecompressionReader.new (crates/cli/src/decompress.rs:352-356)
- sym:13068 parent_ignore_and_custom_ignore (crates/ignore/src/incremental.rs:679-695)
- sym:13076 cached_allowed_parent_matches_children (crates/ignore/src/incremental.rs:833-845)
- sym:13077 cached_ignored_parent_matches_children (crates/ignore/src/incremental.rs:848-860)

---
tokens: 684/800
