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
        if !ch.is_ascii() {
            return FlagLookup::UnrecognizedShort(ch);
        }
        let byte = u8::try_from(ch).unwrap();
        let Some(index) = self.map.find(&[byte]) else {
            return FlagLookup::UnrecognizedShort(ch);
        };
        FlagLookup::Match(&self.info[index])
    }
```
- check_indexing_allowed (crates/core/flags/defs.rs:7950-7955)
```rust
fn check_indexing_allowed() -> anyhow::Result<()> {
    if cfg!(feature = "unstable-index") {
        return Ok(());
    }
    anyhow::bail!("{}", crate::flags::INDEXING_NOT_SUPPORTED)
}
```
- Stack (crates/ignore/src/walk.rs:1642-1649)
```rust
struct Stack {
    /// This thread's index.
    index: usize,
    /// The thread-local stack.
    deque: Deque<Message>,
    /// The work stealers.
    stealers: Arc<[Stealer<Message>]>,
}
```
- interpolate_string (crates/matcher/src/interpolate.rs:185-209)
```rust
    fn interpolate_string(
        mut name_to_index: Vec<(&'static str, usize)>,
        caps: Vec<&'static str>,
        replacement: &str,
    ) -> String {
        name_to_index.sort_by_key(|x| x.0);

        let mut dst = vec![];
        interpolate(
            replacement.as_bytes(),
            |i, dst| {
                if let Some(&s) = caps.get(i) {
                    dst.extend(s.as_bytes());
                }
            },
            |name| -> Option<usize> {
                name_to_index
                    .binary_search_by_key(&name, |x| x.0)
                    .ok()
                    .map(|i| name_to_index[i].1)
            },
            &mut dst,
        );
        String::from_utf8(dst).unwrap()
    }
```
- LiteralStrategy.add (crates/globset/src/lib.rs:717-719)
```rust
    fn add(&mut self, global_index: usize, lit: String) {
        self.0.entry(lit.into_bytes()).or_insert(vec![]).push(global_index);
    }
```
- ExtensionStrategy.add (crates/globset/src/lib.rs:787-789)
```rust
    fn add(&mut self, global_index: usize, ext: String) {
        self.0.entry(ext.into_bytes()).or_insert(vec![]).push(global_index);
    }
```
- BasenameLiteralStrategy.add (crates/globset/src/lib.rs:749-751)
```rust
    fn add(&mut self, global_index: usize, lit: String) {
        self.0.entry(lit.into_bytes()).or_insert(vec![]).push(global_index);
    }
```
- RequiredExtensionStrategyBuilder.add (crates/globset/src/lib.rs:1078-1083)
```rust
    fn add(&mut self, global_index: usize, ext: String, regex: String) {
        self.0
            .entry(ext.into_bytes())
            .or_insert(vec![])
            .push((global_index, regex));
    }
```
- MultiStrategyBuilder.add (crates/globset/src/lib.rs:1027-1033)
```rust
    fn add(&mut self, global_index: usize, literal: String) {
        if literal.len() > self.longest {
            self.longest = literal.len();
        }
        self.map.push(global_index);
        self.literals.push(literal);
    }
```
- captures (crates/matcher/tests/test_matcher.rs:82-94)
```rust
fn captures() {
    let matcher = matcher(r"(?P<a>\w+)\s+(?P<b>\w+)");
    assert_eq!(matcher.capture_count(), 3);
    assert_eq!(matcher.capture_index("a"), Some(1));
    assert_eq!(matcher.capture_index("b"), Some(2));
    assert_eq!(matcher.capture_index("nada"), None);

    let mut caps = matcher.new_captures().unwrap();
    assert!(matcher.captures(b" homer simpson ", &mut caps).unwrap());
    assert_eq!(caps.get(0), Some(m(1, 14)));
    assert_eq!(caps.get(1), Some(m(1, 6)));
    assert_eq!(caps.get(2), Some(m(7, 14)));
}
```
- no_captures (crates/matcher/tests/test_matcher.rs:156-174)
```rust
fn no_captures() {
    let matcher = matcher_no_caps(r"(?P<a>\w+)\s+(?P<b>\w+)");
    assert_eq!(matcher.capture_count(), 0);
    assert_eq!(matcher.capture_index("a"), None);
    assert_eq!(matcher.capture_index("b"), None);
    assert_eq!(matcher.capture_index("nada"), None);

    let mut caps = matcher.new_captures().unwrap();
    assert!(!matcher.captures(b"homer simpson", &mut caps).unwrap());

    let mut called = false;
    matcher
        .captures_iter(b"homer simpson", &mut caps, |_| {
            called = true;
            true
        })
        .unwrap();
    assert!(!called);
}
```
- Captures.interpolate (crates/matcher/src/lib.rs:441-460)
```rust
    fn interpolate<F>(
        &self,
        name_to_index: F,
        haystack: &[u8],
        replacement: &[u8],
        dst: &mut Vec<u8>,
    ) where
        F: FnMut(&str) -> Option<usize>,
    {
        interpolate(
            replacement,
            |i, dst| {
                if let Some(range) = self.get(i) {
                    dst.extend(&haystack[range]);
                }
            },
            name_to_index,
            dst,
        )
    }
```
- sgramq (crates/index/src/literal.rs:865-867)
```rust
    fn sgramq(pattern: &str) -> String {
        format!("{:?}", gramq(pattern))
    }
```
- StandardImpl.write_colored_matches (crates/printer/src/standard.rs:1247-1288)
```rust
    fn write_colored_matches(
        &self,
        bytes: &[u8],
        mut line: Match,
        matches: &[Match],
        match_index: &mut usize,
    ) -> io::Result<()> {
        self.trim_line_terminator(bytes, &mut line);
        if matches.is_empty() {
            self.write(&bytes[line])?;
            return Ok(());
        }
        self.start_line_highlight()?;
        while !line.is_empty() {
            if matches[*match_index].end() <= line.start() {
                if *match_index + 1 < matches.len() {
                    *match_index += 1;
                    continue;
                } else {
                    self.end_color_match()?;
                    self.write(&bytes[line])?;
                    break;
                }
            }

            let m = matches[*match_index];
            if line.start() < m.start() {
                let upto = cmp::min(line.end(), m.start());
                self.end_color_match()?;
                self.write(&bytes[line.with_end(upto)])?;
                line = line.with_start(upto);
            } else {
                let upto = cmp::min(line.end(), m.end());
                self.start_color_match()?;
                self.write(&bytes[line.with_end(upto)])?;
                line = line.with_start(upto);
            }
        }
        self.end_color_match()?;
        self.end_line_highlight()?;
        Ok(())
    }
```
- impossible (crates/regex/src/literal.rs:888-909)
```rust
    fn impossible() {
        // N.B. The extractor in this module "optimizes" the sequence and makes
        // it infinite if it isn't "good." An empty sequence (generated by a
        // concatenantion containing an expression that can never match) is
        // considered "not good." Since infinite sequences are not actionably
        // and disable optimizations, this winds up being okay.
        //
        // The literal extractor in regex-syntax doesn't combine these two
        // steps and makes the caller choose to optimize. That is, it returns
        // the sequences as they are. Which in this case, for some of the tests
        // below, would be an empty Seq and not an infinite Seq.
        assert_eq!(Seq::infinite(), e(r"[a&&b]"));
        assert_eq!(Seq::infinite(), e(r"a[a&&b]"));
        assert_eq!(Seq::infinite(), e(r"[a&&b]b"));
        assert_eq!(Seq::infinite(), e(r"a[a&&b]b"));
        assert_eq!(exact(["a", "b"]), e(r"a|[a&&b]|b"));
        assert_eq!(exact(["a", "b"]), e(r"a|c[a&&b]|b"));
        assert_eq!(exact(["a", "b"]), e(r"a|[a&&b]d|b"));
        assert_eq!(exact(["a", "b"]), e(r"a|c[a&&b]d|b"));
        assert_eq!(Seq::infinite(), e(r"[a&&b]*"));
        assert_eq!(exact(["MN"]), e(r"M[a&&b]*N"));
    }
```
- LiteralSet (crates/index/src/literal.rs:505-507)
```rust
struct LiteralSet {
    lits: Vec<BString>,
}
```
- LiteralSet.clear (crates/index/src/literal.rs:524-526)
```rust
    fn clear(&mut self) {
        self.lits.clear();
    }
```
- LiteralSet.len (crates/index/src/literal.rs:619-621)
```rust
    fn len(&self) -> usize {
        self.lits.len()
    }
```
- find_similar_names (crates/core/flags/parse.rs:418-448)
```rust
fn find_similar_names(unrecognized: &str) -> Vec<&'static str> {
    // The jaccard similarity threshold at which we consider two flag names
    // similar enough that it's worth suggesting it to the end user.
    //
    // This value was determined by some ad hoc experimentation. It might need
    // further tweaking.
    const THRESHOLD: f64 = 0.4;

    let mut similar = vec![];
    let bow_given = ngrams(unrecognized);
    for &flag in FLAGS.iter() {
        let name = flag.name_long();
        let bow = ngrams(name);
        if jaccard_index(&bow_given, &bow) >= THRESHOLD {
            similar.push(name);
        }
        if let Some(name) = flag.name_negated() {
            let bow = ngrams(name);
            if jaccard_index(&bow_given, &bow) >= THRESHOLD {
                similar.push(name);
            }
        }
        for name in flag.aliases() {
            let bow = ngrams(name);
            if jaccard_index(&bow_given, &bow) >= THRESHOLD {
                similar.push(name);
            }
        }
    }
    similar
}
```
- Analysis.is_exact (crates/index/src/literal.rs:370-372)
```rust
    fn is_exact(&self) -> bool {
        !self.exact.is_empty()
    }
