- respects_git_repository_boundaries (crates/ignore/src/incremental.rs:788-806)
```rust
    fn respects_git_repository_boundaries() {
        let td = tmpdir();
        let root = td.path().join("repo/src");
        mkdirp(td.path().join("repo/.git"));
        mkdirp(&root);
        wfile(td.path().join(".gitignore"), "outside-rule\n");
        wfile(td.path().join(".ignore"), "tool-rule\n");
        wfile(td.path().join("repo/.gitignore"), "inside-rule\n");

        let mut m = one_matcher(&builder(&root));
        assert!(matchedf(&mut m, "inside-rule").is_ignore());
        assert!(matchedf(&mut m, "outside-rule").is_none());
        assert!(matchedf(&mut m, "tool-rule").is_ignore());

        let mut no_git_required = builder(&root);
        no_git_required.require_git(false);
        let mut m = one_matcher(&no_git_required);
        assert!(matchedf(&mut m, "outside-rule").is_ignore());
    }
```
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
- NoRequireGit.doc_long (crates/core/flags/defs.rs:4850-4863)
```rust
    fn doc_long(&self) -> &'static str {
        r"
When this flag is given, source control ignore files such as \fB.gitignore\fP
are respected even if no \fBgit\fP repository is present.
.sp
By default, ripgrep will only respect filter rules from source control ignore
files when ripgrep detects that the search is executed inside a source control
repository. For example, when a \fB.git\fP directory is observed.
.sp
This flag relaxes the default restriction. For example, it might be useful when
the contents of a \fBgit\fP repository are stored or copied somewhere, but
where the repository state is absent.
"
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
- NoIgnoreVcs.doc_long (crates/core/flags/defs.rs:4699-4714)
```rust
    fn doc_long(&self) -> &'static str {
        r"
When given, filter rules from source control ignore files (e.g.,
\fB.gitignore\fP) are not respected. By default, ripgrep respects \fBgit\fP's
ignore rules for automatic filtering. In some cases, it may not be desirable
to respect the source control's ignore rules and instead only respect rules in
\fB.ignore\fP or \fB.rgignore\fP.
.sp
Note that this flag does not directly affect the filtering of source control
files or folders that start with a dot (\fB.\fP), like \fB.git\fP. These are
affected by \flag{hidden} and its related flags instead.
.sp
This flag implies \flag{no-ignore-parent} for source control ignore files as
well.
"
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
- multiple_roots_keep_their_own_context (crates/ignore/src/incremental.rs:1009-1032)
```rust
    fn multiple_roots_keep_their_own_context() {
        let td = tmpdir();
        let root_a = td.path().join("a");
        let root_b = td.path().join("b");
        mkdirp(td.path().join(".git"));
        mkdirp(&root_a);
        mkdirp(&root_b);
        wfile(td.path().join(".gitignore"), "/a/*.tmp\n/b/*.log\n");

        let mut builder =
            builders([root_a.as_path(), Path::new("-"), root_b.as_path()]);
        builder.git_global(false);
        let mut ms = matchers(&builder);
        assert_eq!(ms.len(), 3);
        assert_eq!(ms[0].root(), root_a);
        assert_eq!(ms[1].root(), Path::new("-"));
        assert_eq!(ms[2].root(), root_b);
        assert!(matchedf(&mut ms[0], "drop.tmp").is_ignore());
        assert!(matchedf(&mut ms[0], "keep.log").is_none());
        assert!(matchedf(&mut ms[1], "anything").is_none());
        assert_eq!(ms[1].normalize("anything"), None);
        assert!(matchedf(&mut ms[2], "keep.tmp").is_none());
        assert!(matchedf(&mut ms[2], "drop.log").is_ignore());
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
- Ignore.add_parents (crates/ignore/src/dir.rs:192-254)
```rust
    pub(crate) fn add_parents<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> (Ignore, Option<Error>) {
        if !self.inner.opts.parents
            && !self.inner.opts.git_ignore
            && !self.inner.opts.git_exclude
            && !self.inner.opts.git_global
        {
            // If we never need info from parent directories, then don't do
            // anything.
            return (self.clone(), None);
        }
        if !self.is_root() {
            panic!("Ignore::add_parents called on non-root matcher");
        }
        let absolute_base = match path.as_ref().canonicalize() {
            Ok(path) => Arc::new(path),
            Err(_) => {
                // There's not much we can do here, so just return our
                // existing matcher. We drop the error to be consistent
                // with our general pattern of ignoring I/O errors when
                // processing ignore files.
                return (self.clone(), None);
            }
        };
        // List of parents, from child to root.
        let mut parents = vec![];
        let mut path = &**absolute_base;
        while let Some(parent) = path.parent() {
            parents.push(parent);
            path = parent;
        }
        let mut errs = PartialErrorBuilder::default();
        let mut ig = self.clone();
        for parent in parents.into_iter().rev() {
            let mut compiled = self.inner.compiled.write().unwrap();
            if let Some(weak) = compiled.get(parent.as_os_str()) {
                if let Some(prebuilt) = weak.upgrade() {
                    ig = Ignore {
                        inner: prebuilt,
                        absolute_base: Some(absolute_base.clone()),
                    };
                    continue;
                }
            }
            let (mut igtmp, err) = ig.add_child_path(parent);
            errs.maybe_push(err);
            igtmp.is_absolute_parent = true;
            // `add_child_path` already checked repository markers when needed.
            igtmp.has_git &= self.inner.opts.git_ignore;
            let ig_arc = Arc::new(igtmp);
            ig = Ignore {
                inner: ig_arc.clone(),
                absolute_base: Some(absolute_base.clone()),
            };
            compiled.insert(
                parent.as_os_str().to_os_string(),
                Arc::downgrade(&ig_arc),
            );
        }
        (ig, errs.into_error_option())
    }
