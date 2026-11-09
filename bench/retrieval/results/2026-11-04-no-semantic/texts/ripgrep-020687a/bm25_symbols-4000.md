- NoRequireGit.doc_long (crates/core/flags/defs.rs:5225-5238)
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
- IncrementalIgnore.matched_with_errors_impl (crates/ignore/src/incremental.rs:224-308)
```rust
    fn matched_with_errors_impl(
        &mut self,
        relative: &Path,
        is_dir: bool,
        errs: &mut PartialErrorBuilder,
    ) -> IncrementalMatch {
        // We short-circuit here when our matcher corresponds to `Stdin` in
        // order to always return a non-match. This is somewhat redundant with
        // `root_ignore()` which does this too, but we do it here so that it
        // always happens, e.g., before depth filtering.
        if relative.is_absolute() || matches!(self.ignore, RootIgnore::Stdin) {
            return IncrementalMatch::none(is_dir);
        }

        let (mut satisfies_min, mut edge_max) = (true, false);
        if self.options.min_depth.is_some() || self.options.max_depth.is_some()
        {
            let depth = relative
                .components()
                .filter_map(|component| match component {
                    std::path::Component::CurDir => None,
                    component => Some(component),
                })
                .count();
            satisfies_min =
                self.options.min_depth.is_none_or(|min| depth >= min);
            let satisfies_max =
                self.options.max_depth.is_none_or(|max| depth <= max);
            edge_max = self.options.max_depth.is_some_and(|max| depth == max);
            // When we have a file that isn't past our min depth, we can give
            // up right away.
            if !is_dir && !satisfies_min {
                return IncrementalMatch::ignore().not_within_depth();
            }
            // Same for *anything* that exceeds our max depth.
            if !satisfies_max {
                return IncrementalMatch::ignore().not_within_depth();
            }
        }

        // When the path is invalid in some way, we bail out early to avoid
        // potentially doing a stat call below.
        let (mut mat, valid) = self
            .matched_with_errors_ignore(relative, is_dir, errs)
            .map(|mat| (mat, true))
            .unwrap_or_else(|| (IncrementalMatch::none(is_dir), false));

        if !is_dir
            && !mat.is_ignore()
            && valid
            && let Some(max_filesize) = self.options.max_filesize
        {
            let path = self.root().join(relative);
            let result = if self.options.follow_links {
                path.metadata()
            } else {
                path.symlink_metadata()
            };
            match result {
                Ok(md) if md.len() > max_filesize => {
                    return IncrementalMatch::ignore();
                }
                Ok(_) => {}
                Err(err) => {
                    // Record the error but otherwise fall through
                    let err = Error::from(err);
                    errs.push(err.with_path(path));
                }
            }
        }

        // We still need to tag our `mat` if it doesn't pass the depth filter,
        // or if it's a directory on the edge of a depth filter. This can only
        // happen when the path is reported as a directory. Otherwise regular
        // files are always handled above.
        if is_dir {
            if !satisfies_min {
                mat = mat.not_within_depth();
            }
            if edge_max {
                mat = mat.no_descent();
            }
        }
        mat
    }
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
- Extractor.extract_repetition (crates/regex/src/literal.rs:264-315)
```rust
    fn extract_repetition(&self, rep: &hir::Repetition) -> TSeq {
        let mut subseq = self.extract(&rep.sub);
        match *rep {
            hir::Repetition { min: 0, max, greedy, .. } => {
                // When 'max=1', we can retain exactness, since 'a?' is
                // equivalent to 'a|'. Similarly below, 'a??' is equivalent to
                // '|a'.
                if max != Some(1) {
                    subseq.make_inexact();
                }
                let mut empty = TSeq::singleton(Literal::exact(vec![]));
                if !greedy {
                    std::mem::swap(&mut subseq, &mut empty);
                }
                self.union(subseq, &mut empty)
            }
            hir::Repetition { min, max: Some(max), .. } if min == max => {
                assert!(min > 0); // handled above
                let limit =
                    u32::try_from(self.limit_repeat).unwrap_or(u32::MAX);
                let mut seq = TSeq::singleton(Literal::exact(vec![]));
                for _ in 0..std::cmp::min(min, limit) {
                    if seq.is_inexact() {
                        break;
                    }
                    seq = self.cross(seq, subseq.clone());
                }
                if usize::try_from(min).is_err() || min > limit {
                    seq.make_inexact();
                }
                seq
            }
            hir::Repetition { min, max: Some(max), .. } if min < max => {
                assert!(min > 0); // handled above
                let limit =
                    u32::try_from(self.limit_repeat).unwrap_or(u32::MAX);
                let mut seq = TSeq::singleton(Literal::exact(vec![]));
                for _ in 0..std::cmp::min(min, limit) {
                    if seq.is_inexact() {
                        break;
                    }
                    seq = self.cross(seq, subseq.clone());
                }
                seq.make_inexact();
                seq
            }
            hir::Repetition { .. } => {
                subseq.make_inexact();
                subseq
            }
        }
    }
