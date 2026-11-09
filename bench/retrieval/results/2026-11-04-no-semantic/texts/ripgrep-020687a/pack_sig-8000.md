## Task
# ripgrep-020687a · ripgrep-020687a-2 · open
Goal: ignore,globset: increase pool capacity

When do matching inside of `ignore` or `globset`, we sometimes need a
reusable mutable scratch space for matching. To do this, we use a `Pool`
abstraction from `regex-automata` to do this heavy lifting for us.

It turns out that this can be a limiting factor in some scenarios when
gitignore matching dominates. For example, in my checkout of the
Chromium repository:

```
$ hyperfine 'rg-master -j32 --files' 'rg-bigger-pool -j32 --files' 'rg-master -j32 --files --no-ignore'
Benchmark 1: rg-master -j32 --files
  Time (mean ± σ):     156.3 ms ±   5.5 ms    [User: 2356.4 ms, System: 380.8 ms]
  Range (min … max):   147.6 ms … 164.5 ms    19 runs

Benchmark 2: rg-bigger-pool -j32 --files
  Time (mean ± σ):     131.6 ms ±   2.1 ms    [User: 1628.2 ms, System: 310.1 ms]
  Range (min … max):   128.8 ms … 136.3 ms    22 runs

Benchmark 3: rg-master -j32 --files --no-ignore
  Time (mean ± σ):     107.4 ms ±   1.0 ms    [User: 664.7 ms, System: 304.7 ms]
  Range (min … max):   104.9 ms … 109.6 ms    28 runs

Summary
  rg-master -j32 --files --no-ignore ran
    1.22 ± 0.02 times faster than rg-bigger-pool -j32 --files
    1.45 ± 0.05 times faster than rg-master -j32 --files
```

