//! Benchmark harness (Plan 4): replays real tasks with headless Claude Code in three arms
//! (`baseline` git checkout, `av_no_packs`, full `av`) and reports cost and wall time per accepted
//! task. The analyzer (`events`, `metrics`, `report`, `calibrate`) is pure; only `runner` and
//! `orchestrate` start processes, and only they can spend money.

pub mod calibrate;
pub mod events;
pub mod metrics;
pub mod orchestrate;
pub mod report;
pub mod runner;
pub mod spec;
pub mod stats;

pub type Result<T> = std::result::Result<T, String>;
