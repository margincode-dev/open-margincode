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
