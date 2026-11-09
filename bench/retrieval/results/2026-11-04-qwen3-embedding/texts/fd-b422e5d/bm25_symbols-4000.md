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
