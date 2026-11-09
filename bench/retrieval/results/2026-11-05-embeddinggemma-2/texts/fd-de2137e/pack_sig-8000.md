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
  L22: pub enum ExecutionMode
  L30: pub struct CommandSet
  L36: pub fn new<I, T, S>(input: I) -> Result<CommandSet> where I: IntoIterator<Item = T>, T: IntoIterator<Item = S>, S: AsRef<str>,
  L51: pub fn new_batch<I, T, S>(input: I) -> Result<CommandSet> where I: IntoIterator<Item = T>, T: IntoIterator<Item = S>, S: AsRef<str>,
  L72: pub fn in_batch_mode(&self) -> bool
  L76: pub fn execute( &self, input: &Path, path_separator: Option<&str>, null_separator: bool, buffer_output: bool, ) -> ExitCode
  L90: pub fn execute_batch<I>(&self, paths: I, limit: usize, path_separator: Option<&str>) -> ExitCode where I: Iterator<Item = PathBuf>,
  L125: struct CommandBuilder
  L136: fn new(template: &CommandTemplate, limit: usize) -> io::Result<Self>
  L164: fn new_command(pre_args: &[OsString]) -> io::Result<Command>
  L173: fn push(&mut self, path: &Path, separator: Option<&str>) -> io::Result<()>
  L191: fn finish(&mut self) -> io::Result<()>
  L205: fn exit_code(&self) -> ExitCode
  L215: struct CommandTemplate
  L220: fn new<I, S>(input: I, mode: ExecutionMode) -> Result<CommandTemplate> where I: IntoIterator<Item = S>, S: AsRef<str>,
  L258: fn number_of_tokens(&self) -> usize
  L266: fn generate(&self, input: &Path, path_separator: Option<&str>) -> io::Result<Command>
  L279: fn generate_str(template: &CommandTemplate, input: &str) -> Vec<String>
  L288: fn tokens_with_placeholder()
  L305: fn tokens_with_no_extension()
  L321: fn tokens_with_basename()
  L337: fn tokens_with_parent()
  L353: fn tokens_with_basename_no_extension()
  L369: fn tokens_with_literal_braces()
  L379: fn tokens_with_literal_braces_and_placeholder()
  L386: fn tokens_multiple()
  L406: fn tokens_single_batch()
  L422: fn tokens_multiple_batch()
  L427: fn template_no_args()
  L434: fn command_set_no_args()
- src/exit_codes.rs
  L7: pub enum ExitCode
  L15: fn from(code: ExitCode) -> Self
  L26: fn is_error(self) -> bool
  L31: pub fn exit(self) -> !
  L46: pub fn merge_exitcodes(results: impl IntoIterator<Item = ExitCode>) -> ExitCode
  L58: fn success_when_no_results()
  L63: fn general_error_if_at_least_one_error()
  L87: fn success_if_no_error()
- src/filesystem.rs
  L14: pub fn path_absolute_form(path: &Path) -> io::Result<PathBuf>
  L23: pub fn absolute_path(path: &Path) -> io::Result<PathBuf>
  L38: pub fn is_existing_directory(path: &Path) -> bool
  L44: pub fn is_empty(entry: &dir_entry::DirEntry) -> bool
  L63: pub fn is_block_device(ft: fs::FileType) -> bool
  L73: pub fn is_char_device(ft: fs::FileType) -> bool
  L83: pub fn is_socket(ft: fs::FileType) -> bool
  L93: pub fn is_pipe(ft: fs::FileType) -> bool
  L103: pub fn osstr_to_bytes(input: &OsStr) -> Cow<'_, [u8]>
  L119: pub fn strip_current_dir(path: &Path) -> &Path
  L128: pub fn default_path_separator() -> Option<String>
  L144: fn strip_current_dir_basic()
