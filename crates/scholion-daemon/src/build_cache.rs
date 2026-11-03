//! Plan 11: one cargo build cache per repo. A cargo `target/` per task workspace is ~24 GB each,
//! so a repo whose trunk has a root `Cargo.toml` gets `<ws_root>/<repo>/.cargo/config.toml`
//! pointing every workspace's builds at `<ws_root>/<repo>/.cache/cargo-target`.
//!
//! Cargo probes `.cargo/config.toml` in the working directory and in every parent; deeper files
//! win, and `CARGO_TARGET_DIR` in the environment beats every file. So the repo's own
//! `.cargo/config.toml` (inside the workspace, deeper) or the user's export still wins; this file
//! only fills the gap, without touching the repo tree or any snapshot. A relative `target-dir`
//! resolves against the parent of `.cargo`, i.e. `<ws_root>/<repo>` on whatever machine runs
//! cargo, so no host-path translation is needed. Cargo locks a target directory, so parallel
//! builds of one repo's workspaces queue instead of running side by side.

use std::path::PathBuf;

use scholion_core::{AvError, Result};

use crate::AppState;
use crate::disk::under_ws_root;

/// First line of the file av writes; a file starting otherwise is the user's and stays as it is.
const MARKER: &str = "# written by av: one cargo target for every task workspace of this repo.";

const CONFIG: &str = "# written by av: one cargo target for every task workspace of this repo.
# Delete this file, or set AV_BUILD_CACHE=off before `av up`, to opt out.
[build]
target-dir = \".cache/cargo-target\"
[profile.dev]
debug = \"line-tables-only\"
";

/// `<ws_root>/<repo>/.cargo/config.toml` (in this process).
pub(crate) fn config_path(st: &AppState, repo: &str) -> Result<PathBuf> {
    under_ws_root(st, &[repo, ".cargo", "config.toml"])
}

/// Whether `<ws_root>/<repo>/.cargo/config.toml` is av's.
pub(crate) fn is_ours(st: &AppState, repo: &str) -> bool {
    config_path(st, repo)
        .ok()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .is_some_and(|text| text.lines().next() == Some(MARKER))
}

/// Writes the repo's cargo config when the build cache is on and trunk has a root `Cargo.toml`,
/// unless a file that is not av's is already there. Returns whether it wrote.
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
    if current
        .as_deref()
        .is_some_and(|text| text.lines().next() != Some(MARKER))
    {
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
        Ok(true) => tracing::info!(%repo, "build cache: wrote .cargo/config.toml"),
        Ok(false) => {}
        Err(e) => {
            tracing::warn!(%repo, error = %e, "build cache: could not write .cargo/config.toml")
        }
    }
}
