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
- hidden_files_and_directories (crates/ignore/src/incremental.rs:1194-1218)
```rust
    fn hidden_files_and_directories() {
        let td = tmpdir();
        mkdirp(td.path().join(".hidden-dir"));
        mkdirp(td.path().join("visible-dir"));
        wfile(td.path().join(".hidden-file"), "");
        wfile(td.path().join("visible-file"), "");

        let mut m = one_matcher(&builder(td.path()));
        assert!(matchedf(&mut m, ".hidden-file").is_ignore());
        let dir = matchedd(&mut m, ".hidden-dir");
        assert!(dir.is_ignore());
        assert!(!dir.should_descend());
        assert!(matchedf(&mut m, "visible-file").is_none());
        let dir = matchedd(&mut m, "visible-dir");
        assert!(dir.is_none());
        assert!(dir.should_descend());

        let mut b = builder(td.path());
        b.hidden(false);
        let mut m = one_matcher(&b);
        assert!(matchedf(&mut m, ".hidden-file").is_none());
        let dir = matchedd(&mut m, ".hidden-dir");
        assert!(dir.is_none());
        assert!(dir.should_descend());
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
- Hidden.doc_long (crates/core/flags/defs.rs:2791-2808)
```rust
    fn doc_long(&self) -> &'static str {
        r#"
Search hidden files and directories. By default, hidden files and directories
are skipped. Note that if a hidden file or a directory is whitelisted in
an ignore file, then it will be searched even if this flag isn't provided.
Similarly if a hidden file or directory is given explicitly as an argument to
ripgrep.
.sp
A file or directory is considered hidden if its base name starts with a dot
character (\fB.\fP). On operating systems which support a "hidden" file
attribute, like Windows, files with this attribute are also considered hidden.
.sp
Note that \flag{hidden} will include files and folders like \fB.git\fP
regardless of \flag{no-ignore-vcs}. To exclude such paths when using
\flag{hidden}, you must explicitly ignore them using another flag or ignore
file.
"#
    }
