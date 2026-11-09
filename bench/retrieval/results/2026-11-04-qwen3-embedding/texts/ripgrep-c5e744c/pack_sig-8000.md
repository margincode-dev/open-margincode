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
  L32: pub struct Glob
  L47: pub fn from(&self) -> Option<&Path>
  L52: pub fn original(&self) -> &str
  L58: pub fn actual(&self) -> &str
  L63: pub fn is_whitelist(&self) -> bool
  L68: pub fn is_only_dir(&self) -> bool
  L73: fn has_doublestar_prefix(&self) -> bool
  L81: pub struct Gitignore
  L103: pub fn new<P: AsRef<Path>>( gitignore_path: P, ) -> (Gitignore, Option<Error>)
  L141: pub fn global() -> (Gitignore, Option<Error>)
  L151: pub fn empty() -> Gitignore
  L165: pub fn path(&self) -> &Path
  L171: pub fn is_empty(&self) -> bool
  L177: pub fn len(&self) -> usize
  L182: pub fn num_ignores(&self) -> u64
  L187: pub fn num_whitelists(&self) -> u64
  L202: pub fn matched<P: AsRef<Path>>( &self, path: P, is_dir: bool, ) -> Match<&Glob>
  L234: pub fn matched_path_or_any_parents<P: AsRef<Path>>( &self, path: P, is_dir: bool, ) -> Match<&Glob>
  L259: fn matched_stripped<P: AsRef<Path>>( &self, path: P, is_dir: bool, ) -> Match<&Glob>
  L286: fn strip<'a, P: 'a + AsRef<Path> + ?Sized>( &'a self, path: &'a P, ) -> &'a Path
  L320: pub struct GitignoreBuilder
  L335: pub fn new<P: AsRef<Path>>(root: P) -> GitignoreBuilder
  L349: pub fn build(&self) -> Result<Gitignore, Error>
  L377: pub fn build_global(mut self) -> (Gitignore, Option<Error>)
  L405: pub fn add<P: AsRef<Path>>(&mut self, path: P) -> Option<Error>
  L443: fn add_str( &mut self, from: Option<PathBuf>, gitignore: &str, ) -> Result<&mut GitignoreBuilder, Error>
  L460: pub fn add_line( &mut self, from: Option<PathBuf>, mut line: &str, ) -> Result<&mut GitignoreBuilder, Error>
  L549: pub fn case_insensitive( &mut self, yes: bool, ) -> Result<&mut GitignoreBuilder, Error>
  L571: pub fn allow_unclosed_class( &mut self, yes: bool, ) -> &mut GitignoreBuilder
  L583: pub fn gitconfig_excludes_path() -> Option<PathBuf>
- crates/cli/src/human.rs
  L6: pub struct ParseSizeError
  L12: enum ParseSizeErrorKind
  L19: fn format(original: &str) -> ParseSizeError
  L26: fn int(original: &str, err: std::num::ParseIntError) -> ParseSizeError
  L33: fn overflow(original: &str) -> ParseSizeError
  L44: fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  L66: fn from(size_err: ParseSizeError) -> std::io::Error
  L79: pub fn parse_human_readable_size(size: &str) -> Result<u64, ParseSizeError>
  L107: fn suffix_none()
  L113: fn suffix_k()
  L119: fn suffix_m()
  L125: fn suffix_g()
  L131: fn invalid_empty()
  L136: fn invalid_non_digit()
  L141: fn invalid_overflow()
  L146: fn invalid_suffix()
