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

