# Lean av: cut the tokens av's workflow costs

Date 2026-10-29. Status: approved in conversation, spec for review.

## Why

Scholion is a product people install, and its measure is tokens: save them, or keep them flat
while agents succeed more often or faster. The evidence so far:

- E4 relay (90 sessions, DeepSeek flash): `av` cost 22–25 % more per accepted task than `notes`,
  with no gain (every arm solved every task).
- The `cold` pilot (no lessons at all): 11/19 accepted, about $0.030 per accepted task against
  about $0.014 with the lessons. Recorded knowledge does save tokens; av has to deliver it no
  more expensively than plain files.
- Token analysis of 250 sessions (E3 Haiku, E4 DeepSeek):
  - 85 % of av's extra cost in E4 is requests that only make av calls after the last edit
    (`checkpoint` 39, `record` 32 + 11 via Bash, `submit` 23, `status` 21, `ToolSearch` 38 to load
    the deferred MCP tool schemas, over 30 sessions).
  - The start pack is about 600 tokens (7 % of the extra); `get_context` results average 12.6k
    characters.
  - E3's hooks-only av (no MCP) was the one setup cheaper than baseline (-4 %).
  - The `post-edit` hook already checkpoints after every edit, so the manual `checkpoint` calls
    were pure overhead; the MCP server's instructions asked for them.
  - The bench agents could not run tests: `pytest` is only in `~/bench-work/venv`, which is not
    on their PATH (195 `No module named pytest`, 204 `command not found` in E4). This hits every
    arm equally but inflates turns and failures.

## Goal and success criterion

On a rerun of E4 with arms `cold`, `notes` and `av` (10 tasks x 3 reps, about $1.50), lean `av`
costs no more per accepted task than `notes` (target: less) and accepts clearly more than `cold`.

## Changes

### 1. Bench: agents can run the tests

- New optional task-spec field `path_prepend = ["~/bench-work/venv/bin"]` (list, `~` expanded):
  prepended to `PATH` in the agent's environment for every arm. `scholion-bench check` warns if
  a listed directory does not exist.
- Set it in `bench/tasks-tallyho` and `bench/tasks-relay` specs.

### 2. Claude Code integration: hooks + CLI, MCP optional

- `integrations/claude-code` installs the hooks and relies on the `av` CLI (through Bash) by
  default; the MCP server entry becomes opt-in (documented, not in the default snippet).
- `scholion-bench run` gets `--av-mcp` (default off) so both setups stay comparable. Today the av
  arms add MCP when `cfg.av_mcp` is set; the default flips.
- The MCP server stays for clients without hooks.

### 3. No ritual calls

- `submit` already snapshots the workspace (`flow.rs`); `record` is unchanged.
- MCP `INSTRUCTIONS` and tool descriptions: drop "call `checkpoint` after edits"; say that
  `submit` checkpoints, and that `checkpoint` is only needed to refresh `get_context` mid-task
  in clients without hooks. Keep them short.
- `submit` accepts optional inline `records` (same shape as `record`) and writes them before the
  gate, in the same request, so record-then-submit is one call. CLI: `av submit --record
  "KIND: SUMMARY @ path[:Symbol]"` (repeatable; the `@ anchor` part optional).
- `submit`'s response says the new status and the next step, so no `status` follow-up is needed.
- `av record --json FILE` (`-` for stdin) takes a list of records in the `record` shape, so
  several records are one call; the stop-hook nudge text shows the one-call form.

### 4. Cap what av returns

- `get_context` (MCP and `av context`) defaults to 800 tokens instead of 4000; agents can ask for
  more with `budget`.
- The stop hook does not nudge when every unrecorded changed symbol is test code
  (`is_test_code` from `scholion-pack::score`).

## Out of scope

Git-native records and a no-daemon solo mode (option B), drafting records from the diff, E3
reruns (no Anthropic budget).

## Testing

- TDD per change: daemon tests in `crates/scholion-daemon/tests/api.rs` and `tests/mcp.rs`
  (inline records on submit, submit response, default budget), CLI tests in
  `crates/scholion-cli/tests/cli.rs` (multi-record, stop hook skips test-only changes), bench
  tests in `crates/scholion-bench/tests/` (`path_prepend`, `--av-mcp` default).
- fmt, clippy `-D warnings`, `cargo test --workspace` green.
- Then the E4 rerun (`cold,notes,av`, `--reps 3`, `--budget-usd 2`), report committed under
  `bench/results/`, result written into HANDOFF.md whatever it shows.

## Delivery

One PR off `main` with: the `cold` arm and goal fixes (branch `e4-harder`), the E1 CI gate
(branch `e1-ci-gate`), lean av, and the E4 rerun result.
