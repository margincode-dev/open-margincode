//! Benchmark harness: replays real tasks with headless Claude Code (or Grok) in several arms and reports cost and wall time per accepted task.
//! The analyzer (`events`, `metrics`, `report`, `calibrate`) is pure; only `runner` and `orchestrate` start processes and can spend money.

pub mod calibrate;
pub mod chunklen;
pub mod conflict;
pub mod dreamed;
pub mod isolate;
pub mod metrics;
pub mod mine;
pub mod noise;
pub mod orchestrate;
pub mod report;
pub mod retrieval;
pub mod routing_profile;
pub mod runner;
pub mod scale;
pub mod spec;
pub mod stats;

/// Shared with the commercial runner; the old path keeps working.
pub use margincode_agentrun::events;

pub type Result<T> = std::result::Result<T, String>;