- src/filter/owner.rs
  L6: pub struct OwnerFilter
  L12: enum Check<T>
  L19: const IGNORE: Self = OwnerFilter {
  L27: pub fn from_string(input: &str) -> Result<Self>
  L61: pub fn filter_ignore(self) -> Option<Self>
  L69: pub fn matches(&self, md: &fs::Metadata) -> bool
  L77: fn check(&self, v: T) -> bool
  L85: fn parse<F>(s: Option<&str>, f: F) -> Result<Self> where F: Fn(&str) -> Result<T>,
- src/filter/size.rs
  L6: static SIZE_CAPTURES: OnceLock<Regex> = OnceLock::new()
  L9: pub enum SizeFilter
  L16: const KILO: u64 = 1000
  L17: const MEGA: u64 = KILO * 1000
  L18: const GIGA: u64 = MEGA * 1000
  L19: const TERA: u64 = GIGA * 1000
  L22: const KIBI: u64 = 1024
  L23: const MEBI: u64 = KIBI * 1024
  L24: const GIBI: u64 = MEBI * 1024
  L25: const TEBI: u64 = GIBI * 1024
  L28: pub fn from_string(s: &str) -> anyhow::Result<Self>
  L33: fn parse_opt(s: &str) -> Option<Self>
  L68: pub fn is_within(&self, size: u64) -> bool
  L197: fn is_within_less_than()
  L203: fn is_within_less_than_equal()
  L209: fn is_within_greater_than()
  L215: fn is_within_greater_than_equal()
- src/filter/time.rs
  L7: pub enum TimeFilter
  L13: fn now() -> Zoned
  L29: fn from_str(s: &str) -> Option<SystemTime>
  L49: pub fn before(s: &str) -> Option<TimeFilter>
  L53: pub fn after(s: &str) -> Option<TimeFilter>
  L57: pub fn applies_to(&self, t: &SystemTime) -> bool
  L70: struct TestTime
  L73: fn new(time: Zoned) -> Self
  L78: fn set(&mut self, time: Zoned)
  L83: fn timestamp(&self) -> SystemTime
  L89: fn drop(&mut self)
  L96: fn is_time_filter_applicable()
  L204: fn out_of_range_unix_timestamp_is_rejected()
- src/fmt/mod.rs
  L18: pub enum Token
  L28: fn fmt(&self, f: &mut Formatter) -> fmt::Result
  L46: pub enum FormatTemplate
  L51: static PLACEHOLDERS: OnceLock<AhoCorasick> = OnceLock::new()
  L54: pub fn has_tokens(&self) -> bool
  L58: pub fn parse(fmt: &str) -> Self
  L112: pub fn generate(&self, path: impl AsRef<Path>, path_separator: Option<&str>) -> OsString
  L147: fn replace_separator<'a>(path: &'a OsStr, path_separator: Option<&str>) -> Cow<'a, OsStr>
  L201: fn token_from_pattern_id(id: u32) -> Token
  L219: fn parse_no_placeholders()
  L228: fn parse_only_brace_escapes()
  L237: fn all_placeholders()
- src/hyperlink.rs
  L5: pub(crate) struct PathUrl
  L8: pub(crate) fn new(path: &Path) -> Option<PathUrl>
  L14: fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
  L24: fn encode(f: &mut Formatter, byte: u8) -> fmt::Result
  L44: fn host() -> &'static str
  L69: struct Encoded
  L72: fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
  L81: fn test_unicode_encoding()
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
    exec/ (3 files)
    filter/ (4 files)
    fmt/ (2 files)
    output.rs
    regex_helper.rs
    sanitize.rs
    walk.rs
  tests/ (2 files)
    testenv/ (1 files)
    tests.rs

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
- sym:39062 construct_config (src/main.rs:253-398)
  fn construct_config(mut opts: Opts, pattern_regexps: &[String]) -> Result<Config>
- sym:38893 DirEntry.file_type (src/dir_entry.rs:84-89)
  pub fn file_type(&self) -> Option<FileType>
- sym:38883 Config (src/config.rs:14-136)
  pub struct Config
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
- sym:38927 CommandSet (src/exec/mod.rs:30-33)
  pub struct CommandSet
- sym:39011 TimeFilter (src/filter/time.rs:7-10)
  pub enum TimeFilter
- sym:38977 is_pipe (src/filesystem.rs:93-95)
  pub fn is_pipe(ft: fs::FileType) -> bool
- sym:38973 is_char_device (src/filesystem.rs:73-75)
  pub fn is_char_device(ft: fs::FileType) -> bool
- sym:39037 FormatTemplate.generate (src/fmt/mod.rs:112-141)
  pub fn generate(&self, path: impl AsRef<Path>, path_separator: Option<&str>) -> OsString
- sym:38940 CommandTemplate.new (src/exec/mod.rs:220-256)
  fn new<I, S>(input: I, mode: ExecutionMode) -> Result<CommandTemplate> where I: IntoIterator<Item = S>, S: AsRef<str>,
