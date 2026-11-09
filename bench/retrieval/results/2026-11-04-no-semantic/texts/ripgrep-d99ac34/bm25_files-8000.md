### crates/core/flags/doc/man.rs
```rust
/*!
Provides routines for generating ripgrep's man page in `roff` format.
*/

use std::{collections::BTreeMap, fmt::Write};

use crate::flags::{Flag, defs::FLAGS, doc::version};

const TEMPLATE: &'static str = include_str!("template.rg.1");

/// Wraps `std::write!` and asserts there is no failure.
///
/// We only write to `String` in this module.
macro_rules! write {
    ($($tt:tt)*) => { std::write!($($tt)*).unwrap(); }
}

/// Wraps `std::writeln!` and asserts there is no failure.
///
/// We only write to `String` in this module.
macro_rules! writeln {
    ($($tt:tt)*) => { std::writeln!($($tt)*).unwrap(); }
}

/// Returns a `roff` formatted string corresponding to ripgrep's entire man
/// page.
pub(crate) fn generate() -> String {
    let mut cats = BTreeMap::new();
    for flag in FLAGS.iter().copied() {
        let mut cat = cats.entry(flag.doc_category()).or_insert(String::new());
        if !cat.is_empty() {
            writeln!(cat, ".sp");
        }
        generate_flag(flag, &mut cat);
    }

    let mut out = TEMPLATE.replace("!!VERSION!!", &version::generate_digits());
    for (cat, value) in cats.iter() {
        let var = format!("!!{name}!!", name = cat.as_str());
        if !cfg!(feature = "unstable-index")
            && matches!(cat, crate::flags::Category::Indexing)
        {
            out = out.replace(&var, crate::flags::INDEXING_NOT_SUPPORTED);
        } else {
            out = out.replace(&var, value);
        }
    }
    out
}

/// Writes `roff` formatted documentation for `flag` to `out`.
fn generate_flag(flag: &'static dyn Flag, out: &mut String) {
    if let Some(byte) = flag.name_short() {
        let name = char::from(byte);
        write!(out, r"\fB\-{name}\fP");
        if let Some(var) = flag.doc_variable() {
            write!(out, r" \fI{var}\fP");
        }
        write!(out, r", ");
    }

    let name = flag.name_long().replace("-", r"\-");
    write!(out, r"\fB\-\-{name}\fP");
    if let Some(var) = flag.doc_variable() {
        write!(out, r"=\fI{var}\fP");
    }
    write!(out, "\n");

    writeln!(out, ".RS 4");
    let doc = flag.doc_long().trim();
    // Convert \flag{foo} into something nicer.
    let doc = super::render_custom_markup(doc, "flag", |name, out| {
        let Some(flag) = crate::flags::parse::lookup(name) else {
            unreachable!(r"found unrecognized \flag{{{name}}} in roff docs")
        };
        out.push_str(r"\fB");
        if let Some(name) = flag.name_short() {
            write!(out, r"\-{}/", char::from(name));
        }
        write!(out, r"\-\-{}", flag.name_long().replace("-", r"\-"));
        out.push_str(r"\fP");
    });
    // Convert \flag-negate{foo} into something nicer.
    let doc = super::render_custom_markup(&doc, "flag-negate", |name, out| {
        let Some(flag) = crate::flags::parse::lookup(name) else {
            unreachable!(
                r"found unrecognized \flag-negate{{{name}}} in roff docs"
            )
        };
        let Some(name) = flag.name_negated() else {
            let long = flag.name_long();
            unreachable!(
                "found \\flag-negate{{{long}}} in roff docs but \
                 {long} does not have a negation"
            );
        };
        out.push_str(r"\fB");
        write!(out, r"\-\-{name}");
        out.push_str(r"\fP");
    });
    writeln!(out, "{doc}");
    if let Some(negated) = flag.name_negated() {
        // Flags that can be negated that aren't switches, like
        // --context-separator, are somewhat weird. Because of that, the docs
        // for those flags should discuss the semantics of negation explicitly.
        // But for switches, the behavior is always the same.
        if flag.is_switch() {
            writeln!(out, ".sp");
            writeln!(
                out,
                r"This flag can be disabled with \fB\-\-{negated}\fP."
            );
        }
    }
    writeln!(out, ".RE");
}
```
### crates/core/flags/doc/help.rs
```rust
/*!
Provides routines for generating ripgrep's "short" and "long" help
documentation.

The short version is used when the `-h` flag is given, while the long version
is used when the `--help` flag is given.
*/

use std::{collections::BTreeMap, fmt::Write};

use crate::flags::{Category, Flag, defs::FLAGS, doc::version};

const TEMPLATE_SHORT: &'static str = include_str!("template.short.help");
const TEMPLATE_LONG: &'static str = include_str!("template.long.help");

/// Wraps `std::write!` and asserts there is no failure.
///
/// We only write to `String` in this module.
macro_rules! write {
    ($($tt:tt)*) => { std::write!($($tt)*).unwrap(); }
}

/// Generate short documentation, i.e., for `-h`.
pub(crate) fn generate_short() -> String {
    let mut cats: BTreeMap<Category, (Vec<String>, Vec<String>)> =
        BTreeMap::new();
    let (mut maxcol1, mut maxcol2) = (0, 0);
    for flag in FLAGS.iter().copied() {
        let columns =
            cats.entry(flag.doc_category()).or_insert((vec![], vec![]));
        let (col1, col2) = generate_short_flag(flag);
        maxcol1 = maxcol1.max(col1.len());
        maxcol2 = maxcol2.max(col2.len());
        columns.0.push(col1);
        columns.1.push(col2);
    }
    let mut out =
        TEMPLATE_SHORT.replace("!!VERSION!!", &version::generate_digits());
    for (cat, (col1, col2)) in cats.iter() {
        let var = format!("!!{name}!!", name = cat.as_str());
        if !cfg!(feature = "unstable-index")
            && matches!(cat, crate::flags::Category::Indexing)
        {
            out = out.replace(
                &var,
                &format!("  {}", crate::flags::INDEXING_NOT_SUPPORTED),
            );
        } else {
            let val = format_short_columns(col1, col2, maxcol1, maxcol2);
            out = out.replace(&var, &val);
        }
    }
    out
}

/// Generate short for a single flag.
///
/// The first element corresponds to the flag name while the second element
/// corresponds to the documentation string.
fn generate_short_flag(flag: &dyn Flag) -> (String, String) {
    let (mut col1, mut col2) = (String::new(), String::new());

    // Some of the variable names are fine for longer form
    // docs, but they make the succinct short help very noisy.
    // So just shorten some of them.
    let var = flag.doc_variable().map(|s| {
        let mut s = s.to_string();
        s = s.replace("SEPARATOR", "SEP");
        s = s.replace("REPLACEMENT", "TEXT");
        s = s.replace("NUM+SUFFIX?", "NUM");
        s
    });

    // Generate the first column, the flag name.
    if let Some(byte) = flag.name_short() {
        let name = char::from(byte);
        write!(col1, r"-{name}");
        write!(col1, r", ");
    }
    write!(col1, r"--{name}", name = flag.name_long());
    if let Some(var) = var.as_ref() {
        write!(col1, r"={var}");
    }

    // And now the second column, with the description.
    write!(col2, "{}", flag.doc_short());

    (col1, col2)
}

/// Write two columns of documentation.
///
/// `maxcol1` should be the maximum length (in bytes) of the first column,
/// while `maxcol2` should be the maximum length (in bytes) of the second
/// column.
fn format_short_columns(
    col1: &[String],
    col2: &[String],
    maxcol1: usize,
    _maxcol2: usize,
) -> String {
    assert_eq!(col1.len(), col2.len(), "columns must have equal length");
    const PAD: usize = 2;
    let mut out = String::new();
    for (i, (c1, c2)) in col1.iter().zip(col2.iter()).enumerate() {
        if i > 0 {
            write!(out, "\n");
        }

        let pad = maxcol1 - c1.len() + PAD;
        write!(out, "  ");
        write!(out, "{c1}");
        write!(out, "{}", " ".repeat(pad));
        write!(out, "{c2}");
    }
    out
}

/// Generate long documentation, i.e., for `--help`.
pub(crate) fn generate_long() -> String {
    let mut cats = BTreeMap::new();
    for flag in FLAGS.iter().copied() {
        let mut cat = cats.entry(flag.doc_category()).or_insert(String::new());
        if !cat.is_empty() {
            write!(cat, "\n\n");
        }
        generate_long_flag(flag, &mut cat);
    }

    let mut out =
        TEMPLATE_LONG.replace("!!VERSION!!", &version::generate_digits());
    for (cat, value) in cats.iter() {
        let var = format!("!!{name}!!", name = cat.as_str());
        if !cfg!(feature = "unstable-index")
            && matches!(cat, crate::flags::Category::Indexing)
        {
            out = out.replace(
                &var,
                &format!("    {}", crate::flags::INDEXING_NOT_SUPPORTED),
            );
        } else {
            out = out.replace(&var, value);
        }
    }
    out
}

/// Write generated documentation for `flag` to `out`.
fn generate_long_flag(flag: &dyn Flag, out: &mut String) {
    if let Some(byte) = flag.name_short() {
        let name = char::from(byte);
        write!(out, r"    -{name}");
        if let Some(var) = flag.doc_variable() {
            write!(out, r" {var}");
        }
        write!(out, r", ");
    } else {
        write!(out, r"    ");
    }

    let name = flag.name_long();
    write!(out, r"--{name}");
    if let Some(var) = flag.doc_variable() {
        write!(out, r"={var}");
    }
    write!(out, "\n");

    let doc = flag.doc_long().trim();
    let doc = super::render_custom_markup(doc, "flag", |name, out| {
        let Some(flag) = crate::flags::parse::lookup(name) else {
            unreachable!(r"found unrecognized \flag{{{name}}} in --help docs")
        };
        if let Some(name) = flag.name_short() {
            write!(out, r"-{}/", char::from(name));
        }
        write!(out, r"--{}", flag.name_long());
    });
    let doc = super::render_custom_markup(&doc, "flag-negate", |name, out| {
        let Some(flag) = crate::flags::parse::lookup(name) else {
            unreachable!(
                r"found unrecognized \flag-negate{{{name}}} in --help docs"
            )
        };
        let Some(name) = flag.name_negated() else {
            let long = flag.name_long();
            unreachable!(
                "found \\flag-negate{{{long}}} in --help docs but \
                 {long} does not have a negation"
            );
        };
        write!(out, r"--{name}");
    });

    let mut cleaned = remove_roff(&doc);
    if let Some(negated) = flag.name_negated() {
        // Flags that can be negated that aren't switches, like
        // --context-separator, are somewhat weird. Because of that, the docs
        // for those flags should discuss the semantics of negation explicitly.
        // But for switches, the behavior is always the same.
        if flag.is_switch() {
            write!(cleaned, "\n\nThis flag can be disabled with --{negated}.");
        }
    }
    let indent = " ".repeat(8);
    let wrapopts = textwrap::Options::new(71)
        // Normally I'd be fine with breaking at hyphens, but ripgrep's docs
        // includes a lot of flag names, and they in turn contain hyphens.
        // Breaking flag names across lines is not great.
        .word_splitter(textwrap::WordSplitter::NoHyphenation);
    for (i, paragraph) in cleaned.split("\n\n").enumerate() {
        if i > 0 {
            write!(out, "\n\n");
        }
        let mut new = paragraph.to_string();
        if paragraph.lines().all(|line| line.starts_with("    ")) {
            // Re-indent but don't refill so as to preserve line breaks
            // in code/shell example snippets.
            new = textwrap::indent(&new, &indent);
        } else {
            new = new.replace("\n", " ");
            new = textwrap::refill(&new, &wrapopts);
            new = textwrap::indent(&new, &indent);
        }
        write!(out, "{}", new.trim_end());
    }
}

/// Removes roff syntax from `v` such that the result is approximately plain
/// text readable.
///
/// This is basically a mish mash of heuristics based on the specific roff used
/// in the docs for the flags in this tool. If new kinds of roff are used in
/// the docs, then this may need to be updated to handle them.
fn remove_roff(v: &str) -> String {
    let mut lines = vec![];
    for line in v.trim().lines() {
        assert!(!line.is_empty(), "roff should have no empty lines");
        if line.starts_with(".") {
            if line.starts_with(".IP ") {
                let item_label = line
                    .split(" ")
                    .nth(1)
                    .expect("first argument to .IP")
                    .replace(r"\(bu", r"•")
                    .replace(r"\fB", "")
                    .replace(r"\fP", ":");
                lines.push(format!("{item_label}"));
            } else if line.starts_with(".IB ") || line.starts_with(".BI ") {
                let pieces = line
                    .split_whitespace()
                    .skip(1)
                    .collect::<Vec<_>>()
                    .concat();
                lines.push(format!("{pieces}"));
            } else if line.starts_with(".sp")
                || line.starts_with(".PP")
                || line.starts_with(".TP")
            {
                lines.push("".to_string());
            }
        } else if line.starts_with(r"\fB") && line.ends_with(r"\fP") {
            let line = line.replace(r"\fB", "").replace(r"\fP", "");
            lines.push(format!("{line}:"));
        } else {
            lines.push(line.to_string());
        }
    }
    // Squash multiple adjacent paragraph breaks into one.
    lines.dedup_by(|l1, l2| l1.is_empty() && l2.is_empty());
    lines
        .join("\n")
        .replace(r"\fB", "")
        .replace(r"\fI", "")
        .replace(r"\fP", "")
        .replace(r"\-", "-")
        .replace(r"\\", r"\")
}
```
### crates/core/flags/mod.rs
```rust
/*!
Defines ripgrep's command line interface.

This modules deals with everything involving ripgrep's flags and positional
arguments. This includes generating shell completions, `--help` output and even
ripgrep's man page. It's also responsible for parsing and validating every
flag (including reading ripgrep's config file), and manages the contact points
between these flags and ripgrep's cast of supporting libraries. For example,
once [`HiArgs`] has been created, it knows how to create a multi threaded
recursive directory traverser.
*/
use std::{
    ffi::OsString,
    fmt::Debug,
    panic::{RefUnwindSafe, UnwindSafe},
};

pub(crate) use crate::flags::{
    complete::{
        bash::generate as generate_complete_bash,
        fish::generate as generate_complete_fish,
        powershell::generate as generate_complete_powershell,
        zsh::generate as generate_complete_zsh,
    },
    doc::{
        help::{
            generate_long as generate_help_long,
            generate_short as generate_help_short,
        },
        man::generate as generate_man_page,
        version::{
            generate_long as generate_version_long,
            generate_pcre2 as generate_version_pcre2,
            generate_short as generate_version_short,
        },
    },
    hiargs::HiArgs,
    lowargs::{GenerateMode, Mode, SearchMode, SpecialMode},
    parse::{ParseResult, parse},
};

mod complete;
mod config;
mod defs;
mod doc;
mod hiargs;
mod lowargs;
mod parse;

const INDEXING_NOT_SUPPORTED: &'static str =
    "Indexing is not supported in this build of ripgrep.";

/// A trait that encapsulates the definition of an optional flag for ripgrep.
///
/// This trait is meant to be used via dynamic dispatch. Namely, the `defs`
/// module provides a single global slice of `&dyn Flag` values correspondings
/// to all of the flags in ripgrep.
///
/// ripgrep's required positional arguments are handled by the parser and by
/// the conversion from low-level arguments to high level arguments. Namely,
/// all of ripgrep's positional arguments are treated as file paths, except
/// in certain circumstances where the first argument is treated as a regex
/// pattern.
///
/// Note that each implementation of this trait requires a long flag name,
/// but can also optionally have a short version and even a negation flag.
/// For example, the `-E/--encoding` flag accepts a value, but it also has a
/// `--no-encoding` negation flag for reverting back to "automatic" encoding
/// detection. All three of `-E`, `--encoding` and `--no-encoding` are provided
/// by a single implementation of this trait.
///
/// ripgrep only supports flags that are switches or flags that accept a single
/// value. Flags that accept multiple values are an unsupported aberration.
trait Flag: Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static {
    /// Returns true if this flag is a switch. When a flag is a switch, the
    /// CLI parser will not look for a value after the flag is seen.
    fn is_switch(&self) -> bool;

    /// A short single byte name for this flag. This returns `None` by default,
    /// which signifies that the flag has no short name.
    ///
    /// The byte returned must be an ASCII codepoint that is a `.` or is
    /// alpha-numeric.
    fn name_short(&self) -> Option<u8> {
        None
    }

    /// Returns the long name of this flag. All flags must have a "long" name.
    ///
    /// The long name must be at least 2 bytes, and all of its bytes must be
    /// ASCII codepoints that are either `-` or alpha-numeric.
    fn name_long(&self) -> &'static str;

    /// Returns a list of aliases for this flag.
    ///
    /// The aliases must follow the same rules as `Flag::name_long`.
    ///
    /// By default, an empty slice is returned.
    fn aliases(&self) -> &'static [&'static str] {
        &[]
    }

    /// Returns a negated name for this flag. The negation of a flag is
    /// intended to have the opposite meaning of a flag or to otherwise turn
    /// something "off" or revert it to its default behavior.
    ///
    /// Negated flags are not listed in their own section in the `-h/--help`
    /// output or man page. Instead, they are automatically mentioned at the
    /// end of the documentation section of the flag they negated.
    ///
    /// The aliases must follow the same rules as `Flag::name_long`.
    ///
    /// By default, a flag has no negation and this returns `None`.
    fn name_negated(&self) -> Option<&'static str> {
        None
    }

    /// Returns the variable name describing the type of value this flag
    /// accepts. This should always be set for non-switch flags and never set
    /// for switch flags.
    ///
    /// For example, the `--max-count` flag has its variable name set to `NUM`.
    ///
    /// The convention is to capitalize variable names.
    ///
    /// By default this returns `None`.
    fn doc_variable(&self) -> Option<&'static str> {
        None
    }

    /// Returns the category of this flag.
    ///
    /// Every flag must have a single category. Categories are used to organize
    /// flags in the generated documentation.
    fn doc_category(&self) -> Category;

    /// A (very) short documentation string describing what this flag does.
    ///
    /// This may sacrifice "proper English" in order to be as terse as
    /// possible. Generally, we try to ensure that `rg -h` doesn't have any
    /// lines that exceed 79 columns.
    fn doc_short(&self) -> &'static str;

    /// A (possibly very) longer documentation string describing in full
    /// detail what this flag does. This should be in mandoc/mdoc format.
    fn doc_long(&self) -> &'static str;

    /// If this is a non-switch flag that accepts a small set of specific
    /// values, then this should list them.
    ///
    /// This returns an empty slice by default.
    fn doc_choices(&self) -> &'static [&'static str] {
        &[]
    }

    fn completion_type(&self) -> CompletionType {
        CompletionType::Other
    }

    /// Given the parsed value (which might just be a switch), this should
    /// update the state in `args` based on the value given for this flag.
    ///
    /// This may update state for other flags as appropriate.
    ///
    /// The `-V/--version` and `-h/--help` flags are treated specially in the
    /// parser and should do nothing here.
    ///
    /// By convention, implementations should generally not try to "do"
    /// anything other than validate the value given. For example, the
    /// implementation for `--hostname-bin` should not try to resolve the
    /// hostname to use by running the binary provided. That should be saved
    /// for a later step. This convention is used to ensure that getting the
    /// low-level arguments is as reliable and quick as possible. It also
    /// ensures that "doing something" occurs a minimal number of times. For
    /// example, by avoiding trying to find the hostname here, we can do it
    /// once later no matter how many times `--hostname-bin` is provided.
    ///
    /// Implementations should not include the flag name in the error message
    /// returned. The flag name is included automatically by the parser.
    fn update(
        &self,
        value: FlagValue,
        args: &mut crate::flags::lowargs::LowArgs,
    ) -> anyhow::Result<()>;
}

/// The category that a flag belongs to.
///
/// Categories are used to organize flags into "logical" groups in the
/// generated documentation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum Category {
    /// Flags related to how ripgrep reads its input. Its "input" generally
    /// consists of the patterns it is trying to match and the haystacks it is
    /// trying to search.
    Input,
    /// Flags related to the operation of the search itself. For example,
    /// whether case insensitive matching is enabled.
    Search,
    /// Flags related to how ripgrep filters haystacks. For example, whether
    /// to respect gitignore files or not.
    Filter,
    /// Flags related to how ripgrep shows its search results. For example,
    /// whether to show line numbers or not.
    Output,
    /// Flags related to changing ripgrep's output at a more fundamental level.
    /// For example, flags like `--count` suppress printing of individual
    /// lines, and instead just print the total count of matches for each file
    /// searched.
    OutputModes,
    /// Flags related to indexing. This section includes flags both for
    /// searching an index and updating an index.
    Indexing,
    /// Flags related to logging behavior such as emitting non-fatal error
    /// messages or printing search statistics.
    Logging,
    /// Other behaviors not related to ripgrep's core functionality. For
    /// example, printing the file type globbing rules, or printing the list
    /// of files ripgrep would search without actually searching them.
    OtherBehaviors,
}

impl Category {
    /// Returns a string representation of this category.
    ///
    /// This string is the name of the variable used in various templates for
    /// generated documentation. This name can be used for interpolation.
    fn as_str(&self) -> &'static str {
        match *self {
            Category::Input => "input",
            Category::Search => "search",
            Category::Filter => "filter",
            Category::Output => "output",
            Category::OutputModes => "output-modes",
            Category::Indexing => "indexing",
            Category::Logging => "logging",
            Category::OtherBehaviors => "other-behaviors",
        }
    }
}

/// The kind of argument a flag accepts, to be used for shell completions.
#[derive(Clone, Copy, Debug)]
enum CompletionType {
    /// No special category. is_switch() and doc_choices() may apply.
    Other,
    /// A path to a file.
    Filename,
    /// A command in $PATH.
    Executable,
    /// The name of a file type, as used by e.g. --type.
    Filetype,
    /// The name of an encoding_rs encoding, as used by --encoding.
    Encoding,
}

/// Represents a value parsed from the command line.
///
/// This doesn't include the corresponding flag, but values come in one of
/// two forms: a switch (on or off) or an arbitrary value.
///
/// Note that the CLI doesn't directly support negated switches. For example,
/// you can'd do anything like `-n=false` or any of that nonsense. Instead,
/// the CLI parser knows about which flag names are negations and which aren't
/// (courtesy of the `Flag` trait). If a flag given is known as a negation,
/// then a `FlagValue::Switch(false)` value is passed into `Flag::update`.
#[derive(Debug)]
enum FlagValue {
    /// A flag that is either on or off.
    Switch(bool),
    /// A flag that comes with an arbitrary user value.
    Value(OsString),
}

impl FlagValue {
    /// Return the yes or no value of this switch.
    ///
    /// If this flag value is not a switch, then this panics.
    ///
    /// This is useful when writing the implementation of `Flag::update`.
    /// namely, callers usually know whether a switch or a value is expected.
    /// If a flag is something different, then it indicates a bug, and thus a
    /// panic is acceptable.
    fn unwrap_switch(self) -> bool {
        match self {
            FlagValue::Switch(yes) => yes,
            FlagValue::Value(_) => {
                unreachable!("got flag value but expected switch")
            }
        }
    }

    /// Return the user provided value of this flag.
    ///
    /// If this flag is a switch, then this panics.
    ///
    /// This is useful when writing the implementation of `Flag::update`.
    /// namely, callers usually know whether a switch or a value is expected.
    /// If a flag is something different, then it indicates a bug, and thus a
    /// panic is acceptable.
    fn unwrap_value(self) -> OsString {
        match self {
            FlagValue::Switch(_) => {
                unreachable!("got switch but expected flag value")
            }
            FlagValue::Value(v) => v,
        }
    }
}
```
### crates/ignore/src/gitignore.rs
```rust
/*!
The gitignore module provides a way to match globs from a gitignore file
against file paths.

Note that this module implements the specification as described in the
`gitignore` man page from scratch. That is, this module does *not* shell out to
the `git` command line tool.
*/

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::{Path, PathBuf},
    sync::Arc,
};

use {
    globset::{Candidate, GlobBuilder, GlobSet, GlobSetBuilder},
    regex_automata::util::pool::Pool,
};

use crate::{
    Error, Match, PartialErrorBuilder,
    pathutil::{is_file_name, strip_prefix},
};

/// Glob represents a single glob in a gitignore file.
///
/// This is used to report information about the highest precedent glob that
/// matched in one or more gitignore files.
#[derive(Clone, Debug)]
pub struct Glob {
    /// The file path that this glob was extracted from.
    from: Option<PathBuf>,
    /// The original glob string.
    original: String,
    /// The actual glob string used to convert to a regex.
    actual: String,
    /// Whether this is a whitelisted glob or not.
    is_whitelist: bool,
    /// Whether this glob should only match directories or not.
    is_only_dir: bool,
}

impl Glob {
    /// Returns the file path that defined this glob.
    pub fn from(&self) -> Option<&Path> {
        self.from.as_ref().map(|p| &**p)
    }

    /// The original glob as it was defined in a gitignore file.
    pub fn original(&self) -> &str {
        &self.original
    }

    /// The actual glob that was compiled to respect gitignore
    /// semantics.
    pub fn actual(&self) -> &str {
        &self.actual
    }

    /// Whether this was a whitelisted glob or not.
    pub fn is_whitelist(&self) -> bool {
        self.is_whitelist
    }

    /// Whether this glob must match a directory or not.
    pub fn is_only_dir(&self) -> bool {
        self.is_only_dir
    }

    /// Returns true if and only if this glob has a `**/` prefix.
    fn has_doublestar_prefix(&self) -> bool {
        self.actual.starts_with("**/") || self.actual == "**"
    }
}

/// Gitignore is a matcher for the globs in one or more gitignore files
/// in the same directory.
#[derive(Clone, Debug)]
pub struct Gitignore {
    set: GlobSet,
    root: PathBuf,
    globs: Vec<Glob>,
    num_ignores: u64,
    num_whitelists: u64,
    matches: Option<Arc<Pool<Vec<usize>>>>,
}

impl Gitignore {
    /// Creates a new gitignore matcher from the gitignore file path given.
    ///
    /// If it's desirable to include multiple gitignore files in a single
    /// matcher, or read gitignore globs from a different source, then
    /// use `GitignoreBuilder`.
    ///
    /// This always returns a valid matcher, even if it's empty. In particular,
    /// a Gitignore file can be partially valid, e.g., when one glob is invalid
    /// but the rest aren't.
    ///
    /// Note that I/O errors are ignored. For more granular control over
    /// errors, use `GitignoreBuilder`.
    pub fn new<P: AsRef<Path>>(
        gitignore_path: P,
    ) -> (Gitignore, Option<Error>) {
        let path = gitignore_path.as_ref();
        let parent = path.parent().unwrap_or(Path::new("/"));
        let mut builder = GitignoreBuilder::new(parent);
        let mut errs = PartialErrorBuilder::default();
        errs.maybe_push_ignore_io(builder.add(path));
        match builder.build() {
            Ok(gi) => (gi, errs.into_error_option()),
            Err(err) => {
                errs.push(err);
                (Gitignore::empty(), errs.into_error_option())
            }
        }
    }

    /// Creates a new gitignore matcher from the global ignore file, if one
    /// exists.
    ///
    /// The global config file path is specified by git's `core.excludesFile`
    /// config option.
    ///
    /// # Behavior
    ///
    /// This routine does its best to discover any global git exclude files.
    /// This will try to parse out the `excludesFile` config option in your
    /// global git configuration, if necessary.
    ///
    /// The specific things this routine tries (which are subject to change
    /// based on how git behaves) are:
    ///
    ///
    ///
    /// Git's config file location is `$HOME/.gitconfig`. If `$HOME/.gitconfig`
    /// does not exist or does not specify `core.excludesFile`, then
    /// `$XDG_CONFIG_HOME/git/ignore` is read. If `$XDG_CONFIG_HOME` is not
    /// set or is empty, then `$HOME/.config/git/ignore` is used instead.
    pub fn global() -> (Gitignore, Option<Error>) {
        match std::env::current_dir() {
            Ok(cwd) => GitignoreBuilder::new(cwd).build_global(),
            Err(err) => (Gitignore::empty(), Some(err.into())),
        }
    }

    /// Creates a new empty gitignore matcher that never matches anything.
    ///
    /// Its path is empty.
    pub fn empty() -> Gitignore {
        Gitignore {
            set: GlobSet::empty(),
            root: PathBuf::from(""),
            globs: vec![],
            num_ignores: 0,
            num_whitelists: 0,
            matches: None,
        }
    }

    /// Returns the directory containing this gitignore matcher.
    ///
    /// All matches are done relative to this path.
    pub fn path(&self) -> &Path {
        &*self.root
    }

    /// Returns true if and only if this gitignore has zero globs, and
    /// therefore never matches any file path.
    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    /// Returns the total number of globs, which should be equivalent to
    /// `num_ignores + num_whitelists`.
    pub fn len(&self) -> usize {
        self.set.len()
    }

    /// Returns the total number of ignore globs.
    pub fn num_ignores(&self) -> u64 {
        self.num_ignores
    }

    /// Returns the total number of whitelisted globs.
    pub fn num_whitelists(&self) -> u64 {
        self.num_whitelists
    }

    /// Returns whether the given path (file or directory) matched a pattern in
    /// this gitignore matcher.
    ///
    /// `is_dir` should be true if the path refers to a directory and false
    /// otherwise.
    ///
    /// The given path is matched relative to the path given when building
    /// the matcher. Specifically, before matching `path`, its prefix (as
    /// determined by a common suffix of the directory containing this
    /// gitignore) is stripped. If there is no common suffix/prefix overlap,
    /// then `path` is assumed to be relative to this matcher.
    pub fn matched<P: AsRef<Path>>(
        &self,
        path: P,
        is_dir: bool,
    ) -> Match<&Glob> {
        if self.is_empty() {
            return Match::None;
        }
        self.matched_stripped(self.strip(path.as_ref()), is_dir)
    }

    /// Returns whether the given path (file or directory, and expected to be
    /// under the root) or any of its parent directories (up to the root)
    /// matched a pattern in this gitignore matcher.
    ///
    /// NOTE: This method is more expensive than walking the directory hierarchy
    /// top-to-bottom and matching the entries. But, is easier to use in cases
