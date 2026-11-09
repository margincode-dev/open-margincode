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
