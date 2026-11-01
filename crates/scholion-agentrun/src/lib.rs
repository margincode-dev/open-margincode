//! Run an agent CLI as a child process and read what it prints: the part of the benchmark
//! harness (`scholion-bench`) that does not depend on the benchmark, shared with the commercial
//! runner. `AgentCommand` builds the launch (arguments, environment, budget cap flag, deadline,
//! where stdout and stderr go) and `run()` returns a `RunResult`: exit, whether it was killed,
//! usage totals, the events file and the parsed stream. Underneath, `process` starts a command
//! with a deadline and kills its whole process group when it runs out, and `events` parses the
//! `claude -p --output-format stream-json` transcript into usage, tool calls and the final
//! result. None of it knows about tasks, daemons or arms: which CLI, which variables and which
//! prompt are the caller's.

pub mod command;
pub mod events;
pub mod process;

pub use command::{AgentCommand, RunResult};
