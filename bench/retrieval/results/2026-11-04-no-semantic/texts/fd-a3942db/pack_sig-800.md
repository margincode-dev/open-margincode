## Task
# fd-a3942db · fd-a3942db-2 · open

## Map
- src/error.rs
- src/sanitize.rs
- src/cli.rs
- src/exec/command.rs
- src/exec/job.rs
- src/main.rs
- src/output.rs
- src/walk.rs
- src/config.rs
- src/dir_entry.rs
- src/exec/mod.rs
- src/exit_codes.rs
- repo map: .cargo/ .github/ .gitignore CHANGELOG.md CONTRIBUTING.md Cargo.lock Cargo.toml Cross.toml LICENSE-APACHE LICENSE-MIT Makefile README.md (59 files)

## Code
- sym:40159 print_error (src/error.rs:5-9)
  pub fn print_error(msg: impl Into<String>)
- sym:40328 maybe_sanitize (src/sanitize.rs:49-55)
  pub fn maybe_sanitize<'a>(s: &'a str, is_terminal: bool) -> Cow<'a, str>
- sym:40315 print_entry_colorized (src/output.rs:89-139)
- sym:40316 print_entry_uncolorized_base (src/output.rs:142-156)
- sym:40314 print_entry_format (src/output.rs:69-86)
- sym:40359 ReceiverBuffer.poll (src/walk.rs:198-249)
  fn poll(&mut self) -> Result<(), ExitCode>
- sym:40206 merge_exitcodes (src/exit_codes.rs:46-51)
- sym:40113 Opts.search_paths (src/cli.rs:696-721)
- sym:40166 handle_cmd_error (src/exec/command.rs:101-115)
- sym:40167 job (src/exec/job.rs:11-44)
- sym:40168 batch (src/exec/job.rs:46-64)
- sym:40297 main (src/main.rs:62-73)
  fn main()
- sym:40327 sanitize_for_terminal (src/sanitize.rs:28-46)
- sym:40360 ReceiverBuffer.print (src/walk.rs:252-267)
- sym:40367 WorkerState.build_walker (src/walk.rs:347-404)
- sym:40340 maybe_sanitize_passthrough_when_not_terminal (src/sanitize.rs:159-167)
- sym:40114 Opts.normalize_path (src/cli.rs:723-736)
- sym:40117 Opts.max_depth (src/cli.rs:747-749)

---
tokens: 709/800
omitted 25 lower-ranked items (expand an id for detail):
- sym:40119 Opts.threads (src/cli.rs:755-757)
- sym:40120 Opts.max_results (src/cli.rs:759-763)
- sym:40134 ensure_current_directory_exists (src/cli.rs:963-971)
- sym:40135 Config (src/config.rs:14-136)
- sym:40138 DirEntry (src/dir_entry.rs:18-22)
- sym:40141 DirEntry.path (src/dir_entry.rs:42-47)
- sym:40143 DirEntry.stripped_path (src/dir_entry.rs:59-71)
- sym:40144 DirEntry.into_stripped_path (src/dir_entry.rs:74-80)
- sym:40148 DirEntry.style (src/dir_entry.rs:105-109)
- … and 16 more
