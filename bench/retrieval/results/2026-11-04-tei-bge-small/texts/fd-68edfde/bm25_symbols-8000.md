- DirEntryInner (src/dir_entry.rs:12-17)
```rust
enum DirEntryInner {
    Normal(ignore::DirEntry),
    // Broken symlinks are surfaced by the walker as errors that carry no depth,
    // so we record the depth (relative to the search root) at creation time.
    BrokenSymlink(PathBuf, usize),
}
```
- DirEntry.depth (src/dir_entry.rs:100-105)
```rust
    pub fn depth(&self) -> Option<usize> {
        match &self.inner {
            DirEntryInner::Normal(e) => Some(e.depth()),
            DirEntryInner::BrokenSymlink(_, depth) => Some(*depth),
        }
    }
```
- DirEntry.path (src/dir_entry.rs:44-49)
```rust
    pub fn path(&self) -> &Path {
        match &self.inner {
            DirEntryInner::Normal(e) => e.path(),
            DirEntryInner::BrokenSymlink(pathbuf, _) => pathbuf.as_path(),
        }
    }
```
- DirEntry.into_path (src/dir_entry.rs:51-56)
```rust
    pub fn into_path(self) -> PathBuf {
        match self.inner {
            DirEntryInner::Normal(e) => e.into_path(),
            DirEntryInner::BrokenSymlink(p, _) => p,
        }
    }
```
- DirEntry.broken_symlink (src/dir_entry.rs:36-42)
```rust
    pub fn broken_symlink(path: PathBuf, depth: usize) -> Self {
        Self {
            inner: DirEntryInner::BrokenSymlink(path, depth),
            metadata: OnceCell::new(),
            style: OnceCell::new(),
        }
    }
```
- DirEntry.metadata (src/dir_entry.rs:91-98)
```rust
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata
            .get_or_init(|| match &self.inner {
                DirEntryInner::Normal(e) => e.metadata().ok(),
                DirEntryInner::BrokenSymlink(path, _) => path.symlink_metadata().ok(),
            })
            .as_ref()
    }
```
- DirEntry.file_type (src/dir_entry.rs:84-89)
```rust
    pub fn file_type(&self) -> Option<FileType> {
        match &self.inner {
            DirEntryInner::Normal(e) => e.file_type(),
            DirEntryInner::BrokenSymlink(..) => self.metadata().map(|m| m.file_type()),
        }
    }
```
- DirEntry.file_name (src/dir_entry.rs:146-160)
```rust
    fn file_name(&self) -> OsString {
        let name = match &self.inner {
            DirEntryInner::Normal(e) => e.file_name(),
            DirEntryInner::BrokenSymlink(path, _) => {
                // Path::file_name() only works if the last component is Normal,
                // but we want it for all component types, so we open code it.
                // Copied from LsColors::style_for_path_with_metadata().
                path.components()
                    .next_back()
                    .map(|c| c.as_os_str())
                    .unwrap_or_else(|| path.as_os_str())
            }
        };
        name.to_owned()
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
- DirEntry (src/dir_entry.rs:20-24)
```rust
pub struct DirEntry {
    inner: DirEntryInner,
    metadata: OnceCell<Option<Metadata>>,
    style: OnceCell<Option<Style>>,
}
```
- test_symlink_as_root (tests/tests.rs:1605-1643)
```rust
fn test_symlink_as_root() {
    let mut te = TestEnv::new(DEFAULT_DIRS, DEFAULT_FILES);
    te.create_broken_symlink("broken_symlink")
        .expect("Failed to create broken symlink.");

    // From: http://pubs.opengroup.org/onlinepubs/9699919799/functions/getcwd.html
    // The getcwd() function shall place an absolute pathname of the current working directory in
    // the array pointed to by buf, and return buf. The pathname shall contain no components that
    // are dot or dot-dot, or are symbolic links.
    //
    // Key points:
    // 1. The path of the current working directory of a Unix process cannot contain symlinks.
    // 2. The path of the current working directory of a Windows process can contain symlinks.
    //
    // More:
    // 1. On Windows, symlinks are resolved after the ".." component.
    // 2. On Unix, symlinks are resolved immediately as encountered.

    let parent_parent = if cfg!(windows) { ".." } else { "../.." };
    te.assert_output_subdirectory(
        "symlink",
        &["", parent_parent],
        &format!(
            "{dir}/a.foo
            {dir}/broken_symlink
            {dir}/e1 e2
            {dir}/one/
            {dir}/one/b.foo
            {dir}/one/two/
            {dir}/one/two/c.foo
            {dir}/one/two/C.Foo2
            {dir}/one/two/three/
            {dir}/one/two/three/d.foo
            {dir}/one/two/three/directory_foo/
            {dir}/symlink",
            dir = parent_parent
        ),
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
- Opts.max_depth (src/cli.rs:747-749)
```rust
    pub fn max_depth(&self) -> Option<usize> {
        self.max_depth.or(self.exact_depth)
    }
```
- remove_symlink (tests/tests.rs:2456-2465)
```rust
fn remove_symlink<P: AsRef<Path>>(path: P) {
    #[cfg(unix)]
    fs::remove_file(path).expect("remove symlink");

    // On Windows, symlinks remember whether they point to files or directories, so try both
    #[cfg(windows)]
    fs::remove_file(path.as_ref())
        .or_else(|_| fs::remove_dir(path.as_ref()))
        .expect("remove symlink");
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
- TestEnv.create_broken_symlink (tests/testenv/mod.rs:179-197)
```rust
    pub fn create_broken_symlink<P: AsRef<Path>>(
        &mut self,
        link_path: P,
    ) -> Result<PathBuf, io::Error> {
        let root = self.test_root();
        let broken_symlink_link = root.join(link_path);
        {
            let temp_target_dir = tempfile::Builder::new()
                .prefix("fd-tests-broken-symlink")
                .tempdir()?;
            let broken_symlink_target = temp_target_dir.path().join("broken_symlink_target");
            fs::File::create(&broken_symlink_target)?;
            #[cfg(unix)]
            unix::fs::symlink(&broken_symlink_target, &broken_symlink_link)?;
            #[cfg(windows)]
            windows::fs::symlink_file(&broken_symlink_target, &broken_symlink_link)?;
        }
        Ok(broken_symlink_link)
    }
```
- test_max_depth (tests/tests.rs:1155-1188)
```rust
fn test_max_depth() {
    let te = TestEnv::new(DEFAULT_DIRS, DEFAULT_FILES);

    te.assert_output(
        &["--max-depth", "3"],
        "a.foo
        e1 e2
        one/
        one/b.foo
        one/two/
        one/two/c.foo
        one/two/C.Foo2
        one/two/three/
        symlink",
    );

    te.assert_output(
        &["--max-depth", "2"],
        "a.foo
        e1 e2
        one/
        one/b.foo
        one/two/
        symlink",
    );

    te.assert_output(
        &["--max-depth", "1"],
        "a.foo
        e1 e2
        one/
        symlink",
    );
}
```
- WorkerResult (src/walk.rs:40-45)
```rust
pub enum WorkerResult {
    // Errors should be rare, so it's probably better to allow large_enum_variant than
    // to box the Entry variant
    Entry(DirEntry),
    Error(ignore::Error),
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
- test_number_parsing_errors (tests/tests.rs:2733-2749)
```rust
fn test_number_parsing_errors() {
    let te = TestEnv::new(&[], &[]);

    te.assert_failure(&["--threads=a"]);
    te.assert_failure(&["-j", ""]);
    te.assert_failure(&["--threads=0"]);

    te.assert_failure(&["--min-depth=a"]);
    te.assert_failure(&["--mindepth=a"]);
    te.assert_failure(&["--max-depth=a"]);
    te.assert_failure(&["--maxdepth=a"]);
    te.assert_failure(&["--exact-depth=a"]);

    te.assert_failure(&["--max-buffer-time=a"]);

    te.assert_failure(&["--max-results=a"]);
}
```
- FileTypes.should_ignore (src/filetypes.rs:21-42)
```rust
    pub fn should_ignore(&self, entry: &dir_entry::DirEntry) -> bool {
        if let Some(ref entry_type) = entry.file_type() {
            (!self.files && entry_type.is_file())
                || (!self.directories && entry_type.is_dir())
                || (!self.symlinks && entry_type.is_symlink())
                || (!self.block_devices && filesystem::is_block_device(*entry_type))
                || (!self.char_devices && filesystem::is_char_device(*entry_type))
                || (!self.sockets && filesystem::is_socket(*entry_type))
                || (!self.pipes && filesystem::is_pipe(*entry_type))
                || (self.executables_only && !entry.path().executable())
                || (self.empty_only && !filesystem::is_empty(entry))
                || !(entry_type.is_file()
                    || entry_type.is_dir()
                    || entry_type.is_symlink()
                    || filesystem::is_block_device(*entry_type)
                    || filesystem::is_char_device(*entry_type)
                    || filesystem::is_socket(*entry_type)
                    || filesystem::is_pipe(*entry_type))
        } else {
            true
        }
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
    #[arg(long, short = 'S', value_parser = SizeFilter::from_string, allow_hyphen_values = true, verbatim_doc_comment, value_name = "size",
        help = "Limit results based on the size of files",
        long_help,
        verbatim_doc_comment,
        )]
    pub size: Vec<SizeFilter>,

    /// Filter results based on the file modification time. Files with modification times
    /// greater than the argument are returned. The argument can be provided
    /// as a specific point in time (YYYY-MM-DD HH:MM:SS or @timestamp) or as a duration (10h, 1d, 35min).
    /// If the time is not specified, it defaults to 00:00:00.
    /// '--change-newer-than', '--newer', or '--changed-after' can be used as aliases.
    ///
    /// Examples:
    /// {n}    --changed-within 2weeks
    /// {n}    --change-newer-than '2018-10-27 10:00:00'
    /// {n}    --newer 2018-10-27
    /// {n}    --changed-after 1day
    #[arg(
        long,
        alias("change-newer-than"),
        alias("newer"),
        alias("changed-after"),
        value_name = "date|dur",
        help = "Filter by file modification time (newer than)",
        long_help
    )]
    pub changed_within: Option<String>,

    /// Filter results based on the file modification time. Files with modification times
    /// less than the argument are returned. The argument can be provided
    /// as a specific point in time (YYYY-MM-DD HH:MM:SS or @timestamp) or as a duration (10h, 1d, 35min).
    /// '--change-older-than' or '--older' can be used as aliases.
    ///
    /// Examples:
    /// {n}    --changed-before '2018-10-27 10:00:00'
    /// {n}    --change-older-than 2weeks
    /// {n}    --older 2018-10-27
    #[arg(
        long,
        alias("change-older-than"),
        alias("older"),
        value_name = "date|dur",
        help = "Filter by file modification time (older than)",
        long_help
    )]
    pub changed_before: Option<String>,

    /// Filter files by their user and/or group.
    /// Format: [(user|uid)][:(group|gid)]. Either side is optional.
    /// Precede either side with a '!' to exclude files instead.
    ///
    /// Examples:
