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
