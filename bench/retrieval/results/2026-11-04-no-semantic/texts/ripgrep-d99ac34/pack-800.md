## Task
# ripgrep-d99ac34 · ripgrep-d99ac34-2 · open
Goal: core: add `index` module

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
- crates/core/flags/complete/fish.rs
- crates/core/flags/complete/powershell.rs

## Code
- sym:35514 Index (crates/core/flags/defs.rs:3455-3455)
```rust
struct Index;
```
- sym:35517 Index.name_long (crates/core/flags/defs.rs:3464-3466)
  fn name_long(&self) -> &'static str
- sym:35523 IndexCrud (crates/core/flags/defs.rs:3554-3554)
  struct IndexCrud
- sym:35531 IndexForce (crates/core/flags/defs.rs:3651-3651)
  struct IndexForce
- sym:35135 FLAGS (crates/core/flags/defs.rs:44-156)
  pub(super) const FLAGS: &[&dyn Flag] = &[
- sym:35529 IndexCrud.update (crates/core/flags/defs.rs:3619-3624)
  fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()>
- sym:36176 HiArgs.from_low_args (crates/core/flags/hiargs.rs:113-331)
- sym:36226 Mode (crates/core/flags/lowargs.rs:156-169)
- sym:36331 run (crates/core/main.rs:77-102)
- sym:35530 test_index_crud (crates/core/flags/defs.rs:3629-3647)
- sym:35008 DecompressionMatcherBuilder.new (crates/cli/src/decompress.rs:42-44)
- sym:35015 DecompressionMatcher.new (crates/cli/src/decompress.rs:167-171)
- sym:35019 DecompressionReaderBuilder.new (crates/cli/src/decompress.rs:205-207)
- sym:35025 DecompressionReader.new (crates/cli/src/decompress.rs:352-356)

---
tokens: 708/800
omitted 36 lower-ranked items (expand an id for detail):
- crates/core/flags/complete/zsh.rs
- crates/core/flags/config.rs
- repo map: .cargo/ .github/ .gitignore .ignore .nvim.lua AI_POLICY.md CHANGELOG.md CONTRIBUTING.md COPYING Cargo.lock Cargo.toml FAQ.md (232 files)
- sym:35057 Error.from (crates/cli/src/human.rs:66-68)
- sym:35074 Error.from (crates/cli/src/pattern.rs:40-42)
- sym:35088 CommandError.from (crates/cli/src/process.rs:66-68)
- sym:35089 Error.from (crates/cli/src/process.rs:72-79)
- sym:35091 CommandReaderBuilder.new (crates/cli/src/process.rs:90-92)
- … and 28 more
