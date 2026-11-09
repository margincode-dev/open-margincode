## Task
# fd-b422e5d · fd-b422e5d · open
Goal: fix: allow hyphen-leading --format templates

Clap treated a space-separated --format value that starts with '-' as a
new flag. Enable allow_hyphen_values for --format (same as --and) and
add a regression covering markdown-list-style templates. See #2126.
Workspace: /root/bench/ws/rebf815-fd-b422e5d/fd-b422e5d
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
- sym:39727 Exec.augment_args (src/cli.rs:883-952)
  fn augment_args(cmd: Command) -> Command
- sym:39779 CommandTemplate.new (src/exec/mod.rs:220-256)
  fn new<I, S>(input: I, mode: ExecutionMode) -> Result<CommandTemplate> where I: IntoIterator<Item = S>, S: AsRef<str>,
- sym:39726 Exec.update_from_arg_matches (src/cli.rs:876-879)
  fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> clap::error::Result<()>
- sym:39875 FormatTemplate.parse (src/fmt/mod.rs:58-107)
  pub fn parse(fmt: &str) -> Self
- sym:39905 ensure_use_hidden_option_for_leading_dot_pattern (src/main.rs:521-539)
  fn ensure_use_hidden_option_for_leading_dot_pattern( config: &Config, pattern_regexps: &[String], ) -> Result<()>
- sym:39725 Exec.from_arg_matches (src/cli.rs:862-874)
  fn from_arg_matches(matches: &ArgMatches) -> clap::error::Result<Self>
- sym:39918 hir_matches_strings_with_leading_dot (src/regex_helper.rs:50-79)
  fn hir_matches_strings_with_leading_dot(hir: &Hir) -> bool
- sym:39922 needs_escape (src/sanitize.rs:9-24)
  fn needs_escape(c: char) -> bool
- sym:39720 FileType (src/cli.rs:803-823)
  pub enum FileType
- sym:39778 CommandTemplate (src/exec/mod.rs:215-217)
  struct CommandTemplate
- sym:39872 FormatTemplate (src/fmt/mod.rs:46-49)
  pub enum FormatTemplate
- sym:39708 Opts (src/cli.rs:32-693)
  pub struct Opts
- sym:39761 execute_commands (src/exec/command.rs:60-99)
  pub fn execute_commands<I: Iterator<Item = io::Result<Command>>>( cmds: I, mut output_buffer: OutputBuffer, enable_output_buffering: bool, ) -> ExitCode
- sym:39917 pattern_matches_strings_with_leading_dot (src/regex_helper.rs:40-47)
  pub fn pattern_matches_strings_with_leading_dot(pattern: &str) -> bool
- sym:39910 print_entry_format (src/output.rs:69-86)
  fn print_entry_format<W: Write>( stdout: &mut W, entry: &DirEntry, config: &Config, format: &FormatTemplate, ) -> io::Result<()>
- sym:39874 FormatTemplate.has_tokens (src/fmt/mod.rs:54-56)
  pub fn has_tokens(&self) -> bool
- sym:39895 print_completions (src/main.rs:116-129)
  fn print_completions(shell: clap_complete::Shell) -> Result<ExitCode>
- sym:39901 construct_config (src/main.rs:248-393)
  fn construct_config(mut opts: Opts, pattern_regexps: &[String]) -> Result<Config>
- sym:39728 Exec.augment_args_for_update (src/cli.rs:954-956)
  fn augment_args_for_update(cmd: Command) -> Command
- sym:39961 WorkerState.new (src/walk.rs:318-328)
  fn new(patterns: Vec<Regex>, config: Config) -> Self
