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
