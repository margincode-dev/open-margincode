## Task
# ripgrep-435f59f · ripgrep-435f59f · open
Goal: ignore: skip loading unreachable ignore files

When parallelism isn't used (e.g., when sorting results), then
it's possible in some cases for the `ignore` crate to still try and
load gitignore files inside of directories that are never
descended into. This can occur when `--max-depth` is used, or
even just when a directory is skipped for some other reason.

Fixes #2565, PR #3496

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
- repo map (237 files):
  .cargo/ (1 files)
  .github/ (6 files)
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
  build.rs
  ci/ (4 files)
  crates/ (147 files)
  fuzz/ (5 files)
  pkg/ (3 files)
  rustfmt.toml
  scripts/ (1 files)
  tests/ (22 files)

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
  fn doc_long(&self) -> &'static str
- sym:33298 IncrementalIgnore.match_path (crates/ignore/src/incremental.rs:393-411)
  fn match_path( &self, ignore: &Ignore, relative: &Path, is_dir: bool, ) -> IncrementalMatch
- sym:33290 CachedDir (crates/ignore/src/incremental.rs:94-103)
  enum CachedDir
- sym:32809 SortMode.supported (crates/core/flags/lowargs.rs:689-720)
  pub(crate) fn supported(&self) -> anyhow::Result<()>
- sym:33389 GlobInner (crates/ignore/src/overrides.rs:32-37)
  enum GlobInner<'a>
- sym:33296 IncrementalIgnore.matched_with_errors_impl (crates/ignore/src/incremental.rs:224-308)
  fn matched_with_errors_impl( &mut self, relative: &Path, is_dir: bool, errs: &mut PartialErrorBuilder, ) -> IncrementalMatch
- sym:33544 WalkState (crates/ignore/src/walk.rs:1318-1330)
  pub enum WalkState
- sym:33291 IncrementalIgnore.new (crates/ignore/src/incremental.rs:106-131)
- sym:33189 IgnoreBuilder (crates/ignore/src/dir.rs:753-776)
  pub(crate) struct IgnoreBuilder
- sym:33302 IncrementalMatch.ignore (crates/ignore/src/incremental.rs:464-470)
- sym:33303 IncrementalMatch.from_match (crates/ignore/src/incremental.rs:472-475)
- sym:31548 DecompressionMatcherBuilder.new (crates/cli/src/decompress.rs:42-44)
- sym:31549 DecompressionMatcherBuilder.build (crates/cli/src/decompress.rs:50-69)
- sym:31555 DecompressionMatcher.new (crates/cli/src/decompress.rs:167-171)
- sym:31559 DecompressionReaderBuilder.new (crates/cli/src/decompress.rs:205-207)
- sym:31560 DecompressionReaderBuilder.build (crates/cli/src/decompress.rs:221-244)
- sym:31565 DecompressionReader.new (crates/cli/src/decompress.rs:352-356)
- sym:31631 CommandReaderBuilder.new (crates/cli/src/process.rs:90-92)
- sym:31632 CommandReaderBuilder.build (crates/cli/src/process.rs:104-118)
- sym:31635 CommandReader.new (crates/cli/src/process.rs:195-199)
- sym:32715 HiArgs (crates/core/flags/hiargs.rs:36-107)
- sym:32738 HiArgs.types (crates/core/flags/hiargs.rs:872-874)
- sym:32744 State.new (crates/core/flags/hiargs.rs:986-996)
- sym:32752 BinaryDetection.is_none (crates/core/flags/hiargs.rs:1215-1218)
- sym:32753 types (crates/core/flags/hiargs.rs:1222-1242)
- sym:32754 globs (crates/core/flags/hiargs.rs:1246-1267)
- sym:32755 preprocessor_globs (crates/core/flags/hiargs.rs:1270-1282)
- sym:32791 ContextSeparator.new (crates/core/flags/lowargs.rs:495-503)
- sym:32798 FieldContextSeparator.new (crates/core/flags/lowargs.rs:573-581)
- sym:32802 FieldMatchSeparator.new (crates/core/flags/lowargs.rs:607-615)
- sym:32839 Parser.new (crates/core/flags/parse.rs:176-216)
- sym:32848 FlagMap.new (crates/core/flags/parse.rs:374-387)
- sym:32856 HaystackBuilder.new (crates/core/haystack.rs:22-24)
- sym:32857 HaystackBuilder.build_from_result (crates/core/haystack.rs:31-42)
- sym:32858 HaystackBuilder.build (crates/core/haystack.rs:49-79)
- sym:32860 Haystack (crates/core/haystack.rs:98-101)
- sym:32861 Haystack.path (crates/core/haystack.rs:108-114)
- sym:32897 Config (crates/core/search.rs:19-25)
  struct Config
- sym:32898 Config.default (crates/core/search.rs:28-36)
- sym:32901 SearchWorkerBuilder.new (crates/core/search.rs:54-59)

---
tokens: 1787/2000
omitted 13 lower-ranked items (expand an id for detail):
- sym:32902 SearchWorkerBuilder.build (crates/core/search.rs:63-85)
- sym:32904 SearchWorkerBuilder.preprocessor_globs (crates/core/search.rs:108-114)
- sym:32952 MatchStrategy.new (crates/globset/src/glob.rs:51-67)
- sym:32976 Glob.new (crates/globset/src/glob.rs:283-285)
- sym:32988 GlobBuilder.new (crates/globset/src/glob.rs:574-576)
- sym:32989 GlobBuilder.build (crates/globset/src/glob.rs:579-610)
- sym:33050 GlobSet.new (crates/globset/src/lib.rs:461-553)
- sym:33053 GlobSetBuilder.new (crates/globset/src/lib.rs:574-576)
- sym:33054 GlobSetBuilder.build (crates/globset/src/lib.rs:581-583)
- sym:33058 Candidate.new (crates/globset/src/lib.rs:617-619)
- … and 3 more
