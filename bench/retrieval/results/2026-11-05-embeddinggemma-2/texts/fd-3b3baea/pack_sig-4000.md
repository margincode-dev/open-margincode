## Task
# fd-3b3baea · fd-3b3baea · open
Goal: refactor: Avoid depending on ignore implementation detail

This refactors how we handle ignore::Error in our walk code so that we
don't depend on the implementation detail of how the different Error
enum values are nested. It may also have a slight perfermance benefit
since we only walk the chain of errors once, instead of multiple times.

Related to #2039
Workspace: /root/bench/av_gemma/ws/rec728b-fd-3b3baea/fd-3b3baea
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
- sym:39309 DirEntryInner (src/dir_entry.rs:12-17)
  enum DirEntryInner
- sym:39552 WorkerState.spawn_senders (src/walk.rs:442-613)
  fn spawn_senders(&self, walker: WalkParallel, tx: Sender<Batch>)
- sym:39411 Check (src/filter/owner.rs:12-16)
  enum Check<T>
- sym:39543 ReceiverBuffer.print (src/walk.rs:251-266)
  fn print(&mut self, entry: &DirEntry) -> Result<(), ExitCode>
- sym:39524 ReceiverMode (src/walk.rs:27-34)
  enum ReceiverMode
- sym:39347 handle_cmd_error (src/exec/command.rs:100-114)
  pub fn handle_cmd_error(cmd: Option<&Command>, err: io::Error) -> ExitCode
- sym:39356 CommandSet.execute_batch (src/exec/mod.rs:90-120)
  pub fn execute_batch<I>(&self, paths: I, limit: usize, path_separator: Option<&str>) -> ExitCode where I: Iterator<Item = PathBuf>,
- sym:39414 OwnerFilter.filter_ignore (src/filter/owner.rs:61-67)
  pub fn filter_ignore(self) -> Option<Self>
- sym:39549 WorkerState.build_overrides (src/walk.rs:329-344)
  fn build_overrides(&self, paths: &[PathBuf]) -> Result<Override>
- sym:39551 WorkerState.receive (src/walk.rs:407-439)
  fn receive(&self, rx: Receiver<Batch>) -> ExitCode
- sym:39346 execute_commands (src/exec/command.rs:59-98)
  pub fn execute_commands<I: Iterator<Item = io::Result<Command>>>( cmds: I, mut output_buffer: OutputBuffer, enable_output_buffering: bool, ) -> ExitCode
- sym:39383 ExitCode (src/exit_codes.rs:7-12)
  pub enum ExitCode
- sym:39555 search_str_for_entry (src/walk.rs:670-692)
  fn search_str_for_entry<'a>( entry_path: &'a std::path::Path, full_path_base: Option<&std::path::Path>, ) -> Cow<'a, OsStr>
- sym:39530 Batch.IntoIter (src/walk.rs:66-66)
  type IntoIter = std::vec::IntoIter<WorkerResult>
- sym:39412 OwnerFilter.IGNORE (src/filter/owner.rs:19-22)
  const IGNORE: Self = OwnerFilter {
- sym:39554 is_broken_symlink (src/walk.rs:661-668)
  fn is_broken_symlink(path: &Path, err: &ignore::Error) -> bool
- sym:39550 WorkerState.build_walker (src/walk.rs:346-403)
  fn build_walker(&self, paths: &[PathBuf]) -> Result<WalkParallel>
- sym:39546 ReceiverBuffer.flush (src/walk.rs:295-301)
  fn flush(&mut self) -> Result<(), ExitCode>
- sym:39553 WorkerState.scan (src/walk.rs:616-652)
  fn scan(&self, paths: &[PathBuf]) -> Result<ExitCode>
- sym:39417 Check.parse (src/filter/owner.rs:85-102)
  fn parse<F>(s: Option<&str>, f: F) -> Result<Self> where F: Fn(&str) -> Result<T>,
- sym:39548 WorkerState.new (src/walk.rs:317-327)
  fn new(patterns: Vec<Regex>, config: Config) -> Self
- sym:39542 ReceiverBuffer.poll (src/walk.rs:197-248)
  fn poll(&mut self) -> Result<(), ExitCode>
- sym:39290 Opts.min_depth (src/cli.rs:754-756)
  pub fn min_depth(&self) -> Option<usize>
- sym:39291 Opts.threads (src/cli.rs:758-760)
  pub fn threads(&self) -> NonZeroUsize
- sym:39308 Config.is_printing (src/config.rs:140-142)
  pub fn is_printing(&self) -> bool
- sym:39310 DirEntry (src/dir_entry.rs:20-24)
  pub struct DirEntry
- sym:39311 DirEntry.normal (src/dir_entry.rs:28-34)
  pub fn normal(e: ignore::DirEntry) -> Self
- sym:39312 DirEntry.broken_symlink (src/dir_entry.rs:36-42)
  pub fn broken_symlink(path: PathBuf, depth: Option<usize>) -> Self
- sym:39313 DirEntry.path (src/dir_entry.rs:44-49)
  pub fn path(&self) -> &Path
- sym:39317 DirEntry.file_type (src/dir_entry.rs:84-89)
  pub fn file_type(&self) -> Option<FileType>
- sym:39632 test_custom_ignore_precedence (tests/tests.rs:862-884)
  fn test_custom_ignore_precedence()
- sym:39389 general_error_if_at_least_one_error (src/exit_codes.rs:63-84)
  fn general_error_if_at_least_one_error()
- sym:39633 test_respect_ignore_files (tests/tests.rs:888-940)
  fn test_respect_ignore_files()

---
tokens: 1629/4000
