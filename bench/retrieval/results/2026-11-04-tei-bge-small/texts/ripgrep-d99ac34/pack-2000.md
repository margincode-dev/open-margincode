## Task
# ripgrep-d99ac34 · ripgrep-d99ac34 · open
Goal: core: add `index` module
Workspace: /root/bench/av_tei/ws/rec09fc-ripgrep-d99ac34/ripgrep-d99ac34
Changed (0): nothing yet

## Map
- crates/core/flags/defs.rs
- crates/core/flags/hiargs.rs
- crates/core/flags/lowargs.rs
- crates/core/main.rs
- crates/cli/src/decompress.rs
- crates/cli/src/human.rs
- crates/cli/src/pattern.rs
- crates/cli/src/process.rs
- crates/core/flags/complete/bash.rs
  L7: const TEMPLATE_FULL: &'static str = "
  L45: const TEMPLATE_CASE: &'static str = "
  L52: const TEMPLATE_CASE_CHOICES: &'static str = "
  L63: pub(crate) fn generate() -> String
- crates/core/flags/complete/fish.rs
  L7: const TEMPLATE: &'static str = "complete -c rg !SHORT! -l !LONG! -d '!DOC!'"
  L8: const TEMPLATE_NEGATED: &'static str = "complete -c rg -l !NEGATED! -n '__rg_contains_opt !LONG! !SHORT!' -d '!DOC!'\n"
  L13: pub(crate) fn generate() -> String
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
- sym:35531 IndexForce (crates/core/flags/defs.rs:3651-3651)
  struct IndexForce
- sym:36534 ExtensionStrategy.add (crates/globset/src/lib.rs:787-789)
  fn add(&mut self, global_index: usize, ext: String)
- sym:36565 RequiredExtensionStrategyBuilder.add (crates/globset/src/lib.rs:1078-1083)
  fn add(&mut self, global_index: usize, ext: String, regex: String)
- sym:35517 Index.name_long (crates/core/flags/defs.rs:3464-3466)
  fn name_long(&self) -> &'static str
- sym:35539 IndexPath (crates/core/flags/defs.rs:3698-3698)
  struct IndexPath
- sym:37181 [u8].index (crates/matcher/src/lib.rs:162-164)
  fn index(&self, index: Match) -> &[u8]
- sym:36229 IndexMode (crates/core/flags/lowargs.rs:199-202)
  pub(crate) enum IndexMode
- sym:35135 FLAGS (crates/core/flags/defs.rs:44-156)
  pub(super) const FLAGS: &[&dyn Flag] = &[
- sym:35529 IndexCrud.update (crates/core/flags/defs.rs:3619-3624)
  fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()>
- sym:36176 HiArgs.from_low_args (crates/core/flags/hiargs.rs:113-331)
  pub(crate) fn from_low_args(mut low: LowArgs) -> anyhow::Result<HiArgs>
- sym:36226 Mode (crates/core/flags/lowargs.rs:156-169)
  pub(crate) enum Mode
- sym:36331 run (crates/core/main.rs:77-102)
- sym:35530 test_index_crud (crates/core/flags/defs.rs:3629-3647)
- sym:35008 DecompressionMatcherBuilder.new (crates/cli/src/decompress.rs:42-44)
- sym:35015 DecompressionMatcher.new (crates/cli/src/decompress.rs:167-171)
- sym:35019 DecompressionReaderBuilder.new (crates/cli/src/decompress.rs:205-207)
- sym:35025 DecompressionReader.new (crates/cli/src/decompress.rs:352-356)
- sym:35057 Error.from (crates/cli/src/human.rs:66-68)
- sym:35074 Error.from (crates/cli/src/pattern.rs:40-42)
- sym:35088 CommandError.from (crates/cli/src/process.rs:66-68)
- sym:35089 Error.from (crates/cli/src/process.rs:72-79)
- sym:35091 CommandReaderBuilder.new (crates/cli/src/process.rs:90-92)
- sym:35095 CommandReader.new (crates/cli/src/process.rs:195-199)
- sym:35120 generate (crates/core/flags/complete/bash.rs:63-107)
- sym:35123 generate (crates/core/flags/complete/fish.rs:13-75)
- sym:35127 generate (crates/core/flags/complete/powershell.rs:43-85)
- sym:35128 generate (crates/core/flags/complete/zsh.rs:21-32)
- sym:35129 args (crates/core/flags/config.rs:16-53)
- sym:35136 LowArgs.indexing_unsupported_flag (crates/core/flags/defs.rs:167-228)
- sym:35137 AfterContext (crates/core/flags/defs.rs:233-233)
- sym:35140 AfterContext.name_long (crates/core/flags/defs.rs:242-244)
- sym:35147 AutoHybridRegex (crates/core/flags/defs.rs:319-319)
- sym:35149 AutoHybridRegex.name_long (crates/core/flags/defs.rs:325-327)
- sym:35156 BeforeContext (crates/core/flags/defs.rs:418-418)
- sym:35159 BeforeContext.name_long (crates/core/flags/defs.rs:427-429)
- sym:35166 Binary (crates/core/flags/defs.rs:504-504)
- sym:35168 Binary.name_long (crates/core/flags/defs.rs:510-512)
- sym:35175 BlockBuffered (crates/core/flags/defs.rs:595-595)
- sym:35177 BlockBuffered.name_long (crates/core/flags/defs.rs:601-603)
- sym:35184 ByteOffset (crates/core/flags/defs.rs:655-655)
- sym:35187 ByteOffset.name_long (crates/core/flags/defs.rs:664-666)
- sym:35194 CaseSensitive (crates/core/flags/defs.rs:715-715)
- sym:35203 Color (crates/core/flags/defs.rs:768-768)

---
tokens: 1715/2000
omitted 7 lower-ranked items (expand an id for detail):
- sym:35197 CaseSensitive.name_long (crates/core/flags/defs.rs:724-726)
- sym:35205 Color.name_long (crates/core/flags/defs.rs:774-776)
- sym:35213 Colors (crates/core/flags/defs.rs:891-891)
- sym:35215 Colors.name_long (crates/core/flags/defs.rs:897-899)
- sym:35522 test_index (crates/core/flags/defs.rs:3525-3550)
- sym:35538 test_index_force (crates/core/flags/defs.rs:3683-3694)
- sym:35548 test_index_path (crates/core/flags/defs.rs:3735-3749)
