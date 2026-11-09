## Task
# ripgrep-d99ac34 · ripgrep-d99ac34 · open
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
- crates/core/flags/config.rs

## Code
- sym:35514 Index (crates/core/flags/defs.rs:3455-3455)
  struct Index
- sym:35523 IndexCrud (crates/core/flags/defs.rs:3554-3554)
  struct IndexCrud
- sym:35528 IndexCrud.doc_long (crates/core/flags/defs.rs:3569-3617)
  fn doc_long(&self) -> &'static str
- sym:36229 IndexMode (crates/core/flags/lowargs.rs:199-202)
  pub(crate) enum IndexMode
- sym:35517 Index.name_long (crates/core/flags/defs.rs:3464-3466)
  fn name_long(&self) -> &'static str
- sym:36140 check_indexing_allowed (crates/core/flags/defs.rs:7950-7955)
  fn check_indexing_allowed() -> anyhow::Result<()>
- sym:35520 Index.doc_long (crates/core/flags/defs.rs:3473-3509)
- sym:35527 IndexCrud.doc_short (crates/core/flags/defs.rs:3566-3568)
- sym:35531 IndexForce (crates/core/flags/defs.rs:3651-3651)
- sym:35530 test_index_crud (crates/core/flags/defs.rs:3629-3647)
- sym:35135 FLAGS (crates/core/flags/defs.rs:44-156)
- sym:35529 IndexCrud.update (crates/core/flags/defs.rs:3619-3624)
- sym:36176 HiArgs.from_low_args (crates/core/flags/hiargs.rs:113-331)
- sym:36331 run (crates/core/main.rs:77-102)

---
tokens: 712/800
omitted 40 lower-ranked items (expand an id for detail):
- crates/core/flags/complete/zsh.rs
- repo map: .cargo/ .github/ .gitignore .ignore .nvim.lua AI_POLICY.md CHANGELOG.md CONTRIBUTING.md COPYING Cargo.lock Cargo.toml FAQ.md (232 files)
- sym:36226 Mode (crates/core/flags/lowargs.rs:156-169)
- sym:35008 DecompressionMatcherBuilder.new (crates/cli/src/decompress.rs:42-44)
- sym:35015 DecompressionMatcher.new (crates/cli/src/decompress.rs:167-171)
- sym:35019 DecompressionReaderBuilder.new (crates/cli/src/decompress.rs:205-207)
- sym:35025 DecompressionReader.new (crates/cli/src/decompress.rs:352-356)
- … and 33 more
