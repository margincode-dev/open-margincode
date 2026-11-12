//! One test binary for the crate (`cargo test -p scholion-bench --test it MODULE::NAME`): each
//! file here is a module, so the workspace links one executable per crate instead of one per
//! file (Plan 12: build output is the disk agents spend).

mod common;
mod isolation;
mod metrics;
mod noise;
mod report;
mod retrieval;
mod retrieval_gate;
mod routing_profile;
mod routing_tasks;
mod runner;
mod scale;
mod spec;
