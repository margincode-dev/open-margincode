## Task
# fd-b422e5d · fd-b422e5d · open
Goal: fix: allow hyphen-leading --format templates

Clap treated a space-separated --format value that starts with '-' as a
new flag. Enable allow_hyphen_values for --format (same as --and) and
add a regression covering markdown-list-style templates. See #2126.
Workspace: /root/bench/av_tei/ws/rec09fc-fd-b422e5d/fd-b422e5d
Changed (0): nothing yet

## Map
- repo map (60 files):
  .cargo/ (1 files)
    config.toml
  .github/ (7 files)
    FUNDING.yml
    ISSUE_TEMPLATE/ (4 files)
    dependabot.yml
    workflows/ (1 files)
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
    completion/ (4 files)
  doc/ (8 files)
    .gitattributes
    fd.1
    logo.png
    logo.svg
    release-checklist.md
    screencast.sh
    screencast.svg
    sponsors.md
  rustfmt.toml
  scripts/ (4 files)
    create-archive.sh
    create-deb.sh
    update-help.awk
    version-bump.sh
  src/ (22 files)
    cli.rs
    config.rs
    dir_entry.rs
    error.rs
    exec/ (3 files)
    exit_codes.rs
    filesystem.rs
    filetypes.rs
    filter/ (4 files)
    fmt/ (2 files)
    hyperlink.rs
    main.rs
    output.rs
    regex_helper.rs
    sanitize.rs
    walk.rs
  tests/ (2 files)
    testenv/ (1 files)
    tests.rs

## Code
- sym:39918 hir_matches_strings_with_leading_dot (src/regex_helper.rs:50-79)
  fn hir_matches_strings_with_leading_dot(hir: &Hir) -> bool
- sym:39922 needs_escape (src/sanitize.rs:9-24)
  fn needs_escape(c: char) -> bool
- sym:39875 FormatTemplate.parse (src/fmt/mod.rs:58-107)
  pub fn parse(fmt: &str) -> Self
- sym:39917 pattern_matches_strings_with_leading_dot (src/regex_helper.rs:40-47)
  pub fn pattern_matches_strings_with_leading_dot(pattern: &str) -> bool
- sym:39726 Exec.update_from_arg_matches (src/cli.rs:876-879)
  fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> clap::error::Result<()>
- sym:39727 Exec.augment_args (src/cli.rs:883-952)
  fn augment_args(cmd: Command) -> Command
- sym:39905 ensure_use_hidden_option_for_leading_dot_pattern (src/main.rs:521-539)
  fn ensure_use_hidden_option_for_leading_dot_pattern( config: &Config, pattern_regexps: &[String], ) -> Result<()>
- sym:39725 Exec.from_arg_matches (src/cli.rs:862-874)
  fn from_arg_matches(matches: &ArgMatches) -> clap::error::Result<Self>
- sym:39720 FileType (src/cli.rs:803-823)
  pub enum FileType
- sym:39708 Opts (src/cli.rs:32-693)
  pub struct Opts
- sym:39761 execute_commands (src/exec/command.rs:60-99)
  pub fn execute_commands<I: Iterator<Item = io::Result<Command>>>( cmds: I, mut output_buffer: OutputBuffer, enable_output_buffering: bool, ) -> ExitCode
- sym:39876 FormatTemplate.generate (src/fmt/mod.rs:112-141)
  pub fn generate(&self, path: impl AsRef<Path>, path_separator: Option<&str>) -> OsString
- sym:39779 CommandTemplate.new (src/exec/mod.rs:220-256)
  fn new<I, S>(input: I, mode: ExecutionMode) -> Result<CommandTemplate> where I: IntoIterator<Item = S>, S: AsRef<str>,
- sym:39872 FormatTemplate (src/fmt/mod.rs:46-49)
  pub enum FormatTemplate
- sym:39874 FormatTemplate.has_tokens (src/fmt/mod.rs:54-56)
  pub fn has_tokens(&self) -> bool
- sym:39921 matches_strings_with_leading_dot_simple (src/regex_helper.rs:99-105)
  fn matches_strings_with_leading_dot_simple()
- sym:39934 strips_bidi_overrides_and_zero_width (src/sanitize.rs:131-140)
  fn strips_bidi_overrides_and_zero_width()
- sym:39830 OwnerFilter.matches (src/filter/owner.rs:69-73)
  pub fn matches(&self, md: &fs::Metadata) -> bool
- sym:39923 sanitize_for_terminal (src/sanitize.rs:28-46)
  pub fn sanitize_for_terminal(s: &str) -> Cow<'_, str>
- sym:39789 tokens_with_literal_braces_and_placeholder (src/exec/mod.rs:379-383)
  fn tokens_with_literal_braces_and_placeholder()
- sym:40082 test_fixed_strings (tests/tests.rs:2195-2227)
  fn test_fixed_strings()
- sym:39788 tokens_with_literal_braces (src/exec/mod.rs:369-376)
  fn tokens_with_literal_braces()
- sym:39881 all_placeholders (src/fmt/mod.rs:237-280)
  fn all_placeholders()
- sym:39880 parse_only_brace_escapes (src/fmt/mod.rs:228-234)
  fn parse_only_brace_escapes()
- sym:40083 test_exact_literal_nonsubstring (tests/tests.rs:2231-2264)
  fn test_exact_literal_nonsubstring()
- sym:39787 tokens_with_basename_no_extension (src/exec/mod.rs:353-366)
  fn tokens_with_basename_no_extension()
- sym:39862 is_time_filter_applicable (src/filter/time.rs:96-201)
  fn is_time_filter_applicable()

---
tokens: 1364/4000
