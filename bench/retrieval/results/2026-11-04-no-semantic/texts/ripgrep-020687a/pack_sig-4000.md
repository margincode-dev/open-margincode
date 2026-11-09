## Task
# ripgrep-020687a · ripgrep-020687a-2 · open

## Map
- crates/core/flags/defs.rs
  L44: pub(super) const FLAGS: &[&dyn Flag] = &[
  L167: pub(super) fn indexing_unsupported_flag( &self, ) -> Option<&'static dyn Flag>
  L233: struct AfterContext
  L236: fn is_switch(&self) -> bool
  L239: fn name_short(&self) -> Option<u8>
  L242: fn name_long(&self) -> &'static str
  L245: fn doc_variable(&self) -> Option<&'static str>
  L248: fn doc_category(&self) -> Category
  L251: fn doc_short(&self) -> &'static str
  L254: fn doc_long(&self) -> &'static str
  L263: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()>
  L271: fn test_after_context()
  L319: struct AutoHybridRegex
  L322: fn is_switch(&self) -> bool
  L325: fn name_long(&self) -> &'static str
  L328: fn name_negated(&self) -> Option<&'static str>
  L331: fn doc_category(&self) -> Category
  L334: fn doc_short(&self) -> &'static str
  L337: fn doc_long(&self) -> &'static str
  L367: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()>
  L380: fn test_auto_hybrid_regex()
  L418: struct BeforeContext
  L421: fn is_switch(&self) -> bool
  L424: fn name_short(&self) -> Option<u8>
  L427: fn name_long(&self) -> &'static str
  L430: fn doc_variable(&self) -> Option<&'static str>
  L433: fn doc_category(&self) -> Category
  L436: fn doc_short(&self) -> &'static str
  L439: fn doc_long(&self) -> &'static str
  L448: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()>
- crates/core/main.rs
  L41: static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc
  L44: fn main() -> ExitCode
  L78: fn run(result: crate::flags::ParseResult<HiArgs>) -> anyhow::Result<ExitCode>
  L113: fn search(args: &HiArgs, mode: SearchMode) -> anyhow::Result<bool>
  L166: fn search_parallel(args: &HiArgs, mode: SearchMode) -> anyhow::Result<bool>
  L241: fn files(args: &HiArgs) -> anyhow::Result<bool>
  L277: fn files_parallel(args: &HiArgs) -> anyhow::Result<bool>
  L335: fn types(args: &HiArgs) -> anyhow::Result<ExitCode>
  L361: fn generate(mode: crate::flags::GenerateMode) -> anyhow::Result<ExitCode>
  L389: fn special(mode: crate::flags::SpecialMode) -> anyhow::Result<ExitCode>
  L420: fn eprint_nothing_searched()
  L440: fn print_stats<W: Write>( mode: SearchMode, stats: &grep::printer::Stats, started: std::time::Instant, mut wtr: W, ) -> std::io::Result<()>
- crates/globset/src/lib.rs
- crates/ignore/src/dir.rs
- crates/cli/src/decompress.rs
- crates/cli/src/process.rs
- crates/cli/src/wtr.rs
  L7: pub struct StandardStream
  L23: pub fn stdout(color_choice: termcolor::ColorChoice) -> StandardStream
  L39: pub fn stdout_buffered_line( color_choice: termcolor::ColorChoice, ) -> StandardStream
  L54: pub fn stdout_buffered_block( color_choice: termcolor::ColorChoice, ) -> StandardStream
  L62: enum StandardStreamKind
  L69: fn write(&mut self, buf: &[u8]) -> io::Result<usize>
  L79: fn flush(&mut self) -> io::Result<()>
  L91: fn supports_color(&self) -> bool
  L101: fn supports_hyperlinks(&self) -> bool
  L111: fn set_color(&mut self, spec: &termcolor::ColorSpec) -> io::Result<()>
  L121: fn set_hyperlink(&mut self, link: &HyperlinkSpec) -> io::Result<()>
  L131: fn reset(&mut self) -> io::Result<()>
  L141: fn is_synchronous(&self) -> bool
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
- repo map: .cargo/ .github/ .gitignore .ignore .nvim.lua AI_POLICY.md CHANGELOG.md CONTRIBUTING.md COPYING Cargo.lock Cargo.toml FAQ.md (237 files)

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
- sym:30329 LineMatchKind (crates/matcher/src/lib.rs:519-531)
  pub enum LineMatchKind
- sym:31155 LineBuffer.fill (crates/searcher/src/line_buffer.rs:406-472)
  fn fill<R: io::Read>(&mut self, mut rdr: R) -> Result<bool, io::Error>
- sym:29637 PatternSetPoolFn (crates/globset/src/lib.rs:978-979)
  type PatternSetPoolFn = Box<dyn Fn() -> PatternSet + Send + Sync + UnwindSafe + RefUnwindSafe>
- sym:30982 Error.regex (crates/regex/src/error.rs:16-27)
  pub(crate) fn regex(err: regex_automata::meta::BuildError) -> Error
- sym:29784 Gitignore (crates/ignore/src/gitignore.rs:81-88)
  pub struct Gitignore
- sym:29567 ErrorKind (crates/globset/src/lib.rs:167-196)
  pub enum ErrorKind
- sym:31096 RegexCaptures.new (crates/regex/src/matcher.rs:543-545)
  pub(crate) fn new(caps: AutomataCaptures) -> RegexCaptures
- sym:31097 RegexCaptures.captures_mut (crates/regex/src/matcher.rs:548-550)
  pub(crate) fn captures_mut(&mut self) -> &mut AutomataCaptures
- sym:28087 DecompressionMatcherBuilder.new (crates/cli/src/decompress.rs:42-44)
  pub fn new() -> DecompressionMatcherBuilder
- sym:28088 DecompressionMatcherBuilder.build (crates/cli/src/decompress.rs:50-69)
  pub fn build(&self) -> Result<DecompressionMatcher, CommandError>
- sym:28092 DecompressionMatcher (crates/cli/src/decompress.rs:147-154)
  pub struct DecompressionMatcher
- sym:28094 DecompressionMatcher.new (crates/cli/src/decompress.rs:167-171)
  pub fn new() -> DecompressionMatcher
- sym:28098 DecompressionReaderBuilder.new (crates/cli/src/decompress.rs:205-207)
  pub fn new() -> DecompressionReaderBuilder
- sym:28099 DecompressionReaderBuilder.build (crates/cli/src/decompress.rs:221-244)
  pub fn build<P: AsRef<Path>>( &self, path: P, ) -> Result<DecompressionReader, CommandError>
- sym:28104 DecompressionReader.new (crates/cli/src/decompress.rs:352-356)
  pub fn new<P: AsRef<Path>>( path: P, ) -> Result<DecompressionReader, CommandError>
- sym:28163 CommandError.io (crates/cli/src/process.rs:23-25)
  pub(crate) fn io(ioerr: io::Error) -> CommandError
- sym:28170 CommandReaderBuilder.new (crates/cli/src/process.rs:90-92)
  pub fn new() -> CommandReaderBuilder
- sym:28171 CommandReaderBuilder.build (crates/cli/src/process.rs:104-118)
  pub fn build( &self, command: &mut process::Command, ) -> Result<CommandReader, CommandError>
- sym:28174 CommandReader.new (crates/cli/src/process.rs:195-199)
  pub fn new( cmd: &mut process::Command, ) -> Result<CommandReader, CommandError>
- sym:28184 stdout (crates/cli/src/wtr.rs:23-29)
  pub fn stdout(color_choice: termcolor::ColorChoice) -> StandardStream
- sym:28188 StandardStream.write (crates/cli/src/wtr.rs:69-76)
  fn write(&mut self, buf: &[u8]) -> io::Result<usize>
- sym:28208 args (crates/core/flags/config.rs:16-53)
  pub fn args() -> Vec<OsString>
- sym:28214 FLAGS (crates/core/flags/defs.rs:44-156)
  pub(super) const FLAGS: &[&dyn Flag] = &[
- sym:28431 Files.update (crates/core/flags/defs.rs:2232-2236)
  fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()>
- sym:29254 HiArgs (crates/core/flags/hiargs.rs:36-107)
  pub(crate) struct HiArgs
- sym:29258 HiArgs.haystack_builder (crates/core/flags/hiargs.rs:368-372)
  pub(crate) fn haystack_builder(&self) -> HaystackBuilder
- sym:29264 HiArgs.path_printer_builder (crates/core/flags/hiargs.rs:556-566)
  pub(crate) fn path_printer_builder( &self, ) -> grep::printer::PathPrinterBuilder
- sym:29270 HiArgs.quit_after_match (crates/core/flags/hiargs.rs:697-699)
  pub(crate) fn quit_after_match(&self) -> bool
- sym:29273 HiArgs.sort (crates/core/flags/hiargs.rs:773-826)
  pub(crate) fn sort<'a, I>( &self, haystacks: I, ) -> Box<dyn Iterator<Item = Haystack> + 'a> where I: Iterator<Item = Haystack> + 'a,
- sym:29275 HiArgs.stdout (crates/core/flags/hiargs.rs:842-855)
  pub(crate) fn stdout(&self) -> grep::cli::StandardStream
- sym:29277 HiArgs.types (crates/core/flags/hiargs.rs:872-874)
  pub(crate) fn types(&self) -> &ignore::types::Types
- sym:29278 HiArgs.walk_builder (crates/core/flags/hiargs.rs:885-929)
  pub(crate) fn walk_builder(&self) -> anyhow::Result<ignore::WalkBuilder>
- sym:29283 State.new (crates/core/flags/hiargs.rs:986-996)
  fn new() -> anyhow::Result<State>
- sym:29291 BinaryDetection.is_none (crates/core/flags/hiargs.rs:1215-1218)
  pub(crate) fn is_none(&self) -> bool
- sym:29292 types (crates/core/flags/hiargs.rs:1222-1242)
  fn types(low: &LowArgs) -> anyhow::Result<ignore::types::Types>
- sym:29293 globs (crates/core/flags/hiargs.rs:1246-1267)
  fn globs( state: &State, low: &LowArgs, ) -> anyhow::Result<ignore::overrides::Override>
- sym:29294 preprocessor_globs (crates/core/flags/hiargs.rs:1270-1282)
  fn preprocessor_globs( state: &State, low: &LowArgs, ) -> anyhow::Result<ignore::overrides::Override>
- sym:29308 Mode (crates/core/flags/lowargs.rs:156-169)
  pub(crate) enum Mode
- sym:29330 ContextSeparator.new (crates/core/flags/lowargs.rs:495-503)
  pub(crate) fn new(os: &OsStr) -> anyhow::Result<ContextSeparator>
- sym:29337 FieldContextSeparator.new (crates/core/flags/lowargs.rs:573-581)
  pub(crate) fn new(os: &OsStr) -> anyhow::Result<FieldContextSeparator>
- sym:29341 FieldMatchSeparator.new (crates/core/flags/lowargs.rs:607-615)
  pub(crate) fn new(os: &OsStr) -> anyhow::Result<FieldMatchSeparator>
- sym:29378 Parser.new (crates/core/flags/parse.rs:176-216)
  fn new() -> &'static Parser
- sym:29387 FlagMap.new (crates/core/flags/parse.rs:374-387)
  fn new(infos: &[FlagInfo]) -> FlagMap
- sym:29395 HaystackBuilder.new (crates/core/haystack.rs:22-24)
  pub(crate) fn new() -> HaystackBuilder
- sym:29396 HaystackBuilder.build_from_result (crates/core/haystack.rs:31-42)
  pub(crate) fn build_from_result( &self, result: Result<ignore::DirEntry, ignore::Error>, ) -> Option<Haystack>
- sym:29759 ignore (crates/ignore/src/dir.rs:1226-1235)
  fn ignore()
- sym:28432 test_files (crates/core/flags/defs.rs:2241-2247)
  fn test_files()
- sym:28609 test_index_crud (crates/core/flags/defs.rs:3629-3647)
  fn test_index_crud()
- sym:28646 test_json (crates/core/flags/defs.rs:3893-3908)
  fn test_json()

---
tokens: 3585/4000
