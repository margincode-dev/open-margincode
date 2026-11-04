//! `{crates}` in a convention's check command (Plan 12 Task 2): the packages a change can affect,
//! as `-p a -p b`, from `cargo metadata` run in the workspace. Anything this cannot place falls
//! back to `--workspace`, so a wrong guess costs a full build, never a missed dependant.

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};

use serde_json::Value;

pub const PLACEHOLDER: &str = "{crates}";

/// What `{crates}` becomes.
#[derive(Debug, PartialEq, Eq)]
pub enum Scope {
    Workspace,
    Packages(Vec<String>),
}

impl Scope {
    pub fn args(&self) -> String {
        match self {
            Scope::Workspace => "--workspace".into(),
            Scope::Packages(p) => p
                .iter()
                .map(|n| format!("-p {n}"))
                .collect::<Vec<_>>()
                .join(" "),
        }
    }
}

/// `cmd` with every `{crates}` replaced, running `cargo metadata` in `root` only when it has one.
pub fn expand(cmd: &str, root: &Path, changed: &[String]) -> String {
    if !cmd.contains(PLACEHOLDER) {
        return cmd.to_owned();
    }
    cmd.replace(PLACEHOLDER, &scope_in(root, changed).args())
}

/// The scope of `changed` (paths relative to `root`) in the cargo workspace at `root`.
pub fn scope_in(root: &Path, changed: &[String]) -> Scope {
    let out = std::process::Command::new("cargo")
        .args([
            "metadata",
            "--no-deps",
            "--offline",
            "--format-version",
            "1",
        ])
        .current_dir(root)
        .stderr(std::process::Stdio::null())
        .output();
    let Ok(out) = out else {
        return Scope::Workspace;
    };
    if !out.status.success() {
        return Scope::Workspace;
    }
    let Ok(meta) = serde_json::from_slice::<Value>(&out.stdout) else {
        return Scope::Workspace;
    };
    let Some(prefix) = workspace_prefix(root, &meta) else {
        return Scope::Workspace;
    };
    scope(&meta, &prefix, changed)
}

/// Where the cargo workspace root sits inside `root` (`""` when it is `root`); `None` when it is
/// not inside it.
fn workspace_prefix(root: &Path, meta: &Value) -> Option<String> {
    let ws = PathBuf::from(meta["workspace_root"].as_str()?);
    let ws = ws.canonicalize().ok()?;
    let root = root.canonicalize().ok()?;
    let rel = ws.strip_prefix(&root).ok()?;
    Some(slashed(rel))
}

fn slashed(p: &Path) -> String {
    p.components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/")
}

