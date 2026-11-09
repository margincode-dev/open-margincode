## Task
# ripgrep-c5e744c · ripgrep-c5e744c · open
Goal: ignore: avoid scratch pools for empty matchers

... by deferring to `Gitignore::empty()` when there aren't any globs.

This plausibly skips some other work as well, in addition to avoiding
allocation for scratch pools.

PR #3555
Workspace: /root/bench/ws/rebf815-ripgrep-c5e744c/ripgrep-c5e744c
Changed (0): nothing yet

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
- crates/matcher/src/interpolate.rs
- crates/matcher/src/lib.rs
- crates/printer/src/hyperlink/mod.rs
- crates/printer/src/json.rs
- crates/printer/src/standard.rs
- crates/printer/src/stats.rs
- crates/printer/src/util.rs
- crates/regex/src/literal.rs
- crates/regex/src/strip.rs
- repo map: .cargo/ .github/ .gitignore .ignore .nvim.lua AI_POLICY.md CHANGELOG.md CONTRIBUTING.md COPYING Cargo.lock Cargo.toml FAQ.md (226 files)

## Code
- sym:23098 Gitignore.empty (crates/ignore/src/gitignore.rs:151-160)
  pub fn empty() -> Gitignore
- sym:23000 incremental_gitignore_only (crates/ignore/benches/workloads.rs:563-576)
  fn incremental_gitignore_only(b: &mut test::Bencher)
- sym:23106 Gitignore.matched_stripped (crates/ignore/src/gitignore.rs:259-282)
  fn matched_stripped<P: AsRef<Path>>( &self, path: P, is_dir: bool, ) -> Match<&Glob>
- sym:23095 Gitignore (crates/ignore/src/gitignore.rs:81-88)
  pub struct Gitignore
- sym:23110 GitignoreBuilder.build (crates/ignore/src/gitignore.rs:349-366)
  pub fn build(&self) -> Result<Gitignore, Error>
- sym:23039 IgnoreBuilder (crates/ignore/src/dir.rs:753-776)
  pub(crate) struct IgnoreBuilder
- sym:22996 match_single_file_glob (crates/ignore/benches/workloads.rs:487-505)
  fn match_single_file_glob(b: &mut test::Bencher)
- sym:23445 path_equals (crates/ignore/src/walk.rs:2123-2142)
  fn path_equals(dent: &DirEntry, handle: &Handle) -> Result<bool, Error>
- sym:23187 compiled_multi_child_matchers_are_not_reused (crates/ignore/src/incremental.rs:901-936)
  fn compiled_multi_child_matchers_are_not_reused()
- sym:23182 compiled_matchers_are_reused (crates/ignore/src/incremental.rs:811-830)
  fn compiled_matchers_are_reused()
- sym:21448 Error.from (crates/cli/src/human.rs:66-68)
  fn from(size_err: ParseSizeError) -> std::io::Error
- sym:21465 Error.from (crates/cli/src/pattern.rs:40-42)
  fn from(paterr: InvalidPatternError) -> io::Error
- sym:21479 CommandError.from (crates/cli/src/process.rs:66-68)
  fn from(ioerr: io::Error) -> CommandError
- sym:21480 Error.from (crates/cli/src/process.rs:72-79)
  fn from(cmderr: CommandError) -> io::Error
- sym:22565 globs (crates/core/flags/hiargs.rs:1210-1231)
  fn globs( state: &State, low: &LowArgs, ) -> anyhow::Result<ignore::overrides::Override>
- sym:22566 preprocessor_globs (crates/core/flags/hiargs.rs:1234-1246)
  fn preprocessor_globs( state: &State, low: &LowArgs, ) -> anyhow::Result<ignore::overrides::Override>
- sym:22703 Config.default (crates/core/search.rs:28-36)
  fn default() -> Config
- sym:22842 GlobSet (crates/globset/src/lib.rs:309-312)
  pub struct GlobSet
- sym:22851 GlobSet.matches (crates/globset/src/lib.rs:401-403)
- sym:22855 GlobSet.new (crates/globset/src/lib.rs:461-553)
- sym:22856 GlobSet.default (crates/globset/src/lib.rs:558-560)
  fn default() -> Self
- sym:23025 Ignore.add_child_path_with_found_ignore_files (crates/ignore/src/dir.rs:340-467)
- sym:23040 IgnoreBuilder.new (crates/ignore/src/dir.rs:784-803)
  pub(crate) fn new() -> IgnoreBuilder
- sym:23042 IgnoreBuilder.build_with_cwd (crates/ignore/src/dir.rs:817-862)
- sym:23089 Glob.from (crates/ignore/src/gitignore.rs:47-49)
- sym:23096 Gitignore.new (crates/ignore/src/gitignore.rs:103-118)
- sym:23097 Gitignore.global (crates/ignore/src/gitignore.rs:141-146)
- sym:23102 Gitignore.num_ignores (crates/ignore/src/gitignore.rs:182-184)
- sym:23103 Gitignore.num_whitelists (crates/ignore/src/gitignore.rs:187-189)
- sym:23111 GitignoreBuilder.build_global (crates/ignore/src/gitignore.rs:377-396)
- sym:23142 IncrementalIgnore.root (crates/ignore/src/incremental.rs:134-136)
- sym:23218 Error.from (crates/ignore/src/lib.rs:367-369)
- sym:23245 Override.num_ignores (crates/ignore/src/overrides.rs:70-72)
- sym:23246 Override.num_whitelists (crates/ignore/src/overrides.rs:75-77)
- sym:23281 FileTypeDef.globs (crates/ignore/src/types.rs:158-160)
- sym:23356 WalkBuilder.from_iter (crates/ignore/src/walk.rs:582-590)
- sym:23395 WalkEventIter.from (crates/ignore/src/walk.rs:1284-1286)
- sym:23496 Ref.from (crates/matcher/src/interpolate.rs:80-82)
- sym:23548 Error.from (crates/matcher/src/lib.rs:512-514)
- sym:23813 JSONSink.context (crates/printer/src/json.rs:755-783)
- sym:23911 StandardImpl.new (crates/printer/src/standard.rs:886-896)
- sym:24049 Stats.matches (crates/printer/src/stats.rs:67-69)
- sym:24143 Sunk.matches (crates/printer/src/util.rs:238-240)
- sym:24216 Extractor.extract_alternation (crates/regex/src/literal.rs:231-247)
- sym:24217 Extractor.extract_repetition (crates/regex/src/literal.rs:264-315)
- sym:24218 Extractor.extract_class_unicode (crates/regex/src/literal.rs:320-332)
- sym:24219 Extractor.extract_class_bytes (crates/regex/src/literal.rs:336-348)
- sym:24333 strip_from_match_ascii (crates/regex/src/strip.rs:55-119)
- sym:23186 compiled_multi_matchers_are_reused (crates/ignore/src/incremental.rs:881-898)
- sym:23778 build_empty_format (crates/printer/src/hyperlink/mod.rs:973-979)

---
tokens: 1793/2000
