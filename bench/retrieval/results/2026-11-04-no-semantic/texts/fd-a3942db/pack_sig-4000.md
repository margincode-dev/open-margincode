## Task
# fd-a3942db · fd-a3942db-2 · open
Goal: fix: don't sanitize error message

Calling `maybe_sanitize` on error messages results in newlines getting
replaced with "\x0a". This is not desirable for error messages that have
newlines.

I audited everwhere we construct an error, and call print_error and
confirmed that none of those places have inputs that include found
paths. I'm not too worried about inputs that come from command line
arguments.

This bug was introduced in 92c54d37c0a3425b4d8b25dd8c9e4ddbbecfbe0a.

Fixes: #2104
Workspace: /root/bench/ws/rebf815-fd-a3942db/fd-a3942db-2
Changed (0): nothing yet

## Map
- src/error.rs
  L5: pub fn print_error(msg: impl Into<String>)
- src/sanitize.rs
  L9: fn needs_escape(c: char) -> bool
  L28: pub fn sanitize_for_terminal(s: &str) -> Cow<'_, str>
  L49: pub fn maybe_sanitize<'a>(s: &'a str, is_terminal: bool) -> Cow<'a, str>
  L62: fn preserves_safe_content()
  L80: fn strips_osc52_clipboard_payload()
  L88: fn strips_cr_output_forgery()
  L93: fn strips_osc8_hyperlink_injection()
  L99: fn strips_del()
  L104: fn strips_bel_and_null()
  L110: fn strips_newline()
  L115: fn escape_preserves_information()
  L121: fn strips_c1_csi_and_osc_initiators()
  L131: fn strips_bidi_overrides_and_zero_width()
  L143: fn keeps_legitimate_unicode_features()
  L159: fn maybe_sanitize_passthrough_when_not_terminal()
- src/cli.rs
  L32: pub struct Opts
  L696: pub fn search_paths(&self) -> anyhow::Result<Vec<PathBuf>>
  L723: fn normalize_path(&self, path: &Path) -> PathBuf
  L738: pub fn no_search_paths(&self) -> bool
  L743: pub fn rg_alias_ignore(&self) -> bool
  L747: pub fn max_depth(&self) -> Option<usize>
  L751: pub fn min_depth(&self) -> Option<usize>
  L755: pub fn threads(&self) -> NonZeroUsize
  L759: pub fn max_results(&self) -> Option<usize>
  L765: pub fn strip_cwd_prefix<P: FnOnce() -> bool>(&self, auto_pred: P) -> bool
  L776: pub fn gen_completions(&self) -> anyhow::Result<Option<Shell>>
  L789: fn default_num_threads() -> NonZeroUsize
  L803: pub enum FileType
  L826: pub enum ColorWhen
  L836: pub enum StripCwdWhen
  L846: pub enum HyperlinkWhen
  L857: pub struct Exec
  L862: fn from_arg_matches(matches: &ArgMatches) -> clap::error::Result<Self>
  L876: fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> clap::error::Result<()>
  L883: fn augment_args(cmd: Command) -> Command
  L954: fn augment_args_for_update(cmd: Command) -> Command
  L959: fn parse_millis(arg: &str) -> Result<Duration, std::num::ParseIntError>
  L963: fn ensure_current_directory_exists(current_directory: &Path) -> anyhow::Result<()>
- src/exec/command.rs
  L9: struct Outputs
  L13: pub struct OutputBuffer
  L19: pub fn new(null_separator: bool) -> Self
  L26: fn push(&mut self, stdout: Vec<u8>, stderr: Vec<u8>)
  L30: fn write(self)
  L60: pub fn execute_commands<I: Iterator<Item = io::Result<Command>>>( cmds: I, mut output_buffer: OutputBuffer, enable_output_buffering: bool, ) -> ExitCode
  L101: pub fn handle_cmd_error(cmd: Option<&Command>, err: io::Error) -> ExitCode
- src/exec/job.rs
  L11: pub fn job( results: impl IntoIterator<Item = WorkerResult>, cmd: &CommandSet, config: &Config, ) -> ExitCode
  L46: pub fn batch( results: impl IntoIterator<Item = WorkerResult>, cmd: &CommandSet, config: &Config, ) -> ExitCode
- src/main.rs
  L54: static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc
  L57: const DEFAULT_LS_COLORS: &str = "
  L62: fn main()
  L75: fn run() -> Result<ExitCode>
  L116: fn print_completions(shell: clap_complete::Shell) -> Result<ExitCode>
  L131: fn set_working_dir(opts: &Opts) -> Result<()>
  L169: fn ensure_search_pattern_is_not_a_path(opts: &Opts) -> Result<()>
  L185: fn ensure_single_search_pattern_is_not_a_path(pattern: &str) -> Result<()>
  L218: fn build_pattern_regex(pattern: &str, opts: &Opts) -> Result<String>
  L234: fn check_path_separator_length(path_separator: Option<&str>) -> Result<()>
  L248: fn construct_config(mut opts: Opts, pattern_regexps: &[String]) -> Result<Config>
  L395: fn extract_command(opts: &mut Opts, colored_output: bool) -> Result<Option<CommandSet>>
  L412: fn determine_ls_command(colored_output: bool) -> Result<Vec<&'static str>>
  L496: fn extract_time_constraints(opts: &Opts) -> Result<Vec<TimeFilter>>
  L521: fn ensure_use_hidden_option_for_leading_dot_pattern( config: &Config, pattern_regexps: &[String], ) -> Result<()>
  L541: fn build_regex(pattern_regex: String, config: &Config) -> Result<regex::bytes::Regex>