/// The pure rule: `meta` is `cargo metadata --no-deps` output, `prefix` the cargo workspace
/// root's place in the repo, `changed` repo-relative paths.
pub fn scope(meta: &Value, prefix: &str, changed: &[String]) -> Scope {
    let Some(ws_root) = meta["workspace_root"].as_str() else {
        return Scope::Workspace;
    };
    let ws_root = Path::new(ws_root);
    let members: BTreeSet<&str> = meta["workspace_members"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect();
    // name -> dir relative to the cargo workspace root, and name -> names it depends on by
    // path (every kind)
    let mut dirs: Vec<(String, String)> = vec![];
    let mut deps: HashMap<String, Vec<String>> = HashMap::new();
    for p in meta["packages"].as_array().into_iter().flatten() {
        let (Some(id), Some(name), Some(manifest)) = (
            p["id"].as_str(),
            p["name"].as_str(),
            p["manifest_path"].as_str(),
        ) else {
            return Scope::Workspace;
        };
        if !members.contains(id) {
            continue;
        }
        let Some(dir) = Path::new(manifest).parent() else {
            return Scope::Workspace;
        };
        let Ok(rel) = dir.strip_prefix(ws_root) else {
            return Scope::Workspace;
        };
        dirs.push((name.to_owned(), slashed(rel)));
        let on: Vec<String> = p["dependencies"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|d| d["path"].is_string())
            .filter_map(|d| d["name"].as_str().map(str::to_owned))
            .collect();
        deps.insert(name.to_owned(), on);
    }
    let mut touched = BTreeSet::new();
    for path in changed {
        let rest = if prefix.is_empty() {
            Some(path.as_str())
        } else {
            path.strip_prefix(prefix).and_then(|r| r.strip_prefix('/'))
        };
        let Some(rest) = rest else {
            return Scope::Workspace; // outside the cargo workspace
        };
        let owner = dirs
            .iter()
            .filter(|(_, d)| {
                if d.is_empty() {
                    root_package_owns(rest)
                } else {
                    rest == d || rest.starts_with(&format!("{d}/"))
                }
            })
            .max_by_key(|(_, d)| d.len());
        match owner {
            Some((name, _)) => {
                touched.insert(name.clone());
            }
            None => return Scope::Workspace,
        }
    }
    if touched.is_empty() {
        return Scope::Workspace;
    }
    // every member that depends, transitively, on a touched one
    let mut chosen = touched;
    loop {
        let before = chosen.len();
        for (name, on) in &deps {
            if on.iter().any(|d| chosen.contains(d)) {
                chosen.insert(name.clone());
            }
        }
        if chosen.len() == before {
            break;
        }
    }
    Scope::Packages(chosen.into_iter().collect())
}

/// What a package at the workspace root owns: its sources, not the rest of the tree (fixture dirs
/// other members read, and `Cargo.toml`, `Cargo.lock`, `.cargo/`, which every member's build
/// reads).
fn root_package_owns(path: &str) -> bool {
    path == "build.rs"
        || ["src/", "tests/", "benches/", "examples/"]
            .iter()
            .any(|d| path.starts_with(d))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// core <- store (normal), core <- testutil? no: testutil <- core (dev), app <- store (build).
    fn meta() -> Value {
        let pkg = |name: &str, deps: Value| {
            json!({"id": format!("path+file:///w/crates/{name}#0.1.0"), "name": name,
                   "manifest_path": format!("/w/crates/{name}/Cargo.toml"), "dependencies": deps})
        };
        json!({
            "workspace_root": "/w",
            "workspace_members": ["path+file:///w/crates/core#0.1.0",
                                  "path+file:///w/crates/store#0.1.0",
                                  "path+file:///w/crates/testutil#0.1.0",
                                  "path+file:///w/crates/app#0.1.0",
                                  "path+file:///w/crates/core-extra#0.1.0"],
            "packages": [
                pkg("core", json!([
                    {"name": "serde", "kind": null},
                    {"name": "testutil", "kind": "dev", "path": "/w/crates/testutil"}])),
                pkg("store", json!([
                    {"name": "core", "kind": null, "path": "/w/crates/core", "rename": "c"}])),
                pkg("testutil", json!([])),
                pkg("app", json!([{"name": "store", "kind": "build", "path": "/w/crates/store"}])),
                pkg("core-extra", json!([])),
            ]
        })
    }

    fn paths(p: &[&str]) -> Vec<String> {
        p.iter().map(|s| (*s).to_owned()).collect()
    }

    fn pkgs(p: &[&str]) -> Scope {
        Scope::Packages(paths(p))
    }

    #[test]
    fn a_change_reaches_its_dependants_through_every_kind_of_dependency() {
        let m = meta();
        assert_eq!(
            scope(&m, "", &paths(&["crates/app/src/lib.rs"])),
            pkgs(&["app"])
        );
        // store <- app (build)
        assert_eq!(
            scope(&m, "", &paths(&["crates/store/src/lib.rs"])),
            pkgs(&["app", "store"])
        );
        // core <- store (renamed, normal) <- app
        assert_eq!(
            scope(&m, "", &paths(&["crates/core/Cargo.toml"])),
            pkgs(&["app", "core", "store"])
        );
        // testutil <- core (dev) <- store <- app
        assert_eq!(
            scope(&m, "", &paths(&["crates/testutil/src/lib.rs"])),
            pkgs(&["app", "core", "store", "testutil"])
        );
    }

    #[test]
    fn a_directory_that_only_shares_a_prefix_is_another_package() {
        assert_eq!(
            scope(&meta(), "", &paths(&["crates/core-extra/src/lib.rs"])),
            pkgs(&["core-extra"])
        );
    }

    #[test]
    fn anything_outside_every_member_means_the_whole_workspace() {
        let m = meta();
        for p in [
            "Cargo.toml",
            "Cargo.lock",
            "rust-toolchain.toml",
            "bench/x.toml",
            "README.md",
        ] {
            assert_eq!(
                scope(&m, "", &paths(&["crates/app/src/lib.rs", p])),
                Scope::Workspace,
                "{p}"
            );
        }
        assert_eq!(scope(&m, "", &[]), Scope::Workspace, "nothing changed");
    }

    #[test]
    fn a_root_package_owns_its_sources_but_not_the_workspace_files() {
        let m = json!({
            "workspace_root": "/w",
            "workspace_members": ["path+file:///w#0.1.0", "path+file:///w/sub#0.1.0"],
            "packages": [
                {"id": "path+file:///w#0.1.0", "name": "root", "manifest_path": "/w/Cargo.toml",
                 "dependencies": [{"name": "sub", "kind": null, "path": "/w/sub"}]},
                {"id": "path+file:///w/sub#0.1.0", "name": "sub",
                 "manifest_path": "/w/sub/Cargo.toml", "dependencies": []},
            ]
        });
        assert_eq!(scope(&m, "", &paths(&["src/main.rs"])), pkgs(&["root"]));
        assert_eq!(
            scope(&m, "", &paths(&["sub/src/lib.rs"])),
            pkgs(&["root", "sub"])
        );
        assert_eq!(scope(&m, "", &paths(&["Cargo.lock"])), Scope::Workspace);
        assert_eq!(
            scope(&m, "", &paths(&[".cargo/config.toml"])),
            Scope::Workspace
        );
        // a fixture dir another member may read through `../`
        assert_eq!(
            scope(&m, "", &paths(&["fixtures/a.json"])),
            Scope::Workspace
        );
        assert_eq!(scope(&m, "", &paths(&["build.rs"])), pkgs(&["root"]));
        // the same under a repo subdirectory
        assert_eq!(
            scope(&m, "rust", &paths(&["rust/src/main.rs"])),
            pkgs(&["root"])
        );
        assert_eq!(
            scope(&m, "rust", &paths(&["rust/Cargo.lock"])),
            Scope::Workspace
        );
    }

    #[test]
    fn a_cargo_workspace_in_a_subdirectory_of_the_repo() {
        let m = meta();
        assert_eq!(
            scope(&m, "rust", &paths(&["rust/crates/store/src/lib.rs"])),
            pkgs(&["app", "store"])
        );
        assert_eq!(
            scope(&m, "rust", &paths(&["crates/store/src/lib.rs"])),
            Scope::Workspace,
            "not under the cargo workspace"
        );
    }

    #[test]
    fn args_are_the_cargo_flags() {
        assert_eq!(Scope::Workspace.args(), "--workspace");
        assert_eq!(pkgs(&["a", "b"]).args(), "-p a -p b");
    }

    /// Real `cargo metadata` on a two-crate workspace where `b` dev-depends on `a`.
    #[test]
    fn expand_runs_cargo_metadata_in_the_workspace() {
        if std::process::Command::new("cargo")
            .arg("--version")
            .output()
            .is_err()
        {
            return; // no cargo on this host: nothing to run the check with either
        }
        let dir = tempfile::tempdir().unwrap();
        let w = dir.path();
        let write = |p: &str, s: &str| {
            let p = w.join(p);
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(p, s).unwrap();
        };
        write(
            "Cargo.toml",
            "[workspace]\nmembers = [\"a\", \"b\"]\nresolver = \"2\"\n",
        );
        write(
            "a/Cargo.toml",
            "[package]\nname = \"a\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        );
        write("a/src/lib.rs", "");
        write(
            "b/Cargo.toml",
            "[package]\nname = \"b\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\
             [dev-dependencies]\na = { path = \"../a\" }\n",
        );
        write("b/src/lib.rs", "");
        let cmd = "cargo test {crates} --no-run";
        assert_eq!(
            expand(cmd, w, &paths(&["a/src/lib.rs"])),
            "cargo test -p a -p b --no-run"
        );
        assert_eq!(
            expand(cmd, w, &paths(&["b/src/lib.rs"])),
            "cargo test -p b --no-run"
        );
        assert_eq!(
            expand(cmd, w, &paths(&["Cargo.toml"])),
            "cargo test --workspace --no-run"
        );
        assert_eq!(expand("cargo test", w, &[]), "cargo test", "no placeholder");
        // not a cargo workspace at all
        let empty = tempfile::tempdir().unwrap();
        assert_eq!(
            expand(cmd, empty.path(), &paths(&["a/src/lib.rs"])),
            "cargo test --workspace --no-run"
        );
    }
}
