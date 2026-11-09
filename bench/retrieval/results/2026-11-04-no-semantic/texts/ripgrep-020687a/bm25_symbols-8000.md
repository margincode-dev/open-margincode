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
- RegexMatcherBuilder.build_many (crates/regex/src/matcher.rs:53-85)
```rust
    pub fn build_many<P: AsRef<str>>(
        &self,
        patterns: &[P],
    ) -> Result<RegexMatcher, Error> {
        let mut chir = self.config.build_many(patterns)?;
        // 'whole_line' is a strict subset of 'word', so when it is enabled,
        // we don't need to both with any specific to word matching.
        if chir.config().whole_line {
            chir = chir.into_whole_line();
        } else if chir.config().word {
            chir = chir.into_word();
        }
        let regex = chir.to_regex()?;
        log::trace!("final regex: {:?}", chir.hir().to_string());

        let non_matching_bytes = chir.non_matching_bytes();
        // If we can pick out some literals from the regex, then we might be
        // able to build a faster regex that quickly identifies candidate
        // matching lines. The regex engine will do what it can on its own, but
        // we can specifically do a little more when a line terminator is set.
        // For example, for a regex like `\w+foo\w+`, we can look for `foo`,
        // and when a match is found, look for the line containing `foo` and
        // then run the original regex on only that line. (In this case, the
        // regex engine is likely to handle this case for us since it's so
        // simple, but the idea applies.)
        let fast_line_regex = InnerLiterals::new(&chir, &regex).one_regex()?;

        // We override the line terminator in case the configured HIR doesn't
        // support it.
        let mut config = self.config.clone();
        config.line_terminator = chir.line_terminator();
        Ok(RegexMatcher { config, regex, fast_line_regex, non_matching_bytes })
    }
```
- TSeq.is_good (crates/regex/src/literal.rs:553-565)
```rust
    fn is_good(&self) -> bool {
        if self.has_poisonous_literal() {
            return false;
        }
        let Some(min) = self.min_literal_len() else { return false };
        let Some(len) = self.len() else { return false };
        // If we have some very short literals, then let's require that our
        // sequence is itself very small.
        if min <= 1 {
            return len <= 3;
        }
        min >= 2 && len <= 64
    }
```
- WalkBuilder.fmt (crates/ignore/src/walk.rs:524-542)
```rust
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WalkBuilder")
            .field("paths", &self.paths)
            .field("ig_builder", &self.ig_builder)
            .field("max_depth", &self.max_depth)
            .field("min_depth", &self.min_depth)
            .field("max_filesize", &self.max_filesize)
            .field("follow_links", &self.follow_links)
            .field("same_file_system", &self.same_file_system)
            .field("sorter", &"<...>")
            .field("threads", &self.threads)
            .field("skip", &self.skip)
            .field("filter", &"<...>")
            .field(
                "global_gitignores_relative_to",
                &self.global_gitignores_relative_to,
            )
            .finish()
    }
```
- find_iter_at_in_context (crates/printer/src/util.rs:473-530)
```rust
pub(crate) fn find_iter_at_in_context<M, F>(
    searcher: &Searcher,
    matcher: M,
    mut bytes: &[u8],
    range: std::ops::Range<usize>,
    mut matched: F,
) -> io::Result<()>
where
    M: Matcher,
    F: FnMut(Match) -> bool,
{
    // This strange dance is to account for the possibility of look-ahead in
    // the regex. The problem here is that mat.bytes() doesn't include the
    // lines beyond the match boundaries in multi-line mode, which means that
    // when we try to rediscover the full set of matches here, the regex may no
    // longer match if it required some look-ahead beyond the matching lines.
    //
    // PCRE2 (and the grep-matcher interfaces) has no way of specifying an end
    // bound of the search. So we kludge it and let the regex engine search the
    // rest of the buffer... But to avoid things getting too crazy, we cap the
    // buffer.
    //
    // If it weren't for multi-line mode, then none of this would be needed.
    // Alternatively, if we refactored the grep interfaces to pass along the
    // full set of matches (if available) from the searcher, then that might
    // also help here. But that winds up paying an upfront unavoidable cost for
    // the case where matches don't need to be counted. So then you'd have to
    // introduce a way to pass along matches conditionally, only when needed.
    // Yikes.
    //
    // Maybe the bigger picture thing here is that the searcher should be
    // responsible for finding matches when necessary, and the printer
    // shouldn't be involved in this business in the first place. Sigh. Live
    // and learn. Abstraction boundaries are hard.
    let is_multi_line = searcher.multi_line_with_matcher(&matcher);
    if is_multi_line {
        if bytes[range.end..].len() >= MAX_LOOK_AHEAD {
            bytes = &bytes[..range.end + MAX_LOOK_AHEAD];
        }
    } else {
        // When searching a single line, we should remove the line terminator.
        // Otherwise, it's possible for the regex (via look-around) to observe
        // the line terminator and not match because of it.
        let mut m = Match::new(0, range.end);
        // No need to remember the line terminator as we aren't doing a replace
        // here.
        trim_line_terminator(searcher, bytes, &mut m);
        bytes = &bytes[..m.end()];
    }
    matcher
        .find_iter_at(bytes, range.start, |m| {
            if m.start() >= range.end {
                return false;
            }
            matched(m)
        })
        .map_err(io::Error::error_message)
}
```
- big_error_multi_line (crates/searcher/src/searcher/glue.rs:679-697)
```rust
    fn big_error_multi_line() {
        let mut haystack = String::new();
        haystack.push_str("a\n");
        // Pick an arbitrary number above the capacity.
        for _ in 0..(4 * (DEFAULT_BUFFER_CAPACITY + 7)) {
            haystack.push_str("zzz\n");
        }
        haystack.push_str("a\n");

        let matcher = RegexMatcher::new("a");
        let mut sink = KitchenSink::new();
        let mut searcher = SearcherBuilder::new()
            .multi_line(true)
            .heap_limit(Some(haystack.len())) // actually need one more byte
            .build();
        let result =
            searcher.search_reader(&matcher, haystack.as_bytes(), &mut sink);
        assert!(result.is_err());
    }
```
- Gitignore.strip (crates/ignore/src/gitignore.rs:286-315)
```rust
    fn strip<'a, P: 'a + AsRef<Path> + ?Sized>(
        &'a self,
        path: &'a P,
    ) -> &'a Path {
        let mut path = path.as_ref();
        // A leading ./ is completely superfluous. We also strip it from
        // our gitignore root path, so we need to strip it from our candidate
        // path too.
        if let Some(p) = strip_prefix("./", path) {
            path = p;
        }
        // Strip any common prefix between the candidate path and the root
        // of the gitignore, to make sure we get relative matching right.
        // BUT, a file name might not have any directory components to it,
        // in which case, we don't want to accidentally strip any part of the
        // file name.
        //
        // As an additional special case, if the root is just `.`, then we
        // shouldn't try to strip anything, e.g., when path begins with a `.`.
        if self.root != Path::new(".") && !is_file_name(path) {
            if let Some(p) = strip_prefix(&self.root, path) {
                path = p;
                // If we're left with a leading slash, get rid of it.
                if let Some(p) = strip_prefix("/", path) {
                    path = p;
                }
            }
        }
        path
    }
```
- Types (crates/ignore/src/types.rs:165-181)
```rust
pub struct Types {
    /// All of the file type definitions, sorted lexicographically by name.
    defs: Vec<FileTypeDef>,
    /// All of the selections made by the user.
    selections: Vec<Selection<FileTypeDef>>,
    /// Whether there is at least one Selection::Select in our selections.
    /// When this is true, a Match::None is converted to Match::Ignore.
    has_selected: bool,
    /// A mapping from glob index in the set to two indices. The first is an
    /// index into `selections` and the second is an index into the
    /// corresponding file type definition's list of globs.
    glob_to_selection: Vec<(usize, usize)>,
    /// The set of all glob selections, used for actual matching.
    set: GlobSet,
    /// Temporary storage for globs that match.
    matches: Arc<Pool<Vec<usize>>>,
}
```
- Searcher.fill_multi_line_buffer_from_reader (crates/searcher/src/searcher/mod.rs:975-1028)
```rust
    fn fill_multi_line_buffer_from_reader<R: io::Read, S: Sink>(
        &self,
        mut read_from: R,
    ) -> Result<(), S::Error> {
        assert!(self.config.multi_line);

        let mut buf = self.multi_line_buffer.borrow_mut();
        buf.clear();

        // If we don't have a heap limit, then we can defer to std's
        // read_to_end implementation...
        let heap_limit = match self.config.heap_limit {
            Some(heap_limit) => heap_limit,
            None => {
                read_from
                    .read_to_end(&mut *buf)
                    .map_err(S::Error::error_io)?;
                return Ok(());
            }
        };
        if heap_limit == 0 {
            return Err(S::Error::error_io(alloc_error(heap_limit)));
        }

        // ... otherwise we need to roll our own. This is likely quite a bit
        // slower than what is optimal, but we avoid worry about memory safety
        // until there's a compelling reason to speed this up.
        buf.resize(cmp::min(DEFAULT_BUFFER_CAPACITY, heap_limit), 0);
        let mut pos = 0;
        loop {
            let nread = match read_from.read(&mut buf[pos..]) {
                Ok(nread) => nread,
                Err(ref err) if err.kind() == io::ErrorKind::Interrupted => {
                    continue;
                }
                Err(err) => return Err(S::Error::error_io(err)),
            };
            if nread == 0 {
                buf.resize(pos, 0);
                return Ok(());
            }

            pos += nread;
            if buf[pos..].is_empty() {
                let additional = heap_limit - buf.len();
                if additional == 0 {
                    return Err(S::Error::error_io(alloc_error(heap_limit)));
                }
                let limit = buf.len() + additional;
                let doubled = 2 * buf.len();
                buf.resize(cmp::min(doubled, limit), 0);
            }
        }
    }
```
- ErrorKind (crates/globset/src/lib.rs:167-196)
```rust
pub enum ErrorKind {
    /// **DEPRECATED**.
    ///
    /// This error used to occur for consistency with git's glob specification,
    /// but the specification now accepts all uses of `**`. When `**` does not
    /// appear adjacent to a path separator or at the beginning/end of a glob,
    /// it is now treated as two consecutive `*` patterns. As such, this error
    /// is no longer used.
    InvalidRecursive,
    /// Occurs when a character class (e.g., `[abc]`) is not closed.
    UnclosedClass,
    /// Occurs when a range in a character (e.g., `[a-z]`) is invalid. For
    /// example, if the range starts with a lexicographically larger character
    /// than it ends with.
    InvalidRange(char, char),
    /// Occurs when a `}` is found without a matching `{`.
    UnopenedAlternates,
    /// Occurs when a `{` is found without a matching `}`.
    UnclosedAlternates,
    /// **DEPRECATED**.
    ///
    /// This error used to occur when an alternating group was nested inside
    /// another alternating group, e.g., `{{a,b},{c,d}}`. However, this is now
    /// supported and as such this error cannot occur.
    NestedAlternates,
    /// Occurs when an unescaped '\' is found at the end of a glob.
    DanglingEscape,
    /// An error associated with parsing or compiling a regex.
    Regex(String),
}
```
- WalkParallel (crates/ignore/src/walk.rs:1411-1422)
```rust
pub struct WalkParallel {
    paths: std::vec::IntoIter<PathBuf>,
    ig_root: Ignore,
    max_filesize: Option<u64>,
    max_depth: Option<usize>,
    min_depth: Option<usize>,
    follow_links: bool,
    same_file_system: bool,
    threads: usize,
    skip: Option<Arc<Handle>>,
    filter: Option<Filter>,
}
```
- ConfiguredHIR.to_regex (crates/regex/src/config.rs:242-260)
```rust
    pub(crate) fn to_regex(&self) -> Result<Regex, Error> {
        let meta = Regex::config()
            .utf8_empty(false)
            .nfa_size_limit(Some(self.config.size_limit))
            // We don't expose a knob for this because the one-pass DFA is
            // usually not a perf bottleneck for ripgrep. But we give it some
            // extra room than the default.
            .onepass_size_limit(Some(10 * (1 << 20)))
            // Same deal here. The default limit for full DFAs is VERY small,
            // but with ripgrep we can afford to spend a bit more time on
            // building them I think.
            .dfa_size_limit(Some(1 * (1 << 20)))
            .dfa_state_limit(Some(1_000))
            .hybrid_cache_capacity(self.config.dfa_size_limit);
        Regex::builder()
            .configure(meta)
            .build_from_hir(&self.hir)
            .map_err(Error::regex)
    }
```
- WalkBuilder.empty (crates/ignore/src/walk.rs:560-575)
```rust
    pub fn empty() -> WalkBuilder {
        WalkBuilder {
            paths: vec![],
            ig_builder: IgnoreBuilder::new(),
            max_depth: None,
            min_depth: None,
            max_filesize: None,
            follow_links: false,
            same_file_system: false,
            sorter: None,
            threads: 0,
            skip: None,
            filter: None,
            global_gitignores_relative_to: OnceLock::new(),
        }
    }
```
- Worker (crates/ignore/src/walk.rs:1721-1754)
```rust
struct Worker<'s> {
    /// The caller's callback.
    visitor: Box<dyn ParallelVisitor + 's>,
    /// A work-stealing stack of work to do.
    ///
    /// We use a stack instead of a channel because a stack lets us visit
    /// directories in depth first order. This can substantially reduce peak
    /// memory usage by keeping both the number of file paths and gitignore
    /// matchers in memory lower.
    stack: Stack,
    /// Whether all workers should terminate at the next opportunity. Note
    /// that we need this because we don't want other `Work` to be done after
    /// we quit. We wouldn't need this if have a priority channel.
    quit_now: Arc<AtomicBool>,
    /// The number of currently active workers.
    active_workers: Arc<AtomicUsize>,
    /// The maximum depth of directories to descend. A value of `0` means no
    /// descension at all.
    max_depth: Option<usize>,
    /// The minimum depth of directories to descend.
    min_depth: Option<usize>,
    /// The maximum size a searched file can be (in bytes). If a file exceeds
    /// this size it will be skipped.
    max_filesize: Option<u64>,
    /// Whether to follow symbolic links or not. When this is enabled, loop
    /// detection is performed.
    follow_links: bool,
    /// A file handle to skip, currently is either `None` or stdout, if it's
    /// a file and it has been requested to skip files identical to stdout.
    skip: Option<Arc<Handle>>,
    /// A predicate applied to dir entries. If true, the entry and all
    /// children will be skipped.
    filter: Option<Filter>,
}
```
- big_error_one_line (crates/searcher/src/searcher/glue.rs:659-676)
```rust
    fn big_error_one_line() {
        let mut haystack = String::new();
        haystack.push_str("a\n");
        // Pick an arbitrary number above the capacity.
        for _ in 0..(4 * (DEFAULT_BUFFER_CAPACITY + 7)) {
            haystack.push_str("zzz\n");
        }
        haystack.push_str("a\n");

        let matcher = RegexMatcher::new("a");
        let mut sink = KitchenSink::new();
        let mut searcher = SearcherBuilder::new()
            .heap_limit(Some(3)) // max line length is 4, one byte short
            .build();
        let result =
            searcher.search_reader(&matcher, haystack.as_bytes(), &mut sink);
        assert!(result.is_err());
    }
```
- trim_ascii_prefix (crates/printer/src/util.rs:452-471)
```rust
pub(crate) fn trim_ascii_prefix(
    line_term: LineTerminator,
    slice: &[u8],
    range: Match,
