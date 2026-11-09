- WalkState (crates/ignore/src/walk.rs:1318-1330)
```rust
pub enum WalkState {
    /// Continue walking as normal.
    Continue,
    /// If the directory entry given is a directory, don't descend into it.
    /// In all other cases, this has no effect.
    Skip,
    /// Quit the entire iterator as soon as possible.
    ///
    /// Note that this is an inherently asynchronous action. It is possible
    /// for more entries to be yielded even after instructing the iterator
    /// to quit.
    Quit,
}
```
- CachedDir (crates/ignore/src/incremental.rs:94-103)
```rust
enum CachedDir {
    /// The directory may be descended into. The matcher includes the ignore
    /// rules loaded through this directory and is therefore the matcher to use
    /// for its children.
    Allowed(Ignore),
    /// The directory may not be descended into because it was ignored by a
    /// path rule or hidden-file filtering. Every descendant is consequently
    /// ignored, and ignore files inside this directory are not loaded.
    Ignored,
}
```
- Hidden.doc_long (crates/core/flags/defs.rs:2868-2885)
```rust
    fn doc_long(&self) -> &'static str {
        r#"
Search hidden files and directories. By default, hidden files and directories
are skipped. Note that if a hidden file or a directory is whitelisted in
an ignore file, then it will be searched even if this flag isn't provided.
Similarly if a hidden file or directory is given explicitly as an argument to
ripgrep.
.sp
A file or directory is considered hidden if its base name starts with a dot
character (\fB.\fP). On operating systems which support a "hidden" file
attribute, like Windows, files with this attribute are also considered hidden.
.sp
Note that \flag{hidden} will include files and folders like \fB.git\fP
regardless of \flag{no-ignore-vcs}. To exclude such paths when using
\flag{hidden}, you must explicitly ignore them using another flag or ignore
file.
"#
    }
```
- Worker (crates/ignore/src/walk.rs:1714-1747)
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
- Sort.doc_long (crates/core/flags/defs.rs:6694-6728)
```rust
    fn doc_long(&self) -> &'static str {
        r"
This flag enables sorting of results in ascending order. The possible values
for this flag are:
.sp
.TP 12
\fBnone\fP
(Default) Do not sort results. Fastest. Can be multi-threaded.
.TP 12
\fBpath\fP
Sort by file path. Always single-threaded. The order is determined by sorting
files in each directory entry during traversal. This means that given the files
\fBa/b\fP and \fBa+\fP, the latter will sort after the former even though
\fB+\fP would normally sort before \fB/\fP.
.TP 12
\fBmodified\fP
Sort by the last modified time on a file. Always single-threaded.
.TP 12
\fBaccessed\fP
Sort by the last accessed time on a file. Always single-threaded.
.TP 12
\fBcreated\fP
Sort by the creation time on a file. Always single-threaded.
.PP
If the chosen (manually or by-default) sorting criteria isn't available on your
system (for example, creation time is not available on ext4 file systems), then
ripgrep will attempt to detect this, print an error and exit without searching.
.sp
To sort results in reverse or descending order, use the \flag{sortr} flag.
Also, this flag overrides \flag{sortr}.
.sp
Note that sorting results currently always forces ripgrep to abandon
parallelism and run in a single thread.
"
    }
```
- Sortr.doc_long (crates/core/flags/defs.rs:6795-6830)
```rust
    fn doc_long(&self) -> &'static str {
        r"
This flag enables sorting of results in descending order. The possible values
for this flag are:
.sp
.TP 12
\fBnone\fP
(Default) Do not sort results. Fastest. Can be multi-threaded.
.TP 12
\fBpath\fP
Sort by file path. Always single-threaded. The order is determined by sorting
files in each directory entry during traversal. This means that given the files
\fBa/b\fP and \fBa+\fP, the latter will sort before the former even though
\fB+\fP would normally sort after \fB/\fP when doing a reverse lexicographic
sort.
.TP 12
\fBmodified\fP
Sort by the last modified time on a file. Always single-threaded.
.TP 12
\fBaccessed\fP
Sort by the last accessed time on a file. Always single-threaded.
.TP 12
\fBcreated\fP
Sort by the creation time on a file. Always single-threaded.
.PP
If the chosen (manually or by-default) sorting criteria isn't available on your
system (for example, creation time is not available on ext4 file systems), then
ripgrep will attempt to detect this, print an error and exit without searching.
.sp
To sort results in ascending order, use the \flag{sort} flag. Also, this flag
overrides \flag{sort}.
.sp
Note that sorting results currently always forces ripgrep to abandon
parallelism and run in a single thread.
"
    }
```
- Haystack.is_explicit (crates/core/haystack.rs:131-139)
```rust
    pub(crate) fn is_explicit(&self) -> bool {
        // stdin is obvious. When an entry has a depth of 0, that means it
        // was explicitly provided to our directory iterator, which means it
        // was in turn explicitly provided by the end user. The !is_dir check
        // means that we want to search files even if their symlinks, again,
        // because they were explicitly provided. (And we never want to try
        // to search a directory.)
        self.is_stdin() || (self.dent.depth() == 0 && !self.is_dir())
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
    compiled: Arc<RwLock<HashMap<OsString, Weak<IgnoreInner>>>>,
    /// The path to the directory that this matcher was built from.
    dir: PathBuf,
    /// An override matcher (default is empty).
    overrides: Arc<Override>,
    /// A file type matcher.
    types: Arc<Types>,
    /// The parent directory to match next.
    ///
    /// If this is the root directory or there are otherwise no more
    /// directories to match, then `parent` is `None`.
    parent: Option<Arc<IgnoreInner>>,
    /// Whether this is an absolute parent matcher, as added by add_parent.
    is_absolute_parent: bool,
    /// The directory that gitignores should be interpreted relative to.
    ///
    /// Usually this is the directory containing the gitignore file. But in
    /// some cases, like for global gitignores or for gitignores specified
    /// explicitly, this should generally be set to the current working
    /// directory. This is only used for global gitignores or "explicit"
    /// gitignores.
    ///
    /// When `None`, this means the CWD could not be determined or is unknown.
    /// In this case, global gitignore files are ignored because they otherwise
    /// cannot be matched correctly.
    global_gitignores_relative_to: Option<PathBuf>,
    /// Explicit global ignore matchers specified by the caller.
    explicit_ignores: Arc<Vec<Gitignore>>,
    /// Ignore files used in addition to `.ignore`
    custom_ignore_filenames: Arc<Vec<OsString>>,
    /// The matcher for custom ignore files
    custom_ignore_matcher: Gitignore,
    /// The matcher for .ignore files.
    ignore_matcher: Gitignore,
    /// A global gitignore matcher, usually from $XDG_CONFIG_HOME/git/ignore.
    git_global_matcher: Arc<Gitignore>,
    /// The matcher for .gitignore files.
    git_ignore_matcher: Gitignore,
    /// Special matcher for `.git/info/exclude` files.
    git_exclude_matcher: Gitignore,
    /// Whether this directory contains a .git sub-directory.
    has_git: bool,
    /// Ignore config.
    opts: IgnoreOptions,
}
```
- HiArgs.walk_builder (crates/core/flags/hiargs.rs:885-929)
```rust
    pub(crate) fn walk_builder(&self) -> anyhow::Result<ignore::WalkBuilder> {
        let mut builder = ignore::WalkBuilder::new(&self.paths.paths[0]);
        for path in self.paths.paths.iter().skip(1) {
            builder.add(path);
        }
        if !self.no_ignore_files {
            for path in self.ignore_file.iter() {
                if let Some(err) = builder.add_ignore(path) {
                    ignore_message!("{err}");
                }
            }
        }
        builder
            .max_depth(self.max_depth)
            .follow_links(self.follow)
            .max_filesize(self.max_filesize)
            .threads(self.threads)
            .same_file_system(self.one_file_system)
            .skip_stdout(matches!(self.mode, Mode::Search(_)))
            .overrides(self.globs.clone())
            .types(self.types.clone())
            .hidden(!self.hidden)
            .parents(!self.no_ignore_parent)
            .ignore(!self.no_ignore_dot)
            .git_global(!self.no_ignore_vcs && !self.no_ignore_global)
            .git_ignore(!self.no_ignore_vcs)
            .git_exclude(!self.no_ignore_vcs && !self.no_ignore_exclude)
            .require_git(!self.no_require_git)
            .ignore_case_insensitive(self.ignore_file_case_insensitive)
            .current_dir(&self.cwd);
        if !self.no_ignore_dot {
            builder.add_custom_ignore_filename(".rgignore");
        }
        // When we want to sort paths lexicographically in ascending order,
        // then we can actually do this during directory traversal itself.
        // Otherwise, sorting is done by collecting all paths, sorting them and
        // then searching them.
        if let Some(ref sort) = self.sort {
            assert_eq!(1, self.threads, "sorting implies single threaded");
            if !sort.reverse && matches!(sort.kind, SortModeKind::Path) {
                builder.sort_by_file_name(|a, b| a.cmp(b));
            }
        }
        Ok(builder)
    }
```
- NoIgnoreParent.doc_long (crates/core/flags/defs.rs:5025-5032)
```rust
    fn doc_long(&self) -> &'static str {
        r"
When this flag is set, filter rules from ignore files found in parent
directories are not respected. By default, ripgrep will ascend the parent
directories of the current working directory to look for any applicable ignore
files that should be applied. In some cases this may not be desirable.
"
    }
```
- same_file_system (crates/ignore/src/walk.rs:2565-2591)
```rust
    fn same_file_system() {
        use super::device_num;

        // If for some reason /sys doesn't exist or isn't a directory, just
        // skip this test.
        if !Path::new("/sys").is_dir() {
            return;
        }

        // If our test directory actually isn't a different volume from /sys,
        // then this test is meaningless and we shouldn't run it.
        let td = tmpdir();
        if device_num(td.path()).unwrap() == device_num("/sys").unwrap() {
            return;
        }

        mkdirp(td.path().join("same_file"));
        symlink("/sys", td.path().join("same_file").join("alink"));

        // Create a symlink to sys and enable following symlinks. If the
        // same_file_system option doesn't work, then this probably will hit a
        // permission error. Otherwise, it should just skip over the symlink
        // completely.
        let mut builder = WalkBuilder::new(td.path());
        builder.follow_links(true).same_file_system(true);
        assert_paths(td.path(), &builder, &["same_file", "same_file/alink"]);
    }
```
- MmapMode (crates/core/flags/lowargs.rs:637-650)
```rust
pub(crate) enum MmapMode {
    /// This instructs ripgrep to use heuristics for selecting when to and not
    /// to use memory maps for searching.
    #[default]
    Auto,
    /// This instructs ripgrep to always try memory maps when possible. (Memory
    /// maps are not possible to use in all circumstances, for example, for
    /// virtual files.)
    AlwaysTryMmap,
    /// Never use memory maps under any circumstances. This includes even
    /// when multi-line search is enabled where ripgrep will read the entire
    /// contents of a file on to the heap before searching it.
    Never,
}
```
- WalkBuilder (crates/ignore/src/walk.rs:488-512)
```rust
pub struct WalkBuilder {
    paths: Vec<PathBuf>,
    ig_builder: IgnoreBuilder,
    max_depth: Option<usize>,
    min_depth: Option<usize>,
    max_filesize: Option<u64>,
    follow_links: bool,
    same_file_system: bool,
    sorter: Option<Sorter>,
    threads: usize,
    skip: Option<Arc<Handle>>,
    filter: Option<Filter>,
    /// The directory that gitignores should be interpreted relative to.
    ///
    /// Usually this is the directory containing the gitignore file. But in
    /// some cases, like for global gitignores or for gitignores specified
    /// explicitly, this should generally be set to the current working
    /// directory. This is only used for global gitignores or "explicit"
    /// gitignores.
    ///
    /// When `None`, the CWD is fetched from `std::env::current_dir()`. If
    /// that fails, then global gitignores are ignored (an error is logged).
    global_gitignores_relative_to:
        OnceLock<Result<PathBuf, Arc<std::io::Error>>>,
}
```
- Message (crates/ignore/src/walk.rs:1540-1547)
```rust
enum Message {
    /// A work item corresponds to a directory that should be descended into.
    /// Work items for entries that should be skipped or ignored should not
    /// be produced.
    Work(Work),
    /// This instruction indicates that the worker should quit.
    Quit,
}
```
- Walk.skip_entry (crates/ignore/src/walk.rs:1147-1181)
```rust
    fn skip_entry(&self, ent: &DirEntry) -> Result<bool, Error> {
        if ent.depth() == 0 {
            return Ok(false);
        }
        // We ensure that trivial skipping is done before any other potentially
        // expensive operations (stat, filesystem other) are done. This seems
        // like an obvious optimization but becomes critical when filesystem
        // operations even as simple as stat can result in significant
        // overheads; an example of this was a bespoke filesystem layer in
        // Windows that hosted files remotely and would download them on-demand
        // when particular filesystem operations occurred. Users of this system
        // who ensured correct file-type filters were being used could still
        // get unnecessary file access resulting in large downloads.
        if should_skip_entry(&self.ig, ent) {
            return Ok(true);
        }
        if let Some(ref stdout) = self.skip {
            if path_equals(ent, stdout)? {
                return Ok(true);
            }
        }
        if self.max_filesize.is_some() && !ent.is_dir() {
            return Ok(skip_filesize(
                self.max_filesize.unwrap(),
                ent.path(),
                &ent.metadata().ok(),
            ));
        }
        if let Some(Filter(filter)) = &self.filter {
            if !filter(ent) {
                return Ok(true);
            }
        }
        Ok(false)
    }
```
- directly_ignored_directory_is_not_descended (crates/ignore/src/incremental.rs:751-761)
```rust
    fn directly_ignored_directory_is_not_descended() {
        let td = tmpdir();
        mkdirp(td.path().join(".git"));
        mkdirp(td.path().join("blocked"));
        wfile(td.path().join(".gitignore"), "blocked/\n");

        let mut m = one_matcher(&builder(td.path()));
        let dir = matchedd(&mut m, "blocked");
        assert!(dir.is_ignore());
        assert!(!dir.should_descend());
    }
```
- hidden_directories_outside_depth_limits (crates/ignore/src/incremental.rs:1268-1285)
```rust
    fn hidden_directories_outside_depth_limits() {
        let td = tmpdir();
        mkdirp(td.path().join("a/.hidden"));

        let mut b = builder(td.path());
        b.min_depth(Some(3));
        let mut m = one_matcher(&b);
        let dir = matchedd(&mut m, "a/.hidden");
        assert!(dir.is_ignore());
        assert!(!dir.is_within_depth());

        let mut b = builder(td.path());
        b.max_depth(Some(1));
        let mut m = one_matcher(&b);
        let dir = matchedd(&mut m, "a/.hidden");
        assert!(dir.is_ignore());
        assert!(!dir.is_within_depth());
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
- Glob.doc_long (crates/core/flags/defs.rs:2602-2629)
```rust
    fn doc_long(&self) -> &'static str {
        r#"