- crates/cli/src/pattern.rs
  L12: pub struct InvalidPatternError
  L20: pub fn valid_up_to(&self) -> usize
  L28: fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  L40: fn from(paterr: InvalidPatternError) -> io::Error
  L51: pub fn pattern_from_os(pattern: &OsStr) -> Result<&str, InvalidPatternError>
  L67: pub fn pattern_from_bytes( pattern: &[u8], ) -> Result<&str, InvalidPatternError>
  L82: pub fn patterns_from_path<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>>
  L104: pub fn patterns_from_stdin() -> io::Result<Vec<String>>
  L141: pub fn patterns_from_reader<R: io::Read>(rdr: R) -> io::Result<Vec<String>>
  L165: fn bytes()
  L173: fn os()
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
- crates/core/flags/hiargs.rs
  L36: pub(crate) struct HiArgs
  L113: pub(crate) fn from_low_args(mut low: LowArgs) -> anyhow::Result<HiArgs>
  L331: pub(crate) fn buffer_writer(&self) -> termcolor::BufferWriter
  L349: pub(crate) fn has_implicit_path(&self) -> bool
  L357: pub(crate) fn haystack_builder(&self) -> HaystackBuilder
  L368: pub(crate) fn matcher(&self) -> anyhow::Result<PatternMatcher>
  L411: fn matcher_pcre2(&self) -> anyhow::Result<PatternMatcher>
  L461: fn matcher_rust(&self) -> anyhow::Result<PatternMatcher>
  L521: pub(crate) fn matches_possible(&self) -> bool
  L536: pub(crate) fn mode(&self) -> Mode
  L545: pub(crate) fn path_printer_builder( &self, ) -> grep::printer::PathPrinterBuilder
  L561: pub(crate) fn printer<W: termcolor::WriteColor>( &self, search_mode: SearchMode, wtr: W, ) -> Printer<W>
  L595: fn printer_json<W: std::io::Write>( &self, wtr: W, ) -> grep::printer::JSON<W>
  L608: fn printer_standard<W: termcolor::WriteColor>( &self, wtr: W, ) -> grep::printer::Standard<W>
  L650: fn printer_summary<W: termcolor::WriteColor>( &self, wtr: W, kind: SummaryKind, ) -> grep::printer::Summary<W>
  L674: pub(crate) fn quiet(&self) -> bool
  L686: pub(crate) fn quit_after_match(&self) -> bool
  L694: pub(crate) fn search_worker<W: termcolor::WriteColor>( &self, matcher: PatternMatcher, searcher: grep::searcher::Searcher, printer: Printer<W>, ) -> anyhow::Result<SearchWorker<W>>
  L711: pub(crate) fn searcher(&self) -> anyhow::Result<grep::searcher::Searcher>
  L762: pub(crate) fn sort<'a, I>( &self, haystacks: I, ) -> Box<dyn Iterator<Item = Haystack> + 'a> where I: Iterator<Item = Haystack> + 'a,
  L822: pub(crate) fn stats(&self) -> Option<grep::printer::Stats>
  L831: pub(crate) fn stdout(&self) -> grep::cli::StandardStream
  L853: pub(crate) fn threads(&self) -> usize
  L861: pub(crate) fn types(&self) -> &ignore::types::Types
  L874: pub(crate) fn walk_builder(&self) -> anyhow::Result<ignore::WalkBuilder>
  L932: struct State
  L950: fn new() -> anyhow::Result<State>
  L967: struct Patterns
  L980: fn from_low_args( state: &mut State, low: &mut LowArgs, ) -> anyhow::Result<Patterns>
  L1055: struct Paths
