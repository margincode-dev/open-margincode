//! Git-format patches for remote checkpoints (Plan 14). `render` writes one from two versions of
//! each path; `apply` works out what a patch does to the current files. Both are pure (no disk, no
//! git, no repository; `render` uses jj-lib's line diff). The format is what
//! `git diff --binary --full-index --no-renames` writes and what
//! `git apply` reads: text hunks, `GIT binary patch` (`literal` and `delta`), new and deleted
//! files, `old mode`/`new mode`.
//!
//! `apply` is strict where `git apply` is lenient: no fuzz and no offsets (a remote checkpoint
//! names the base it was made against, and the daemon has checked that base before it applies),
//! the blob ids of `index` lines must match, and the whole patch is worked out before anything is
//! returned, so a patch either applies whole or not at all. Renames and copies, symlinks and
//! submodules are refused, as are paths outside the workspace or into its metadata.

use std::collections::HashSet;
use std::io::{Read as _, Write as _};

use margincode_core::{AvError, Result};
use sha1::{Digest as _, Sha1};

pub const FILE: u32 = 0o100644;
pub const EXECUTABLE: u32 = 0o100755;
pub const SYMLINK: u32 = 0o120000;
const GITLINK: u32 = 0o160000;
const NULL_ID: &str = "0000000000000000000000000000000000000000";
/// Largest file a binary hunk may inflate to (the snapshot's own limit for new files).
pub const MAX_FILE_BYTES: usize = 256 * 1024 * 1024;
/// Context lines around each rendered hunk (git's default).
const CONTEXT: usize = 3;

/// One version of a file: its git mode (`FILE`, `EXECUTABLE`; `SYMLINK` with the target as bytes)
/// and its content.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Blob {
    pub mode: u32,
    pub bytes: Vec<u8>,
}

/// One path in two versions, for `render`. `None` is absent.
#[derive(Clone, Debug)]
pub struct FileDiff {
    pub path: String,
    pub before: Option<Blob>,
    pub after: Option<Blob>,
}

/// What a patch does to one path: its new version, or `None` when it deletes it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Applied {
    pub path: String,
    pub after: Option<Blob>,
}

fn bad(msg: impl std::fmt::Display) -> AvError {
    AvError::Invalid(format!("patch: {msg}"))
}