The `--no-ignore` benchmark is our baseline. It doesn't process
gitignore files, so it's quite a bit faster. But with this change,
processing gitignore files gets faster than what is currently on master.
Specifically, re-running the above without the b
Workspace: /root/bench/ws/rebf815-ripgrep-020687a/ripgrep-020687a-2
Changed (0): nothing yet

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
  L157: pub struct Error
  L167: pub enum ErrorKind
  L199: fn description(&self) -> &str
  L206: pub fn glob(&self) -> Option<&str>
  L211: pub fn kind(&self) -> &ErrorKind
  L217: fn description(&self) -> &str
  L244: fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  L255: fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  L271: fn new_regex(pat: &str) -> Result<Regex, Error>
  L287: fn new_regex_set(pats: Vec<String>) -> Result<Regex, Error>
  L309: pub struct GlobSet
  L319: pub fn builder() -> GlobSetBuilder
  L325: pub const fn empty() -> GlobSet
  L331: pub fn is_empty(&self) -> bool
  L337: pub fn len(&self) -> usize
  L342: pub fn is_match<P: AsRef<Path>>(&self, path: P) -> bool
  L350: pub fn is_match_candidate(&self, path: &Candidate<'_>) -> bool
  L379: pub fn matches_all<P: AsRef<Path>>(&self, path: P) -> bool
  L390: pub fn matches_all_candidate(&self, path: &Candidate<'_>) -> bool
  L401: pub fn matches<P: AsRef<Path>>(&self, path: P) -> Vec<usize>
  L410: pub fn matches_candidate(&self, path: &Candidate<'_>) -> Vec<usize>
  L425: pub fn matches_into<P: AsRef<Path>>( &self, path: P, into: &mut Vec<usize>, )
  L442: pub fn matches_candidate_into( &self, path: &Candidate<'_>, into: &mut Vec<usize>, )
  L461: pub fn new<I, G>(globs: I) -> Result<GlobSet, Error> where I: IntoIterator<Item = G>, G: AsRef<Glob>,
  L558: fn default() -> Self
  L566: pub struct GlobSetBuilder
  L574: pub fn new() -> GlobSetBuilder
  L581: pub fn build(&self) -> Result<GlobSet, Error>
  L586: pub fn add(&mut self, pat: Glob) -> &mut GlobSetBuilder
  L599: pub struct Candidate<'a>
- crates/ignore/src/dir.rs
  L38: pub(crate) struct IgnoreMatch<'a>
  L44: enum IgnoreMatchInner<'a>
  L52: fn overrides(x: overrides::Glob<'a>) -> IgnoreMatch<'a>
  L56: fn gitignore(x: &'a gitignore::Glob) -> IgnoreMatch<'a>
  L60: fn types(x: types::Glob<'a>) -> IgnoreMatch<'a>
  L64: fn hidden() -> IgnoreMatch<'static>
  L72: struct IgnoreOptions
  L94: pub(crate) struct Ignore
  L115: struct IgnoreInner
  L171: pub(crate) fn path(&self) -> &Path
  L176: pub(crate) fn is_root(&self) -> bool
  L181: pub(crate) fn parent(&self) -> Option<Ignore>
  L192: pub(crate) fn add_parents<P: AsRef<Path>>( &self, path: P, ) -> (Ignore, Option<Error>)
  L268: pub(crate) fn add_child<P: AsRef<Path>>( &self, dir: P, ) -> (Ignore, Option<Error>)
  L284: pub(crate) fn add_child_with_entries<P: AsRef<Path>>( &self, dir: P, entries: &[fs::DirEntry], ) -> (Ignore, Option<Error>)
  L304: fn add_child_path(&self, dir: &Path) -> (IgnoreInner, Option<Error>)
  L308: fn collect_ignore_files( &self, entries: &[fs::DirEntry], ) -> IgnoreFilesFound
  L340: fn add_child_path_with_found_ignore_files( &self, dir: &Path, ignore_files_list: Option<&IgnoreFilesFound>, ) -> (IgnoreInner, Option<Error>)
  L470: fn has_any_ignore_rules(&self) -> bool
  L485: pub(crate) fn matched_dir_entry<'a>( &'a self, dent: &DirEntry, ) -> Match<IgnoreMatch<'a>>
  L500: pub(crate) fn matched<'a, P: AsRef<Path>>( &'a self, path: P, is_dir: bool, ) -> Match<IgnoreMatch<'a>>
  L548: pub(crate) fn matched_ignore<'a>( &'a self, path: &Path, is_dir: bool, ) -> Match<IgnoreMatch<'a>>
  L688: pub(crate) fn parents(&self) -> Parents<'_>
  L694: fn absolute_base(&self) -> Option<&Path>
  L707: struct IgnoreFilesFound
  L716: pub(crate) struct IgnoreRef<'a>
  L721: pub(crate) fn path(&self) -> &Path
  L725: pub(crate) fn is_absolute_parent(&self) -> bool
  L731: pub(crate) struct Parents<'a>
  L734: type Item = IgnoreRef<'a>
- crates/cli/src/decompress.rs
  L15: pub struct DecompressionMatcherBuilder
  L25: struct DecompressionCommand
  L35: fn default() -> DecompressionMatcherBuilder
  L42: pub fn new() -> DecompressionMatcherBuilder
  L50: pub fn build(&self) -> Result<DecompressionMatcher, CommandError>
  L76: pub fn defaults(&mut self, yes: bool) -> &mut DecompressionMatcherBuilder
  L96: pub fn associate<P, I, A>( &mut self, glob: &str, program: P, args: I, ) -> &mut DecompressionMatcherBuilder where P: AsRef<OsStr>, I: IntoIterator<Item = A>, A: AsRef<OsStr>,
  L125: pub fn try_associate<P, I, A>( &mut self, glob: &str, program: P, args: I, ) -> Result<&mut DecompressionMatcherBuilder, CommandError> where P: AsRef<OsStr>, I: IntoIterator<Item = A>, A: AsRef<OsStr>,
  L147: pub struct DecompressionMatcher
  L157: fn default() -> DecompressionMatcher
  L167: pub fn new() -> DecompressionMatcher
  L179: pub fn command<P: AsRef<Path>>(&self, path: P) -> Option<Command>
  L191: pub fn has_command<P: AsRef<Path>>(&self, path: P) -> bool
  L198: pub struct DecompressionReaderBuilder
  L205: pub fn new() -> DecompressionReaderBuilder
  L221: pub fn build<P: AsRef<Path>>( &self, path: P, ) -> Result<DecompressionReader, CommandError>
  L251: pub fn matcher( &mut self, matcher: DecompressionMatcher, ) -> &mut DecompressionReaderBuilder
  L260: pub fn get_matcher(&self) -> &DecompressionMatcher
  L276: pub fn async_stderr( &mut self, yes: bool, ) -> &mut DecompressionReaderBuilder
  L332: pub struct DecompressionReader
  L352: pub fn new<P: AsRef<Path>>( path: P, ) -> Result<DecompressionReader, CommandError>
  L361: fn new_passthru(path: &Path) -> Result<DecompressionReader, CommandError>
  L383: pub fn close(&mut self) -> io::Result<()>
  L392: fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
  L420: pub fn resolve_binary<P: AsRef<Path>>( prog: P, ) -> Result<PathBuf, CommandError>
  L448: fn try_resolve_binary<P: AsRef<Path>>( prog: P, ) -> Result<PathBuf, CommandError>
  L490: fn default_decompression_commands() -> Vec<DecompressionCommand>
- crates/cli/src/process.rs
  L11: pub struct CommandError
  L16: enum CommandErrorKind
  L23: pub(crate) fn io(ioerr: io::Error) -> CommandError
  L28: pub(crate) fn stderr(bytes: Vec<u8>) -> CommandError
  L33: pub(crate) fn is_empty(&self) -> bool
  L44: fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  L66: fn from(ioerr: io::Error) -> CommandError
  L72: fn from(cmderr: CommandError) -> io::Error
  L84: pub struct CommandReaderBuilder
  L90: pub fn new() -> CommandReaderBuilder
  L104: pub fn build( &self, command: &mut process::Command, ) -> Result<CommandReader, CommandError>
  L132: pub fn async_stderr(&mut self, yes: bool) -> &mut CommandReaderBuilder
  L171: pub struct CommandReader
  L195: pub fn new( cmd: &mut process::Command, ) -> Result<CommandReader, CommandError>
  L218: pub fn close(&mut self) -> io::Result<()>
  L247: fn drop(&mut self)
  L255: fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
  L273: enum StderrReader
  L280: fn r#async(mut stderr: process::ChildStderr) -> StderrReader
  L287: fn sync(stderr: process::ChildStderr) -> StderrReader
  L295: fn read_to_end(&mut self) -> CommandError
  L310: fn stderr_to_command_error(stderr: &mut process::ChildStderr) -> CommandError
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
  L16: pub(crate) struct HaystackBuilder
  L22: pub(crate) fn new() -> HaystackBuilder
  L31: pub(crate) fn build_from_result( &self, result: Result<ignore::DirEntry, ignore::Error>, ) -> Option<Haystack>
  L49: fn build(&self, dent: ignore::DirEntry) -> Option<Haystack>
  L85: pub(crate) fn strip_dot_prefix( &mut self, yes: bool, ) -> &mut HaystackBuilder
  L98: pub(crate) struct Haystack
  L108: pub(crate) fn path(&self) -> &Path
  L117: pub(crate) fn is_stdin(&self) -> bool
  L131: pub(crate) fn is_explicit(&self) -> bool
  L143: fn is_dir(&self) -> bool
  L157: fn is_file(&self) -> bool
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
- sym:29967 Types (crates/ignore/src/types.rs:165-181)
  pub struct Types
- sym:29334 EngineChoice (crates/core/flags/lowargs.rs:541-555)
  pub(crate) enum EngineChoice
- sym:31363 Searcher.multi_line_with_matcher (crates/searcher/src/searcher/mod.rs:896-915)
  pub fn multi_line_with_matcher<M: Matcher>(&self, matcher: M) -> bool
- sym:31059 RegexMatcherBuilder.build_many (crates/regex/src/matcher.rs:53-85)
  pub fn build_many<P: AsRef<str>>( &self, patterns: &[P], ) -> Result<RegexMatcher, Error>
- sym:29537 bytes_to_escaped_literal (crates/globset/src/glob.rs:776-789)
  fn bytes_to_escaped_literal(bs: &[u8]) -> String
- sym:29641 RegexSetStrategy.matches_into (crates/globset/src/lib.rs:1002-1012)
  fn matches_into( &self, candidate: &Candidate<'_>, matches: &mut Vec<usize>, )
- sym:30943 find_iter_at_in_context (crates/printer/src/util.rs:473-530)
  pub(crate) fn find_iter_at_in_context<M, F>( searcher: &Searcher, matcher: M, mut bytes: &[u8], range: std::ops::Range<usize>, mut matched: F, ) -> io::Result<()> where M: Matcher, F: FnMut(Match) -> bool,
- sym:30914 Replacer.replacement (crates/printer/src/util.rs:115-128)
  pub(crate) fn replacement<'a>( &'a self, ) -> Option<(&'a [u8], &'a [Match])>
- sym:31245 Core.is_line_by_line_fast (crates/searcher/src/searcher/core.rs:673-708)
  fn is_line_by_line_fast(&self) -> bool
- sym:30916 Replacer.allocate (crates/printer/src/util.rs:146-153)
  fn allocate(&mut self, matcher: &M) -> io::Result<&mut Space<M>>
- sym:30915 Replacer.clear (crates/printer/src/util.rs:134-139)
  pub(crate) fn clear(&mut self)
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
- sym:31286 big_error_multi_line (crates/searcher/src/searcher/glue.rs:679-697)
  fn big_error_multi_line()
- sym:29870 respects_git_repository_boundaries (crates/ignore/src/incremental.rs:788-806)
  fn respects_git_repository_boundaries()
- sym:31171 buffer_limited_capacity2 (crates/searcher/src/line_buffer.rs:740-762)
  fn buffer_limited_capacity2()
- sym:31304 scratch (crates/searcher/src/searcher/glue.rs:1495-1521)
  fn scratch()
- sym:30272 scratch (crates/index/src/literal.rs:976-999)
  fn scratch()
- sym:28432 test_files (crates/core/flags/defs.rs:2241-2247)
  fn test_files()
- sym:28609 test_index_crud (crates/core/flags/defs.rs:3629-3647)
  fn test_index_crud()
- sym:28646 test_json (crates/core/flags/defs.rs:3893-3908)
  fn test_json()

---
tokens: 7109/8000
