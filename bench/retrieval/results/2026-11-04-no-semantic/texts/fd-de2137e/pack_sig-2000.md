## Task
# fd-de2137e · fd-de2137e-2 · open
Goal: refactor(filetypes): extract FileTypes construction into impl block
Workspace: /root/bench/ws/rebf815-fd-de2137e/fd-de2137e-2
Changed (0): nothing yet

## Map
- src/filetypes.rs
  L8: pub struct FileTypes
  L21: pub fn should_ignore(&self, entry: &dir_entry::DirEntry) -> bool
- src/config.rs
  L14: pub struct Config
  L140: pub fn is_printing(&self) -> bool
- src/main.rs
- src/cli.rs
- src/dir_entry.rs
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
- sym:38985 FileTypes.should_ignore (src/filetypes.rs:21-42)
  pub fn should_ignore(&self, entry: &dir_entry::DirEntry) -> bool
- sym:38971 is_block_device (src/filesystem.rs:63-65)
  pub fn is_block_device(ft: fs::FileType) -> bool
- sym:38902 DirEntry.file_name (src/dir_entry.rs:146-160)
  fn file_name(&self) -> OsString
- sym:39063 extract_command (src/main.rs:400-415)
  fn extract_command(opts: &mut Opts, colored_output: bool) -> Result<Option<CommandSet>>
- sym:39065 extract_time_constraints (src/main.rs:501-524)
  fn extract_time_constraints(opts: &Opts) -> Result<Vec<TimeFilter>>
- sym:38963 merge_exitcodes (src/exit_codes.rs:46-51)
  pub fn merge_exitcodes(results: impl IntoIterator<Item = ExitCode>) -> ExitCode
- sym:38883 Config (src/config.rs:14-136)
  pub struct Config
- sym:39062 construct_config (src/main.rs:253-398)
  fn construct_config(mut opts: Opts, pattern_regexps: &[String]) -> Result<Config>
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
- sym:38924 job (src/exec/job.rs:10-43)
- sym:38925 batch (src/exec/job.rs:45-63)
- sym:38927 CommandSet (src/exec/mod.rs:30-33)
  pub struct CommandSet
- sym:38928 CommandSet.new (src/exec/mod.rs:36-49)
- sym:38934 CommandBuilder.new (src/exec/mod.rs:136-162)
  fn new(template: &CommandTemplate, limit: usize) -> io::Result<Self>
- sym:38940 CommandTemplate.new (src/exec/mod.rs:220-256)
- sym:38960 i32.from (src/exit_codes.rs:15-22)
- sym:38970 is_empty (src/filesystem.rs:44-60)
- sym:38982 default_path_separator (src/filesystem.rs:128-136)
- sym:38986 OwnerFilter (src/filter/owner.rs:6-9)
- sym:38989 OwnerFilter.from_string (src/filter/owner.rs:27-58)
- sym:38990 OwnerFilter.filter_ignore (src/filter/owner.rs:61-67)
- sym:38993 Check.parse (src/filter/owner.rs:85-102)
- sym:38995 SizeFilter (src/filter/size.rs:9-13)
- sym:39004 SizeFilter.from_string (src/filter/size.rs:28-31)
- sym:39011 TimeFilter (src/filter/time.rs:7-10)
- sym:39032 Token.fmt (src/fmt/mod.rs:28-38)
- sym:39033 FormatTemplate (src/fmt/mod.rs:46-49)
- sym:39036 FormatTemplate.parse (src/fmt/mod.rs:58-107)
- sym:39044 PathUrl.new (src/hyperlink.rs:8-10)
- sym:39045 PathUrl.fmt (src/hyperlink.rs:14-21)
- sym:39019 TestTime.new (src/filter/time.rs:73-76)
- sym:39050 Encoded.fmt (src/hyperlink.rs:72-77)

---
tokens: 1797/2000
