- Index (crates/core/flags/defs.rs:3455-3455)
```rust
struct Index;
```
- IndexCrud (crates/core/flags/defs.rs:3554-3554)
```rust
struct IndexCrud;
```
- IndexForce (crates/core/flags/defs.rs:3651-3651)
```rust
struct IndexForce;
```
- IndexPath (crates/core/flags/defs.rs:3698-3698)
```rust
struct IndexPath;
```
- IndexMode (crates/core/flags/lowargs.rs:199-202)
```rust
pub(crate) enum IndexMode {
    /// Add, update or remove files in an index.
    Crud,
}
```
- Index.update (crates/core/flags/defs.rs:3511-3520)
```rust
    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        check_indexing_allowed()?;
        assert!(v.unwrap_switch(), "--index has no negation");
        args.index = args.index.saturating_add(1);
        anyhow::ensure!(
            args.index <= 2,
            "-X/--index may be given at most twice"
        );
        Ok(())
    }
```
- Index.name_long (crates/core/flags/defs.rs:3464-3466)
```rust
    fn name_long(&self) -> &'static str {
        "index"
    }
```
- test_index_force (crates/core/flags/defs.rs:3683-3694)
```rust
fn test_index_force() {
    if !cfg!(feature = "unstable-index") {
        assert!(parse_low_raw(["--x-force"]).is_err());
        return;
    }

    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.index_force);

    let args = parse_low_raw(["--x-force"]).unwrap();
    assert_eq!(true, args.index_force);
}
```
- Index.doc_short (crates/core/flags/defs.rs:3470-3472)
```rust
    fn doc_short(&self) -> &'static str {
        r"Use a search index when one is available."
    }
```
- IndexCrud.doc_short (crates/core/flags/defs.rs:3566-3568)
```rust
    fn doc_short(&self) -> &'static str {
        r"Create or update a search index."
    }
```
- test_index (crates/core/flags/defs.rs:3525-3550)
```rust
fn test_index() {
    if !cfg!(feature = "unstable-index") {
        assert!(parse_low_raw(["-X"]).is_err());
        return;
    }

    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(0, args.index);
    assert_eq!(Mode::Search(SearchMode::Standard), args.mode);

    let args = parse_low_raw(["-X"]).unwrap();
    assert_eq!(1, args.index);
    assert_eq!(Mode::Search(SearchMode::Standard), args.mode);

    let args = parse_low_raw(["--index"]).unwrap();
    assert_eq!(1, args.index);

    let args = parse_low_raw(["-XX"]).unwrap();
    assert_eq!(2, args.index);

    let args = parse_low_raw(["-X", "--index"]).unwrap();
    assert_eq!(2, args.index);

    let result = parse_low_raw(["-XXX"]);
    assert!(result.is_err(), "{result:?}");
}
```
- IndexPath.doc_short (crates/core/flags/defs.rs:3713-3715)
```rust
    fn doc_short(&self) -> &'static str {
        r"Set the path of the index to update."
    }
```
- IndexForce.doc_long (crates/core/flags/defs.rs:3666-3671)
```rust
    fn doc_long(&self) -> &'static str {
        r"
When used with \flag{x-crud}, re-index every selected file even when its
modification time indicates that the index is already up to date.
"
    }
```
- IndexPath.doc_long (crates/core/flags/defs.rs:3716-3721)
```rust
    fn doc_long(&self) -> &'static str {
        r"
Set the index path used by \flag{x-crud}. This takes precedence over
\fBRIPGREP_INDEX_PATH\fP and automatic discovery of a \fB.ripgrep\fP directory.
"
    }
```
- test_index_path (crates/core/flags/defs.rs:3735-3749)
```rust
fn test_index_path() {
    if !cfg!(feature = "unstable-index") {
        assert!(parse_low_raw(["--x-path"]).is_err());
        return;
    }

    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.index_path);

    let args = parse_low_raw(["--x-path", "foo"]).unwrap();
    assert_eq!(Some(PathBuf::from("foo")), args.index_path);

    let args = parse_low_raw(["--x-path=foo", "--x-path", "bar"]).unwrap();
    assert_eq!(Some(PathBuf::from("bar")), args.index_path);
}
```
- IndexCrud.update (crates/core/flags/defs.rs:3619-3624)
```rust
    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        check_indexing_allowed()?;
        assert!(v.unwrap_switch(), "--x-crud has no negation");
        args.mode.update(Mode::Index(IndexMode::Crud));
        Ok(())
    }
```
- [u8].index (crates/matcher/src/lib.rs:162-164)
```rust
    fn index(&self, index: Match) -> &[u8] {
        &self[index.start..index.end]
    }
```
- str.index (crates/matcher/src/lib.rs:178-180)
```rust
    fn index(&self, index: Match) -> &str {
        &self[index.start..index.end]
    }
```
- [u8].index_mut (crates/matcher/src/lib.rs:169-171)
```rust
    fn index_mut(&mut self, index: Match) -> &mut [u8] {
        &mut self[index.start..index.end]
    }
```
- test_index_crud (crates/core/flags/defs.rs:3629-3647)
```rust
fn test_index_crud() {
    if !cfg!(feature = "unstable-index") {
        assert!(parse_low_raw(["--x-crud"]).is_err());
        return;
    }

    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(Mode::Search(SearchMode::Standard), args.mode);

    let args = parse_low_raw(["--x-crud"]).unwrap();
    assert_eq!(Mode::Index(IndexMode::Crud), args.mode);

    let args = parse_low_raw(["--files", "--x-crud", "foo"]).unwrap();
    assert_eq!(Mode::Index(IndexMode::Crud), args.mode);
    assert_eq!(vec![std::ffi::OsString::from("foo")], args.positional);

    let args = parse_low_raw(["--x-crud", "--files"]).unwrap();
    assert_eq!(Mode::Files, args.mode);
}
```
- Index.is_switch (crates/core/flags/defs.rs:3458-3460)
```rust
    fn is_switch(&self) -> bool {
        true
    }
```
- IndexPath.update (crates/core/flags/defs.rs:3726-3730)
```rust
    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        check_indexing_allowed()?;
        args.index_path = Some(PathBuf::from(v.unwrap_value()));
        Ok(())
    }
```
- IndexForce.update (crates/core/flags/defs.rs:3673-3678)
```rust
    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        check_indexing_allowed()?;
        assert!(v.unwrap_switch(), "--x-force has no negation");
        args.index_force = true;
        Ok(())
    }
```
- Index.name_short (crates/core/flags/defs.rs:3461-3463)
```rust
    fn name_short(&self) -> Option<u8> {
        Some(b'X')
    }
```
- Index.doc_category (crates/core/flags/defs.rs:3467-3469)
```rust
    fn doc_category(&self) -> Category {
        Category::Indexing
    }
```
- IndexCrud.is_switch (crates/core/flags/defs.rs:3557-3559)
```rust
    fn is_switch(&self) -> bool {
        true
    }
```
- IndexForce.is_switch (crates/core/flags/defs.rs:3654-3656)
```rust
    fn is_switch(&self) -> bool {
        true
    }
```
- IndexPath.is_switch (crates/core/flags/defs.rs:3701-3703)
```rust
    fn is_switch(&self) -> bool {
        false
    }
```
