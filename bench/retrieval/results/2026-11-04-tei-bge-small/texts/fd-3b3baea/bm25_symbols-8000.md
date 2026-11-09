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
                let result = builder.add_ignore(global_ignore_file);
                match result {
                    Some(ignore::Error::Partial(_)) => (),
                    Some(err) => {
                        print_error!("Malformed pattern in global ignore file. {}.", err);
                    }
                    None => (),
                }
            }
        }

        for ignore_file in &config.ignore_files {
            let result = builder.add_ignore(ignore_file);
            match result {
                Some(ignore::Error::Partial(_)) => (),
                Some(err) => {
                    print_error!("Malformed pattern in custom ignore file. {}.", err);
                }
                None => (),
            }
        }

        for path in &paths[1..] {
            builder.add(path);
        }

        let walker = builder.threads(config.threads).build_parallel();
        Ok(walker)
    }
```
- Exec.augment_args (src/cli.rs:886-960)
```rust
    fn augment_args(cmd: Command) -> Command {
        cmd.arg(Arg::new("exec")
            .action(ArgAction::Append)
            .long("exec")
            .short('x')
            .num_args(1..)
                .allow_hyphen_values(true)
                .value_terminator(";")
                .value_name("cmd")
                .conflicts_with("list_details")
                .help("Execute a command for each search result")
                .long_help(
                    "Execute a command for each search result in parallel (use --threads=1 for sequential command execution). \
                     The order in which different search results are processed and their output is printed is not guaranteed, even with --threads=1. \
                     All positional arguments following --exec are considered to be arguments to the command - not to fd. \
                     It is therefore recommended to place the '-x'/'--exec' option last. \
                     Use '\\;' to terminate the command template if you need to continue passing fd arguments afterwards.\n\n\
                     Specify this option multiple times to run several commands for each file or directory found. \
                     For each file or directory, fd runs these commands one after another in the order they appear on the command line. \
                     Terminate each command except the last with '\\;'. \
                     When running in parallel, fd buffers command output and prints it together for each search result, \
                     without interleaving it with command output for other results.\n\n\
                     The following placeholders are substituted before the command is executed:\n  \
                       '{}':   path (of the current search result)\n  \
                       '{/}':  basename\n  \
                       '{//}': parent directory\n  \
                       '{.}':  path without file extension\n  \
                       '{/.}': basename without file extension\n  \
                       '{{':   literal '{' (for escaping)\n  \
                       '}}':   literal '}' (for escaping)\n\n\
                     If no placeholder is present, an implicit \"{}\" at the end is assumed.\n\n\
                     Examples:\n\n  \
                       - find all *.zip files and unzip them:\n\n      \
                           fd -e zip -x unzip\n\n  \
                       - find *.h and *.cpp files and run \"clang-format -i ..\" for each of them:\n\n      \
                           fd -e h -e cpp -x clang-format -i\n\n  \
                       - search within `src/` and echo each match (place `-x` last):\n\n      \
                           fd . src -x echo\n\n  \
                       - Convert all *.jpg files to *.png files:\n\n      \
                           fd -e jpg -x convert {} {.}.png\
                    ",
                ),
        )
        .arg(
            Arg::new("exec_batch")
                .action(ArgAction::Append)
                .long("exec-batch")
                .short('X')
                .num_args(1..)
                .allow_hyphen_values(true)
                .value_terminator(";")
                .value_name("cmd")
                .conflicts_with_all(["exec", "list_details"])
                .help("Execute a command with all search results at once")
                .long_help(
                    "Execute the given command once, with all search results as arguments.\n\
                     The order of the arguments is non-deterministic, and should not be relied upon.\n\
                     One of the following placeholders is substituted before the command is executed:\n  \
                       '{}':   path (of all search results)\n  \
                       '{/}':  basename\n  \
                       '{//}': parent directory\n  \
                       '{.}':  path without file extension\n  \
                       '{/.}': basename without file extension\n  \
                       '{{':   literal '{' (for escaping)\n  \
                       '}}':   literal '}' (for escaping)\n\n\
                     If no placeholder is present, an implicit \"{}\" at the end is assumed.\n\n\
                     Examples:\n\n  \
                       - Find all test_*.py files and open them in your favorite editor:\n\n      \
                           fd -g 'test_*.py' -X vim\n\n  \
                       - Find all *.rs files and count the lines with \"wc -l ...\":\n\n      \
                           fd -e rs -X wc -l\
                     "
                ),
        )
    }
