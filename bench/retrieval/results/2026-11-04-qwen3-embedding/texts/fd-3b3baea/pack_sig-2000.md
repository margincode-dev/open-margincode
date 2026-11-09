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
- sym:39347 handle_cmd_error (src/exec/command.rs:100-114)
  pub fn handle_cmd_error(cmd: Option<&Command>, err: io::Error) -> ExitCode
- sym:39554 is_broken_symlink (src/walk.rs:661-668)
  fn is_broken_symlink(path: &Path, err: &ignore::Error) -> bool
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
- sym:39313 DirEntry.path (src/dir_entry.rs:44-49)
  pub fn path(&self) -> &Path
- sym:39343 OutputBuffer.new (src/exec/command.rs:18-23)
  pub fn new(null_separator: bool) -> Self
- sym:39344 OutputBuffer.push (src/exec/command.rs:25-27)
  fn push(&mut self, stdout: Vec<u8>, stderr: Vec<u8>)
- sym:39349 batch (src/exec/job.rs:45-63)
  pub fn batch( results: impl IntoIterator<Item = WorkerResult>, cmd: &CommandSet, config: &Config, ) -> ExitCode
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

---
tokens: 1111/2000
