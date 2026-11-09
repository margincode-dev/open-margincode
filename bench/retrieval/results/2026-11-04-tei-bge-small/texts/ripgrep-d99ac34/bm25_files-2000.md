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
