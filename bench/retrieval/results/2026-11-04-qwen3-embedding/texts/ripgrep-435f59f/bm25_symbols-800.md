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
