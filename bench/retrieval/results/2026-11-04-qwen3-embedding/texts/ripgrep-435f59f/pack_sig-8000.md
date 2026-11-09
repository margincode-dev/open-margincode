## Task
# ripgrep-435f59f · ripgrep-435f59f · open
Goal: ignore: skip loading unreachable ignore files

When parallelism isn't used (e.g., when sorting results), then
it's possible in some cases for the `ignore` crate to still try and
load gitignore files inside of directories that are never
descended into. This can occur when `--max-depth` is used, or
even just when a directory is skipped for some other reason.

Fixes #2565, PR #3496
Workspace: /root/bench/ws/rebf815-ripgrep-435f59f/ripgrep-435f59f
Changed (0): nothing yet

## Map
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
- crates/core/flags/hiargs.rs
  L36: pub(crate) struct HiArgs
  L114: pub(crate) fn from_low_args(mut low: LowArgs) -> anyhow::Result<HiArgs>
  L342: pub(crate) fn buffer_writer(&self) -> termcolor::BufferWriter
  L360: pub(crate) fn has_implicit_path(&self) -> bool
  L368: pub(crate) fn haystack_builder(&self) -> HaystackBuilder
  L379: pub(crate) fn matcher(&self) -> anyhow::Result<PatternMatcher>
  L422: fn matcher_pcre2(&self) -> anyhow::Result<PatternMatcher>
  L472: fn matcher_rust(&self) -> anyhow::Result<PatternMatcher>
  L532: pub(crate) fn matches_possible(&self) -> bool
  L547: pub(crate) fn mode(&self) -> Mode
  L556: pub(crate) fn path_printer_builder( &self, ) -> grep::printer::PathPrinterBuilder
  L572: pub(crate) fn printer<W: termcolor::WriteColor>( &self, search_mode: SearchMode, wtr: W, ) -> Printer<W>
  L606: fn printer_json<W: std::io::Write>( &self, wtr: W, ) -> grep::printer::JSON<W>
  L619: fn printer_standard<W: termcolor::WriteColor>( &self, wtr: W, ) -> grep::printer::Standard<W>
  L661: fn printer_summary<W: termcolor::WriteColor>( &self, wtr: W, kind: SummaryKind, ) -> grep::printer::Summary<W>
  L685: pub(crate) fn quiet(&self) -> bool
  L697: pub(crate) fn quit_after_match(&self) -> bool
  L705: pub(crate) fn search_worker<W: termcolor::WriteColor>( &self, matcher: PatternMatcher, searcher: grep::searcher::Searcher, printer: Printer<W>, ) -> anyhow::Result<SearchWorker<W>>
  L722: pub(crate) fn searcher(&self) -> anyhow::Result<grep::searcher::Searcher>
  L773: pub(crate) fn sort<'a, I>( &self, haystacks: I, ) -> Box<dyn Iterator<Item = Haystack> + 'a> where I: Iterator<Item = Haystack> + 'a,
  L833: pub(crate) fn stats(&self) -> Option<grep::printer::Stats>
  L842: pub(crate) fn stdout(&self) -> grep::cli::StandardStream
  L864: pub(crate) fn threads(&self) -> usize
  L872: pub(crate) fn types(&self) -> &ignore::types::Types
  L885: pub(crate) fn walk_builder(&self) -> anyhow::Result<ignore::WalkBuilder>
  L931: pub(crate) fn index(&self) -> usize
  L936: pub(crate) fn index_write(&self) -> anyhow::Result<grep_index::Index>
  L949: pub(crate) fn index_read(&self) -> anyhow::Result<grep_index::Index>
  L968: struct State
  L986: fn new() -> anyhow::Result<State>
