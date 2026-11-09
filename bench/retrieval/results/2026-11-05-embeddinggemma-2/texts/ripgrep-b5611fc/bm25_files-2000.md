### crates/ignore/src/incremental.rs
```rust
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::OnceLock,
};

use crate::{
    Error, Match, PartialErrorBuilder, dir::Ignore, pathutil::is_hidden_path,
};

/// A cached matcher for checking paths against hierarchical ignore files.
///
/// An `IncrementalIgnore` is built from a [`crate::WalkBuilder`]. Unlike a
/// recursive walk, it can check individual paths while still respecting the
/// ignore files in every relevant parent directory. Matchers for directories
/// are compiled on first use and then retained for later queries.
/// Each matcher corresponds to exactly one root configured on the builder,
/// and paths passed to it are interpreted relative to that root.
/// A matcher for the special `-` root representing standard input is inert
/// and always returns a non-match.
///
/// The matcher checks path-based filters in the same precedence order as
/// a traversal. This includes glob overrides, `.ignore`, `.gitignore`,
/// `.git/info/exclude`, global and explicitly added ignore files, custom
/// ignore file names and file type selections. It does not apply filters that
/// require a directory entry or other traversal state, such as custom entry
/// predicates. Hidden-file detection, minimum and maximum depth limits and the
/// maximum file size are applied.
///
/// A matcher is a snapshot at directory granularity. Once the ignore files in
/// a directory have been loaded, edits to those files are not observed. Build
/// a new matcher to reload them.
///
/// # Warning
///
/// The incremental path checking here necessarily needs to do a lot more work
/// per path matched. Callers should _not_ use this to run directory traversal.
/// This is intended to avoid the work of re-traversing an entire directory
/// tree when only a few changes are detected. (For example, in response to
/// file additions or deletions.)
///
/// # Example
///
/// ```rust,no_run
/// use ignore::WalkBuilder;
///
/// let mut builder = WalkBuilder::new(".");
/// builder.add_custom_ignore_filename(".rgignore");
/// let mut matchers = builder.build_matchers();
/// let matcher = &mut matchers[0];
///
/// if matcher.matched("src/generated.rs", false).is_ignore() {
///     println!("ignored");
/// }
/// ```
#[derive(Clone, Debug)]
pub struct IncrementalIgnore {
    /// The root exactly as it was given to `WalkBuilder`.
    root: PathBuf,
    /// The normalized root used only by the opt-in normalization routine.
    normalized_root: OnceLock<Option<PathBuf>>,
    /// The matcher for the configured root directory, loaded on first use.
    ignore: RootIgnore,
    /// Directory paths relative to `root`, excluding the root itself.
    dirs: HashMap<PathBuf, CachedDir>,
    /// Options for additional filtering beyond gitignore.
    options: IncrementalIgnoreOptions,
}

/// The options for a matcher, mostly meant to duplicate as much as we can from
/// `WalkParallel`.
#[derive(Clone, Debug)]
pub(crate) struct IncrementalIgnoreOptions {
    pub(crate) min_depth: Option<usize>,
    pub(crate) max_depth: Option<usize>,
    pub(crate) max_filesize: Option<u64>,
    pub(crate) hidden: bool,
    pub(crate) follow_links: bool,
}

#[derive(Clone, Debug)]
enum RootIgnore {
    Unloaded(Ignore),
    Loaded(Ignore),
    NotDirectory,
    Stdin,
}

/// Cached traversal state for a directory relative to the configured root.
///
/// The presence of an entry means that the directory and every ancestor
/// between it and the root have already been checked.
#[derive(Clone, Debug)]
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

