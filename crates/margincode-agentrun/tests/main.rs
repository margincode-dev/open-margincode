//! One test binary for the crate (`cargo test -p margincode-agentrun --test it MODULE::NAME`): each
//! file here is a module, so the workspace links one executable per crate instead of one per
//! file (Plan 12: build output is the disk agents spend).

#[cfg(unix)]
mod command;
mod events;
#[cfg(unix)]
mod process;
mod slots;
