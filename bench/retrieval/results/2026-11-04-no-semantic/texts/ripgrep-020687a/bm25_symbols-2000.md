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
