## Task
# ripgrep-7155f64 · ripgrep-7155f64 · open
Goal: ignore: treat dotfiles ending in a dot as hidden (#3564)

`file_name()` returned `None` for any path ending in `.` on Unix,
which was meant to cover the `.` and `..` components but also matched
real names like `.foo.`. The hidden check then found no file name and
treated those files as visible. Only a final component that is exactly
`.` or `..` has no file name now.

PR #3564
Workspace: /root/bench/ws/rebf815-ripgrep-7155f64/ripgrep-7155f64
Changed (0): nothing yet

## Map
- crates/globset/src/pathutil.rs
  L9: pub(crate) fn file_name<'a>(path: &Cow<'a, [u8]>) -> Option<Cow<'a, [u8]>>
  L40: pub(crate) fn file_name_ext<'a>( name: &Cow<'a, [u8]>, ) -> Option<Cow<'a, [u8]>>
  L59: pub(crate) fn normalize_path(path: Cow<'_, [u8]>) -> Cow<'_, [u8]>
- crates/ignore/src/incremental.rs
  L57: pub struct IncrementalIgnore
  L73: pub(crate) struct IncrementalIgnoreOptions
  L82: enum RootIgnore
  L94: enum CachedDir
  L106: pub(crate) fn new( root: PathBuf, ignore: Ignore, options: IncrementalIgnoreOptions, ) -> IncrementalIgnore
  L134: pub fn root(&self) -> &Path
  L155: pub fn normalize<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf>
  L194: pub fn matched<P: AsRef<Path>>( &mut self, path: P, is_dir: bool, ) -> IncrementalMatch
  L213: pub fn matched_with_errors<P: AsRef<Path>>( &mut self, path: P, is_dir: bool, ) -> (IncrementalMatch, Option<Error>)
  L224: fn matched_with_errors_impl( &mut self, relative: &Path, is_dir: bool, errs: &mut PartialErrorBuilder, ) -> IncrementalMatch
  L310: fn matched_with_errors_ignore( &mut self, relative: &Path, is_dir: bool, errs: &mut PartialErrorBuilder, ) -> Option<IncrementalMatch>
  L393: fn match_path( &self, ignore: &Ignore, relative: &Path, is_dir: bool, ) -> IncrementalMatch
  L413: fn root_ignore( &mut self, errs: &mut PartialErrorBuilder, ) -> Option<Ignore>
  L449: pub struct IncrementalMatch
  L456: fn none(is_dir: bool) -> IncrementalMatch
  L464: fn ignore() -> IncrementalMatch
  L472: fn from_match(mat: Match<()>, is_dir: bool) -> IncrementalMatch
  L477: fn no_descent(self) -> IncrementalMatch
  L481: fn not_within_depth(self) -> IncrementalMatch
  L486: pub fn is_none(&self) -> bool
  L491: pub fn is_ignore(&self) -> bool
  L497: pub fn is_whitelist(&self) -> bool
  L513: pub fn should_descend(&self) -> bool
  L526: pub fn is_within_depth(&self) -> bool
  L532: pub fn invert(self) -> IncrementalMatch
  L542: fn normalize_absolute(path: &Path) -> Option<PathBuf>
  L572: fn wfile<P: AsRef<Path>>(path: P, contents: &str)
  L577: fn mkdirp<P: AsRef<Path>>(path: P)
  L581: fn tmpdir() -> TempDir
  L585: fn builder<P: AsRef<Path>>(path: P) -> WalkBuilder
- crates/ignore/src/pathutil.rs
  L19: pub(crate) fn is_hidden_path(dent: &Path) -> bool
  L56: pub(crate) fn is_hidden_entry(dent: &DirEntry) -> bool
  L85: fn is_hidden_path_only(path: &Path) -> bool
  L96: pub(crate) fn strip_prefix<'a, P: AsRef<Path> + ?Sized>( prefix: &'a P, path: &'a Path, ) -> Option<&'a Path>
  L123: pub(crate) fn is_file_name<P: AsRef<Path>>(path: P) -> bool
  L142: pub(crate) fn file_name<'a, P: AsRef<Path> + ?Sized>( path: &'a P, ) -> Option<&'a OsStr>