```
- GramQuery.nothing (crates/index/src/literal.rs:23-25)
```rust
    fn nothing() -> GramQuery {
        GramQuery::Or(vec![])
    }
```
- GramQuery.anything (crates/index/src/literal.rs:27-29)
```rust
    fn anything() -> GramQuery {
        GramQuery::And(vec![])
    }
```
- LiteralSet.new (crates/index/src/literal.rs:510-512)
```rust
    fn new() -> LiteralSet {
        LiteralSet { lits: vec![] }
    }
```
- LiteralSet.canonicalize (crates/index/src/literal.rs:551-554)
```rust
    fn canonicalize(&mut self) {
        self.lits.sort();
        self.lits.dedup();
    }
```
- LiteralSet.is_empty (crates/index/src/literal.rs:615-617)
```rust
    fn is_empty(&self) -> bool {
        self.lits.is_empty()
    }
```
- RequiredExtensionStrategy.matches_into (crates/globset/src/lib.rs:945-960)
```rust
    fn matches_into(
        &self,
        candidate: &Candidate<'_>,
        matches: &mut Vec<usize>,
    ) {
        if candidate.ext.is_empty() {
            return;
        }
        if let Some(regexes) = self.0.get(candidate.ext.as_bytes()) {
            for &(global_index, ref re) in regexes {
                if re.is_match(candidate.path.as_bytes()) {
                    matches.push(global_index);
                }
            }
        }
    }
