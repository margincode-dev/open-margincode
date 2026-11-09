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
    let s = output.to_string_lossy();
    write!(
        stdout,
        "{}",
        maybe_sanitize(&s, config.interactive_terminal)
    )
}
```
- print_entry_uncolorized_base (src/output.rs:142-156)
```rust
fn print_entry_uncolorized_base<W: Write>(
    stdout: &mut W,
    entry: &DirEntry,
    config: &Config,
) -> io::Result<()> {
    let path = entry.stripped_path(config);

    let mut path_string = path.to_string_lossy();
    if let Some(ref separator) = config.path_separator {
        *path_string.to_mut() = replace_path_separator(&path_string, separator);
    }
    let safe = maybe_sanitize(&path_string, config.interactive_terminal);
    write!(stdout, "{safe}")?;
    print_trailing_slash(stdout, entry, config, None)
}
```
- Exec.augment_args (src/cli.rs:883-952)
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
                     There is no guarantee of the order commands are executed in, and the order should not be depended upon. \
                     All positional arguments following --exec are considered to be arguments to the command - not to fd. \
                     It is therefore recommended to place the '-x'/'--exec' option last. \
                     Use '\\;' to terminate the command template if you need to continue passing fd arguments afterwards.\n\
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
- ReceiverBuffer.poll (src/walk.rs:198-249)
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
                                print_error(err.to_string());
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
- job (src/exec/job.rs:11-44)
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
                    print_error(err.to_string());
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
- run (src/main.rs:75-112)
```rust
fn run() -> Result<ExitCode> {
    let opts = Opts::parse();

    #[cfg(feature = "completions")]
    if let Some(shell) = opts.gen_completions()? {
        return print_completions(shell);
    }

    set_working_dir(&opts)?;
    let search_paths = opts.search_paths()?;
    if search_paths.is_empty() {
        bail!("No valid search paths given.");
    }

    ensure_search_pattern_is_not_a_path(&opts)?;
    let pattern = &opts.pattern;
    let exprs = &opts.exprs;
    let empty = Vec::new();

    let pattern_regexps = exprs
        .as_ref()
        .unwrap_or(&empty)
        .iter()
        .chain([pattern])
        .map(|pat| build_pattern_regex(pat, &opts))
        .collect::<Result<Vec<String>>>()?;

    let config = construct_config(opts, &pattern_regexps)?;

    ensure_use_hidden_option_for_leading_dot_pattern(&config, &pattern_regexps)?;

    let regexps = pattern_regexps
        .into_iter()
        .map(|pat| build_regex(pat, &config))
        .collect::<Result<Vec<Regex>>>()?;

    walk::scan(&search_paths, regexps, config)
}
```
- print_entry_colorized (src/output.rs:89-139)
```rust
fn print_entry_colorized<W: Write>(
    stdout: &mut W,
    entry: &DirEntry,
    config: &Config,
    ls_colors: &LsColors,
) -> io::Result<()> {
    let mut offset = 0;
    let path = entry.stripped_path(config);
    let path_str = path.to_string_lossy();

    if let Some(parent) = path.parent() {
        offset = parent.to_string_lossy().len();
        for c in path_str[offset..].chars() {
            if std::path::is_separator(c) {
                offset += c.len_utf8();
            } else {
                break;
            }
        }
    }

    if offset > 0 {
        let mut parent_str = Cow::from(&path_str[..offset]);
        if let Some(ref separator) = config.path_separator {
            *parent_str.to_mut() = replace_path_separator(&parent_str, separator);
        }

        let style = ls_colors
            .style_for_indicator(Indicator::Directory)
            .map(Style::to_nu_ansi_term_style)
            .unwrap_or_default();
        let safe_parent = maybe_sanitize(&parent_str, config.interactive_terminal);
        write!(stdout, "{}", style.paint(safe_parent.as_ref()))?;
    }

    let style = entry
        .style(ls_colors)
        .map(Style::to_nu_ansi_term_style)
        .unwrap_or_default();
    let safe_basename = maybe_sanitize(&path_str[offset..], config.interactive_terminal);
    write!(stdout, "{}", style.paint(safe_basename.as_ref()))?;

    print_trailing_slash(
        stdout,
        entry,
        config,
        ls_colors.style_for_indicator(Indicator::Directory),
    )?;

    Ok(())
}
```
- test_exec_batch_with_limit (tests/tests.rs:2065-2099)
```rust
fn test_exec_batch_with_limit() {
    // TODO Test for windows
    if cfg!(windows) {
        return;
    }

    let te = TestEnv::new(DEFAULT_DIRS, DEFAULT_FILES);

    let output = te.assert_success_and_get_output(
        ".",
        &["foo", "--batch-size=2", "--exec-batch", "echo", "{}"],
    );
    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        assert_eq!(2, line.split_whitespace().count());
    }

    let mut paths: Vec<_> = stdout
        .lines()
        .flat_map(|line| line.split_whitespace())
        .collect();
    paths.sort_unstable();
    assert_eq!(
        &paths,
        &[
            "./a.foo",
            "./one/b.foo",
            "./one/two/C.Foo2",
            "./one/two/c.foo",
            "./one/two/three/d.foo",
            "./one/two/three/directory_foo"
        ],
    );
}
```
- test_and_pattern_with_forward_slash_is_rejected (tests/tests.rs:425-443)
```rust
fn test_and_pattern_with_forward_slash_is_rejected() {
    let te = TestEnv::new(DEFAULT_DIRS, DEFAULT_FILES);

    // Clean primary pattern, but a `--and` pattern carrying a path separator.
    te.assert_failure_with_error(
        &["foo", "--and", "nonexistent/path"],
        "[fd error]: The search pattern 'nonexistent/path' contains a path-separation character and will not lead to any search results.",
    );

    // `--full-path` is the explicit opt-in and must still suppress the
    // diagnostic for `--and` patterns too (the invocation runs and matches
    // instead of erroring on the path separator).
    #[cfg(not(windows))]
    te.assert_output(
        &["--full-path", "one/two/c", "--and", "two/c"],
        "one/two/c.foo
        one/two/C.Foo2",
    );
}
```
- Opts.search_paths (src/cli.rs:696-721)
```rust
    pub fn search_paths(&self) -> anyhow::Result<Vec<PathBuf>> {
        // would it make sense to concatenate these?
        let paths = if !self.path.is_empty() {
            &self.path
        } else if !self.search_path.is_empty() {
            &self.search_path
        } else {
            let current_directory = Path::new("./");
            ensure_current_directory_exists(current_directory)?;
            return Ok(vec![self.normalize_path(current_directory)]);
        };
        Ok(paths
            .iter()
            .filter_map(|path| {
                if filesystem::is_existing_directory(path) {
                    Some(self.normalize_path(path))
                } else {
                    print_error(format!(
                        "Search path '{}' is not a directory.",
                        path.to_string_lossy()
                    ));
                    None
                }
            })
            .collect())
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
- WorkerState.build_walker (src/walk.rs:347-404)
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
                        print_error(format!("Malformed pattern in global ignore file. {err}."));
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
                    print_error(format!("Malformed pattern in custom ignore file. {err}."));
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
- FormatTemplate.replace_separator (src/fmt/mod.rs:147-196)
```rust
    fn replace_separator<'a>(path: &'a OsStr, path_separator: Option<&str>) -> Cow<'a, OsStr> {
        // fast-path - no replacement necessary
        if path_separator.is_none() {
            return Cow::Borrowed(path);
        }

        let path_separator = path_separator.unwrap();
        let mut out = OsString::with_capacity(path.len());
        let mut components = Path::new(path).components().peekable();

        while let Some(comp) = components.next() {
            match comp {
                // Absolute paths on Windows are tricky.  A Prefix component is usually a drive
                // letter or UNC path, and is usually followed by RootDir. There are also
                // "verbatim" prefixes beginning with "\\?\" that skip normalization. We choose to
                // ignore verbatim path prefixes here because they're very rare, might be
                // impossible to reach here, and there's no good way to deal with them. If users
                // are doing something advanced involving verbatim windows paths, they can do their
                // own output filtering with a tool like sed.
                Component::Prefix(prefix) => {
                    if let Prefix::UNC(server, share) = prefix.kind() {
                        // Prefix::UNC is a parsed version of '\\server\share'
                        out.push(path_separator);
                        out.push(path_separator);
                        out.push(server);
                        out.push(path_separator);
                        out.push(share);
                    } else {
                        // All other Windows prefix types are rendered as-is. This results in e.g. "C:" for
                        // drive letters. DeviceNS and Verbatim* prefixes won't have backslashes converted,
                        // but they're not returned by directories fd can search anyway so we don't worry
                        // about them.
                        out.push(comp.as_os_str());
                    }
                }

                // Root directory is always replaced with the custom separator.
                Component::RootDir => out.push(path_separator),

                // Everything else is joined normally, with a trailing separator if we're not last
                _ => {
                    out.push(comp.as_os_str());
                    if components.peek().is_some() {
                        out.push(path_separator);
                    }
                }
            }
        }
        Cow::Owned(out)
    }
```
- test_ignore_contain (tests/tests.rs:2830-2853)
```rust
fn test_ignore_contain() {
    let te = TestEnv::new(
        &["include", "exclude", "exclude/sub", "other"],
        &[
            "top",
            "include/foo",
            "exclude/CACHEDIR.TAG",
            "exclude/sub/nope",
            "other/ignoremyparent",
        ],
    );
    let expected = "include/
    include/foo
    symlink
    top";
    te.assert_output(
        &[
            "--ignore-contain=CACHEDIR.TAG",
            "--ignore-contain=ignoremyparent",
            ".",
        ],
        expected,
    );
}
```
- main (src/main.rs:62-73)
```rust
fn main() {
    let result = run();
    match result {
        Ok(exit_code) => {
            exit_code.exit();
        }
        Err(err) => {
            crate::error::print_error(format!("{err:#}"));
            ExitCode::GeneralError.exit();
        }
    }
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
- test_ignore_contain_precedence_over_depth_check (tests/tests.rs:2856-2871)
```rust
fn test_ignore_contain_precedence_over_depth_check() {
    let te = TestEnv::new(
        &["include", "exclude", "exclude/sub"],
        &[
            "top",
            "include/foo",
            "exclude/CACHEDIR.TAG",
            "exclude/sub/nope",
        ],
    );
    let expected = "include/foo";
    te.assert_output(
        &["--ignore-contain=CACHEDIR.TAG", "--min-depth=2", "."],
        expected,
    );
}
```
- Opts (src/cli.rs:32-693)
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

    /// By default, the search pattern is only matched against the filename (or directory name). Using this flag, the pattern is matched against the full (absolute) path. Example:
    ///   fd --glob -p '**/.git/config'
    #[arg(
        long,
        short = 'p',
        help = "Search full abs. path (default: filename only)",
        long_help,
        verbatim_doc_comment
    )]
    pub full_path: bool,

    /// Separate search results by the null character (instead of newlines).
    /// Useful for piping results to 'xargs'.
    #[arg(
        long = "print0",
        short = '0',
        conflicts_with("list_details"),
        hide_short_help = true,
        help = "Separate search results by the null character",
        long_help
    )]
    pub null_separator: bool,

    /// Limit the directory traversal to a given depth. By default, there is no
    /// limit on the search depth.
    #[arg(
        long,
        short = 'd',
        value_name = "depth",
        alias("maxdepth"),
        help = "Set maximum search depth (default: none)",
        long_help
    )]
    max_depth: Option<usize>,

    /// Only show search results starting at the given depth.
    /// See also: '--max-depth' and '--exact-depth'
    #[arg(
        long,
        value_name = "depth",
        hide_short_help = true,
        alias("mindepth"),
        help = "Only show search results starting at the given depth.",
        long_help
    )]
    min_depth: Option<usize>,

    /// Only show search results at the exact given depth. This is an alias for
    /// '--min-depth <depth> --max-depth <depth>'.
    #[arg(long, value_name = "depth", hide_short_help = true, conflicts_with_all(&["max_depth", "min_depth"]),
    help = "Only show search results at the exact given depth",
        long_help,
        )]
    exact_depth: Option<usize>,

    /// Exclude files/directories that match the given glob pattern. This
    /// overrides any other ignore logic. Multiple exclude patterns can be
    /// specified.
    ///
    /// Examples:
    /// {n}  --exclude '*.pyc'
    /// {n}  --exclude node_modules
    #[arg(
        long,
        short = 'E',
        value_name = "glob",
        help = "Exclude entries that match the given glob pattern",
        long_help
    )]
    pub exclude: Vec<String>,

    /// Do not traverse into directories that match the search criteria. If
    /// you want to exclude specific directories, use the '--exclude=…' option.
    #[arg(long, hide_short_help = true, conflicts_with_all(&["size", "exact_depth"]),
        long_help,
        )]
    pub prune: bool,

    /// Filter the search by type:
    /// {n}  'f' or 'file':         regular files
    /// {n}  'd' or 'dir' or 'directory':    directories
    /// {n}  'l' or 'symlink':      symbolic links
    /// {n}  's' or 'socket':       socket
    /// {n}  'p' or 'pipe':         named pipe (FIFO)
    /// {n}  'b' or 'block-device': block device
    /// {n}  'c' or 'char-device':  character device
    /// {n}{n}  'x' or 'executable':   executables
    /// {n}  'e' or 'empty':        empty files or directories
    ///
    /// This option can be specified more than once to include multiple file types.
    /// Searching for '--type file --type symlink' will show both regular files as
    /// well as symlinks. Note that the 'executable' and 'empty' filters work differently:
    /// '--type executable' implies '--type file' by default. And '--type empty' searches
    /// for empty files and directories, unless either '--type file' or '--type directory'
    /// is specified in addition.
    ///
    /// Examples:
    /// {n}  - Only search for files:
