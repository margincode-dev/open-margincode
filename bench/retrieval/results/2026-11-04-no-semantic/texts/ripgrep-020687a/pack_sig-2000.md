## Task
# ripgrep-020687a · ripgrep-020687a-2 · open

## Map
- crates/core/flags/defs.rs
- crates/core/main.rs
- crates/globset/src/lib.rs
- crates/ignore/src/dir.rs
- crates/cli/src/decompress.rs
- crates/cli/src/process.rs
- crates/cli/src/wtr.rs
- crates/core/flags/config.rs
  L16: pub fn args() -> Vec<OsString>
  L63: fn parse<P: AsRef<Path>>( path: P, ) -> anyhow::Result<(Vec<OsString>, Vec<anyhow::Error>)>
  L84: fn parse_reader<R: std::io::Read>( rdr: R, ) -> anyhow::Result<(Vec<OsString>, Vec<anyhow::Error>)>
  L116: fn basic()
  L139: fn error()
- crates/core/flags/hiargs.rs
- crates/core/flags/lowargs.rs
- crates/core/flags/parse.rs
- crates/core/haystack.rs
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
- sym:29653 escape (crates/globset/src/lib.rs:1115-1132)
  pub fn escape(s: &str) -> String
- sym:29575 new_regex_set (crates/globset/src/lib.rs:287-304)
  fn new_regex_set(pats: Vec<String>) -> Result<Regex, Error>
- sym:29574 new_regex (crates/globset/src/lib.rs:271-285)
  fn new_regex(pat: &str) -> Result<Regex, Error>
- sym:29647 MultiStrategyBuilder.regex_set (crates/globset/src/lib.rs:1051-1061)
  fn regex_set(self) -> Result<RegexSetStrategy, Error>
- sym:31346 Searcher (crates/searcher/src/searcher/mod.rs:597-625)
  pub struct Searcher
- sym:29636 RegexSetStrategy (crates/globset/src/lib.rs:964-976)
  struct RegexSetStrategy
- sym:29546 Parser.parse_comma (crates/globset/src/glob.rs:876-885)
  fn parse_comma(&mut self) -> Result<(), Error>
- sym:28087 DecompressionMatcherBuilder.new (crates/cli/src/decompress.rs:42-44)
  pub fn new() -> DecompressionMatcherBuilder
- sym:28088 DecompressionMatcherBuilder.build (crates/cli/src/decompress.rs:50-69)
- sym:28092 DecompressionMatcher (crates/cli/src/decompress.rs:147-154)
- sym:28094 DecompressionMatcher.new (crates/cli/src/decompress.rs:167-171)
- sym:28098 DecompressionReaderBuilder.new (crates/cli/src/decompress.rs:205-207)
- sym:28099 DecompressionReaderBuilder.build (crates/cli/src/decompress.rs:221-244)
- sym:28104 DecompressionReader.new (crates/cli/src/decompress.rs:352-356)
- sym:28163 CommandError.io (crates/cli/src/process.rs:23-25)
- sym:28170 CommandReaderBuilder.new (crates/cli/src/process.rs:90-92)
- sym:28171 CommandReaderBuilder.build (crates/cli/src/process.rs:104-118)
- sym:28174 CommandReader.new (crates/cli/src/process.rs:195-199)
- sym:28184 stdout (crates/cli/src/wtr.rs:23-29)
- sym:28188 StandardStream.write (crates/cli/src/wtr.rs:69-76)
- sym:28208 args (crates/core/flags/config.rs:16-53)
- sym:28214 FLAGS (crates/core/flags/defs.rs:44-156)
- sym:28431 Files.update (crates/core/flags/defs.rs:2232-2236)
- sym:29254 HiArgs (crates/core/flags/hiargs.rs:36-107)
- sym:29258 HiArgs.haystack_builder (crates/core/flags/hiargs.rs:368-372)
- sym:29264 HiArgs.path_printer_builder (crates/core/flags/hiargs.rs:556-566)
- sym:29270 HiArgs.quit_after_match (crates/core/flags/hiargs.rs:697-699)
- sym:29273 HiArgs.sort (crates/core/flags/hiargs.rs:773-826)
- sym:29275 HiArgs.stdout (crates/core/flags/hiargs.rs:842-855)
- sym:29277 HiArgs.types (crates/core/flags/hiargs.rs:872-874)
- sym:29278 HiArgs.walk_builder (crates/core/flags/hiargs.rs:885-929)
- sym:29283 State.new (crates/core/flags/hiargs.rs:986-996)
- sym:29291 BinaryDetection.is_none (crates/core/flags/hiargs.rs:1215-1218)
- sym:29292 types (crates/core/flags/hiargs.rs:1222-1242)
- sym:29293 globs (crates/core/flags/hiargs.rs:1246-1267)
- sym:29294 preprocessor_globs (crates/core/flags/hiargs.rs:1270-1282)
- sym:29308 Mode (crates/core/flags/lowargs.rs:156-169)
- sym:29330 ContextSeparator.new (crates/core/flags/lowargs.rs:495-503)
- sym:29337 FieldContextSeparator.new (crates/core/flags/lowargs.rs:573-581)

---
tokens: 1756/2000
omitted 9 lower-ranked items (expand an id for detail):
- sym:29341 FieldMatchSeparator.new (crates/core/flags/lowargs.rs:607-615)
- sym:29378 Parser.new (crates/core/flags/parse.rs:176-216)
- sym:29387 FlagMap.new (crates/core/flags/parse.rs:374-387)
- sym:29395 HaystackBuilder.new (crates/core/haystack.rs:22-24)
- sym:29396 HaystackBuilder.build_from_result (crates/core/haystack.rs:31-42)
- sym:29759 ignore (crates/ignore/src/dir.rs:1226-1235)
- sym:28432 test_files (crates/core/flags/defs.rs:2241-2247)
- sym:28609 test_index_crud (crates/core/flags/defs.rs:3629-3647)
- sym:28646 test_json (crates/core/flags/defs.rs:3893-3908)
