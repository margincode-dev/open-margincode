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
- test_no_ignore_parent (crates/core/flags/defs.rs:4667-4677)
```rust
fn test_no_ignore_parent() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.no_ignore_parent);

    let args = parse_low_raw(["--no-ignore-parent"]).unwrap();
    assert_eq!(true, args.no_ignore_parent);

    let args =
        parse_low_raw(["--no-ignore-parent", "--ignore-parent"]).unwrap();
    assert_eq!(false, args.no_ignore_parent);
}
```
- Ignore.parent (crates/ignore/src/dir.rs:181-186)
```rust
    pub(crate) fn parent(&self) -> Option<Ignore> {
        self.inner.parent.as_ref().map(|parent| Ignore {
            inner: parent.clone(),
            absolute_base: self.absolute_base.clone(),
        })
    }
```
- test_dirs_in_deep (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:193-318)
```rust
fn test_dirs_in_deep() {
    let gitignore = get_gitignore();
    let m = |path: &str, is_dir: bool| {
        gitignore.matched_path_or_any_parents(Path::new(path), is_dir)
    };

    // 00
    assert!(m("ROOT/parent_dir/dir_deep_00", true).is_ignore());
    assert!(m("ROOT/parent_dir/dir_deep_00/file", false).is_ignore());
    assert!(m("ROOT/parent_dir/dir_deep_00/child_dir", true).is_ignore());
    assert!(
        m("ROOT/parent_dir/dir_deep_00/child_dir/file", false).is_ignore()
    );

    // 01
    assert!(m("ROOT/parent_dir/dir_deep_01", true).is_ignore());
    assert!(m("ROOT/parent_dir/dir_deep_01/file", false).is_ignore());
    assert!(m("ROOT/parent_dir/dir_deep_01/child_dir", true).is_ignore());
    assert!(
        m("ROOT/parent_dir/dir_deep_01/child_dir/file", false).is_ignore()
    );

    // 02
    assert!(m("ROOT/parent_dir/dir_deep_02", true).is_none());
    assert!(m("ROOT/parent_dir/dir_deep_02/file", false).is_none());
    assert!(m("ROOT/parent_dir/dir_deep_02/child_dir", true).is_none());
    assert!(m("ROOT/parent_dir/dir_deep_02/child_dir/file", false).is_none());

    // 03
    assert!(m("ROOT/parent_dir/dir_deep_03", true).is_none());
    assert!(m("ROOT/parent_dir/dir_deep_03/file", false).is_none());
    assert!(m("ROOT/parent_dir/dir_deep_03/child_dir", true).is_none());
    assert!(m("ROOT/parent_dir/dir_deep_03/child_dir/file", false).is_none());

    // 10
    assert!(m("ROOT/parent_dir/dir_deep_10", true).is_none());
    assert!(m("ROOT/parent_dir/dir_deep_10/file", false).is_none());
    assert!(m("ROOT/parent_dir/dir_deep_10/child_dir", true).is_none());
    assert!(m("ROOT/parent_dir/dir_deep_10/child_dir/file", false).is_none());

    // 11
    assert!(m("ROOT/parent_dir/dir_deep_11", true).is_none());
    assert!(m("ROOT/parent_dir/dir_deep_11/file", false).is_none());
    assert!(m("ROOT/parent_dir/dir_deep_11/child_dir", true).is_none());
    assert!(m("ROOT/parent_dir/dir_deep_11/child_dir/file", false).is_none());

    // 12
    assert!(m("ROOT/parent_dir/dir_deep_12", true).is_none());
    assert!(m("ROOT/parent_dir/dir_deep_12/file", false).is_none());
    assert!(m("ROOT/parent_dir/dir_deep_12/child_dir", true).is_none());
    assert!(m("ROOT/parent_dir/dir_deep_12/child_dir/file", false).is_none());

    // 13
    assert!(m("ROOT/parent_dir/dir_deep_13", true).is_none());
    assert!(m("ROOT/parent_dir/dir_deep_13/file", false).is_none());
    assert!(m("ROOT/parent_dir/dir_deep_13/child_dir", true).is_none());
    assert!(m("ROOT/parent_dir/dir_deep_13/child_dir/file", false).is_none());

    // 20
    assert!(m("ROOT/parent_dir/dir_deep_20", true).is_ignore());
    assert!(m("ROOT/parent_dir/dir_deep_20/file", false).is_ignore());
    assert!(m("ROOT/parent_dir/dir_deep_20/child_dir", true).is_ignore());
    assert!(
        m("ROOT/parent_dir/dir_deep_20/child_dir/file", false).is_ignore()
    );

    // 21
    assert!(m("ROOT/parent_dir/dir_deep_21", true).is_ignore());
    assert!(m("ROOT/parent_dir/dir_deep_21/file", false).is_ignore());
    assert!(m("ROOT/parent_dir/dir_deep_21/child_dir", true).is_ignore());
    assert!(
        m("ROOT/parent_dir/dir_deep_21/child_dir/file", false).is_ignore()
    );

    // 22
    // dir itself doesn't match
    assert!(m("ROOT/parent_dir/dir_deep_22", true).is_none());
    assert!(m("ROOT/parent_dir/dir_deep_22/file", false).is_ignore());
    assert!(m("ROOT/parent_dir/dir_deep_22/child_dir", true).is_ignore());
    assert!(
        m("ROOT/parent_dir/dir_deep_22/child_dir/file", false).is_ignore()
    );

    // 23
    // dir itself doesn't match
    assert!(m("ROOT/parent_dir/dir_deep_23", true).is_none());
    assert!(m("ROOT/parent_dir/dir_deep_23/file", false).is_ignore());
    assert!(m("ROOT/parent_dir/dir_deep_23/child_dir", true).is_ignore());
    assert!(
        m("ROOT/parent_dir/dir_deep_23/child_dir/file", false).is_ignore()
    );

    // 30
    assert!(m("ROOT/parent_dir/dir_deep_30", true).is_ignore());
    assert!(m("ROOT/parent_dir/dir_deep_30/file", false).is_ignore());
    assert!(m("ROOT/parent_dir/dir_deep_30/child_dir", true).is_ignore());
    assert!(
        m("ROOT/parent_dir/dir_deep_30/child_dir/file", false).is_ignore()
    );

    // 31
    assert!(m("ROOT/parent_dir/dir_deep_31", true).is_ignore());
    assert!(m("ROOT/parent_dir/dir_deep_31/file", false).is_ignore());
    assert!(m("ROOT/parent_dir/dir_deep_31/child_dir", true).is_ignore());
    assert!(
        m("ROOT/parent_dir/dir_deep_31/child_dir/file", false).is_ignore()
    );

    // 32
    // dir itself doesn't match
    assert!(m("ROOT/parent_dir/dir_deep_32", true).is_none());
    assert!(m("ROOT/parent_dir/dir_deep_32/file", false).is_ignore());
    assert!(m("ROOT/parent_dir/dir_deep_32/child_dir", true).is_ignore());
    assert!(
        m("ROOT/parent_dir/dir_deep_32/child_dir/file", false).is_ignore()
    );

    // 33
    // dir itself doesn't match
    assert!(m("ROOT/parent_dir/dir_deep_33", true).is_none());
    assert!(m("ROOT/parent_dir/dir_deep_33/file", false).is_ignore());
    assert!(m("ROOT/parent_dir/dir_deep_33/child_dir", true).is_ignore());
    assert!(
        m("ROOT/parent_dir/dir_deep_33/child_dir/file", false).is_ignore()
    );
}
```
- IgnoreRef.is_absolute_parent (crates/ignore/src/dir.rs:725-727)
```rust
    pub(crate) fn is_absolute_parent(&self) -> bool {
        self.inner.is_absolute_parent
    }
```
- NoIgnoreParent (crates/core/flags/defs.rs:4632-4632)
```rust
struct NoIgnoreParent;
```
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
- NoIgnoreParent.name_long (crates/core/flags/defs.rs:4638-4640)
```rust
    fn name_long(&self) -> &'static str {
        "no-ignore-parent"
    }
```
- NoIgnoreParent.name_negated (crates/core/flags/defs.rs:4641-4643)
```rust
    fn name_negated(&self) -> Option<&'static str> {
        Some("ignore-parent")
    }
```
- test_files_in_deep (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:57-86)
```rust
fn test_files_in_deep() {
    let gitignore = get_gitignore();
    let m = |path: &str| {
        gitignore.matched_path_or_any_parents(Path::new(path), false)
    };

    // 0x
    assert!(m("ROOT/parent_dir/file_deep_00").is_ignore());
    assert!(m("ROOT/parent_dir/file_deep_01").is_none());
    assert!(m("ROOT/parent_dir/file_deep_02").is_none());
    assert!(m("ROOT/parent_dir/file_deep_03").is_none());

    // 1x
    assert!(m("ROOT/parent_dir/file_deep_10").is_none());
    assert!(m("ROOT/parent_dir/file_deep_11").is_none());
    assert!(m("ROOT/parent_dir/file_deep_12").is_none());
    assert!(m("ROOT/parent_dir/file_deep_13").is_none());

    // 2x
    assert!(m("ROOT/parent_dir/file_deep_20").is_ignore());
    assert!(m("ROOT/parent_dir/file_deep_21").is_none());
    assert!(m("ROOT/parent_dir/file_deep_22").is_none());
    assert!(m("ROOT/parent_dir/file_deep_23").is_none());

    // 3x
    assert!(m("ROOT/parent_dir/file_deep_30").is_ignore());
    assert!(m("ROOT/parent_dir/file_deep_31").is_none());
    assert!(m("ROOT/parent_dir/file_deep_32").is_none());
    assert!(m("ROOT/parent_dir/file_deep_33").is_none());
}
```
- NoIgnoreParent.doc_short (crates/core/flags/defs.rs:4647-4649)
```rust
    fn doc_short(&self) -> &'static str {
        r"Don't use ignore files in parent directories."
    }
```
- NoIgnoreExclude.doc_long (crates/core/flags/defs.rs:4460-4465)
```rust
    fn doc_long(&self) -> &'static str {
        r"
Don't respect filter rules from files that are manually configured for the repository.
For example, this includes \fBgit\fP's \fB.git/info/exclude\fP.
"
    }
```
- NoIgnoreParent.update (crates/core/flags/defs.rs:4659-4662)
```rust
    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.no_ignore_parent = v.unwrap_switch();
        Ok(())
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
- Ignore.add_parents (crates/ignore/src/dir.rs:192-258)
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
            igtmp.has_git =
                if self.inner.opts.require_git && self.inner.opts.git_ignore {
                    parent.join(".git").exists() || parent.join(".jj").exists()
                } else {
                    false
                };
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
- NoIgnoreParent.doc_long (crates/core/flags/defs.rs:4650-4657)
```rust
    fn doc_long(&self) -> &'static str {
        r"
