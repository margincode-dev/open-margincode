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
- sym:23106 Gitignore.matched_stripped (crates/ignore/src/gitignore.rs:259-282)
  fn matched_stripped<P: AsRef<Path>>( &self, path: P, is_dir: bool, ) -> Match<&Glob>
- sym:23110 GitignoreBuilder.build (crates/ignore/src/gitignore.rs:349-366)
  pub fn build(&self) -> Result<Gitignore, Error>
- sym:23095 Gitignore (crates/ignore/src/gitignore.rs:81-88)
- sym:22996 match_single_file_glob (crates/ignore/benches/workloads.rs:487-505)
- sym:23104 Gitignore.matched (crates/ignore/src/gitignore.rs:202-211)
- sym:23000 incremental_gitignore_only (crates/ignore/benches/workloads.rs:563-576)
- sym:22844 GlobSet.empty (crates/globset/src/lib.rs:325-327)
- sym:23039 IgnoreBuilder (crates/ignore/src/dir.rs:753-776)
- sym:22979 build_empty_gitignore (crates/ignore/benches/workloads.rs:304-307)
- sym:23445 path_equals (crates/ignore/src/walk.rs:2123-2142)
- sym:21448 Error.from (crates/cli/src/human.rs:66-68)
- sym:21465 Error.from (crates/cli/src/pattern.rs:40-42)
- sym:21479 CommandError.from (crates/cli/src/process.rs:66-68)

---
tokens: 725/800
omitted 47 lower-ranked items (expand an id for detail):
- crates/matcher/src/interpolate.rs
- crates/printer/src/hyperlink/mod.rs
- crates/printer/src/json.rs
- crates/printer/src/standard.rs
- crates/printer/src/stats.rs
- crates/printer/src/util.rs
- crates/regex/src/literal.rs
- crates/regex/src/strip.rs
- repo map: .cargo/ .github/ .gitignore .ignore .nvim.lua AI_POLICY.md CHANGELOG.md CONTRIBUTING.md COPYING Cargo.lock Cargo.toml FAQ.md (226 files)
- sym:21480 Error.from (crates/cli/src/process.rs:72-79)
- sym:22565 globs (crates/core/flags/hiargs.rs:1210-1231)
- sym:22566 preprocessor_globs (crates/core/flags/hiargs.rs:1234-1246)
- … and 35 more
