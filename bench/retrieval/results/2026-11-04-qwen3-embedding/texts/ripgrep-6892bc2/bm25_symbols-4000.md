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
