# Scholion Plan 8: Route tasks to the agent that fits

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Let one person with several agent subscriptions (Claude Code, Codex, Grok Build, Gemini CLI, Muse Code) post tasks to a backlog with tags; each agent pulls the item that fits it best.

**Design:** `docs/superpowers/specs/2026-10-31-agent-routing-design.md` (read it first).

**Status (2026-10-31):** Tasks 1–9 are done on branch `plan-8-routing`; Task 10 (`scripts/route-demo.sh`) follows. Task 8 is partly done: Muse Code's MCP settings file could not be checked without a login, so there is no `av init --for muse`; `integrations/muse/README.md` records what was and was not verified. Differences from the text below: the backlog list also returns `best_agent`; `X-Av-Agent` is written for Grok (checked) and Gemini (same header map as `Authorization`, not checked against the real CLI), not Codex.

## Tasks (TDD each: failing test, code, `cargo fmt`, `clippy -D warnings`, `cargo test --workspace`)
 (TDD each: failing test → code → `cargo fmt`/`clippy -D warnings`/`cargo test --workspace`)

1. **Spec + plan docs.**
   - Copy this design to `docs/superpowers/specs/2026-10-31-agent-routing-design.md` and the tasks
     to `docs/superpowers/plans/2026-10-31-scholion-plan-8-routing.md`.
   - Update CLAUDE.md's plan list.
   - Branch `plan-8-routing` off `main`.
2. **Routing core.** `crates/scholion-core/src/routing.rs`: `Profile`, `BacklogItem`, `rank()`,
   `can_do()`, `eligible(agent, item, now)`, `order_for(agent, items, profiles, now)`.
   - Unit tests: untagged → cheapest; hard tags; `prefer` pin; reserve expiry; unknown agent;
     deterministic ties.
3. **Store.** Migration `0022_routing.sql`: the profiles and backlog tables, plus `tasks.needs` and
   `tasks.prefer`.
   - Check `main`'s highest migration number first.
   - Add the hash line to `tests/migrations_frozen.rs`.
   - Add the `IntentStore` methods in `traits.rs` and implement them in `scholion-store`:
     `put_profile`, `list_profiles`, `post_backlog`, `list_backlog`, `drop_backlog`,
     `take_backlog(id, agent)` (atomic), `untake_backlog`, and `set_handoff_needs`.
   - Store tests: two concurrent takes of one item yield exactly one winner.
4. **Daemon API.** Routes in `api.rs`. `next` reuses `create_started`/`pack_for_task` for backlog
   items and `flow::claim` for handed-off tasks. `flow::handoff` stores `needs`/`prefer`, and
   `flow::claim` clears them.
   - Tests in `crates/scholion-daemon/tests/api.rs`, using `tests/common/mod.rs` helpers.
5. **MCP.** In `mcp.rs`: optional `title`/`goal` on `start_task` (absent → next), and `needs` on
   `handoff`; alias resolution in `initialize`. Tests in `tests/mcp.rs`.
6. **Swarm test.** In `tests/swarm.rs`: five agents with the example profiles pull 20 mixed items
   concurrently.
   - Assert: each item is taken once; `image` items only by Grok; untagged items by Muse within the
     reserve window; after the reserve window, others take what is left.
7. **CLI.**
   - Add `agent`, `post`, `backlog`, `next`, and `handoff --needs` in `scholion-cli/src/main.rs`.
   - `av init --for` writes the `X-Av-Agent` header where the vendor supports it (fixture tests in
     `agents.rs`).
   - Tests in `crates/scholion-cli/tests/cli.rs`.
8. **Muse Code integration.**
   - Read its documentation, then add `Agent::Muse`, `integrations/muse/README.md`, and fixtures.
   - If it has no MCP config file, the README documents the `av-wrap.sh` path instead.
9. **Dashboard + docs.**
   - Add the Backlog panel to `ui.html`.
   - Update the README sections "Agent integrations" and "Use", plus `agents-instructions.md`.
10. **Free end-to-end demo.** `scripts/route-demo.sh --scripted`:
    - post an untagged item, a `code` item and an `image` item;
    - three scripted agents each call `av next` and get the expected item;
    - Claude hands off with `--needs image`, and Grok's `av next` claims it.

    Modeled on `scripts/relay-demo.sh`. Spends nothing.

## Verification

- `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings &&
  cargo test --workspace` all green (embedded Postgres; no Docker).
- `av up` then `scripts/route-demo.sh --scripted` prints the expected agent → item assignments.
- Optional, on the user's own subscriptions (no API spend): `scripts/route-demo.sh` with the real
  `claude`, `grok` and `codex` CLIs. Each CLI, given only "take your next task", lands on its tagged
  item.
- Token check: compare the `tools/list` response size before and after. The target is under 60
  extra tokens.

## Evaluation

How to find out whether routing (and Grok for image work) pays is in
`2026-10-31-plan-8-grok-evaluation.md`: conformance of Grok Build as an `av` agent (E-G0), a
per-tag capability profile of the agents (E-G1), routed vs solo vs random (E-G2), quota exhaustion
(E-G3) and a relay with Grok in the chain (E-G4). It is run after Tasks 1–10 and spends no API money;
the runner adapter it needs is listed there.
