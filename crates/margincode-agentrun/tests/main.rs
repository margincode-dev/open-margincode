//! One test binary for the crate (`cargo test -p margincode-agentrun --test it MODULE::NAME`): each
//! file here is a module, so the workspace links one executable per crate instead of one per
//! file (Plan 12: build output is the disk agents spend).

/// The one bound on any wait in a test: past it, the test hung.
pub const HANG: std::time::Duration = std::time::Duration::from_secs(120);

/// Deadlines for a test whose child must print before its deadline kills it: 400 ms, doubled,
/// eight of them (51.2 s the last, 102 s in all: within `HANG`). A loaded host may not run the
/// child in time for a short one, so such a test tries the next instead of failing.
#[cfg(unix)]
pub fn deadlines() -> impl Iterator<Item = std::time::Duration> {
    (0..8).map(|k| std::time::Duration::from_millis(400 << k))
}

#[cfg(unix)]
mod command;
mod events;
#[cfg(unix)]
mod process;
mod slots;
