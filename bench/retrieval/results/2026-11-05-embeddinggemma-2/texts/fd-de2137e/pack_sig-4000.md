## Task
# fd-de2137e · fd-de2137e · open
Goal: refactor(filetypes): extract FileTypes construction into impl block
Workspace: /root/bench/av_gemma/ws/rec728b-fd-de2137e/fd-de2137e
Changed (0): nothing yet

## Map
- src/filetypes.rs
  L8: pub struct FileTypes
  L21: pub fn should_ignore(&self, entry: &dir_entry::DirEntry) -> bool
- src/config.rs
  L14: pub struct Config
  L140: pub fn is_printing(&self) -> bool
- src/main.rs
  L57: static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc
  L60: const DEFAULT_LS_COLORS: &str = "
  L65: fn main()
  L80: fn run() -> Result<ExitCode>
  L121: fn print_completions(shell: clap_complete::Shell) -> Result<ExitCode>
  L136: fn set_working_dir(opts: &Opts) -> Result<()>
  L174: fn ensure_search_pattern_is_not_a_path(opts: &Opts) -> Result<()>
  L190: fn ensure_single_search_pattern_is_not_a_path(pattern: &str) -> Result<()>
  L223: fn build_pattern_regex(pattern: &str, opts: &Opts) -> Result<String>
  L239: fn check_path_separator_length(path_separator: Option<&str>) -> Result<()>
  L253: fn construct_config(mut opts: Opts, pattern_regexps: &[String]) -> Result<Config>
  L400: fn extract_command(opts: &mut Opts, colored_output: bool) -> Result<Option<CommandSet>>
  L417: fn determine_ls_command(colored_output: bool) -> Result<Vec<&'static str>>
  L501: fn extract_time_constraints(opts: &Opts) -> Result<Vec<TimeFilter>>
  L526: fn ensure_use_hidden_option_for_leading_dot_pattern( config: &Config, pattern_regexps: &[String], ) -> Result<()>
  L546: fn build_regex(pattern_regex: String, config: &Config) -> Result<regex::bytes::Regex>
- src/cli.rs
  L31: pub struct Opts
  L697: pub fn search_paths(&self) -> anyhow::Result<Vec<PathBuf>>
  L726: fn normalize_path(&self, path: &Path) -> PathBuf
  L741: pub fn no_search_paths(&self) -> bool
  L746: pub fn rg_alias_ignore(&self) -> bool
  L750: pub fn max_depth(&self) -> Option<usize>
  L754: pub fn min_depth(&self) -> Option<usize>
  L758: pub fn threads(&self) -> NonZeroUsize
  L762: pub fn max_results(&self) -> Option<usize>
  L768: pub fn strip_cwd_prefix<P: FnOnce() -> bool>(&self, auto_pred: P) -> bool
  L779: pub fn gen_completions(&self) -> anyhow::Result<Option<Shell>>
  L792: fn default_num_threads() -> NonZeroUsize
  L806: pub enum FileType
  L829: pub enum ColorWhen
  L839: pub enum StripCwdWhen
  L849: pub enum HyperlinkWhen
  L860: pub struct Exec
  L865: fn from_arg_matches(matches: &ArgMatches) -> clap::error::Result<Self>
  L879: fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> clap::error::Result<()>
  L886: fn augment_args(cmd: Command) -> Command
  L962: fn augment_args_for_update(cmd: Command) -> Command
  L967: fn parse_millis(arg: &str) -> Result<Duration, std::num::ParseIntError>
  L971: fn ensure_current_directory_exists(current_directory: &Path) -> anyhow::Result<()>
- src/dir_entry.rs
  L12: enum DirEntryInner
  L20: pub struct DirEntry
  L28: pub fn normal(e: ignore::DirEntry) -> Self
  L36: pub fn broken_symlink(path: PathBuf, depth: Option<usize>) -> Self
  L44: pub fn path(&self) -> &Path
  L51: pub fn into_path(self) -> PathBuf
  L61: pub fn stripped_path(&self, config: &Config) -> &Path
  L76: pub fn into_stripped_path(self, config: &Config) -> PathBuf
  L84: pub fn file_type(&self) -> Option<FileType>
  L91: pub fn metadata(&self) -> Option<&Metadata>
  L100: pub fn depth(&self) -> Option<usize>
  L107: pub fn style(&self, ls_colors: &LsColors) -> Option<&Style>
  L114: fn starts_with_dash(path: &Path) -> bool
  L120: fn eq(&self, other: &Self) -> bool
  L129: fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
  L136: fn cmp(&self, other: &Self) -> std::cmp::Ordering
  L146: fn file_name(&self) -> OsString
  L177: fn dash_prefixed_paths_detected()
  L184: fn safe_paths_not_flagged()
