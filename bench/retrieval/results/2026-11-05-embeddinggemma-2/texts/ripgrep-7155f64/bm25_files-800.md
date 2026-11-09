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
