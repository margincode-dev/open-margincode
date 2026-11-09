- Exec.augment_args (src/cli.rs:883-952)
```rust
    fn augment_args(cmd: Command) -> Command {
        cmd.arg(Arg::new("exec")
            .action(ArgAction::Append)
            .long("exec")
            .short('x')
            .num_args(1..)
                .allow_hyphen_values(true)
                .value_terminator(";")
                .value_name("cmd")
                .conflicts_with("list_details")
                .help("Execute a command for each search result")
                .long_help(
                    "Execute a command for each search result in parallel (use --threads=1 for sequential command execution). \
                     There is no guarantee of the order commands are executed in, and the order should not be depended upon. \
                     All positional arguments following --exec are considered to be arguments to the command - not to fd. \
                     It is therefore recommended to place the '-x'/'--exec' option last. \
                     Use '\\;' to terminate the command template if you need to continue passing fd arguments afterwards.\n\
                     The following placeholders are substituted before the command is executed:\n  \
                       '{}':   path (of the current search result)\n  \
                       '{/}':  basename\n  \
                       '{//}': parent directory\n  \
                       '{.}':  path without file extension\n  \
                       '{/.}': basename without file extension\n  \
                       '{{':   literal '{' (for escaping)\n  \
                       '}}':   literal '}' (for escaping)\n\n\
                     If no placeholder is present, an implicit \"{}\" at the end is assumed.\n\n\
                     Examples:\n\n  \
                       - find all *.zip files and unzip them:\n\n      \
                           fd -e zip -x unzip\n\n  \
                       - find *.h and *.cpp files and run \"clang-format -i ..\" for each of them:\n\n      \
                           fd -e h -e cpp -x clang-format -i\n\n  \
                       - search within `src/` and echo each match (place `-x` last):\n\n      \
                           fd . src -x echo\n\n  \
                       - Convert all *.jpg files to *.png files:\n\n      \
                           fd -e jpg -x convert {} {.}.png\
                    ",
                ),
        )
        .arg(
            Arg::new("exec_batch")
                .action(ArgAction::Append)
                .long("exec-batch")
                .short('X')
                .num_args(1..)
                .allow_hyphen_values(true)
                .value_terminator(";")
                .value_name("cmd")
                .conflicts_with_all(["exec", "list_details"])
                .help("Execute a command with all search results at once")
                .long_help(
                    "Execute the given command once, with all search results as arguments.\n\
                     The order of the arguments is non-deterministic, and should not be relied upon.\n\
                     One of the following placeholders is substituted before the command is executed:\n  \
                       '{}':   path (of all search results)\n  \
                       '{/}':  basename\n  \
                       '{//}': parent directory\n  \
                       '{.}':  path without file extension\n  \
                       '{/.}': basename without file extension\n  \
                       '{{':   literal '{' (for escaping)\n  \
                       '}}':   literal '}' (for escaping)\n\n\
                     If no placeholder is present, an implicit \"{}\" at the end is assumed.\n\n\
                     Examples:\n\n  \
                       - Find all test_*.py files and open them in your favorite editor:\n\n      \
                           fd -g 'test_*.py' -X vim\n\n  \
                       - Find all *.rs files and count the lines with \"wc -l ...\":\n\n      \
                           fd -e rs -X wc -l\
                     "
                ),
        )
    }
```
- Opts (src/cli.rs:32-693)
```rust
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
    /// for empty files and directories, unless either '--type file' or '--type directory'
    /// is specified in addition.
    ///
    /// Examples:
    /// {n}  - Only search for files:
    /// {n}      fd --type file …
    /// {n}      fd -tf …
    /// {n}  - Find both files and symlinks
    /// {n}      fd --type file --type symlink …
    /// {n}      fd -tf -tl …
    /// {n}  - Find executable files:
    /// {n}      fd --type executable
    /// {n}      fd -tx
    /// {n}  - Find empty files:
    /// {n}      fd --type empty --type file
    /// {n}      fd -te -tf
    /// {n}  - Find empty directories:
    /// {n}      fd --type empty --type directory
    /// {n}      fd -te -td
    #[arg(
        long = "type",
        short = 't',
        value_name = "filetype",
        hide_possible_values = true,
        value_enum,
        help = "Filter by type: file (f), directory (d/dir), symlink (l), \
                executable (x), empty (e), socket (s), pipe (p), \
                char-device (c), block-device (b)",
        long_help
    )]
    pub filetype: Option<Vec<FileType>>,

    /// Filter results by extension. By default, this matches all entry types
    /// (including directories whose names end with the extension). Use `--type`
    /// to restrict the search to specific types. Multiple extensions can be specified.
    ///
    /// If you want to search for files without extension,
    /// you can use the regex '^[^.]+$' as a normal search pattern.
    #[arg(
        long = "extension",
        short = 'e',
        value_name = "ext",
        help = "Filter by extension",
        long_help
    )]
    pub extensions: Option<Vec<String>>,

    /// Limit results based on the size of files using the format <+-><NUM><UNIT>.
    ///    '+': file size must be greater than or equal to this
    ///    '-': file size must be less than or equal to this
    ///
    /// If neither '+' nor '-' is specified, file size must be exactly equal to this.
    ///    'NUM':  The numeric size (e.g. 500)
    ///    'UNIT': The units for NUM. They are not case-sensitive.
    /// Allowed unit values:
    ///     'b':  bytes
    ///     'k':  kilobytes (base ten, 10^3 = 1000 bytes)
    ///     'm':  megabytes
    ///     'g':  gigabytes
    ///     't':  terabytes
    ///     'ki': kibibytes (base two, 2^10 = 1024 bytes)
    ///     'mi': mebibytes
    ///     'gi': gibibytes
    ///     'ti': tebibytes
    #[arg(long, short = 'S', value_parser = SizeFilter::from_string, allow_hyphen_values = true, verbatim_doc_comment, value_name = "size",
        help = "Limit results based on the size of files",
        long_help,
        verbatim_doc_comment,
        )]
    pub size: Vec<SizeFilter>,

    /// Filter results based on the file modification time. Files with modification times
    /// greater than the argument are returned. The argument can be provided
    /// as a specific point in time (YYYY-MM-DD HH:MM:SS or @timestamp) or as a duration (10h, 1d, 35min).
    /// If the time is not specified, it defaults to 00:00:00.
    /// '--change-newer-than', '--newer', or '--changed-after' can be used as aliases.
    ///
    /// Examples:
    /// {n}    --changed-within 2weeks
    /// {n}    --change-newer-than '2018-10-27 10:00:00'
    /// {n}    --newer 2018-10-27
    /// {n}    --changed-after 1day
    #[arg(
        long,
        alias("change-newer-than"),
        alias("newer"),
        alias("changed-after"),
        value_name = "date|dur",
        help = "Filter by file modification time (newer than)",
        long_help
    )]
    pub changed_within: Option<String>,

    /// Filter results based on the file modification time. Files with modification times
    /// less than the argument are returned. The argument can be provided
    /// as a specific point in time (YYYY-MM-DD HH:MM:SS or @timestamp) or as a duration (10h, 1d, 35min).
    /// '--change-older-than' or '--older' can be used as aliases.
    ///
    /// Examples:
    /// {n}    --changed-before '2018-10-27 10:00:00'
    /// {n}    --change-older-than 2weeks
    /// {n}    --older 2018-10-27
    #[arg(
        long,
        alias("change-older-than"),
        alias("older"),
        value_name = "date|dur",
        help = "Filter by file modification time (older than)",
        long_help
    )]
    pub changed_before: Option<String>,

    /// Filter files by their user and/or group.
    /// Format: [(user|uid)][:(group|gid)]. Either side is optional.
    /// Precede either side with a '!' to exclude files instead.
    ///
    /// Examples:
    /// {n}    --owner john
    /// {n}    --owner :students
    /// {n}    --owner '!john:students'
    #[cfg(unix)]
    #[arg(long, short = 'o', value_parser = OwnerFilter::from_string, value_name = "user:group",
        help = "Filter by owning user and/or group",
        long_help,
        )]
    pub owner: Option<OwnerFilter>,

    /// Instead of printing the file normally, print the format string with the following placeholders replaced:
    ///   '{}': path (of the current search result)
    ///   '{/}': basename
    ///   '{//}': parent directory
    ///   '{.}': path without file extension
    ///   '{/.}': basename without file extension
    #[arg(
        long,
        value_name = "fmt",
        help = "Print results according to template",
        conflicts_with = "list_details"
    )]
    pub format: Option<String>,

    #[command(flatten)]
    pub exec: Exec,

    /// Maximum number of arguments to pass to the command given with -X.
    /// If the number of results is greater than the given size,
    /// the command given with -X is run again with remaining arguments.
    /// A batch size of zero means there is no limit (default), but note
    /// that batching might still happen due to OS restrictions on the
    /// maximum length of command lines.
    #[arg(
        long,
        value_name = "size",
        hide_short_help = true,
        requires("exec_batch"),
        value_parser = value_parser!(usize),
        default_value_t,
        help = "Max number of arguments to run as a batch size with -X",
        long_help,
    )]
    pub batch_size: usize,

    /// Add a custom ignore-file in '.gitignore' format. These files have a low precedence.
    #[arg(
        long,
        value_name = "path",
        hide_short_help = true,
        help = "Add a custom ignore-file in '.gitignore' format",
        long_help
    )]
    pub ignore_file: Vec<PathBuf>,

    /// Declare when to use color for the pattern match output
    #[arg(
        long,
        short = 'c',
        value_enum,
        default_value_t = ColorWhen::Auto,
        value_name = "when",
        help = "When to use colors",
        long_help,
    )]
    pub color: ColorWhen,

    /// Add a terminal hyperlink to a file:// url for each path in the output.
    ///
    /// Auto mode  is used if no argument is given to this option.
    ///
    /// This doesn't do anything for --exec and --exec-batch.
    #[arg(
        long,
        alias = "hyper",
        value_name = "when",
        require_equals = true,
        value_enum,
        default_value_t = HyperlinkWhen::Never,
        default_missing_value = "auto",
        num_args = 0..=1,
        help = "Add hyperlinks to output paths"
    )]
    pub hyperlink: HyperlinkWhen,

    /// Ignore directories containing the named entry.
    #[arg(long, value_name = "name")]
    pub ignore_contain: Vec<String>,

    /// Set number of threads to use for searching & executing (default: number
    /// of available CPU cores)
    #[arg(long, short = 'j', value_name = "num", hide_short_help = true, value_parser = str::parse::<NonZeroUsize>)]
    pub threads: Option<NonZeroUsize>,

    /// Milliseconds to buffer before streaming search results to console
    ///
    /// Amount of time in milliseconds to buffer, before streaming the search
    /// results to the console.
    #[arg(long, hide = true, value_parser = parse_millis)]
    pub max_buffer_time: Option<Duration>,

    ///Limit the number of search results to 'count' and quit immediately.
    #[arg(
        long,
        value_name = "count",
        hide_short_help = true,
        overrides_with("max_one_result"),
        help = "Limit the number of search results",
        long_help
    )]
    max_results: Option<usize>,

    /// Limit the search to a single result and quit immediately.
    /// This is an alias for '--max-results=1'.
    #[arg(
        short = '1',
        hide_short_help = true,
        overrides_with("max_results"),
        help = "Limit search to a single result",
        long_help
    )]
    max_one_result: bool,

    /// When the flag is present, the program does not print anything and will
    /// return with an exit code of 0 if there is at least one match. Otherwise, the
    /// exit code will be 1.
    /// '--has-results' can be used as an alias.
    #[arg(
        long,
        short = 'q',
        alias = "has-results",
        hide_short_help = true,
        conflicts_with("max_results"),
        help = "Print nothing, exit code 0 if match found, 1 otherwise",
        long_help
    )]
    pub quiet: bool,

    /// Enable the display of filesystem errors for situations such as
    /// insufficient permissions or dead symlinks.
    #[arg(
        long,
        hide_short_help = true,
        help = "Show filesystem errors",
        long_help
    )]
    pub show_errors: bool,

    /// Change the current working directory of fd to the provided path. This
    /// means that search results will be shown with respect to the given base
    /// path. Note that relative paths which are passed to fd via the positional
    /// <path> argument or the '--search-path' option will also be resolved
    /// relative to this directory.
    #[arg(
        long,
        short = 'C',
        value_name = "path",
        hide_short_help = true,
        help = "Change current working directory",
        long_help
    )]
    pub base_directory: Option<PathBuf>,

    /// the search pattern which is either a regular expression (default) or a glob
    /// pattern (if --glob is used). If no pattern has been specified, every entry
    /// is considered a match. If your pattern starts with a dash (-), make sure to
    /// pass '--' first, or it will be considered as a flag (fd -- '-foo').
    #[arg(
        default_value = "",
        hide_default_value = true,
        value_name = "pattern",
        help = "the search pattern (a regular expression, unless '--glob' is used; optional)",
        long_help
    )]
    pub pattern: String,

    /// Set the path separator to use when printing file paths. The default is
    /// the OS-specific separator ('/' on Unix, '\' on Windows).
    #[arg(
        long,
        value_name = "separator",
        hide_short_help = true,
        help = "Set path separator when printing file paths",
        long_help
    )]
    pub path_separator: Option<String>,

    /// The directory where the filesystem search is rooted (optional). If
    /// omitted, search the current working directory.
    #[arg(action = ArgAction::Append,
        value_name = "path",
        help = "the root directories for the filesystem search (optional)",
        long_help,
        )]
    path: Vec<PathBuf>,

    /// Provide paths to search as an alternative to the positional <path>
    /// argument. Changes the usage to `fd [OPTIONS] --search-path <path>
    /// --search-path <path2> [<pattern>]`
    #[arg(
        long,
        conflicts_with("path"),
        value_name = "search-path",
        hide_short_help = true,
        help = "Provides paths to search as an alternative to the positional <path> argument",
        long_help
    )]
    search_path: Vec<PathBuf>,

    /// By default, relative paths are prefixed with './' when -x/--exec,
    /// -X/--exec-batch, or -0/--print0 are given, to reduce the risk of a
    /// path starting with '-' being treated as a command line option. Use
    /// this flag to change this behavior. If this flag is used without a value,
    /// it is equivalent to passing "always".
    #[arg(long, conflicts_with_all(&["path", "search_path"]), value_name = "when", hide_short_help = true, require_equals = true, long_help)]
    strip_cwd_prefix: Option<Option<StripCwdWhen>>,

    /// By default, fd will traverse the file system tree as far as other options
    /// dictate. With this flag, fd ensures that it does not descend into a
    /// different file system than the one it started in. Comparable to the -mount
    /// or -xdev filters of find(1).
    #[cfg(any(unix, windows))]
    #[arg(long, aliases(&["mount", "xdev"]), hide_short_help = true, long_help)]
    pub one_file_system: bool,

    #[cfg(feature = "completions")]
    #[arg(long, hide = true, exclusive = true)]
    gen_completions: Option<Option<Shell>>,
}
```
- Exec.from_arg_matches (src/cli.rs:862-874)
```rust
    fn from_arg_matches(matches: &ArgMatches) -> clap::error::Result<Self> {
        let command = matches
            .get_occurrences::<String>("exec")
            .map(CommandSet::new)
            .or_else(|| {
                matches
                    .get_occurrences::<String>("exec_batch")
                    .map(CommandSet::new_batch)
            })
            .transpose()
            .map_err(|e| clap::Error::raw(ErrorKind::InvalidValue, e))?;
        Ok(Exec { command })
    }