```
- IncrementalIgnore.match_path (crates/ignore/src/incremental.rs:393-411)
```rust
    fn match_path(
        &self,
        ignore: &Ignore,
        relative: &Path,
        is_dir: bool,
    ) -> IncrementalMatch {
        let path = self.root.join(relative);
        let mut mat = IncrementalMatch::from_match(
            ignore.matched(&path, is_dir).map(|_| ()),
            is_dir,
        );
        // Whether a file is hidden or not has low precedence in filtering. We
        // only check it if we haven't matched anything above. This permits
        // callers to whitelist hidden files or directories.
        if self.options.hidden && mat.is_none() && is_hidden_path(&path) {
            mat = IncrementalMatch::ignore();
        }
        mat
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
- NoIgnoreDot.doc_long (crates/core/flags/defs.rs:4411-4419)
```rust
    fn doc_long(&self) -> &'static str {
        r"
Don't respect filter rules from \fB.ignore\fP or \fB.rgignore\fP files.
.sp
This does not impact whether ripgrep will ignore files and directories whose
names begin with a dot. For that, see the \flag{hidden} flag. This flag also
does not impact whether filter rules from \fB.gitignore\fP files are respected.
"
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
- Ignore.collect_ignore_files (crates/ignore/src/dir.rs:308-338)
```rust
    fn collect_ignore_files(
        &self,
        entries: &[fs::DirEntry],
    ) -> IgnoreFilesFound {
        let custom_ignore_filenames = &self.inner.custom_ignore_filenames;
        let mut files = IgnoreFilesFound {
            has_ignore: false,
            has_git_ignore: false,
            has_git_dir: false,
            has_jj_dir: false,
            custom_ignore_files: vec![false; custom_ignore_filenames.len()],
        };
        for entry in entries {
            let file_name = entry.file_name();
            if file_name == OsStr::new(".ignore") {
                files.has_ignore = true;
            } else if file_name == OsStr::new(".gitignore") {
                files.has_git_ignore = true;
            } else if file_name == OsStr::new(".git") {
                files.has_git_dir = true;
            } else if file_name == OsStr::new(".jj") {
                files.has_jj_dir = true;
            }
            for (i, name) in custom_ignore_filenames.iter().enumerate() {
                if file_name == name.as_os_str() {
                    files.custom_ignore_files[i] = true;
                }
            }
        }
        files
    }
```
- Parser (crates/core/flags/parse.rs:161-169)
```rust
struct Parser {
    /// A single map that contains all possible flag names. This includes
    /// short and long names, aliases and negations. This maps those names to
    /// indices into `info`.
    map: FlagMap,
    /// A map from IDs returned by the `map` to the corresponding flag
    /// information.
    info: Vec<FlagInfo>,
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
- is_hidden_path_only (crates/ignore/src/pathutil.rs:85-91)
```rust
fn is_hidden_path_only(path: &Path) -> bool {
    if let Some(name) = file_name(path) {
        name.as_encoded_bytes().starts_with(b".")
    } else {
        false
    }
}
```
- resolve_git_commondir (crates/ignore/src/dir.rs:1061-1105)
```rust
fn resolve_git_commondir(
    dir: &Path,
    git_type: Option<FileType>,
) -> Result<PathBuf, Option<Error>> {
    let git_dir_path = || dir.join(".git");
    let git_dir = git_dir_path();
    if !git_type.map_or(false, |ft| ft.is_file()) {
        return Ok(git_dir);
    }
    let file = match File::open(git_dir) {
        Ok(file) => io::BufReader::new(file),
        Err(err) => {
            return Err(Some(Error::Io(err).with_path(git_dir_path())));
        }
    };
    let dot_git_line = match file.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(err)) => {
            return Err(Some(Error::Io(err).with_path(git_dir_path())));
        }
        None => return Err(None),
    };
    if !dot_git_line.starts_with("gitdir: ") {
        return Err(None);
    }
    let real_git_dir = PathBuf::from(&dot_git_line["gitdir: ".len()..]);
    let git_commondir_file = || real_git_dir.join("commondir");
    let file = match File::open(git_commondir_file()) {
        Ok(file) => io::BufReader::new(file),
        Err(_) => return Err(None),
    };
    let commondir_line = match file.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(err)) => {
            return Err(Some(Error::Io(err).with_path(git_commondir_file())));
        }
        None => return Err(None),
    };
    let commondir_abs = if commondir_line.starts_with(".") {
        real_git_dir.join(commondir_line) // relative commondir
    } else {
        PathBuf::from(commondir_line)
    };
    Ok(commondir_abs)
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
        wfile(td.path().join(".gitignore"), "foo");
        wfile(td.path().join(".ignore"), "bar");

        let ig0 = IgnoreBuilder::new().build();
        let (ig1, err) = ig0.add_child(td.path());
        assert!(err.is_none());
        let (ig2, err) = ig1.add_child(ig1.path().join("foo"));
        assert!(err.is_none());

        assert!(ig1.matched("foo", false).is_ignore());
        assert!(ig2.matched("foo", false).is_none());

        assert!(ig1.matched("bar", false).is_ignore());
        assert!(ig2.matched("bar", false).is_ignore());
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
- create_gitignore (crates/ignore/src/dir.rs:1016-1051)
```rust
pub(crate) fn create_gitignore<T: AsRef<OsStr>>(
    dir: &Path,
    dir_for_ignorefile: &Path,
    names: &[T],
    case_insensitive: bool,
) -> (Gitignore, Option<Error>) {
    let mut builder = GitignoreBuilder::new(dir);
    let mut errs = PartialErrorBuilder::default();
    builder.case_insensitive(case_insensitive).unwrap();
    for name in names {
        let gipath = dir_for_ignorefile.join(name.as_ref());
        // This check is not necessary, but is added for performance. Namely,
        // a simple stat call checking for existence can often be just a bit
        // quicker than actually trying to open a file. Since the number of
        // directories without ignore files likely greatly exceeds the number
        // with ignore files, this check generally makes sense.
        //
        // However, until demonstrated otherwise, we speculatively do not do
        // this on Windows since Windows is notorious for having slow file
        // system operations. Namely, it's not clear whether this analysis
        // makes sense on Windows.
        //
        // For more details: https://github.com/BurntSushi/ripgrep/pull/1381
        if cfg!(windows) || gipath.exists() {
            errs.maybe_push_ignore_io(builder.add(gipath));
        }
    }
    let gi = match builder.build() {
        Ok(gi) => gi,
        Err(err) => {
            errs.push(err);
            GitignoreBuilder::new(dir).build().unwrap()
        }
    };
    (gi, errs.into_error_option())
}
```
- Ignore.matched_dir_entry (crates/ignore/src/dir.rs:485-494)
```rust
    pub(crate) fn matched_dir_entry<'a>(
        &'a self,
        dent: &DirEntry,
    ) -> Match<IgnoreMatch<'a>> {
        let m = self.matched(dent.path(), dent.is_dir());
        if m.is_none() && self.inner.opts.hidden && is_hidden_entry(dent) {
            return Match::Ignore(IgnoreMatch::hidden());
        }
        m
    }
