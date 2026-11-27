//! One test binary for the crate (`cargo test -p margincode-daemon --test it MODULE::NAME`): each
//! file here is a module, so the workspace links one executable per crate instead of one per
//! file (Plan 12: build output is the disk agents spend).

mod api;
mod common;
mod constitution;
mod disk;
mod dream;
mod embedded_pg;
mod history;
mod hosts;
mod mcp;
mod office;
mod outcomes;
mod questions;
mod remote;
mod routing;
mod serve;
mod swarm;
mod worker;
