- DirEntryInner (src/dir_entry.rs:12-17)
```rust
enum DirEntryInner {
    Normal(ignore::DirEntry),
    // Broken symlinks are surfaced by the walker as errors that carry no depth,
    // so we record the depth (relative to the search root) at creation time.
    BrokenSymlink { path: PathBuf, depth: usize },
}
```
- construct_config (src/main.rs:248-393)
```rust
fn construct_config(mut opts: Opts, pattern_regexps: &[String]) -> Result<Config> {
    // The search will be case-sensitive if the command line flag is set or
    // if any of the patterns has an uppercase character (smart case).
    let case_sensitive = !opts.ignore_case
        && (opts.case_sensitive
            || pattern_regexps
                .iter()
                .any(|pat| pattern_has_uppercase_char(pat)));

    let path_separator = opts
        .path_separator
        .take()
        .or_else(filesystem::default_path_separator);
    let actual_path_separator = path_separator
        .clone()
        .unwrap_or_else(|| std::path::MAIN_SEPARATOR.to_string());
    check_path_separator_length(path_separator.as_deref())?;

    let size_limits = std::mem::take(&mut opts.size);
    let time_constraints = extract_time_constraints(&opts)?;
    #[cfg(unix)]
    let owner_constraint: Option<OwnerFilter> = opts.owner.and_then(OwnerFilter::filter_ignore);

    #[cfg(windows)]
    let ansi_colors_support =
        nu_ansi_term::enable_ansi_support().is_ok() || std::env::var_os("TERM").is_some();
    #[cfg(not(windows))]
    let ansi_colors_support = true;

    let interactive_terminal = std::io::stdout().is_terminal();

    let colored_output = match opts.color {
        ColorWhen::Always => true,
        ColorWhen::Never => false,
        ColorWhen::Auto => {
            let no_color = env::var_os("NO_COLOR").is_some_and(|x| !x.is_empty());
            ansi_colors_support && !no_color && interactive_terminal
        }
    };

    let ls_colors = if colored_output {
        Some(LsColors::from_env().unwrap_or_else(|| LsColors::from_string(DEFAULT_LS_COLORS)))
    } else {
        None
    };
    let hyperlink = match opts.hyperlink {
        HyperlinkWhen::Always => true,
        HyperlinkWhen::Never => false,
        HyperlinkWhen::Auto => colored_output,
    };
    let command = extract_command(&mut opts, colored_output)?;
    let has_command = command.is_some();

    let full_path_base = if opts.full_path {
        Some(env::current_dir().context(
            "Could not determine current directory. \
             This is required for --full-path.",
        )?)
    } else {
        None
    };

    Ok(Config {
        case_sensitive,
        full_path_base,
        ignore_hidden: !(opts.hidden || opts.rg_alias_ignore()),
        read_fdignore: !(opts.no_ignore || opts.rg_alias_ignore()),
        read_vcsignore: !(opts.no_ignore || opts.rg_alias_ignore() || opts.no_ignore_vcs),
        require_git_to_read_vcsignore: !opts.no_require_git,
        read_parent_ignore: !opts.no_ignore_parent,
        read_global_ignore: !(opts.no_ignore
            || opts.rg_alias_ignore()
            || opts.no_global_ignore_file),
        follow_links: opts.follow,
        one_file_system: opts.one_file_system,
        null_separator: opts.null_separator,
        quiet: opts.quiet,
        max_depth: opts.max_depth(),
        min_depth: opts.min_depth(),
        prune: opts.prune,
        threads: opts.threads().get(),
        max_buffer_time: opts.max_buffer_time,
        ls_colors,
        hyperlink,
        interactive_terminal,
        file_types: opts.filetype.as_ref().map(|values| {
            use crate::cli::FileType::*;
            let mut file_types = FileTypes::default();
            for value in values {
                match value {
                    File => file_types.files = true,
                    Directory => file_types.directories = true,
                    Symlink => file_types.symlinks = true,
                    Executable => {
                        file_types.executables_only = true;
                        file_types.files = true;
                    }
                    Empty => file_types.empty_only = true,
                    BlockDevice => file_types.block_devices = true,
                    CharDevice => file_types.char_devices = true,
                    Socket => file_types.sockets = true,
                    Pipe => file_types.pipes = true,
                }
            }

            // If only 'empty' was specified, search for both files and directories:
            if file_types.empty_only && !(file_types.files || file_types.directories) {
                file_types.files = true;
                file_types.directories = true;
            }

            file_types
        }),
        extensions: opts
            .extensions
            .as_ref()
            .map(|exts| {
                let patterns = exts
                    .iter()
                    .map(|e| e.trim_start_matches('.'))
                    .map(|e| format!(r".\.{}$", regex::escape(e)));
                RegexSetBuilder::new(patterns)
                    .case_insensitive(true)
                    .build()
            })
            .transpose()?,
        format: opts
            .format
            .as_deref()
            .map(crate::fmt::FormatTemplate::parse),
        command: command.map(Arc::new),
        batch_size: opts.batch_size,
        exclude_patterns: opts.exclude.iter().map(|p| String::from("!") + p).collect(),
        ignore_files: std::mem::take(&mut opts.ignore_file),
        size_constraints: size_limits,
        time_constraints,
        #[cfg(unix)]
        owner_constraint,
        show_filesystem_errors: opts.show_errors,
        path_separator,
        actual_path_separator,
        max_results: opts.max_results(),
        strip_cwd_prefix: opts.strip_cwd_prefix(|| !(opts.null_separator || has_command)),
        ignore_contain: opts.ignore_contain,
    })
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
- Config (src/config.rs:14-136)
```rust
pub struct Config {
    /// Whether the search is case-sensitive or case-insensitive.
    pub case_sensitive: bool,