```
- absolute_parent_matchers_are_cached_across_roots (crates/ignore/src/dir.rs:1427-1447)
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
- incremental_ignored_nested (crates/ignore/benches/workloads.rs:366-377)
```rust
fn incremental_ignored_nested(b: &mut test::Bencher) {
    let ws = Workspace::new(1, 1, 1, false);
    let path = Path::new(
        "packages/pkg0/level0/.venv/lib/python3.13/site-packages/dependency.py",
    );
    ws.write(path.to_str().unwrap(), "");
    let mut matchers = ws.builder().build_matchers();
    assert!(matchers[0].matched(path, false).is_ignore());
    b.iter(|| {
        assert!(matchers[0].matched(black_box(path), false).is_ignore())
    });
}
```
- partial_errors_keep_valid_rules (crates/ignore/src/incremental.rs:941-953)
```rust
    fn partial_errors_keep_valid_rules() {
        let td = tmpdir();
        let root = td.path().join("work");
        mkdirp(&root);
        wfile(td.path().join(".ignore"), "{bad\n*.tmp\n");

        let mut builder = builder(&root);
        builder.git_ignore(false).git_exclude(false);
        let mut m = one_matcher(&builder);
        let (matched, err) = m.matched_with_errors("drop.tmp", false);
        assert!(err.is_some());
        assert!(matched.is_ignore());
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
- IgnoreInner (crates/ignore/src/dir.rs:115-166)
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
- IncrementalIgnore.normalize (crates/ignore/src/incremental.rs:155-165)
```rust
    pub fn normalize<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf> {
        if matches!(self.ignore, RootIgnore::Stdin) {
            return None;
        }
        let path = normalize_absolute(path.as_ref())?;
        let root = self
            .normalized_root
            .get_or_init(|| normalize_absolute(&self.root))
            .as_ref()?;
        path.strip_prefix(root).ok().map(Path::to_path_buf)
    }
```
- compiled_multi_matchers_are_reused (crates/ignore/src/incremental.rs:881-898)
```rust
    fn compiled_multi_matchers_are_reused() {
        let td = tmpdir();
        mkdirp(td.path().join("a/b/c"));

        wfile(td.path().join("a/.ignore"), "*.tmp\n");
        let mut mats = matchers(&builders([
            td.path().join("a/b"),
            td.path().join("a/b/c"),
        ]));
        assert!(matchedf(&mut mats[0], "first.tmp").is_ignore());
        assert!(matchedf(&mut mats[0], "c/first.tmp").is_ignore());

        // Even though we haven't used the second matcher, it should still
        // reuse the `a/.ignore` above. If it didn't, then `a/b/c/first.tmp`
        // below would be whitelisted.
        wfile(td.path().join("a/.ignore"), "!*.tmp\n");
        assert!(matchedf(&mut mats[1], "first.tmp").is_ignore());
    }
```
- matchers (crates/ignore/src/incremental.rs:605-607)
```rust
    fn matchers(builder: &WalkBuilder) -> Vec<IncrementalIgnore> {
        builder.build_matchers()
    }
```
- nested_parent_gitignores (crates/ignore/src/incremental.rs:629-643)
```rust
    fn nested_parent_gitignores() {
        let td = tmpdir();
        let root = td.path().join("project/work");
        mkdirp(td.path().join(".git"));
        mkdirp(root.join("src"));
        wfile(td.path().join(".gitignore"), "*.tmp\n");
        wfile(td.path().join("project/.gitignore"), "!keep.tmp\nnested.log\n");

        let mut m = one_matcher(&builder(&root));
        assert_eq!(m.root(), root);
        assert!(matchedf(&mut m, "src/drop.tmp").is_ignore());
        assert!(matchedf(&mut m, "src/keep.tmp").is_whitelist());
        assert!(matchedf(&mut m, "src/nested.log").is_ignore());
        assert!(matchedf(&mut m, "src/ok.rs").is_none());
    }
```
- no_read_permissions (crates/ignore/src/walk.rs:2658-2673)
```rust
    fn no_read_permissions() {
        let dir_path = Path::new("/root");

        // There's no /etc/sudoers.d, skip the test.
        if !dir_path.is_dir() {
            return;
        }
        // We're the root, so the test won't check what we want it to.
        if fs::read_dir(&dir_path).is_ok() {
            return;
        }

        // Check that we can't descend but get an entry for the parent dir.
        let builder = WalkBuilder::new(&dir_path);
        assert_paths(dir_path.parent().unwrap(), &builder, &["root"]);
    }
```
- IgnoreOptions (crates/ignore/src/dir.rs:72-90)
```rust
struct IgnoreOptions {
    /// Whether to ignore hidden file paths or not.
    hidden: bool,
    /// Whether to read .ignore files.
    ignore: bool,
    /// Whether to respect any ignore files in parent directories.
    parents: bool,
    /// Whether to read git's global gitignore file.
    git_global: bool,
    /// Whether to read .gitignore files.
    git_ignore: bool,
    /// Whether to read .git/info/exclude files.
    git_exclude: bool,
    /// Whether to ignore files case insensitively
    ignore_case_insensitive: bool,
    /// Whether a git repository must be present in order to apply any
    /// git-related ignore rules.
    require_git: bool,
}
```
- parent_loading_can_be_disabled (crates/ignore/src/incremental.rs:958-976)
```rust
    fn parent_loading_can_be_disabled() {
        let td = tmpdir();
        let root = td.path().join("work");
        mkdirp(&root);
        wfile(td.path().join(".ignore"), "parent-rule\n");
        wfile(root.join(".ignore"), "root-rule\n");

        let mut b = builder(&root);
        b.parents(false);
        let mut m = one_matcher(&b);
        assert!(matchedf(&mut m, "parent-rule").is_none());
        assert!(matchedf(&mut m, "root-rule").is_ignore());

        // Sanity check that without `parents(false)`, the parent rule is
        // respected.
        let mut m = one_matcher(&builder(&root));
        assert!(matchedf(&mut m, "parent-rule").is_ignore());
        assert!(matchedf(&mut m, "root-rule").is_ignore());
    }
```
- Workspace.builder (crates/ignore/benches/workloads.rs:94-99)
```rust
    fn builder(&self) -> WalkBuilder {
        let mut builder = WalkBuilder::new(&self.root);
        // Keep parent discovery, but do not depend on the user's Git config.
        builder.current_dir(&self.root).hidden(false).git_global(false);
        builder
    }
```
- one_matcher (crates/ignore/src/incremental.rs:591-595)
```rust
    fn one_matcher(builder: &WalkBuilder) -> IncrementalIgnore {
        let mut matchers = builder.build_matchers();
        assert_eq!(matchers.len(), 1);
        matchers.pop().unwrap()
    }
```
- Work.add_parents (crates/ignore/src/walk.rs:1591-1600)
```rust
    fn add_parents(&mut self) -> Option<Error> {
        if self.dent.depth() > 0 {
            return None;
        }
        // At depth 0, the path of this entry is a root path, so we can
        // use it directly to add parent ignore rules.
        let (ig, err) = self.ignore.add_parents(self.dent.path());
        self.ignore = ig;
        err
    }
```
- multi_root_gitignore_order_independent (crates/ignore/src/dir.rs:1460-1516)
```rust
    fn multi_root_gitignore_order_independent() {
        let td = tmpdir();
        let cwd = std::env::current_dir().unwrap();
        // Use paths relative to CWD like the CLI walk does for `rg pat src tests`.
        let root = td.path().strip_prefix(&cwd).unwrap_or(td.path());
        let src_root = root.join("src");
        let tests_root = root.join("tests");

        mkdirp(td.path().join(".git"));
        mkdirp(td.path().join("src"));
        mkdirp(td.path().join("tests"));
        wfile(td.path().join(".gitignore"), "src/invalid\n");
        wfile(td.path().join("src/invalid"), "x");
        wfile(td.path().join("src/valid"), "x");
        wfile(td.path().join("tests/valid"), "x");

        let ig0 = IgnoreBuilder::new().build();

        // Historically buggy order: search `src` first, then `tests`.
        let (src_parents, err) = ig0.add_parents(&src_root);
        assert!(err.is_none());
        let (src, err) = src_parents.add_child(&src_root);
        assert!(err.is_none());
        let (tests_parents, err) = ig0.add_parents(&tests_root);
        assert!(err.is_none());
        let (tests, err) = tests_parents.add_child(&tests_root);
        assert!(err.is_none());

        assert!(Arc::ptr_eq(&src_parents.inner, &tests_parents.inner));
        // Each root must carry its own absolute_base even though inners are shared.
        assert_ne!(
            src.absolute_base.as_ref().unwrap().as_path(),
            tests.absolute_base.as_ref().unwrap().as_path()
        );
        assert!(
            src.matched(src_root.join("invalid"), false).is_ignore(),
            "parent .gitignore must apply for the src root even when \
             another root was prepared in the same process"
        );
        assert!(src.matched(src_root.join("valid"), false).is_none());
        assert!(tests.matched(tests_root.join("valid"), false).is_none());

        // Reverse order should behave the same way.
        let ig0 = IgnoreBuilder::new().build();