- src/error.rs
  L14: pub(crate) trait SanitizeErr
  L17: fn sanitize(self) -> Self::Sanitized
  L21: type Sanitized = Self
  L23: fn sanitize(self) -> Self
  L29: type Sanitized = SanitizedError<io::Error>
  L31: fn sanitize(self) -> Self::Sanitized
  L44: pub struct SanitizedError<E>
  L47: fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
- src/exec/command.rs
  L8: struct Outputs
  L12: pub struct OutputBuffer
  L18: pub fn new(null_separator: bool) -> Self
  L25: fn push(&mut self, stdout: Vec<u8>, stderr: Vec<u8>)
  L29: fn write(self)
  L59: pub fn execute_commands<I: Iterator<Item = io::Result<Command>>>( cmds: I, mut output_buffer: OutputBuffer, enable_output_buffering: bool, ) -> ExitCode
  L100: pub fn handle_cmd_error(cmd: Option<&Command>, err: io::Error) -> ExitCode
- src/exec/job.rs
  L10: pub fn job( results: impl IntoIterator<Item = WorkerResult>, cmd: &CommandSet, config: &Config, ) -> ExitCode
  L45: pub fn batch( results: impl IntoIterator<Item = WorkerResult>, cmd: &CommandSet, config: &Config, ) -> ExitCode
- src/exec/mod.rs
- src/exit_codes.rs
- src/filesystem.rs
- src/filter/owner.rs
- src/filter/size.rs
- src/filter/time.rs
- src/fmt/mod.rs
- src/hyperlink.rs
- repo map: .cargo/ .github/ .gitignore CHANGELOG.md CONTRIBUTING.md Cargo.lock Cargo.toml Cross.toml LICENSE-APACHE LICENSE-MIT Makefile README.md (60 files)

## Code
- sym:38984 FileTypes (src/filetypes.rs:8-18)
  pub struct FileTypes
- sym:38872 FileType (src/cli.rs:806-826)
  pub enum FileType
- sym:38971 is_block_device (src/filesystem.rs:63-65)
  pub fn is_block_device(ft: fs::FileType) -> bool
- sym:38985 FileTypes.should_ignore (src/filetypes.rs:21-42)
  pub fn should_ignore(&self, entry: &dir_entry::DirEntry) -> bool
- sym:39065 extract_time_constraints (src/main.rs:501-524)
  fn extract_time_constraints(opts: &Opts) -> Result<Vec<TimeFilter>>
- sym:38883 Config (src/config.rs:14-136)
  pub struct Config
- sym:39062 construct_config (src/main.rs:253-398)
  fn construct_config(mut opts: Opts, pattern_regexps: &[String]) -> Result<Config>
- sym:39033 FormatTemplate (src/fmt/mod.rs:46-49)
  pub enum FormatTemplate
- sym:38925 batch (src/exec/job.rs:45-63)
  pub fn batch( results: impl IntoIterator<Item = WorkerResult>, cmd: &CommandSet, config: &Config, ) -> ExitCode
- sym:38924 job (src/exec/job.rs:10-43)
  pub fn job( results: impl IntoIterator<Item = WorkerResult>, cmd: &CommandSet, config: &Config, ) -> ExitCode
- sym:38928 CommandSet.new (src/exec/mod.rs:36-49)
  pub fn new<I, T, S>(input: I) -> Result<CommandSet> where I: IntoIterator<Item = T>, T: IntoIterator<Item = S>, S: AsRef<str>,
- sym:38902 DirEntry.file_name (src/dir_entry.rs:146-160)
  fn file_name(&self) -> OsString
- sym:39063 extract_command (src/main.rs:400-415)
  fn extract_command(opts: &mut Opts, colored_output: bool) -> Result<Option<CommandSet>>
- sym:38975 is_socket (src/filesystem.rs:83-85)
  pub fn is_socket(ft: fs::FileType) -> bool
- sym:38939 CommandTemplate (src/exec/mod.rs:215-217)
  struct CommandTemplate
- sym:38963 merge_exitcodes (src/exit_codes.rs:46-51)
  pub fn merge_exitcodes(results: impl IntoIterator<Item = ExitCode>) -> ExitCode
- sym:38977 is_pipe (src/filesystem.rs:93-95)
  pub fn is_pipe(ft: fs::FileType) -> bool
- sym:38973 is_char_device (src/filesystem.rs:73-75)
  pub fn is_char_device(ft: fs::FileType) -> bool
- sym:39037 FormatTemplate.generate (src/fmt/mod.rs:112-141)
  pub fn generate(&self, path: impl AsRef<Path>, path_separator: Option<&str>) -> OsString
- sym:38893 DirEntry.file_type (src/dir_entry.rs:84-89)
  pub fn file_type(&self) -> Option<FileType>
- sym:39130 broken_symlink_from_err (src/walk.rs:647-686)
  fn broken_symlink_from_err(mut err: ignore::Error) -> Result<DirEntry, ignore::Error>
- sym:39031 Token (src/fmt/mod.rs:18-25)
  pub enum Token
