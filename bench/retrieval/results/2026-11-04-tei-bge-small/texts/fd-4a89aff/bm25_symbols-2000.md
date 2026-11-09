- depth_relative_to_roots (src/walk.rs:665-674)
```rust
fn depth_relative_to_roots(path: &Path, roots: &[PathBuf]) -> usize {
    let components = path.components().count();
    let root_components = roots
        .iter()
        .filter(|root| path.starts_with(root))
        .map(|root| root.components().count())
        .max()
        .unwrap_or(0);
    components.saturating_sub(root_components)
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
- DirEntryInner (src/dir_entry.rs:12-17)
```rust
enum DirEntryInner {
    Normal(ignore::DirEntry),
    // Broken symlinks are surfaced by the walker as errors that carry no depth,
    // so we record the depth (relative to the search root) at creation time.
    BrokenSymlink(PathBuf, usize),
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
- DirEntry.depth (src/dir_entry.rs:100-105)
```rust
    pub fn depth(&self) -> Option<usize> {
        match &self.inner {
            DirEntryInner::Normal(e) => Some(e.depth()),
            DirEntryInner::BrokenSymlink(_, depth) => Some(*depth),
        }
    }
```
- Opts (src/cli.rs:32-693)
```rust
pub struct Opts {
    /// Include hidden directories and files in the search results (default:
    /// hidden files and directories are skipped). Files and directories are
    /// considered to be hidden if their name starts with a `.` sign (dot).