- crates/ignore/src/walk.rs
  L31: pub struct DirEntry
  L38: pub fn path(&self) -> &Path
  L44: pub fn into_path(self) -> PathBuf
  L49: pub fn path_is_symlink(&self) -> bool
  L56: pub fn is_stdin(&self) -> bool
  L61: pub fn metadata(&self) -> Result<Metadata, Error>
  L68: pub fn file_type(&self) -> Option<FileType>
  L76: pub fn file_name(&self) -> &OsStr
  L81: pub fn depth(&self) -> usize
  L89: pub fn ino(&self) -> Option<u64>
  L98: pub fn error(&self) -> Option<&Error>
  L103: pub(crate) fn is_dir(&self) -> bool
  L107: fn new_stdin() -> DirEntry
  L111: fn new_walkdir(dent: walkdir::DirEntry, err: Option<Error>) -> DirEntry
  L115: fn new_raw(dent: DirEntryRaw, err: Option<Error>) -> DirEntry
  L131: enum DirEntryInner
  L138: fn path(&self) -> &Path
  L147: fn into_path(self) -> PathBuf
  L156: fn path_is_symlink(&self) -> bool
  L165: fn is_stdin(&self) -> bool
  L172: fn metadata(&self) -> Result<Metadata, Error>
  L191: fn file_type(&self) -> Option<FileType>
  L200: fn file_name(&self) -> &OsStr
  L209: fn depth(&self) -> usize
  L219: fn ino(&self) -> Option<u64>
  L230: fn is_dir(&self) -> bool
  L238: struct DirEntryRaw
  L259: fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  L272: fn path(&self) -> &Path
  L276: fn into_path(self) -> PathBuf
- crates/regex/src/literal.rs
  L40: pub(crate) struct InnerLiterals
  L54: pub(crate) fn new(chir: &ConfiguredHIR, re: &Regex) -> InnerLiterals
  L96: pub(crate) fn none() -> InnerLiterals
  L106: pub(crate) fn one_regex(&self) -> Result<Option<Regex>, Error>
  L131: struct Extractor
  L140: fn new() -> Extractor
  L151: fn extract_untagged(&self, hir: &Hir) -> Seq
  L169: fn extract(&self, hir: &Hir) -> TSeq
  L196: fn extract_concat<'a, I: Iterator<Item = &'a Hir>>(&self, it: I) -> TSeq
  L231: fn extract_alternation<'a, I: Iterator<Item = &'a Hir>>( &self, it: I, ) -> TSeq
  L264: fn extract_repetition(&self, rep: &hir::Repetition) -> TSeq
  L320: fn extract_class_unicode(&self, cls: &hir::ClassUnicode) -> TSeq
  L336: fn extract_class_bytes(&self, cls: &hir::ClassBytes) -> TSeq
  L352: fn class_over_limit_unicode(&self, cls: &hir::ClassUnicode) -> bool
  L365: fn class_over_limit_bytes(&self, cls: &hir::ClassBytes) -> bool
  L379: fn cross(&self, mut seq1: TSeq, mut seq2: TSeq) -> TSeq
  L398: fn union(&self, mut seq1: TSeq, seq2: &mut TSeq) -> TSeq
  L434: fn enforce_literal_len(&self, seq: &mut TSeq)
  L440: struct TSeq
  L447: fn empty() -> TSeq
  L451: fn infinite() -> TSeq
  L455: fn singleton(lit: Literal) -> TSeq
  L459: fn new<I, B>(it: I) -> TSeq where I: IntoIterator<Item = B>, B: AsRef<[u8]>,
  L467: fn literals(&self) -> Option<&[Literal]>
  L471: fn push(&mut self, lit: Literal)
  L475: fn make_inexact(&mut self)
  L479: fn make_infinite(&mut self)
  L483: fn cross_forward(&mut self, other: &mut TSeq)
  L488: fn union(&mut self, other: &mut TSeq)
  L492: fn dedup(&mut self)
