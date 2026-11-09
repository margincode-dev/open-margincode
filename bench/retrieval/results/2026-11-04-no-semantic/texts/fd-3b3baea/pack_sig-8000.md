## Task
# fd-3b3baea · fd-3b3baea-2 · open
Goal: refactor: Avoid depending on ignore implementation detail

This refactors how we handle ignore::Error in our walk code so that we
don't depend on the implementation detail of how the different Error
enum values are nested. It may also have a slight perfermance benefit
since we only walk the chain of errors once, instead of multiple times.

Related to #2039
Workspace: /root/bench/ws/rebf815-fd-3b3baea/fd-3b3baea-2
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
- sym:39347 handle_cmd_error (src/exec/command.rs:100-114)
  pub fn handle_cmd_error(cmd: Option<&Command>, err: io::Error) -> ExitCode
- sym:39309 DirEntryInner (src/dir_entry.rs:12-17)
  enum DirEntryInner
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
- sym:39552 WorkerState.spawn_senders (src/walk.rs:442-613)
  fn spawn_senders(&self, walker: WalkParallel, tx: Sender<Batch>)
- sym:39554 is_broken_symlink (src/walk.rs:661-668)
  fn is_broken_symlink(path: &Path, err: &ignore::Error) -> bool
- sym:39550 WorkerState.build_walker (src/walk.rs:346-403)
  fn build_walker(&self, paths: &[PathBuf]) -> Result<WalkParallel>
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
- sym:39478 main (src/main.rs:65-78)
  fn main()
- sym:39556 scan (src/walk.rs:699-701)
  pub fn scan(paths: &[PathBuf], patterns: Vec<Regex>, config: Config) -> Result<ExitCode>
- sym:39544 ReceiverBuffer.stream (src/walk.rs:269-278)
  fn stream(&mut self) -> Result<(), ExitCode>
- sym:39540 ReceiverBuffer.process (src/walk.rs:173-180)
  fn process(&mut self) -> ExitCode
- sym:39348 job (src/exec/job.rs:10-43)
  pub fn job( results: impl IntoIterator<Item = WorkerResult>, cmd: &CommandSet, config: &Config, ) -> ExitCode
- sym:39553 WorkerState.scan (src/walk.rs:616-652)
  fn scan(&self, paths: &[PathBuf]) -> Result<ExitCode>
- sym:39384 i32.from (src/exit_codes.rs:15-22)
  fn from(code: ExitCode) -> Self
- sym:39385 ExitCode.is_error (src/exit_codes.rs:26-28)
  fn is_error(self) -> bool
- sym:39307 Config (src/config.rs:14-136)
  pub struct Config
- sym:39535 BatchSender.send (src/walk.rs:100-120)
  fn send(&mut self, item: WorkerResult) -> Result<(), SendError<()>>
- sym:39541 ReceiverBuffer.recv (src/walk.rs:183-194)
  fn recv(&self) -> Result<Batch, RecvTimeoutError>
- sym:39419 SizeFilter (src/filter/size.rs:9-13)
  pub enum SizeFilter
- sym:39313 DirEntry.path (src/dir_entry.rs:44-49)
  pub fn path(&self) -> &Path
- sym:39343 OutputBuffer.new (src/exec/command.rs:18-23)
  pub fn new(null_separator: bool) -> Self
- sym:39344 OutputBuffer.push (src/exec/command.rs:25-27)
  fn push(&mut self, stdout: Vec<u8>, stderr: Vec<u8>)
- sym:39349 batch (src/exec/job.rs:45-63)
  pub fn batch( results: impl IntoIterator<Item = WorkerResult>, cmd: &CommandSet, config: &Config, ) -> ExitCode
- sym:39352 CommandSet.new (src/exec/mod.rs:36-49)
  pub fn new<I, T, S>(input: I) -> Result<CommandSet> where I: IntoIterator<Item = T>, T: IntoIterator<Item = S>, S: AsRef<str>,
- sym:39357 CommandBuilder (src/exec/mod.rs:125-133)
  struct CommandBuilder
- sym:39358 CommandBuilder.new (src/exec/mod.rs:136-162)
  fn new(template: &CommandTemplate, limit: usize) -> io::Result<Self>
- sym:39360 CommandBuilder.push (src/exec/mod.rs:173-189)
  fn push(&mut self, path: &Path, separator: Option<&str>) -> io::Result<()>
- sym:39361 CommandBuilder.finish (src/exec/mod.rs:191-203)
  fn finish(&mut self) -> io::Result<()>
- sym:39362 CommandBuilder.exit_code (src/exec/mod.rs:205-207)
  fn exit_code(&self) -> ExitCode
- sym:39364 CommandTemplate.new (src/exec/mod.rs:220-256)
  fn new<I, S>(input: I, mode: ExecutionMode) -> Result<CommandTemplate> where I: IntoIterator<Item = S>, S: AsRef<str>,
- sym:39468 PathUrl.new (src/hyperlink.rs:8-10)
  pub(crate) fn new(path: &Path) -> Option<PathUrl>
- sym:39527 Batch.new (src/walk.rs:53-57)
  fn new() -> Self
- sym:39529 Batch.Item (src/walk.rs:65-65)
  type Item = WorkerResult
- sym:39533 BatchSender.new (src/walk.rs:81-87)
  fn new(tx: Sender<Batch>, limit: usize) -> Self
- sym:39389 general_error_if_at_least_one_error (src/exit_codes.rs:63-84)
  fn general_error_if_at_least_one_error()
- sym:39390 success_if_no_error (src/exit_codes.rs:87-93)
  fn success_if_no_error()
- sym:39443 TestTime.new (src/filter/time.rs:73-76)
  fn new(time: Zoned) -> Self

---
tokens: 2089/8000
