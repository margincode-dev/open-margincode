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