    /// Cached current working directory for absolute path construction.
    /// Populated when `--full-path` is set; `None` means search by filename only.
    pub full_path_base: Option<PathBuf>,

    /// Whether to ignore hidden files and directories (or not).
    pub ignore_hidden: bool,

    /// Whether to respect `.fdignore` files or not.
    pub read_fdignore: bool,

    /// Whether to respect ignore files in parent directories or not.
    pub read_parent_ignore: bool,

    /// Whether to respect VCS ignore files (`.gitignore`, ..) or not.
    pub read_vcsignore: bool,

    /// Whether to require a `.git` directory to respect gitignore files.
    pub require_git_to_read_vcsignore: bool,

    /// Whether to respect the global ignore file or not.
    pub read_global_ignore: bool,

    /// Whether to follow symlinks or not.
    pub follow_links: bool,

    /// Whether to limit the search to starting file system or not.
    pub one_file_system: bool,

    /// Whether elements of output should be separated by a null character
    pub null_separator: bool,

    /// The maximum search depth, or `None` if no maximum search depth should be set.
    ///
    /// A depth of `1` includes all files under the current directory, a depth of `2` also includes
    /// all files under subdirectories of the current directory, etc.
    pub max_depth: Option<usize>,

    /// The minimum depth for reported entries, or `None`.
    pub min_depth: Option<usize>,

    /// Whether to stop traversing into matching directories.
    pub prune: bool,

    /// The number of threads to use.
    pub threads: usize,

    /// If true, the program doesn't print anything and will instead return an exit code of 0
    /// if there's at least one match. Otherwise, the exit code will be 1.
    pub quiet: bool,

    /// Time to buffer results internally before streaming to the console. This is useful to
    /// provide a sorted output, in case the total execution time is shorter than
    /// `max_buffer_time`.
    pub max_buffer_time: Option<Duration>,

    /// `None` if the output should not be colorized. Otherwise, a `LsColors` instance that defines
    /// how to style different filetypes.
    pub ls_colors: Option<LsColors>,

    /// Whether or not we are writing to an interactive terminal
    #[cfg_attr(not(unix), allow(unused))]
    pub interactive_terminal: bool,

