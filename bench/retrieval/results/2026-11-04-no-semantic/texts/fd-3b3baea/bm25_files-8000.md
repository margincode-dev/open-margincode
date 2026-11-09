### src/walk.rs
```rust
use std::borrow::Cow;
use std::ffi::OsStr;
use std::io::{self, Write};
use std::mem;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{Result, anyhow};
use crossbeam_channel::{Receiver, RecvTimeoutError, SendError, Sender, bounded};
use etcetera::BaseStrategy;
use ignore::overrides::{Override, OverrideBuilder};
use ignore::{WalkBuilder, WalkParallel, WalkState};
use regex::bytes::Regex;

use crate::config::Config;
use crate::dir_entry::DirEntry;
use crate::exec;
use crate::exit_codes::{ExitCode, merge_exitcodes};
use crate::filesystem;
use crate::output;

/// The receiver thread can either be buffering results or directly streaming to the console.
#[derive(PartialEq)]
enum ReceiverMode {
    /// Receiver is still buffering in order to sort the results, if the search finishes fast
    /// enough.
    Buffering,

    /// Receiver is directly printing results to the output.
    Streaming,
}

/// The Worker threads can result in a valid entry having PathBuf or an error.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum WorkerResult {
    // Errors should be rare, so it's probably better to allow large_enum_variant than
    // to box the Entry variant
    Entry(DirEntry),
    Error(ignore::Error),
}

/// A batch of WorkerResults to send over a channel.
#[derive(Clone)]
struct Batch {
    items: Arc<Mutex<Option<Vec<WorkerResult>>>>,
}

impl Batch {
    fn new() -> Self {
        Self {
            items: Arc::new(Mutex::new(Some(vec![]))),
        }
    }

    fn lock(&self) -> MutexGuard<'_, Option<Vec<WorkerResult>>> {
        self.items.lock().unwrap()
    }
}

impl IntoIterator for Batch {
    type Item = WorkerResult;
    type IntoIter = std::vec::IntoIter<WorkerResult>;

    fn into_iter(self) -> Self::IntoIter {
        self.lock().take().unwrap().into_iter()
    }
}

/// Wrapper that sends batches of items at once over a channel.
struct BatchSender {
    batch: Batch,
    tx: Sender<Batch>,
    limit: usize,
}

impl BatchSender {
    fn new(tx: Sender<Batch>, limit: usize) -> Self {
        Self {
            batch: Batch::new(),
            tx,
            limit,
        }
    }

    /// Check if we need to flush a batch.
    fn needs_flush(&self, batch: Option<&Vec<WorkerResult>>) -> bool {
        match batch {
            // Limit the batch size to provide some backpressure
            Some(vec) => vec.len() >= self.limit,
            // Batch was already taken by the receiver, so make a new one
            None => true,
        }
    }

    /// Add an item to a batch.
    fn send(&mut self, item: WorkerResult) -> Result<(), SendError<()>> {
        let mut batch = self.batch.lock();

        if self.needs_flush(batch.as_ref()) {
            drop(batch);
            self.batch = Batch::new();
            batch = self.batch.lock();
        }

        let items = batch.as_mut().unwrap();
        items.push(item);

        if items.len() == 1 {
            // New batch, send it over the channel
            self.tx
                .send(self.batch.clone())
                .map_err(|_| SendError(()))?;
        }

        Ok(())
    }
}

/// Maximum size of the output buffer before flushing results to the console
const MAX_BUFFER_LENGTH: usize = 1000;
/// Default duration until output buffering switches to streaming.
const DEFAULT_MAX_BUFFER_TIME: Duration = Duration::from_millis(100);

/// Wrapper for the receiver thread's buffering behavior.
struct ReceiverBuffer<'a, W> {
    /// The configuration.
    config: &'a Config,
    /// For shutting down the senders.
    quit_flag: &'a AtomicBool,
    /// The ^C notifier.
    interrupt_flag: &'a AtomicBool,
    /// Receiver for worker results.
    rx: Receiver<Batch>,
    /// Standard output.
    stdout: W,
    /// The current buffer mode.
    mode: ReceiverMode,
    /// The deadline to switch to streaming mode.
    deadline: Instant,
    /// The buffer of quickly received paths.
    buffer: Vec<DirEntry>,
    /// Result count.
    num_results: usize,
}

impl<'a, W: Write> ReceiverBuffer<'a, W> {
    /// Create a new receiver buffer.
    fn new(state: &'a WorkerState, rx: Receiver<Batch>, stdout: W) -> Self {
        let config = &state.config;
        let quit_flag = state.quit_flag.as_ref();
        let interrupt_flag = state.interrupt_flag.as_ref();
        let max_buffer_time = config.max_buffer_time.unwrap_or(DEFAULT_MAX_BUFFER_TIME);
        let deadline = Instant::now() + max_buffer_time;

        Self {
            config,
            quit_flag,
            interrupt_flag,
            rx,
            stdout,
            mode: ReceiverMode::Buffering,
            deadline,
            buffer: Vec::with_capacity(MAX_BUFFER_LENGTH),
            num_results: 0,
        }
    }

    /// Process results until finished.
    fn process(&mut self) -> ExitCode {
        loop {
            if let Err(ec) = self.poll() {
                self.quit_flag.store(true, Ordering::Relaxed);
                return ec;
            }
        }
    }

    /// Receive the next worker result.
    fn recv(&self) -> Result<Batch, RecvTimeoutError> {
        match self.mode {
            ReceiverMode::Buffering => {
                // Wait at most until we should switch to streaming
                self.rx.recv_deadline(self.deadline)
            }
            ReceiverMode::Streaming => {
                // Wait however long it takes for a result
                Ok(self.rx.recv()?)
            }
        }
    }

    /// Wait for a result or state change.
    fn poll(&mut self) -> Result<(), ExitCode> {
        match self.recv() {
            Ok(batch) => {
                for result in batch {
                    match result {
                        WorkerResult::Entry(dir_entry) => {
                            if self.config.quiet {
                                return Err(ExitCode::HasResults(true));
                            }

                            match self.mode {
                                ReceiverMode::Buffering => {
                                    self.buffer.push(dir_entry);
                                    if self.buffer.len() > MAX_BUFFER_LENGTH {
                                        self.stream()?;
                                    }
                                }
                                ReceiverMode::Streaming => {
                                    self.print(&dir_entry)?;
                                }
                            }

                            self.num_results += 1;
                            if let Some(max_results) = self.config.max_results
                                && self.num_results >= max_results
                            {
                                return self.stop();
                            }
                        }
                        WorkerResult::Error(err) => {
                            if self.config.show_filesystem_errors {
                                print_error!("{}", err);
                            }
                        }
                    }
                }

                // If we don't have another batch ready, flush before waiting
                if self.mode == ReceiverMode::Streaming && self.rx.is_empty() {
                    self.flush()?;
                }
            }
            Err(RecvTimeoutError::Timeout) => {
                self.stream()?;
            }
            Err(RecvTimeoutError::Disconnected) => {
                return self.stop();
            }
        }

        Ok(())
    }

    /// Output a path.
    fn print(&mut self, entry: &DirEntry) -> Result<(), ExitCode> {
        if let Err(e) = output::print_entry(&mut self.stdout, entry, self.config)
            && e.kind() != ::std::io::ErrorKind::BrokenPipe
        {
            print_error!("Could not write to output: {}", e);
            return Err(ExitCode::GeneralError);
        }

        if self.interrupt_flag.load(Ordering::Relaxed) {
            // Ignore any errors on flush, because we're about to exit anyway
            let _ = self.flush();
            return Err(ExitCode::KilledBySigint);
        }

        Ok(())
    }

    /// Switch ourselves into streaming mode.
    fn stream(&mut self) -> Result<(), ExitCode> {
        self.mode = ReceiverMode::Streaming;

        let buffer = mem::take(&mut self.buffer);
        for path in buffer {
            self.print(&path)?;
        }

        self.flush()
    }

    /// Stop looping.
    fn stop(&mut self) -> Result<(), ExitCode> {
        if self.mode == ReceiverMode::Buffering {
            self.buffer.sort();
            self.stream()?;
        }

        if self.config.quiet {
            Err(ExitCode::HasResults(self.num_results > 0))
        } else {
            Err(ExitCode::Success)
        }
    }

    /// Flush stdout if necessary.
    fn flush(&mut self) -> Result<(), ExitCode> {
        if self.stdout.flush().is_err() {
            // Probably a broken pipe. Exit gracefully.
            return Err(ExitCode::GeneralError);
        }
        Ok(())
    }
}

/// State shared by the sender and receiver threads.
struct WorkerState {
    /// The search patterns.
    patterns: Vec<Regex>,
    /// The command line configuration.
    config: Config,
    /// Flag for cleanly shutting down the parallel walk
    quit_flag: Arc<AtomicBool>,
    /// Flag specifically for quitting due to ^C
    interrupt_flag: Arc<AtomicBool>,
}

impl WorkerState {
    fn new(patterns: Vec<Regex>, config: Config) -> Self {
        let quit_flag = Arc::new(AtomicBool::new(false));
        let interrupt_flag = Arc::new(AtomicBool::new(false));

        Self {
            patterns,
            config,
            quit_flag,
            interrupt_flag,
        }
    }

    fn build_overrides(&self, paths: &[PathBuf]) -> Result<Override> {
        let first_path = &paths[0];
        let config = &self.config;

        let mut builder = OverrideBuilder::new(first_path);

        for pattern in &config.exclude_patterns {
            builder
                .add(pattern)
                .map_err(|e| anyhow!("Malformed exclude pattern: {}", e))?;
        }

        builder
            .build()
            .map_err(|_| anyhow!("Mismatch in exclude patterns"))
    }

    fn build_walker(&self, paths: &[PathBuf]) -> Result<WalkParallel> {
        let first_path = &paths[0];
        let config = &self.config;
        let overrides = self.build_overrides(paths)?;

        let mut builder = WalkBuilder::new(first_path);
        builder
            .hidden(config.ignore_hidden)
            .ignore(config.read_fdignore)
            .parents(config.read_parent_ignore && (config.read_fdignore || config.read_vcsignore))
            .git_ignore(config.read_vcsignore)
            .git_global(config.read_vcsignore)
            .git_exclude(config.read_vcsignore)
            .require_git(config.require_git_to_read_vcsignore)
            .overrides(overrides)
            .follow_links(config.follow_links)
            // No need to check for supported platforms, option is unavailable on unsupported ones
            .same_file_system(config.one_file_system)
            .max_depth(config.max_depth);

        if config.read_fdignore {
            builder.add_custom_ignore_filename(".fdignore");
        }

        if config.read_global_ignore
            && let Ok(basedirs) = etcetera::choose_base_strategy()
        {
            let global_ignore_file = basedirs.config_dir().join("fd").join("ignore");
            if global_ignore_file.is_file() {
                let result = builder.add_ignore(global_ignore_file);
                match result {
                    Some(ignore::Error::Partial(_)) => (),
                    Some(err) => {
                        print_error!("Malformed pattern in global ignore file. {}.", err);
                    }
                    None => (),
                }
            }
        }

        for ignore_file in &config.ignore_files {
            let result = builder.add_ignore(ignore_file);
            match result {
                Some(ignore::Error::Partial(_)) => (),
                Some(err) => {
                    print_error!("Malformed pattern in custom ignore file. {}.", err);
                }
                None => (),
            }
        }

        for path in &paths[1..] {
            builder.add(path);
        }

        let walker = builder.threads(config.threads).build_parallel();
        Ok(walker)
    }

    /// Run the receiver work, either on this thread or a pool of background
    /// threads (for --exec).
    fn receive(&self, rx: Receiver<Batch>) -> ExitCode {
        let config = &self.config;

        // This will be set to `Some` if the `--exec` argument was supplied.
        if let Some(ref cmd) = config.command {
            if cmd.in_batch_mode() {
                exec::batch(rx.into_iter().flatten(), cmd, config)
            } else {
                thread::scope(|scope| {
                    // Each spawned job will store its thread handle in here.
                    let threads = config.threads;
                    let mut handles = Vec::with_capacity(threads);
                    for _ in 0..threads {
                        let rx = rx.clone();

                        // Spawn a job thread that will listen for and execute inputs.
                        let handle =
                            scope.spawn(|| exec::job(rx.into_iter().flatten(), cmd, config));

                        // Push the handle of the spawned thread into the vector for later joining.
                        handles.push(handle);
                    }
                    let exit_codes = handles.into_iter().map(|handle| handle.join().unwrap());
                    merge_exitcodes(exit_codes)
                })
            }
        } else {
            let stdout = io::stdout().lock();
            let stdout = io::BufWriter::new(stdout);

            ReceiverBuffer::new(self, rx, stdout).process()
        }
    }

    /// Spawn the sender threads.
    fn spawn_senders(&self, walker: WalkParallel, tx: Sender<Batch>) {
        walker.run(|| {
            let patterns = &self.patterns;
            let config = &self.config;
            let quit_flag = self.quit_flag.as_ref();

            let mut limit = 0x100;
            if let Some(cmd) = &config.command
                && !cmd.in_batch_mode()
                && config.threads > 1
            {
                // Evenly distribute work between multiple receivers
                limit = 1;
            }
            let mut tx = BatchSender::new(tx.clone(), limit);

            Box::new(move |entry| {
                if quit_flag.load(Ordering::Relaxed) {
                    return WalkState::Quit;
                }

                if let Ok(e) = &entry {
                    // If the entry is a directory that contains a
                    // "ignore contain" file", we want to skip this
                    // directory.
                    // Check the filetype first to avoid unnecessary
                    // syscalls.
                    if e.file_type().is_some_and(|t| t.is_dir()) {
                        let entry_path = e.path();
                        if config
                            .ignore_contain
                            .iter()
                            .any(|ic| entry_path.join(ic).exists())
                        {
                            return WalkState::Skip;
                        }
                    }
                    if e.depth() == 0 {
                        // Skip the root directory entry.
                        return WalkState::Continue;
                    }
                }
                let entry = match entry {
                    Ok(e) => DirEntry::normal(e),
                    Err(err) => {
                        // The depth has to be read off the error before it is
                        // taken apart, since it is recorded on an inner variant.
                        let depth = err.depth();
                        match err {
                            ignore::Error::WithPath {
                                path,
                                err: inner_err,
                            } if is_broken_symlink(&path, &inner_err) => {
                                DirEntry::broken_symlink(path, depth)
                            }
                            err => {
                                return match tx.send(WorkerResult::Error(err)) {
                                    Ok(_) => WalkState::Continue,
                                    Err(_) => WalkState::Quit,
                                };
                            }
                        }
                    }
                };

                if let Some(min_depth) = config.min_depth
                    && entry.depth().is_none_or(|d| d < min_depth)
                {
                    return WalkState::Continue;
                }

                // Check the name first, since it doesn't require metadata
                let entry_path = entry.path();

                let search_str = search_str_for_entry(entry_path, config.full_path_base.as_deref());

                if !patterns
                    .iter()
                    .all(|pat| pat.is_match(&filesystem::osstr_to_bytes(search_str.as_ref())))
                {
                    return WalkState::Continue;
                }

                // Filter out unwanted extensions.
                if let Some(ref exts_regex) = config.extensions {
                    if let Some(path_str) = entry_path.file_name() {
                        if !exts_regex.is_match(&filesystem::osstr_to_bytes(path_str)) {
                            return WalkState::Continue;
                        }
                    } else {
                        return WalkState::Continue;
                    }
                }

                // Filter out unwanted file types.
                if let Some(ref file_types) = config.file_types
                    && file_types.should_ignore(&entry)
                {
                    return WalkState::Continue;
                }

                #[cfg(unix)]
                {
                    if let Some(ref owner_constraint) = config.owner_constraint {
                        if let Some(metadata) = entry.metadata() {
                            if !owner_constraint.matches(metadata) {
                                return WalkState::Continue;
                            }
                        } else {
                            return WalkState::Continue;
                        }
                    }
                }

                // Filter out unwanted sizes if it is a file and we have been given size constraints.
                if !config.size_constraints.is_empty() {
                    if entry_path.is_file() {
                        if let Some(metadata) = entry.metadata() {
                            let file_size = metadata.len();
                            if config
                                .size_constraints
                                .iter()
                                .any(|sc| !sc.is_within(file_size))
                            {
                                return WalkState::Continue;
                            }
                        } else {
                            return WalkState::Continue;
                        }
                    } else {
                        return WalkState::Continue;
                    }
                }

                // Filter out unwanted modification times
                if !config.time_constraints.is_empty() {
                    let mut matched = false;
                    if let Some(metadata) = entry.metadata()
                        && let Ok(modified) = metadata.modified()
                    {
                        matched = config
                            .time_constraints
                            .iter()
                            .all(|tf| tf.applies_to(&modified));
                    }
                    if !matched {
                        return WalkState::Continue;
                    }
                }

                if config.is_printing()
                    && let Some(ls_colors) = &config.ls_colors
                {
                    // Compute colors in parallel
                    entry.style(ls_colors);
                }

                let send_result = tx.send(WorkerResult::Entry(entry));

                if send_result.is_err() {
                    return WalkState::Quit;
                }

                // Apply pruning.
                if config.prune {
                    return WalkState::Skip;
                }

                WalkState::Continue
            })
        });
    }

    /// Perform the recursive scan.
    fn scan(&self, paths: &[PathBuf]) -> Result<ExitCode> {
        let config = &self.config;
        let walker = self.build_walker(paths)?;

        if config.ls_colors.is_some() && config.is_printing() {
            let quit_flag = Arc::clone(&self.quit_flag);
            let interrupt_flag = Arc::clone(&self.interrupt_flag);

            ctrlc::set_handler(move || {
                quit_flag.store(true, Ordering::Relaxed);

                if interrupt_flag.fetch_or(true, Ordering::Relaxed) {
                    // Ctrl-C has been pressed twice, exit NOW
                    ExitCode::KilledBySigint.exit();
                }
            })
            .unwrap();
        }

        let (tx, rx) = bounded(2 * config.threads);

        let exit_code = thread::scope(|scope| {
            // Spawn the receiver thread(s)
            let receiver = scope.spawn(|| self.receive(rx));

            // Spawn the sender threads.
            self.spawn_senders(walker, tx);

            receiver.join().unwrap()
        });

        if self.interrupt_flag.load(Ordering::Relaxed) {
            Ok(ExitCode::KilledBySigint)
        } else {
            Ok(exit_code)
        }
    }
}

/// Whether a walk error is really a broken symlink rather than a failure worth
/// reporting.
///
/// A symlink whose target is missing is surfaced by the walker as a NotFound
/// error against the link's own path, so it never arrives as an entry. fd still
/// wants to match and print it (see issue #1017), which means recovering it here.
fn is_broken_symlink(path: &Path, err: &ignore::Error) -> bool {
    err.io_error()
        .is_some_and(|io_error| io_error.kind() == io::ErrorKind::NotFound)
        && path
            .symlink_metadata()
            .ok()
            .is_some_and(|m| m.file_type().is_symlink())
}

fn search_str_for_entry<'a>(
    entry_path: &'a std::path::Path,
    full_path_base: Option<&std::path::Path>,
) -> Cow<'a, OsStr> {
    if let Some(cwd) = full_path_base {
        // If full_path_base is some, that means that we need to return
        // the absolute path
        if entry_path.is_absolute() {
            return Cow::Borrowed(entry_path.as_os_str());
        }
        let path = entry_path.strip_prefix(".").unwrap_or(entry_path);
        Cow::Owned(cwd.join(path).into())
    } else {
        match entry_path.file_name() {
            Some(filename) => Cow::Borrowed(filename),
            None => unreachable!(
                "Encountered file system entry without a file name. This should only \
                 happen for paths like 'foo/bar/..' or '/' which are not supposed to \
                 appear in a file system traversal."
            ),
        }
    }
}

/// Recursively scan the given search path for files / pathnames matching the patterns.
///
/// If the `--exec` argument was supplied, this will create a thread pool for executing
/// jobs in parallel from a given command line and the discovered paths. Otherwise, each
/// path will simply be written to standard output.
pub fn scan(paths: &[PathBuf], patterns: Vec<Regex>, config: Config) -> Result<ExitCode> {
    WorkerState::new(patterns, config).scan(paths)
}

#[cfg(test)]
mod tests {
    use super::search_str_for_entry;
    use std::path::{Path, PathBuf};

    #[test]
    fn search_str_for_entry_with_relative_path() {
        let full_path_base = Some(Path::new("/home/user"));
        assert_eq!(
            search_str_for_entry(Path::new("foo/bar"), full_path_base),
            PathBuf::from("/home/user/foo/bar")
        );
    }

    #[test]
    fn search_str_for_entry_strips_dot_prefix() {
        let full_path_base = Some(Path::new("/home/user"));
        assert_eq!(
            search_str_for_entry(Path::new("./foo/bar"), full_path_base),
            PathBuf::from("/home/user/foo/bar")
        );
    }

    #[test]
    fn search_str_for_entry_with_absolute_path() {
        let full_path_base = Some(Path::new("/home/user"));
        assert_eq!(
            search_str_for_entry(Path::new("/absolute/path"), full_path_base),
            PathBuf::from("/absolute/path")
        );
    }

    #[test]
    fn search_str_no_base_dir() {
        assert_eq!(
            search_str_for_entry(Path::new("./foo/bar"), None),
            PathBuf::from("bar")
        );
    }

    #[test]
    fn search_str_no_base_dir_with_plain_relative_path() {
        assert_eq!(
            search_str_for_entry(Path::new("foo/bar"), None),
            PathBuf::from("bar")
        );
    }

    #[test]
    fn search_str_no_base_dir_with_file_in_current_dir() {
        assert_eq!(
            search_str_for_entry(Path::new("foo"), None),
            PathBuf::from("foo")
        );
    }
}
```
### src/cli.rs
```rust
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::anyhow;
use clap::{
    Arg, ArgAction, ArgGroup, ArgMatches, Command, Parser, ValueEnum, error::ErrorKind,
    value_parser,
};
#[cfg(feature = "completions")]
use clap_complete::Shell;
use normpath::PathExt;

use crate::exec::CommandSet;
use crate::filesystem;
#[cfg(unix)]
use crate::filter::OwnerFilter;
use crate::filter::SizeFilter;

#[derive(Parser)]
#[command(
    name = "fd",
    version,
    about = "A program to find entries in your filesystem with regex and glob based matching. By default, fd respects gitignore rules, ignores hidden directories, and is case insensitive.",
    after_long_help = "Bugs can be reported on GitHub: https://github.com/sharkdp/fd/issues",
    max_term_width = 98,
    args_override_self = true,
    group(ArgGroup::new("execs").args(&["exec", "exec_batch", "list_details"]).conflicts_with_all(&[
            "max_results", "quiet", "max_one_result"])),
)]
pub struct Opts {
    /// Include hidden directories and files in the search results (default:
    /// hidden files and directories are skipped). Files and directories are
    /// considered to be hidden if their name starts with a `.` sign (dot).
    /// Any files or directories that are ignored due to the rules described by
    /// --no-ignore are still ignored unless otherwise specified.
    /// The flag can be overridden with --no-hidden.
    #[arg(
        long,
        short = 'H',
        help = "Search hidden files and directories",
        long_help
    )]
    pub hidden: bool,

    /// Overrides --hidden
    #[arg(long, overrides_with = "hidden", hide = true, action = ArgAction::SetTrue)]
    no_hidden: (),

    /// Show search results from files and directories that would otherwise be
    /// ignored by '.gitignore', '.ignore', '.fdignore', or the global ignore file,
    /// The flag can be overridden with --ignore.
    #[arg(
        long,
        short = 'I',
        help = "Do not respect .(git|fd)ignore files",
        long_help
    )]
    pub no_ignore: bool,

    /// Overrides --no-ignore
    #[arg(long, overrides_with = "no_ignore", hide = true, action = ArgAction::SetTrue)]
    ignore: (),

    ///Show search results from files and directories that
    ///would otherwise be ignored by '.gitignore' files.
    ///The flag can be overridden with --ignore-vcs.
    #[arg(
        long,
        hide_short_help = true,
        help = "Do not respect .gitignore files",
        long_help
    )]
    pub no_ignore_vcs: bool,

    /// Overrides --no-ignore-vcs
    #[arg(long, overrides_with = "no_ignore_vcs", hide = true, action = ArgAction::SetTrue)]
    ignore_vcs: (),

    /// Do not require a git repository to respect gitignores.
    /// By default, fd will only respect global gitignore rules, .gitignore rules,
    /// and local exclude rules if fd detects that you are searching inside a
    /// git repository. This flag allows you to relax this restriction such that
    /// fd will respect all git related ignore rules regardless of whether you're
    /// searching in a git repository or not.
    ///
    ///
    /// This flag can be disabled with --require-git.
    #[arg(
        long,
        overrides_with = "require_git",
        hide_short_help = true,
        // same description as ripgrep's flag: ripgrep/crates/core/app.rs
        long_help
    )]
    pub no_require_git: bool,

    /// Overrides --no-require-git
    #[arg(long, overrides_with = "no_require_git", hide = true, action = ArgAction::SetTrue)]
    require_git: (),

    /// Show search results from files and directories that would otherwise be
    /// ignored by '.gitignore', '.ignore', or '.fdignore' files in parent directories.
    /// The flag can be overridden with --ignore-parent.
    #[arg(
        long,
        hide_short_help = true,
        help = "Do not respect .(git|fd)ignore files in parent directories",
        long_help
    )]
    pub no_ignore_parent: bool,

    /// Overrides --no-ignore-parent
    #[arg(long, overrides_with = "no_ignore_parent", hide = true, action = ArgAction::SetTrue)]
    ignore_parent: (),

    /// Do not respect the global ignore file
    #[arg(long, hide = true)]
    pub no_global_ignore_file: bool,

    /// Perform an unrestricted search, including ignored and hidden files. This is
    /// an alias for '--no-ignore --hidden'.
    #[arg(long = "unrestricted", short = 'u', overrides_with_all(&["ignore", "no_hidden"]), action(ArgAction::Count), hide_short_help = true,
    help = "Unrestricted search, alias for '--no-ignore --hidden'",
        long_help,
        )]
    rg_alias_hidden_ignore: u8,

    /// Case-sensitive search (default: smart case)
    #[arg(
        long,
        short = 's',
        overrides_with("ignore_case"),
        long_help = "Perform a case-sensitive search. By default, fd uses case-insensitive \
                     searches, unless the pattern contains an uppercase character (smart \
                     case)."
    )]
    pub case_sensitive: bool,

    /// Perform a case-insensitive search. By default, fd uses case-insensitive
    /// searches, unless the pattern contains an uppercase character (smart
    /// case).
    #[arg(
        long,
        short = 'i',
        overrides_with("case_sensitive"),
        help = "Case-insensitive search (default: smart case)",
        long_help
    )]
    pub ignore_case: bool,

    /// Perform a glob-based search instead of a regular expression search.
    #[arg(
        long,
        short = 'g',
        conflicts_with("fixed_strings"),
        help = "Glob-based search (default: regular expression)",
        long_help
    )]
    pub glob: bool,

    /// Perform a regular-expression based search (default). This can be used to
    /// override --glob.
    #[arg(
        long,
        overrides_with("glob"),
        hide_short_help = true,
        help = "Regular-expression based search (default)",
        long_help
    )]
    pub regex: bool,

    /// Treat the pattern as a literal string instead of a regular expression. Note
    /// that this also performs substring comparison. If you want to match on an
    /// exact filename, consider using '--glob' or '--exact' instead.
    #[arg(
        long,
        short = 'F',
        alias = "literal",
        hide_short_help = true,
        help = "Treat pattern as literal string instead of regex",
        long_help
    )]
    pub fixed_strings: bool,

    /// Perform an exact match. This is equivalent to '--fixed-strings' but requires
    /// the pattern to match the entire filename (or path if '--full-path' is used),
    /// rather than a substring. Special regex characters in the pattern are treated
    /// as literal characters.
    #[arg(
        long,
        conflicts_with_all(["glob", "fixed_strings"]),
        hide_short_help = true,
        help = "Match the entire filename exactly (literal, non-substring)",
        long_help
    )]
    pub exact: bool,

    /// Add additional required search patterns, all of which must be matched. Multiple
    /// additional patterns can be specified. The patterns are regular
    /// expressions, unless '--glob' or '--fixed-strings' is used.
    #[arg(
        long = "and",
        value_name = "pattern",
        help = "Additional search patterns that need to be matched",
        long_help,
        hide_short_help = true,
        allow_hyphen_values = true
    )]
    pub exprs: Option<Vec<String>>,

    /// Shows the full path starting from the root as opposed to relative paths.
    /// The flag can be overridden with --relative-path.
    #[arg(
        long,
        short = 'a',
        help = "Show absolute instead of relative paths",
        long_help
    )]
    pub absolute_path: bool,

    /// Overrides --absolute-path
    #[arg(long, overrides_with = "absolute_path", hide = true, action = ArgAction::SetTrue)]
    relative_path: (),

    /// Use a detailed listing format like 'ls -l'. This is basically an alias
    /// for '--exec-batch ls -l' with some additional 'ls' options. This can be
    /// used to see more metadata, to show symlink targets and to achieve a
    /// deterministic sort order.
    #[arg(
        long,
        short = 'l',
        conflicts_with("absolute_path"),
        help = "Use a long listing format with file metadata",
        long_help
    )]
    pub list_details: bool,

    /// Follow symbolic links
    #[arg(
        long,
        short = 'L',
        alias = "dereference",
        long_help = "By default, fd does not descend into symlinked directories. Using this \
                     flag, symbolic links are also traversed. \
                     Flag can be overridden with --no-follow."
    )]
    pub follow: bool,

    /// Overrides --follow
    #[arg(long, overrides_with = "follow", hide = true, action = ArgAction::SetTrue)]
    no_follow: (),

    /// By default, the search pattern is only matched against the filename (or directory name). Using this flag, the pattern is matched against the full (absolute) path. Example:
    ///   fd --glob -p '**/.git/config'
    #[arg(
        long,
        short = 'p',
        help = "Search full abs. path (default: filename only)",
        long_help,
        verbatim_doc_comment
    )]
    pub full_path: bool,

    /// Separate search results by the null character (instead of newlines).
    /// Useful for piping results to 'xargs'.
    #[arg(
        long = "print0",
        short = '0',
        conflicts_with("list_details"),
        hide_short_help = true,
        help = "Separate search results by the null character",
        long_help
    )]
    pub null_separator: bool,

    /// Limit the directory traversal to a given depth. By default, there is no
    /// limit on the search depth.
    #[arg(
        long,
        short = 'd',
        value_name = "depth",
        alias("maxdepth"),
        help = "Set maximum search depth (default: none)",
        long_help
    )]
    max_depth: Option<usize>,

    /// Only show search results starting at the given depth.
    /// See also: '--max-depth' and '--exact-depth'
    #[arg(
        long,
        value_name = "depth",
        hide_short_help = true,
        alias("mindepth"),
        help = "Only show search results starting at the given depth.",
        long_help
    )]
    min_depth: Option<usize>,

    /// Only show search results at the exact given depth. This is an alias for
    /// '--min-depth <depth> --max-depth <depth>'.
    #[arg(long, value_name = "depth", hide_short_help = true, conflicts_with_all(&["max_depth", "min_depth"]),
    help = "Only show search results at the exact given depth",
        long_help,
        )]
    exact_depth: Option<usize>,

    /// Exclude files/directories that match the given glob pattern. This
    /// overrides any other ignore logic. Multiple exclude patterns can be
    /// specified.
    ///
    /// Examples:
    /// {n}  --exclude '*.pyc'
    /// {n}  --exclude node_modules
    #[arg(
        long,
        short = 'E',
        value_name = "glob",
        help = "Exclude entries that match the given glob pattern",
        long_help
    )]
    pub exclude: Vec<String>,

    /// Do not traverse into directories that match the search criteria. If
    /// you want to exclude specific directories, use the '--exclude=…' option.
    #[arg(long, hide_short_help = true, conflicts_with_all(&["size", "exact_depth"]),
        long_help,
        )]
    pub prune: bool,

    /// Filter the search by type:
    /// {n}  'f' or 'file':         regular files
    /// {n}  'd' or 'dir' or 'directory':    directories
    /// {n}  'l' or 'symlink':      symbolic links
    /// {n}  's' or 'socket':       socket
    /// {n}  'p' or 'pipe':         named pipe (FIFO)
    /// {n}  'b' or 'block-device': block device
    /// {n}  'c' or 'char-device':  character device
    /// {n}{n}  'x' or 'executable':   executables
    /// {n}  'e' or 'empty':        empty files or directories
    ///
    /// This option can be specified more than once to include multiple file types.
    /// Searching for '--type file --type symlink' will show both regular files as
    /// well as symlinks. Note that the 'executable' and 'empty' filters work differently:
    /// '--type executable' implies '--type file' by default. And '--type empty' searches
