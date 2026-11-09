## Task
# fd-3b3baea · fd-3b3baea-2 · open
Goal: refactor: Avoid depending on ignore implementation detail

This refactors how we handle ignore::Error in our walk code so that we
don't depend on the implementation detail of how the different Error
enum values are nested. It may also have a slight perfermance benefit
since we only walk the chain of errors once, instead of multiple times.

Related to #2039

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
- sym:39313 DirEntry.path (src/dir_entry.rs:44-49)
  pub fn path(&self) -> &Path
- sym:39343 OutputBuffer.new (src/exec/command.rs:18-23)
  pub fn new(null_separator: bool) -> Self
- sym:39344 OutputBuffer.push (src/exec/command.rs:25-27)
  fn push(&mut self, stdout: Vec<u8>, stderr: Vec<u8>)
- sym:39349 batch (src/exec/job.rs:45-63)
  pub fn batch( results: impl IntoIterator<Item = WorkerResult>, cmd: &CommandSet, config: &Config, ) -> ExitCode

---
tokens: 669/800