    /// The type of file to search for. If set to `None`, all file types are displayed. If
    /// set to `Some(..)`, only the types that are specified are shown.
    pub file_types: Option<FileTypes>,

    /// The extension to search for. Only entries matching the extension will be included.
    ///
    /// The value (if present) will be a lowercase string without leading dots.
    pub extensions: Option<RegexSet>,

    /// A format string to use to format results, similarly to exec
    pub format: Option<FormatTemplate>,

    /// If a value is supplied, each item found will be used to generate and execute commands.
    pub command: Option<Arc<CommandSet>>,

    /// Maximum number of search results to pass to each `command`. If zero, the number is
    /// unlimited.
    pub batch_size: usize,

    /// A list of glob patterns that should be excluded from the search.
    pub exclude_patterns: Vec<String>,

    /// A list of custom ignore files.
    pub ignore_files: Vec<PathBuf>,

    /// The given constraints on the size of returned files
    pub size_constraints: Vec<SizeFilter>,

    /// Constraints on last modification time of files
    pub time_constraints: Vec<TimeFilter>,

    #[cfg(unix)]
    /// User/group ownership constraint
    pub owner_constraint: Option<OwnerFilter>,

    /// Whether or not to display filesystem errors
    pub show_filesystem_errors: bool,

    /// The separator used to print file paths.
    pub path_separator: Option<String>,

    /// The actual separator, either the system default separator or `path_separator`
    pub actual_path_separator: String,

    /// The maximum number of search results
    pub max_results: Option<usize>,

    /// Whether or not to strip the './' prefix for search results
    pub strip_cwd_prefix: bool,

    /// Whether or not to use hyperlinks on paths
    pub hyperlink: bool,

