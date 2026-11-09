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
    te.assert_output(
        &["--hidden", "--no-ignore-vcs", "foo"],
        ".git/one/foo.a
         .git/.foo
         .git/a.foo
         other_dir/.git/foo1
         nested/dir/.git/foo2",
    );
}
```
- CommandSet.execute_batch (src/exec/mod.rs:90-120)
```rust
    pub fn execute_batch<I>(&self, paths: I, limit: usize, path_separator: Option<&str>) -> ExitCode
    where
        I: Iterator<Item = PathBuf>,
    {
        let builders: io::Result<Vec<_>> = self
            .commands
            .iter()
            .map(|c| CommandBuilder::new(c, limit))
            .collect();

        match builders {
            Ok(mut builders) => {
                for path in paths {
                    for builder in &mut builders {
                        if let Err(e) = builder.push(&path, path_separator) {
                            return handle_cmd_error(Some(&builder.cmd), e);
                        }
                    }
                }

                for builder in &mut builders {
                    if let Err(e) = builder.finish() {
                        return handle_cmd_error(Some(&builder.cmd), e);
                    }
                }

                merge_exitcodes(builders.iter().map(|b| b.exit_code()))
            }
            Err(e) => handle_cmd_error(None, e),
        }
    }
```
- main (src/main.rs:65-78)
```rust
fn main() {
    let result = run();
    match result {
        Ok(exit_code) => {
            exit_code.exit();
        }
        Err(err) => {
            // NB: we use eprintln directly instead of print_error!() because
            // we sanitize anyhow errors at the generation site
            eprintln!("[fd error]: {:#}", err);
            ExitCode::GeneralError.exit();
        }
    }
}
```
- ExitCode (src/exit_codes.rs:7-12)
```rust
pub enum ExitCode {
    Success,
    HasResults(bool),
    GeneralError,
    KilledBySigint,
}
```
- execute_commands (src/exec/command.rs:59-98)
```rust
pub fn execute_commands<I: Iterator<Item = io::Result<Command>>>(
    cmds: I,
    mut output_buffer: OutputBuffer,
    enable_output_buffering: bool,
) -> ExitCode {
    for result in cmds {
        let mut cmd = match result {
            Ok(cmd) => cmd,
            Err(e) => return handle_cmd_error(None, e),
        };

        // Spawn the supplied command.
        let output = if enable_output_buffering {
            cmd.output()
        } else {
            // If running on only one thread, don't buffer output
            // Allows for viewing and interacting with intermediate command output
            cmd.spawn().and_then(|c| c.wait_with_output())
        };

        // Then wait for the command to exit, if it was spawned.
        match output {
            Ok(output) => {
                if enable_output_buffering {
                    output_buffer.push(output.stdout, output.stderr);
                }
                if output.status.code() != Some(0) {
                    output_buffer.write();
                    return ExitCode::GeneralError;
                }
            }
            Err(why) => {
                output_buffer.write();
                return handle_cmd_error(Some(&cmd), why);
            }
        }
    }
    output_buffer.write();
    ExitCode::Success
}
```
- WorkerState.receive (src/walk.rs:407-439)
```rust
    fn receive(&self, rx: Receiver<Batch>) -> ExitCode {
        let config = &self.config;

        // This will be set to `Some` if the `--exec` argument was supplied.
        if let Some(ref cmd) = config.command {
            if cmd.in_batch_mode() {
                exec::batch(rx.into_iter().flatten(), cmd, config)
            } else {
                thread::scope(|scope| {
                    // Each spawned job will store its thread handle in here.
                    let threads = config.threads;
                    let mut handles = Vec::with_capacity(threads);
                    for _ in 0..threads {
                        let rx = rx.clone();

                        // Spawn a job thread that will listen for and execute inputs.
                        let handle =
                            scope.spawn(|| exec::job(rx.into_iter().flatten(), cmd, config));

                        // Push the handle of the spawned thread into the vector for later joining.
                        handles.push(handle);
                    }
                    let exit_codes = handles.into_iter().map(|handle| handle.join().unwrap());
                    merge_exitcodes(exit_codes)
                })
            }
        } else {
            let stdout = io::stdout().lock();
            let stdout = io::BufWriter::new(stdout);

            ReceiverBuffer::new(self, rx, stdout).process()
        }
    }
