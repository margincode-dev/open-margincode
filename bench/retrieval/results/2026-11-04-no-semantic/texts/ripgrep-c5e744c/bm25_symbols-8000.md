- Gitignore.empty (crates/ignore/src/gitignore.rs:151-160)
```rust
    pub fn empty() -> Gitignore {
        Gitignore {
            set: GlobSet::empty(),
            root: PathBuf::from(""),
            globs: vec![],
            num_ignores: 0,
            num_whitelists: 0,
            matches: None,
        }
    }
```
- IgnoreBuilder (crates/ignore/src/dir.rs:753-776)
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
- GitignoreBuilder.build (crates/ignore/src/gitignore.rs:349-366)
```rust
    pub fn build(&self) -> Result<Gitignore, Error> {
        let nignore = self.globs.iter().filter(|g| !g.is_whitelist()).count();
        let nwhite = self.globs.iter().filter(|g| g.is_whitelist()).count();
        let set = self
            .builder
            .build()
            .map_err(|err| Error::Glob { glob: None, err: err.to_string() })?;
        Ok(Gitignore {
            set,
            root: self.root.clone(),
            globs: self.globs.clone(),
            num_ignores: nignore as u64,
            num_whitelists: nwhite as u64,
            matches: Some(Arc::new(
                Pool::with_available_parallelism_capacity(|| vec![]),
            )),
        })
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
- Work (crates/ignore/src/walk.rs:1560-1568)
```rust
struct Work {
    /// The directory entry.
    dent: DirEntry,
    /// Any ignore matchers that have been built for this directory's parents.
    ignore: Ignore,
    /// The root device number. When present, only files with the same device
    /// number should be considered.
    root_device: Option<u64>,
}
```
- Gitignore.global (crates/ignore/src/gitignore.rs:141-146)
```rust
    pub fn global() -> (Gitignore, Option<Error>) {
        match std::env::current_dir() {
            Ok(cwd) => GitignoreBuilder::new(cwd).build_global(),
            Err(err) => (Gitignore::empty(), Some(err.into())),
        }
    }
```
- IGlob.doc_long (crates/core/flags/defs.rs:3109-3118)
```rust
    fn doc_long(&self) -> &'static str {
        r"
Include or exclude files and directories for searching that match the given
glob. This always overrides any other ignore logic. Multiple glob flags may
be used. Globbing rules match \fB.gitignore\fP globs. Precede a glob with a
\fB!\fP to exclude it. If multiple globs match a file or directory, the glob
given later in the command line takes precedence. Globs used via this flag are
matched case insensitively.
"
    }
```
- GitignoreBuilder.build_global (crates/ignore/src/gitignore.rs:377-396)
```rust
    pub fn build_global(mut self) -> (Gitignore, Option<Error>) {
        match gitconfig_excludes_path() {
            None => (Gitignore::empty(), None),
            Some(path) => {
                if !path.is_file() {
                    (Gitignore::empty(), None)
                } else {
                    let mut errs = PartialErrorBuilder::default();
                    errs.maybe_push_ignore_io(self.add(path));
                    match self.build() {
                        Ok(gi) => (gi, errs.into_error_option()),
                        Err(err) => {
                            errs.push(err);
                            (Gitignore::empty(), errs.into_error_option())
                        }
                    }
                }
            }
        }
    }
```
- SearchWorker.should_preprocess (crates/core/search.rs:284-292)
```rust
    fn should_preprocess(&self, path: &Path) -> bool {
        if !self.config.preprocessor.is_some() {
            return false;
        }
        if self.config.preprocessor_globs.is_empty() {
            return true;
        }
        !self.config.preprocessor_globs.matched(path, false).is_ignore()
    }