```
- BinaryMode (crates/core/flags/lowargs.rs:233-253)
```rust
pub(crate) enum BinaryMode {
    /// Automatically determine the binary mode to use. Essentially, when
    /// a file is searched explicitly, then it will be searched using the
    /// `SearchAndSuppress` strategy. Otherwise, it will be searched in a way
    /// that attempts to skip binary files as much as possible. That is, once
    /// a file is classified as binary, searching will immediately stop.
    #[default]
    Auto,
    /// Search files even when they have binary data, but if a match is found,
    /// suppress it and emit a warning.
    ///
    /// In this mode, `NUL` bytes are replaced with line terminators. This is
    /// a heuristic meant to reduce heap memory usage, since true binary data
    /// isn't line oriented. If one attempts to treat such data as line
    /// oriented, then one may wind up with impractically large lines. For
    /// example, many binary files contain very long runs of NUL bytes.
    SearchAndSuppress,
    /// Treat all files as if they were plain text. There's no skipping and no
    /// replacement of `NUL` bytes with line terminators.
    AsText,
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
        } else if path.len() == 1 && path[0] == b'.' {
            return None;
        } else if path.last() == Some(&b'.') {
            return None;
        } else if path.len() >= 2 && &path[path.len() - 2..] == &b".."[..] {
            return None;
        }
        let last_slash = memrchr(b'/', path).map(|i| i + 1).unwrap_or(0);
        Some(OsStr::from_bytes(&path[last_slash..]))
    }

    #[cfg(not(unix))]
    fn imp(path: &Path) -> Option<&OsStr> {
        path.file_name()
    }

    imp(path.as_ref())
}
```
- IgnoreFile.doc_long (crates/core/flags/defs.rs:3227-3241)
```rust
    fn doc_long(&self) -> &'static str {
        r"