```
- RegexSetStrategy.find_matches (crates/globset/src/lib.rs:986-995)
```rust
    fn find_matches(
        &self,
        candidate: &Candidate<'_>,
    ) -> PoolGuard<'_, PatternSet, PatternSetPoolFn> {
        let input = regex_automata::Input::new(candidate.path.as_bytes());
        let mut patset = self.patset.get();
        patset.clear();
        self.matcher.which_overlapping_matches(&input, &mut patset);
        patset
    }
```
- escape (crates/globset/src/lib.rs:1115-1132)
```rust
pub fn escape(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            // note that ! does not need escaping because it is only special
            // inside brackets
            '?' | '*' | '[' | ']' | '{' | '}' => {
                escaped.push('[');
                escaped.push(c);
                escaped.push(']');
            }
            c => {
                escaped.push(c);
            }
        }
    }
    escaped
}
```
- print_stats (crates/core/main.rs:440-489)
```rust
fn print_stats<W: Write>(
    mode: SearchMode,
    stats: &grep::printer::Stats,
    started: std::time::Instant,
    mut wtr: W,
) -> std::io::Result<()> {
    let elapsed = std::time::Instant::now().duration_since(started);
    if matches!(mode, SearchMode::JSON) {
        // We specifically match the format laid out by the JSON printer in
        // the grep-printer crate. We simply "extend" it with the 'summary'
        // message type.
        serde_json::to_writer(
            &mut wtr,
            &serde_json::json!({
                "type": "summary",
                "data": {
                    "stats": stats,
                    "elapsed_total": {
                        "secs": elapsed.as_secs(),
                        "nanos": elapsed.subsec_nanos(),
                        "human": format!("{:0.6}s", elapsed.as_secs_f64()),
                    },
                }
            }),
        )?;
        write!(wtr, "\n")
    } else {
        write!(
            wtr,
            "
{matches} matches
{lines} matched lines
{searches_with_match} files contained matches
{searches} files searched
{bytes_printed} bytes printed
{bytes_searched} bytes searched
{search_time:0.6} seconds spent searching
{process_time:0.6} seconds total
",
            matches = stats.matches(),
            lines = stats.matched_lines(),
            searches_with_match = stats.searches_with_match(),
            searches = stats.searches(),
            bytes_printed = stats.bytes_printed(),
            bytes_searched = stats.bytes_searched(),
            search_time = stats.elapsed().as_secs_f64(),
            process_time = elapsed.as_secs_f64(),
        )
    }
}
```
- new_regex_set (crates/globset/src/lib.rs:287-304)
```rust
fn new_regex_set(pats: Vec<String>) -> Result<Regex, Error> {
    let syntax = regex_automata::util::syntax::Config::new()
        .utf8(false)
        .dot_matches_new_line(true);
    let config = Regex::config()
        .match_kind(regex_automata::MatchKind::All)
        .utf8_empty(false)
        .nfa_size_limit(Some(10 * (1 << 20)))
        .hybrid_cache_capacity(10 * (1 << 20));
    Regex::builder()
        .syntax(syntax)
        .configure(config)
        .build_many(&pats)
        .map_err(|err| Error {
            glob: None,
            kind: ErrorKind::Regex(err.to_string()),
        })
}
```
- Gitignore (crates/ignore/src/gitignore.rs:81-88)
```rust
pub struct Gitignore {
    set: GlobSet,
    root: PathBuf,
    globs: Vec<Glob>,
    num_ignores: u64,
    num_whitelists: u64,
    matches: Option<Arc<Pool<Vec<usize>>>>,
}
```
- WalkBuilder.build (crates/ignore/src/walk.rs:593-644)
```rust
    pub fn build(&self) -> Walk {
        let follow_links = self.follow_links;
        let max_depth = self.max_depth;
        let min_depth = self.min_depth;
        let sorter = self.sorter.clone();
        let its = self
            .paths
            .iter()
            .map(move |p| {
                if p == Path::new("-") {
                    (p.to_path_buf(), None)
                } else {
                    let mut wd = WalkDir::new(p);
                    wd = wd.follow_links(follow_links || p.is_file());
                    wd = wd.same_file_system(self.same_file_system);
                    if let Some(max_depth) = max_depth {
                        wd = wd.max_depth(max_depth);
                    }
                    if let Some(min_depth) = min_depth {
                        wd = wd.min_depth(min_depth);
                    }
                    if let Some(ref sorter) = sorter {
                        match sorter.clone() {
                            Sorter::ByName(cmp) => {
                                wd = wd.sort_by(move |a, b| {
                                    cmp(a.file_name(), b.file_name())
                                });
                            }
                            Sorter::ByPath(cmp) => {
                                wd = wd.sort_by(move |a, b| {
                                    cmp(a.path(), b.path())
                                });
                            }
                        }
                    }
                    (p.to_path_buf(), Some(WalkEventIter::from(wd)))
                }
            })
            .collect::<Vec<_>>()
            .into_iter();
        let ig_root = self.build_ignore();
        Walk {
            its,
            it: None,
            ig_root: ig_root.clone(),
            ig: ig_root.clone(),
            max_depth: self.max_depth,
            max_filesize: self.max_filesize,
            skip: self.skip.clone(),
            filter: self.filter.clone(),
        }
    }
