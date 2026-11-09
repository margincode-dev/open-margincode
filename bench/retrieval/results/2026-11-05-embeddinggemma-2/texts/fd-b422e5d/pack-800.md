## Task
# fd-b422e5d · fd-b422e5d · open
Goal: fix: allow hyphen-leading --format templates

Clap treated a space-separated --format value that starts with '-' as a
new flag. Enable allow_hyphen_values for --format (same as --and) and
add a regression covering markdown-list-style templates. See #2126.

## Map
- repo map (60 files):
  .cargo/ (1 files)
  .github/ (7 files)
  .gitignore
  CHANGELOG.md
  CONTRIBUTING.md
  Cargo.lock
  Cargo.toml
  Cross.toml
  LICENSE-APACHE
  LICENSE-MIT
  Makefile
  README.md
  SECURITY.md
  contrib/ (4 files)
  doc/ (8 files)
  rustfmt.toml
  scripts/ (4 files)
  src/ (22 files)
  tests/ (2 files)

## Code
- sym:39727 Exec.augment_args (src/cli.rs:883-952)
  fn augment_args(cmd: Command) -> Command
- sym:39778 CommandTemplate (src/exec/mod.rs:215-217)
  struct CommandTemplate
- sym:39725 Exec.from_arg_matches (src/cli.rs:862-874)
  fn from_arg_matches(matches: &ArgMatches) -> clap::error::Result<Self>
- sym:39872 FormatTemplate (src/fmt/mod.rs:46-49)
  pub enum FormatTemplate
- sym:39918 hir_matches_strings_with_leading_dot (src/regex_helper.rs:50-79)
  fn hir_matches_strings_with_leading_dot(hir: &Hir) -> bool
- sym:39779 CommandTemplate.new (src/exec/mod.rs:220-256)
- sym:39922 needs_escape (src/sanitize.rs:9-24)
  fn needs_escape(c: char) -> bool
- sym:39720 FileType (src/cli.rs:803-823)
  pub enum FileType
- sym:39874 FormatTemplate.has_tokens (src/fmt/mod.rs:54-56)
  pub fn has_tokens(&self) -> bool
- sym:39875 FormatTemplate.parse (src/fmt/mod.rs:58-107)
- sym:39708 Opts (src/cli.rs:32-693)
  pub struct Opts
- sym:39871 Token.fmt (src/fmt/mod.rs:28-38)
- sym:39724 Exec (src/cli.rs:857-859)
  pub struct Exec
- sym:39726 Exec.update_from_arg_matches (src/cli.rs:876-879)
- sym:39728 Exec.augment_args_for_update (src/cli.rs:954-956)
- sym:39758 OutputBuffer.new (src/exec/command.rs:19-24)
- sym:39880 parse_only_brace_escapes (src/fmt/mod.rs:228-234)
- sym:40082 test_fixed_strings (tests/tests.rs:2195-2227)
- sym:39921 matches_strings_with_leading_dot_simple (src/regex_helper.rs:99-105)
- sym:40073 format (tests/tests.rs:1701-1758)
  fn format()

---
tokens: 689/800
