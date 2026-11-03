//! One test binary for the crate (`cargo test -p scholion-index --test it MODULE::NAME`): each
//! file here is a module, so the workspace links one executable per crate instead of one per
//! file (Plan 12: build output is the disk agents spend).

mod chunks;
mod cleanup;
mod clone;
mod extract;
mod graph;
mod lexical;
mod overlay_equiv;
