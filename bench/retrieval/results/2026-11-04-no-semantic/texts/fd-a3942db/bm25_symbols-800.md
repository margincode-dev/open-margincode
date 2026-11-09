- print_error (src/error.rs:5-9)
```rust
pub fn print_error(msg: impl Into<String>) {
    let msg = msg.into();
    let safe = maybe_sanitize(&msg, std::io::stderr().is_terminal());
    eprintln!("[fd error]: {safe}");
}
```
- handle_cmd_error (src/exec/command.rs:101-115)
```rust
pub fn handle_cmd_error(cmd: Option<&Command>, err: io::Error) -> ExitCode {
    match (cmd, err) {
        (Some(cmd), err) if err.kind() == io::ErrorKind::NotFound => {
            print_error(format!(
                "Command not found: {}",
                cmd.get_program().to_string_lossy()
            ));
            ExitCode::GeneralError
        }
        (_, err) => {
            print_error(format!("Problem while executing command: {err}"));
            ExitCode::GeneralError
        }
    }
}
```
- batch (src/exec/job.rs:46-64)
```rust
pub fn batch(
    results: impl IntoIterator<Item = WorkerResult>,
    cmd: &CommandSet,
    config: &Config,
) -> ExitCode {
    let paths = results
        .into_iter()
        .filter_map(|worker_result| match worker_result {
            WorkerResult::Entry(dir_entry) => Some(dir_entry.into_stripped_path(config)),
            WorkerResult::Error(err) => {
                if config.show_filesystem_errors {
                    print_error(err.to_string());
                }
                None
            }
        });

    cmd.execute_batch(paths, config.batch_size, config.path_separator.as_deref())
}
```
- ReceiverBuffer.print (src/walk.rs:252-267)
```rust
    fn print(&mut self, entry: &DirEntry) -> Result<(), ExitCode> {
        if let Err(e) = output::print_entry(&mut self.stdout, entry, self.config)
            && e.kind() != ::std::io::ErrorKind::BrokenPipe
        {
            print_error(format!("Could not write to output: {e}"));
            return Err(ExitCode::GeneralError);
        }

        if self.interrupt_flag.load(Ordering::Relaxed) {
            // Ignore any errors on flush, because we're about to exit anyway
            let _ = self.flush();
            return Err(ExitCode::KilledBySigint);
        }

        Ok(())
    }
```
- maybe_sanitize_passthrough_when_not_terminal (src/sanitize.rs:159-167)
```rust
    fn maybe_sanitize_passthrough_when_not_terminal() {
        let attack = "x\x1by";
        // Pipe context: bytes pass through unchanged (zero-copy).
        let out = maybe_sanitize(attack, false);
        assert!(matches!(out, Cow::Borrowed(_)));
        assert_eq!(out, attack);
        // TTY context: escapes apply.
        assert_eq!(maybe_sanitize(attack, true), "x\\x1By");
    }
```
- maybe_sanitize (src/sanitize.rs:49-55)
```rust
pub fn maybe_sanitize<'a>(s: &'a str, is_terminal: bool) -> Cow<'a, str> {
    if is_terminal {
        sanitize_for_terminal(s)
    } else {
        Cow::Borrowed(s)
    }
}
```
- print_entry_format (src/output.rs:69-86)
```rust
fn print_entry_format<W: Write>(
    stdout: &mut W,
    entry: &DirEntry,
    config: &Config,
    format: &FormatTemplate,
) -> io::Result<()> {
    let output = format.generate(
        entry.stripped_path(config),
        config.path_separator.as_deref(),
    );
    // TODO: support writing raw bytes on unix?