- crates/searcher/src/searcher/mod.rs
  L32: type Range = Match
  L55: pub struct BinaryDetection
  L62: pub fn none() -> BinaryDetection
  L79: pub fn quit(binary_byte: u8) -> BinaryDetection
  L95: pub fn convert(binary_byte: u8) -> BinaryDetection
  L102: pub fn quit_byte(&self) -> Option<u8>
  L112: pub fn convert_byte(&self) -> Option<u8>
  L127: pub struct Encoding
  L137: pub fn new(label: &str) -> Result<Encoding, ConfigError>
  L151: pub struct Config
  L188: fn default() -> Config
  L213: fn max_context(&self) -> usize
  L218: fn line_buffer(&self) -> LineBuffer
  L244: pub enum ConfigError
  L267: fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  L298: pub struct SearcherBuilder
  L303: fn default() -> SearcherBuilder
  L310: pub fn new() -> SearcherBuilder
  L315: pub fn build(&self) -> Searcher
  L346: pub fn line_terminator( &mut self, line_term: LineTerminator, ) -> &mut SearcherBuilder
  L358: pub fn invert_match(&mut self, yes: bool) -> &mut SearcherBuilder
  L368: pub fn line_number(&mut self, yes: bool) -> &mut SearcherBuilder
  L388: pub fn multi_line(&mut self, yes: bool) -> &mut SearcherBuilder
  L399: pub fn after_context( &mut self, line_count: usize, ) -> &mut SearcherBuilder
  L413: pub fn before_context( &mut self, line_count: usize, ) -> &mut SearcherBuilder
  L431: pub fn passthru(&mut self, yes: bool) -> &mut SearcherBuilder
  L457: pub fn heap_limit( &mut self, bytes: Option<usize>, ) -> &mut SearcherBuilder
  L495: pub fn memory_map( &mut self, strategy: MmapChoice, ) -> &mut SearcherBuilder
  L510: pub fn binary_detection( &mut self, detection: BinaryDetection, ) -> &mut SearcherBuilder
  L533: pub fn encoding( &mut self, encoding: Option<Encoding>, ) -> &mut SearcherBuilder
- crates/cli/src/decompress.rs
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
- crates/core/search.rs
- crates/globset/src/glob.rs
- crates/globset/src/lib.rs
- crates/ignore/examples/walk.rs
  L5: fn main()
  L52: enum DirEntry
  L58: fn path(&self) -> &Path
- repo map (224 files):
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
  crates/ (137 files)
  fuzz/ (5 files)
  pkg/ (3 files)
  rustfmt.toml
  scripts/ (1 files)
  tests/ (19 files)

## Code
- sym:26594 file_name (crates/ignore/src/pathutil.rs:142-171)
  pub(crate) fn file_name<'a, P: AsRef<Path> + ?Sized>( path: &'a P, ) -> Option<&'a OsStr>
- sym:26591 is_hidden_path_only (crates/ignore/src/pathutil.rs:85-91)
  fn is_hidden_path_only(path: &Path) -> bool
- sym:25204 Hidden.doc_long (crates/core/flags/defs.rs:2791-2808)
  fn doc_long(&self) -> &'static str
- sym:26472 IncrementalIgnore.match_path (crates/ignore/src/incremental.rs:393-411)
  fn match_path( &self, ignore: &Ignore, relative: &Path, is_dir: bool, ) -> IncrementalMatch
- sym:26589 is_hidden_path (crates/ignore/src/pathutil.rs:19-39)
  pub(crate) fn is_hidden_path(dent: &Path) -> bool
- sym:26593 is_file_name (crates/ignore/src/pathutil.rs:123-136)
  pub(crate) fn is_file_name<P: AsRef<Path>>(path: P) -> bool
- sym:26590 is_hidden_entry (crates/ignore/src/pathutil.rs:56-79)
  pub(crate) fn is_hidden_entry(dent: &DirEntry) -> bool
- sym:26351 Ignore.matched_dir_entry (crates/ignore/src/dir.rs:485-494)
  pub(crate) fn matched_dir_entry<'a>( &'a self, dent: &DirEntry, ) -> Match<IgnoreMatch<'a>>
- sym:26304 file_name_ext (crates/globset/src/pathutil.rs:40-54)
  pub(crate) fn file_name_ext<'a>( name: &Cow<'a, [u8]>, ) -> Option<Cow<'a, [u8]>>
- sym:26349 Ignore.add_child_path_with_found_ignore_files (crates/ignore/src/dir.rs:340-467)
  fn add_child_path_with_found_ignore_files( &self, dir: &Path, ignore_files_list: Option<&IgnoreFilesFound>, ) -> (IgnoreInner, Option<Error>)
- sym:26303 file_name (crates/globset/src/pathutil.rs:9-22)
  pub(crate) fn file_name<'a>(path: &Cow<'a, [u8]>) -> Option<Cow<'a, [u8]>>
- sym:26475 IncrementalMatch.none (crates/ignore/src/incremental.rs:456-462)
  fn none(is_dir: bool) -> IncrementalMatch
