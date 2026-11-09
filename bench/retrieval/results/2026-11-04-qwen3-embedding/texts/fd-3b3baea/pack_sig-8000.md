## Task
# fd-3b3baea · fd-3b3baea · open
Goal: refactor: Avoid depending on ignore implementation detail

This refactors how we handle ignore::Error in our walk code so that we
don't depend on the implementation detail of how the different Error
enum values are nested. It may also have a slight perfermance benefit
since we only walk the chain of errors once, instead of multiple times.

Related to #2039
Workspace: /root/bench/ws/rebf815-fd-3b3baea/fd-3b3baea
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
- sym:39525 WorkerResult (src/walk.rs:39-44)
  pub enum WorkerResult
- sym:39550 WorkerState.build_walker (src/walk.rs:346-403)
  fn build_walker(&self, paths: &[PathBuf]) -> Result<WalkParallel>
- sym:39554 is_broken_symlink (src/walk.rs:661-668)
  fn is_broken_symlink(path: &Path, err: &ignore::Error) -> bool
- sym:39309 DirEntryInner (src/dir_entry.rs:12-17)
  enum DirEntryInner
- sym:39552 WorkerState.spawn_senders (src/walk.rs:442-613)
  fn spawn_senders(&self, walker: WalkParallel, tx: Sender<Batch>)
- sym:39307 Config (src/config.rs:14-136)
  pub struct Config
- sym:39347 handle_cmd_error (src/exec/command.rs:100-114)
  pub fn handle_cmd_error(cmd: Option<&Command>, err: io::Error) -> ExitCode
- sym:39356 CommandSet.execute_batch (src/exec/mod.rs:90-120)
  pub fn execute_batch<I>(&self, paths: I, limit: usize, path_separator: Option<&str>) -> ExitCode where I: Iterator<Item = PathBuf>,
- sym:39551 WorkerState.receive (src/walk.rs:407-439)
  fn receive(&self, rx: Receiver<Batch>) -> ExitCode
- sym:39346 execute_commands (src/exec/command.rs:59-98)
  pub fn execute_commands<I: Iterator<Item = io::Result<Command>>>( cmds: I, mut output_buffer: OutputBuffer, enable_output_buffering: bool, ) -> ExitCode
- sym:39383 ExitCode (src/exit_codes.rs:7-12)
  pub enum ExitCode
- sym:39543 ReceiverBuffer.print (src/walk.rs:251-266)
  fn print(&mut self, entry: &DirEntry) -> Result<(), ExitCode>
- sym:39546 ReceiverBuffer.flush (src/walk.rs:295-301)
  fn flush(&mut self) -> Result<(), ExitCode>
- sym:39411 Check (src/filter/owner.rs:12-16)
  enum Check<T>
- sym:39524 ReceiverMode (src/walk.rs:27-34)
  enum ReceiverMode
- sym:39542 ReceiverBuffer.poll (src/walk.rs:197-248)
  fn poll(&mut self) -> Result<(), ExitCode>
- sym:39303 Exec.augment_args (src/cli.rs:886-960)
  fn augment_args(cmd: Command) -> Command
- sym:39545 ReceiverBuffer.stop (src/walk.rs:281-292)
  fn stop(&mut self) -> Result<(), ExitCode>
- sym:39387 merge_exitcodes (src/exit_codes.rs:46-51)
  pub fn merge_exitcodes(results: impl IntoIterator<Item = ExitCode>) -> ExitCode
- sym:39414 OwnerFilter.filter_ignore (src/filter/owner.rs:61-67)
  pub fn filter_ignore(self) -> Option<Self>
- sym:39409 FileTypes.should_ignore (src/filetypes.rs:21-42)
  pub fn should_ignore(&self, entry: &dir_entry::DirEntry) -> bool
- sym:39478 main (src/main.rs:65-78)
  fn main()
- sym:39284 Opts (src/cli.rs:31-694)
  pub struct Opts
- sym:39556 scan (src/walk.rs:699-701)
  pub fn scan(paths: &[PathBuf], patterns: Vec<Regex>, config: Config) -> Result<ExitCode>
- sym:39544 ReceiverBuffer.stream (src/walk.rs:269-278)
  fn stream(&mut self) -> Result<(), ExitCode>
- sym:39540 ReceiverBuffer.process (src/walk.rs:173-180)
  fn process(&mut self) -> ExitCode
- sym:39348 job (src/exec/job.rs:10-43)
  pub fn job( results: impl IntoIterator<Item = WorkerResult>, cmd: &CommandSet, config: &Config, ) -> ExitCode
- sym:39549 WorkerState.build_overrides (src/walk.rs:329-344)
  fn build_overrides(&self, paths: &[PathBuf]) -> Result<Override>
- sym:39553 WorkerState.scan (src/walk.rs:616-652)
  fn scan(&self, paths: &[PathBuf]) -> Result<ExitCode>
- sym:39384 i32.from (src/exit_codes.rs:15-22)
  fn from(code: ExitCode) -> Self
- sym:39385 ExitCode.is_error (src/exit_codes.rs:26-28)
  fn is_error(self) -> bool
- sym:39288 Opts.rg_alias_ignore (src/cli.rs:746-748)
  pub fn rg_alias_ignore(&self) -> bool
- sym:39535 BatchSender.send (src/walk.rs:100-120)
  fn send(&mut self, item: WorkerResult) -> Result<(), SendError<()>>
- sym:39541 ReceiverBuffer.recv (src/walk.rs:183-194)
  fn recv(&self) -> Result<Batch, RecvTimeoutError>