```
- needs_escape (src/sanitize.rs:9-24)
```rust
fn needs_escape(c: char) -> bool {
    if c == '\t' {
        return false;
    }
    c.is_control()
        || matches!(c,
            '\u{00AD}'                  // soft hyphen (invisible)
            | '\u{180E}'                // Mongolian vowel separator
            | '\u{200B}'..='\u{200F}'   // zero-width + LRM/RLM
            | '\u{202A}'..='\u{202E}'   // bidi embedding/override
            | '\u{2060}'..='\u{206F}'   // word joiner, invisibles, deprecated formats
            | '\u{FEFF}'                // BOM / zero-width no-break space
            | '\u{FFF9}'..='\u{FFFB}'   // interlinear annotation
            | '\u{E0000}'..='\u{E007F}' // language tags
        )
}
```
- strips_bidi_overrides_and_zero_width (src/sanitize.rs:131-140)
```rust
    fn strips_bidi_overrides_and_zero_width() {
        // Trojan-Source style RLO/LRO that flip rendered order of filename text.
        assert_eq!(
            sanitize_for_terminal("safe\u{202E}fil\u{202D}gnp.exe"),
            "safe\\u{202E}fil\\u{202D}gnp.exe"
        );
        // Zero-width space and BOM are also format chars used to disguise filenames.
        assert_eq!(sanitize_for_terminal("a\u{200B}b"), "a\\u{200B}b");
        assert_eq!(sanitize_for_terminal("\u{FEFF}name"), "\\u{FEFF}name");
    }