- sym:26632 DirEntry.file_name (crates/ignore/src/walk.rs:76-78)
  pub fn file_name(&self) -> &OsStr
- sym:26647 DirEntryInner.file_name (crates/ignore/src/walk.rs:200-207)
  fn file_name(&self) -> &OsStr
- sym:27528 InnerLiterals.none (crates/regex/src/literal.rs:96-98)
  pub(crate) fn none() -> InnerLiterals
- sym:27854 BinaryDetection.none (crates/searcher/src/searcher/mod.rs:62-64)
  pub fn none() -> BinaryDetection
- sym:26382 resolve_git_commondir (crates/ignore/src/dir.rs:1061-1105)
  fn resolve_git_commondir( dir: &Path, git_type: Option<FileType>, ) -> Result<PathBuf, Option<Error>>
- sym:26373 IgnoreBuilder.is_hidden (crates/ignore/src/dir.rs:925-927)
  pub(crate) fn is_hidden(&self) -> bool
- sym:25428 NoIgnoreDot.doc_long (crates/core/flags/defs.rs:4411-4419)
  fn doc_long(&self) -> &'static str
- sym:26471 IncrementalIgnore.matched_with_errors_ignore (crates/ignore/src/incremental.rs:310-391)
  fn matched_with_errors_ignore( &mut self, relative: &Path, is_dir: bool, errs: &mut PartialErrorBuilder, ) -> Option<IncrementalMatch>
- sym:25922 HiArgs.walk_builder (crates/core/flags/hiargs.rs:874-918)
  pub(crate) fn walk_builder(&self) -> anyhow::Result<ignore::WalkBuilder>
- sym:26337 IgnoreMatch.hidden (crates/ignore/src/dir.rs:64-66)
  fn hidden() -> IgnoreMatch<'static>
- sym:26372 IgnoreBuilder.hidden (crates/ignore/src/dir.rs:919-922)
  pub(crate) fn hidden(&mut self, yes: bool) -> &mut IgnoreBuilder
- sym:26612 Types.matched (crates/ignore/src/types.rs:263-302)
  pub fn matched<'a, P: AsRef<Path>>( &'a self, path: P, is_dir: bool, ) -> Match<Glob<'a>>
- sym:25425 NoIgnoreDot.name_negated (crates/core/flags/defs.rs:4402-4404)
  fn name_negated(&self) -> Option<&'static str>
- sym:26431 Gitignore.strip (crates/ignore/src/gitignore.rs:286-315)
  fn strip<'a, P: 'a + AsRef<Path> + ?Sized>( &'a self, path: &'a P, ) -> &'a Path
- sym:26338 IgnoreOptions (crates/ignore/src/dir.rs:72-90)
  struct IgnoreOptions
- sym:25203 Hidden.doc_short (crates/core/flags/defs.rs:2788-2790)
  fn doc_short(&self) -> &'static str
- sym:26348 Ignore.collect_ignore_files (crates/ignore/src/dir.rs:308-338)
  fn collect_ignore_files( &self, entries: &[fs::DirEntry], ) -> IgnoreFilesFound
- sym:25427 NoIgnoreDot.doc_short (crates/core/flags/defs.rs:4408-4410)
  fn doc_short(&self) -> &'static str
- sym:26333 IgnoreMatchInner (crates/ignore/src/dir.rs:44-49)
  enum IgnoreMatchInner<'a>
- sym:26470 IncrementalIgnore.matched_with_errors_impl (crates/ignore/src/incremental.rs:224-308)
  fn matched_with_errors_impl( &mut self, relative: &Path, is_dir: bool, errs: &mut PartialErrorBuilder, ) -> IncrementalMatch
- sym:26766 walkdir_is_dir (crates/ignore/src/walk.rs:2152-2160)
  fn walkdir_is_dir(dent: &walkdir::DirEntry) -> bool
- sym:26660 DirEntryRaw.file_name (crates/ignore/src/walk.rs:314-316)
  fn file_name(&self) -> &OsStr
- sym:25424 NoIgnoreDot.name_long (crates/core/flags/defs.rs:4399-4401)
  fn name_long(&self) -> &'static str
- sym:25429 NoIgnoreDot.update (crates/core/flags/defs.rs:4421-4424)
  fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()>
- sym:26596 GlobInner (crates/ignore/src/types.rs:114-122)
  enum GlobInner<'a>
