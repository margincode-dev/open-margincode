- build_regex (src/main.rs:522-536)
```rust
fn build_regex(pattern_regex: String, config: &Config) -> Result<regex::bytes::Regex> {
    RegexBuilder::new(&pattern_regex)
        .case_insensitive(!config.case_sensitive)
        .dot_matches_new_line(true)
        .build()
        .map_err(|e| {
            anyhow!(
                "{}\n\nNote: You can search for literal substrings with '--fixed-strings' \
                 or literal strings with '--exact' options (instead of a regular expression). \
                 Alternatively, you can \
                 also use the '--glob' option to match on a glob pattern.",
                e
            )
        })
}
```
- hir_matches_strings_with_leading_dot (src/regex_helper.rs:50-79)
```rust
fn hir_matches_strings_with_leading_dot(hir: &Hir) -> bool {
    use regex_syntax::hir::*;

    // Note: this only really detects the simplest case where a regex starts with
    // "^\\.", i.e. a start text anchor and a literal dot character. There are a lot
    // of other patterns that ONLY match hidden files, e.g. ^(\\.foo|\\.bar) which are
    // not (yet) detected by this algorithm.
    match hir.kind() {
        HirKind::Concat(hirs) => {
            let mut hirs = hirs.iter();
            if let Some(hir) = hirs.next() {
                if hir.kind() != &HirKind::Look(Look::Start) {
                    return false;
                }
            } else {
                return false;
            }

            if let Some(hir) = hirs.next() {
                match hir.kind() {
                    HirKind::Literal(Literal(bytes)) => bytes.starts_with(b"."),
                    _ => false,
                }
            } else {
                false
            }
        }
        _ => false,
    }
}
```
- test_smart_case (tests/tests.rs:517-537)
```rust
fn test_smart_case() {
    let te = TestEnv::new(DEFAULT_DIRS, DEFAULT_FILES);

    te.assert_output(
        &["c.foo"],
        "one/two/c.foo
        one/two/C.Foo2",
    );

    te.assert_output(&["C.Foo"], "one/two/C.Foo2");

    te.assert_output(&["Foo"], "one/two/C.Foo2");

    // Only literal uppercase chars should trigger case sensitivity.
    te.assert_output(
        &["\\Ac"],
        "one/two/c.foo
        one/two/C.Foo2",
    );
    te.assert_output(&["\\AC"], "one/two/C.Foo2");
}
```
- test_fixed_strings (tests/tests.rs:2412-2444)
```rust
fn test_fixed_strings() {
    let dirs = &["test1", "test2"];
    let files = &["test1/a.foo", "test1/a_foo", "test2/Download (1).tar.gz"];
    let te = TestEnv::new(dirs, files);

    // Regex search, dot is treated as "any character"
    te.assert_output(
        &["a.foo"],
        "test1/a.foo
         test1/a_foo",
    );

    // Literal search, dot is treated as character
    te.assert_output(&["--fixed-strings", "a.foo"], "test1/a.foo");

    // Regex search, parens are treated as group
    te.assert_output(&["download (1)"], "");

    // Literal search, parens are treated as characters. Case-insensitive by default.
    te.assert_output(
        &["--fixed-strings", "download (1)"],
        "test2/Download (1).tar.gz",
    );

    // Combine with --case-sensitive (unmatched).
    te.assert_output(&["--fixed-strings", "--case-sensitive", "download (1)"], "");

    // Combine with --case-sensitive (matched).
    te.assert_output(
        &["--fixed-strings", "--case-sensitive", "Download (1)"],
        "test2/Download (1).tar.gz",
    );
}
```
- hir_has_uppercase_char (src/regex_helper.rs:15-37)
```rust
fn hir_has_uppercase_char(hir: &Hir) -> bool {
    use regex_syntax::hir::*;

    match hir.kind() {
        HirKind::Literal(Literal(bytes)) => match std::str::from_utf8(bytes) {
            Ok(s) => s.chars().any(|c| c.is_uppercase()),
            Err(_) => bytes.iter().any(|b| char::from(*b).is_uppercase()),
        },
        HirKind::Class(Class::Unicode(ranges)) => ranges
            .iter()
            .any(|r| r.start().is_uppercase() || r.end().is_uppercase()),
        HirKind::Class(Class::Bytes(ranges)) => ranges
            .iter()
            .any(|r| char::from(r.start()).is_uppercase() || char::from(r.end()).is_uppercase()),
        HirKind::Capture(Capture { sub, .. }) | HirKind::Repetition(Repetition { sub, .. }) => {
            hir_has_uppercase_char(sub)
        }
        HirKind::Concat(hirs) | HirKind::Alternation(hirs) => {
            hirs.iter().any(hir_has_uppercase_char)
        }
        _ => false,
    }
}
```
- test_exact_literal_nonsubstring (tests/tests.rs:2448-2481)
```rust
fn test_exact_literal_nonsubstring() {
    let dirs = &["test1", "test2"];
    let files = &[
        "test1/a.foo",
        "test1/aa.foo",
        "test1/a.food",
        "test1/ca.food",
        "test1/a_foo",
        "test2/Download (1).tar.gz",
    ];
    let te = TestEnv::new(dirs, files);

    // Literal search, dot is treated as character. Should match only the exact name,
    // not "aa.foo", "a.food", or "ca.food".
    te.assert_output(&["--exact", "a.foo"], "test1/a.foo");

    // Literal search, parens are treated as characters. Substring should not match.
    te.assert_output(&["--exact", "download (1)"], "");

    // Literal search, parens are treated as characters. Case-insensitive by default.
    te.assert_output(
        &["--exact", "download (1).tar.gz"],
        "test2/Download (1).tar.gz",
    );

    // Combine with --case-sensitive, should not match due to case mismatch.
    te.assert_output(&["--exact", "--case-sensitive", "download (1).tar.gz"], "");

    // Combine with --case-sensitive, exact match should match.
    te.assert_output(
        &["--exact", "--case-sensitive", "Download (1).tar.gz"],
        "test2/Download (1).tar.gz",
    );
}
```
- pattern_has_uppercase_char (src/regex_helper.rs:5-12)
```rust
pub fn pattern_has_uppercase_char(pattern: &str) -> bool {
    let mut parser = ParserBuilder::new().utf8(false).build();

    parser
        .parse(pattern)
        .map(|hir| hir_has_uppercase_char(&hir))
        .unwrap_or(false)
}
```
- test_opposing (tests/tests.rs:2913-2928)
```rust
fn test_opposing(flag: &str, opposing_flags: &[&str]) {
    let te = TestEnv::new(DEFAULT_DIRS, DEFAULT_FILES);

    let mut flags = vec![flag];
    flags.extend_from_slice(opposing_flags);
    let out_no_flags = te.assert_success_and_get_normalized_output(".", &[]);
    let out_opposing_flags = te.assert_success_and_get_normalized_output(".", &flags);

    assert_eq!(
        out_no_flags,
        out_opposing_flags,
        "{} should override {}",
        opposing_flags.join(" "),
        flag
    );
}
```
- test_case_insensitive (tests/tests.rs:556-570)
```rust
fn test_case_insensitive() {
    let te = TestEnv::new(DEFAULT_DIRS, DEFAULT_FILES);

    te.assert_output(
        &["--ignore-case", "C.Foo"],
        "one/two/c.foo
        one/two/C.Foo2",
    );

    te.assert_output(
        &["--case-sensitive", "--ignore-case", "C.Foo"],
        "one/two/c.foo
        one/two/C.Foo2",
    );
}
```
- construct_config (src/main.rs:253-374)
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
        file_types: opts
            .filetype
            .as_ref()
            .map(|values| FileTypes::from_types(values)),
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
- test_type_executable (tests/tests.rs:1575-1613)
```rust
fn test_type_executable() {
    use std::os::unix::fs::OpenOptionsExt;

    // This test assumes the current user isn't root
    // (otherwise if the executable bit is set for any level, it is executable for the current
    // user)
    if Uid::current().is_root() {
        return;
    }

    let te = TestEnv::new(DEFAULT_DIRS, DEFAULT_FILES);

    fs::OpenOptions::new()
        .create_new(true)
        .truncate(true)
        .write(true)
        .mode(0o777)
        .open(te.test_root().join("executable-file.sh"))
        .unwrap();

    fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .mode(0o645)
        .open(te.test_root().join("not-user-executable-file.sh"))
        .unwrap();

    te.assert_output(&["--type", "executable"], "executable-file.sh");

    te.assert_output(
        &["--type", "executable", "--type", "directory"],
        "executable-file.sh
        one/
        one/two/
        one/two/three/
        one/two/three/directory_foo/",
    );
}
```
- WorkerState.new (src/walk.rs:317-327)
```rust
    fn new(patterns: Vec<Regex>, config: Config) -> Self {
        let quit_flag = Arc::new(AtomicBool::new(false));
        let interrupt_flag = Arc::new(AtomicBool::new(false));

        Self {
            patterns,
            config,
            quit_flag,
            interrupt_flag,
        }
    }
```
- WorkerState (src/walk.rs:305-314)
```rust
struct WorkerState {
    /// The search patterns.
    patterns: Vec<Regex>,
    /// The command line configuration.
    config: Config,
    /// Flag for cleanly shutting down the parallel walk
    quit_flag: Arc<AtomicBool>,
    /// Flag specifically for quitting due to ^C
    interrupt_flag: Arc<AtomicBool>,
}
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