- sym:38860 Opts (src/cli.rs:31-694)
  pub struct Opts
- sym:38864 Opts.rg_alias_ignore (src/cli.rs:746-748)
  pub fn rg_alias_ignore(&self) -> bool
- sym:38865 Opts.max_depth (src/cli.rs:750-752)
  pub fn max_depth(&self) -> Option<usize>
- sym:38866 Opts.min_depth (src/cli.rs:754-756)
  pub fn min_depth(&self) -> Option<usize>
- sym:38867 Opts.threads (src/cli.rs:758-760)
  pub fn threads(&self) -> NonZeroUsize
- sym:38868 Opts.max_results (src/cli.rs:762-766)
  pub fn max_results(&self) -> Option<usize>
- sym:38869 Opts.strip_cwd_prefix (src/cli.rs:768-776)
  pub fn strip_cwd_prefix<P: FnOnce() -> bool>(&self, auto_pred: P) -> bool
- sym:38873 ColorWhen (src/cli.rs:829-836)
  pub enum ColorWhen
- sym:38875 HyperlinkWhen (src/cli.rs:849-856)
  pub enum HyperlinkWhen
- sym:38889 DirEntry.path (src/dir_entry.rs:44-49)
  pub fn path(&self) -> &Path
- sym:38891 DirEntry.stripped_path (src/dir_entry.rs:61-73)
  pub fn stripped_path(&self, config: &Config) -> &Path
- sym:38892 DirEntry.into_stripped_path (src/dir_entry.rs:76-82)
  pub fn into_stripped_path(self, config: &Config) -> PathBuf
- sym:38916 SanitizedError.fmt (src/error.rs:47-50)
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
- sym:38919 OutputBuffer.new (src/exec/command.rs:18-23)
  pub fn new(null_separator: bool) -> Self
- sym:38927 CommandSet (src/exec/mod.rs:30-33)
  pub struct CommandSet
- sym:38934 CommandBuilder.new (src/exec/mod.rs:136-162)
  fn new(template: &CommandTemplate, limit: usize) -> io::Result<Self>
- sym:38940 CommandTemplate.new (src/exec/mod.rs:220-256)
  fn new<I, S>(input: I, mode: ExecutionMode) -> Result<CommandTemplate> where I: IntoIterator<Item = S>, S: AsRef<str>,
- sym:38960 i32.from (src/exit_codes.rs:15-22)
  fn from(code: ExitCode) -> Self
- sym:38970 is_empty (src/filesystem.rs:44-60)
  pub fn is_empty(entry: &dir_entry::DirEntry) -> bool
- sym:38982 default_path_separator (src/filesystem.rs:128-136)
  pub fn default_path_separator() -> Option<String>
- sym:38986 OwnerFilter (src/filter/owner.rs:6-9)
  pub struct OwnerFilter
- sym:38989 OwnerFilter.from_string (src/filter/owner.rs:27-58)
  pub fn from_string(input: &str) -> Result<Self>
- sym:38990 OwnerFilter.filter_ignore (src/filter/owner.rs:61-67)
  pub fn filter_ignore(self) -> Option<Self>
- sym:38993 Check.parse (src/filter/owner.rs:85-102)
  fn parse<F>(s: Option<&str>, f: F) -> Result<Self> where F: Fn(&str) -> Result<T>,
- sym:38995 SizeFilter (src/filter/size.rs:9-13)
  pub enum SizeFilter
- sym:39004 SizeFilter.from_string (src/filter/size.rs:28-31)
  pub fn from_string(s: &str) -> anyhow::Result<Self>
- sym:39011 TimeFilter (src/filter/time.rs:7-10)
  pub enum TimeFilter
- sym:39032 Token.fmt (src/fmt/mod.rs:28-38)
  fn fmt(&self, f: &mut Formatter) -> fmt::Result
- sym:39036 FormatTemplate.parse (src/fmt/mod.rs:58-107)
  pub fn parse(fmt: &str) -> Self
- sym:39044 PathUrl.new (src/hyperlink.rs:8-10)
  pub(crate) fn new(path: &Path) -> Option<PathUrl>
- sym:39045 PathUrl.fmt (src/hyperlink.rs:14-21)
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
- sym:39217 test_file_system_boundaries (tests/tests.rs:1074-1109)
  fn test_file_system_boundaries()
- sym:39018 TestTime (src/filter/time.rs:70-70)
  struct TestTime
- sym:39022 TestTime.drop (src/filter/time.rs:89-92)
  fn drop(&mut self)
- sym:39228 test_prune (tests/tests.rs:1427-1457)
  fn test_prune()
- sym:39232 test_type (tests/tests.rs:1537-1570)
  fn test_type()
- sym:39019 TestTime.new (src/filter/time.rs:73-76)
  fn new(time: Zoned) -> Self
- sym:39050 Encoded.fmt (src/hyperlink.rs:72-77)
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result

---
tokens: 3572/4000