- crates/core/flags/lowargs.rs
  L33: pub(crate) struct LowArgs
  L125: pub(crate) enum SpecialMode
  L156: pub(crate) enum Mode
  L172: fn default() -> Mode
  L181: pub(crate) fn update(&mut self, new: Mode)
  L199: pub(crate) enum IndexMode
  L206: pub(crate) enum SearchMode
  L230: pub(crate) enum GenerateMode
  L245: pub(crate) enum BinaryMode
  L269: pub(crate) enum BoundaryMode
  L280: pub(crate) enum BufferMode
  L302: pub(crate) enum CaseMode
  L318: pub(crate) enum ColorChoice
  L338: pub(crate) fn to_termcolor(&self) -> termcolor::ColorChoice
  L352: pub(crate) enum ContextMode
  L360: fn default() -> ContextMode
  L371: pub(crate) fn set_before(&mut self, lines: usize)
  L392: pub(crate) fn set_after(&mut self, lines: usize)
  L412: pub(crate) fn set_both(&mut self, lines: usize)
  L430: pub(crate) fn get_limited(&self) -> (usize, usize)
  L452: pub(crate) struct ContextModeLimited
  L464: pub(crate) fn get(&self) -> (usize, usize)
  L484: pub(crate) struct ContextSeparator
  L487: fn default() -> ContextSeparator
  L495: pub(crate) fn new(os: &OsStr) -> anyhow::Result<ContextSeparator>
  L507: pub(crate) fn disabled() -> ContextSeparator
  L516: pub(crate) fn into_bytes(self) -> Option<Vec<u8>>
  L525: pub(crate) enum EncodingMode
  L541: pub(crate) enum EngineChoice
  L562: pub(crate) struct FieldContextSeparator
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
- sym:33165 Ignore (crates/ignore/src/dir.rs:94-112)
  pub(crate) struct Ignore
- sym:33175 Ignore.add_child_path_with_found_ignore_files (crates/ignore/src/dir.rs:340-467)
  fn add_child_path_with_found_ignore_files( &self, dir: &Path, ignore_files_list: Option<&IgnoreFilesFound>, ) -> (IgnoreInner, Option<Error>)
- sym:32739 HiArgs.walk_builder (crates/core/flags/hiargs.rs:885-929)
  pub(crate) fn walk_builder(&self) -> anyhow::Result<ignore::WalkBuilder>
- sym:32509 Sort.doc_long (crates/core/flags/defs.rs:6694-6728)
  fn doc_long(&self) -> &'static str
- sym:32519 Sortr.doc_long (crates/core/flags/defs.rs:6795-6830)
  fn doc_long(&self) -> &'static str
- sym:33299 IncrementalIgnore.root_ignore (crates/ignore/src/incremental.rs:413-433)
  fn root_ignore( &mut self, errs: &mut PartialErrorBuilder, ) -> Option<Ignore>
- sym:32809 SortMode.supported (crates/core/flags/lowargs.rs:689-720)
  pub(crate) fn supported(&self) -> anyhow::Result<()>
- sym:33207 create_gitignore (crates/ignore/src/dir.rs:1016-1051)
  pub(crate) fn create_gitignore<T: AsRef<OsStr>>( dir: &Path, dir_for_ignorefile: &Path, names: &[T], case_insensitive: bool, ) -> (Gitignore, Option<Error>)
- sym:33544 WalkState (crates/ignore/src/walk.rs:1318-1330)
  pub enum WalkState
- sym:33189 IgnoreBuilder (crates/ignore/src/dir.rs:753-776)
  pub(crate) struct IgnoreBuilder
- sym:33294 IncrementalIgnore.matched (crates/ignore/src/incremental.rs:194-204)
  pub fn matched<P: AsRef<Path>>( &mut self, path: P, is_dir: bool, ) -> IncrementalMatch
- sym:32805 MmapMode (crates/core/flags/lowargs.rs:637-650)
  pub(crate) enum MmapMode
- sym:32893 ignore_messages (crates/core/messages.rs:113-115)
  pub(crate) fn ignore_messages() -> bool
- sym:32775 BinaryMode (crates/core/flags/lowargs.rs:245-265)
  pub(crate) enum BinaryMode
- sym:33287 IncrementalIgnore (crates/ignore/src/incremental.rs:57-68)
  pub struct IncrementalIgnore
- sym:31900 FilesWithMatches.doc_long (crates/core/flags/defs.rs:2269-2283)
  fn doc_long(&self) -> &'static str
- sym:32779 ColorChoice (crates/core/flags/lowargs.rs:318-334)
  pub(crate) enum ColorChoice
- sym:33196 IgnoreBuilder.add_ignore (crates/ignore/src/dir.rs:897-900)
  pub(crate) fn add_ignore(&mut self, ig: Gitignore) -> &mut IgnoreBuilder
- sym:33166 IgnoreInner (crates/ignore/src/dir.rs:115-166)
  struct IgnoreInner
- sym:32298 NoIgnoreVcs.doc_long (crates/core/flags/defs.rs:5074-5089)
  fn doc_long(&self) -> &'static str
