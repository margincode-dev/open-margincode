## Task
# ripgrep-7155f64 · ripgrep-7155f64-2 · open
Goal: ignore: treat dotfiles ending in a dot as hidden (#3564)

`file_name()` returned `None` for any path ending in `.` on Unix,
which was meant to cover the `.` and `..` components but also matched
real names like `.foo.`. The hidden check then found no file name and
treated those files as visible. Only a final component that is exactly
`.` or `..` has no file name now.

PR #3564
Workspace: /root/bench/ws/rebf815-ripgrep-7155f64/ripgrep-7155f64-2
Changed (0): nothing yet

## Map
- crates/globset/src/pathutil.rs
  L9: pub(crate) fn file_name<'a>(path: &Cow<'a, [u8]>) -> Option<Cow<'a, [u8]>>
  L40: pub(crate) fn file_name_ext<'a>( name: &Cow<'a, [u8]>, ) -> Option<Cow<'a, [u8]>>
  L59: pub(crate) fn normalize_path(path: Cow<'_, [u8]>) -> Cow<'_, [u8]>
- crates/ignore/src/incremental.rs
- crates/ignore/src/pathutil.rs
  L19: pub(crate) fn is_hidden_path(dent: &Path) -> bool
  L56: pub(crate) fn is_hidden_entry(dent: &DirEntry) -> bool
  L85: fn is_hidden_path_only(path: &Path) -> bool
  L96: pub(crate) fn strip_prefix<'a, P: AsRef<Path> + ?Sized>( prefix: &'a P, path: &'a Path, ) -> Option<&'a Path>
  L123: pub(crate) fn is_file_name<P: AsRef<Path>>(path: P) -> bool
  L142: pub(crate) fn file_name<'a, P: AsRef<Path> + ?Sized>( path: &'a P, ) -> Option<&'a OsStr>
- crates/ignore/src/walk.rs
- crates/regex/src/literal.rs
- crates/searcher/src/searcher/mod.rs
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
- repo map: .cargo/ .github/ .gitignore .ignore .nvim.lua AI_POLICY.md CHANGELOG.md CONTRIBUTING.md COPYING Cargo.lock Cargo.toml FAQ.md (224 files)

## Code
- sym:26594 file_name (crates/ignore/src/pathutil.rs:142-171)
  pub(crate) fn file_name<'a, P: AsRef<Path> + ?Sized>( path: &'a P, ) -> Option<&'a OsStr>
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
- sym:26591 is_hidden_path_only (crates/ignore/src/pathutil.rs:85-91)
  fn is_hidden_path_only(path: &Path) -> bool
- sym:26593 is_file_name (crates/ignore/src/pathutil.rs:123-136)
  pub(crate) fn is_file_name<P: AsRef<Path>>(path: P) -> bool
- sym:26472 IncrementalIgnore.match_path (crates/ignore/src/incremental.rs:393-411)
  fn match_path( &self, ignore: &Ignore, relative: &Path, is_dir: bool, ) -> IncrementalMatch
- sym:25204 Hidden.doc_long (crates/core/flags/defs.rs:2791-2808)
  fn doc_long(&self) -> &'static str
- sym:26471 IncrementalIgnore.matched_with_errors_ignore (crates/ignore/src/incremental.rs:310-391)
  fn matched_with_errors_ignore( &mut self, relative: &Path, is_dir: bool, errs: &mut PartialErrorBuilder, ) -> Option<IncrementalMatch>
- sym:25922 HiArgs.walk_builder (crates/core/flags/hiargs.rs:874-918)
  pub(crate) fn walk_builder(&self) -> anyhow::Result<ignore::WalkBuilder>
- sym:26351 Ignore.matched_dir_entry (crates/ignore/src/dir.rs:485-494)
  pub(crate) fn matched_dir_entry<'a>( &'a self, dent: &DirEntry, ) -> Match<IgnoreMatch<'a>>
- sym:26589 is_hidden_path (crates/ignore/src/pathutil.rs:19-39)
  pub(crate) fn is_hidden_path(dent: &Path) -> bool
- sym:26612 Types.matched (crates/ignore/src/types.rs:263-302)
  pub fn matched<'a, P: AsRef<Path>>( &'a self, path: P, is_dir: bool, ) -> Match<Glob<'a>>
- sym:26431 Gitignore.strip (crates/ignore/src/gitignore.rs:286-315)
  fn strip<'a, P: 'a + AsRef<Path> + ?Sized>( &'a self, path: &'a P, ) -> &'a Path
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
- sym:26526 descendants_of_hidden_directories_are_ignored (crates/ignore/src/incremental.rs:1236-1250)
  fn descendants_of_hidden_directories_are_ignored()
- sym:26524 hidden_files_and_directories (crates/ignore/src/incremental.rs:1194-1218)
  fn hidden_files_and_directories()
- sym:26525 gitignore_whitelist_overrides_hidden_filter (crates/ignore/src/incremental.rs:1221-1233)
  fn gitignore_whitelist_overrides_hidden_filter()
- sym:26527 whitelisted_hidden_directory_allows_descendants (crates/ignore/src/incremental.rs:1253-1265)
  fn whitelisted_hidden_directory_allows_descendants()
- sym:24841 os (crates/cli/src/pattern.rs:173-180)
  fn os()

---
tokens: 3521/4000