```
- CommandBuilder.push (src/exec/mod.rs:173-189)
```rust
    fn push(&mut self, path: &Path, separator: Option<&str>) -> io::Result<()> {
        if self.limit > 0 && self.count >= self.limit {
            self.finish()?;
        }

        let arg = self.path_arg.generate(path, separator);
        if !self
            .cmd
            .args_would_fit(iter::once(&arg).chain(&self.post_args))
        {
            self.finish()?;
        }

        self.cmd.try_arg(arg)?;
        self.count += 1;
        Ok(())
    }
```
- general_error_if_at_least_one_error (src/exit_codes.rs:63-84)
```rust
    fn general_error_if_at_least_one_error() {
        assert_eq!(
            merge_exitcodes([ExitCode::GeneralError]),
            ExitCode::GeneralError
        );
        assert_eq!(
            merge_exitcodes([ExitCode::KilledBySigint]),
            ExitCode::GeneralError
        );
        assert_eq!(
            merge_exitcodes([ExitCode::KilledBySigint, ExitCode::Success]),
            ExitCode::GeneralError
        );
        assert_eq!(
            merge_exitcodes([ExitCode::Success, ExitCode::GeneralError]),
            ExitCode::GeneralError
        );
        assert_eq!(
            merge_exitcodes([ExitCode::GeneralError, ExitCode::KilledBySigint]),
            ExitCode::GeneralError
        );
    }
