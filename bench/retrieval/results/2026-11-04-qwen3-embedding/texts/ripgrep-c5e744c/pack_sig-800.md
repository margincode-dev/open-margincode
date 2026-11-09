## Task
# ripgrep-c5e744c · ripgrep-c5e744c · open

## Map
- crates/ignore/src/gitignore.rs
- crates/cli/src/human.rs
- crates/cli/src/pattern.rs
- crates/cli/src/process.rs
- crates/core/flags/hiargs.rs
- crates/core/search.rs
- crates/globset/src/lib.rs
- crates/ignore/src/dir.rs
- crates/ignore/src/incremental.rs
- crates/ignore/src/lib.rs
- crates/ignore/src/overrides.rs
- crates/ignore/src/types.rs
- crates/ignore/src/walk.rs
- crates/matcher/src/lib.rs

## Code
- sym:23098 Gitignore.empty (crates/ignore/src/gitignore.rs:151-160)
  pub fn empty() -> Gitignore
- sym:23000 incremental_gitignore_only (crates/ignore/benches/workloads.rs:563-576)
  fn incremental_gitignore_only(b: &mut test::Bencher)
- sym:23106 Gitignore.matched_stripped (crates/ignore/src/gitignore.rs:259-282)
- sym:23095 Gitignore (crates/ignore/src/gitignore.rs:81-88)
  pub struct Gitignore
- sym:23110 GitignoreBuilder.build (crates/ignore/src/gitignore.rs:349-366)
  pub fn build(&self) -> Result<Gitignore, Error>
- sym:23039 IgnoreBuilder (crates/ignore/src/dir.rs:753-776)
  pub(crate) struct IgnoreBuilder
- sym:22996 match_single_file_glob (crates/ignore/benches/workloads.rs:487-505)
- sym:23445 path_equals (crates/ignore/src/walk.rs:2123-2142)
- sym:23187 compiled_multi_child_matchers_are_not_reused (crates/ignore/src/incremental.rs:901-936)
- sym:23182 compiled_matchers_are_reused (crates/ignore/src/incremental.rs:811-830)
- sym:21448 Error.from (crates/cli/src/human.rs:66-68)
- sym:21465 Error.from (crates/cli/src/pattern.rs:40-42)
- sym:21479 CommandError.from (crates/cli/src/process.rs:66-68)
- sym:21480 Error.from (crates/cli/src/process.rs:72-79)

---
tokens: 726/800
omitted 45 lower-ranked items (expand an id for detail):
- crates/matcher/src/interpolate.rs
- crates/printer/src/hyperlink/mod.rs
- crates/printer/src/json.rs
- crates/printer/src/standard.rs
- crates/printer/src/stats.rs
- crates/printer/src/util.rs
- crates/regex/src/literal.rs
- crates/regex/src/strip.rs
- repo map: .cargo/ .github/ .gitignore .ignore .nvim.lua AI_POLICY.md CHANGELOG.md CONTRIBUTING.md COPYING Cargo.lock Cargo.toml FAQ.md (226 files)
- sym:22565 globs (crates/core/flags/hiargs.rs:1210-1231)
- sym:22566 preprocessor_globs (crates/core/flags/hiargs.rs:1234-1246)
- sym:22703 Config.default (crates/core/search.rs:28-36)
- … and 33 more
