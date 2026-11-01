//! Benchmark harness (Plan 4): replays real tasks with headless Claude Code in three arms
//! (`baseline` git checkout, `av_no_packs`, full `av`) and reports cost and wall time per accepted
//! task. The analyzer (`events`, `metrics`, `report`, `calibrate`) is pure; only `runner` and
//! `orchestrate` start processes, and only they can spend money. `retrieval` (Plan 6, E1) scores
//! context packs against baselines offline, with no agent; `mine` builds its task sets from history.

pub mod calibrate;
pub mod metrics;
pub mod mine;
pub mod orchestrate;
pub mod report;
pub mod retrieval;
pub mod runner;
pub mod scale;
pub mod spec;
pub mod stats;

/// The transcript parser lives in `scholion-agentrun`, shared with the commercial runner; the
/// old path keeps working.
pub use scholion_agentrun::events;

pub type Result<T> = std::result::Result<T, String>;
