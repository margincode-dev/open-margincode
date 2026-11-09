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