```
- ReceiverBuffer.poll (src/walk.rs:197-248)
```rust
    fn poll(&mut self) -> Result<(), ExitCode> {
        match self.recv() {
            Ok(batch) => {
                for result in batch {
                    match result {
                        WorkerResult::Entry(dir_entry) => {
                            if self.config.quiet {
                                return Err(ExitCode::HasResults(true));
                            }

                            match self.mode {
                                ReceiverMode::Buffering => {
                                    self.buffer.push(dir_entry);
                                    if self.buffer.len() > MAX_BUFFER_LENGTH {
                                        self.stream()?;
                                    }
                                }
                                ReceiverMode::Streaming => {
                                    self.print(&dir_entry)?;
                                }
                            }

                            self.num_results += 1;
                            if let Some(max_results) = self.config.max_results
                                && self.num_results >= max_results
                            {
                                return self.stop();
                            }
                        }
                        WorkerResult::Error(err) => {
                            if self.config.show_filesystem_errors {
                                print_error!("{}", err);
                            }
                        }
                    }
                }

                // If we don't have another batch ready, flush before waiting
                if self.mode == ReceiverMode::Streaming && self.rx.is_empty() {
                    self.flush()?;
                }
            }
            Err(RecvTimeoutError::Timeout) => {
                self.stream()?;
            }
            Err(RecvTimeoutError::Disconnected) => {
                return self.stop();
            }
        }

        Ok(())
    }
```
- ReceiverBuffer.flush (src/walk.rs:295-301)
```rust
    fn flush(&mut self) -> Result<(), ExitCode> {
        if self.stdout.flush().is_err() {
            // Probably a broken pipe. Exit gracefully.
            return Err(ExitCode::GeneralError);
        }
        Ok(())
    }
```
- job (src/exec/job.rs:10-43)
```rust
pub fn job(
    results: impl IntoIterator<Item = WorkerResult>,
    cmd: &CommandSet,
    config: &Config,
) -> ExitCode {
    // Output should be buffered when only running a single thread
    let buffer_output: bool = config.threads > 1;

    let mut ret = ExitCode::Success;
    for result in results {
        // Obtain the next result from the receiver, else if the channel
        // has closed, exit from the loop
        let dir_entry = match result {
            WorkerResult::Entry(dir_entry) => dir_entry,
            WorkerResult::Error(err) => {
                if config.show_filesystem_errors {
                    print_error!("{}", err);
                }
                continue;
            }
        };

        // Generate a command, execute it and store its exit code.
        let code = cmd.execute(
            dir_entry.stripped_path(config),
            config.path_separator.as_deref(),
            config.null_separator,
            buffer_output,
        );
        ret = merge_exitcodes([ret, code]);
    }
    // Returns error in case of any error.
    ret
}
```
- is_broken_symlink (src/walk.rs:661-668)
```rust
fn is_broken_symlink(path: &Path, err: &ignore::Error) -> bool {
    err.io_error()
        .is_some_and(|io_error| io_error.kind() == io::ErrorKind::NotFound)
        && path
            .symlink_metadata()
            .ok()
            .is_some_and(|m| m.file_type().is_symlink())
}
```
- WorkerState.build_walker (src/walk.rs:346-403)
```rust
    fn build_walker(&self, paths: &[PathBuf]) -> Result<WalkParallel> {
        let first_path = &paths[0];
        let config = &self.config;
        let overrides = self.build_overrides(paths)?;

        let mut builder = WalkBuilder::new(first_path);
        builder
            .hidden(config.ignore_hidden)
            .ignore(config.read_fdignore)
            .parents(config.read_parent_ignore && (config.read_fdignore || config.read_vcsignore))
            .git_ignore(config.read_vcsignore)
            .git_global(config.read_vcsignore)
            .git_exclude(config.read_vcsignore)
            .require_git(config.require_git_to_read_vcsignore)
            .overrides(overrides)
            .follow_links(config.follow_links)
            // No need to check for supported platforms, option is unavailable on unsupported ones
            .same_file_system(config.one_file_system)
            .max_depth(config.max_depth);

        if config.read_fdignore {
            builder.add_custom_ignore_filename(".fdignore");
        }

        if config.read_global_ignore
            && let Ok(basedirs) = etcetera::choose_base_strategy()
        {
            let global_ignore_file = basedirs.config_dir().join("fd").join("ignore");
            if global_ignore_file.is_file() {
