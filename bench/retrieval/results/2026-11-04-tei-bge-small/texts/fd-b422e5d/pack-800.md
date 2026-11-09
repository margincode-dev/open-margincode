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
- sym:39918 hir_matches_strings_with_leading_dot (src/regex_helper.rs:50-79)
  fn hir_matches_strings_with_leading_dot(hir: &Hir) -> bool
- sym:39922 needs_escape (src/sanitize.rs:9-24)
  fn needs_escape(c: char) -> bool
- sym:39727 Exec.augment_args (src/cli.rs:883-952)
  fn augment_args(cmd: Command) -> Command
- sym:39905 ensure_use_hidden_option_for_leading_dot_pattern (src/main.rs:521-539)
  fn ensure_use_hidden_option_for_leading_dot_pattern( config: &Config, pattern_regexps: &[String], ) -> Result<()>
- sym:39725 Exec.from_arg_matches (src/cli.rs:862-874)
  fn from_arg_matches(matches: &ArgMatches) -> clap::error::Result<Self>
- sym:39875 FormatTemplate.parse (src/fmt/mod.rs:58-107)
  pub fn parse(fmt: &str) -> Self
- sym:39917 pattern_matches_strings_with_leading_dot (src/regex_helper.rs:40-47)
```rust
pub fn pattern_matches_strings_with_leading_dot(pattern: &str) -> bool {
    let mut parser = ParserBuilder::new().utf8(false).build();

    parser
        .parse(pattern)
        .map(|hir| hir_matches_strings_with_leading_dot(&hir))
        .unwrap_or(false)
}
```
- sym:39720 FileType (src/cli.rs:803-823)
  pub enum FileType
- sym:39708 Opts (src/cli.rs:32-693)
- sym:39921 matches_strings_with_leading_dot_simple (src/regex_helper.rs:99-105)
- sym:39830 OwnerFilter.matches (src/filter/owner.rs:69-73)
- sym:39923 sanitize_for_terminal (src/sanitize.rs:28-46)
- sym:39789 tokens_with_literal_braces_and_placeholder (src/exec/mod.rs:379-383)
- sym:40082 test_fixed_strings (tests/tests.rs:2195-2227)
- sym:39788 tokens_with_literal_braces (src/exec/mod.rs:369-376)

---
tokens: 684/800
