//! `margincode_vcs::patch` (Plan 14): git-format patches for remote checkpoints, checked against git
//! itself. `git diff --binary --full-index --no-renames` output must apply to the same files with
//! `patch::apply`, and what `patch::render` writes must apply with `git apply`.

use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

use margincode_core::AvError;
use margincode_vcs::patch::{self, Applied, Blob, EXECUTABLE, FILE, FileDiff};
use tempfile::TempDir;

fn git(dir: &Path, args: &[&str]) -> Vec<u8> {
    let out = Command::new("git")
        .current_dir(dir)
        .args([
            "-c",
            "user.name=t",
            "-c",
            "user.email=t@t",
            "-c",
            "commit.gpgsign=false",
            "-c",
            "core.autocrlf=false",
        ])
        .args(args)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    out.stdout
}

type Files = BTreeMap<String, Blob>;

fn blob(bytes: &[u8]) -> Blob {
    Blob {
        mode: FILE,
        bytes: bytes.to_vec(),
    }
}

fn write_files(dir: &Path, files: &Files) {
    for (path, b) in files {
        let p = dir.join(path);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(&p, &b.bytes).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            let mode = if b.mode == EXECUTABLE { 0o755 } else { 0o644 };
            std::fs::set_permissions(&p, std::fs::Permissions::from_mode(mode)).unwrap();
        }
    }
}

/// `files` after `applied`.
fn after(mut files: Files, applied: Vec<Applied>) -> Files {
    for a in applied {
        match a.after {
            Some(b) => {
                files.insert(a.path, b);
            }
            None => {
                files.remove(&a.path);
            }
        }
    }
    files
}

fn apply_to(files: &Files, p: &[u8]) -> margincode_core::Result<Files> {
    let applied = patch::apply(p, |path| Ok(files.get(path).cloned()))?;
    Ok(after(files.clone(), applied))
}

/// A binary file of `n` bytes with a NUL in it, varied by `seed`.
fn binary(n: usize, seed: u8) -> Vec<u8> {
    (0..n)
        .map(|i| (i as u8).wrapping_mul(31).wrapping_add(seed) ^ ((i / 7) as u8))
        .chain([0u8])
        .collect()
}

/// The old and new versions used by both directions: modified, added, deleted, executable,
/// no final newline, CRLF, non-UTF-8 text, empty files, binary (small edit: git sends a delta),
/// a new binary file, and a path git quotes.
fn versions() -> (Files, Files) {
    let mut old = Files::new();
    let long: String = (1..=40).map(|i| format!("line {i}\n")).collect();
    old.insert("app/long.txt".into(), blob(long.as_bytes()));
    old.insert("app/gone.txt".into(), blob(b"bye\n"));
    old.insert("run.sh".into(), blob(b"#!/bin/sh\necho hi\n"));
    old.insert("noeol.txt".into(), blob(b"a\nb"));
    old.insert("crlf.txt".into(), blob(b"one\r\ntwo\r\n"));
    old.insert("latin1.txt".into(), blob(b"caf\xe9\n"));
    old.insert("empty-gone".into(), blob(b""));
    old.insert("img.bin".into(), blob(&binary(4000, 1)));
    old.insert("sp ace/\"q\" \u{e9}.txt".into(), blob(b"x\n"));

    let mut new = old.clone();
    let long2 = long
        .replace("line 3\n", "line three\n")
        .replace("line 30\n", "line 30\nline 30.5\n");
    new.insert("app/long.txt".into(), blob(long2.as_bytes()));
    new.remove("app/gone.txt");
    new.insert(
        "run.sh".into(),
        Blob {
            mode: EXECUTABLE,
            bytes: b"#!/bin/sh\necho hi\n".to_vec(),
        },
    );
    new.insert("noeol.txt".into(), blob(b"a\nc"));
    new.insert("crlf.txt".into(), blob(b"one\r\n2\r\n"));
    new.insert("latin1.txt".into(), blob(b"caf\xe9 cr\xe8me\n"));
    new.remove("empty-gone");
    new.insert("empty-new".into(), blob(b""));
    let mut img = binary(4000, 1);
    img[100] ^= 0xff;
    new.insert("img.bin".into(), blob(&img));
    new.insert("new/logo.bin".into(), blob(&binary(300, 9)));
    new.insert("new/hello.py".into(), blob(b"print('hi')\n"));
    new.insert("sp ace/\"q\" \u{e9}.txt".into(), blob(b"y\n"));
    (old, new)
}