/// Git's id for a blob with this content (`git hash-object`).
pub fn git_blob_id(bytes: &[u8]) -> String {
    let mut h = Sha1::new();
    h.update(format!("blob {}\0", bytes.len()).as_bytes());
    h.update(bytes);
    hex(&h.finalize())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Git's test for binary content: a NUL in the first 8000 bytes.
fn is_binary(bytes: &[u8]) -> bool {
    bytes[..bytes.len().min(8000)].contains(&0)
}

// ---------------------------------------------------------------------------------------------
// render

/// A patch that turns every `before` into its `after`. Paths whose versions are equal add
/// nothing; with none changed the patch is empty.
pub fn render(diffs: &[FileDiff]) -> Vec<u8> {
    let mut out = Vec::new();
    for d in diffs {
        render_one(&mut out, d);
    }
    out
}

fn line(out: &mut Vec<u8>, s: &str) {
    out.extend_from_slice(s.as_bytes());
    out.push(b'\n');
}

fn render_one(out: &mut Vec<u8>, d: &FileDiff) {
    let (before, after) = (d.before.as_ref(), d.after.as_ref());
    if before == after {
        return;
    }
    let a = quote(&format!("a/{}", d.path));
    let b = quote(&format!("b/{}", d.path));
    line(out, &format!("diff --git {a} {b}"));
    let id = |x: Option<&Blob>| x.map_or(NULL_ID.to_owned(), |x| git_blob_id(&x.bytes));
    let (old_id, new_id) = (id(before), id(after));
    match (before, after) {
        (None, Some(n)) => {
            line(out, &format!("new file mode {:o}", n.mode));
            line(out, &format!("index {old_id}..{new_id}"));
        }
        (Some(o), None) => {
            line(out, &format!("deleted file mode {:o}", o.mode));
            line(out, &format!("index {old_id}..{new_id}"));
        }
        (Some(o), Some(n)) => {
            if o.mode != n.mode {
                line(out, &format!("old mode {:o}", o.mode));
                line(out, &format!("new mode {:o}", n.mode));
            }
            if o.bytes == n.bytes {
                return;
            }
            if o.mode == n.mode {
                line(out, &format!("index {old_id}..{new_id} {:o}", o.mode));
            } else {
                line(out, &format!("index {old_id}..{new_id}"));
            }
        }
        (None, None) => unreachable!("equal versions returned above"),
    }
    let empty: &[u8] = &[];
    let old = before.map_or(empty, |x| &x.bytes);
    let new = after.map_or(empty, |x| &x.bytes);
    if old.is_empty() && new.is_empty() {
        return;
    }
    if is_binary(old) || is_binary(new) {
        line(out, "GIT binary patch");
        binary_literal(out, new);
        binary_literal(out, old);
        return;
    }
    let side = |x: Option<&Blob>, name: &str| {
        if x.is_some() {
            name.to_owned()
        } else {
            "/dev/null".to_owned()
        }
    };
    line(out, &format!("--- {}", side(before, &a)));
    line(out, &format!("+++ {}", side(after, &b)));
    text_hunks(out, old, new);
}

fn text_hunks(out: &mut Vec<u8>, old: &[u8], new: &[u8]) {
    use jj_lib::diff_presentation::LineCompareMode;
    use jj_lib::diff_presentation::unified::{DiffLineType, unified_diff_hunks};
    use jj_lib::merge::Diff;
    let hunks = unified_diff_hunks(
        Diff::new(old.into(), new.into()),
        CONTEXT,
        LineCompareMode::Exact,
    );
    let range = |r: &std::ops::Range<usize>| match r.len() {
        0 => format!("{},0", r.start),
        1 => format!("{}", r.start + 1),
        n => format!("{},{n}", r.start + 1),
    };
    for h in hunks {
        line(
            out,
            &format!(
                "@@ -{} +{} @@",
                range(&h.left_line_range),
                range(&h.right_line_range)
            ),
        );
        for (kind, tokens) in &h.lines {
            out.push(match kind {
                DiffLineType::Context => b' ',
                DiffLineType::Removed => b'-',
                DiffLineType::Added => b'+',
            });
            let mut text = Vec::new();
            for (_, t) in tokens {
                text.extend_from_slice(t);
            }
            out.extend_from_slice(&text);
            if !text.ends_with(b"\n") {
                out.push(b'\n');
                line(out, "\\ No newline at end of file");
            }
        }
    }
}

const B85: &[u8; 85] =
    b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+-;<=>?@^_`{|}~";

fn binary_literal(out: &mut Vec<u8>, bytes: &[u8]) {
    let mut z = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    z.write_all(bytes).expect("writing to a Vec");
    let data = z.finish().expect("writing to a Vec");
    line(out, &format!("literal {}", bytes.len()));
    for chunk in data.chunks(52) {
        let n = chunk.len() as u8;
        out.push(if n <= 26 { b'A' + n - 1 } else { b'a' + n - 27 });
        for group in chunk.chunks(4) {
            let mut word = [0u8; 4];
            word[..group.len()].copy_from_slice(group);
            let mut v = u32::from_be_bytes(word);
            let mut enc = [0u8; 5];
            for slot in enc.iter_mut().rev() {
                *slot = B85[(v % 85) as usize];
                v /= 85;
            }
            out.extend_from_slice(&enc);
        }
        out.push(b'\n');
    }
    out.push(b'\n');
}

/// `s` as git writes a path in a header: in C quotes when it holds a quote, a backslash, a
/// control byte or a non-ASCII byte.
fn quote(s: &str) -> String {
    let needs = s
        .bytes()
        .any(|b| b == b'"' || b == b'\\' || !(0x20..0x7f).contains(&b));
    if !needs {
        return s.to_owned();
    }
    let mut q = String::from("\"");
    for b in s.bytes() {
        match b {
            b'"' => q.push_str("\\\""),
            b'\\' => q.push_str("\\\\"),
            b'\t' => q.push_str("\\t"),
            b'\n' => q.push_str("\\n"),
            b'\r' => q.push_str("\\r"),
            0x20..=0x7e => q.push(b as char),
            _ => q.push_str(&format!("\\{b:03o}")),
        }
    }
    q.push('"');
    q
}

// ---------------------------------------------------------------------------------------------
// parse

#[derive(Debug)]
enum Kind {
    Context,
    Removed,
    Added,
}

#[derive(Debug)]
struct Hunk {
    old_start: usize,
    old_len: usize,
    lines: Vec<(Kind, Vec<u8>)>,
}

#[derive(Debug)]
enum Binary {
    Literal(Vec<u8>),
    Delta(Vec<u8>),
}

#[derive(Debug, Default)]
struct FilePatch {
    /// From `diff --git`, used when no `---`/`+++` lines name the file.
    header_path: String,
    /// `---` / `+++`: `Some(None)` is `/dev/null`.
    old_path: Option<Option<String>>,
    new_path: Option<Option<String>>,
    new_file: Option<u32>,
    deleted: Option<u32>,
    old_mode: Option<u32>,
    new_mode: Option<u32>,
    old_id: Option<String>,
    new_id: Option<String>,
    hunks: Vec<Hunk>,
    binary: Option<Binary>,
}

/// The patch's lines without their `\n`.
fn lines(p: &[u8]) -> Vec<&[u8]> {
    let mut v: Vec<&[u8]> = p.split(|b| *b == b'\n').collect();
    if v.last().is_some_and(|l| l.is_empty()) {
        v.pop();
    }
    v
}

fn text(l: &[u8]) -> Result<&str> {
    std::str::from_utf8(l).map_err(|_| bad("a header line is not UTF-8"))
}

fn mode(s: &str) -> Result<u32> {
    u32::from_str_radix(s.trim(), 8).map_err(|_| bad(format!("bad mode {s:?}")))
}

fn parse(p: &[u8]) -> Result<Vec<FilePatch>> {
    let ls = lines(p);
    let mut files: Vec<FilePatch> = Vec::new();
    let mut i = 0;
    while i < ls.len() {
        let l = ls[i];
        if let Some(rest) = l.strip_prefix(b"diff --git ") {
            files.push(FilePatch {
                header_path: header_path(text(rest)?)?,
                ..FilePatch::default()
            });
            i += 1;
            continue;
        }
        let Some(f) = files.last_mut() else {
            if l.iter().all(u8::is_ascii_whitespace) {
                i += 1;
                continue;
            }
            return Err(bad("expected `diff --git` (make it with `git diff`)"));
        };
        if l.starts_with(b"@@ ") {
            let (h, next) = parse_hunk(&ls, i)?;
            f.hunks.push(h);
            i = next;
            continue;
        }
        if l == b"GIT binary patch" {
            let (b, next) = parse_binary(&ls, i + 1)?;
            f.binary = Some(b);
            i = next;
            continue;
        }
        let t = text(l)?;
        if let Some(m) = t.strip_prefix("new file mode ") {
            f.new_file = Some(mode(m)?);
        } else if let Some(m) = t.strip_prefix("deleted file mode ") {
            f.deleted = Some(mode(m)?);
        } else if let Some(m) = t.strip_prefix("old mode ") {
            f.old_mode = Some(mode(m)?);
        } else if let Some(m) = t.strip_prefix("new mode ") {
            f.new_mode = Some(mode(m)?);
        } else if let Some(ids) = t.strip_prefix("index ") {
            let ids = ids.split(' ').next().unwrap_or_default();
            let (a, b) = ids
                .split_once("..")
                .ok_or_else(|| bad(format!("bad index line {t:?}")))?;
            f.old_id = Some(a.to_owned());
            f.new_id = Some(b.to_owned());
        } else if let Some(name) = t.strip_prefix("--- ") {
            f.old_path = Some(side_path(name, "a/")?);
        } else if let Some(name) = t.strip_prefix("+++ ") {
            f.new_path = Some(side_path(name, "b/")?);
        } else if t.starts_with("rename ")
            || t.starts_with("copy ")
            || t.starts_with("similarity index")
            || t.starts_with("dissimilarity index")
        {
            return Err(bad(
                "renames and copies are not supported; make the patch with `git diff --no-renames`",
            ));
        } else if t.starts_with("Binary files ") {
            return Err(bad(format!(
                "{}: a binary file without its content; make the patch with `git diff --binary`",
                f.header_path
            )));
        } else if !t.trim().is_empty() {
            return Err(bad(format!("unexpected line {t:?}")));
        }
        i += 1;
    }
    Ok(files)
}

/// The path of `a/X b/X` (either side may be quoted).
fn header_path(rest: &str) -> Result<String> {
    let a = if rest.starts_with('"') {
        let (a, _) = unquote_prefix(rest)?;
        a
    } else if let Some(q) = rest.find(" \"b/") {
        rest[..q].to_owned()
    } else {
        // `a/X b/X`: both halves are the same path (no renames)
        let n = rest.len();
        let half = (n.max(1) - 1) / 2;
        if n < 5
            || n.is_multiple_of(2)
            || !rest.is_char_boundary(half)
            || !rest.is_char_boundary(half + 1)
        {
            return Err(bad(format!("bad `diff --git` line {rest:?}")));
        }
        let (a, b) = (&rest[..half], &rest[half + 1..]);
        if a.get(2..) != b.get(2..) {
            return Err(bad(
                "renames and copies are not supported; make the patch with `git diff --no-renames`",
            ));
        }
        a.to_owned()
    };
    a.strip_prefix("a/").map(str::to_owned).ok_or_else(|| {
        bad("paths must start with a/ and b/ (`git diff --src-prefix=a/ --dst-prefix=b/`)")
    })
}

/// `---`/`+++` name: `None` for `/dev/null`, else the path without its prefix.
fn side_path(name: &str, prefix: &str) -> Result<Option<String>> {
    // git ends a name holding a space with a tab
    let name = name.strip_suffix('\t').unwrap_or(name);
    if name == "/dev/null" {
        return Ok(None);
    }
    let name = if name.starts_with('"') {
        let (s, rest) = unquote_prefix(name)?;
        if !rest.trim().is_empty() {
            return Err(bad(format!("bad file name {name:?}")));
        }
        s
    } else {
        name.to_owned()
    };
    name.strip_prefix(prefix)
        .map(|p| Some(p.to_owned()))
        .ok_or_else(|| {
            bad("paths must start with a/ and b/ (`git diff --src-prefix=a/ --dst-prefix=b/`)")
        })
}

/// A C-quoted string at the start of `s`, and what follows it.
fn unquote_prefix(s: &str) -> Result<(String, &str)> {
    let b = s.as_bytes();
    let mut out = Vec::new();
    let mut i = 1;
    while i < b.len() {
        match b[i] {
            b'"' => {
                let name = String::from_utf8(out).map_err(|_| bad("a path is not UTF-8"))?;
                return Ok((name, &s[i + 1..]));
            }
            b'\\' => {
                let c = *b.get(i + 1).ok_or_else(|| bad("bad quoted path"))?;
                i += 2;
                out.push(match c {
                    b'a' => 7,
                    b'b' => 8,
                    b't' => b'\t',
                    b'n' => b'\n',
                    b'v' => 11,
                    b'f' => 12,
                    b'r' => b'\r',
                    b'"' | b'\\' => c,
                    b'0'..=b'7' => {
                        let digits = b.get(i - 1..i + 2).ok_or_else(|| bad("bad quoted path"))?;
                        i += 2;
                        let d = std::str::from_utf8(digits).map_err(|_| bad("bad quoted path"))?;
                        u8::from_str_radix(d, 8).map_err(|_| bad("bad quoted path"))?
                    }
                    _ => return Err(bad("bad quoted path")),
                });
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    Err(bad("unterminated quoted path"))
}

/// `@@ -a[,b] +c[,d] @@` and its lines, from `ls[at]`; returns the hunk and the next line index.
fn parse_hunk(ls: &[&[u8]], at: usize) -> Result<(Hunk, usize)> {
    let header = text(ls[at])?;
    let inner = header
        .strip_prefix("@@ -")
        .and_then(|r| r.split_once(" @@"))
        .map(|(r, _)| r)
        .ok_or_else(|| bad(format!("bad hunk header {header:?}")))?;
    let (old, new) = inner
        .split_once(" +")
        .ok_or_else(|| bad(format!("bad hunk header {header:?}")))?;
    let range = |r: &str| -> Result<(usize, usize)> {
        let (s, n) = r.split_once(',').unwrap_or((r, "1"));
        let parse = |x: &str| {
            x.parse::<usize>()
                .map_err(|_| bad(format!("bad hunk header {header:?}")))
        };
        Ok((parse(s)?, parse(n)?))
    };
    let (old_start, old_len) = range(old)?;
    let (_, new_len) = range(new)?;
    let (mut old_left, mut new_left) = (old_len, new_len);
    let mut lines: Vec<(Kind, Vec<u8>)> = Vec::new();
    let mut i = at + 1;
    while old_left > 0 || new_left > 0 {
        let Some(l) = ls.get(i) else {
            return Err(bad(format!("hunk {header:?} ends early")));
        };
        let (kind, body) = match l.first() {
            Some(b' ') => (Kind::Context, &l[1..]),
            // a context line whose space was stripped
            None => (Kind::Context, &l[..]),
            Some(b'-') => (Kind::Removed, &l[1..]),
            Some(b'+') => (Kind::Added, &l[1..]),
            Some(b'\\') => {
                no_newline(&mut lines)?;
                i += 1;
                continue;
            }
            _ => return Err(bad(format!("hunk {header:?} ends early"))),
        };
        match kind {
            Kind::Context if old_left > 0 && new_left > 0 => {
                old_left -= 1;
                new_left -= 1;
            }
            Kind::Removed if old_left > 0 => old_left -= 1,
            Kind::Added if new_left > 0 => new_left -= 1,
            _ => return Err(bad(format!("hunk {header:?} has more lines than it says"))),
        }
        let mut content = body.to_vec();
        content.push(b'\n');
        lines.push((kind, content));
        i += 1;
    }
    if ls.get(i).is_some_and(|l| l.starts_with(b"\\")) {
        no_newline(&mut lines)?;
        i += 1;
    }
    Ok((
        Hunk {
            old_start,
            old_len,
            lines,
        },
        i,
    ))
}

fn no_newline(lines: &mut [(Kind, Vec<u8>)]) -> Result<()> {
    let last = lines
        .last_mut()
        .ok_or_else(|| bad("`\\ No newline` before any line"))?;
    last.1.pop();
    Ok(())
}

/// The forward block of a `GIT binary patch` from `ls[at]`, skipping the reverse block.
fn parse_binary(ls: &[&[u8]], at: usize) -> Result<(Binary, usize)> {
    let (forward, mut i) = parse_binary_block(ls, at)?;
    let forward = forward.ok_or_else(|| bad("binary patch without data"))?;
    if ls
        .get(i)
        .is_some_and(|l| l.starts_with(b"literal ") || l.starts_with(b"delta "))
    {
        i = parse_binary_block(ls, i)?.1;
    }
    Ok((forward, i))
}

fn parse_binary_block(ls: &[&[u8]], at: usize) -> Result<(Option<Binary>, usize)> {
    let Some(head) = ls.get(at) else {
        return Ok((None, at));
    };
    let head = text(head)?;
    let (kind, size) = head
        .split_once(' ')
        .ok_or_else(|| bad(format!("bad binary hunk {head:?}")))?;
    let size: usize = size
        .parse()
        .map_err(|_| bad(format!("bad binary hunk {head:?}")))?;
    if size > MAX_FILE_BYTES {
        return Err(bad(format!("a binary hunk of {size} bytes is too large")));
    }
    let mut data = Vec::new();
    let mut i = at + 1;
    while let Some(l) = ls.get(i) {
        i += 1;
        if l.is_empty() {
            break;
        }
        decode_b85_line(l, &mut data)?;
    }
    let mut inflated = Vec::new();
    flate2::read::ZlibDecoder::new(&data[..])
        .take(size as u64 + 1)
        .read_to_end(&mut inflated)
        .map_err(|e| bad(format!("binary hunk does not inflate: {e}")))?;
    if inflated.len() != size {
        return Err(bad("binary hunk has the wrong size"));
    }
    let b = match kind {
        "literal" => Binary::Literal(inflated),
        "delta" => Binary::Delta(inflated),
        _ => return Err(bad(format!("bad binary hunk {head:?}"))),
    };
    Ok((Some(b), i))
}

fn decode_b85_line(l: &[u8], out: &mut Vec<u8>) -> Result<()> {
    let n = match l.first() {
        Some(c @ b'A'..=b'Z') => (c - b'A' + 1) as usize,
        Some(c @ b'a'..=b'z') => (c - b'a' + 27) as usize,
        _ => return Err(bad("bad binary line")),
    };
    let chars = &l[1..];
    if chars.len() != n.div_ceil(4) * 5 {
        return Err(bad("bad binary line length"));
    }
    let mut got = Vec::with_capacity(chars.len() / 5 * 4);
    for group in chars.chunks(5) {
        let mut v: u64 = 0;
        for c in group {
            let d = B85
                .iter()
                .position(|x| x == c)
                .ok_or_else(|| bad("bad base85 character"))?;
            v = v * 85 + d as u64;
        }
        let v = u32::try_from(v).map_err(|_| bad("bad base85 group"))?;
        got.extend_from_slice(&v.to_be_bytes());
    }
    out.extend_from_slice(&got[..n]);
    Ok(())
}

/// Git's delta format: source and target sizes, then copy and insert instructions.
fn apply_delta(src: &[u8], delta: &[u8]) -> Result<Vec<u8>> {
    let mut i = 0;
    let mut varint = || -> Result<usize> {
        let (mut v, mut shift) = (0usize, 0);
        loop {
            let b = *delta.get(i).ok_or_else(|| bad("truncated delta"))?;
            i += 1;
            v |= ((b & 0x7f) as usize) << shift;
            shift += 7;
            if b & 0x80 == 0 || shift > 56 {
                return Ok(v);
            }
        }
    };
    let (src_len, dst_len) = (varint()?, varint()?);
    if src_len != src.len() {
        return Err(bad(
            "binary delta was made against another version of the file",
        ));
    }
    if dst_len > MAX_FILE_BYTES {
        return Err(bad("binary delta is too large"));
    }
    let mut out = Vec::with_capacity(dst_len);
    while i < delta.len() {
        let cmd = delta[i];
        i += 1;
        if cmd & 0x80 != 0 {
            let mut field = |bits: u8, count: usize| -> Result<usize> {
                let mut v = 0usize;
                for k in 0..count {
                    if bits & (1 << k) != 0 {
                        let b = *delta.get(i).ok_or_else(|| bad("truncated delta"))?;
                        i += 1;
                        v |= (b as usize) << (8 * k);
                    }
                }
                Ok(v)
            };
            let offset = field(cmd & 0x0f, 4)?;
            let mut size = field((cmd >> 4) & 0x07, 3)?;
            if size == 0 {
                size = 0x10000;
            }
            let piece = src
                .get(offset..offset.saturating_add(size))
                .ok_or_else(|| bad("binary delta copies outside the file"))?;
            out.extend_from_slice(piece);
        } else if cmd != 0 {
            let n = cmd as usize;
            let piece = delta.get(i..i + n).ok_or_else(|| bad("truncated delta"))?;
            out.extend_from_slice(piece);
            i += n;
        } else {
            return Err(bad("bad delta instruction"));
        }
        if out.len() > dst_len {
            return Err(bad("binary delta is longer than it says"));
        }
    }
    if out.len() != dst_len {
        return Err(bad("binary delta is shorter than it says"));
    }
    Ok(out)
}

// ---------------------------------------------------------------------------------------------
// apply

/// A workspace-relative path a patch may write: plain components only, nothing in `.jj` or `.git`,
/// not the CLI's own `.av-task`.
fn check_path(path: &str) -> Result<()> {
    let refuse = |why: &str| Err(bad(format!("path {path:?} {why}")));
    if path.is_empty() || path.starts_with('/') || path.contains('\\') || path.contains('\0') {
        return refuse("is not a relative path");
    }
    for c in path.split('/') {
        if c.is_empty() || c == "." || c == ".." {
            return refuse("is not a plain relative path");
        }
        if c == ".jj" || c.eq_ignore_ascii_case(".git") {
            return refuse("is repository metadata");
        }
    }
    if path == ".av-task" {
        return refuse("is the workspace's own task file");
    }
    Ok(())
}

fn check_mode(path: &str, m: u32) -> Result<()> {
    match m {
        FILE | EXECUTABLE => Ok(()),
        SYMLINK => Err(bad(format!(
            "{path}: symlinks cannot be changed through a patch"
        ))),
        GITLINK => Err(bad(format!(
            "{path}: submodules cannot be changed through a patch"
        ))),
        m => Err(bad(format!("{path}: unsupported mode {m:o}"))),
    }
}

/// Whether `id` (full or abbreviated, from an `index` line) names `bytes`.
fn id_matches(id: &str, bytes: &[u8]) -> bool {
    id.len() < 7 || id.bytes().all(|b| b == b'0') || git_blob_id(bytes).starts_with(id)
}

/// What `patch` does to the files `current` reads (`None`: no such file). Nothing is returned
/// unless every file of the patch applies; paths come back in patch order.
pub fn apply(
    patch: &[u8],
    mut current: impl FnMut(&str) -> Result<Option<Blob>>,
) -> Result<Vec<Applied>> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for f in parse(patch)? {
        let path = match (&f.old_path, &f.new_path) {
            (Some(Some(a)), Some(Some(b))) if a != b => {
                return Err(bad(
                    "renames and copies are not supported; make the patch with `git diff --no-renames`",
                ));
            }
            (_, Some(Some(p))) | (Some(Some(p)), _) => p.clone(),
            _ => f.header_path.clone(),
        };
        check_path(&path)?;
        if !seen.insert(path.clone()) {
            return Err(bad(format!("{path} appears twice")));
        }
        for m in [f.new_file, f.deleted, f.old_mode, f.new_mode]
            .into_iter()
            .flatten()
        {
            check_mode(&path, m)?;
        }
        let adds = f.new_file.is_some() || matches!(f.old_path, Some(None));
        let deletes = f.deleted.is_some() || matches!(f.new_path, Some(None));
        let now = current(&path)?;
        if let Some(b) = &now {
            check_mode(&path, b.mode)?;
        }
        let old = match (&now, adds) {
            (Some(_), true) => return Err(bad(format!("{path} already exists"))),
            (None, false) => return Err(bad(format!("{path} does not exist"))),
            (None, true) => Vec::new(),
            (Some(b), false) => b.bytes.clone(),
        };
        if let Some(id) = &f.old_id
            && !id_matches(id, &old)
        {
            return Err(bad(format!(
                "{path} is not the version the patch was made against"
            )));
        }
        let new = match &f.binary {
            Some(Binary::Literal(bytes)) => bytes.clone(),
            Some(Binary::Delta(d)) => apply_delta(&old, d)?,
            None if f.hunks.is_empty() => old.clone(),
            None => apply_hunks(&path, &old, &f.hunks)?,
        };
        if deletes {
            if !new.is_empty() && f.binary.is_none() {
                return Err(bad(format!("{path}: a deletion leaves lines behind")));
            }
            out.push(Applied { path, after: None });
            continue;
        }
        if let Some(id) = &f.new_id
            && !id_matches(id, &new)
        {
            return Err(bad(format!(
                "{path}: the result is not the blob the patch names"
            )));
        }
        let mode = f
            .new_file
            .or(f.new_mode)
            .or(now.as_ref().map(|b| b.mode))
            .unwrap_or(FILE);
        out.push(Applied {
            path,
            after: Some(Blob { mode, bytes: new }),
        });
    }
    Ok(out)
}

fn apply_hunks(path: &str, old: &[u8], hunks: &[Hunk]) -> Result<Vec<u8>> {
    let old_lines: Vec<&[u8]> = old.split_inclusive(|b| *b == b'\n').collect();
    let mismatch = |n: usize| {
        bad(format!(
            "{path}: hunk {n} does not match the file (the patch was made against other content)"
        ))
    };
    let mut out = Vec::with_capacity(old.len());
    let mut pos = 0;
    for (n, h) in hunks.iter().enumerate() {
        let start = match (h.old_start, h.old_len) {
            (0, 0) => 0,
            (0, _) => return Err(mismatch(n + 1)),
            (s, 0) => s,
            (s, _) => s - 1,
        };
        if start < pos || start > old_lines.len() {
            return Err(mismatch(n + 1));
        }
        for l in &old_lines[pos..start] {
            out.extend_from_slice(l);
        }
        pos = start;
        for (kind, content) in &h.lines {
            match kind {
                Kind::Context | Kind::Removed => {
                    if old_lines.get(pos).copied() != Some(&content[..]) {
                        return Err(mismatch(n + 1));
                    }
                    if matches!(kind, Kind::Context) {
                        out.extend_from_slice(content);
                    }
                    pos += 1;
                }
                Kind::Added => out.extend_from_slice(content),
            }
        }
    }
    for l in &old_lines[pos..] {
        out.extend_from_slice(l);
    }
    Ok(out)
}