When this flag is set, filter rules from ignore files found in parent
directories are not respected. By default, ripgrep will ascend the parent
directories of the current working directory to look for any applicable ignore
files that should be applied. In some cases this may not be desirable.
"
    }
```
- absolute_parent (crates/ignore/src/dir.rs:1388-1408)
```rust
    fn absolute_parent() {
        let td = tmpdir();
        mkdirp(td.path().join(".git"));
        mkdirp(td.path().join("foo"));
        wfile(td.path().join(".gitignore"), "bar");

        // First, check that the parent gitignore file isn't detected if the
        // parent isn't added. This establishes a baseline.
        let ig0 = IgnoreBuilder::new().build();
        let (ig1, err) = ig0.add_child(td.path().join("foo"));
        assert!(err.is_none());
        assert!(ig1.matched("bar", false).is_none());

        // Second, check that adding a parent directory actually works.
        let ig0 = IgnoreBuilder::new().build();
        let (ig1, err) = ig0.add_parents(td.path().join("foo"));
        assert!(err.is_none());
        let (ig2, err) = ig1.add_child(td.path().join("foo"));
        assert!(err.is_none());
        assert!(ig2.matched("bar", false).is_ignore());
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
- Ignore.is_root (crates/ignore/src/dir.rs:176-178)
```rust
    pub(crate) fn is_root(&self) -> bool {
        self.inner.parent.is_none()
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
- NoIgnoreParent.is_switch (crates/core/flags/defs.rs:4635-4637)
```rust
    fn is_switch(&self) -> bool {
        true
    }
```
- test_no_ignore (crates/core/flags/defs.rs:4368-4389)
```rust
fn test_no_ignore() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.no_ignore_dot);
    assert_eq!(false, args.no_ignore_exclude);
    assert_eq!(false, args.no_ignore_global);
    assert_eq!(false, args.no_ignore_parent);
    assert_eq!(false, args.no_ignore_vcs);

    let args = parse_low_raw(["--no-ignore"]).unwrap();
    assert_eq!(true, args.no_ignore_dot);
    assert_eq!(true, args.no_ignore_exclude);
    assert_eq!(true, args.no_ignore_global);
    assert_eq!(true, args.no_ignore_parent);
    assert_eq!(true, args.no_ignore_vcs);

    let args = parse_low_raw(["--no-ignore", "--ignore"]).unwrap();
    assert_eq!(false, args.no_ignore_dot);
    assert_eq!(false, args.no_ignore_exclude);
    assert_eq!(false, args.no_ignore_global);
    assert_eq!(false, args.no_ignore_parent);
    assert_eq!(false, args.no_ignore_vcs);
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
- NoIgnoreParent.doc_category (crates/core/flags/defs.rs:4644-4646)
```rust
    fn doc_category(&self) -> Category {
        Category::Filter
    }
```
- Gitignore.new (crates/ignore/src/gitignore.rs:103-118)
```rust
    pub fn new<P: AsRef<Path>>(
        gitignore_path: P,
    ) -> (Gitignore, Option<Error>) {
        let path = gitignore_path.as_ref();
        let parent = path.parent().unwrap_or(Path::new("/"));
        let mut builder = GitignoreBuilder::new(parent);
        let mut errs = PartialErrorBuilder::default();
        errs.maybe_push_ignore_io(builder.add(path));
        match builder.build() {
            Ok(gi) => (gi, errs.into_error_option()),
            Err(err) => {
                errs.push(err);
                (Gitignore::empty(), errs.into_error_option())
            }
        }
    }
```
- Config (crates/core/search.rs:19-25)
```rust
struct Config {
    preprocessor: Option<std::path::PathBuf>,
    preprocessor_globs: ignore::overrides::Override,
    search_zip: bool,
    binary_implicit: grep::searcher::BinaryDetection,
    binary_explicit: grep::searcher::BinaryDetection,
}
```
- Config.default (crates/core/search.rs:28-36)
```rust
    fn default() -> Config {
        Config {
            preprocessor: None,
            preprocessor_globs: ignore::overrides::Override::empty(),
            search_zip: false,
            binary_implicit: grep::searcher::BinaryDetection::none(),
            binary_explicit: grep::searcher::BinaryDetection::none(),
        }
    }
```
- Searcher.set_binary_detection (crates/searcher/src/searcher/mod.rs:798-801)
```rust
    pub fn set_binary_detection(&mut self, detection: BinaryDetection) {
        self.config.binary = detection.clone();
        self.line_buffer.borrow_mut().set_binary_detection(detection.0);
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
- SearcherTester.binary_detection (crates/searcher/src/testutil.rs:400-406)
```rust
    pub(crate) fn binary_detection(
        &mut self,
        detection: BinaryDetection,
    ) -> &mut SearcherTester {
        self.binary = detection;
        self
    }
