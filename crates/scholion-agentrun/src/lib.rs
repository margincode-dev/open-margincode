//! Run an agent CLI as a child process and read what it prints: the part of the benchmark
//! harness (`scholion-bench`) that does not depend on the benchmark, shared with the commercial
//! runner. `AgentCommand` builds the launch (arguments, environment, budget cap flag, deadline,
//! where stdout and stderr go) and `run()` returns a `RunResult`: exit, whether it was killed,
//! usage totals, the events file and the parsed stream. Underneath, `process` starts a command
//! with a deadline and kills its whole process group when it runs out, and `events` parses the
//! `claude -p --output-format stream-json` transcript into usage, tool calls and the final
//! result. None of it knows about tasks, daemons or arms: which CLI, which variables and which
//! prompt are the caller's. `slots` counts what runs at once on a host (agents, builds) across
//! every process that starts them, so `av watch`, `av validate` and any other runner on the
//! machine share one budget.

// Starting and killing process groups is Unix-only; `events` and `slots` build everywhere (the
// CLI, which also builds on Windows, uses `slots`).
#[cfg(unix)]
pub mod command;
pub mod events;
#[cfg(unix)]
pub mod process;
pub mod slots;

#[cfg(unix)]
pub use command::{AgentCommand, RunResult};