Include or exclude files and directories for searching that match the given
glob. This always overrides any other ignore logic. Multiple glob flags may
be used. Globbing rules match \fB.gitignore\fP globs. Precede a glob with a
\fB!\fP to exclude it. If multiple globs match a file or directory, the glob
given later in the command line takes precedence.
.sp
As an extension, globs support specifying alternatives:
.BI "\-g '" ab{c,d}* '
is equivalent to
.BI "\-g " "abc " "\-g " abd.
Empty alternatives like
.BI "\-g '" ab{,c} '
are not currently supported. Note that this syntax extension is also currently
enabled in \fBgitignore\fP files, even though this syntax isn't supported by
git itself. ripgrep may disable this syntax extension in gitignore files, but
it will always remain available via the \flag{glob} flag.
.sp
When this flag is set, every file and directory is applied to it to test for
a match. For example, if you only want to search in a particular directory
\fIfoo\fP, then
.BI "\-g " foo
is incorrect because \fIfoo/bar\fP does not match
the glob \fIfoo\fP. Instead, you should use
.BI "\-g '" foo/** '.
"#
    }
```
- WalkBuilder.build_parallel (crates/ignore/src/walk.rs:700-714)
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
- WalkBuilder.build (crates/ignore/src/walk.rs:593-643)
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
            max_filesize: self.max_filesize,
            skip: self.skip.clone(),
            filter: self.filter.clone(),
        }
    }
```
- MaxDepth.doc_long (crates/core/flags/defs.rs:4337-4349)
```rust
    fn doc_long(&self) -> &'static str {
        r"
This flag limits the depth of directory traversal to \fINUM\fP levels beyond
the paths given. A value of \fB0\fP only searches the explicitly given paths
themselves.
.sp
For example, \fBrg --max-depth 0 \fP\fIdir/\fP is a no-op because \fIdir/\fP
will not be descended into. \fBrg --max-depth 1 \fP\fIdir/\fP will search only
the direct children of \fIdir\fP.
.sp
An alternative spelling for this flag is \fB\-\-maxdepth\fP.
"
    }
```
- Worker.run_one (crates/ignore/src/walk.rs:1762-1852)
```rust
    fn run_one(&mut self, mut work: Work) -> WalkState {
        let should_visit = self
            .min_depth
            .map(|min_depth| work.dent.depth() >= min_depth)
            .unwrap_or(true);

        // If the work is not a directory, then we can just execute the
        // caller's callback immediately and move on.
        if work.is_symlink() || !work.is_dir() {
            return if should_visit {
                self.visitor.visit(Ok(work.dent))
            } else {
                WalkState::Continue
            };
        }
        if let Some(err) = work.add_parents() {
            let state = self.visitor.visit(Err(err));
            if state.is_quit() {
                return state;
            }
        }

        let descend = if let Some(root_device) = work.root_device {
            match is_same_file_system(root_device, work.dent.path()) {
                Ok(true) => true,
                Ok(false) => false,
                Err(err) => {
                    let state = self.visitor.visit(Err(err));
                    if state.is_quit() {
                        return state;
                    }
                    false
                }
            }
        } else {
            true
        };

        // Try to read the directory first before we transfer ownership
        // to the provided closure. Do not unwrap it immediately, though,
        // as we may receive an `Err` value e.g. in the case when we do not
        // have sufficient read permissions to list the directory.
        // In that case we still want to provide the closure with a valid
        // entry before passing the error value.
        let depth = work.dent.depth();
        let readdir = if descend && self.max_depth.is_none_or(|m| depth < m) {
            Some(work.read_dir())
        } else {
            work.add_ignore();
            None
        };
        if should_visit {
            let state = self.visitor.visit(Ok(work.dent));
            if !state.is_continue() {
                return state;
            }
        }
        if !descend {
            return WalkState::Skip;
        }

        let readdir = match readdir {
            Some(readdir) => readdir,
            None => return WalkState::Skip,
        };
        let readdir = match readdir {
            Ok(readdir) => readdir,
            Err(err) => {
                return self.visitor.visit(Err(err));
            }
        };

        for result in readdir.entries {
            let state = self.generate_work(
                &work.ignore,
                depth + 1,
                work.root_device,
                result,
            );
            if state.is_quit() {
                return state;
            }
        }
        for err in readdir.errors {
            let state = self.visitor.visit(Err(err));
            if state.is_quit() {
                return state;
            }
        }
        WalkState::Continue
    }
```
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
- WalkBuilder.max_depth (crates/ignore/src/walk.rs:729-738)
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
- IGlob.doc_long (crates/core/flags/defs.rs:3186-3195)
```rust
    fn doc_long(&self) -> &'static str {
        r"
Include or exclude files and directories for searching that match the given
glob. This always overrides any other ignore logic. Multiple glob flags may
be used. Globbing rules match \fB.gitignore\fP globs. Precede a glob with a
\fB!\fP to exclude it. If multiple globs match a file or directory, the glob
given later in the command line takes precedence. Globs used via this flag are
matched case insensitively.
"
    }
```
- IGNORE_FILE (crates/ignore/tests/gitignore_skip_bom.rs:3-3)
```rust
const IGNORE_FILE: &'static str = "tests/gitignore_skip_bom.gitignore";
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
- IncrementalIgnoreOptions (crates/ignore/src/incremental.rs:73-79)
```rust
pub(crate) struct IncrementalIgnoreOptions {
    pub(crate) min_depth: Option<usize>,
    pub(crate) max_depth: Option<usize>,
    pub(crate) max_filesize: Option<u64>,
    pub(crate) hidden: bool,
    pub(crate) follow_links: bool,
}
```
- OneFileSystem.doc_short (crates/core/flags/defs.rs:5469-5471)
```rust
    fn doc_short(&self) -> &'static str {
        r"Skip directories on other file systems."
    }
```
- IgnoreFile.doc_long (crates/core/flags/defs.rs:3304-3318)
```rust
    fn doc_long(&self) -> &'static str {
        r"
Specifies a path to one or more \fBgitignore\fP formatted rules files.
These patterns are applied after the patterns found in \fB.gitignore\fP,
\fB.rgignore\fP and \fB.ignore\fP are applied and are matched relative to the
current working directory. That is, files specified via this flag have lower
precedence than files automatically found in the directory tree. Multiple
additional ignore files can be specified by using this flag repeatedly. When
specifying multiple ignore files, earlier files have lower precedence than
later files.
.sp
If you are looking for a way to include or exclude files and directories
directly on the command line, then use \flag{glob} instead.
"
    }
```
- WalkBuilder.min_depth (crates/ignore/src/walk.rs:743-752)
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
- gitignore_skip_bom (crates/ignore/tests/gitignore_skip_bom.rs:10-17)
```rust
fn gitignore_skip_bom() {
    let mut builder = GitignoreBuilder::new("ROOT");