- sym:38970 is_empty (src/filesystem.rs:44-60)
  pub fn is_empty(entry: &dir_entry::DirEntry) -> bool
- sym:39130 broken_symlink_from_err (src/walk.rs:647-686)
  fn broken_symlink_from_err(mut err: ignore::Error) -> Result<DirEntry, ignore::Error>
- sym:39031 Token (src/fmt/mod.rs:18-25)
  pub enum Token
- sym:39004 SizeFilter.from_string (src/filter/size.rs:28-31)
  pub fn from_string(s: &str) -> anyhow::Result<Self>
- sym:39053 DEFAULT_LS_COLORS (src/main.rs:60-63)
  const DEFAULT_LS_COLORS: &str = "
- sym:39034 PLACEHOLDERS (src/fmt/mod.rs:51-51)
  static PLACEHOLDERS: OnceLock<AhoCorasick> = OnceLock::new()
- sym:39025 basename (src/fmt/input.rs:7-9)
  pub fn basename(path: &Path) -> &OsStr
- sym:39035 FormatTemplate.has_tokens (src/fmt/mod.rs:54-56)
  pub fn has_tokens(&self) -> bool
- sym:39052 ALLOC (src/main.rs:57-57)
  static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc
- sym:39126 WorkerState.build_walker (src/walk.rs:346-403)
  fn build_walker(&self, paths: &[PathBuf]) -> Result<WalkParallel>
- sym:38899 DirEntry.partial_cmp (src/dir_entry.rs:129-131)
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
- sym:38876 Exec (src/cli.rs:860-862)
  pub struct Exec
- sym:39054 main (src/main.rs:65-78)
  fn main()
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
- sym:38934 CommandBuilder.new (src/exec/mod.rs:136-162)
  fn new(template: &CommandTemplate, limit: usize) -> io::Result<Self>
- sym:38960 i32.from (src/exit_codes.rs:15-22)
  fn from(code: ExitCode) -> Self
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
- sym:39019 TestTime.new (src/filter/time.rs:73-76)
  fn new(time: Zoned) -> Self
- sym:39228 test_prune (tests/tests.rs:1427-1457)
  fn test_prune()
- sym:39232 test_type (tests/tests.rs:1537-1570)
  fn test_type()
- sym:39177 test_and_plus_type (tests/tests.rs:200-213)
  fn test_and_plus_type()
- sym:39203 test_hidden_file_attribute (tests/tests.rs:700-717)
  fn test_hidden_file_attribute()
- sym:39141 create_config_directory_with_global_ignore (tests/testenv/mod.rs:65-73)
  fn create_config_directory_with_global_ignore(ignore_file_content: &str) -> io::Result<TempDir>
- sym:39234 test_type_empty (tests/tests.rs:1617-1640)
  fn test_type_empty()
- sym:39256 create_file_with_modified (tests/tests.rs:2588-2593)
  fn create_file_with_modified<P: AsRef<Path>>(path: P, duration_in_secs: u64)
- sym:38951 tokens_multiple (src/exec/mod.rs:386-403)
  fn tokens_multiple()
- sym:39258 test_modified_relative (tests/tests.rs:2608-2638)
  fn test_modified_relative()
- sym:39169 create_file_with_size (tests/tests.rs:56-60)
  fn create_file_with_size<P: AsRef<Path>>(path: P, size_in_bytes: usize)
- sym:39259 change_file_modified (tests/tests.rs:2641-2648)
  fn change_file_modified<P: AsRef<Path>>(path: P, iso_date: &str)
- sym:39184 test_multi_file (tests/tests.rs:316-336)
  fn test_multi_file()
- sym:39042 all_placeholders (src/fmt/mod.rs:237-280)
  fn all_placeholders()
- sym:39149 TestEnv.global_ignore_file (tests/testenv/mod.rs:169-176)
  pub fn global_ignore_file(self, content: &str) -> TestEnv
- sym:39138 search_str_no_base_dir_with_file_in_current_dir (src/walk.rs:770-775)
  fn search_str_no_base_dir_with_file_in_current_dir()
- sym:39255 test_size (tests/tests.rs:2510-2585)
  fn test_size()
- sym:39185 test_multi_file_with_missing (tests/tests.rs:340-375)
  fn test_multi_file_with_missing()
- sym:39050 Encoded.fmt (src/hyperlink.rs:72-77)
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result

---
tokens: 6315/8000
