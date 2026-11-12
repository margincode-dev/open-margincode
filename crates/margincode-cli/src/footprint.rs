//! Plan 11: what a session leaves on disk outside any task workspace. `scan` finds the heavy
//! directories (build output, dependencies, caches) under some roots and measures them; `diff`
//! says which appeared or grew between two scans. Pure: no daemon, no hook input.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Directory names whose contents agents make in bulk. `__pycache__` is left out: small and
/// everywhere, it would only be noise.
pub const HEAVY: &[&str] = &[
    "target",
    "node_modules",
    ".venv",
    "venv",
    "dist",
    "build",
    ".cache",
    "worktrees",
    ".gradle",
    ".next",
];

/// How deep under a root a heavy directory is looked for (a root's children are depth 1).
pub const MAX_DEPTH: usize = 4;

/// Growth below this is not reported.
pub const MIN_BYTES: u64 = 1024 * 1024;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Scan {
    /// Heavy directories with their size in bytes, sorted by path.
    pub dirs: Vec<(PathBuf, u64)>,
    /// The deadline passed before the scan ended: some directories are missing or undercounted.
    pub partial: bool,
}

fn excluded(path: &Path, exclude: &[PathBuf]) -> bool {
    exclude.iter().any(|ex| path.starts_with(ex))
}

/// Every heavy directory under `roots` (to `MAX_DEPTH`, never through a symlink, never inside
/// `exclude`), each with its total size; a heavy directory is measured whole and not searched for
/// more. Stops at `deadline` with `partial` set.
pub fn scan(roots: &[PathBuf], exclude: &[PathBuf], deadline: Instant) -> Scan {
    let mut found = BTreeMap::new();
    let mut partial = false;
    for root in roots {
        visit(root, 0, exclude, deadline, &mut found, &mut partial);
    }
    Scan {
        dirs: found.into_iter().collect(),
        partial,
    }
}

fn visit(
    dir: &Path,
    depth: usize,
    exclude: &[PathBuf],
    deadline: Instant,
    found: &mut BTreeMap<PathBuf, u64>,
    partial: &mut bool,
) {
    if excluded(dir, exclude) {
        return;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut children: Vec<_> = entries
        .flatten()
        .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
        .map(|e| e.path())
        .collect();
    children.sort();
    for child in children {
        if Instant::now() >= deadline {
            *partial = true;
            return;
        }
        if excluded(&child, exclude) || found.contains_key(&child) {
            continue;
        }
        let heavy = child
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| HEAVY.contains(&n));
        if heavy {
            let bytes = measure(&child, exclude, deadline, partial);
            found.insert(child, bytes);
        } else if depth + 1 < MAX_DEPTH {
            visit(&child, depth + 1, exclude, deadline, found, partial);
        }
    }
}

/// Bytes of the files under `dir` (symlinks count as themselves), up to `deadline`.
fn measure(dir: &Path, exclude: &[PathBuf], deadline: Instant, partial: &mut bool) -> u64 {
    let mut total = 0;
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else {
            continue;
        };
        for entry in entries.flatten() {
            if Instant::now() >= deadline {
                *partial = true;
                return total;
            }
            let Ok(kind) = entry.file_type() else {
                continue;
            };
            if kind.is_dir() {
                let p = entry.path();
                if !excluded(&p, exclude) {
                    stack.push(p);
                }
            } else if let Ok(meta) = entry.metadata() {
                total += meta.len();
            }
        }
    }
    total
}

/// What appeared or grew from `before` to `after`: a new directory counts whole, a grown one by
/// its growth. Entries under `MIN_BYTES` are dropped; the largest come first.
pub fn diff(before: &[(PathBuf, u64)], after: &[(PathBuf, u64)]) -> Vec<(PathBuf, u64)> {
    let old: BTreeMap<&PathBuf, u64> = before.iter().map(|(p, b)| (p, *b)).collect();
    let mut added: Vec<(PathBuf, u64)> = after
        .iter()
        .filter_map(|(p, now)| {
            let grew = now.saturating_sub(old.get(p).copied().unwrap_or(0));
            (grew >= MIN_BYTES).then(|| (p.clone(), grew))
        })
        .collect();
    added.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    added
}