```
- WalkBuilder.build_parallel (crates/ignore/src/walk.rs:701-715)
```rust
    pub fn build_parallel(&self) -> WalkParallel {
        let ig_root = self.build_ignore();
        WalkParallel {
            paths: self.paths.clone().into_iter(),
            ig_root,
            max_depth: self.max_depth,
            min_depth: self.min_depth,
            max_filesize: self.max_filesize,
            follow_links: self.follow_links,
            same_file_system: self.same_file_system,
            threads: self.threads,
            skip: self.skip.clone(),
            filter: self.filter.clone(),
        }
    }
```
- optimize (crates/regex/src/literal.rs:982-1008)
```rust
    fn optimize() {
        // This gets a common prefix that isn't too short.
        let s = e(r"foobarfoobar|foobar|foobarzfoobar|foobarfoobar");
        assert_eq!(seq([I("foobar")]), s);

        // This also finds a common prefix, but since it's only one byte, it
        // prefers the multiple literals.
        let s = e(r"abba|akka|abccba");
        assert_eq!(exact(["abba", "akka", "abccba"]), s);

        let s = e(r"sam|samwise");
        assert_eq!(seq([E("sam")]), s);

        // The empty string is poisonous, so our seq becomes infinite, even
        // though all literals are exact.
        let s = e(r"foobarfoo|foo||foozfoo|foofoo");
        assert_eq!(Seq::infinite(), s);

        // A space is also poisonous, so our seq becomes infinite. But this
        // only gets triggered when we don't have a completely exact sequence.
        // When the sequence is exact, spaces are okay, since we presume that
        // any prefilter will match a space more quickly than the regex engine.
        // (When the sequence is exact, there's a chance of the prefilter being
        // used without needing the regex engine at all.)
        let s = e(r"foobarfoo|foo| |foofoo");
        assert_eq!(Seq::infinite(), s);
    }
```
- new_regex (crates/globset/src/lib.rs:271-285)
```rust
fn new_regex(pat: &str) -> Result<Regex, Error> {
    let syntax = regex_automata::util::syntax::Config::new()
        .utf8(false)
        .dot_matches_new_line(true);
    let config = Regex::config()
        .utf8_empty(false)
        .nfa_size_limit(Some(10 * (1 << 20)))
        .hybrid_cache_capacity(10 * (1 << 20));
    Regex::builder().syntax(syntax).configure(config).build(pat).map_err(
        |err| Error {
            glob: Some(pat.to_string()),
            kind: ErrorKind::Regex(err.to_string()),
        },
    )
}
```
- WalkBuilder.min_depth (crates/ignore/src/walk.rs:744-753)
```rust
    pub fn min_depth(&mut self, depth: Option<usize>) -> &mut WalkBuilder {
        self.min_depth = depth;
        if self.max_depth.is_some()
            && self.min_depth.is_some()
            && self.min_depth > self.max_depth
        {
            self.min_depth = self.max_depth;
        }
        self
    }
```
- NoIgnoreExclude.doc_long (crates/core/flags/defs.rs:4835-4840)
```rust
    fn doc_long(&self) -> &'static str {
        r"
Don't respect filter rules from files that are manually configured for the repository.
For example, this includes \fBgit\fP's \fB.git/info/exclude\fP.
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
- WalkBuilder.max_depth (crates/ignore/src/walk.rs:730-739)
```rust
    pub fn max_depth(&mut self, depth: Option<usize>) -> &mut WalkBuilder {
        self.max_depth = depth;
        if self.min_depth.is_some()
            && self.max_depth.is_some()
            && self.max_depth < self.min_depth
        {
            self.max_depth = self.min_depth;
        }
        self
    }
```