```
- GramQuery.is_literal (crates/index/src/literal.rs:45-47)
```rust
    fn is_literal(&self) -> bool {
        matches!(*self, GramQuery::Literal(..))
    }
```
- GramQuery.is_and (crates/index/src/literal.rs:49-51)
```rust
    fn is_and(&self) -> bool {
        matches!(*self, GramQuery::And(..))
    }
```
- GramQuery.is_or (crates/index/src/literal.rs:53-55)
```rust
    fn is_or(&self) -> bool {
        matches!(*self, GramQuery::Or(..))
    }
```
- Analysis.save_exact (crates/index/src/literal.rs:382-384)
```rust
    fn save_exact(&mut self) {
        self.query.and_ngrams(self.size, &self.exact);
    }
```
- GramQueryBuilder.default (crates/index/src/literal.rs:761-763)
```rust
    fn default() -> GramQueryBuilder {
        GramQueryBuilder::new()
    }
```
- parse (crates/index/src/literal.rs:854-856)
```rust
    fn parse(pattern: &str) -> Hir {
        ParserBuilder::new().utf8(false).build().parse(pattern).unwrap()
    }
```
- LiteralSet.union (crates/index/src/literal.rs:591-594)
```rust
    fn union(&mut self, o: LiteralSet) {
        self.lits.extend(o.lits);
        self.canonicalize();
    }
