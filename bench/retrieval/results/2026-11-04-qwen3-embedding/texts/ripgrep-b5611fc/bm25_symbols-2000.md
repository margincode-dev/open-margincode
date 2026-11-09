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