- crates/core/search.rs
  L19: struct Config
  L28: fn default() -> Config
  L41: pub(crate) struct SearchWorkerBuilder
  L47: fn default() -> SearchWorkerBuilder
  L54: pub(crate) fn new() -> SearchWorkerBuilder
  L63: pub(crate) fn build<W: WriteColor>( &self, matcher: PatternMatcher, searcher: grep::searcher::Searcher, printer: Printer<W>, ) -> SearchWorker<W>
  L92: pub(crate) fn preprocessor( &mut self, cmd: Option<std::path::PathBuf>, ) -> anyhow::Result<&mut SearchWorkerBuilder>
  L108: pub(crate) fn preprocessor_globs( &mut self, globs: ignore::overrides::Override, ) -> &mut SearchWorkerBuilder
  L123: pub(crate) fn search_zip( &mut self, yes: bool, ) -> &mut SearchWorkerBuilder
  L139: pub(crate) fn binary_detection_implicit( &mut self, detection: grep::searcher::BinaryDetection, ) -> &mut SearchWorkerBuilder
  L155: pub(crate) fn binary_detection_explicit( &mut self, detection: grep::searcher::BinaryDetection, ) -> &mut SearchWorkerBuilder
  L171: pub(crate) struct SearchResult
  L178: pub(crate) fn has_match(&self) -> bool
  L186: pub(crate) fn stats(&self) -> Option<&grep::printer::Stats>
  L193: pub(crate) enum PatternMatcher
  L203: pub(crate) enum Printer<W>
  L215: pub(crate) fn get_mut(&mut self) -> &mut W
  L230: pub(crate) struct SearchWorker<W>
  L245: pub(crate) fn search( &mut self, haystack: &crate::haystack::Haystack, ) -> io::Result<SearchResult>
  L270: pub(crate) fn printer(&mut self) -> &mut Printer<W>
  L276: fn should_decompress(&self, path: &Path) -> bool
  L284: fn should_preprocess(&self, path: &Path) -> bool
  L296: fn search_preprocessor( &mut self, path: &Path, ) -> io::Result<SearchResult>
  L329: fn search_decompress(&mut self, path: &Path) -> io::Result<SearchResult>
  L342: fn search_path(&mut self, path: &Path) -> io::Result<SearchResult>
  L362: fn search_reader<R: io::Read>( &mut self, path: &Path, rdr: &mut R, ) -> io::Result<SearchResult>
  L380: fn search_path<M: Matcher, W: WriteColor>( matcher: M, searcher: &mut grep::searcher::Searcher, printer: &mut Printer<W>, path: &Path, ) -> io::Result<SearchResult>
  L416: fn search_reader<M: Matcher, R: io::Read, W: WriteColor>( matcher: M, searcher: &mut grep::searcher::Searcher, printer: &mut Printer<W>, path: &Path, mut rdr: R, ) -> io::Result<SearchResult>
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
- crates/ignore/src/incremental.rs
- crates/ignore/src/lib.rs
- crates/ignore/src/overrides.rs
- crates/ignore/src/types.rs
- crates/ignore/src/walk.rs
- crates/matcher/src/interpolate.rs
  L14: pub fn interpolate<A, N>( mut replacement: &[u8], mut append: A, mut name_to_index: N, dst: &mut Vec<u8>, ) where A: FnMut(usize, &mut Vec<u8>), N: FnMut(&str) -> Option<usize>,
  L64: struct CaptureRef<'a>
  L73: enum Ref<'a>
  L80: fn from(x: &'a str) -> Ref<'a>
  L97: fn find_cap_ref(replacement: &[u8]) -> Option<CaptureRef<'_>>
  L138: fn is_valid_cap_letter(b: &u8) -> bool
  L185: fn interpolate_string( mut name_to_index: Vec<(&'static str, usize)>, caps: Vec<&'static str>, replacement: &str, ) -> String
- crates/matcher/src/lib.rs
- crates/printer/src/hyperlink/mod.rs
- crates/printer/src/json.rs
- crates/printer/src/standard.rs
- crates/printer/src/stats.rs
- crates/printer/src/util.rs
- crates/regex/src/literal.rs
- crates/regex/src/strip.rs
- repo map (226 files):
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
  crates/ (139 files)
  fuzz/ (5 files)
  pkg/ (3 files)
  rustfmt.toml
  scripts/ (1 files)
  tests/ (19 files)

## Code
- sym:23098 Gitignore.empty (crates/ignore/src/gitignore.rs:151-160)
  pub fn empty() -> Gitignore
- sym:23095 Gitignore (crates/ignore/src/gitignore.rs:81-88)
  pub struct Gitignore
- sym:23000 incremental_gitignore_only (crates/ignore/benches/workloads.rs:563-576)
  fn incremental_gitignore_only(b: &mut test::Bencher)
- sym:23106 Gitignore.matched_stripped (crates/ignore/src/gitignore.rs:259-282)
  fn matched_stripped<P: AsRef<Path>>( &self, path: P, is_dir: bool, ) -> Match<&Glob>
- sym:23110 GitignoreBuilder.build (crates/ignore/src/gitignore.rs:349-366)
  pub fn build(&self) -> Result<Gitignore, Error>
- sym:23039 IgnoreBuilder (crates/ignore/src/dir.rs:753-776)
  pub(crate) struct IgnoreBuilder
- sym:23016 IgnoreInner (crates/ignore/src/dir.rs:115-166)
  struct IgnoreInner
- sym:22979 build_empty_gitignore (crates/ignore/benches/workloads.rs:304-307)
  fn build_empty_gitignore(b: &mut test::Bencher)
- sym:23108 GitignoreBuilder (crates/ignore/src/gitignore.rs:320-326)
  pub struct GitignoreBuilder
- sym:23100 Gitignore.is_empty (crates/ignore/src/gitignore.rs:171-173)
  pub fn is_empty(&self) -> bool
- sym:23042 IgnoreBuilder.build_with_cwd (crates/ignore/src/dir.rs:817-862)
  pub(crate) fn build_with_cwd(&self, cwd: Option<PathBuf>) -> Ignore
- sym:23111 GitignoreBuilder.build_global (crates/ignore/src/gitignore.rs:377-396)
  pub fn build_global(mut self) -> (Gitignore, Option<Error>)
- sym:22565 globs (crates/core/flags/hiargs.rs:1210-1231)
  fn globs( state: &State, low: &LowArgs, ) -> anyhow::Result<ignore::overrides::Override>
- sym:23097 Gitignore.global (crates/ignore/src/gitignore.rs:141-146)
  pub fn global() -> (Gitignore, Option<Error>)
- sym:22566 preprocessor_globs (crates/core/flags/hiargs.rs:1234-1246)
  fn preprocessor_globs( state: &State, low: &LowArgs, ) -> anyhow::Result<ignore::overrides::Override>
- sym:23025 Ignore.add_child_path_with_found_ignore_files (crates/ignore/src/dir.rs:340-467)
  fn add_child_path_with_found_ignore_files( &self, dir: &Path, ignore_files_list: Option<&IgnoreFilesFound>, ) -> (IgnoreInner, Option<Error>)
- sym:22996 match_single_file_glob (crates/ignore/benches/workloads.rs:487-505)
  fn match_single_file_glob(b: &mut test::Bencher)
- sym:23445 path_equals (crates/ignore/src/walk.rs:2123-2142)
  fn path_equals(dent: &DirEntry, handle: &Handle) -> Result<bool, Error>
- sym:23011 IgnoreMatch.gitignore (crates/ignore/src/dir.rs:56-58)
  fn gitignore(x: &'a gitignore::Glob) -> IgnoreMatch<'a>
- sym:22723 SearchWorker.should_preprocess (crates/core/search.rs:284-292)
  fn should_preprocess(&self, path: &Path) -> bool
- sym:23104 Gitignore.matched (crates/ignore/src/gitignore.rs:202-211)
  pub fn matched<P: AsRef<Path>>( &self, path: P, is_dir: bool, ) -> Match<&Glob>
- sym:23416 Work (crates/ignore/src/walk.rs:1560-1568)
  struct Work
- sym:23225 Match (crates/ignore/src/lib.rs:416-425)
  pub enum Match<T>
- sym:21864 IGlob.doc_long (crates/core/flags/defs.rs:3109-3118)
  fn doc_long(&self) -> &'static str
- sym:22982 match_python_gitignore (crates/ignore/benches/workloads.rs:326-349)
  fn match_python_gitignore(b: &mut test::Bencher)
- sym:23009 IgnoreMatchInner (crates/ignore/src/dir.rs:44-49)
  enum IgnoreMatchInner<'a>
- sym:23239 GlobInner (crates/ignore/src/overrides.rs:32-37)
  enum GlobInner<'a>
- sym:22978 incremental_ignored (crates/ignore/benches/workloads.rs:293-301)
  fn incremental_ignored(b: &mut test::Bencher)
- sym:21798 Glob.doc_long (crates/core/flags/defs.rs:2525-2552)
  fn doc_long(&self) -> &'static str
- sym:23015 Ignore (crates/ignore/src/dir.rs:94-112)
  pub(crate) struct Ignore
- sym:23428 Worker (crates/ignore/src/walk.rs:1721-1754)
  struct Worker<'s>
- sym:23432 Worker.get_work (crates/ignore/src/walk.rs:1942-1992)
  fn get_work(&mut self) -> Option<Work>
- sym:22990 incremental_dense_ignore_cold (crates/ignore/benches/workloads.rs:424-433)
  fn incremental_dense_ignore_cold(b: &mut test::Bencher)
- sym:23109 GitignoreBuilder.new (crates/ignore/src/gitignore.rs:335-344)
  pub fn new<P: AsRef<Path>>(root: P) -> GitignoreBuilder
- sym:23242 Override.empty (crates/ignore/src/overrides.rs:51-53)
  pub fn empty() -> Override
- sym:23046 IgnoreBuilder.add_ignore (crates/ignore/src/dir.rs:897-900)
  pub(crate) fn add_ignore(&mut self, ig: Gitignore) -> &mut IgnoreBuilder
- sym:23366 WalkBuilder.add_ignore (crates/ignore/src/walk.rs:791-812)
  pub fn add_ignore<P: AsRef<Path>>(&mut self, path: P) -> Option<Error>
- sym:23227 Match.is_ignore (crates/ignore/src/lib.rs:437-442)
  pub fn is_ignore(&self) -> bool
- sym:23301 TypesBuilder.add_def (crates/ignore/src/types.rs:446-483)
  pub fn add_def(&mut self, def: &str) -> Result<(), Error>
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
- sym:22703 Config.default (crates/core/search.rs:28-36)
  fn default() -> Config
- sym:22842 GlobSet (crates/globset/src/lib.rs:309-312)
  pub struct GlobSet
- sym:22851 GlobSet.matches (crates/globset/src/lib.rs:401-403)
  pub fn matches<P: AsRef<Path>>(&self, path: P) -> Vec<usize>
- sym:22855 GlobSet.new (crates/globset/src/lib.rs:461-553)
  pub fn new<I, G>(globs: I) -> Result<GlobSet, Error> where I: IntoIterator<Item = G>, G: AsRef<Glob>,
- sym:22856 GlobSet.default (crates/globset/src/lib.rs:558-560)
  fn default() -> Self
- sym:23040 IgnoreBuilder.new (crates/ignore/src/dir.rs:784-803)
  pub(crate) fn new() -> IgnoreBuilder
- sym:23089 Glob.from (crates/ignore/src/gitignore.rs:47-49)
  pub fn from(&self) -> Option<&Path>
- sym:23096 Gitignore.new (crates/ignore/src/gitignore.rs:103-118)
  pub fn new<P: AsRef<Path>>( gitignore_path: P, ) -> (Gitignore, Option<Error>)
- sym:23102 Gitignore.num_ignores (crates/ignore/src/gitignore.rs:182-184)
  pub fn num_ignores(&self) -> u64
- sym:23103 Gitignore.num_whitelists (crates/ignore/src/gitignore.rs:187-189)
  pub fn num_whitelists(&self) -> u64
- sym:23142 IncrementalIgnore.root (crates/ignore/src/incremental.rs:134-136)
  pub fn root(&self) -> &Path
- sym:23218 Error.from (crates/ignore/src/lib.rs:367-369)
  fn from(err: std::io::Error) -> Error
- sym:23245 Override.num_ignores (crates/ignore/src/overrides.rs:70-72)
  pub fn num_ignores(&self) -> u64
- sym:23246 Override.num_whitelists (crates/ignore/src/overrides.rs:75-77)
  pub fn num_whitelists(&self) -> u64
- sym:23281 FileTypeDef.globs (crates/ignore/src/types.rs:158-160)
  pub fn globs(&self) -> &[String]
- sym:23356 WalkBuilder.from_iter (crates/ignore/src/walk.rs:582-590)
  pub fn from_iter<P: AsRef<Path>, I: IntoIterator<Item = P>>( paths: I, ) -> WalkBuilder
- sym:23395 WalkEventIter.from (crates/ignore/src/walk.rs:1284-1286)
  fn from(it: WalkDir) -> WalkEventIter
- sym:23496 Ref.from (crates/matcher/src/interpolate.rs:80-82)
  fn from(x: &'a str) -> Ref<'a>
- sym:23548 Error.from (crates/matcher/src/lib.rs:512-514)
  fn from(_: NoError) -> std::io::Error
- sym:23813 JSONSink.context (crates/printer/src/json.rs:755-783)
  fn context( &mut self, searcher: &Searcher, ctx: &SinkContext<'_>, ) -> Result<bool, io::Error>
- sym:23911 StandardImpl.new (crates/printer/src/standard.rs:886-896)
  fn new( searcher: &'a Searcher, sink: &'a StandardSink<'_, '_, M, W>, ) -> StandardImpl<'a, M, W>
- sym:24049 Stats.matches (crates/printer/src/stats.rs:67-69)
  pub fn matches(&self) -> u64
- sym:24143 Sunk.matches (crates/printer/src/util.rs:238-240)
  pub(crate) fn matches(&self) -> &'a [Match]
- sym:24216 Extractor.extract_alternation (crates/regex/src/literal.rs:231-247)
  fn extract_alternation<'a, I: Iterator<Item = &'a Hir>>( &self, it: I, ) -> TSeq
- sym:24217 Extractor.extract_repetition (crates/regex/src/literal.rs:264-315)
  fn extract_repetition(&self, rep: &hir::Repetition) -> TSeq
- sym:24218 Extractor.extract_class_unicode (crates/regex/src/literal.rs:320-332)
  fn extract_class_unicode(&self, cls: &hir::ClassUnicode) -> TSeq
- sym:24219 Extractor.extract_class_bytes (crates/regex/src/literal.rs:336-348)
  fn extract_class_bytes(&self, cls: &hir::ClassBytes) -> TSeq
- sym:24333 strip_from_match_ascii (crates/regex/src/strip.rs:55-119)
  fn strip_from_match_ascii(expr: Hir, byte: u8) -> Result<Hir, Error>
- sym:23186 compiled_multi_matchers_are_reused (crates/ignore/src/incremental.rs:881-898)
  fn compiled_multi_matchers_are_reused()
- sym:23068 gitignore_no_git (crates/ignore/src/dir.rs:1199-1208)
  fn gitignore_no_git()
- sym:23256 empty (crates/ignore/src/overrides.rs:201-206)
  fn empty()
- sym:23081 stops_at_git_dir (crates/ignore/src/dir.rs:1365-1385)
  fn stops_at_git_dir()
- sym:23069 gitignore_allowed_no_git (crates/ignore/src/dir.rs:1211-1223)
  fn gitignore_allowed_no_git()
- sym:23084 absolute_parent_matchers_are_cached_across_roots (crates/ignore/src/dir.rs:1431-1451)
  fn absolute_parent_matchers_are_cached_across_roots()
- sym:23176 glob_overrides_are_applied (crates/ignore/src/incremental.rs:699-712)
  fn glob_overrides_are_applied()
- sym:23181 respects_git_repository_boundaries (crates/ignore/src/incremental.rs:788-806)
  fn respects_git_repository_boundaries()
- sym:23074 ignore_over_gitignore (crates/ignore/src/dir.rs:1289-1297)
  fn ignore_over_gitignore()
- sym:24524 scratch (crates/searcher/src/searcher/glue.rs:1495-1521)
  fn scratch()
- sym:23258 only_ignores (crates/ignore/src/overrides.rs:220-226)
  fn only_ignores()
- sym:23778 build_empty_format (crates/printer/src/hyperlink/mod.rs:973-979)
  fn build_empty_format()

---
tokens: 7192/8000