fn diffs(old: &Files, new: &Files) -> Vec<FileDiff> {
    let mut paths: Vec<&String> = old.keys().chain(new.keys()).collect();
    paths.sort();
    paths.dedup();
    paths
        .into_iter()
        .map(|p| FileDiff {
            path: p.clone(),
            before: old.get(p).cloned(),
            after: new.get(p).cloned(),
        })
        .collect()
}

#[test]
fn a_git_diff_applies_to_the_files_it_was_made_from() {
    let (old, new) = versions();
    let dir = TempDir::new().unwrap();
    git(dir.path(), &["init", "-q"]);
    write_files(dir.path(), &old);
    git(dir.path(), &["add", "-A"]);
    git(dir.path(), &["commit", "-q", "-m", "old"]);
    for p in old.keys() {
        std::fs::remove_file(dir.path().join(p)).unwrap();
    }
    write_files(dir.path(), &new);
    git(dir.path(), &["add", "-A"]);
    let p = git(
        dir.path(),
        &[
            "diff",
            "--cached",
            "--binary",
            "--full-index",
            "--no-renames",
            "--no-color",
            "--no-ext-diff",
            "--src-prefix=a/",
            "--dst-prefix=b/",
            "HEAD",
        ],
    );
    let text = String::from_utf8_lossy(&p);
    assert!(text.contains("GIT binary patch"), "{text}");
    assert!(
        text.contains("\ndelta "),
        "a small binary edit is a delta: {text}"
    );
    assert!(text.contains("\\ No newline at end of file"), "{text}");
    #[cfg(unix)]
    assert!(text.contains("new mode 100755"), "{text}");
    let got = apply_to(&old, &p).unwrap();
    #[cfg(not(unix))]
    let new = {
        let mut n = new;
        n.get_mut("run.sh").unwrap().mode = FILE;
        n
    };
    assert_eq!(got, new);
}

#[test]
fn a_rendered_patch_applies_with_git_and_with_apply() {
    let (old, new) = versions();
    let p = patch::render(&diffs(&old, &new));
    assert_eq!(apply_to(&old, &p).unwrap(), new);

    let dir = TempDir::new().unwrap();
    git(dir.path(), &["init", "-q"]);
    write_files(dir.path(), &old);
    git(dir.path(), &["add", "-A"]);
    git(dir.path(), &["commit", "-q", "-m", "old"]);
    let file = dir.path().join(".git/p.diff");
    std::fs::write(&file, &p).unwrap();
    git(dir.path(), &["apply", "--check", file.to_str().unwrap()]);
    git(dir.path(), &["apply", "--index", file.to_str().unwrap()]);
    for (path, b) in &new {
        let on_disk = std::fs::read(dir.path().join(path)).unwrap();
        assert_eq!(&on_disk, &b.bytes, "{path}");
    }
    for path in old.keys().filter(|p| !new.contains_key(*p)) {
        assert!(!dir.path().join(path).exists(), "{path} is deleted");
    }
    // git's tree of the result is the one git would write for `new` itself
    let tree = String::from_utf8(git(dir.path(), &["write-tree"])).unwrap();
    let fresh = TempDir::new().unwrap();
    git(fresh.path(), &["init", "-q"]);
    write_files(fresh.path(), &new);
    git(fresh.path(), &["add", "-A"]);
    let want = String::from_utf8(git(fresh.path(), &["write-tree"])).unwrap();
    assert_eq!(tree, want);
}

#[test]
fn a_patch_from_nothing_creates_every_file() {
    let (_, new) = versions();
    let p = patch::render(&diffs(&Files::new(), &new));
    assert_eq!(apply_to(&Files::new(), &p).unwrap(), new);
    let dir = TempDir::new().unwrap();
    git(dir.path(), &["init", "-q"]);
    let file = dir.path().join(".git/p.diff");
    std::fs::write(&file, &p).unwrap();
    git(dir.path(), &["apply", "--cached", file.to_str().unwrap()]);
    let tree = String::from_utf8(git(dir.path(), &["write-tree"])).unwrap();
    let fresh = TempDir::new().unwrap();
    git(fresh.path(), &["init", "-q"]);
    write_files(fresh.path(), &new);
    git(fresh.path(), &["add", "-A"]);
    assert_eq!(
        tree,
        String::from_utf8(git(fresh.path(), &["write-tree"])).unwrap()
    );
}

#[test]
fn equal_versions_render_nothing() {
    let (old, _) = versions();
    assert!(patch::render(&diffs(&old, &old)).is_empty());
    assert!(apply_to(&old, b"").unwrap() == old);
}