- src/output.rs
  L12: fn replace_path_separator(path: &str, new_path_separator: &str) -> String
  L17: pub fn print_entry<W: Write>(stdout: &mut W, entry: &DirEntry, config: &Config) -> io::Result<()>
  L49: fn print_trailing_slash<W: Write>( stdout: &mut W, entry: &DirEntry, config: &Config, style: Option<&Style>, ) -> io::Result<()>
  L69: fn print_entry_format<W: Write>( stdout: &mut W, entry: &DirEntry, config: &Config, format: &FormatTemplate, ) -> io::Result<()>
  L89: fn print_entry_colorized<W: Write>( stdout: &mut W, entry: &DirEntry, config: &Config, ls_colors: &LsColors, ) -> io::Result<()>
  L142: fn print_entry_uncolorized_base<W: Write>( stdout: &mut W, entry: &DirEntry, config: &Config, ) -> io::Result<()>
  L159: fn print_entry_uncolorized<W: Write>( stdout: &mut W, entry: &DirEntry, config: &Config, ) -> io::Result<()>
- src/walk.rs
- src/config.rs
  L14: pub struct Config
  L140: pub fn is_printing(&self) -> bool
- src/dir_entry.rs
- src/exec/mod.rs
- src/exit_codes.rs
- repo map: .cargo/ .github/ .gitignore CHANGELOG.md CONTRIBUTING.md Cargo.lock Cargo.toml Cross.toml LICENSE-APACHE LICENSE-MIT Makefile README.md (59 files)

## Code
- sym:40159 print_error (src/error.rs:5-9)
  pub fn print_error(msg: impl Into<String>)
- sym:40328 maybe_sanitize (src/sanitize.rs:49-55)
  pub fn maybe_sanitize<'a>(s: &'a str, is_terminal: bool) -> Cow<'a, str>
- sym:40315 print_entry_colorized (src/output.rs:89-139)
  fn print_entry_colorized<W: Write>( stdout: &mut W, entry: &DirEntry, config: &Config, ls_colors: &LsColors, ) -> io::Result<()>
- sym:40316 print_entry_uncolorized_base (src/output.rs:142-156)
  fn print_entry_uncolorized_base<W: Write>( stdout: &mut W, entry: &DirEntry, config: &Config, ) -> io::Result<()>
- sym:40314 print_entry_format (src/output.rs:69-86)
  fn print_entry_format<W: Write>( stdout: &mut W, entry: &DirEntry, config: &Config, format: &FormatTemplate, ) -> io::Result<()>
- sym:40359 ReceiverBuffer.poll (src/walk.rs:198-249)
  fn poll(&mut self) -> Result<(), ExitCode>
- sym:40168 batch (src/exec/job.rs:46-64)
  pub fn batch( results: impl IntoIterator<Item = WorkerResult>, cmd: &CommandSet, config: &Config, ) -> ExitCode
- sym:40167 job (src/exec/job.rs:11-44)
  pub fn job( results: impl IntoIterator<Item = WorkerResult>, cmd: &CommandSet, config: &Config, ) -> ExitCode
- sym:40203 i32.from (src/exit_codes.rs:15-22)
  fn from(code: ExitCode) -> Self
- sym:40206 merge_exitcodes (src/exit_codes.rs:46-51)
  pub fn merge_exitcodes(results: impl IntoIterator<Item = ExitCode>) -> ExitCode
- sym:40305 construct_config (src/main.rs:248-393)
  fn construct_config(mut opts: Opts, pattern_regexps: &[String]) -> Result<Config>
- sym:40122 Opts.gen_completions (src/cli.rs:776-785)
  pub fn gen_completions(&self) -> anyhow::Result<Option<Shell>>
- sym:40302 ensure_single_search_pattern_is_not_a_path (src/main.rs:185-216)
  fn ensure_single_search_pattern_is_not_a_path(pattern: &str) -> Result<()>
- sym:40113 Opts.search_paths (src/cli.rs:696-721)
  pub fn search_paths(&self) -> anyhow::Result<Vec<PathBuf>>
- sym:40166 handle_cmd_error (src/exec/command.rs:101-115)
  pub fn handle_cmd_error(cmd: Option<&Command>, err: io::Error) -> ExitCode
- sym:40297 main (src/main.rs:62-73)
  fn main()
