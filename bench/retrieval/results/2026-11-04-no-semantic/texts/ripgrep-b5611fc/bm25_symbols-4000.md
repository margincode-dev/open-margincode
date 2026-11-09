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
