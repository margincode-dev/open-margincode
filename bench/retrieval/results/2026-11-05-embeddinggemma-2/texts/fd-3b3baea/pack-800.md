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
```rust
pub enum WorkerResult {
    // Errors should be rare, so it's probably better to allow large_enum_variant than
    // to box the Entry variant
    Entry(DirEntry),
    Error(ignore::Error),
}
```
- sym:39309 DirEntryInner (src/dir_entry.rs:12-17)
```rust
enum DirEntryInner {
    Normal(ignore::DirEntry),
    // Broken symlinks reach us as walk errors rather than entries, so we carry
    // over the depth the walker recorded on the error.
    BrokenSymlink { path: PathBuf, depth: Option<usize> },
}
```
- sym:39347 handle_cmd_error (src/exec/command.rs:100-114)
  pub fn handle_cmd_error(cmd: Option<&Command>, err: io::Error) -> ExitCode
- sym:39552 WorkerState.spawn_senders (src/walk.rs:442-613)
  fn spawn_senders(&self, walker: WalkParallel, tx: Sender<Batch>)
- sym:39356 CommandSet.execute_batch (src/exec/mod.rs:90-120)
  pub fn execute_batch<I>(&self, paths: I, limit: usize, path_separator: Option<&str>) -> ExitCode where I: Iterator<Item = PathBuf>,
- sym:39414 OwnerFilter.filter_ignore (src/filter/owner.rs:61-67)
  pub fn filter_ignore(self) -> Option<Self>
- sym:39549 WorkerState.build_overrides (src/walk.rs:329-344)
- sym:39551 WorkerState.receive (src/walk.rs:407-439)
- sym:39346 execute_commands (src/exec/command.rs:59-98)
- sym:39411 Check (src/filter/owner.rs:12-16)
- sym:39383 ExitCode (src/exit_codes.rs:7-12)
- sym:39524 ReceiverMode (src/walk.rs:27-34)
- sym:39543 ReceiverBuffer.print (src/walk.rs:251-266)
- sym:39555 search_str_for_entry (src/walk.rs:670-692)
- sym:39313 DirEntry.path (src/dir_entry.rs:44-49)
- sym:39343 OutputBuffer.new (src/exec/command.rs:18-23)
- sym:39344 OutputBuffer.push (src/exec/command.rs:25-27)
- sym:39349 batch (src/exec/job.rs:45-63)

---
tokens: 681/800