```
- LiteralSet.has_only_empty (crates/index/src/literal.rs:631-633)
```rust
    fn has_only_empty(&self) -> bool {
        self.len() == 1 && self.has_empty()
    }
```
- LiteralSet.single (crates/index/src/literal.rs:514-516)
```rust
    fn single(lit: BString) -> LiteralSet {
        LiteralSet { lits: vec![lit] }
    }
```
- LiteralSet.min_len (crates/index/src/literal.rs:623-625)
```rust
    fn min_len(&self) -> usize {
        self.lits.iter().map(|x| x.len()).min().unwrap_or(0)
    }
```
- LiteralSet.has_empty (crates/index/src/literal.rs:627-629)
```rust
    fn has_empty(&self) -> bool {
        self.lits.first().is_some_and(|x| x.is_empty())
    }
```
- Captures (crates/matcher/src/lib.rs:379-461)
```rust
pub trait Captures {
    /// Return the total number of capturing groups. This includes capturing
    /// groups that have not matched anything.
    fn len(&self) -> usize;

    /// Return the capturing group match at the given index. If no match of
    /// that capturing group exists, then this returns `None`.
    ///
    /// When a matcher reports a match with capturing groups, then the first
    /// capturing group (at index `0`) must always correspond to the offsets
    /// for the overall match.
    fn get(&self, i: usize) -> Option<Match>;

    /// Return the overall match for the capture.
    ///
    /// This returns the match for index `0`. That is it is equivalent to
    /// `get(0).unwrap()`
    #[inline]
    fn as_match(&self) -> Match {
        self.get(0).unwrap()
    }

    /// Returns true if and only if these captures are empty. This occurs
    /// when `len` is `0`.
    ///
    /// Note that capturing groups that have non-zero length but otherwise
    /// contain no matching groups are *not* empty.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Expands all instances of `$name` in `replacement` to the corresponding
    /// capture group `name`, and writes them to the `dst` buffer given.
    ///
    /// (Note: If you're looking for a convenient way to perform replacements
    /// with interpolation, then you'll want to use the `replace_with_captures`
    /// method on the `Matcher` trait.)
    ///
    /// `name` may be an integer corresponding to the index of the
    /// capture group (counted by order of opening parenthesis where `0` is the
    /// entire match) or it can be a name (consisting of letters, digits or
    /// underscores) corresponding to a named capture group.
    ///
    /// A `name` is translated to a capture group index via the given
    /// `name_to_index` function. If `name` isn't a valid capture group
    /// (whether the name doesn't exist or isn't a valid index), then it is
    /// replaced with the empty string.
    ///
    /// The longest possible name is used. e.g., `$1a` looks up the capture
    /// group named `1a` and not the capture group at index `1`. To exert
    /// more precise control over the name, use braces, e.g., `${1}a`. In all
    /// cases, capture group names are limited to ASCII letters, numbers and
    /// underscores.
    ///
    /// To write a literal `$` use `$$`.
    ///
    /// Note that the capture group match indices are resolved by slicing
    /// the given `haystack`. Generally, this means that `haystack` should be
    /// the same slice that was searched to get the current capture group
    /// matches.
    #[inline]
    fn interpolate<F>(
        &self,
        name_to_index: F,
        haystack: &[u8],
        replacement: &[u8],
        dst: &mut Vec<u8>,
    ) where
        F: FnMut(&str) -> Option<usize>,
    {
        interpolate(
