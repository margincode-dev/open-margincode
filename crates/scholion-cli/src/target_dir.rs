//! A task workspace's builds go to its own `target/` (Plan 12 Task 3). av's per-repo cargo
//! config sets no `target-dir`, so a `CARGO_TARGET_DIR` in the user's environment, or
//! `build.target-dir` in `~/.cargo/config.toml`, would put every workspace of a repo into one
//! dir, where `cargo test` in one workspace runs another task's test binary. `CARGO_TARGET_DIR`
//! outranks every config file, so setting it to the workspace's `target/` restores the rule; that
//! dir is the one the daemon seeds with dependencies and reclaims with the task.

use std::path::{Path, PathBuf};
use std::process::Command;

/// `<root>/target` when `root` is a cargo workspace (a root `Cargo.toml`, as the daemon's
/// `build_cache` decides), else `None`.
pub fn own(root: &Path) -> Option<PathBuf> {
    root.join("Cargo.toml")
        .is_file()
        .then(|| root.join("target"))
}

/// `cmd` with cargo pointed at `root`'s own `target/` when `root` is a cargo workspace. An
/// explicit `--target-dir` in the command still wins.
pub fn apply<'a>(root: &Path, cmd: &'a mut Command) -> &'a mut Command {
    if let Some(target) = own(root) {
        cmd.env("CARGO_TARGET_DIR", target)
            .env_remove("CARGO_BUILD_TARGET_DIR");
    }
    cmd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_a_cargo_workspace_gets_its_own_target() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(own(dir.path()), None);
        std::fs::write(dir.path().join("Cargo.toml"), "[workspace]\n").unwrap();
        assert_eq!(own(dir.path()), Some(dir.path().join("target")));
    }
}