```
- ExecutionMode (src/exec/mod.rs:22-27)
```rust
pub enum ExecutionMode {
    /// Command is executed for each search result
    OneByOne,
    /// Command is run for a batch of results at once
    Batch,
}
```
- batch (src/exec/job.rs:45-63)
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
                    print_error!("{}", err);
                }
                None
            }
        });

    cmd.execute_batch(paths, config.batch_size, config.path_separator.as_deref())
}
```
- DirEntry.normal (src/dir_entry.rs:28-34)
```rust
    pub fn normal(e: ignore::DirEntry) -> Self {
        Self {
            inner: DirEntryInner::Normal(e),
            metadata: OnceCell::new(),
            style: OnceCell::new(),
        }
    }
```
- Check (src/filter/owner.rs:12-16)
```rust
enum Check<T> {
    Equal(T),
    NotEq(T),
    Ignore,
}
```
- success_if_no_error (src/exit_codes.rs:87-93)
```rust
    fn success_if_no_error() {
        assert_eq!(merge_exitcodes([ExitCode::Success]), ExitCode::Success);
        assert_eq!(
            merge_exitcodes([ExitCode::Success, ExitCode::Success]),
            ExitCode::Success
        );
    }
```
- ensure_search_pattern_is_not_a_path (src/main.rs:174-186)
```rust
fn ensure_search_pattern_is_not_a_path(opts: &Opts) -> Result<()> {
    if opts.full_path {
        return Ok(());
    }

    // Check the primary pattern and every `--and` pattern. They are all matched
    // against the file name (see `run`), so a path separator in any of them is
    // the same silent "no results" footgun.
    for pattern in std::iter::once(&opts.pattern).chain(opts.exprs.iter().flatten()) {
        ensure_single_search_pattern_is_not_a_path(pattern)?;
    }
    Ok(())
}
```
- merge_exitcodes (src/exit_codes.rs:46-51)
```rust
pub fn merge_exitcodes(results: impl IntoIterator<Item = ExitCode>) -> ExitCode {
    if results.into_iter().any(ExitCode::is_error) {
        return ExitCode::GeneralError;
    }
    ExitCode::Success
}
```
- tokens_multiple_batch (src/exec/mod.rs:422-424)
```rust
    fn tokens_multiple_batch() {
        assert!(CommandSet::new_batch(vec![vec!["echo", "{.}", "{}"]]).is_err());
    }
```
- i32.from (src/exit_codes.rs:15-22)
```rust
    fn from(code: ExitCode) -> Self {
        match code {
            ExitCode::Success => 0,
            ExitCode::HasResults(has_results) => !has_results as i32,
            ExitCode::GeneralError => 1,
            ExitCode::KilledBySigint => 130,
        }
    }
```
- Error.Sanitized (src/error.rs:37-37)
```rust
    type Sanitized = SanitizedError<ignore::Error>;
```
- Opts (src/cli.rs:31-694)
```rust
pub struct Opts {
    /// Include hidden directories and files in the search results (default:
    /// hidden files and directories are skipped). Files and directories are
    /// considered to be hidden if their name starts with a `.` sign (dot).
    /// Any files or directories that are ignored due to the rules described by
    /// --no-ignore are still ignored unless otherwise specified.
    /// The flag can be overridden with --no-hidden.
    #[arg(
        long,
        short = 'H',
        help = "Search hidden files and directories",
        long_help
    )]
    pub hidden: bool,

    /// Overrides --hidden
    #[arg(long, overrides_with = "hidden", hide = true, action = ArgAction::SetTrue)]
    no_hidden: (),

    /// Show search results from files and directories that would otherwise be
    /// ignored by '.gitignore', '.ignore', '.fdignore', or the global ignore file,
    /// The flag can be overridden with --ignore.
    #[arg(
        long,
        short = 'I',
        help = "Do not respect .(git|fd)ignore files",
        long_help
    )]
    pub no_ignore: bool,

    /// Overrides --no-ignore
    #[arg(long, overrides_with = "no_ignore", hide = true, action = ArgAction::SetTrue)]
    ignore: (),

    ///Show search results from files and directories that
    ///would otherwise be ignored by '.gitignore' files.
    ///The flag can be overridden with --ignore-vcs.
    #[arg(
        long,
        hide_short_help = true,
        help = "Do not respect .gitignore files",
        long_help
    )]
    pub no_ignore_vcs: bool,

    /// Overrides --no-ignore-vcs
    #[arg(long, overrides_with = "no_ignore_vcs", hide = true, action = ArgAction::SetTrue)]
    ignore_vcs: (),

    /// Do not require a git repository to respect gitignores.
    /// By default, fd will only respect global gitignore rules, .gitignore rules,
    /// and local exclude rules if fd detects that you are searching inside a
    /// git repository. This flag allows you to relax this restriction such that
    /// fd will respect all git related ignore rules regardless of whether you're
    /// searching in a git repository or not.
    ///
    ///
    /// This flag can be disabled with --require-git.
    #[arg(
        long,
        overrides_with = "require_git",
        hide_short_help = true,
        // same description as ripgrep's flag: ripgrep/crates/core/app.rs
        long_help
    )]
    pub no_require_git: bool,

    /// Overrides --no-require-git
    #[arg(long, overrides_with = "no_require_git", hide = true, action = ArgAction::SetTrue)]
    require_git: (),

    /// Show search results from files and directories that would otherwise be
    /// ignored by '.gitignore', '.ignore', or '.fdignore' files in parent directories.
    /// The flag can be overridden with --ignore-parent.
    #[arg(
        long,
        hide_short_help = true,
        help = "Do not respect .(git|fd)ignore files in parent directories",
        long_help
    )]
    pub no_ignore_parent: bool,

    /// Overrides --no-ignore-parent
    #[arg(long, overrides_with = "no_ignore_parent", hide = true, action = ArgAction::SetTrue)]
    ignore_parent: (),

    /// Do not respect the global ignore file
    #[arg(long, hide = true)]
    pub no_global_ignore_file: bool,

    /// Perform an unrestricted search, including ignored and hidden files. This is
    /// an alias for '--no-ignore --hidden'.
    #[arg(long = "unrestricted", short = 'u', overrides_with_all(&["ignore", "no_hidden"]), action(ArgAction::Count), hide_short_help = true,
    help = "Unrestricted search, alias for '--no-ignore --hidden'",
        long_help,
        )]
    rg_alias_hidden_ignore: u8,

    /// Case-sensitive search (default: smart case)
    #[arg(
        long,
        short = 's',
        overrides_with("ignore_case"),
        long_help = "Perform a case-sensitive search. By default, fd uses case-insensitive \
                     searches, unless the pattern contains an uppercase character (smart \
                     case)."
    )]
    pub case_sensitive: bool,

    /// Perform a case-insensitive search. By default, fd uses case-insensitive
    /// searches, unless the pattern contains an uppercase character (smart
    /// case).
    #[arg(
        long,
        short = 'i',
        overrides_with("case_sensitive"),
        help = "Case-insensitive search (default: smart case)",
        long_help
    )]
    pub ignore_case: bool,

    /// Perform a glob-based search instead of a regular expression search.
    #[arg(
        long,
        short = 'g',
        conflicts_with("fixed_strings"),
        help = "Glob-based search (default: regular expression)",
        long_help
    )]
    pub glob: bool,

    /// Perform a regular-expression based search (default). This can be used to
    /// override --glob.
    #[arg(
        long,
        overrides_with("glob"),
        hide_short_help = true,
        help = "Regular-expression based search (default)",
        long_help
    )]
    pub regex: bool,

    /// Treat the pattern as a literal string instead of a regular expression. Note
    /// that this also performs substring comparison. If you want to match on an
    /// exact filename, consider using '--glob' or '--exact' instead.
    #[arg(
        long,
        short = 'F',
        alias = "literal",
        hide_short_help = true,
        help = "Treat pattern as literal string instead of regex",
        long_help
    )]
    pub fixed_strings: bool,

    /// Perform an exact match. This is equivalent to '--fixed-strings' but requires
    /// the pattern to match the entire filename (or path if '--full-path' is used),
    /// rather than a substring. Special regex characters in the pattern are treated
    /// as literal characters.
    #[arg(
        long,
        conflicts_with_all(["glob", "fixed_strings"]),
        hide_short_help = true,
        help = "Match the entire filename exactly (literal, non-substring)",
        long_help
    )]
    pub exact: bool,

    /// Add additional required search patterns, all of which must be matched. Multiple
    /// additional patterns can be specified. The patterns are regular
    /// expressions, unless '--glob' or '--fixed-strings' is used.
    #[arg(
        long = "and",
        value_name = "pattern",
        help = "Additional search patterns that need to be matched",
        long_help,
        hide_short_help = true,
        allow_hyphen_values = true
    )]
    pub exprs: Option<Vec<String>>,

    /// Shows the full path starting from the root as opposed to relative paths.
    /// The flag can be overridden with --relative-path.
    #[arg(
        long,
        short = 'a',
        help = "Show absolute instead of relative paths",
        long_help
    )]
    pub absolute_path: bool,

    /// Overrides --absolute-path
    #[arg(long, overrides_with = "absolute_path", hide = true, action = ArgAction::SetTrue)]
    relative_path: (),

    /// Use a detailed listing format like 'ls -l'. This is basically an alias
    /// for '--exec-batch ls -l' with some additional 'ls' options. This can be
    /// used to see more metadata, to show symlink targets and to achieve a
    /// deterministic sort order.
    #[arg(
        long,
        short = 'l',
        conflicts_with("absolute_path"),
        help = "Use a long listing format with file metadata",
        long_help
    )]
    pub list_details: bool,

    /// Follow symbolic links
    #[arg(
        long,
        short = 'L',
        alias = "dereference",
        long_help = "By default, fd does not descend into symlinked directories. Using this \
                     flag, symbolic links are also traversed. \
                     Flag can be overridden with --no-follow."
    )]
    pub follow: bool,

    /// Overrides --follow
    #[arg(long, overrides_with = "follow", hide = true, action = ArgAction::SetTrue)]
    no_follow: (),