/// `12.3 GB`, `512 MB`, `40 KB`, `7 B` (binary units).
pub fn human(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    let b = bytes as f64;
    if b >= KB * KB * KB {
        format!("{:.1} GB", b / (KB * KB * KB))
    } else if b >= KB * KB {
        format!("{:.0} MB", b / (KB * KB))
    } else if b >= KB {
        format!("{:.0} KB", b / KB)
    } else {
        format!("{bytes} B")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn file(path: &Path, bytes: usize) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, vec![0u8; bytes]).unwrap();
    }

    fn later() -> Instant {
        Instant::now() + Duration::from_secs(30)
    }

    #[test]
    fn heavy_dirs_are_found_to_depth_four_and_measured_whole() {
        let root = tempfile::tempdir().unwrap();
        let r = root.path();
        file(&r.join("proj/target/debug/a"), 1000);
        file(&r.join("proj/target/debug/build/b"), 500); // inside a heavy dir: part of it
        file(&r.join("a/b/c/node_modules/x.js"), 300); // depth 4
        file(&r.join("a/b/c/d/target/deep"), 700); // depth 5: too deep
        file(&r.join("proj/src/main.rs"), 50); // not heavy
        file(&r.join("proj/__pycache__/m.pyc"), 900); // noise, not counted
        let s = scan(&[r.to_path_buf()], &[], later());
        assert!(!s.partial);
        assert_eq!(
            s.dirs,
            vec![
                (r.join("a/b/c/node_modules"), 300),
                (r.join("proj/target"), 1500),
            ]
        );
    }

    #[test]
    fn excluded_roots_are_skipped() {
        let root = tempfile::tempdir().unwrap();
        let r = root.path();
        file(&r.join("ws/demo/task/target/a"), 1000);
        file(&r.join("other/target/a"), 10);
        let s = scan(&[r.to_path_buf()], &[r.join("ws")], later());
        assert_eq!(s.dirs, vec![(r.join("other/target"), 10)]);
    }

    #[test]
    fn symlinks_are_never_followed() {
        let root = tempfile::tempdir().unwrap();
        let elsewhere = tempfile::tempdir().unwrap();
        file(&elsewhere.path().join("target/big"), 4096);
        let r = root.path();
        std::fs::create_dir_all(r.join("proj")).unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink(elsewhere.path(), r.join("proj/link")).unwrap();
        let s = scan(&[r.to_path_buf()], &[], later());
        assert!(s.dirs.is_empty(), "{:?}", s.dirs);
    }

    #[test]
    fn a_passed_deadline_marks_the_scan_partial() {
        let root = tempfile::tempdir().unwrap();
        file(&root.path().join("p/target/a"), 10);
        let s = scan(&[root.path().to_path_buf()], &[], Instant::now());
        assert!(s.partial);
    }

    #[test]
    fn diff_counts_new_dirs_whole_and_grown_dirs_by_growth() {
        const MB: u64 = 1024 * 1024;
        let before = vec![
            (PathBuf::from("/h/a/target"), 10 * MB),
            (PathBuf::from("/h/b/node_modules"), 5 * MB),
            (PathBuf::from("/h/c/.venv"), 3 * MB),
        ];
        let after = vec![
            (PathBuf::from("/h/a/target"), 30 * MB),      // grew 20
            (PathBuf::from("/h/b/node_modules"), 5 * MB), // same
            (PathBuf::from("/h/c/.venv"), MB),            // shrank
            (PathBuf::from("/h/d/dist"), 40 * MB),        // new
            (PathBuf::from("/h/e/build"), MB / 2),        // new but tiny
        ];
        assert_eq!(
            diff(&before, &after),
            vec![
                (PathBuf::from("/h/d/dist"), 40 * MB),
                (PathBuf::from("/h/a/target"), 20 * MB),
            ]
        );
    }

    #[test]
    fn sizes_read_like_people_write_them() {
        assert_eq!(human(512 * 1024 * 1024), "512 MB");
        assert_eq!(human(13_207_024_435), "12.3 GB");
        assert_eq!(human(2048), "2 KB");
        assert_eq!(human(7), "7 B");
    }
}
