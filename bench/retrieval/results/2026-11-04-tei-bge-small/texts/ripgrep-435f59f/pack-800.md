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
- sym:32519 Sortr.doc_long (crates/core/flags/defs.rs:6795-6830)
  fn doc_long(&self) -> &'static str
- sym:33164 IgnoreOptions (crates/ignore/src/dir.rs:72-90)
  struct IgnoreOptions
- sym:33207 create_gitignore (crates/ignore/src/dir.rs:1016-1051)
- sym:32809 SortMode.supported (crates/core/flags/lowargs.rs:689-720)
- sym:33544 WalkState (crates/ignore/src/walk.rs:1318-1330)
  pub enum WalkState
- sym:33189 IgnoreBuilder (crates/ignore/src/dir.rs:753-776)
- sym:33636 gitignore_skip_bom (crates/ignore/tests/gitignore_skip_bom.rs:10-17)
- sym:33635 IGNORE_FILE (crates/ignore/tests/gitignore_skip_bom.rs:3-3)
- sym:31548 DecompressionMatcherBuilder.new (crates/cli/src/decompress.rs:42-44)
- sym:31549 DecompressionMatcherBuilder.build (crates/cli/src/decompress.rs:50-69)
- sym:31555 DecompressionMatcher.new (crates/cli/src/decompress.rs:167-171)
- sym:31559 DecompressionReaderBuilder.new (crates/cli/src/decompress.rs:205-207)

---
tokens: 717/800
omitted 41 lower-ranked items (expand an id for detail):
- repo map: .cargo/ .github/ .gitignore .ignore .nvim.lua AI_POLICY.md CHANGELOG.md CONTRIBUTING.md COPYING Cargo.lock Cargo.toml FAQ.md (237 files)
- sym:31560 DecompressionReaderBuilder.build (crates/cli/src/decompress.rs:221-244)
- sym:31565 DecompressionReader.new (crates/cli/src/decompress.rs:352-356)
- sym:31631 CommandReaderBuilder.new (crates/cli/src/process.rs:90-92)
- sym:31632 CommandReaderBuilder.build (crates/cli/src/process.rs:104-118)
- sym:31635 CommandReader.new (crates/cli/src/process.rs:195-199)
- sym:32715 HiArgs (crates/core/flags/hiargs.rs:36-107)
- … and 34 more
