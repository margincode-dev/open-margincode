//! Plan 12 Task 5: a repo's dependencies are built once. When a task's build output is reclaimed
//! (merge, abandon), the units of its `target/` that come from outside the repo (registry and git
//! dependencies, their build scripts and those scripts' output) go into the repo's deps cache,
//! `<ws_root>/<repo>/.cache/deps-target/`; a new task's `target/` starts with them, so cargo
//! compiles only the repo's own crates. Measured on margincode's own workspace: 2.4 GB and 38 s for a
//! full `cargo test` build from nothing, 1.2 GB and 15 s seeded, 0 dependencies recompiled.
//!
//! What may be shared, and how, follows from what cargo and rustc do to these files (checked with
//! cargo 1.97 by rebuilding seeded units and watching the cache's inodes):
//! - rustc writes `*.rlib`, `*.rmeta` and a proc-macro's `*.so` to a new file and renames it over
//!   the old one, so a rebuild in one workspace leaves another's link alone. These are hard
//!   links: one copy on disk for every task (1.3 of the 1.4 GB).
//! - rustc rewrites dep-info (`deps/*.d`) and cargo its fingerprints (`.fingerprint/`) in place,
//!   and a build script may write into its `out/` dir: through a shared inode, one task's rebuild
//!   would change the cache under every other. Those are copied (about 100 MB per task).
//! - A build script's `output` names its out dir by the absolute path of the target dir it ran
//!   in; cargo re-points it to the current one (it keeps the old path in `root-output`), so
//!   nothing is rewritten and no workspace ever refers to the cache's paths.
//! - The repo's own crates never enter the cache: cargo hashes a path package's location relative
//!   to the workspace root, so another workspace would take their artifacts as fresh (dogfood run
//!   1's shared target dir ran another task's test binary that way). A unit is shared only when
//!   its dep-info lists sources from cargo's registry or git checkouts and nothing else but files
//!   it generated itself; a build-script run has no dep-info and goes with its script.
//! - Executables (test binaries, bins, and kinds other than lib and build script) are never
//!   shared.
//!
//! The cache only grows (an entry already there is kept: same name, same inputs) until
//! `av gc --build-cache` removes the whole `.cache` dir; workspaces keep their links. Everything
//! here is file operations: the daemon still never runs cargo.

use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

/// The deps cache's directory under `<ws_root>/<repo>/.cache/`.
pub(crate) const DIR: &str = "deps-target";

/// Artifacts rustc replaces rather than rewrites: hard-linked. Everything else is copied.
const LINKED: &[&str] = &["rlib", "rmeta", "so", "dylib", "dll"];

/// What a harvest or a seed placed.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Placed {
    pub linked: u64,
    pub copied: u64,
    /// Bytes of the files placed (linked or copied).
    pub bytes: u64,
}

impl Placed {
    fn add(&mut self, other: Placed) {
        self.linked += other.linked;
        self.copied += other.copied;
        self.bytes += other.bytes;
    }
}

/// Puts the shareable units of `target` (a workspace's `target/`) into `cache`, keeping whatever
/// `cache` already has. Rlibs and the like are hard-linked (the workspace's copy is about to be
/// removed), the rest copied.
pub(crate) fn harvest(target: &Path, cache: &Path) -> io::Result<Placed> {
    let mut placed = Placed::default();
    for rel in profiles(target) {
        let (src, dst) = (target.join(&rel), cache.join(&rel));
        let units = shareable(&src);
        placed.add(place_units(&src, &dst, &units, false)?);
    }
    Ok(placed)
}

/// Gives a new workspace's `target` everything in `cache`: rlibs and the like hard-linked, the
/// rest copied. Stops at the first link that fails (another filesystem): what was placed is
/// valid, and cargo builds the rest.
pub(crate) fn seed(cache: &Path, target: &Path) -> io::Result<Placed> {
    let mut placed = Placed::default();
    for rel in profiles(cache) {
        let (src, dst) = (cache.join(&rel), target.join(&rel));
        let units = Units::everything(&src);
        placed.add(place_units(&src, &dst, &units, true)?);
    }
    Ok(placed)
}

