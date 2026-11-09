## Task
# ripgrep-d99ac34 · ripgrep-d99ac34 · open
Goal: core: add `index` module
Workspace: /root/bench/av_gemma/ws/rec728b-ripgrep-d99ac34/ripgrep-d99ac34
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
- crates/core/flags/hiargs.rs
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
- crates/core/main.rs
- crates/cli/src/decompress.rs
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
- crates/core/flags/complete/bash.rs
- crates/core/flags/complete/fish.rs
- crates/core/flags/complete/powershell.rs
- crates/core/flags/complete/zsh.rs
  L21: pub(crate) fn generate() -> String
- crates/core/flags/config.rs
- repo map: .cargo/ .github/ .gitignore .ignore .nvim.lua AI_POLICY.md CHANGELOG.md CONTRIBUTING.md COPYING Cargo.lock Cargo.toml FAQ.md (232 files)

## Code
- sym:35514 Index (crates/core/flags/defs.rs:3455-3455)
  struct Index
- sym:35523 IndexCrud (crates/core/flags/defs.rs:3554-3554)
  struct IndexCrud
- sym:35517 Index.name_long (crates/core/flags/defs.rs:3464-3466)
  fn name_long(&self) -> &'static str
- sym:36229 IndexMode (crates/core/flags/lowargs.rs:199-202)
  pub(crate) enum IndexMode
- sym:35531 IndexForce (crates/core/flags/defs.rs:3651-3651)
  struct IndexForce
- sym:35527 IndexCrud.doc_short (crates/core/flags/defs.rs:3566-3568)
  fn doc_short(&self) -> &'static str
- sym:35518 Index.doc_category (crates/core/flags/defs.rs:3467-3469)
  fn doc_category(&self) -> Category
- sym:35521 Index.update (crates/core/flags/defs.rs:3511-3520)
  fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()>
- sym:36331 run (crates/core/main.rs:77-102)
  fn run(result: crate::flags::ParseResult<HiArgs>) -> anyhow::Result<ExitCode>
- sym:35529 IndexCrud.update (crates/core/flags/defs.rs:3619-3624)
  fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()>
- sym:36140 check_indexing_allowed (crates/core/flags/defs.rs:7950-7955)
  fn check_indexing_allowed() -> anyhow::Result<()>
- sym:35543 IndexPath.doc_category (crates/core/flags/defs.rs:3710-3712)
  fn doc_category(&self) -> Category
- sym:35539 IndexPath (crates/core/flags/defs.rs:3698-3698)
  struct IndexPath
- sym:35526 IndexCrud.doc_category (crates/core/flags/defs.rs:3563-3565)
  fn doc_category(&self) -> Category
- sym:36310 jaccard_index (crates/core/flags/parse.rs:454-460)
  fn jaccard_index(ngrams1: &BagOfWords<'_>, ngrams2: &BagOfWords<'_>) -> f64
- sym:35519 Index.doc_short (crates/core/flags/defs.rs:3470-3472)
  fn doc_short(&self) -> &'static str
- sym:37184 str.index (crates/matcher/src/lib.rs:178-180)
  fn index(&self, index: Match) -> &str
- sym:35534 IndexForce.doc_category (crates/core/flags/defs.rs:3660-3662)
  fn doc_category(&self) -> Category
- sym:35520 Index.doc_long (crates/core/flags/defs.rs:3473-3509)
  fn doc_long(&self) -> &'static str
- sym:35547 IndexPath.update (crates/core/flags/defs.rs:3726-3730)
  fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()>
