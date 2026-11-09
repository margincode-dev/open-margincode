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
- M.capture_index (crates/matcher/src/lib.rs:1162-1164)
```rust
    fn capture_index(&self, name: &str) -> Option<usize> {
        (*self).capture_index(name)
    }
```
- IndexCrud.name_long (crates/core/flags/defs.rs:3560-3562)
```rust
    fn name_long(&self) -> &'static str {
        "x-crud"
    }
```
- IndexCrud.doc_category (crates/core/flags/defs.rs:3563-3565)
```rust
    fn doc_category(&self) -> Category {
        Category::Indexing
    }
```
- IndexForce.name_long (crates/core/flags/defs.rs:3657-3659)
```rust
    fn name_long(&self) -> &'static str {
        "x-force"
    }
```
- IndexForce.doc_category (crates/core/flags/defs.rs:3660-3662)
```rust
    fn doc_category(&self) -> Category {
        Category::Indexing
    }
```
- IndexPath.name_long (crates/core/flags/defs.rs:3704-3706)
```rust
    fn name_long(&self) -> &'static str {
        "x-path"
    }
```
- IndexPath.doc_category (crates/core/flags/defs.rs:3710-3712)
```rust
    fn doc_category(&self) -> Category {
        Category::Indexing
    }
```
- Index.doc_long (crates/core/flags/defs.rs:3473-3509)
```rust
    fn doc_long(&self) -> &'static str {
        r"
Enable searching with an index. Without this flag, ripgrep never uses an
index.
.sp
ripgrep looks for an index in the following order. The first step that finds
one or more indexes wins:
.sp
.IP 1. 4
When path operands are given on the command line, each operand is interpreted
as an index directory and every index is searched in command line order. An
operand that is not a valid index is an error.
.sp
.IP 2. 4
The index named by the \fBRIPGREP_INDEX_PATH\fP environment variable.
.sp
.IP 3. 4
A valid index in a \fB.ripgrep\fP directory in the current working directory.
.sp
.IP 4. 4
A valid index in a \fB.ripgrep\fP directory in the nearest parent of the current
working directory.
.PP
If no index is found, ripgrep performs an ordinary search.
.sp
Indexed candidates are filtered by explicit glob and file-type selections,
hidden-file and depth settings, and the maximum file size. Ignore files,
including \fB.gitignore\fP, are not read or reapplied during an indexed search.
Options that require transformed contents or every file make the query
ineligible for candidate filtering and therefore trigger the fallback below.
.sp
This flag may be given at most twice. When it is given once and the query
cannot use an index, ripgrep performs an ordinary search. When it is given
twice and an index was found, ripgrep stops instead of performing an ordinary
search if the query cannot use the index.
"
    }
```
- Parser.find_long (crates/core/flags/parse.rs:307-312)
```rust
    fn find_long(&self, name: &str) -> FlagLookup<'_> {
        let Some(index) = self.map.find(name.as_bytes()) else {
            return FlagLookup::UnrecognizedLong(name.to_string());
        };
        FlagLookup::Match(&self.info[index])
    }
```
- IndexPath.doc_variable (crates/core/flags/defs.rs:3707-3709)
```rust
    fn doc_variable(&self) -> Option<&'static str> {
        Some("PATH")
    }
```
- RegexMatcher.capture_index (crates/regex/src/matcher.rs:434-436)
```rust
    fn capture_index(&self, name: &str) -> Option<usize> {
        self.regex.group_info().to_index(PatternID::ZERO, name)
    }
```
- IndexPath.completion_type (crates/core/flags/defs.rs:3722-3724)
```rust
    fn completion_type(&self) -> CompletionType {
        CompletionType::Filename
    }
```
- IndexCrud.doc_long (crates/core/flags/defs.rs:3569-3617)
```rust
    fn doc_long(&self) -> &'static str {
        r"
Create or update an index for the files and directories given on the command
line. When no path is given, this recursively indexes the current working
directory. The files added to the index are exactly those selected by
ripgrep's normal traversal and filtering options.
.sp
An existing file is re-indexed only when its modification time is newer than
the time at which it was indexed. Use \flag{x-force} to re-index every selected
file, including files on file systems with unreliable or deliberately
preserved modification times. A previously indexed path that is no longer
accessible is removed from the index.
.sp
ripgrep chooses the index location in the following order:
.sp
.IP 1. 4
The path given by \flag{x-path}.
.sp
.IP 2. 4
The path in the \fBRIPGREP_INDEX_PATH\fP environment variable.
.sp
.IP 3. 4
A valid index in a \fB.ripgrep\fP directory in the current working directory.
.sp
.IP 4. 4
A valid index in a \fB.ripgrep\fP directory in the nearest parent of the current
working directory.
.sp
.IP 5. 4
A \fB.ripgrep\fP directory in the current working directory, which is created
when necessary.
.PP
If the final location already exists but does not contain a valid index, then
ripgrep reports an error.
.sp
For example, this creates or incrementally updates an index for the current
directory:
.sp
.EX
    rg --x-crud
.EE
.sp
This updates one path in the same index:
.sp
.EX
    rg --x-crud path/to/file
.EE
"
    }
```
- IndexForce.doc_short (crates/core/flags/defs.rs:3663-3665)
```rust
    fn doc_short(&self) -> &'static str {
        r"Force selected files to be re-indexed."
    }
```
- Matcher.capture_index (crates/matcher/src/lib.rs:611-613)
```rust
    fn capture_index(&self, _name: &str) -> Option<usize> {
        None
    }
```
- Mode (crates/core/flags/lowargs.rs:156-169)
```rust
pub(crate) enum Mode {
    /// ripgrep will execute a search of some kind.
    Search(SearchMode),
    /// Create or update a search index.
    Index(IndexMode),
    /// Show the files that *would* be searched, but don't actually search
    /// them.
    Files,
    /// List all file type definitions configured, including the default file
    /// types and any additional file types added to the command line.
    Types,
    /// Generate various things like the man page and completion files.
    Generate(GenerateMode),
}
```
- Stack.steal (crates/ignore/src/walk.rs:1695-1707)
```rust
    fn steal(&self) -> Option<Message> {
        // For fairness, try to steal from index + 1, index + 2, ... len - 1,
        // then wrap around to 0, 1, ... index - 1.
        let (left, right) = self.stealers.split_at(self.index);
        // Don't steal from ourselves
        let right = &right[1..];

        right
            .iter()
            .chain(left.iter())
            .map(|s| s.steal_batch_and_pop(&self.deque))
            .find_map(|s| s.success())
    }
```
- jaccard_index (crates/core/flags/parse.rs:454-460)
```rust
fn jaccard_index(ngrams1: &BagOfWords<'_>, ngrams2: &BagOfWords<'_>) -> f64 {
    let union = u32::try_from(ngrams1.union(ngrams2).count())
        .expect("fewer than u32::MAX flags");
    let intersection = u32::try_from(ngrams1.intersection(ngrams2).count())
        .expect("fewer than u32::MAX flags");
    f64::from(intersection) / f64::from(union)
}
```
- RegexMatcher.capture_index (crates/matcher/tests/util.rs:56-58)
```rust
    fn capture_index(&self, name: &str) -> Option<usize> {
        self.names.get(name).copied()
    }
```
- RegexMatcher.capture_index (crates/pcre2/src/matcher.rs:339-341)
```rust
    fn capture_index(&self, name: &str) -> Option<usize> {
        self.names.get(name).copied()
    }
```
- Parser.find_short (crates/core/flags/parse.rs:293-302)
```rust
    fn find_short(&self, ch: char) -> FlagLookup<'_> {
