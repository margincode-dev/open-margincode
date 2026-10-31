# Agent routing: one person, many subscriptions (design)

## Context

The goal: one person who pays for several agent subscriptions (Claude Code, Codex, Grok Build,
Gemini CLI, Meta's Muse Code) uses all of them on one project, and each kind of work goes to the
agent that does it best or most cheaply. Examples: images to Grok, code to Claude, routine work to
the cheap one (Muse), hard math or algorithms to Codex.

**What already works** (verified in code):
- One daemon, one MCP endpoint (`/mcp`). `av init --for codex|grok|gemini` wires those CLIs in
  (`crates/scholion-cli/src/agents.rs`). Claude Code gets hooks and the CLI.
- Every agent sees the same records and packs. `handoff` → `claim` works across vendors
  (`scripts/relay-demo.sh`: Claude → Codex).
- Each agent runs in its vendor's own CLI on its own subscription. Scholion never holds vendor
  credentials, and this plan keeps it that way.

**What is missing:**
- A backlog. Today `start_task` creates a task and assigns it to whoever called, so there is no
  "work waiting for the right agent".
- Agent profiles: what each agent is best at and how much it costs.
- Routing: matching the two.

**Decisions (user, 2026-10-31):**
- One machine, one local daemon.
- **Pull**: agents ask for work; the daemon never launches agents.
- Tags are set by humans, and the routing rules are deterministic: no LLM, zero extra tokens.
- Muse is "Muse Code" from Meta (installed with `irm https://dev.meta.ai/install.ps1 | iex`).

**Token rule** (product goal): no new MCP tool definitions. `start_task` and `handoff` each gain
one optional parameter (about 40 tokens of tool-list text in total). Routing runs in the daemon.

## Design

### Profiles (who is good at what)
New `agent_profiles` table: `name` (PK), `best text[]`, `can text[]`, `cost_rank int` (1 = cheapest),
`aliases text[]`.
- `aliases` maps MCP `clientInfo` names (e.g. a Codex client name) to the profile name.
- Tags are free text. Suggested set: `code`, `image`, `math`, `ui`, `docs`, `tests`.
- Example:
  `av agent set grok --best image --can ui --cost 2`,
  `av agent set claude --best code,tests --can docs,ui --cost 4`,
  `av agent set muse --can code,docs --cost 1`,
  `av agent set codex --best math --can code --cost 3`.

### Backlog (work waiting for an agent)
New `backlog` table:
- `id`, `repo`, `title`, `goal`
- `needs text[]`
- `prefer text NULL`: one agent name, a hard pin for its reserve window
- `priority int`
- `reserve_secs int` (default 600)
- `state`: `queued` | `taken` | `dropped`
- `task_id`, `taken_by`, `posted_by`, `created_at`, `taken_at`

The task lifecycle is unchanged. A backlog item becomes a normal task only when an agent takes it,
through the existing `create_started` path (`crates/scholion-daemon/src/api.rs` `start_task`). So
the invariant "every open task has a workspace and an intent" is never broken.

The same migration adds `tasks.needs text[]` and `tasks.prefer text` for handoffs. Both are set by
`handoff` and cleared by `claim`.

### Routing rules (pure, in `scholion-core/src/routing.rs`)
- An agent **can do** an item when every tag in `needs` is in its `best ∪ can`. Untagged items fit
  everyone. An agent with no profile fits only untagged items.
- Score = 2 per need in `best` + 1 per need in `can`.
- Agents are ranked by: `prefer` first, then score (high first), then `cost_rank` (cheap first),
  then name.
  - So untagged routine work goes to the cheapest agent (Muse).
  - `--needs code` sends work to Claude.
  - `--needs image` sends work to Grok.
- **Reserve window**: for `reserve_secs` after posting, only the top-ranked agent may take the item.
  After that, any agent that can do it may take it. This covers an agent whose quota ran out: it
  stops pulling, and the next-best agent picks the work up.
- When agent A asks, it gets one item. The order:
  1. items where A ranks first
  2. priority (high first)
  3. age (oldest first)
- Handed-off tasks that carry `needs` are candidates too: A claims them instead of starting new work.
- Taking an item is atomic: `UPDATE backlog SET state='taken' … WHERE id=$1 AND state='queued'`,
  trying candidates in order. If the task cannot be created, the item goes back to `queued`.

### Agent identity
- Agent identity comes from the `X-Av-Agent` header (`mcp.rs` `initialize`), then from
  `clientInfo.name` resolved through profile `aliases`.
- `av init --for X` also writes a static `X-Av-Agent: X` header into that CLI's MCP config, for each
  CLI whose config format supports static headers (check each vendor's documentation; where it does
  not, `aliases` covers the gap).

### Surfaces
- **HTTP**
  - `GET /v1/agents`, `PUT /v1/agents/{name}`
  - `POST /v1/backlog`, `GET /v1/backlog?repo=&state=`, `DELETE /v1/backlog/{id}` (drop)
  - `POST /v1/backlog/next` `{repo?, agent, session, budget}` returns `Started`, the handoff bundle
    of a claimed task, or `{task: null, queued: N}`.
- **MCP**
  - `start_task`: `title` and `goal` become optional. Description adds: "Omit title and goal to
    take the next backlog task that fits you."
  - `handoff`: gains `needs`, e.g. "needs: tags the next agent must have, e.g. [image]".
- **CLI**
  - `av agent set|list`
  - `av post --repo --title --goal [--needs a,b] [--prefer NAME] [--priority N] [--reserve SECS]`
  - `av backlog [--repo]`, `av backlog drop ID`
  - `av next [--repo]`: used by `av-wrap.sh` and scripts
  - `av handoff --needs …`
- **Dashboard** (`ui.html`): a read-only Backlog panel showing the queue, the top-ranked agent of
  each item, and who took what.
- **Agent instructions** (`integrations/agents-instructions.md`): one line: "With no task, call
  `start_task` without title and goal."
- **Muse Code**: read its MCP configuration documentation, then add `Agent::Muse` to `agents.rs`
  with a fixture test, the same as Codex/Grok/Gemini. Until then it works through
  `integrations/generic/av-wrap.sh` plus `av next`, and its profile works either way. Do not guess
  the config format.

### Out of scope (later plans)
- Push dispatch (`av dispatch` launching CLIs).
- Several people sharing one daemon (identity, network exposure).
- Routing learned from history (success rate per agent and tag).
- An LLM that tags tasks.

## Verification

- `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings &&
  cargo test --workspace` all green (embedded Postgres; no Docker).
- `av up` then `scripts/route-demo.sh --scripted` prints the expected agent → item assignments.
- Optional, on the user's own subscriptions (no API spend): `scripts/route-demo.sh` with the real
  `claude`, `grok` and `codex` CLIs. Each CLI, given only "take your next task", lands on its tagged
  item.
- Token check: compare the `tools/list` response size before and after. The target is under 60
  extra tokens.
