//! One test binary for the crate (`cargo test -p scholion-daemon --test it MODULE::NAME`): each
//! file here is a module, so the workspace links one executable per crate instead of one per
//! file (Plan 12: build output is the disk agents spend).

mod api;
mod common;
mod disk;
mod embedded_pg;
mod mcp;
mod office;
mod routing;
mod serve;
mod swarm;
mod worker;
