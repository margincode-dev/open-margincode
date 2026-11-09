## Task
# ripgrep-435f59f · ripgrep-435f59f · open

## Map
- crates/ignore/src/dir.rs
- crates/cli/src/decompress.rs
- crates/cli/src/process.rs
- crates/core/flags/hiargs.rs
- crates/core/flags/lowargs.rs
- crates/core/flags/parse.rs
- crates/core/haystack.rs
- crates/core/search.rs
- crates/globset/src/glob.rs
- crates/globset/src/lib.rs

## Code
- sym:32739 HiArgs.walk_builder (crates/core/flags/hiargs.rs:885-929)
  pub(crate) fn walk_builder(&self) -> anyhow::Result<ignore::WalkBuilder>
- sym:33165 Ignore (crates/ignore/src/dir.rs:94-112)
  pub(crate) struct Ignore
- sym:32509 Sort.doc_long (crates/core/flags/defs.rs:6694-6728)
  fn doc_long(&self) -> &'static str
- sym:33301 IncrementalMatch.none (crates/ignore/src/incremental.rs:456-462)
  fn none(is_dir: bool) -> IncrementalMatch
- sym:32519 Sortr.doc_long (crates/core/flags/defs.rs:6795-6830)
- sym:33298 IncrementalIgnore.match_path (crates/ignore/src/incremental.rs:393-411)
- sym:33290 CachedDir (crates/ignore/src/incremental.rs:94-103)
- sym:32809 SortMode.supported (crates/core/flags/lowargs.rs:689-720)
- sym:33389 GlobInner (crates/ignore/src/overrides.rs:32-37)
- sym:33296 IncrementalIgnore.matched_with_errors_impl (crates/ignore/src/incremental.rs:224-308)
- sym:33544 WalkState (crates/ignore/src/walk.rs:1318-1330)
- sym:33291 IncrementalIgnore.new (crates/ignore/src/incremental.rs:106-131)
- sym:33189 IgnoreBuilder (crates/ignore/src/dir.rs:753-776)
- sym:33302 IncrementalMatch.ignore (crates/ignore/src/incremental.rs:464-470)
- sym:33303 IncrementalMatch.from_match (crates/ignore/src/incremental.rs:472-475)

---
tokens: 700/800
omitted 43 lower-ranked items (expand an id for detail):
- repo map: .cargo/ .github/ .gitignore .ignore .nvim.lua AI_POLICY.md CHANGELOG.md CONTRIBUTING.md COPYING Cargo.lock Cargo.toml FAQ.md (237 files)
- sym:31548 DecompressionMatcherBuilder.new (crates/cli/src/decompress.rs:42-44)
- sym:31549 DecompressionMatcherBuilder.build (crates/cli/src/decompress.rs:50-69)
- sym:31555 DecompressionMatcher.new (crates/cli/src/decompress.rs:167-171)
- sym:31559 DecompressionReaderBuilder.new (crates/cli/src/decompress.rs:205-207)
- sym:31560 DecompressionReaderBuilder.build (crates/cli/src/decompress.rs:221-244)
- … and 37 more
