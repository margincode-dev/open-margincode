//! `.av/sparse`: the path prefixes a repo keeps out of its task workspaces' files on disk. The
//! paths stay in every commit; jj's sparse patterns only decide what is written to disk.

use std::collections::BTreeSet;
use std::path::Path;

use margincode_core::{AvError, Result};

pub const FILE: &str = ".av/sparse";

/// One prefix per line (a directory or a file), `#` comments and blank lines skipped. A prefix
/// another one covers is dropped.
pub fn parse(text: &str) -> Result<Vec<String>> {
    let mut out = BTreeSet::new();
    for (n, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let p = line.trim_start_matches("./").trim_end_matches('/');
        let bad = p.is_empty()
            || p.starts_with('/')
            || p.contains('\\')
            || p.split('/').any(|c| c.is_empty() || c == "." || c == "..")
            || [".jj", ".git", ".av"].contains(&p.split('/').next().unwrap_or(""));
        if bad {
            return Err(AvError::Invalid(format!(
                "{FILE}:{}: {line:?} is not a path in the repo (and .av, .jj, .git cannot be left out)",
                n + 1
            )));
        }
        out.insert(p.to_owned());
    }
    let all: Vec<String> = out.into_iter().collect();
    Ok(all
        .iter()
        .filter(|p| !all.iter().any(|q| q != *p && under(p, q)))
        .cloned()
        .collect())
}

/// `path` is `prefix` or inside it.
pub fn under(path: &str, prefix: &str) -> bool {
    path == prefix
        || (path.len() > prefix.len()
            && path.starts_with(prefix)
            && path.as_bytes()[prefix.len()] == b'/')
}

fn join(dir: &str, name: &str) -> String {
    if dir.is_empty() {
        name.to_owned()
    } else {
        format!("{dir}/{name}")
    }
}

/// The include patterns that leave `excludes` off disk: each path of the tree or of `root` that
/// is not under an exclude, at its shallowest level; every file `root` already holds under an
/// exclude (an agent wrote it there, so a snapshot must see it); and `force` (conflicted paths,
/// whose markers the agent has to see). jj snapshots only what the patterns cover, so a path a
/// pattern misses is never recorded.
pub fn includes(
    excludes: &[String],
    tree_files: impl IntoIterator<Item = String>,
    force: impl IntoIterator<Item = String>,
    root: &Path,
) -> Result<Vec<String>> {
    let ancestors: BTreeSet<String> = excludes
        .iter()
        .flat_map(|e| {
            let parts: Vec<&str> = e.split('/').collect();
            (0..parts.len()).map(move |k| parts[..k].join("/"))
        })
        .collect();
    let mut out = BTreeSet::new();
    let shallowest = |path: &str| -> Option<String> {
        if excludes.iter().any(|e| under(path, e)) {
            return None;
        }
        let parts: Vec<&str> = path.split('/').collect();
        (1..=parts.len())
            .map(|k| parts[..k].join("/"))
            .find(|p| !ancestors.contains(p))
    };
    for f in tree_files {
        out.extend(shallowest(&f));
    }
    for dir in &ancestors {
        let entries = match std::fs::read_dir(root.join(dir)) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            entries => entries.map_err(verr)?,
        };
        for entry in entries {
            let name = name_of(entry, dir)?;
            let name = name.as_str();
            let p = join(dir, name);
            if dir.is_empty() && name == ".jj" {
                continue;
            }
            if excludes.contains(&p) {
                files_under(root, &p, &mut out)?;
            } else if !ancestors.contains(&p) {
                out.insert(p);
            }
        }
    }
    out.extend(force);
    Ok(out.into_iter().collect())
}

fn files_under(root: &Path, rel: &str, out: &mut BTreeSet<String>) -> Result<()> {
    let at = root.join(rel);
    let meta = std::fs::symlink_metadata(&at).map_err(verr)?;
    if !meta.is_dir() {
        out.insert(rel.to_owned());
        return Ok(());
    }
    for entry in std::fs::read_dir(&at).map_err(verr)? {
        let name = name_of(entry, rel)?;
        files_under(root, &join(rel, &name), out)?;
    }
    Ok(())
}

/// A name that is not UTF-8 is an error: no pattern could cover it, so it would go unsnapshotted.
fn name_of(entry: std::io::Result<std::fs::DirEntry>, dir: &str) -> Result<String> {
    let name = entry.map_err(verr)?.file_name();
    name.into_string().map_err(|n| {
        AvError::Vcs(format!(
            "{}: a file name that is not UTF-8 ({n:?}) cannot be snapshotted",
            if dir.is_empty() {
                "the workspace's root"
            } else {
                dir
            }
        ))
    })
}

fn verr<E: std::fmt::Display>(e: E) -> AvError {
    AvError::Vcs(e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_list_is_normalized_and_a_covered_prefix_dropped() {
        let got = parse("# results\nbench/results/\n./big\nbench/results/old\n\n").unwrap();
        assert_eq!(got, ["bench/results", "big"]);
        for bad in ["/abs", "a/../b", ".jj/x", ".av", "a\\b", "a//b"] {
            assert!(parse(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn includes_leave_out_the_excludes_and_keep_new_paths_on_disk() {
        let root = tempfile::TempDir::new().unwrap();
        let w = |rel: &str| {
            let p = root.path().join(rel);
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(p, "x").unwrap();
        };
        w("top.txt");
        w("bench/new.txt");
        w("bench/results/mine/README.md");
        std::fs::create_dir_all(root.path().join(".jj")).unwrap();
        let tree = [
            "a.txt",
            "pkg/b.py",
            "bench/keep.txt",
            "bench/results/r.txt",
            "big/x",
        ]
        .map(String::from);
        let excludes = ["bench/results".to_owned(), "big".to_owned()];
        let got = includes(&excludes, tree, ["big/c.txt".to_owned()], root.path()).unwrap();
        assert_eq!(
            got,
            [
                "a.txt",
                "bench/keep.txt",
                "bench/new.txt",
                "bench/results/mine/README.md",
                "big/c.txt",
                "pkg",
                "top.txt"
            ]
        );
    }
}