```
- globs (crates/core/flags/hiargs.rs:1210-1231)
```rust
fn globs(
    state: &State,
    low: &LowArgs,
) -> anyhow::Result<ignore::overrides::Override> {
    if low.globs.is_empty() && low.iglobs.is_empty() {
        return Ok(ignore::overrides::Override::empty());
    }
    let mut builder = ignore::overrides::OverrideBuilder::new(&state.cwd);
    // Make all globs case insensitive with --glob-case-insensitive.
    if low.glob_case_insensitive {
        builder.case_insensitive(true).unwrap();
    }
    for glob in low.globs.iter() {
        builder.add(glob)?;
    }
    // This only enables case insensitivity for subsequent globs.
    builder.case_insensitive(true).unwrap();
    for glob in low.iglobs.iter() {
        builder.add(&glob)?;
    }
    Ok(builder.build()?)
}
```
- Gitignore (crates/ignore/src/gitignore.rs:81-88)
```rust
pub struct Gitignore {
    set: GlobSet,
    root: PathBuf,
    globs: Vec<Glob>,
    num_ignores: u64,
    num_whitelists: u64,
    matches: Option<Arc<Pool<Vec<usize>>>>,
}
```
- GitignoreBuilder (crates/ignore/src/gitignore.rs:320-326)
```rust
pub struct GitignoreBuilder {
    builder: GlobSetBuilder,
    root: PathBuf,
    globs: Vec<Glob>,
    case_insensitive: bool,
    allow_unclosed_class: bool,
}
```
- Gitignore.is_empty (crates/ignore/src/gitignore.rs:171-173)
```rust
    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }
```
- IgnoreBuilder.build_with_cwd (crates/ignore/src/dir.rs:817-862)
```rust
    pub(crate) fn build_with_cwd(&self, cwd: Option<PathBuf>) -> Ignore {
        let global_gitignores_relative_to =
            cwd.or_else(|| self.global_gitignores_relative_to.clone());
        let git_global_matcher = if !self.opts.git_global {
            Gitignore::empty()
        } else if let Some(ref cwd) = global_gitignores_relative_to {
            let mut builder = GitignoreBuilder::new(cwd);
            builder
                .case_insensitive(self.opts.ignore_case_insensitive)
                .unwrap();
            let (gi, err) = builder.build_global();
            if let Some(err) = err {
                log::debug!("{}", err);
            }
            gi
        } else {
            log::debug!(
                "ignoring global gitignore file because CWD is not known"
            );
            Gitignore::empty()
        };

        Ignore {
            inner: Arc::new(IgnoreInner {
                compiled: Arc::new(RwLock::new(HashMap::new())),
                dir: self.dir.clone(),
                overrides: self.overrides.clone(),
                types: self.types.clone(),
                parent: None,
                is_absolute_parent: true,
                global_gitignores_relative_to,
                explicit_ignores: Arc::new(self.explicit_ignores.clone()),
                custom_ignore_filenames: Arc::new(
                    self.custom_ignore_filenames.clone(),
                ),
                custom_ignore_matcher: Gitignore::empty(),
                ignore_matcher: Gitignore::empty(),
                git_global_matcher: Arc::new(git_global_matcher),
                git_ignore_matcher: Gitignore::empty(),
                git_exclude_matcher: Gitignore::empty(),
                has_git: false,
                opts: self.opts,
            }),
            absolute_base: None,
        }
    }
```
- Override.empty (crates/ignore/src/overrides.rs:51-53)
```rust
    pub fn empty() -> Override {
        Override(Gitignore::empty())
    }
