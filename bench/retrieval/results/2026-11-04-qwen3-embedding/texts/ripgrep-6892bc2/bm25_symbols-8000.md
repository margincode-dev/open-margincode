- IncrementalIgnore.matched_with_errors_ignore (crates/ignore/src/incremental.rs:310-391)
```rust
    fn matched_with_errors_ignore(
        &mut self,
        relative: &Path,
        is_dir: bool,
        errs: &mut PartialErrorBuilder,
    ) -> Option<IncrementalMatch> {
        let mut components = relative
            .components()
            .filter_map(|component| match component {
                std::path::Component::CurDir => None,
                component => Some(component),
            })
            .peekable();
        components.peek()?;

        // If the exact parent is cached, then all of its ancestors have
        // already been checked. An allowed cache entry is the matcher to use
        // for children of that directory, while an ignored cache entry is
        // terminal for every descendant. In the usual case, this avoids both
        // walking every component and allocating a relative directory path.
        //
        // Only try the fast path when the final component is a normal path
        // component. Invalid paths are handled by the component walk below.
        let has_normal_final_component =
            relative.components().next_back().is_some_and(|component| {
                matches!(component, std::path::Component::Normal(_))
            });
        if has_normal_final_component && let Some(parent) = relative.parent() {
            match self.dirs.get(parent) {
                Some(CachedDir::Allowed(ignore)) => {
                    return Some(self.match_path(ignore, relative, is_dir));
                }
                Some(CachedDir::Ignored) => {
                    return Some(IncrementalMatch::ignore());
                }
                None => {}
            }
        }

        let mut ignore = self.root_ignore(errs)?;
        let mut dir = PathBuf::new();
        while let Some(component) = components.next() {
            match component {
                std::path::Component::ParentDir
                | std::path::Component::RootDir
                | std::path::Component::Prefix(_) => {
                    return None;
                }
                std::path::Component::CurDir => continue,
                std::path::Component::Normal(_) => {}
            }
            if components.peek().is_none() {
                break;
            }
            dir.push(component.as_os_str());
            match self.dirs.get(&dir) {
                Some(CachedDir::Allowed(cached)) => {
                    ignore = cached.clone();
                    continue;
                }
                Some(CachedDir::Ignored) => {
                    return Some(IncrementalMatch::ignore());
                }
                None => {}
            }

            let path = self.root.join(&dir);
            let mat = ignore.matched(&path, true);
            let is_hidden =
                self.options.hidden && mat.is_none() && is_hidden_path(&path);
            if mat.is_ignore() || is_hidden {
                self.dirs.insert(dir.clone(), CachedDir::Ignored);
                return Some(IncrementalMatch::ignore());
            }
            let (child, err) = ignore.add_child(&path);
            errs.maybe_push(err);
            self.dirs.insert(dir.clone(), CachedDir::Allowed(child.clone()));
            ignore = child;
        }

        Some(self.match_path(&ignore, relative, is_dir))
    }
```
- cached_ignored_parent_matches_children (crates/ignore/src/incremental.rs:848-860)
```rust
    fn cached_ignored_parent_matches_children() {
        let td = tmpdir();
        mkdirp(td.path().join("blocked"));
        wfile(td.path().join(".ignore"), "blocked/\n");

        let mut m = one_matcher(&builder(td.path()));
        assert!(matchedf(&mut m, "blocked/first").is_ignore());
        assert!(matches!(
            m.dirs.get(Path::new("blocked")),
            Some(CachedDir::Ignored)
        ));
        assert!(matchedf(&mut m, "blocked/second").is_ignore());
    }
```
- CachedDir (crates/ignore/src/incremental.rs:94-103)
```rust
enum CachedDir {
    /// The directory may be descended into. The matcher includes the ignore
    /// rules loaded through this directory and is therefore the matcher to use
    /// for its children.
    Allowed(Ignore),
    /// The directory may not be descended into because it was ignored by a
    /// path rule or hidden-file filtering. Every descendant is consequently
    /// ignored, and ignore files inside this directory are not loaded.
    Ignored,
}
```
- cached_allowed_parent_matches_children (crates/ignore/src/incremental.rs:833-845)
```rust
    fn cached_allowed_parent_matches_children() {
        let td = tmpdir();
        mkdirp(td.path().join("a/b"));
        wfile(td.path().join("a/b/.ignore"), "ignored\n");

        let mut m = one_matcher(&builder(td.path()));
        assert!(matchedf(&mut m, "a/b/allowed").is_none());
        assert!(matches!(
            m.dirs.get(Path::new("a/b")),
            Some(CachedDir::Allowed(_))
        ));
        assert!(matchedf(&mut m, "a/b/ignored").is_ignore());
    }
```
- cached_parent_matcher_is_used_for_directory (crates/ignore/src/incremental.rs:863-877)
```rust
    fn cached_parent_matcher_is_used_for_directory() {
        let td = tmpdir();
        mkdirp(td.path().join("a/b"));
        wfile(td.path().join("a/b/.ignore"), "**\n");

        let mut m = one_matcher(&builder(td.path()));
        assert!(matchedf(&mut m, "a/b/file").is_ignore());
        assert!(matches!(
            m.dirs.get(Path::new("a/b")),
            Some(CachedDir::Allowed(_))
        ));
        let dir = matchedd(&mut m, "a/b");
        assert!(dir.is_none());
        assert!(dir.should_descend());
    }
```
- normalize_absolute (crates/ignore/src/incremental.rs:542-555)
```rust
fn normalize_absolute(path: &Path) -> Option<PathBuf> {
    let absolute = std::path::absolute(path).ok()?;
    let mut normalized = PathBuf::new();
    for component in absolute.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                normalized.pop();
            }
            _ => normalized.push(component.as_os_str()),
        }
    }
    Some(normalized)
}
```
- incremental_dense_ignore_warm (crates/ignore/benches/workloads.rs:436-447)
```rust
fn incremental_dense_ignore_warm(b: &mut test::Bencher) {
    let ws = dense_ignore_workspace();
    let mut matchers = ws.builder().build_matchers();
    for path in &ws.paths {
        assert!(!matchers[0].matched(path, false).is_ignore());
    }
    b.iter(|| {
        for path in &ws.paths {
            assert!(!matchers[0].matched(black_box(path), false).is_ignore());
        }
    });
}
```
- incremental_warm (crates/ignore/benches/workloads.rs:257-268)
```rust
fn incremental_warm(b: &mut test::Bencher) {
    let ws = Workspace::new(16, 4, 4, true);
    let mut matchers = ws.builder().build_matchers();
    for path in &ws.paths {
        assert!(!matchers[0].matched(path, false).is_ignore());
    }
    b.iter(|| {
        for path in &ws.paths {
            assert!(!matchers[0].matched(black_box(path), false).is_ignore());
        }
    });
}
```
- IncrementalIgnore (crates/ignore/src/incremental.rs:57-68)
```rust
pub struct IncrementalIgnore {
    /// The root exactly as it was given to `WalkBuilder`.
    root: PathBuf,
    /// The normalized root used only by the opt-in normalization routine.
    normalized_root: OnceLock<Option<PathBuf>>,
    /// The matcher for the configured root directory, loaded on first use.
    ignore: RootIgnore,
    /// Directory paths relative to `root`, excluding the root itself.
    dirs: HashMap<PathBuf, CachedDir>,
    /// Options for additional filtering beyond gitignore.
    options: IncrementalIgnoreOptions,
}
```
- incremental_warm_deep (crates/ignore/benches/workloads.rs:352-363)
```rust
fn incremental_warm_deep(b: &mut test::Bencher) {
    let ws = Workspace::new(16, 12, 4, true);
    let mut matchers = ws.builder().build_matchers();
    for path in &ws.paths {
        assert!(!matchers[0].matched(path, false).is_ignore());
    }
    b.iter(|| {
        for path in &ws.paths {
            assert!(!matchers[0].matched(black_box(path), false).is_ignore());
        }
    });
}
```
- incremental_warm_noncanonical (crates/ignore/benches/workloads.rs:271-290)
```rust
fn incremental_warm_noncanonical(b: &mut test::Bencher) {
    let ws = Workspace::new(16, 4, 4, true);
    let paths: Vec<_> = ws
        .paths
        .iter()
        .map(|path| {
            Path::new("packages/.")
                .join(path.strip_prefix("packages").unwrap())
        })
        .collect();
    let mut matchers = ws.builder().build_matchers();
    for path in &paths {
        assert!(!matchers[0].matched(path, false).is_ignore());
    }
    b.iter(|| {
        for path in &paths {
            assert!(!matchers[0].matched(black_box(path), false).is_ignore());
        }
    });
}
```
- IgnoreInner (crates/ignore/src/dir.rs:115-169)
```rust
struct IgnoreInner {
    /// A map of all existing directories that have already been
    /// compiled into matchers.
    ///
    /// Note that this is never used during matching, only when adding new
    /// parent directory matchers. This avoids needing to rebuild glob sets for
    /// parent directories if many paths are being searched.
    compiled: Arc<RwLock<HashMap<OsString, Weak<IgnoreInner>>>>,
    /// The path to the directory that this matcher was built from.
    dir: PathBuf,
    /// An override matcher (default is empty).
    overrides: Arc<Override>,
    /// A file type matcher.
    types: Arc<Types>,
    /// The parent directory to match next.
    ///
    /// If this is the root directory or there are otherwise no more
    /// directories to match, then `parent` is `None`.
    parent: Option<Arc<IgnoreInner>>,
    /// Whether this is an absolute parent matcher, as added by add_parent.
    is_absolute_parent: bool,
    /// Whether any absolute parent has ignore rules. When none do, matching
    /// can skip rewriting the candidate path for those parents.
    has_absolute_parent_rules: bool,
    /// The directory that gitignores should be interpreted relative to.
    ///
    /// Usually this is the directory containing the gitignore file. But in
    /// some cases, like for global gitignores or for gitignores specified
    /// explicitly, this should generally be set to the current working
    /// directory. This is only used for global gitignores or "explicit"
    /// gitignores.
    ///
    /// When `None`, this means the CWD could not be determined or is unknown.
    /// In this case, global gitignore files are ignored because they otherwise
    /// cannot be matched correctly.
    global_gitignores_relative_to: Option<PathBuf>,
    /// Explicit global ignore matchers specified by the caller.
    explicit_ignores: Arc<Vec<Gitignore>>,
    /// Ignore files used in addition to `.ignore`
    custom_ignore_filenames: Arc<Vec<OsString>>,
    /// The matcher for custom ignore files
    custom_ignore_matcher: Gitignore,
    /// The matcher for .ignore files.
    ignore_matcher: Gitignore,
    /// A global gitignore matcher, usually from $XDG_CONFIG_HOME/git/ignore.
    git_global_matcher: Arc<Gitignore>,
    /// The matcher for .gitignore files.
    git_ignore_matcher: Gitignore,
    /// Special matcher for `.git/info/exclude` files.
    git_exclude_matcher: Gitignore,
    /// Whether this directory contains a .git sub-directory.
    has_git: bool,
    /// Ignore config.
    opts: IgnoreOptions,
}
```
- incremental_ignored_parent (crates/ignore/benches/workloads.rs:380-388)
```rust
fn incremental_ignored_parent(b: &mut test::Bencher) {
    let ws = Workspace::new(0, 0, 0, false);
    let mut matchers = ws.builder().build_matchers();
    let path = Path::new(".venv/dependency.py");
    assert!(matchers[0].matched(path, false).is_ignore());
    b.iter(|| {
        assert!(matchers[0].matched(black_box(path), false).is_ignore())
    });
}
```
- IncrementalIgnore.matched_with_errors_impl (crates/ignore/src/incremental.rs:224-308)
```rust
    fn matched_with_errors_impl(
        &mut self,
        relative: &Path,
        is_dir: bool,
        errs: &mut PartialErrorBuilder,
    ) -> IncrementalMatch {
        // We short-circuit here when our matcher corresponds to `Stdin` in
        // order to always return a non-match. This is somewhat redundant with
        // `root_ignore()` which does this too, but we do it here so that it
        // always happens, e.g., before depth filtering.
        if relative.is_absolute() || matches!(self.ignore, RootIgnore::Stdin) {
            return IncrementalMatch::none(is_dir);
        }

        let (mut satisfies_min, mut edge_max) = (true, false);
        if self.options.min_depth.is_some() || self.options.max_depth.is_some()
        {
            let depth = relative
                .components()
                .filter_map(|component| match component {
                    std::path::Component::CurDir => None,
                    component => Some(component),
                })
                .count();
            satisfies_min =
                self.options.min_depth.is_none_or(|min| depth >= min);
            let satisfies_max =
                self.options.max_depth.is_none_or(|max| depth <= max);
            edge_max = self.options.max_depth.is_some_and(|max| depth == max);
            // When we have a file that isn't past our min depth, we can give
            // up right away.
            if !is_dir && !satisfies_min {
                return IncrementalMatch::ignore().not_within_depth();
            }
            // Same for *anything* that exceeds our max depth.
            if !satisfies_max {
                return IncrementalMatch::ignore().not_within_depth();
            }
        }

        // When the path is invalid in some way, we bail out early to avoid
        // potentially doing a stat call below.
        let (mut mat, valid) = self
            .matched_with_errors_ignore(relative, is_dir, errs)
            .map(|mat| (mat, true))
            .unwrap_or_else(|| (IncrementalMatch::none(is_dir), false));

        if !is_dir
            && !mat.is_ignore()
            && valid
            && let Some(max_filesize) = self.options.max_filesize
        {
            let path = self.root().join(relative);
            let result = if self.options.follow_links {
                path.metadata()
            } else {
                path.symlink_metadata()
            };
            match result {
                Ok(md) if md.len() > max_filesize => {
                    return IncrementalMatch::ignore();
                }
                Ok(_) => {}
                Err(err) => {
                    // Record the error but otherwise fall through
                    let err = Error::from(err);
                    errs.push(err.with_path(path));
                }
            }
        }

        // We still need to tag our `mat` if it doesn't pass the depth filter,
        // or if it's a directory on the edge of a depth filter. This can only
        // happen when the path is reported as a directory. Otherwise regular
        // files are always handled above.
        if is_dir {
            if !satisfies_min {
                mat = mat.not_within_depth();
            }
            if edge_max {
                mat = mat.no_descent();
            }
        }
        mat
    }
```
- parent_ignore_and_custom_ignore (crates/ignore/src/incremental.rs:679-695)
```rust
    fn parent_ignore_and_custom_ignore() {
        let td = tmpdir();
        let root = td.path().join("project/work");
        mkdirp(root.join("src"));
        wfile(td.path().join(".ignore"), "*.cache\n");
        wfile(td.path().join(".rgignore"), "*.svg\n");
        wfile(td.path().join("project/.ignore"), "!keep.cache\n");
        wfile(td.path().join("project/.rgignore"), "!keep.svg\n");

        let mut builder = builder(&root);
        builder.add_custom_ignore_filename(".rgignore");
        let mut m = one_matcher(&builder);
        assert!(matchedf(&mut m, "src/drop.cache").is_ignore());
        assert!(matchedf(&mut m, "src/keep.cache").is_whitelist());
        assert!(matchedf(&mut m, "src/drop.svg").is_ignore());
        assert!(matchedf(&mut m, "src/keep.svg").is_whitelist());
    }
```
- directly_ignored_directory_is_not_descended (crates/ignore/src/incremental.rs:751-761)
```rust
    fn directly_ignored_directory_is_not_descended() {
        let td = tmpdir();
        mkdirp(td.path().join(".git"));
        mkdirp(td.path().join("blocked"));
        wfile(td.path().join(".gitignore"), "blocked/\n");

        let mut m = one_matcher(&builder(td.path()));
        let dir = matchedd(&mut m, "blocked");
        assert!(dir.is_ignore());
        assert!(!dir.should_descend());
    }
```
- DirEntryRaw (crates/ignore/src/walk.rs:238-256)
```rust
struct DirEntryRaw {
    /// The path as reported by the `fs::ReadDir` iterator (even if it's a
    /// symbolic link).
    path: PathBuf,
    /// The file type. Necessary for recursive iteration, so store it.
    ty: FileType,
    /// Is set when this entry was created from a symbolic link and the user
    /// expects the iterator to follow symbolic links.
    follow_link: bool,
    /// The depth at which this entry was generated relative to the root.
    depth: usize,
    /// The underlying inode number (Unix only).
    #[cfg(unix)]
    ino: u64,
    /// The underlying metadata (Windows only). We store this on Windows
    /// because this comes for free while reading a directory.
    #[cfg(windows)]
    metadata: fs::Metadata,
}
```
- is_file_name (crates/ignore/src/pathutil.rs:123-136)
```rust
pub(crate) fn is_file_name<P: AsRef<Path>>(path: P) -> bool {
    #[cfg(unix)]
    {
        memchr::memchr(b'/', path.as_ref().as_os_str().as_encoded_bytes())
            .is_none()
    }
    #[cfg(not(unix))]
    {
        path.as_ref()
            .parent()
            .map(|p| p.as_os_str().is_empty())
            .unwrap_or(false)
    }
}
```
- max_filesize_does_not_stat_ignored_file (crates/ignore/src/incremental.rs:1159-1169)
```rust
    fn max_filesize_does_not_stat_ignored_file() {
        let td = tmpdir();
        wfile(td.path().join(".ignore"), "ignored\n");

        let mut b = builder(td.path());
        b.max_filesize(Some(0));
        let mut m = one_matcher(&b);
        let (mat, err) = m.matched_with_errors("ignored", false);
        assert!(mat.is_ignore());
        assert!(err.is_none(), "ignored missing file was statted: {err:?}");
    }
```
- panic_in_parallel_builder (crates/ignore/src/walk.rs:2762-2774)
```rust
    fn panic_in_parallel_builder() {
        let td = tmpdir();
        wfile(td.path().join("foo.txt"), "");

        let mut builds = 0;
        WalkBuilder::new(td.path()).threads(2).build_parallel().run(|| {
            builds += 1;
            if builds == 3 {
                panic!("builder panic");
            }
            Box::new(|_| WalkState::Continue)
        });
    }
```
- absolute_parent_matchers_are_cached_across_roots (crates/ignore/src/dir.rs:1486-1506)
```rust
    fn absolute_parent_matchers_are_cached_across_roots() {
        let td = tmpdir();
        mkdirp(td.path().join(".git"));
        mkdirp(td.path().join("src/build"));
        mkdirp(td.path().join("tests/build"));
        wfile(td.path().join(".gitignore"), "tests/**/build/\n");

        let ig0 = IgnoreBuilder::new().build();
        let (src_parents, err) = ig0.add_parents(td.path().join("src"));
        assert!(err.is_none());
        let (src, err) = src_parents.add_child(td.path().join("src"));
        assert!(err.is_none());
        let (tests_parents, err) = ig0.add_parents(td.path().join("tests"));
        assert!(err.is_none());
        let (tests, err) = tests_parents.add_child(td.path().join("tests"));
        assert!(err.is_none());

        assert!(Arc::ptr_eq(&src_parents.inner, &tests_parents.inner));
        assert!(src.matched("build", true).is_none());
        assert!(tests.matched("build", true).is_ignore());
    }
```
- file_name (crates/ignore/src/pathutil.rs:142-171)
```rust
pub(crate) fn file_name<'a, P: AsRef<Path> + ?Sized>(
    path: &'a P,
) -> Option<&'a OsStr> {
    #[cfg(unix)]
    fn imp(path: &Path) -> Option<&OsStr> {
        use std::os::unix::ffi::OsStrExt;

        use memchr::memrchr;

        let path = path.as_os_str().as_bytes();
        if path.is_empty() {
            return None;
        }
        let last_slash = memrchr(b'/', path).map(|i| i + 1).unwrap_or(0);
        let name = &path[last_slash..];
        // Only a final component that is exactly `.` or `..` has no file
        // name. A real name that merely ends in a `.`, like `.foo.`, does.
        if name == b"." || name == b".." {
            return None;
        }
        Some(OsStr::from_bytes(name))
    }

    #[cfg(not(unix))]
    fn imp(path: &Path) -> Option<&OsStr> {
        path.file_name()
    }

    imp(path.as_ref())
}
```
- same_file_system (crates/ignore/src/walk.rs:2628-2654)
```rust
    fn same_file_system() {
        use super::device_num;

        // If for some reason /sys doesn't exist or isn't a directory, just
        // skip this test.
        if !Path::new("/sys").is_dir() {
            return;
        }

        // If our test directory actually isn't a different volume from /sys,
        // then this test is meaningless and we shouldn't run it.
        let td = tmpdir();
        if device_num(td.path()).unwrap() == device_num("/sys").unwrap() {
            return;
        }

        mkdirp(td.path().join("same_file"));
        symlink("/sys", td.path().join("same_file").join("alink"));

        // Create a symlink to sys and enable following symlinks. If the
        // same_file_system option doesn't work, then this probably will hit a
        // permission error. Otherwise, it should just skip over the symlink
        // completely.
        let mut builder = WalkBuilder::new(td.path());
        builder.follow_links(true).same_file_system(true);
        assert_paths(td.path(), &builder, &["same_file", "same_file/alink"]);
    }
```
- ignored_ancestor_wins (crates/ignore/src/incremental.rs:767-783)
```rust
    fn ignored_ancestor_wins() {
        let td = tmpdir();
        mkdirp(td.path().join(".git"));
        mkdirp(td.path().join("blocked"));
        mkdirp(td.path().join("open"));
        // This is the key line: since the entire `blocked` directory is
        // ignored, a proper file traversal won't ever descend into it. So
        // `blocked/keep.rs` should be ignored even if there are ignore rules
        // "beneath" it that whitelist it.
        wfile(td.path().join(".gitignore"), "blocked/\n!blocked/keep.rs\n");
        wfile(td.path().join("blocked/.gitignore"), "!keep.rs\n");

        let mut m = one_matcher(&builder(td.path()));
        assert!(matchedf(&mut m, "blocked/keep.rs").is_ignore());
        assert!(matchedf(&mut m, "blocked/other.rs").is_ignore());
        assert!(matchedf(&mut m, "open/keep.rs").is_none());
    }
```
- HiArgs.walk_builder (crates/core/flags/hiargs.rs:874-918)
```rust
    pub(crate) fn walk_builder(&self) -> anyhow::Result<ignore::WalkBuilder> {
        let mut builder = ignore::WalkBuilder::new(&self.paths.paths[0]);
        for path in self.paths.paths.iter().skip(1) {
            builder.add(path);
        }
        if !self.no_ignore_files {
            for path in self.ignore_file.iter() {
                if let Some(err) = builder.add_ignore(path) {
                    ignore_message!("{err}");
                }
            }
        }
        builder
            .max_depth(self.max_depth)
            .follow_links(self.follow)
            .max_filesize(self.max_filesize)
            .threads(self.threads)
            .same_file_system(self.one_file_system)
            .skip_stdout(matches!(self.mode, Mode::Search(_)))
            .overrides(self.globs.clone())
            .types(self.types.clone())
            .hidden(!self.hidden)
            .parents(!self.no_ignore_parent)
            .ignore(!self.no_ignore_dot)
            .git_global(!self.no_ignore_vcs && !self.no_ignore_global)
            .git_ignore(!self.no_ignore_vcs)
            .git_exclude(!self.no_ignore_vcs && !self.no_ignore_exclude)
            .require_git(!self.no_require_git)
            .ignore_case_insensitive(self.ignore_file_case_insensitive)
            .current_dir(&self.cwd);
        if !self.no_ignore_dot {
            builder.add_custom_ignore_filename(".rgignore");
        }
        // When we want to sort paths lexicographically in ascending order,
        // then we can actually do this during directory traversal itself.
        // Otherwise, sorting is done by collecting all paths, sorting them and
        // then searching them.
        if let Some(ref sort) = self.sort {
            assert_eq!(1, self.threads, "sorting implies single threaded");
            if !sort.reverse && matches!(sort.kind, SortModeKind::Path) {
                builder.sort_by_file_name(|a, b| a.cmp(b));
            }
        }
        Ok(builder)
    }
```
- LiteralStrategy (crates/globset/src/lib.rs:722-726)
```rust
struct LiteralStrategy {
    literals: fnv::HashMap<Vec<u8>, Vec<usize>>,
    // Literal byte lengths modulo 64, checked before hashing the path.
    lengths: u64,
}
```
- WalkBuilder (crates/ignore/src/walk.rs:488-512)
```rust
pub struct WalkBuilder {
    paths: Vec<PathBuf>,
    ig_builder: IgnoreBuilder,
    max_depth: Option<usize>,
    min_depth: Option<usize>,
    max_filesize: Option<u64>,
    follow_links: bool,
    same_file_system: bool,
    sorter: Option<Sorter>,
    threads: usize,
    skip: Option<Arc<Handle>>,
    filter: Option<Filter>,
    /// The directory that gitignores should be interpreted relative to.
    ///
    /// Usually this is the directory containing the gitignore file. But in
    /// some cases, like for global gitignores or for gitignores specified
    /// explicitly, this should generally be set to the current working
    /// directory. This is only used for global gitignores or "explicit"
    /// gitignores.
    ///
    /// When `None`, the CWD is fetched from `std::env::current_dir()`. If
    /// that fails, then global gitignores are ignored (an error is logged).
    global_gitignores_relative_to:
        OnceLock<Result<PathBuf, Arc<std::io::Error>>>,
}
```
- match_single_file_glob (crates/ignore/benches/workloads.rs:487-505)
```rust
fn match_single_file_glob(b: &mut test::Bencher) {
    let mut builder = GitignoreBuilder::new("");
    builder.add_line(None, "*.pyc").unwrap();
    let matcher = builder.build().unwrap();
    let paths = [
        ("module.py", false),
        ("module.pyc", true),
        ("src/module.py", false),
        ("src/__pycache__/module.pyc", true),
    ];
    b.iter(|| {
        for (path, ignored) in paths {
            assert_eq!(
                matcher.matched(black_box(path), false).is_ignore(),
                ignored
            );
        }
    });
}
```
- builders (crates/ignore/src/incremental.rs:597-603)
```rust
    fn builders<I: IntoIterator<Item = P>, P: AsRef<Path>>(
        paths: I,
    ) -> WalkBuilder {
        let mut builder = WalkBuilder::from_iter(paths);
        builder.git_global(false);
        builder
    }
```
- descendants_of_hidden_directories_are_ignored (crates/ignore/src/incremental.rs:1236-1250)
```rust
    fn descendants_of_hidden_directories_are_ignored() {
        let td = tmpdir();
        mkdirp(td.path().join(".git"));
        mkdirp(td.path().join(".hidden/nested"));
        mkdirp(td.path().join("visible/.hidden"));
        wfile(td.path().join(".hidden/file"), "");
        wfile(td.path().join(".hidden/nested/file"), "");
        wfile(td.path().join("visible/.hidden/file"), "");
        wfile(td.path().join(".hidden/.gitignore"), "!file\n");

        let mut m = one_matcher(&builder(td.path()));
        assert!(matchedf(&mut m, ".hidden/file").is_ignore());
        assert!(matchedf(&mut m, ".hidden/nested/file").is_ignore());
        assert!(matchedf(&mut m, "visible/.hidden/file").is_ignore());
    }
```
- IgnoreBuilder (crates/ignore/src/dir.rs:758-781)
```rust
pub(crate) struct IgnoreBuilder {
    /// The root directory path for this ignore matcher.
    dir: PathBuf,
    /// An override matcher (default is empty).
    overrides: Arc<Override>,
    /// A type matcher (default is empty).
    types: Arc<Types>,
    /// Explicit global ignore matchers.
    explicit_ignores: Vec<Gitignore>,
    /// Ignore files in addition to .ignore.
    custom_ignore_filenames: Vec<OsString>,
    /// The directory that gitignores should be interpreted relative to.
    ///
    /// Usually this is the directory containing the gitignore file. But in
    /// some cases, like for global gitignores or for gitignores specified
    /// explicitly, this should generally be set to the current working
    /// directory. This is only used for global gitignores or "explicit"
    /// gitignores.
    ///
    /// When `None`, global gitignores are ignored.
    global_gitignores_relative_to: Option<PathBuf>,
    /// Ignore config.
    opts: IgnoreOptions,
}
```
- IncrementalIgnore.new (crates/ignore/src/incremental.rs:106-131)
```rust
    pub(crate) fn new(
        root: PathBuf,
        ignore: Ignore,
        options: IncrementalIgnoreOptions,
    ) -> IncrementalIgnore {
        // File traversal special cases `-` to search stdin, so we recognize
        // it here for completeness too. In particular, we really want
        // `WalkBuilder::build_matchers` to return a matcher for every root,
        // even when it's a simple file (handled automatically) or when it's
        // stdin (necessarily special cased).
        //
        // If callers need to search a file or directory named `-`, then they
        // can use `./-`. As is the case for file traversal too.
        let ignore = if root == Path::new("-") {
            RootIgnore::Stdin
        } else {
            RootIgnore::Unloaded(ignore)
        };
        IncrementalIgnore {
            root,
            normalized_root: OnceLock::new(),
            ignore,
            dirs: HashMap::new(),
            options,
        }
    }
```
- Ignore (crates/ignore/src/dir.rs:94-112)
```rust
pub(crate) struct Ignore {
    inner: Arc<IgnoreInner>,
    // Parent matchers are cached independently of the path being walked, but
    // matching them still needs the canonicalized path originally passed to
    // `add_parents`. For example, when walking `/tmp/project/src`, parent
    // matchers use `/tmp/project/src` to rewrite `/tmp/project/src/foo.py`
    // before matching it against ignore files from `/tmp/project` and its
    // ancestors.
    //
    // For ripgrep itself, this means that `rg pat src tests` must rewrite
    // `src/foo` relative to `.../src`, and not whatever root was prepared
    // first.
    //
    // See: https://github.com/BurntSushi/ripgrep/pull/3420
    // See: https://github.com/BurntSushi/ripgrep/issues/3376
    // See: https://github.com/BurntSushi/ripgrep/issues/3419
    // See: https://github.com/BurntSushi/ripgrep/issues/3320
    absolute_base: Option<Arc<PathBuf>>,
}
```
- gitignore_parent (crates/ignore/src/walk.rs:2436-2446)
```rust
    fn gitignore_parent() {
        let td = tmpdir();
        mkdirp(td.path().join(".git"));
        mkdirp(td.path().join("a"));
        wfile(td.path().join(".gitignore"), "foo");
        wfile(td.path().join("a/foo"), "");
        wfile(td.path().join("a/bar"), "");

        let root = td.path().join("a");
        assert_paths(&root, &WalkBuilder::new(&root), &["bar"]);
    }
```
- incremental_gitignore_only (crates/ignore/benches/workloads.rs:563-576)
```rust
fn incremental_gitignore_only(b: &mut test::Bencher) {
    let ws = Workspace::new(16, 4, 4, false);
    fs::remove_file(ws.root.join(".ignore")).unwrap();
    fs::remove_file(ws.root.join(".git/info/exclude")).unwrap();
    let mut matchers = ws.builder().build_matchers();
    for path in &ws.paths {
        assert!(!matchers[0].matched(path, false).is_ignore());
    }
    b.iter(|| {
        for path in &ws.paths {
            assert!(!matchers[0].matched(black_box(path), false).is_ignore());
        }
    });
}
```
- incremental_ignored (crates/ignore/benches/workloads.rs:293-301)
```rust
fn incremental_ignored(b: &mut test::Bencher) {
    let ws = Workspace::new(0, 0, 0, false);
    let mut matchers = ws.builder().build_matchers();
    let path = Path::new(".venv/lib/python3.13/site-packages/dependency.py");
    assert!(matchers[0].matched(path, false).is_ignore());
    b.iter(|| {
        assert!(matchers[0].matched(black_box(path), false).is_ignore())
    });
}
```
- WalkBuilder.build_matchers (crates/ignore/src/walk.rs:675-694)
```rust
    pub fn build_matchers(&self) -> Vec<IncrementalIgnore> {
        let ignore = self.build_ignore();
        let options = IncrementalIgnoreOptions {
            min_depth: self.min_depth,
            max_depth: self.max_depth,
            max_filesize: self.max_filesize,
