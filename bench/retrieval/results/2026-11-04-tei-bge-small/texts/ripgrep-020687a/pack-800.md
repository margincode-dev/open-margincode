## Task
# ripgrep-020687a · ripgrep-020687a · open

## Map
- crates/core/flags/defs.rs
- crates/core/main.rs
- crates/globset/src/lib.rs
- crates/ignore/src/dir.rs
- crates/cli/src/decompress.rs
- crates/cli/src/process.rs
- crates/cli/src/wtr.rs
- crates/core/flags/config.rs
- crates/core/flags/hiargs.rs
- crates/core/flags/lowargs.rs
- crates/core/flags/parse.rs
- crates/core/haystack.rs

## Code
- sym:28425 Files (crates/core/flags/defs.rs:2208-2208)
  struct Files
- sym:29420 files (crates/core/main.rs:241-267)
  fn files(args: &HiArgs) -> anyhow::Result<bool>
- sym:29576 GlobSet (crates/globset/src/lib.rs:309-312)
  pub struct GlobSet
- sym:29704 Ignore (crates/ignore/src/dir.rs:94-112)
  pub(crate) struct Ignore
- sym:29639 RegexSetStrategy.find_matches (crates/globset/src/lib.rs:986-995)
  fn find_matches( &self, candidate: &Candidate<'_>, ) -> PoolGuard<'_, PatternSet, PatternSetPoolFn>
- sym:29784 Gitignore (crates/ignore/src/gitignore.rs:81-88)
  pub struct Gitignore
- sym:29653 escape (crates/globset/src/lib.rs:1115-1132)
- sym:29705 IgnoreInner (crates/ignore/src/dir.rs:115-166)
- sym:29575 new_regex_set (crates/globset/src/lib.rs:287-304)
- sym:29795 Gitignore.matched_stripped (crates/ignore/src/gitignore.rs:259-282)
- sym:29574 new_regex (crates/globset/src/lib.rs:271-285)
- sym:29718 Ignore.matched_ignore (crates/ignore/src/dir.rs:548-685)
- sym:29647 MultiStrategyBuilder.regex_set (crates/globset/src/lib.rs:1051-1061)
- sym:29700 IgnoreMatch.gitignore (crates/ignore/src/dir.rs:56-58)
- sym:31346 Searcher (crates/searcher/src/searcher/mod.rs:597-625)

---
tokens: 709/800
omitted 47 lower-ranked items (expand an id for detail):
- repo map: .cargo/ .github/ .gitignore .ignore .nvim.lua AI_POLICY.md CHANGELOG.md CONTRIBUTING.md COPYING Cargo.lock Cargo.toml FAQ.md (237 files)
- sym:29636 RegexSetStrategy (crates/globset/src/lib.rs:964-976)
- sym:29746 create_gitignore (crates/ignore/src/dir.rs:1016-1051)
- sym:29546 Parser.parse_comma (crates/globset/src/glob.rs:876-885)
- sym:29835 IncrementalIgnore.matched_with_errors_impl (crates/ignore/src/incremental.rs:224-308)
- sym:28087 DecompressionMatcherBuilder.new (crates/cli/src/decompress.rs:42-44)
- … and 41 more