Specifies a path to one or more \fBgitignore\fP formatted rules files.
These patterns are applied after the patterns found in \fB.gitignore\fP,
\fB.rgignore\fP and \fB.ignore\fP are applied and are matched relative to the
current working directory. That is, files specified via this flag have lower
precedence than files automatically found in the directory tree. Multiple
additional ignore files can be specified by using this flag repeatedly. When
specifying multiple ignore files, earlier files have lower precedence than
later files.
.sp
If you are looking for a way to include or exclude files and directories
directly on the command line, then use \flag{glob} instead.
"
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
- is_hidden_entry (crates/ignore/src/pathutil.rs:56-79)
```rust
pub(crate) fn is_hidden_entry(dent: &DirEntry) -> bool {
    #[cfg(not(windows))]
    fn imp(dent: &DirEntry) -> bool {
        is_hidden_path_only(dent.path())
    }

    #[cfg(windows)]
    fn imp(dent: &DirEntry) -> bool {
        use std::os::windows::fs::MetadataExt;
        use winapi_util::file;

        // This looks like we're doing an extra stat call, but on Windows, the
        // directory traverser reuses the metadata retrieved from each directory
        // entry and stores it on the DirEntry itself. So this is "free."
        if let Ok(md) = dent.metadata() {
            if file::is_hidden(md.file_attributes() as u64) {
                return true;
            }
        }
        is_hidden_path_only(dent.path())
    }

    imp(dent)
}
```
- gitignore_whitelist_overrides_hidden_filter (crates/ignore/src/incremental.rs:1221-1233)
```rust
    fn gitignore_whitelist_overrides_hidden_filter() {
        let td = tmpdir();
        mkdirp(td.path().join(".git"));
        mkdirp(td.path().join(".hidden-dir"));
        wfile(td.path().join(".hidden-file"), "");
        wfile(td.path().join(".gitignore"), "!.hidden-file\n!.hidden-dir/\n");

        let mut m = one_matcher(&builder(td.path()));
        assert!(matchedf(&mut m, ".hidden-file").is_whitelist());
        let dir = matchedd(&mut m, ".hidden-dir");
        assert!(dir.is_whitelist());
        assert!(dir.should_descend());
    }
```
- is_hidden_path (crates/ignore/src/pathutil.rs:19-39)
```rust
pub(crate) fn is_hidden_path(dent: &Path) -> bool {
    #[cfg(not(windows))]
    fn imp(path: &Path) -> bool {
        is_hidden_path_only(path)
    }

    #[cfg(windows)]
    fn imp(path: &Path) -> bool {
        use std::os::windows::fs::MetadataExt;
        use winapi_util::file;

        if let Ok(md) = path.metadata() {
            if file::is_hidden(md.file_attributes() as u64) {
                return true;
            }
        }
        is_hidden_path_only(path)
    }

    imp(dent)
}
```
- leading_dot_slash_impacts_matching (crates/ignore/src/incremental.rs:662-675)
```rust
    fn leading_dot_slash_impacts_matching() {
        let td = tmpdir();
        let root = td.path().join("root");
        mkdirp(root.join("a"));
        wfile(root.join(".ignore"), "/foo\n");
        wfile(root.join("a/.ignore"), "/foo\n");

        let mut m = one_matcher(&builder(&root));
        assert!(matchedf(&mut m, "foo").is_ignore());
        assert!(matchedf(&mut m, "./foo").is_none());
        assert!(matchedf(&mut m, "a/allowed").is_none());
        assert!(matchedf(&mut m, "a/foo").is_ignore());
        assert!(matchedf(&mut m, "./a/foo").is_none());
    }
```
- Heading.doc_long (crates/core/flags/defs.rs:2669-2681)
```rust
    fn doc_long(&self) -> &'static str {
        r"
This flag prints the file path above clusters of matches from each file instead
of printing the file path as a prefix for each matched line.
.sp
This is the default mode when printing to a tty.
.sp
When \fBstdout\fP is not a tty, then ripgrep will default to the standard
grep-like format. One can force this format in Unix-like environments by
piping the output of ripgrep to \fBcat\fP. For example, \fBrg\fP \fIfoo\fP \fB|
cat\fP.
"
    }
```
- test_ignore_file (crates/core/flags/defs.rs:3255-3268)
```rust
fn test_ignore_file() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(Vec::<PathBuf>::new(), args.ignore_file);

    let args = parse_low_raw(["--ignore-file", "foo"]).unwrap();
    assert_eq!(vec![PathBuf::from("foo")], args.ignore_file);

    let args = parse_low_raw(["--ignore-file", "foo", "--ignore-file", "bar"])
        .unwrap();
    assert_eq!(
        vec![PathBuf::from("foo"), PathBuf::from("bar")],
        args.ignore_file
    );
}
```
- Ignore.matched_ignore (crates/ignore/src/dir.rs:548-685)
```rust
    pub(crate) fn matched_ignore<'a>(
        &'a self,
        path: &Path,
        is_dir: bool,
    ) -> Match<IgnoreMatch<'a>> {
        let (
            mut m_custom_ignore,
            mut m_ignore,
            mut m_gi,
            mut m_gi_exclude,
            mut m_explicit,
        ) = (Match::None, Match::None, Match::None, Match::None, Match::None);
        let any_git = !self.inner.opts.require_git
            || self.parents().any(|ig| ig.inner.has_git);
        let mut saw_git = false;
        for ig in self.parents().take_while(|ig| !ig.inner.is_absolute_parent)
        {
            if m_custom_ignore.is_none() {
                m_custom_ignore = ig
                    .inner
                    .custom_ignore_matcher
                    .matched(path, is_dir)
                    .map(IgnoreMatch::gitignore);
            }
            if m_ignore.is_none() {
                m_ignore = ig
                    .inner
                    .ignore_matcher
                    .matched(path, is_dir)
                    .map(IgnoreMatch::gitignore);
            }
            if any_git && !saw_git && m_gi.is_none() {
                m_gi = ig
                    .inner
                    .git_ignore_matcher
                    .matched(path, is_dir)
                    .map(IgnoreMatch::gitignore);
            }
            if any_git && !saw_git && m_gi_exclude.is_none() {
                m_gi_exclude = ig
                    .inner
                    .git_exclude_matcher
                    .matched(path, is_dir)
                    .map(IgnoreMatch::gitignore);
            }
            saw_git = saw_git || ig.inner.has_git;
        }
        if self.inner.opts.parents {
            if let Some(abs_parent_path) = self.absolute_base() {
                // What we want to do here is take the absolute base path of
                // this directory and join it with the path we're searching.
                // The main issue we want to avoid is accidentally duplicating
                // directory components, so we try to strip any common prefix
                // off of `path`. Overall, this seems a little ham-fisted, but
                // it does fix a nasty bug. It should do fine until we overhaul
                // this crate.
                let path = abs_parent_path.join(
                    self.parents()
                        .take_while(|ig| !ig.inner.is_absolute_parent)
                        .last()
                        .map_or(path, |ig| {
                            // This is a weird special case when ripgrep users
                            // search with just a `.`, as some tools do
                            // automatically (like consult). In this case, if
                            // we don't bail out now, the code below will strip
                            // a leading `.` from `path`, which might mangle
                            // a hidden file name!
                            if ig.inner.dir.as_path() == Path::new(".") {
                                return path;
                            }
                            let without_dot_slash = strip_if_is_prefix(
                                "./",
                                ig.inner.dir.as_path(),
                            );
                            let relative_base =
                                strip_if_is_prefix(without_dot_slash, path);
                            strip_if_is_prefix("/", relative_base)
                        }),
                );

                for ig in self
                    .parents()
                    .skip_while(|ig| !ig.inner.is_absolute_parent)
                {
                    if m_custom_ignore.is_none() {
                        m_custom_ignore = ig
                            .inner
                            .custom_ignore_matcher
                            .matched(&path, is_dir)
                            .map(IgnoreMatch::gitignore);
                    }
                    if m_ignore.is_none() {
                        m_ignore = ig
                            .inner
                            .ignore_matcher
                            .matched(&path, is_dir)
                            .map(IgnoreMatch::gitignore);
                    }
                    if any_git && !saw_git && m_gi.is_none() {
                        m_gi = ig
                            .inner
                            .git_ignore_matcher
                            .matched(&path, is_dir)
                            .map(IgnoreMatch::gitignore);
                    }
                    if any_git && !saw_git && m_gi_exclude.is_none() {
                        m_gi_exclude = ig
                            .inner
                            .git_exclude_matcher
                            .matched(&path, is_dir)
                            .map(IgnoreMatch::gitignore);
                    }
                    saw_git = saw_git || ig.inner.has_git;
                }
            }
        }
        for gi in self.inner.explicit_ignores.iter().rev() {
            if !m_explicit.is_none() {
                break;
            }
            m_explicit = gi.matched(&path, is_dir).map(IgnoreMatch::gitignore);
        }
        let m_global = if any_git {
            self.inner
                .git_global_matcher
                .matched(&path, is_dir)
                .map(IgnoreMatch::gitignore)
        } else {
            Match::None
        };

        m_custom_ignore
            .or(m_ignore)
            .or(m_gi)
            .or(m_gi_exclude)