impl IncrementalIgnore {
    pub(crate) fn new(
        root: PathBuf,
        ignore: Ignore,
        options: IncrementalIgnoreOptions,
    ) -> IncrementalIgnore {
        // File traversal special cases `-` to search stdin, so we recognize
        // it here for completeness too. In particular, we really want
        // `WalkBuilder::build_matchers` to return a matcher for every root,
        // even when it's a simple file (handled automatically) or when it's
        // stdin (necessarily special cased).
        //
        // If callers need to search a file or directory named `-`, then they
        // can use `./-`. As is the case for file traversal too.
        let ignore = if root == Path::new("-") {
            RootIgnore::Stdin
        } else {
            RootIgnore::Unloaded(ignore)
        };
        IncrementalIgnore {
            root,
            normalized_root: OnceLock::new(),
            ignore,
            dirs: HashMap::new(),
            options,
        }
    }

    /// Return the root that paths matched by this matcher are relative to.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Normalize `path` and return it relative to this matcher's root.
    ///
    /// This returns `None` when `path` cannot be made absolute or
    /// when it is known to be outside this matcher's root. Unlike
    /// [`IncrementalIgnore::matched`], this performs absolute path conversion,
    /// lexical normalization and allocation. It is intended as an opt-in
    /// convenience for callers that do not already have root-relative paths.
    ///
    /// Note that `.` is interpreted relative to the process level current
    /// working directory. It is _not_ interpreted relative to the root of
    /// this matcher.
    ///
    /// Note also that this may reject paths that only differ in casing. For
    /// example, if the root path for this matcher is `/FOO` but the provided
    /// path is `/foo/bar`, then this may return `None`. Callers must ensure
    /// casing is consistent between the path provided and the root path for
    /// this matcher.
    pub fn normalize<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf> {
        if matches!(self.ignore, RootIgnore::Stdin) {
            return None;
        }
        let path = normalize_absolute(path.as_ref())?;
        let root = self
            .normalized_root
            .get_or_init(|| normalize_absolute(&self.root))
            .as_ref()?;
        path.strip_prefix(root).ok().map(Path::to_path_buf)
    }

    /// Match a root-relative path against ignore files in its directory and
    /// all relevant parent directories.
    ///
    /// `is_dir` should be true when `path` should be matched as a directory.
    ///
    /// For the return value, use [`IncrementalMatch::is_ignore`],
    /// [`IncrementalMatch::is_whitelist`] or [`IncrementalMatch::is_none`] to
    /// inspect it.
    ///
    /// Matchers for previously unseen directories are loaded and cached during
    /// this call. Errors encountered while loading ignore files are logged. To
    /// receive those errors, use [`IncrementalIgnore::matched_with_errors`].
    ///
    /// `path` must be relative to this matcher's root and must not contain a
    /// parent directory (`..`) component. Behavior is unspecified when these
    /// preconditions are violated. Callers with an absolute path or with a
    /// path containing `.` or `..` may use [`IncrementalIgnore::normalize`]
    /// to get a path satisfying these preconditions. In all cases, a relative
    /// path is *assumed* to be relative to the root of this ignore matcher.
    ///
    /// In general, it is intended that callers doing recursive directory
    /// traversal on the root of this matcher may provide relative paths to
    /// this routine *without* calling [`IncrementalIgnore::normalize`].
    ///
    /// The empty path represents the explicitly configured root and also
    /// returns non-match, consistent with recursive traversal where a root is
    /// always treated as being at depth zero.
    pub fn matched<P: AsRef<Path>>(
        &mut self,
        path: P,
        is_dir: bool,
    ) -> IncrementalMatch {
        let (matched, err) = self.matched_with_errors(path, is_dir);
        if let Some(err) = err {
            log::debug!("error while loading ignore files: {err}");
        }
        matched
    }

    /// Match a root-relative path and return errors encountered while loading
    /// ignore files.
    ///
    /// This is equivalent to [`IncrementalIgnore::matched`], except that
    /// it returns any errors from newly loaded ignore files. Loading can
    /// partially succeed, so valid rules are always applied to the returned
    /// match even when an error is present.
    pub fn matched_with_errors<P: AsRef<Path>>(
        &mut self,
        path: P,
        is_dir: bool,
    ) -> (IncrementalMatch, Option<Error>) {
        let mut errs = PartialErrorBuilder::default();
        let matched =