/// The profile dirs under a target dir: `debug`, `release`, `<triple>/debug`, … (those holding a
/// `.fingerprint`), relative to it.
fn profiles(target: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for first in dirs(target) {
        let one = PathBuf::from(&first);
        if target.join(&one).join(".fingerprint").is_dir() {
            out.push(one);
            continue;
        }
        for second in dirs(&target.join(&first)) {
            let two = one.join(second);
            if target.join(&two).join(".fingerprint").is_dir() {
                out.push(two);
            }
        }
    }
    out
}

/// The names of the plain directories in `dir` (no symlinks), sorted.
fn dirs(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .flatten()
                .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
                .filter_map(|e| e.file_name().to_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names
}

fn files(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .flatten()
                .filter(|e| e.file_type().is_ok_and(|t| t.is_file()))
                .filter_map(|e| e.file_name().to_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names
}

/// The entries of one profile dir to place: file names in `deps/`, dir names in `.fingerprint/`
/// and in `build/`.
#[derive(Debug, Default)]
struct Units {
    deps: Vec<String>,
    fingerprints: Vec<String>,
    builds: Vec<String>,
}

impl Units {
    fn everything(profile: &Path) -> Units {
        Units {
            deps: files(&profile.join("deps")),
            fingerprints: dirs(&profile.join(".fingerprint")),
            builds: dirs(&profile.join("build")),
        }
    }
}

/// `<name>-<16 hex>` → (`name`, hash).
fn unit(name: &str) -> Option<(&str, &str)> {
    let (stem, hash) = name.rsplit_once('-')?;
    (hash.len() == 16 && hash.bytes().all(|b| b.is_ascii_hexdigit()) && !stem.is_empty())
        .then_some((stem, hash))
}

/// A `deps/` file → (crate, hash, extension); `None` for files without an extension
/// (executables), which are never shared.
fn deps_file(name: &str) -> Option<(&str, &str, &str)> {
    let (stem, ext) = name.rsplit_once('.')?;
    let stem = match ext {
        "rlib" | "rmeta" | "so" | "dylib" => stem.strip_prefix("lib")?,
        _ => stem,
    };
    let (krate, hash) = unit(stem)?;
    Some((krate, hash, ext))
}

/// Whether a dep-info file describes a unit built from outside the repo: it lists at least one
/// source in cargo's registry or git checkouts, and nothing else but files under a build
/// script's `out` dir and artifacts.
pub(crate) fn external(dep_info: &str) -> bool {
    let mut any = false;
    for line in dep_info.lines() {
        if line.starts_with('#') {
            continue;
        }
        // `outputs: inputs` or, for each input, `input:`; a `\ ` is a space inside a path
        let line = line.replace("\\ ", "\u{0}");
        let inputs = match rule_split(&line) {
            Some((_, inputs)) if !inputs.trim().is_empty() => inputs.to_owned(),
            Some((target, _)) => target.to_owned(),
            None => line.clone(),
        };
        for token in inputs.split_whitespace() {
            let path = token.replace('\\', "/");
            if path.contains("/registry/src/") || path.contains("/git/checkouts/") {
                any = true;
            } else if !(path.contains("/build/") && path.contains("/out/")) && !artifact(&path) {
                return false;
            }
        }
    }
    any
}

/// Splits `outputs: inputs` at the colon that ends the targets (a Windows drive letter's colon
/// is followed by a separator, not a space or the end).
fn rule_split(line: &str) -> Option<(&str, &str)> {
    let bytes = line.as_bytes();
    for (i, b) in bytes.iter().enumerate() {
        if *b == b':' && bytes.get(i + 1).is_none_or(|n| *n == b' ') {
            return Some((&line[..i], &line[i + 1..]));
        }
    }
    None
}

fn artifact(path: &str) -> bool {
    [".rlib", ".rmeta", ".so", ".dylib", ".dll"]
        .iter()
        .any(|ext| path.ends_with(ext))
}

/// The entries of `profile` that may be shared.
fn shareable(profile: &Path) -> Units {
    let read = |p: PathBuf| std::fs::read_to_string(p).ok();
    // deps/: a (crate, hash) group goes when its dep-info is external
    let names = files(&profile.join("deps"));
    let mut groups: BTreeMap<(String, String), bool> = BTreeMap::new();
    for name in &names {
        if let Some((krate, hash, "d")) = deps_file(name) {
            let ok = read(profile.join("deps").join(name)).is_some_and(|t| external(&t));
            groups.insert((krate.to_owned(), hash.to_owned()), ok);
        }
    }
    let shared_hashes: BTreeSet<&str> = groups
        .iter()
        .filter(|(_, ok)| **ok)
        .map(|((_, hash), _)| hash.as_str())
        .collect();
    let deps = names
        .iter()
        .filter(|name| {
            deps_file(name).is_some_and(|(krate, hash, _)| {
                groups
                    .get(&(krate.to_owned(), hash.to_owned()))
                    .copied()
                    .unwrap_or(false)
            })
        })
        .cloned()
        .collect();
    // build/: a script's dir goes when its dep-info is external; a run's (no dep-info) when its
    // package's scripts are all external
    let build_dirs = dirs(&profile.join("build"));
    let mut script: BTreeMap<String, bool> = BTreeMap::new();
    let mut builds_ok: BTreeMap<String, Option<bool>> = BTreeMap::new();
    for name in &build_dirs {
        let Some((pkg, _)) = unit(name) else { continue };
        let dir = profile.join("build").join(name);
        let infos: Vec<String> = files(&dir)
            .into_iter()
            .filter(|f| f.ends_with(".d"))
            .collect();
        if infos.is_empty() {
            builds_ok.insert(name.clone(), None);
            continue;
        }
        let ok = infos
            .iter()
            .all(|f| read(dir.join(f)).is_some_and(|t| external(&t)));
        let entry = script.entry(pkg.to_owned()).or_insert(true);
        *entry &= ok;
        builds_ok.insert(name.clone(), Some(ok));
    }
    let builds: Vec<String> = builds_ok
        .iter()
        .filter(|(name, ok)| match ok {
            Some(ok) => *ok,
            None => unit(name).is_some_and(|(pkg, _)| script.get(pkg) == Some(&true)),
        })
        .map(|(name, _)| name.clone())
        .collect();
    // .fingerprint/: a unit dir goes when it is a lib or build-script unit whose artifacts go
    let shared_builds: BTreeSet<&str> = builds.iter().map(String::as_str).collect();
    let fingerprints = dirs(&profile.join(".fingerprint"))
        .into_iter()
        .filter(|name| {
            let Some((_, hash)) = unit(name) else {
                return false;
            };
            let kinds_ok = files(&profile.join(".fingerprint").join(name))
                .iter()
                .all(|f| shareable_kind(f));
            kinds_ok && (shared_hashes.contains(hash) || shared_builds.contains(name.as_str()))
        })
        .collect();
    Units {
        deps,
        fingerprints,
        builds,
    }
}

/// A fingerprint file of a lib or build-script unit (never a test, bin, example or bench).
fn shareable_kind(file: &str) -> bool {
    if file == "invoked.timestamp" {
        return true;
    }
    let kind = file.strip_suffix(".json").unwrap_or(file);
    let kind = kind
        .strip_prefix("dep-")
        .or_else(|| kind.strip_prefix("output-"))
        .unwrap_or(kind);
    kind.starts_with("lib-")
        || kind.starts_with("build-script-")
        || kind.starts_with("run-build-script-")
}

/// Places `units` of the profile dir `src` into `dst`, keeping what `dst` has. With `strict`, a
/// failed hard link is an error (seeding); without, the file is skipped (harvesting).
fn place_units(src: &Path, dst: &Path, units: &Units, strict: bool) -> io::Result<Placed> {
    let mut placed = Placed::default();
    if !units.deps.is_empty() {
        std::fs::create_dir_all(dst.join("deps"))?;
    }
    for name in &units.deps {
        let (from, to) = (src.join("deps").join(name), dst.join("deps").join(name));
        let linked = deps_file(name).is_some_and(|(_, _, ext)| LINKED.contains(&ext));
        if linked {
            match std::fs::hard_link(&from, &to) {
                Ok(()) => {
                    if !strict {
                        // the cache keeps whole seconds (see `copy_file`); unrounded, the
                        // units above this one would rebuild, nothing worse
                        if let Err(e) = to_whole_second(&to) {
                            tracing::debug!(file = %to.display(), error = %e, "time not rounded");
                        }
                    }
                    placed.linked += 1;
                    placed.bytes += len(&to);
                }
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
                Err(e) if strict => return Err(e),
                Err(e) => tracing::debug!(file = %from.display(), error = %e, "not linked"),
            }
        } else if copy_new(&from, &to)? {
            placed.copied += 1;
            placed.bytes += len(&to);
        }
    }
    for (kind, names) in [
        (".fingerprint", &units.fingerprints),
        ("build", &units.builds),
    ] {
        for name in names.iter() {
            placed.add(copy_dir_new(
                &src.join(kind).join(name),
                &dst.join(kind).join(name),
            )?);
        }
    }
    Ok(placed)
}

fn len(p: &Path) -> u64 {
    std::fs::symlink_metadata(p).map(|m| m.len()).unwrap_or(0)
}

static TMP: AtomicU64 = AtomicU64::new(0);

/// A name beside `p` no other writer uses.
fn temp_beside(p: &Path) -> PathBuf {
    let n = TMP.fetch_add(1, Ordering::Relaxed);
    let name = p
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    p.with_file_name(format!(".{name}.av-{}-{n}", std::process::id()))
}

/// Copies the file `from` to `to` unless `to` exists, through a temporary name, so a reader
/// never sees half a file. Says whether it copied.
fn copy_new(from: &Path, to: &Path) -> io::Result<bool> {
    if std::fs::symlink_metadata(to).is_ok() {
        return Ok(false);
    }
    let tmp = temp_beside(to);
    copy_file(from, &tmp)?;
    if std::fs::symlink_metadata(to).is_ok() {
        std::fs::remove_file(&tmp)?;
        return Ok(false);
    }
    std::fs::rename(&tmp, to)?;
    Ok(true)
}

/// Copies the directory `from` to `to` unless `to` exists, built under a temporary name and
/// renamed into place whole. Symlinks and other special files are skipped.
fn copy_dir_new(from: &Path, to: &Path) -> io::Result<Placed> {
    if std::fs::symlink_metadata(to).is_ok() {
        return Ok(Placed::default());
    }
    if let Some(parent) = to.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = temp_beside(to);
    let placed = copy_tree(from, &tmp);
    let placed = match placed {
        Ok(p) => p,
        Err(e) => {
            let _ = std::fs::remove_dir_all(&tmp);
            return Err(e);
        }
    };
    if std::fs::rename(&tmp, to).is_err() {
        // another harvest got there first: theirs is as good
        let _ = std::fs::remove_dir_all(&tmp);
        return Ok(Placed::default());
    }
    Ok(placed)
}

fn copy_tree(from: &Path, to: &Path) -> io::Result<Placed> {
    let mut placed = Placed::default();
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)?.flatten() {
        let kind = entry.file_type()?;
        let dst = to.join(entry.file_name());
        if kind.is_dir() {
            placed.add(copy_tree(&entry.path(), &dst)?);
        } else if kind.is_file() {
            placed.bytes += copy_file(&entry.path(), &dst)?;
            placed.copied += 1;
        }
    }
    Ok(placed)
}

/// `std::fs::copy` keeping the times, rounded down to the second: cargo judges a unit stale when
/// a dependency's output is newer than its own, so a copy stamped "now" (a build script's
/// `output`, say) would make everything above it rebuild. Rounded because some filesystems
/// (Docker Desktop's mount of a Windows drive) drop the fraction of a time that is set but keep
/// that of a file they create: a copy could then predate the library it was built after. Rounding
/// every file of the cache the same way keeps their order, and cargo takes equal times as fresh.
fn copy_file(from: &Path, to: &Path) -> io::Result<u64> {
    let bytes = std::fs::copy(from, to)?;
    let meta = std::fs::metadata(from)?;
    filetime::set_file_times(
        to,
        whole_second(filetime::FileTime::from_last_access_time(&meta)),
        whole_second(filetime::FileTime::from_last_modification_time(&meta)),
    )?;
    Ok(bytes)
}

fn whole_second(t: filetime::FileTime) -> filetime::FileTime {
    filetime::FileTime::from_unix_time(t.unix_seconds(), 0)
}

/// Rounds a file's modification time down to the second, as `copy_file` does.
fn to_whole_second(p: &Path) -> io::Result<()> {
    let meta = std::fs::metadata(p)?;
    filetime::set_file_mtime(
        p,
        whole_second(filetime::FileTime::from_last_modification_time(&meta)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const REG: &str = "/usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f";

    /// Cargo compares a unit's outputs' mtimes with its dependencies': a copy stamped "now"
    /// makes everything that depends on it stale (a live run rebuilt 252 of 263 crates so).
    #[test]
    fn copies_keep_the_modification_time() {
        let dir = tempfile::tempdir().unwrap();
        let old = filetime::FileTime::from_unix_time(1_700_000_000, 0);
        let src = dir.path().join("src");
        std::fs::create_dir_all(src.join("out")).unwrap();
        for f in ["output", "out/gen.rs"] {
            std::fs::write(src.join(f), b"x").unwrap();
            filetime::set_file_times(src.join(f), old, old).unwrap();
        }
        let mtime = |p: &Path| {
            filetime::FileTime::from_last_modification_time(&std::fs::metadata(p).unwrap())
        };
        assert!(copy_new(&src.join("output"), &dir.path().join("one")).unwrap());
        assert_eq!(mtime(&dir.path().join("one")), old);
        copy_dir_new(&src, &dir.path().join("tree")).unwrap();
        assert_eq!(mtime(&dir.path().join("tree/output")), old);
        assert_eq!(mtime(&dir.path().join("tree/out/gen.rs")), old);
    }

    #[test]
    fn the_cache_keeps_whole_seconds_so_copies_never_predate_links() {
        // Some filesystems (Docker Desktop's mount of a Windows drive) keep a new file's
        // sub-second time but drop it when a time is set: a copy then lands up to a second
        // before the library it was built after, and cargo rebuilds everything above it.
        let dir = tempfile::tempdir().unwrap();
        let prof = dir.path().join("target/debug");
        let hash = "a266d54d01e4376a";
        let fp = prof.join(format!(".fingerprint/serde-{hash}"));
        std::fs::create_dir_all(&fp).unwrap();
        std::fs::create_dir_all(prof.join("deps")).unwrap();
        let rlib = prof.join(format!("deps/libserde-{hash}.rlib"));
        let dinfo = prof.join(format!("deps/serde-{hash}.d"));
        std::fs::write(&rlib, b"rlib").unwrap();
        std::fs::write(
            &dinfo,
            format!(
                "{}: {REG}/serde-1.0/src/lib.rs\n\n{REG}/serde-1.0/src/lib.rs:\n",
                rlib.display()
            ),
        )
        .unwrap();
        std::fs::write(fp.join("lib-serde"), b"fp").unwrap();
        std::fs::write(fp.join("lib-serde.json"), b"{}").unwrap();
        let at = |s, ns| filetime::FileTime::from_unix_time(s, ns);
        filetime::set_file_mtime(&rlib, at(1_700_000_000, 242_000_000)).unwrap();
        for f in [
            dinfo.clone(),
            fp.join("lib-serde"),
            fp.join("lib-serde.json"),
        ] {
            filetime::set_file_mtime(f, at(1_700_000_000, 900_000_000)).unwrap();
        }
        let cache = dir.path().join("cache");
        let placed = harvest(&dir.path().join("target"), &cache).unwrap();
        assert_eq!((placed.linked, placed.copied), (1, 3), "{placed:?}");
        let mtime = |p: PathBuf| {
            filetime::FileTime::from_last_modification_time(&std::fs::metadata(p).unwrap())
        };
        let c = cache.join("debug");
        for f in [
            c.join(format!("deps/libserde-{hash}.rlib")),
            c.join(format!("deps/serde-{hash}.d")),
            c.join(format!(".fingerprint/serde-{hash}/lib-serde")),
            c.join(format!(".fingerprint/serde-{hash}/lib-serde.json")),
        ] {
            assert_eq!(mtime(f.clone()), at(1_700_000_000, 0), "{}", f.display());
        }
    }

    #[test]
    fn a_registry_crate_is_external() {
        let d = format!(
            "/ws/r/s/target/debug/deps/libserde-a266d54d01e4376a.rlib: {REG}/serde-1.0/src/lib.rs\n\n{REG}/serde-1.0/src/lib.rs:\n"
        );
        assert!(external(&d));
    }

    #[test]
    fn a_crate_of_the_repo_is_not() {
        let d = "/ws/r/s/target/debug/deps/libcore-0b1c2d3e4f5a6b7c.rlib: /ws/r/s/crates/core/src/lib.rs\n\n/ws/r/s/crates/core/src/lib.rs:\n";
        assert!(!external(d));
    }

    #[test]
    fn one_source_of_the_repo_makes_a_crate_not_external() {
        let d = format!("/t/deps/libx-0b1c2d3e4f5a6b7c.rlib: {REG}/x-1/src/lib.rs /ws/r/s/x.rs\n");
        assert!(!external(&d));
    }

    #[test]
    fn git_checkouts_generated_files_and_env_lines_are_fine() {
        let d = "/t/deps/liby-0b1c2d3e4f5a6b7c.rlib: /home/u/.cargo/git/checkouts/y-1a2b/abc/src/lib.rs /ws/r/s/target/debug/build/y-77a0bc3e1d2f4a55/out/gen.rs\n\n# env-dep:OUT_DIR=/ws/r/s/target/debug/build/y-77a0bc3e1d2f4a55/out\n";
        assert!(external(d));
    }

    #[test]
    fn windows_paths_and_escaped_spaces_are_read() {
        let d = "D:\\ws\\r\\s\\target\\debug\\deps\\libz-0b1c2d3e4f5a6b7c.rlib: C:\\Users\\A\\ B\\.cargo\\registry\\src\\index\\z-1\\src\\lib.rs\n";
        assert!(external(d));
        let d = "D:\\ws\\r\\s\\target\\debug\\deps\\libz-0b1c2d3e4f5a6b7c.rlib: D:\\ws\\r\\s\\src\\lib.rs\n";
        assert!(!external(d));
    }

    #[test]
    fn dep_info_without_any_source_is_not_external() {
        assert!(!external(""));
        assert!(!external("/t/deps/liba-0b1c2d3e4f5a6b7c.rlib:\n"));
    }

    #[test]
    fn deps_file_names() {
        assert_eq!(
            deps_file("libserde-a266d54d01e4376a.rlib"),
            Some(("serde", "a266d54d01e4376a", "rlib"))
        );
        assert_eq!(
            deps_file("liblibc-a266d54d01e4376a.rmeta"),
            Some(("libc", "a266d54d01e4376a", "rmeta"))
        );
        assert_eq!(
            deps_file("libc-a266d54d01e4376a.d"),
            Some(("libc", "a266d54d01e4376a", "d"))
        );
        assert_eq!(deps_file("it-a266d54d01e4376a"), None);
        assert_eq!(deps_file("libserde-xyz.rlib"), None);
    }

    #[test]
    fn only_lib_and_build_script_fingerprints_are_shareable() {
        for ok in [
            "lib-serde",
            "lib-serde.json",
            "dep-lib-serde",
            "output-lib-serde",
            "invoked.timestamp",
            "build-script-build-script-build",
            "run-build-script-build-script-build.json",
        ] {
            assert!(shareable_kind(ok), "{ok}");
        }
        for no in [
            "test-lib-serde",
            "bin-av",
            "test-integration-test-it",
            "example-x",
            "bench-y",
        ] {
            assert!(!shareable_kind(no), "{no}");
        }
    }
}
