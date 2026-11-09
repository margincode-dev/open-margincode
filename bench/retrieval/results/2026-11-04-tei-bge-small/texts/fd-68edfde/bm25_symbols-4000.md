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