```
- CommandTemplate.new (src/exec/mod.rs:220-256)
```rust
    fn new<I, S>(input: I, mode: ExecutionMode) -> Result<CommandTemplate>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut args = Vec::new();
        let mut has_placeholder = false;

        for arg in input {
            let arg = arg.as_ref();

            let tmpl = FormatTemplate::parse(arg);
            has_placeholder |= tmpl.has_tokens();
            args.push(tmpl);
        }

        // We need to check that we have at least one argument, because if not
        // it will try to execute each file and directory it finds.
        //
        // Sadly, clap can't currently handle this for us, see
        // https://github.com/clap-rs/clap/issues/3542
        if args.is_empty() {
            bail!("No executable provided for --exec or --exec-batch");
        }

        // A placeholder as the executable is meaningful for `--exec` but never for `--exec-batch`.
        if mode == ExecutionMode::Batch && args[0].has_tokens() {
            bail!("First argument of --exec-batch must be a fixed executable, not a placeholder");
        }

        // If a placeholder token was not supplied, append one at the end of the command.
        if !has_placeholder {
            args.push(FormatTemplate::Tokens(vec![Token::Placeholder]));
        }

        Ok(CommandTemplate { args })
    }
```
- hir_matches_strings_with_leading_dot (src/regex_helper.rs:50-79)
```rust
fn hir_matches_strings_with_leading_dot(hir: &Hir) -> bool {
    use regex_syntax::hir::*;

    // Note: this only really detects the simplest case where a regex starts with
    // "^\\.", i.e. a start text anchor and a literal dot character. There are a lot
    // of other patterns that ONLY match hidden files, e.g. ^(\\.foo|\\.bar) which are
    // not (yet) detected by this algorithm.
    match hir.kind() {
        HirKind::Concat(hirs) => {
            let mut hirs = hirs.iter();
            if let Some(hir) = hirs.next() {
                if hir.kind() != &HirKind::Look(Look::Start) {
                    return false;
                }
            } else {
                return false;
            }

            if let Some(hir) = hirs.next() {
                match hir.kind() {
                    HirKind::Literal(Literal(bytes)) => bytes.starts_with(b"."),
                    _ => false,
                }
            } else {
                false
            }
        }
        _ => false,
    }
}
```
- Config (src/config.rs:14-136)
```rust
pub struct Config {
    /// Whether the search is case-sensitive or case-insensitive.
    pub case_sensitive: bool,

    /// Cached current working directory for absolute path construction.
    /// Populated when `--full-path` is set; `None` means search by filename only.
    pub full_path_base: Option<PathBuf>,

    /// Whether to ignore hidden files and directories (or not).
    pub ignore_hidden: bool,

    /// Whether to respect `.fdignore` files or not.
    pub read_fdignore: bool,

    /// Whether to respect ignore files in parent directories or not.
    pub read_parent_ignore: bool,

    /// Whether to respect VCS ignore files (`.gitignore`, ..) or not.