- sym:40327 sanitize_for_terminal (src/sanitize.rs:28-46)
  pub fn sanitize_for_terminal(s: &str) -> Cow<'_, str>
- sym:40360 ReceiverBuffer.print (src/walk.rs:252-267)
  fn print(&mut self, entry: &DirEntry) -> Result<(), ExitCode>
- sym:40367 WorkerState.build_walker (src/walk.rs:347-404)
  fn build_walker(&self, paths: &[PathBuf]) -> Result<WalkParallel>
- sym:40340 maybe_sanitize_passthrough_when_not_terminal (src/sanitize.rs:159-167)
  fn maybe_sanitize_passthrough_when_not_terminal()
- sym:40114 Opts.normalize_path (src/cli.rs:723-736)
  fn normalize_path(&self, path: &Path) -> PathBuf
- sym:40117 Opts.max_depth (src/cli.rs:747-749)
  pub fn max_depth(&self) -> Option<usize>
- sym:40119 Opts.threads (src/cli.rs:755-757)
  pub fn threads(&self) -> NonZeroUsize
- sym:40120 Opts.max_results (src/cli.rs:759-763)
  pub fn max_results(&self) -> Option<usize>
- sym:40134 ensure_current_directory_exists (src/cli.rs:963-971)
  fn ensure_current_directory_exists(current_directory: &Path) -> anyhow::Result<()>
- sym:40135 Config (src/config.rs:14-136)
  pub struct Config
- sym:40138 DirEntry (src/dir_entry.rs:18-22)
  pub struct DirEntry
- sym:40141 DirEntry.path (src/dir_entry.rs:42-47)
  pub fn path(&self) -> &Path
- sym:40143 DirEntry.stripped_path (src/dir_entry.rs:59-71)
  pub fn stripped_path(&self, config: &Config) -> &Path
- sym:40144 DirEntry.into_stripped_path (src/dir_entry.rs:74-80)
  pub fn into_stripped_path(self, config: &Config) -> PathBuf
- sym:40148 DirEntry.style (src/dir_entry.rs:105-109)
  pub fn style(&self, ls_colors: &LsColors) -> Option<&Style>
- sym:40162 OutputBuffer.new (src/exec/command.rs:19-24)
  pub fn new(null_separator: bool) -> Self
- sym:40163 OutputBuffer.push (src/exec/command.rs:26-28)
  fn push(&mut self, stdout: Vec<u8>, stderr: Vec<u8>)
- sym:40164 OutputBuffer.write (src/exec/command.rs:30-56)
  fn write(self)
- sym:40165 execute_commands (src/exec/command.rs:60-99)
  pub fn execute_commands<I: Iterator<Item = io::Result<Command>>>( cmds: I, mut output_buffer: OutputBuffer, enable_output_buffering: bool, ) -> ExitCode
- sym:40170 CommandSet (src/exec/mod.rs:30-33)
  pub struct CommandSet
- sym:40171 CommandSet.new (src/exec/mod.rs:36-49)
  pub fn new<I, T, S>(input: I) -> Result<CommandSet> where I: IntoIterator<Item = T>, T: IntoIterator<Item = S>, S: AsRef<str>,
- sym:40174 CommandSet.execute (src/exec/mod.rs:76-88)
  pub fn execute( &self, input: &Path, path_separator: Option<&str>, null_separator: bool, buffer_output: bool, ) -> ExitCode
- sym:40175 CommandSet.execute_batch (src/exec/mod.rs:90-120)
  pub fn execute_batch<I>(&self, paths: I, limit: usize, path_separator: Option<&str>) -> ExitCode where I: Iterator<Item = PathBuf>,
- sym:40177 CommandBuilder.new (src/exec/mod.rs:136-162)
  fn new(template: &CommandTemplate, limit: usize) -> io::Result<Self>
- sym:40179 CommandBuilder.push (src/exec/mod.rs:173-189)
  fn push(&mut self, path: &Path, separator: Option<&str>) -> io::Result<()>
- sym:40181 CommandBuilder.exit_code (src/exec/mod.rs:205-207)
  fn exit_code(&self) -> ExitCode
- sym:40183 CommandTemplate.new (src/exec/mod.rs:220-256)
  fn new<I, S>(input: I, mode: ExecutionMode) -> Result<CommandTemplate> where I: IntoIterator<Item = S>, S: AsRef<str>,
- sym:40185 CommandTemplate.generate (src/exec/mod.rs:266-272)
  fn generate(&self, input: &Path, path_separator: Option<&str>) -> io::Result<Command>
- sym:40202 ExitCode (src/exit_codes.rs:7-12)
  pub enum ExitCode
- sym:40205 ExitCode.exit (src/exit_codes.rs:31-43)
  pub fn exit(self) -> !
- sym:40425 test_pattern_with_forward_slash_is_rejected (tests/tests.rs:383-397)
  fn test_pattern_with_forward_slash_is_rejected()
- sym:40485 test_quiet (tests/tests.rs:2182-2191)
  fn test_quiet()

---
tokens: 3526/4000