- sym:35135 FLAGS (crates/core/flags/defs.rs:44-156)
  pub(super) const FLAGS: &[&dyn Flag] = &[
- sym:36176 HiArgs.from_low_args (crates/core/flags/hiargs.rs:113-331)
  pub(crate) fn from_low_args(mut low: LowArgs) -> anyhow::Result<HiArgs>
- sym:36226 Mode (crates/core/flags/lowargs.rs:156-169)
  pub(crate) enum Mode
- sym:35530 test_index_crud (crates/core/flags/defs.rs:3629-3647)
  fn test_index_crud()
- sym:35008 DecompressionMatcherBuilder.new (crates/cli/src/decompress.rs:42-44)
  pub fn new() -> DecompressionMatcherBuilder
- sym:35015 DecompressionMatcher.new (crates/cli/src/decompress.rs:167-171)
  pub fn new() -> DecompressionMatcher
- sym:35019 DecompressionReaderBuilder.new (crates/cli/src/decompress.rs:205-207)
  pub fn new() -> DecompressionReaderBuilder
- sym:35025 DecompressionReader.new (crates/cli/src/decompress.rs:352-356)
  pub fn new<P: AsRef<Path>>( path: P, ) -> Result<DecompressionReader, CommandError>
- sym:35057 Error.from (crates/cli/src/human.rs:66-68)
  fn from(size_err: ParseSizeError) -> std::io::Error
- sym:35074 Error.from (crates/cli/src/pattern.rs:40-42)
  fn from(paterr: InvalidPatternError) -> io::Error
- sym:35088 CommandError.from (crates/cli/src/process.rs:66-68)
  fn from(ioerr: io::Error) -> CommandError
- sym:35089 Error.from (crates/cli/src/process.rs:72-79)
  fn from(cmderr: CommandError) -> io::Error
- sym:35091 CommandReaderBuilder.new (crates/cli/src/process.rs:90-92)
  pub fn new() -> CommandReaderBuilder
- sym:35095 CommandReader.new (crates/cli/src/process.rs:195-199)
  pub fn new( cmd: &mut process::Command, ) -> Result<CommandReader, CommandError>
- sym:35120 generate (crates/core/flags/complete/bash.rs:63-107)
  pub(crate) fn generate() -> String
- sym:35123 generate (crates/core/flags/complete/fish.rs:13-75)
  pub(crate) fn generate() -> String
- sym:35127 generate (crates/core/flags/complete/powershell.rs:43-85)
  pub(crate) fn generate() -> String
- sym:35128 generate (crates/core/flags/complete/zsh.rs:21-32)
  pub(crate) fn generate() -> String
- sym:35129 args (crates/core/flags/config.rs:16-53)
  pub fn args() -> Vec<OsString>
- sym:35136 LowArgs.indexing_unsupported_flag (crates/core/flags/defs.rs:167-228)
  pub(super) fn indexing_unsupported_flag( &self, ) -> Option<&'static dyn Flag>
- sym:35137 AfterContext (crates/core/flags/defs.rs:233-233)
  struct AfterContext
- sym:35140 AfterContext.name_long (crates/core/flags/defs.rs:242-244)
  fn name_long(&self) -> &'static str
- sym:35147 AutoHybridRegex (crates/core/flags/defs.rs:319-319)
  struct AutoHybridRegex
- sym:35149 AutoHybridRegex.name_long (crates/core/flags/defs.rs:325-327)
  fn name_long(&self) -> &'static str
- sym:35156 BeforeContext (crates/core/flags/defs.rs:418-418)
  struct BeforeContext
- sym:35159 BeforeContext.name_long (crates/core/flags/defs.rs:427-429)
  fn name_long(&self) -> &'static str
- sym:35166 Binary (crates/core/flags/defs.rs:504-504)
  struct Binary
- sym:35168 Binary.name_long (crates/core/flags/defs.rs:510-512)
  fn name_long(&self) -> &'static str
- sym:35175 BlockBuffered (crates/core/flags/defs.rs:595-595)
  struct BlockBuffered
- sym:35177 BlockBuffered.name_long (crates/core/flags/defs.rs:601-603)
  fn name_long(&self) -> &'static str
- sym:35184 ByteOffset (crates/core/flags/defs.rs:655-655)
  struct ByteOffset
- sym:35187 ByteOffset.name_long (crates/core/flags/defs.rs:664-666)
  fn name_long(&self) -> &'static str
- sym:35194 CaseSensitive (crates/core/flags/defs.rs:715-715)
  struct CaseSensitive
- sym:35197 CaseSensitive.name_long (crates/core/flags/defs.rs:724-726)
  fn name_long(&self) -> &'static str
- sym:35203 Color (crates/core/flags/defs.rs:768-768)
  struct Color
- sym:35205 Color.name_long (crates/core/flags/defs.rs:774-776)
  fn name_long(&self) -> &'static str
- sym:35213 Colors (crates/core/flags/defs.rs:891-891)
  struct Colors
- sym:35215 Colors.name_long (crates/core/flags/defs.rs:897-899)
  fn name_long(&self) -> &'static str
- sym:35522 test_index (crates/core/flags/defs.rs:3525-3550)
  fn test_index()
- sym:35538 test_index_force (crates/core/flags/defs.rs:3683-3694)
  fn test_index_force()
- sym:35548 test_index_path (crates/core/flags/defs.rs:3735-3749)
  fn test_index_path()

---
tokens: 3622/4000