```
- Parents.next (crates/ignore/src/dir.rs:736-748)
```rust
    fn next(&mut self) -> Option<IgnoreRef<'a>> {
        match self.0.take() {
            None => None,
            Some(ig) => {
                self.0 = ig
                    .inner
                    .parent
                    .as_deref()
                    .map(|inner| IgnoreRef { inner });
                Some(ig)
            }
        }
    }
```
- SearcherBuilder.binary_detection (crates/searcher/src/searcher/mod.rs:510-516)
```rust
    pub fn binary_detection(
        &mut self,
        detection: BinaryDetection,
    ) -> &mut SearcherBuilder {
        self.config.binary = detection;
        self
    }
```
- LineBufferBuilder.binary_detection (crates/searcher/src/line_buffer.rs:196-202)
```rust
    pub(crate) fn binary_detection(
        &mut self,
        detection: BinaryDetection,
    ) -> &mut LineBufferBuilder {
        self.config.binary = detection;
        self
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
- BinaryDetection (crates/core/flags/hiargs.rs:1152-1155)
```rust
struct BinaryDetection {
    explicit: grep::searcher::BinaryDetection,
    implicit: grep::searcher::BinaryDetection,
}
```
- Gitignore.matched_path_or_any_parents (crates/ignore/src/gitignore.rs:234-256)
```rust
    pub fn matched_path_or_any_parents<P: AsRef<Path>>(
        &self,
        path: P,
        is_dir: bool,
    ) -> Match<&Glob> {
        if self.is_empty() {
            return Match::None;
        }
        let mut path = self.strip(path.as_ref());
        assert!(!path.has_root(), "path is expected to be under the root");

        match self.matched_stripped(path, is_dir) {
            Match::None => (), // walk up
            a_match => return a_match,
        }
        while let Some(parent) = path.parent() {
            match self.matched_stripped(parent, /* is_dir */ true) {
                Match::None => path = parent, // walk up
                a_match => return a_match,
            }
        }
        Match::None
    }
```
- BinaryDetection.none (crates/searcher/src/searcher/mod.rs:62-64)
```rust
    pub fn none() -> BinaryDetection {
        BinaryDetection(line_buffer::BinaryDetection::None)
    }
```
- SearchWorkerBuilder.binary_detection_implicit (crates/core/search.rs:139-145)
```rust
    pub(crate) fn binary_detection_implicit(
        &mut self,
        detection: grep::searcher::BinaryDetection,
    ) -> &mut SearchWorkerBuilder {
        self.config.binary_implicit = detection;
        self
    }
```
- SearchWorkerBuilder.binary_detection_explicit (crates/core/search.rs:155-161)
```rust
    pub(crate) fn binary_detection_explicit(
        &mut self,
        detection: grep::searcher::BinaryDetection,
    ) -> &mut SearchWorkerBuilder {
        self.config.binary_explicit = detection;
        self
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
- check_symlink_loop (crates/ignore/src/walk.rs:2038-2059)
```rust
fn check_symlink_loop(
    ig_parent: &Ignore,
    child_path: &Path,
    child_depth: usize,
) -> Result<(), Error> {
    let hchild = Handle::from_path(child_path).map_err(|err| {
        Error::from(err).with_path(child_path).with_depth(child_depth)
    })?;
    for ig in ig_parent.parents().take_while(|ig| !ig.is_absolute_parent()) {
        let h = Handle::from_path(ig.path()).map_err(|err| {
            Error::from(err).with_path(child_path).with_depth(child_depth)
        })?;
        if hchild == h {
            return Err(Error::Loop {
                ancestor: ig.path().to_path_buf(),
                child: child_path.to_path_buf(),
            }
            .with_depth(child_depth));
        }
    }
    Ok(())
}
```
- BinaryDetection.quit (crates/searcher/src/searcher/mod.rs:79-81)
```rust
    pub fn quit(binary_byte: u8) -> BinaryDetection {
        BinaryDetection(line_buffer::BinaryDetection::Quit(binary_byte))
    }
```
- BinaryDetection.convert (crates/searcher/src/searcher/mod.rs:95-97)
```rust
    pub fn convert(binary_byte: u8) -> BinaryDetection {
        BinaryDetection(line_buffer::BinaryDetection::Convert(binary_byte))
    }
```
- BinaryDetection.default (crates/searcher/src/line_buffer.rs:67-69)
```rust
    fn default() -> BinaryDetection {
        BinaryDetection::None
    }
```
- BinaryDetection (crates/searcher/src/searcher/mod.rs:55-55)
```rust
pub struct BinaryDetection(line_buffer::BinaryDetection);
```
- Searcher.binary_detection (crates/searcher/src/searcher/mod.rs:842-844)
```rust
    pub fn binary_detection(&self) -> &BinaryDetection {
        &self.config.binary
    }
```
- NoIgnore.update (crates/core/flags/defs.rs:4355-4363)
```rust
    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let yes = v.unwrap_switch();
        args.no_ignore_dot = yes;
        args.no_ignore_exclude = yes;
        args.no_ignore_global = yes;
        args.no_ignore_parent = yes;
        args.no_ignore_vcs = yes;
        Ok(())
    }
```
- nested_parent_gitignores (crates/ignore/src/incremental.rs:629-643)
```rust
    fn nested_parent_gitignores() {
        let td = tmpdir();