    /// Names that should stop traversal down their parent. (e.g. https://bford.info/cachedir/).
    pub ignore_contain: Vec<String>,
}
```
- test_min_depth_broken_symlink (tests/tests.rs:1216-1246)
```rust
fn test_min_depth_broken_symlink() {
    let mut te = TestEnv::new(DEFAULT_DIRS, DEFAULT_FILES);
    te.create_broken_symlink("one/two/broken_symlink")
        .expect("Failed to create broken symlink.");

    // The broken symlink sits at depth 3, so it is kept up to that depth.
    te.assert_output(
        &[
            "--follow",
            "--type",
            "symlink",
            "--min-depth",
            "3",
            "broken_symlink",
        ],
        "one/two/broken_symlink",
    );

    // A --min-depth beyond its actual depth must exclude it.
    te.assert_output(
        &[
            "--follow",
            "--type",
            "symlink",
            "--min-depth",
            "4",
            "broken_symlink",
        ],
        "",
    );
}
```
- test_min_depth_broken_symlink_absolute_path (tests/tests.rs:1254-1286)
```rust
fn test_min_depth_broken_symlink_absolute_path() {
    let (mut te, abs_path) = get_test_env_with_abs_path(DEFAULT_DIRS, DEFAULT_FILES);
    te.create_broken_symlink("one/two/broken_symlink")
        .expect("Failed to create broken symlink.");

    // The broken symlink sits at depth 3 relative to the (absolute) root.
    te.assert_output(
        &[
            "--follow",
            "--absolute-path",
            "--type",
            "symlink",
            "--min-depth",
            "3",
            "broken_symlink",
        ],
        &format!("{abs_path}/one/two/broken_symlink"),
    );

    // A --min-depth beyond its actual depth must exclude it.
    te.assert_output(
        &[
            "--follow",
            "--absolute-path",
            "--type",
            "symlink",
            "--min-depth",
            "4",
            "broken_symlink",
        ],
        "",
    );
}
```
- WorkerState.spawn_senders (src/walk.rs:443-615)
```rust
    fn spawn_senders(&self, walker: WalkParallel, roots: &[PathBuf], tx: Sender<Batch>) {
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
                    Err(ignore::Error::WithPath {
                        path,
                        err: inner_err,
                    }) if inner_err
                        .io_error()
                        .is_some_and(|io_error| io_error.kind() == io::ErrorKind::NotFound)
                        && path
                            .symlink_metadata()
                            .ok()
                            .is_some_and(|m| m.file_type().is_symlink()) =>
                    {
                        let depth = depth_relative_to_roots(&path, roots);
                        DirEntry::broken_symlink(path, depth)
                    }
                    Err(err) => {
                        return match tx.send(WorkerResult::Error(err)) {
                            Ok(_) => WalkState::Continue,
                            Err(_) => WalkState::Quit,
                        };
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
- test_multi_file_with_missing (tests/tests.rs:340-375)
```rust
fn test_multi_file_with_missing() {
    let dirs = &["real"];
    let files = &["real/a.foo", "real/b.foo"];
    let te = TestEnv::new(dirs, files);
    te.assert_output(&["a.foo", "real", "fake"], "real/a.foo");

    te.assert_error(
        &["a.foo", "real", "fake"],
        "[fd error]: Search path 'fake' is not a directory.",
    );

    te.assert_output(
        &["", "real", "fake"],
        "real/a.foo
        real/b.foo",
    );

    te.assert_output(
        &["", "real", "fake1", "fake2"],
        "real/a.foo
        real/b.foo",
    );

    te.assert_error(
        &["", "real", "fake1", "fake2"],
        "[fd error]: Search path 'fake1' is not a directory.
        [fd error]: Search path 'fake2' is not a directory.",
    );

    te.assert_failure_with_error(
        &["", "fake1", "fake2"],
        "[fd error]: Search path 'fake1' is not a directory.
        [fd error]: Search path 'fake2' is not a directory.
        [fd error]: No valid search paths given.",
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
    /// {n}      fd --type file …
    /// {n}      fd -tf …
    /// {n}  - Find both files and symlinks
    /// {n}      fd --type file --type symlink …
    /// {n}      fd -tf -tl …
    /// {n}  - Find executable files:
    /// {n}      fd --type executable
    /// {n}      fd -tx
    /// {n}  - Find empty files:
    /// {n}      fd --type empty --type file
    /// {n}      fd -te -tf
    /// {n}  - Find empty directories:
    /// {n}      fd --type empty --type directory
    /// {n}      fd -te -td
    #[arg(
        long = "type",
        short = 't',
        value_name = "filetype",
        hide_possible_values = true,
        value_enum,
        help = "Filter by type: file (f), directory (d/dir), symlink (l), \
                executable (x), empty (e), socket (s), pipe (p), \
                char-device (c), block-device (b)",
        long_help
    )]
    pub filetype: Option<Vec<FileType>>,

    /// Filter results by extension. By default, this matches all entry types
    /// (including directories whose names end with the extension). Use `--type`
    /// to restrict the search to specific types. Multiple extensions can be specified.
    ///
    /// If you want to search for files without extension,
    /// you can use the regex '^[^.]+$' as a normal search pattern.
    #[arg(
        long = "extension",
        short = 'e',
        value_name = "ext",
        help = "Filter by extension",
        long_help
    )]
    pub extensions: Option<Vec<String>>,

    /// Limit results based on the size of files using the format <+-><NUM><UNIT>.
    ///    '+': file size must be greater than or equal to this
    ///    '-': file size must be less than or equal to this
    ///
    /// If neither '+' nor '-' is specified, file size must be exactly equal to this.
    ///    'NUM':  The numeric size (e.g. 500)
    ///    'UNIT': The units for NUM. They are not case-sensitive.
    /// Allowed unit values:
    ///     'b':  bytes
    ///     'k':  kilobytes (base ten, 10^3 = 1000 bytes)
    ///     'm':  megabytes
    ///     'g':  gigabytes
    ///     't':  terabytes
    ///     'ki': kibibytes (base two, 2^10 = 1024 bytes)
    ///     'mi': mebibytes
    ///     'gi': gibibytes
    ///     'ti': tebibytes
