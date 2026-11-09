## Task
# fd-3b3baea · fd-3b3baea · open

## Map
- repo map (60 files):
  .cargo/ (1 files)
  .github/ (7 files)
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
  doc/ (8 files)
  rustfmt.toml
  scripts/ (4 files)
  src/ (22 files)
  tests/ (2 files)

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
- sym:39313 DirEntry.path (src/dir_entry.rs:44-49)
  pub fn path(&self) -> &Path
- sym:39343 OutputBuffer.new (src/exec/command.rs:18-23)
- sym:39344 OutputBuffer.push (src/exec/command.rs:25-27)
- sym:39349 batch (src/exec/job.rs:45-63)
- sym:39632 test_custom_ignore_precedence (tests/tests.rs:862-884)
- sym:39633 test_respect_ignore_files (tests/tests.rs:888-940)
- sym:39630 test_no_ignore_parent (tests/tests.rs:790-818)
- sym:39636 test_custom_ignore_files (tests/tests.rs:977-992)
- sym:39631 test_no_ignore_parent_inner_git (tests/tests.rs:822-858)

---
tokens: 684/800