- sym:25954 BinaryMode (crates/core/flags/lowargs.rs:233-253)
  pub(crate) enum BinaryMode
- sym:26524 hidden_files_and_directories (crates/ignore/src/incremental.rs:1194-1218)
  fn hidden_files_and_directories()
- sym:26526 descendants_of_hidden_directories_are_ignored (crates/ignore/src/incremental.rs:1236-1250)
  fn descendants_of_hidden_directories_are_ignored()
- sym:26525 gitignore_whitelist_overrides_hidden_filter (crates/ignore/src/incremental.rs:1221-1233)
  fn gitignore_whitelist_overrides_hidden_filter()
- sym:26527 whitelisted_hidden_directory_allows_descendants (crates/ignore/src/incremental.rs:1253-1265)
  fn whitelisted_hidden_directory_allows_descendants()
- sym:24768 DecompressionMatcherBuilder.new (crates/cli/src/decompress.rs:42-44)
  pub fn new() -> DecompressionMatcherBuilder
- sym:24775 DecompressionMatcher.new (crates/cli/src/decompress.rs:167-171)
  pub fn new() -> DecompressionMatcher
- sym:24779 DecompressionReaderBuilder.new (crates/cli/src/decompress.rs:205-207)
  pub fn new() -> DecompressionReaderBuilder
- sym:24785 DecompressionReader.new (crates/cli/src/decompress.rs:352-356)
  pub fn new<P: AsRef<Path>>( path: P, ) -> Result<DecompressionReader, CommandError>
- sym:24846 CommandError.is_empty (crates/cli/src/process.rs:33-38)
  pub(crate) fn is_empty(&self) -> bool
- sym:24851 CommandReaderBuilder.new (crates/cli/src/process.rs:90-92)
  pub fn new() -> CommandReaderBuilder
- sym:24855 CommandReader.new (crates/cli/src/process.rs:195-199)
  pub fn new( cmd: &mut process::Command, ) -> Result<CommandReader, CommandError>
- sym:25924 State.new (crates/core/flags/hiargs.rs:950-960)
  fn new() -> anyhow::Result<State>
- sym:25930 BinaryDetection (crates/core/flags/hiargs.rs:1152-1155)
  struct BinaryDetection
- sym:25931 BinaryDetection.from_low_args (crates/core/flags/hiargs.rs:1159-1175)
  fn from_low_args(_: &State, low: &LowArgs) -> BinaryDetection
- sym:25932 BinaryDetection.is_none (crates/core/flags/hiargs.rs:1179-1182)
  pub(crate) fn is_none(&self) -> bool
- sym:25970 ContextSeparator.new (crates/core/flags/lowargs.rs:483-491)
  pub(crate) fn new(os: &OsStr) -> anyhow::Result<ContextSeparator>
- sym:25977 FieldContextSeparator.new (crates/core/flags/lowargs.rs:561-569)
  pub(crate) fn new(os: &OsStr) -> anyhow::Result<FieldContextSeparator>
- sym:25981 FieldMatchSeparator.new (crates/core/flags/lowargs.rs:595-603)
  pub(crate) fn new(os: &OsStr) -> anyhow::Result<FieldMatchSeparator>
- sym:26017 Parser.new (crates/core/flags/parse.rs:176-216)
  fn new() -> &'static Parser
- sym:26026 FlagMap.new (crates/core/flags/parse.rs:374-387)
  fn new(infos: &[FlagInfo]) -> FlagMap
- sym:26034 HaystackBuilder.new (crates/core/haystack.rs:22-24)
  pub(crate) fn new() -> HaystackBuilder
- sym:26039 Haystack.path (crates/core/haystack.rs:108-114)
  pub(crate) fn path(&self) -> &Path
- sym:26042 Haystack.is_dir (crates/core/haystack.rs:143-154)
  fn is_dir(&self) -> bool
- sym:26072 Config.default (crates/core/search.rs:28-36)
  fn default() -> Config
- sym:26075 SearchWorkerBuilder.new (crates/core/search.rs:54-59)
  pub(crate) fn new() -> SearchWorkerBuilder
- sym:26126 MatchStrategy.new (crates/globset/src/glob.rs:51-67)
  pub(crate) fn new(pat: &Glob) -> MatchStrategy
- sym:26128 Glob.as_ref (crates/globset/src/glob.rs:84-86)
  fn as_ref(&self) -> &Glob