```
- preprocessor_globs (crates/core/flags/hiargs.rs:1234-1246)
```rust
fn preprocessor_globs(
    state: &State,
    low: &LowArgs,
) -> anyhow::Result<ignore::overrides::Override> {
    if low.pre_glob.is_empty() {
        return Ok(ignore::overrides::Override::empty());
    }
    let mut builder = ignore::overrides::OverrideBuilder::new(&state.cwd);
    for glob in low.pre_glob.iter() {
        builder.add(glob)?;
    }
    Ok(builder.build()?)
}
```
- build_empty_gitignore (crates/ignore/benches/workloads.rs:304-307)
```rust
fn build_empty_gitignore(b: &mut test::Bencher) {
    let builder = GitignoreBuilder::new("workspace/packages/example");
    b.iter(|| black_box(builder.build().unwrap()));
}
```
- compiled_multi_child_matchers_are_not_reused (crates/ignore/src/incremental.rs:901-936)
```rust
    fn compiled_multi_child_matchers_are_not_reused() {
        let td = tmpdir();
        mkdirp(td.path().join("a/b/c/d/e"));

        wfile(td.path().join("a/b/c/d/e/.ignore"), "*.tmp\n");
        let mut mats = matchers(&builders([
            td.path().join("a/b"),
            td.path().join("a/b/c"),
        ]));
        // Some sanity checking first.
        assert!(matchedf(&mut mats[0], "c/first.tmp").is_none());
        assert!(matchedf(&mut mats[0], "c/d/first.tmp").is_none());
        assert!(matchedf(&mut mats[0], "c/d/e/first.tmp").is_ignore());

        // Now write a new ignore file at the same location as above
        // and check that the other matcher still uses the "stale" data.
        wfile(td.path().join("a/b/c/d/e/.ignore"), "!*.tmp\n");
        assert!(matchedf(&mut mats[1], "first.tmp").is_none());
        assert!(matchedf(&mut mats[1], "d/first.tmp").is_none());
        // This is the punch line: because we didn't load `mats[1]` before
        // changing the ignore file, it loads it here and thus this path gets
        // whitelisted.
        assert!(matchedf(&mut mats[1], "d/e/first.tmp").is_whitelist());
        // ... but `mats[0]` still uses the stale gitignore matcher cached in
        // memory!
        assert!(matchedf(&mut mats[0], "c/d/e/first.tmp").is_ignore());

        // If we rebuilder the matcher... then we force reloading and they're
        // now consistent with one another.
        let mut mats = matchers(&builders([
            td.path().join("a/b"),
            td.path().join("a/b/c"),
        ]));
        assert!(matchedf(&mut mats[0], "c/d/e/first.tmp").is_whitelist());
        assert!(matchedf(&mut mats[1], "d/e/first.tmp").is_whitelist());
    }
```
- matchers (crates/ignore/src/incremental.rs:605-607)
```rust
    fn matchers(builder: &WalkBuilder) -> Vec<IncrementalIgnore> {
        builder.build_matchers()
    }
