//! Plan 12: lean builds. A repo whose trunk has a root `Cargo.toml` gets
//! `<ws_root>/<repo>/.cargo/config.toml`, which every task workspace of the repo inherits: no
//! incremental cache, line tables instead of full debug info for the repo's crates and none for
//! dependencies. Measured on margincode's own workspace, that is a fifth of the disk of a default
//! `cargo test` build (21 GB → 4.3 GB; dogfood run 1 paid 20 GB per task, 14 GB of it test
//! binaries that were 73% debug info, 4 GB incremental).
//!
//! The file sets no `target-dir`: each workspace builds into its own `target/` (cargo's default),
//! which goes with the workspace when the task is reclaimed. Plan 11's one target dir per repo
//! was unsafe: cargo names a path package's artifacts by its path relative to the workspace root,
//! so two workspaces of one repo produce the same names, and `cargo test` in one ran the other's
//! test binary (dogfood run 1, found twice). A warm target still serves the reviewer, who
//! validates in the owner's workspace.
//!
//! Cargo probes `.cargo/config.toml` in the working directory and in every parent; deeper files
//! win per key, and `CARGO_*` in the environment beats every file. So the repo's own
//! `.cargo/config.toml` (inside the workspace, deeper) or the user's export still wins; this file
//! only fills the gap, without touching the repo tree or any snapshot.

use std::path::PathBuf;

use margincode_core::{AvError, Result};

use crate::AppState;
use crate::disk::under_ws_root;

/// What every line 1 av ever wrote starts with; a file starting otherwise is the user's and
/// stays as it is.
const MARKER_PREFIX: &str = "# written by av:";

const CONFIG: &str = "# written by av: lean builds for every task workspace of this repo.
# Delete this file, or set AV_BUILD_CACHE=off before `av up`, to opt out.
# Each workspace builds into its own target/ (removed with the workspace); never share a
# target dir between workspaces of one repo: cargo would run another task's test binary.
[build]
incremental = false
[profile.dev]
debug = \"line-tables-only\"
# Dependencies carry no debug info at all; the repo's own crates keep file:line.
[profile.dev.package.\"*\"]
debug = false
";

/// `<ws_root>/<repo>/.cargo/config.toml` (in this process).
pub(crate) fn config_path(st: &AppState, repo: &str) -> Result<PathBuf> {
    under_ws_root(st, &[repo, ".cargo", "config.toml"])
}

fn ours(text: &str) -> bool {
    text.lines()
        .next()
        .is_some_and(|l| l.starts_with(MARKER_PREFIX))
}

/// Whether `<ws_root>/<repo>/.cargo/config.toml` is av's.
pub(crate) fn is_ours(st: &AppState, repo: &str) -> bool {
    config_path(st, repo)
        .ok()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .is_some_and(|text| ours(&text))
}

/// Writes the repo's cargo config when lean builds are on and trunk has a root `Cargo.toml`,
/// unless a file that is not av's is already there. An older av file (Plan 11's shared target
/// dir) is rewritten. Returns whether it wrote.
pub(crate) async fn ensure_build_cache(st: &AppState, repo: &str) -> Result<bool> {
    if !st.build_cache {
        return Ok(false);
    }
    let trunk = st.vcs.trunk(repo).await?;
    let files = st.vcs.list_files(repo, &trunk).await?;
    if !files.iter().any(|f| f == "Cargo.toml") {
        return Ok(false);
    }
    let path = config_path(st, repo)?;
    let current = std::fs::read_to_string(&path).ok();
    if current.as_deref() == Some(CONFIG) {
        return Ok(false);
    }
    if current.as_deref().is_some_and(|text| !ours(text)) {
        return Ok(false);
    }
    let write = || -> std::io::Result<()> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        std::fs::write(&path, CONFIG)
    };
    write().map_err(|e| AvError::Vcs(format!("writing {}: {e}", path.display())))?;
    Ok(true)
}

/// `ensure_build_cache`, with a failure logged instead of returned (imports and start-up go on).
pub(crate) async fn ensure_logged(st: &AppState, repo: &str) {
    match ensure_build_cache(st, repo).await {
        Ok(true) => tracing::info!(%repo, "lean builds: wrote .cargo/config.toml"),
        Ok(false) => {}
        Err(e) => {
            tracing::warn!(%repo, error = %e, "lean builds: could not write .cargo/config.toml")
        }
    }
}
