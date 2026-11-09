- WorkerResult (src/walk.rs:39-44)
```rust
pub enum WorkerResult {
    // Errors should be rare, so it's probably better to allow large_enum_variant than
    // to box the Entry variant
    Entry(DirEntry),
    Error(ignore::Error),
}
```
- DirEntryInner (src/dir_entry.rs:12-17)
```rust
enum DirEntryInner {
    Normal(ignore::DirEntry),
    // Broken symlinks reach us as walk errors rather than entries, so we carry
    // over the depth the walker recorded on the error.
    BrokenSymlink { path: PathBuf, depth: Option<usize> },
}
```
- ReceiverBuffer.print (src/walk.rs:251-266)
```rust
    fn print(&mut self, entry: &DirEntry) -> Result<(), ExitCode> {
        if let Err(e) = output::print_entry(&mut self.stdout, entry, self.config)
            && e.kind() != ::std::io::ErrorKind::BrokenPipe
        {
            print_error!("Could not write to output: {}", e);
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
- handle_cmd_error (src/exec/command.rs:100-114)
```rust
pub fn handle_cmd_error(cmd: Option<&Command>, err: io::Error) -> ExitCode {
    match (cmd, err) {
        (Some(cmd), err) if err.kind() == io::ErrorKind::NotFound => {
            print_error!(
                "Command not found: {:?}",
                cmd.get_program().to_string_lossy().as_ref()
            );
            ExitCode::GeneralError
        }
        (_, err) => {
            print_error!("Problem while executing command: {}", err);
            ExitCode::GeneralError
        }
    }
}
```
- encode (src/hyperlink.rs:24-41)
```rust
fn encode(f: &mut Formatter, byte: u8) -> fmt::Result {
    // NOTE:
    // Most terminals can handle non-ascii unicode characters in a file url fine. But on some OSes (notably
    // windows), the encoded bytes of the path may not be valid UTF-8. Since we don't know if a
    // byte >= 128 is part of a valid UTF-8 encoding or not, we just percent encode any non-ascii
    // byte.
    // Percent encoding these bytes is probably safer anyway.
    match byte {
        b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'/' | b':' | b'-' | b'.' | b'_' | b'~' => {
            f.write_char(byte.into())
        }
        #[cfg(windows)]
        b'\\' => f.write_char('/'),
        _ => {
            write!(f, "%{byte:02X}")
        }
    }
}
```
- WorkerState.spawn_senders (src/walk.rs:442-613)
```rust
    fn spawn_senders(&self, walker: WalkParallel, tx: Sender<Batch>) {
        walker.run(|| {
            let patterns = &self.patterns;
            let config = &self.config;
            let quit_flag = self.quit_flag.as_ref();

            let mut limit = 0x100;
            if let Some(cmd) = &config.command
                && !cmd.in_batch_mode()
                && config.threads > 1
            {
                // Evenly distribute work between multiple receivers
                limit = 1;
            }
            let mut tx = BatchSender::new(tx.clone(), limit);

            Box::new(move |entry| {
                if quit_flag.load(Ordering::Relaxed) {
                    return WalkState::Quit;
                }

                if let Ok(e) = &entry {
                    // If the entry is a directory that contains a
                    // "ignore contain" file", we want to skip this
                    // directory.
                    // Check the filetype first to avoid unnecessary
                    // syscalls.
                    if e.file_type().is_some_and(|t| t.is_dir()) {
                        let entry_path = e.path();
                        if config
                            .ignore_contain
                            .iter()
                            .any(|ic| entry_path.join(ic).exists())
                        {
                            return WalkState::Skip;
                        }
                    }
                    if e.depth() == 0 {
                        // Skip the root directory entry.
                        return WalkState::Continue;
                    }
                }
                let entry = match entry {
                    Ok(e) => DirEntry::normal(e),
                    Err(err) => {
                        // The depth has to be read off the error before it is
                        // taken apart, since it is recorded on an inner variant.
                        let depth = err.depth();
                        match err {
                            ignore::Error::WithPath {
                                path,
                                err: inner_err,
                            } if is_broken_symlink(&path, &inner_err) => {
                                DirEntry::broken_symlink(path, depth)
                            }
                            err => {
                                return match tx.send(WorkerResult::Error(err)) {
                                    Ok(_) => WalkState::Continue,
                                    Err(_) => WalkState::Quit,
                                };
                            }
                        }
                    }
                };

                if let Some(min_depth) = config.min_depth
                    && entry.depth().is_none_or(|d| d < min_depth)
                {
                    return WalkState::Continue;
                }

                // Check the name first, since it doesn't require metadata
                let entry_path = entry.path();

                let search_str = search_str_for_entry(entry_path, config.full_path_base.as_deref());

                if !patterns
                    .iter()
                    .all(|pat| pat.is_match(&filesystem::osstr_to_bytes(search_str.as_ref())))
                {
                    return WalkState::Continue;
                }

                // Filter out unwanted extensions.
                if let Some(ref exts_regex) = config.extensions {
                    if let Some(path_str) = entry_path.file_name() {
                        if !exts_regex.is_match(&filesystem::osstr_to_bytes(path_str)) {
                            return WalkState::Continue;
                        }
                    } else {
                        return WalkState::Continue;
                    }
                }

                // Filter out unwanted file types.
                if let Some(ref file_types) = config.file_types
                    && file_types.should_ignore(&entry)
                {
                    return WalkState::Continue;
                }

                #[cfg(unix)]
                {
                    if let Some(ref owner_constraint) = config.owner_constraint {
                        if let Some(metadata) = entry.metadata() {
                            if !owner_constraint.matches(metadata) {
                                return WalkState::Continue;
                            }
                        } else {
                            return WalkState::Continue;
                        }
                    }
                }

                // Filter out unwanted sizes if it is a file and we have been given size constraints.
                if !config.size_constraints.is_empty() {
                    if entry_path.is_file() {
                        if let Some(metadata) = entry.metadata() {
                            let file_size = metadata.len();
                            if config
                                .size_constraints
                                .iter()
                                .any(|sc| !sc.is_within(file_size))
                            {
                                return WalkState::Continue;
                            }
                        } else {
                            return WalkState::Continue;
                        }
                    } else {
                        return WalkState::Continue;
                    }
                }

                // Filter out unwanted modification times
                if !config.time_constraints.is_empty() {
                    let mut matched = false;
                    if let Some(metadata) = entry.metadata()
                        && let Ok(modified) = metadata.modified()
                    {
                        matched = config
                            .time_constraints
                            .iter()
                            .all(|tf| tf.applies_to(&modified));
                    }
                    if !matched {
                        return WalkState::Continue;
                    }
                }

                if config.is_printing()
                    && let Some(ls_colors) = &config.ls_colors
                {
                    // Compute colors in parallel
                    entry.style(ls_colors);
                }

                let send_result = tx.send(WorkerResult::Entry(entry));

                if send_result.is_err() {
                    return WalkState::Quit;
                }

                // Apply pruning.
                if config.prune {
                    return WalkState::Skip;
                }

                WalkState::Continue
            })
        });
    }
```
- test_git_dir (tests/tests.rs:2990-3027)
```rust
fn test_git_dir() {
    let te = TestEnv::new(
        &[".git/one", "other_dir/.git", "nested/dir/.git"],
        &[
            ".git/one/foo.a",
            ".git/.foo",
            ".git/a.foo",
            "other_dir/.git/foo1",
            "nested/dir/.git/foo2",
        ],
    );

    te.assert_output(
        &["--hidden", "foo"],
        ".git/one/foo.a
        .git/.foo
        .git/a.foo
        other_dir/.git/foo1
        nested/dir/.git/foo2",
    );
    te.assert_output(&["--no-ignore", "foo"], "");
    te.assert_output(
        &["--hidden", "--no-ignore", "foo"],
        ".git/one/foo.a
         .git/.foo
         .git/a.foo
         other_dir/.git/foo1
         nested/dir/.git/foo2",
    );