```
- GitignoreBuilder.new (crates/ignore/src/gitignore.rs:335-344)
```rust
    pub fn new<P: AsRef<Path>>(root: P) -> GitignoreBuilder {
        let root = root.as_ref();
        GitignoreBuilder {
            builder: GlobSetBuilder::new(),
            root: strip_prefix("./", root).unwrap_or(root).to_path_buf(),
            globs: vec![],
            case_insensitive: false,
            allow_unclosed_class: true,
        }
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
- Gitignore.matched_stripped (crates/ignore/src/gitignore.rs:259-282)
```rust
    fn matched_stripped<P: AsRef<Path>>(
        &self,
        path: P,
        is_dir: bool,
    ) -> Match<&Glob> {
        if self.is_empty() {
            return Match::None;
        }
        let path = path.as_ref();
        let mut matches = self.matches.as_ref().unwrap().get();
        let candidate = Candidate::new(path);
        self.set.matches_candidate_into(&candidate, &mut *matches);
        for &i in matches.iter().rev() {
            let glob = &self.globs[i];
            if !glob.is_only_dir() || is_dir {
                return if glob.is_whitelist() {
                    Match::Whitelist(glob)
                } else {
                    Match::Ignore(glob)
                };
            }
        }
        Match::None
    }
```
- compiled_matchers_are_reused (crates/ignore/src/incremental.rs:811-830)
```rust
    fn compiled_matchers_are_reused() {
        let td = tmpdir();
        mkdirp(td.path().join("a"));
        wfile(td.path().join("a/.ignore"), "*.tmp\n");

        let mut m = one_matcher(&builder(td.path()));
        assert!(matchedf(&mut m, "a/first.tmp").is_ignore());

        // Below demonstrates that this new ignore file contents
        // aren't actually picked up because it was already loaded.
        wfile(td.path().join("a/.ignore"), "!*.tmp\n*.rs\n");
        assert!(matchedf(&mut m, "a/first.tmp").is_ignore());
        assert!(matchedf(&mut m, "a/second.tmp").is_ignore());
        assert!(matchedf(&mut m, "a/keep.rs").is_none());
        // To get the new ignore file, we need to rebuild the matcher.
        let mut m = one_matcher(&builder(td.path()));
        assert!(matchedf(&mut m, "a/first.tmp").is_whitelist());
        assert!(matchedf(&mut m, "a/second.tmp").is_whitelist());
        assert!(matchedf(&mut m, "a/keep.rs").is_ignore());
    }
```
- Ignore.add_child_path_with_found_ignore_files (crates/ignore/src/dir.rs:340-467)
```rust
    fn add_child_path_with_found_ignore_files(
        &self,
        dir: &Path,
        ignore_files_list: Option<&IgnoreFilesFound>,
    ) -> (IgnoreInner, Option<Error>) {
        let check_vcs_dir = self.inner.opts.require_git
            && (self.inner.opts.git_ignore || self.inner.opts.git_exclude);
        let git_type = if check_vcs_dir
            && ignore_files_list.is_none_or(|i| i.has_git_dir)
        {
            dir.join(".git").metadata().ok().map(|md| md.file_type())
        } else {
            None
        };
        let has_jj = check_vcs_dir
            && ignore_files_list.is_none_or(|i| i.has_jj_dir)
            && dir.join(".jj").exists();
        let has_git = check_vcs_dir && (git_type.is_some() || has_jj);

        let mut errs = PartialErrorBuilder::default();
        let custom_ig_matcher = if self
            .inner
            .custom_ignore_filenames
            .is_empty()
        {
            Gitignore::empty()
        } else {
            let custom_ignore_names: Vec<&OsString> = match ignore_files_list {
                None => self.inner.custom_ignore_filenames.iter().collect(),
                Some(m) => self
                    .inner
                    .custom_ignore_filenames
                    .iter()
                    .zip(m.custom_ignore_files.iter())
                    .filter(|&(_, &matched)| matched)
                    .map(|(name, _)| name)
                    .collect(),
            };
            if custom_ignore_names.is_empty() {
                Gitignore::empty()
            } else {
                let (m, err) = create_gitignore(
                    &dir,
                    &dir,
                    &custom_ignore_names,
                    self.inner.opts.ignore_case_insensitive,
                );
                errs.maybe_push(err);
                m
            }
        };
        let ig_matcher = if !self.inner.opts.ignore
            || !ignore_files_list.is_none_or(|i| i.has_ignore)
        {
            Gitignore::empty()
        } else {
            let (m, err) = create_gitignore(
                &dir,
                &dir,
                &[".ignore"],
                self.inner.opts.ignore_case_insensitive,
            );
            errs.maybe_push(err);
            m
        };
        let gi_matcher = if !self.inner.opts.git_ignore
            || !ignore_files_list.is_none_or(|i| i.has_git_ignore)
        {
            Gitignore::empty()
        } else {
            let (m, err) = create_gitignore(
                &dir,
                &dir,
                &[".gitignore"],
                self.inner.opts.ignore_case_insensitive,
            );
            errs.maybe_push(err);
            m
        };

        let gi_exclude_matcher = if !self.inner.opts.git_exclude
            || !ignore_files_list.is_none_or(|i| i.has_git_dir)
        {
            Gitignore::empty()
        } else {
            match resolve_git_commondir(dir, git_type) {
                Ok(git_dir) => {
                    let (m, err) = create_gitignore(
                        &dir,
                        &git_dir,
                        &["info/exclude"],
                        self.inner.opts.ignore_case_insensitive,
                    );
                    errs.maybe_push(err);
                    m
                }
                Err(err) => {
                    errs.maybe_push(err);
                    Gitignore::empty()
                }
            }
        };
        let ig = IgnoreInner {
            compiled: self.inner.compiled.clone(),
            dir: dir.to_path_buf(),
            overrides: self.inner.overrides.clone(),
            types: self.inner.types.clone(),
            parent: Some(self.inner.clone()),
            is_absolute_parent: false,
            global_gitignores_relative_to: self
                .inner
                .global_gitignores_relative_to
                .clone(),
            explicit_ignores: self.inner.explicit_ignores.clone(),
            custom_ignore_filenames: self
                .inner
                .custom_ignore_filenames
                .clone(),
            custom_ignore_matcher: custom_ig_matcher,
            ignore_matcher: ig_matcher,
            git_global_matcher: self.inner.git_global_matcher.clone(),
            git_ignore_matcher: gi_matcher,
            git_exclude_matcher: gi_exclude_matcher,
            has_git,
            opts: self.inner.opts,
        };
        (ig, errs.into_error_option())
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
- Glob.doc_long (crates/core/flags/defs.rs:2525-2552)
```rust
    fn doc_long(&self) -> &'static str {
        r#"
Include or exclude files and directories for searching that match the given
glob. This always overrides any other ignore logic. Multiple glob flags may
be used. Globbing rules match \fB.gitignore\fP globs. Precede a glob with a
\fB!\fP to exclude it. If multiple globs match a file or directory, the glob
given later in the command line takes precedence.
.sp
As an extension, globs support specifying alternatives:
.BI "\-g '" ab{c,d}* '
is equivalent to
.BI "\-g " "abc " "\-g " abd.
Empty alternatives like
.BI "\-g '" ab{,c} '
are not currently supported. Note that this syntax extension is also currently
enabled in \fBgitignore\fP files, even though this syntax isn't supported by
git itself. ripgrep may disable this syntax extension in gitignore files, but
it will always remain available via the \flag{glob} flag.
.sp
When this flag is set, every file and directory is applied to it to test for
a match. For example, if you only want to search in a particular directory
\fIfoo\fP, then
.BI "\-g " foo
is incorrect because \fIfoo/bar\fP does not match
the glob \fIfoo\fP. Instead, you should use
.BI "\-g '" foo/** '.
"#
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
- incremental_dense_ignore_cold (crates/ignore/benches/workloads.rs:424-433)
```rust
fn incremental_dense_ignore_cold(b: &mut test::Bencher) {
    let ws = dense_ignore_workspace();
    let builder = ws.builder();
    b.iter(|| {
        let mut matchers = builder.build_matchers();
        for path in &ws.paths {
            assert!(!matchers[0].matched(black_box(path), false).is_ignore());
        }
    });
}
```
- Worker.run (crates/ignore/src/walk.rs:1761-1767)
```rust
    fn run(mut self) {
        while let Some(work) = self.get_work() {
            if let WalkState::Quit = self.run_one(work) {
                self.quit_now();
            }
        }
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
- alloc_error (crates/searcher/src/line_buffer.rs:37-40)
```rust
pub(crate) fn alloc_error(limit: usize) -> io::Error {
    let msg = format!("configured allocation limit ({}) exceeded", limit);
    io::Error::new(io::ErrorKind::Other, msg)
}
```
- WalkBuilder.add_ignore (crates/ignore/src/walk.rs:791-812)
```rust
    pub fn add_ignore<P: AsRef<Path>>(&mut self, path: P) -> Option<Error> {
        let path = path.as_ref();
        let Some(cwd) = self.get_or_set_current_dir() else {
            let err = std::io::Error::other(format!(
                "CWD is not known, ignoring global gitignore {}",
                path.display()
            ));
            return Some(err.into());
        };
        let mut builder = GitignoreBuilder::new(cwd);
        let mut errs = PartialErrorBuilder::default();
        errs.maybe_push(builder.add(path));
        match builder.build() {
            Ok(gi) => {
                self.ig_builder.add_ignore(gi);
            }
            Err(err) => {
                errs.push(err);
            }
        }
        errs.into_error_option()
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
- path_equals (crates/ignore/src/walk.rs:2123-2142)
```rust
fn path_equals(dent: &DirEntry, handle: &Handle) -> Result<bool, Error> {
    #[cfg(unix)]
    fn never_equal(dent: &DirEntry, handle: &Handle) -> bool {
        dent.ino() != Some(handle.ino())
    }

    #[cfg(not(unix))]
    fn never_equal(_: &DirEntry, _: &Handle) -> bool {
        false
    }

    // If we know for sure that these two things aren't equal, then avoid
    // the costly extra stat call to determine equality.
    if dent.is_stdin() || never_equal(dent, handle) {
        return Ok(false);
    }
    Handle::from_path(dent.path()).map(|h| &h == handle).map_err(|err| {
        Error::Io(err).with_depth(dent.depth()).with_path(dent.path())
    })
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
- MANY_SHORT_GLOBS (crates/globset/benches/bench.rs:75-91)
```rust
const MANY_SHORT_GLOBS: &'static [&'static str] = &[
    // Taken from a random .gitignore on my system.
    ".*.swp",
    "tags",
    "target",
    "*.lock",
    "tmp",
    "*.csv",
    "*.fst",
    "*-got",
    "*.csv.idx",
    "words",
    "98m*",
    "dict",
    "test",
    "months",
];
```
- incremental_cold (crates/ignore/benches/workloads.rs:245-254)
```rust
fn incremental_cold(b: &mut test::Bencher) {
    let ws = Workspace::new(16, 4, 4, true);
    let builder = ws.builder();
    b.iter(|| {
        let mut matchers = builder.build_matchers();
        for path in &ws.paths {
            assert!(!matchers[0].matched(black_box(path), false).is_ignore());
        }
    });
}
```
- FileTypeDef.globs (crates/ignore/src/types.rs:158-160)
```rust
    pub fn globs(&self) -> &[String] {
        &self.globs
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
- Worker.get_work (crates/ignore/src/walk.rs:1942-1992)
```rust
    fn get_work(&mut self) -> Option<Work> {
        let mut value = self.recv();
        loop {
            // Simulate a priority channel: If quit_now flag is set, we can
            // receive only quit messages.
            if self.is_quit_now() {
                value = Some(Message::Quit)
            }
            match value {
                Some(Message::Work(work)) => {
                    return Some(work);
                }
                Some(Message::Quit) => {
                    // Repeat quit message to wake up sleeping threads, if
                    // any. The domino effect will ensure that every thread
                    // will quit.
                    self.send_quit();
                    return None;
                }
                None => {
                    if self.deactivate_worker() == 0 {
                        // If deactivate_worker() returns 0, every worker thread
                        // is currently within the critical section between the
                        // acquire in deactivate_worker() and the release in
                        // activate_worker() below.  For this to happen, every
                        // worker's local deque must be simultaneously empty,
                        // meaning there is no more work left at all.
                        self.send_quit();
                        return None;
                    }
                    // Wait for next `Work` or `Quit` message.
                    loop {
                        if self.is_quit_now() {
                            return None;
                        }
                        if let Some(v) = self.recv() {
                            self.activate_worker();
                            value = Some(v);
                            break;
                        }
                        // Our stack isn't blocking. Instead of burning the
                        // CPU waiting, we let the thread sleep for a bit. In
                        // general, this tends to only occur once the search is
                        // approaching termination.
                        let dur = std::time::Duration::from_millis(1);
                        std::thread::sleep(dur);
                    }
                }
            }
        }
    }
```
- SearchWorkerBuilder.preprocessor_globs (crates/core/search.rs:108-114)
```rust
    pub(crate) fn preprocessor_globs(
        &mut self,
        globs: ignore::overrides::Override,
    ) -> &mut SearchWorkerBuilder {
        self.config.preprocessor_globs = globs;
        self
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
- stops_at_git_dir (crates/ignore/src/dir.rs:1365-1385)
```rust
    fn stops_at_git_dir() {
        // This tests that .gitignore files beyond a .git barrier aren't
        // matched, but .ignore files are.
        let td = tmpdir();
        mkdirp(td.path().join(".git"));
        mkdirp(td.path().join("foo/.git"));
