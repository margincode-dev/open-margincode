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