- sym:26150 Glob.new (crates/globset/src/glob.rs:283-285)
  pub fn new(glob: &str) -> Result<Glob, Error>
- sym:26162 GlobBuilder.new (crates/globset/src/glob.rs:574-576)
  pub fn new(glob: &'a str) -> GlobBuilder<'a>
- sym:26214 GlobSet.is_empty (crates/globset/src/lib.rs:331-333)
  pub fn is_empty(&self) -> bool
- sym:26215 GlobSet.len (crates/globset/src/lib.rs:337-339)
  pub fn len(&self) -> usize
- sym:26224 GlobSet.new (crates/globset/src/lib.rs:461-553)
  pub fn new<I, G>(globs: I) -> Result<GlobSet, Error> where I: IntoIterator<Item = G>, G: AsRef<Glob>,
- sym:26227 GlobSetBuilder.new (crates/globset/src/lib.rs:574-576)
  pub fn new() -> GlobSetBuilder
- sym:26232 Candidate.new (crates/globset/src/lib.rs:617-619)
  pub fn new<P: AsRef<Path> + ?Sized>(path: &'a P) -> Candidate<'a>
- sym:26233 Candidate.from_bytes (crates/globset/src/lib.rs:629-631)
  pub fn from_bytes<P: AsRef<[u8]> + ?Sized>(path: &'a P) -> Candidate<'a>
- sym:26234 Candidate.from_cow (crates/globset/src/lib.rs:633-638)
  fn from_cow(path: Cow<'a, [u8]>) -> Candidate<'a>
- sym:26242 LiteralStrategy.new (crates/globset/src/lib.rs:713-715)
  fn new() -> LiteralStrategy
- sym:26248 BasenameLiteralStrategy.new (crates/globset/src/lib.rs:745-747)
  fn new() -> BasenameLiteralStrategy
- sym:26254 ExtensionStrategy.new (crates/globset/src/lib.rs:783-785)
  fn new() -> ExtensionStrategy
- sym:26278 MultiStrategyBuilder.new (crates/globset/src/lib.rs:1023-1025)
  fn new() -> MultiStrategyBuilder
- sym:26283 MultiStrategyBuilder.is_empty (crates/globset/src/lib.rs:1063-1065)
  fn is_empty(&self) -> bool
- sym:26285 RequiredExtensionStrategyBuilder.new (crates/globset/src/lib.rs:1074-1076)
  fn new() -> RequiredExtensionStrategyBuilder
- sym:26328 DirEntry.path (crates/ignore/examples/walk.rs:58-63)
  fn path(&self) -> &Path
- sym:26498 leading_dot_slash_impacts_matching (crates/ignore/src/incremental.rs:662-675)
  fn leading_dot_slash_impacts_matching()
- sym:26516 dot_dash_root_is_not_stdin (crates/ignore/src/incremental.rs:1036-1040)
  fn dot_dash_root_is_not_stdin()
- sym:26528 hidden_directories_outside_depth_limits (crates/ignore/src/incremental.rs:1268-1285)
  fn hidden_directories_outside_depth_limits()
- sym:26803 IGNORE_FILE (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:5-6)
  const IGNORE_FILE: &'static str =
- sym:26808 test_dirs_in_root (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:89-190)
  fn test_dirs_in_root()
- sym:26809 test_dirs_in_deep (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:193-318)
  fn test_dirs_in_deep()
- sym:25206 test_hidden (crates/core/flags/defs.rs:2818-2833)
  fn test_hidden()
- sym:26806 test_files_in_root (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:25-54)
  fn test_files_in_root()
- sym:26807 test_files_in_deep (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:57-86)
  fn test_files_in_deep()
- sym:25430 test_no_ignore_dot (crates/core/flags/defs.rs:4429-4438)
  fn test_no_ignore_dot()
- sym:26405 stops_at_git_dir (crates/ignore/src/dir.rs:1365-1385)
  fn stops_at_git_dir()
- sym:26805 test_path_should_be_under_root (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:17-22)
  fn test_path_should_be_under_root()
- sym:26775 normal_path (crates/ignore/src/walk.rs:2224-2226)
  fn normal_path(unix: &str) -> String
- sym:26804 get_gitignore (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:8-13)
  fn get_gitignore() -> Gitignore
- sym:24841 os (crates/cli/src/pattern.rs:173-180)
  fn os()

---
tokens: 7083/8000