#[test]
fn git_blob_id_is_gits() {
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join("f"), b"hello\n").unwrap();
    let want = String::from_utf8(git(dir.path(), &["hash-object", "f"])).unwrap();
    assert_eq!(patch::git_blob_id(b"hello\n"), want.trim());
}

fn invalid(r: margincode_core::Result<Files>) -> String {
    match r {
        Err(AvError::Invalid(m)) => m,
        other => panic!("expected Invalid, got {other:?}"),
    }
}

fn add_file(path: &str) -> String {
    format!(
        "diff --git a/{path} b/{path}\nnew file mode 100644\n--- /dev/null\n+++ b/{path}\n@@ -0,0 +1 @@\n+x\n"
    )
}

#[test]
fn unsafe_paths_are_refused() {
    let files = Files::new();
    for path in [
        "../escape",
        "a/../../b",
        ".jj/repo/store",
        "x/.git/config",
        ".git/hooks/pre-commit",
        ".av-task",
        "./a",
        "a//b",
    ] {
        let m = invalid(apply_to(&files, add_file(path).as_bytes()));
        assert!(m.contains("path"), "{path}: {m}");
    }
    let abs = "diff --git a//etc/x b//etc/x\nnew file mode 100644\n--- /dev/null\n+++ b//etc/x\n@@ -0,0 +1 @@\n+x\n";
    invalid(apply_to(&files, abs.as_bytes()));
}

#[test]
fn renames_symlinks_and_bare_binary_notices_are_refused() {
    let mut files = Files::new();
    files.insert("a.txt".into(), blob(b"x\n"));
    let rename =
        "diff --git a/a.txt b/b.txt\nsimilarity index 100%\nrename from a.txt\nrename to b.txt\n";
    assert!(invalid(apply_to(&files, rename.as_bytes())).contains("--no-renames"));
    let link = "diff --git a/l b/l\nnew file mode 120000\n--- /dev/null\n+++ b/l\n@@ -0,0 +1 @@\n+/etc/passwd\n\\ No newline at end of file\n";
    assert!(invalid(apply_to(&files, link.as_bytes())).contains("symlink"));
    let bare = "diff --git a/a.txt b/a.txt\nindex 587be6b..0000000 100644\nBinary files a/a.txt and b/a.txt differ\n";
    assert!(invalid(apply_to(&files, bare.as_bytes())).contains("--binary"));
}

#[test]
fn a_patch_that_does_not_match_the_files_is_refused_whole() {
    let mut files = Files::new();
    files.insert("a.txt".into(), blob(b"one\ntwo\n"));
    files.insert("b.txt".into(), blob(b"keep\n"));
    // the first file's change is fine; the second's context is not what b.txt holds
    let p = "diff --git a/a.txt b/a.txt\n--- a/a.txt\n+++ b/a.txt\n@@ -1,2 +1,2 @@\n one\n-two\n+2\n\
             diff --git a/b.txt b/b.txt\n--- a/b.txt\n+++ b/b.txt\n@@ -1 +1 @@\n-other\n+new\n";
    let m = invalid(apply_to(&files, p.as_bytes()));
    assert!(m.contains("b.txt"), "{m}");
    // adding a file that exists, or changing one that does not
    invalid(apply_to(&files, add_file("a.txt").as_bytes()));
    let p = "diff --git a/zz b/zz\n--- a/zz\n+++ b/zz\n@@ -1 +1 @@\n-x\n+y\n";
    invalid(apply_to(&files, p.as_bytes()));
    // an index line names the blob the patch was made against
    let p = format!(
        "diff --git a/b.txt b/b.txt\nindex {}..{} 100644\n--- a/b.txt\n+++ b/b.txt\n@@ -1 +1 @@\n-keep\n+kept\n",
        patch::git_blob_id(b"something else\n"),
        patch::git_blob_id(b"kept\n"),
    );
    invalid(apply_to(&files, p.as_bytes()));
    // a path twice
    let p = format!("{}{}", add_file("n.txt"), add_file("n.txt"));
    invalid(apply_to(&files, p.as_bytes()));
}

#[test]
fn garbage_is_refused() {
    let files = Files::new();
    invalid(apply_to(&files, b"not a patch\n"));
    invalid(apply_to(
        &files,
        b"diff --git a/x b/x\nnew file mode 100644\n--- /dev/null\n+++ b/x\n@@ -0,0 +1,3 @@\n+only one\n",
    ));
}