- sym:39952 ReceiverBuffer.new (src/walk.rs:153-171)
  fn new(state: &'a WorkerState, rx: Receiver<Batch>, stdout: W) -> Self
- sym:39745 starts_with_dash (src/dir_entry.rs:112-114)
  fn starts_with_dash(path: &Path) -> bool
- sym:39844 SizeFilter.parse_opt (src/filter/size.rs:33-66)
  fn parse_opt(s: &str) -> Option<Self>
- sym:39960 WorkerState (src/walk.rs:306-315)
  struct WorkerState
- sym:39966 WorkerState.scan (src/walk.rs:617-653)
  fn scan(&self, paths: &[PathBuf]) -> Result<ExitCode>
- sym:39754 safe_paths_not_flagged (src/dir_entry.rs:182-188)
  fn safe_paths_not_flagged()
- sym:39753 dash_prefixed_paths_detected (src/dir_entry.rs:175-179)
  fn dash_prefixed_paths_detected()
- sym:40102 test_opposing (tests/tests.rs:2682-2697)
  fn test_opposing(flag: &str, opposing_flags: &[&str])
- sym:40103 test_error_if_hidden_not_set_and_pattern_starts_with_dot (tests/tests.rs:2703-2712)
  fn test_error_if_hidden_not_set_and_pattern_starts_with_dot()
- sym:39709 Opts.search_paths (src/cli.rs:696-721)
  pub fn search_paths(&self) -> anyhow::Result<Vec<PathBuf>>
- sym:39710 Opts.normalize_path (src/cli.rs:723-736)
  fn normalize_path(&self, path: &Path) -> PathBuf
- sym:39719 default_num_threads (src/cli.rs:789-800)
  fn default_num_threads() -> NonZeroUsize
- sym:39735 DirEntry.normal (src/dir_entry.rs:26-32)
  pub fn normal(e: ignore::DirEntry) -> Self
- sym:39736 DirEntry.broken_symlink (src/dir_entry.rs:34-40)
  pub fn broken_symlink(path: PathBuf) -> Self
- sym:39758 OutputBuffer.new (src/exec/command.rs:19-24)
  pub fn new(null_separator: bool) -> Self
- sym:39759 OutputBuffer.push (src/exec/command.rs:26-28)
  fn push(&mut self, stdout: Vec<u8>, stderr: Vec<u8>)
- sym:39765 ExecutionMode (src/exec/mod.rs:22-27)
  pub enum ExecutionMode
- sym:39767 CommandSet.new (src/exec/mod.rs:36-49)
  pub fn new<I, T, S>(input: I) -> Result<CommandSet> where I: IntoIterator<Item = T>, T: IntoIterator<Item = S>, S: AsRef<str>,
- sym:39768 CommandSet.new_batch (src/exec/mod.rs:51-70)
  pub fn new_batch<I, T, S>(input: I) -> Result<CommandSet> where I: IntoIterator<Item = T>, T: IntoIterator<Item = S>, S: AsRef<str>,
- sym:39770 CommandSet.execute (src/exec/mod.rs:76-88)
  pub fn execute( &self, input: &Path, path_separator: Option<&str>, null_separator: bool, buffer_output: bool, ) -> ExitCode
- sym:39771 CommandSet.execute_batch (src/exec/mod.rs:90-120)
  pub fn execute_batch<I>(&self, paths: I, limit: usize, path_separator: Option<&str>) -> ExitCode where I: Iterator<Item = PathBuf>,
- sym:39773 CommandBuilder.new (src/exec/mod.rs:136-162)
  fn new(template: &CommandTemplate, limit: usize) -> io::Result<Self>
- sym:39774 CommandBuilder.new_command (src/exec/mod.rs:164-171)
  fn new_command(pre_args: &[OsString]) -> io::Result<Command>
- sym:39775 CommandBuilder.push (src/exec/mod.rs:173-189)
  fn push(&mut self, path: &Path, separator: Option<&str>) -> io::Result<()>
- sym:39781 CommandTemplate.generate (src/exec/mod.rs:266-272)
  fn generate(&self, input: &Path, path_separator: Option<&str>) -> io::Result<Command>
- sym:40010 test_and_pattern_starts_with_dash (tests/tests.rs:147-164)
  fn test_and_pattern_starts_with_dash()
- sym:40073 format (tests/tests.rs:1701-1758)
  fn format()
- sym:40082 test_fixed_strings (tests/tests.rs:2195-2227)
  fn test_fixed_strings()
- sym:39921 matches_strings_with_leading_dot_simple (src/regex_helper.rs:99-105)
  fn matches_strings_with_leading_dot_simple()
- sym:39879 parse_no_placeholders (src/fmt/mod.rs:219-225)
  fn parse_no_placeholders()
- sym:39789 tokens_with_literal_braces_and_placeholder (src/exec/mod.rs:379-383)
  fn tokens_with_literal_braces_and_placeholder()
- sym:40083 test_exact_literal_nonsubstring (tests/tests.rs:2231-2264)
  fn test_exact_literal_nonsubstring()
- sym:40014 test_and_plus_fixed_strings (tests/tests.rs:223-241)
  fn test_and_plus_fixed_strings()
- sym:39880 parse_only_brace_escapes (src/fmt/mod.rs:228-234)
  fn parse_only_brace_escapes()
- sym:39934 strips_bidi_overrides_and_zero_width (src/sanitize.rs:131-140)
  fn strips_bidi_overrides_and_zero_width()
- sym:39788 tokens_with_literal_braces (src/exec/mod.rs:369-376)
  fn tokens_with_literal_braces()
- sym:39862 is_time_filter_applicable (src/filter/time.rs:96-201)
  fn is_time_filter_applicable()
- sym:39796 generate_custom_path_separator (src/exec/mod.rs:446-457)
  fn generate_custom_path_separator()
- sym:39783 tokens_with_placeholder (src/exec/mod.rs:288-302)
  fn tokens_with_placeholder()
- sym:39784 tokens_with_no_extension (src/exec/mod.rs:305-318)
  fn tokens_with_no_extension()
- sym:40025 test_single_dash_root_path (tests/tests.rs:488-492)
  fn test_single_dash_root_path()
- sym:40011 test_and_plus_extension (tests/tests.rs:167-197)
  fn test_and_plus_extension()
- sym:39881 all_placeholders (src/fmt/mod.rs:237-280)
  fn all_placeholders()
- sym:39785 tokens_with_basename (src/exec/mod.rs:321-334)
  fn tokens_with_basename()
- sym:40009 test_and_bad_pattern (tests/tests.rs:137-144)
  fn test_and_bad_pattern()
- sym:39790 tokens_multiple (src/exec/mod.rs:386-403)
  fn tokens_multiple()
- sym:39975 TestEnv (tests/testenv/mod.rs:14-26)
  pub struct TestEnv
- sym:40050 test_no_global_ignore (tests/tests.rs:1049-1052)
  fn test_no_global_ignore(flag: &str, expected_output: &str)
- sym:40016 test_and_plus_case_sensitive (tests/tests.rs:255-262)
  fn test_and_plus_case_sensitive()
- sym:39999 TestEnv.assert_error_subdirectory (tests/testenv/mod.rs:320-343)
  fn assert_error_subdirectory<P: AsRef<Path>>( &self, path: P, args: &[&str], expected: Option<&str>, ) -> process::ExitStatus
- sym:40023 test_and_pattern_with_forward_slash_is_rejected (tests/tests.rs:425-443)
  fn test_and_pattern_with_forward_slash_is_rejected()

---
tokens: 2870/8000
