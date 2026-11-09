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
            self.matched_with_errors_impl(path.as_ref(), is_dir, &mut errs);
        (matched, errs.into_error_option())
    }

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

    fn matched_with_errors_ignore(
        &mut self,
        relative: &Path,
        is_dir: bool,
        errs: &mut PartialErrorBuilder,
    ) -> Option<IncrementalMatch> {
        let mut components = relative
            .components()
            .filter_map(|component| match component {
                std::path::Component::CurDir => None,
                component => Some(component),
            })
            .peekable();
        components.peek()?;

        // If the exact parent is cached, then all of its ancestors have
        // already been checked. An allowed cache entry is the matcher to use
        // for children of that directory, while an ignored cache entry is
        // terminal for every descendant. In the usual case, this avoids both
        // walking every component and allocating a relative directory path.
        //
        // Only try the fast path when the final component is a normal path
        // component. Invalid paths are handled by the component walk below.
        let has_normal_final_component =
            relative.components().next_back().is_some_and(|component| {
                matches!(component, std::path::Component::Normal(_))
            });
        if has_normal_final_component && let Some(parent) = relative.parent() {
            match self.dirs.get(parent) {
                Some(CachedDir::Allowed(ignore)) => {
                    return Some(self.match_path(ignore, relative, is_dir));
                }
                Some(CachedDir::Ignored) => {
                    return Some(IncrementalMatch::ignore());
                }
                None => {}
            }
        }

        let mut ignore = self.root_ignore(errs)?;
        let mut dir = PathBuf::new();
        while let Some(component) = components.next() {
            match component {
                std::path::Component::ParentDir
                | std::path::Component::RootDir
                | std::path::Component::Prefix(_) => {
                    return None;
                }
                std::path::Component::CurDir => continue,
                std::path::Component::Normal(_) => {}
            }
            if components.peek().is_none() {
                break;
            }
            dir.push(component.as_os_str());
            match self.dirs.get(&dir) {
                Some(CachedDir::Allowed(cached)) => {
                    ignore = cached.clone();
                    continue;
                }
                Some(CachedDir::Ignored) => {
                    return Some(IncrementalMatch::ignore());
                }
                None => {}
            }

            let path = self.root.join(&dir);
            let mat = ignore.matched(&path, true);
            let is_hidden =
                self.options.hidden && mat.is_none() && is_hidden_path(&path);
            if mat.is_ignore() || is_hidden {
                self.dirs.insert(dir.clone(), CachedDir::Ignored);
                return Some(IncrementalMatch::ignore());
            }
            let (child, err) = ignore.add_child(&path);
            errs.maybe_push(err);
            self.dirs.insert(dir.clone(), CachedDir::Allowed(child.clone()));
            ignore = child;
        }

        Some(self.match_path(&ignore, relative, is_dir))
    }

    fn match_path(
        &self,
        ignore: &Ignore,
        relative: &Path,
        is_dir: bool,
    ) -> IncrementalMatch {
        let path = self.root.join(relative);
        let mut mat = IncrementalMatch::from_match(
            ignore.matched(&path, is_dir).map(|_| ()),
            is_dir,
        );
        // Whether a file is hidden or not has low precedence in filtering. We
        // only check it if we haven't matched anything above. This permits
        // callers to whitelist hidden files or directories.
        if self.options.hidden && mat.is_none() && is_hidden_path(&path) {
            mat = IncrementalMatch::ignore();
        }
        mat
    }

    fn root_ignore(
        &mut self,
        errs: &mut PartialErrorBuilder,
    ) -> Option<Ignore> {
        let ignore = match self.ignore {
            RootIgnore::Unloaded(ref ignore) => ignore.clone(),
            RootIgnore::Loaded(ref ignore) => return Some(ignore.clone()),
            RootIgnore::NotDirectory | RootIgnore::Stdin => return None,
        };
        if !self.root.is_dir() {
            self.ignore = RootIgnore::NotDirectory;
            return None;
        }

        let (parents, err) = ignore.add_parents(&self.root);
        errs.maybe_push(err);
        let (root, err) = parents.add_child(&self.root);
        errs.maybe_push(err);
        self.ignore = RootIgnore::Loaded(root.clone());
        Some(root)
    }
}

/// The result of an incremental match.
///
/// This is similar to [`Match`] in that it reports whether a file path should
/// be ignored, whitelisted or didn't match anything at all. It also has extra
/// data, such as whether a directory matched but should not be descended into.
///
/// Generally speaking, callers that only care about specific files can stick
/// to the `is_none()`, `is_ignore()` or `is_whitelist()` predicates. Directory
/// entries are more complicated because we sometimes want to yield directories
/// to descend into, but not actually visit (e.g., for the minimum depth
/// filter). Or, we may want to yield a directory to visit but not descend into
/// (e.g., for the maximum depth filter).
#[derive(Clone, Debug)]
pub struct IncrementalMatch {
    mat: Match<()>,
    should_descend: bool,
    is_within_depth: bool,
}

impl IncrementalMatch {
    fn none(is_dir: bool) -> IncrementalMatch {
        IncrementalMatch {
            mat: Match::None,
            should_descend: is_dir,
            is_within_depth: true,
        }
    }

    fn ignore() -> IncrementalMatch {
        IncrementalMatch {
            mat: Match::Ignore(()),
            should_descend: false,
            is_within_depth: true,
        }
    }

    fn from_match(mat: Match<()>, is_dir: bool) -> IncrementalMatch {
        let should_descend = is_dir && !mat.is_ignore();
