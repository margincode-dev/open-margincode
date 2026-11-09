## Task
# ripgrep-c5e744c · ripgrep-c5e744c-2 · open
Goal: ignore: avoid scratch pools for empty matchers

... by deferring to `Gitignore::empty()` when there aren't any globs.

This plausibly skips some other work as well, in addition to avoiding
allocation for scratch pools.

PR #3555
Workspace: /root/bench/ws/rebf815-ripgrep-c5e744c/ripgrep-c5e744c-2
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
- sym:23042 IgnoreBuilder.build_with_cwd (crates/ignore/src/dir.rs:817-862)
  pub(crate) fn build_with_cwd(&self, cwd: Option<PathBuf>) -> Ignore
- sym:23111 GitignoreBuilder.build_global (crates/ignore/src/gitignore.rs:377-396)
  pub fn build_global(mut self) -> (Gitignore, Option<Error>)
- sym:22565 globs (crates/core/flags/hiargs.rs:1210-1231)
  fn globs( state: &State, low: &LowArgs, ) -> anyhow::Result<ignore::overrides::Override>
- sym:23097 Gitignore.global (crates/ignore/src/gitignore.rs:141-146)
  pub fn global() -> (Gitignore, Option<Error>)
- sym:23110 GitignoreBuilder.build (crates/ignore/src/gitignore.rs:349-366)
  pub fn build(&self) -> Result<Gitignore, Error>
- sym:23000 incremental_gitignore_only (crates/ignore/benches/workloads.rs:563-576)
  fn incremental_gitignore_only(b: &mut test::Bencher)
- sym:23039 IgnoreBuilder (crates/ignore/src/dir.rs:753-776)
  pub(crate) struct IgnoreBuilder
- sym:23106 Gitignore.matched_stripped (crates/ignore/src/gitignore.rs:259-282)
  fn matched_stripped<P: AsRef<Path>>( &self, path: P, is_dir: bool, ) -> Match<&Glob>
- sym:23445 path_equals (crates/ignore/src/walk.rs:2123-2142)
  fn path_equals(dent: &DirEntry, handle: &Handle) -> Result<bool, Error>
- sym:23016 IgnoreInner (crates/ignore/src/dir.rs:115-166)
  struct IgnoreInner
- sym:22723 SearchWorker.should_preprocess (crates/core/search.rs:284-292)
  fn should_preprocess(&self, path: &Path) -> bool
- sym:23416 Work (crates/ignore/src/walk.rs:1560-1568)
  struct Work
- sym:21864 IGlob.doc_long (crates/core/flags/defs.rs:3109-3118)
  fn doc_long(&self) -> &'static str
- sym:21448 Error.from (crates/cli/src/human.rs:66-68)
  fn from(size_err: ParseSizeError) -> std::io::Error
- sym:21465 Error.from (crates/cli/src/pattern.rs:40-42)
  fn from(paterr: InvalidPatternError) -> io::Error
- sym:21479 CommandError.from (crates/cli/src/process.rs:66-68)
  fn from(ioerr: io::Error) -> CommandError
- sym:21480 Error.from (crates/cli/src/process.rs:72-79)
  fn from(cmderr: CommandError) -> io::Error
- sym:22566 preprocessor_globs (crates/core/flags/hiargs.rs:1234-1246)
  fn preprocessor_globs( state: &State, low: &LowArgs, ) -> anyhow::Result<ignore::overrides::Override>
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
- sym:23025 Ignore.add_child_path_with_found_ignore_files (crates/ignore/src/dir.rs:340-467)
  fn add_child_path_with_found_ignore_files( &self, dir: &Path, ignore_files_list: Option<&IgnoreFilesFound>, ) -> (IgnoreInner, Option<Error>)
- sym:23040 IgnoreBuilder.new (crates/ignore/src/dir.rs:784-803)
  pub(crate) fn new() -> IgnoreBuilder
- sym:23089 Glob.from (crates/ignore/src/gitignore.rs:47-49)
  pub fn from(&self) -> Option<&Path>
- sym:23095 Gitignore (crates/ignore/src/gitignore.rs:81-88)
  pub struct Gitignore
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
- sym:23187 compiled_multi_child_matchers_are_not_reused (crates/ignore/src/incremental.rs:901-936)
  fn compiled_multi_child_matchers_are_not_reused()
- sym:23182 compiled_matchers_are_reused (crates/ignore/src/incremental.rs:811-830)
  fn compiled_matchers_are_reused()
- sym:23778 build_empty_format (crates/printer/src/hyperlink/mod.rs:973-979)
  fn build_empty_format()

---
tokens: 3571/4000