- sym:39335 Error.Sanitized (src/error.rs:29-29)
  type Sanitized = SanitizedError<io::Error>
- sym:39419 SizeFilter (src/filter/size.rs:9-13)
  pub enum SizeFilter
- sym:39289 Opts.max_depth (src/cli.rs:750-752)
  pub fn max_depth(&self) -> Option<usize>
- sym:39291 Opts.threads (src/cli.rs:758-760)
  pub fn threads(&self) -> NonZeroUsize
- sym:39313 DirEntry.path (src/dir_entry.rs:44-49)
  pub fn path(&self) -> &Path
- sym:39317 DirEntry.file_type (src/dir_entry.rs:84-89)
  pub fn file_type(&self) -> Option<FileType>
- sym:39343 OutputBuffer.new (src/exec/command.rs:18-23)
  pub fn new(null_separator: bool) -> Self
- sym:39352 CommandSet.new (src/exec/mod.rs:36-49)
  pub fn new<I, T, S>(input: I) -> Result<CommandSet> where I: IntoIterator<Item = T>, T: IntoIterator<Item = S>, S: AsRef<str>,
- sym:39358 CommandBuilder.new (src/exec/mod.rs:136-162)
  fn new(template: &CommandTemplate, limit: usize) -> io::Result<Self>
- sym:39364 CommandTemplate.new (src/exec/mod.rs:220-256)
  fn new<I, S>(input: I, mode: ExecutionMode) -> Result<CommandTemplate> where I: IntoIterator<Item = S>, S: AsRef<str>,
- sym:39468 PathUrl.new (src/hyperlink.rs:8-10)
  pub(crate) fn new(path: &Path) -> Option<PathUrl>
- sym:39527 Batch.new (src/walk.rs:53-57)
  fn new() -> Self
- sym:39533 BatchSender.new (src/walk.rs:81-87)
  fn new(tx: Sender<Batch>, limit: usize) -> Self
- sym:39539 ReceiverBuffer.new (src/walk.rs:152-170)
  fn new(state: &'a WorkerState, rx: Receiver<Batch>, stdout: W) -> Self
- sym:39548 WorkerState.new (src/walk.rs:317-327)
  fn new(patterns: Vec<Regex>, config: Config) -> Self
- sym:39632 test_custom_ignore_precedence (tests/tests.rs:862-884)
  fn test_custom_ignore_precedence()
- sym:39633 test_respect_ignore_files (tests/tests.rs:888-940)
  fn test_respect_ignore_files()
- sym:39630 test_no_ignore_parent (tests/tests.rs:790-818)
  fn test_no_ignore_parent()
- sym:39636 test_custom_ignore_files (tests/tests.rs:977-992)
  fn test_custom_ignore_files()
- sym:39631 test_no_ignore_parent_inner_git (tests/tests.rs:822-858)
  fn test_no_ignore_parent_inner_git()
- sym:39629 test_gitignore_and_fdignore (tests/tests.rs:752-786)
  fn test_gitignore_and_fdignore()
- sym:39704 test_ignore_contain_precedence_over_depth_check (tests/tests.rs:3087-3102)
  fn test_ignore_contain_precedence_over_depth_check()
- sym:39703 test_ignore_contain (tests/tests.rs:3061-3084)
  fn test_ignore_contain()
- sym:39705 test_ignore_contain_precedence_over_root_check (tests/tests.rs:3105-3109)
  fn test_ignore_contain_precedence_over_root_check()
- sym:39634 test_no_ignore_vcs (tests/tests.rs:944-957)
  fn test_no_ignore_vcs()
- sym:39635 test_no_ignore_vcs_child_dir (tests/tests.rs:961-973)
  fn test_no_ignore_vcs_child_dir()
- sym:39638 test_global_ignore (tests/tests.rs:1015-1023)
  fn test_global_ignore()
- sym:39389 general_error_if_at_least_one_error (src/exit_codes.rs:63-84)
  fn general_error_if_at_least_one_error()
- sym:39390 success_if_no_error (src/exit_codes.rs:87-93)
  fn success_if_no_error()
- sym:39701 test_gitignore_parent (tests/tests.rs:3030-3040)
  fn test_gitignore_parent()
- sym:39628 test_no_ignore (tests/tests.rs:721-748)
  fn test_no_ignore()
- sym:39637 test_no_ignore_aliases (tests/tests.rs:996-1011)
  fn test_no_ignore_aliases()
- sym:39697 test_error_if_hidden_not_set_and_pattern_starts_with_dot (tests/tests.rs:2934-2943)
  fn test_error_if_hidden_not_set_and_pattern_starts_with_dot()
- sym:39639 test_no_global_ignore (tests/tests.rs:1049-1052)
  fn test_no_global_ignore(flag: &str, expected_output: &str)
- sym:39573 TestEnv.global_ignore_file (tests/testenv/mod.rs:169-176)
  pub fn global_ignore_file(self, content: &str) -> TestEnv
- sym:39700 test_git_dir (tests/tests.rs:2990-3027)
  fn test_git_dir()
- sym:39565 create_config_directory_with_global_ignore (tests/testenv/mod.rs:65-73)
  fn create_config_directory_with_global_ignore(ignore_file_content: &str) -> io::Result<TempDir>
- sym:39443 TestTime.new (src/filter/time.rs:73-76)
  fn new(time: Zoned) -> Self
- sym:39571 TestEnv.new (tests/testenv/mod.rs:148-158)
  pub fn new(directories: &[&'static str], files: &[&'static str]) -> TestEnv

---
tokens: 2874/8000