- sym:31660 generate (crates/core/flags/complete/bash.rs:63-107)
  pub(crate) fn generate() -> String
- sym:33164 IgnoreOptions (crates/ignore/src/dir.rs:72-90)
  struct IgnoreOptions
- sym:33290 CachedDir (crates/ignore/src/incremental.rs:94-103)
  enum CachedDir
- sym:33179 Ignore.matched_ignore (crates/ignore/src/dir.rs:548-685)
  pub(crate) fn matched_ignore<'a>( &'a self, path: &Path, is_dir: bool, ) -> Match<IgnoreMatch<'a>>
- sym:33275 home_dir (crates/ignore/src/gitignore.rs:697-702)
  fn home_dir() -> Option<PathBuf>
- sym:32705 generate_flag (crates/core/flags/doc/man.rs:52-116)
  fn generate_flag(flag: &'static dyn Flag, out: &mut String)
- sym:33203 IgnoreBuilder.git_ignore (crates/ignore/src/dir.rs:970-973)
  pub(crate) fn git_ignore(&mut self, yes: bool) -> &mut IgnoreBuilder
- sym:33496 WalkBuilder (crates/ignore/src/walk.rs:488-512)
  pub struct WalkBuilder
- sym:32500 SortFiles.doc_long (crates/core/flags/defs.rs:6606-6616)
  fn doc_long(&self) -> &'static str
- sym:33172 Ignore.add_child_with_entries (crates/ignore/src/dir.rs:284-301)
  pub(crate) fn add_child_with_entries<P: AsRef<Path>>( &self, dir: P, entries: &[fs::DirEntry], ) -> (Ignore, Option<Error>)
- sym:33245 Gitignore (crates/ignore/src/gitignore.rs:81-88)
  pub struct Gitignore
- sym:32891 messages (crates/core/messages.rs:101-103)
  pub(crate) fn messages() -> bool
- sym:33288 IncrementalIgnoreOptions (crates/ignore/src/incremental.rs:73-79)
  pub(crate) struct IncrementalIgnoreOptions
- sym:32723 HiArgs.matches_possible (crates/core/flags/hiargs.rs:532-540)
  pub(crate) fn matches_possible(&self) -> bool
- sym:32701 generate_long_flag (crates/core/flags/doc/help.rs:149-226)
  fn generate_long_flag(flag: &dyn Flag, out: &mut String)
- sym:33192 IgnoreBuilder.build_with_cwd (crates/ignore/src/dir.rs:817-862)
  pub(crate) fn build_with_cwd(&self, cwd: Option<PathBuf>) -> Ignore
- sym:32895 errored (crates/core/messages.rs:129-131)
  pub(crate) fn errored() -> bool
- sym:32878 run (crates/core/main.rs:78-107)
  fn run(result: crate::flags::ParseResult<HiArgs>) -> anyhow::Result<ExitCode>
- sym:31548 DecompressionMatcherBuilder.new (crates/cli/src/decompress.rs:42-44)
  pub fn new() -> DecompressionMatcherBuilder
- sym:31549 DecompressionMatcherBuilder.build (crates/cli/src/decompress.rs:50-69)
  pub fn build(&self) -> Result<DecompressionMatcher, CommandError>
- sym:31555 DecompressionMatcher.new (crates/cli/src/decompress.rs:167-171)
  pub fn new() -> DecompressionMatcher
- sym:31559 DecompressionReaderBuilder.new (crates/cli/src/decompress.rs:205-207)
  pub fn new() -> DecompressionReaderBuilder
- sym:31560 DecompressionReaderBuilder.build (crates/cli/src/decompress.rs:221-244)
  pub fn build<P: AsRef<Path>>( &self, path: P, ) -> Result<DecompressionReader, CommandError>
- sym:31565 DecompressionReader.new (crates/cli/src/decompress.rs:352-356)
  pub fn new<P: AsRef<Path>>( path: P, ) -> Result<DecompressionReader, CommandError>
- sym:31631 CommandReaderBuilder.new (crates/cli/src/process.rs:90-92)
  pub fn new() -> CommandReaderBuilder
- sym:31632 CommandReaderBuilder.build (crates/cli/src/process.rs:104-118)
  pub fn build( &self, command: &mut process::Command, ) -> Result<CommandReader, CommandError>
- sym:31635 CommandReader.new (crates/cli/src/process.rs:195-199)
  pub fn new( cmd: &mut process::Command, ) -> Result<CommandReader, CommandError>
- sym:32715 HiArgs (crates/core/flags/hiargs.rs:36-107)
  pub(crate) struct HiArgs
- sym:32738 HiArgs.types (crates/core/flags/hiargs.rs:872-874)
  pub(crate) fn types(&self) -> &ignore::types::Types
- sym:32744 State.new (crates/core/flags/hiargs.rs:986-996)
  fn new() -> anyhow::Result<State>
- sym:32752 BinaryDetection.is_none (crates/core/flags/hiargs.rs:1215-1218)
  pub(crate) fn is_none(&self) -> bool
- sym:32753 types (crates/core/flags/hiargs.rs:1222-1242)
  fn types(low: &LowArgs) -> anyhow::Result<ignore::types::Types>
- sym:32754 globs (crates/core/flags/hiargs.rs:1246-1267)
  fn globs( state: &State, low: &LowArgs, ) -> anyhow::Result<ignore::overrides::Override>
- sym:32755 preprocessor_globs (crates/core/flags/hiargs.rs:1270-1282)
  fn preprocessor_globs( state: &State, low: &LowArgs, ) -> anyhow::Result<ignore::overrides::Override>
- sym:32791 ContextSeparator.new (crates/core/flags/lowargs.rs:495-503)
  pub(crate) fn new(os: &OsStr) -> anyhow::Result<ContextSeparator>
- sym:32798 FieldContextSeparator.new (crates/core/flags/lowargs.rs:573-581)
  pub(crate) fn new(os: &OsStr) -> anyhow::Result<FieldContextSeparator>
- sym:32802 FieldMatchSeparator.new (crates/core/flags/lowargs.rs:607-615)
  pub(crate) fn new(os: &OsStr) -> anyhow::Result<FieldMatchSeparator>
- sym:32839 Parser.new (crates/core/flags/parse.rs:176-216)
  fn new() -> &'static Parser
- sym:32848 FlagMap.new (crates/core/flags/parse.rs:374-387)
  fn new(infos: &[FlagInfo]) -> FlagMap
- sym:32856 HaystackBuilder.new (crates/core/haystack.rs:22-24)
  pub(crate) fn new() -> HaystackBuilder
- sym:32857 HaystackBuilder.build_from_result (crates/core/haystack.rs:31-42)
  pub(crate) fn build_from_result( &self, result: Result<ignore::DirEntry, ignore::Error>, ) -> Option<Haystack>
- sym:32858 HaystackBuilder.build (crates/core/haystack.rs:49-79)
  fn build(&self, dent: ignore::DirEntry) -> Option<Haystack>
- sym:32860 Haystack (crates/core/haystack.rs:98-101)
  pub(crate) struct Haystack
- sym:32861 Haystack.path (crates/core/haystack.rs:108-114)
  pub(crate) fn path(&self) -> &Path
- sym:32897 Config (crates/core/search.rs:19-25)
  struct Config
- sym:32898 Config.default (crates/core/search.rs:28-36)
  fn default() -> Config
- sym:32901 SearchWorkerBuilder.new (crates/core/search.rs:54-59)
  pub(crate) fn new() -> SearchWorkerBuilder
- sym:32902 SearchWorkerBuilder.build (crates/core/search.rs:63-85)
  pub(crate) fn build<W: WriteColor>( &self, matcher: PatternMatcher, searcher: grep::searcher::Searcher, printer: Printer<W>, ) -> SearchWorker<W>
- sym:32904 SearchWorkerBuilder.preprocessor_globs (crates/core/search.rs:108-114)
  pub(crate) fn preprocessor_globs( &mut self, globs: ignore::overrides::Override, ) -> &mut SearchWorkerBuilder
- sym:32952 MatchStrategy.new (crates/globset/src/glob.rs:51-67)
  pub(crate) fn new(pat: &Glob) -> MatchStrategy
- sym:32976 Glob.new (crates/globset/src/glob.rs:283-285)
  pub fn new(glob: &str) -> Result<Glob, Error>
- sym:32988 GlobBuilder.new (crates/globset/src/glob.rs:574-576)
  pub fn new(glob: &'a str) -> GlobBuilder<'a>
- sym:32989 GlobBuilder.build (crates/globset/src/glob.rs:579-610)
  pub fn build(&self) -> Result<Glob, Error>
- sym:33050 GlobSet.new (crates/globset/src/lib.rs:461-553)
  pub fn new<I, G>(globs: I) -> Result<GlobSet, Error> where I: IntoIterator<Item = G>, G: AsRef<Glob>,
- sym:33053 GlobSetBuilder.new (crates/globset/src/lib.rs:574-576)
  pub fn new() -> GlobSetBuilder
- sym:33054 GlobSetBuilder.build (crates/globset/src/lib.rs:581-583)
  pub fn build(&self) -> Result<GlobSet, Error>
- sym:33058 Candidate.new (crates/globset/src/lib.rs:617-619)
  pub fn new<P: AsRef<Path> + ?Sized>(path: &'a P) -> Candidate<'a>
- sym:33339 parent_loading_can_be_disabled (crates/ignore/src/incremental.rs:958-976)
  fn parent_loading_can_be_disabled()
- sym:33220 ignore (crates/ignore/src/dir.rs:1226-1235)
  fn ignore()
- sym:33329 directly_ignored_directory_is_not_descended (crates/ignore/src/incremental.rs:751-761)
  fn directly_ignored_directory_is_not_descended()
- sym:33229 errored_partial_and_ignore (crates/ignore/src/dir.rs:1346-1354)
  fn errored_partial_and_ignore()
- sym:33348 max_filesize_does_not_stat_ignored_file (crates/ignore/src/incremental.rs:1159-1169)
  fn max_filesize_does_not_stat_ignored_file()
- sym:33635 IGNORE_FILE (crates/ignore/tests/gitignore_skip_bom.rs:3-3)
  const IGNORE_FILE: &'static str = "tests/gitignore_skip_bom.gitignore"
- sym:33235 multi_root_gitignore_order_independent (crates/ignore/src/dir.rs:1464-1520)
  fn multi_root_gitignore_order_independent()
- sym:33231 stops_at_git_dir (crates/ignore/src/dir.rs:1365-1385)
  fn stops_at_git_dir()
- sym:33636 gitignore_skip_bom (crates/ignore/tests/gitignore_skip_bom.rs:10-17)
  fn gitignore_skip_bom()
- sym:33224 ignore_over_gitignore (crates/ignore/src/dir.rs:1289-1297)
  fn ignore_over_gitignore()
- sym:33218 gitignore_no_git (crates/ignore/src/dir.rs:1199-1208)
  fn gitignore_no_git()
- sym:33330 ignored_ancestor_wins (crates/ignore/src/incremental.rs:767-783)
  fn ignored_ancestor_wins()
- sym:33219 gitignore_allowed_no_git (crates/ignore/src/dir.rs:1211-1223)
  fn gitignore_allowed_no_git()
- sym:33331 respects_git_repository_boundaries (crates/ignore/src/incremental.rs:788-806)
  fn respects_git_repository_boundaries()
- sym:33214 explicit_ignore (crates/ignore/src/dir.rs:1145-1157)
  fn explicit_ignore()
- sym:33621 same_file_system (crates/ignore/src/walk.rs:2565-2591)
  fn same_file_system()
- sym:33236 multi_root_custom_ignore_order_independent (crates/ignore/src/dir.rs:1527-1567)
  fn multi_root_custom_ignore_order_independent()
- sym:33228 errored_partial (crates/ignore/src/dir.rs:1335-1343)
  fn errored_partial()
- sym:33325 parent_ignore_and_custom_ignore (crates/ignore/src/incremental.rs:679-695)
  fn parent_ignore_and_custom_ignore()
- sym:33227 errored_both (crates/ignore/src/dir.rs:1325-1332)
  fn errored_both()
- sym:33322 nested_parent_gitignores (crates/ignore/src/incremental.rs:629-643)
  fn nested_parent_gitignores()
- sym:33222 custom_ignore_over_ignore (crates/ignore/src/dir.rs:1255-1267)
  fn custom_ignore_over_ignore()
- sym:33337 compiled_multi_child_matchers_are_not_reused (crates/ignore/src/incremental.rs:901-936)
  fn compiled_multi_child_matchers_are_not_reused()
- sym:33631 test_files_in_root (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:25-54)
  fn test_files_in_root()
- sym:33352 descendants_of_hidden_directories_are_ignored (crates/ignore/src/incremental.rs:1236-1250)
  fn descendants_of_hidden_directories_are_ignored()

---
tokens: 7150/8000
