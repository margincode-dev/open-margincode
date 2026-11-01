# Handoff to Grok: a Teams panel on the dashboard

Read `CLAUDE.md` first (repo rules: TDD, fmt, clippy, tests green, one PR per change off `main`).

## Why

Each agent CLI (Claude Code, Cursor, Grok, Codex, ...) is one *team* that runs its own subagents.
PR #36 gives a team a WIP limit (`max_open`): at the limit, `next` only offers it reviews, merges
and its own abandoned tasks. The dashboard (`/ui`) shows tasks and the backlog but not the teams, so
a person cannot see who is full, who is idle, and what each team is good at.

## Before you start

- Wait until PR #36 (`team-wip-limit`) is merged, then `git checkout main && git pull`, and branch
  `ui-teams-panel` off `main`. If #36 is not merged yet, stop and say so; do not stack on it.
- Leave these untracked dirs alone (do not add or delete them): `bench/results/2026-10-31-relay-*`,
  `bench/retrieval/results/2026-10-29-budget-800/{gold.jsonl,texts/}`.
- `export PATH=$HOME/.cargo/bin:$PATH` in non-login shells.

## What to build

One file changes: `crates/scholion-daemon/src/ui.html` (vanilla JS, embedded with `include_str!`,
read-only). No new API route, no Rust change except the test below.

A **Teams** panel in the right-hand `aside.feed`, above "Backlog". One row per profile from
`GET /v1/agents` (`{agents: [{name, best, can, cost_rank, aliases, max_open?}]}`), sorted by
`cost_rank` then name. Each row shows:

- the name, and `cost N`;
- `best a,b` and `can c,d` (omit an empty list);
- load: the number of the team's **open** tasks across **all repos**, as `2/3 open` when `max_open`
  is set, `2 open` when it is not. Count from the task list `loadTasks` already fetches
  (`/v1/tasks?limit=500`, all repos, before the repo filter): `status === "open"` and
  `owner_agent === name`. Tasks `in_review` do not count (that matches the daemon's
  `count_open_tasks`);
- a `full` chip when open >= `max_open` (reuse the existing chip/dot styles, no new colours);
- `idle` (faint) when the team holds nothing open.

Also list owners seen on open tasks that have **no profile** (e.g. `codex`), with load only and the
note `no profile: takes untagged work only`.

Rules the page already follows; keep them:
- Only `textContent`/`h()`; never `innerHTML` with data (every string comes from agents).
- A failing panel must not hide the board: call it like `loadBacklog().catch(() => {})`.
- Refresh it wherever `loadBacklog` is refreshed (initial load and live events).
- `max_open` is `null` with no limit and absent on an older daemon: treat both as no limit.
- Works in both themes and under 720 px width.

## Test (write it first)

In `crates/scholion-daemon/tests/api.rs`, next to `ui_page_is_served_without_a_token_but_its_data_is_not`
(around line 2810), add a test `ui_page_reads_the_team_profiles` that GETs `/ui` and asserts the HTML
contains `/v1/agents` and `max_open`. That is the same level of check the page has today; the
behaviour itself is checked by hand (below).

```bash
cargo test -p scholion-daemon --test api ui_page_   # fails first, then passes
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Check by hand

Do not stop or restart the live daemon on :7420 (it runs on another build). Instead:

```bash
cargo build --release -p scholion-cli -p scholion-daemon
```

and open the page served by a throwaway daemon, or ask the human to restart theirs and open
`http://127.0.0.1:7420/ui`. With the live profiles (grok, cursor, claude) you should see three rows;
set a limit with `av agent set cursor --best code --can tests,ui,docs --cost 3 --max-open 1` and the
row shows `n/1 open` and `full` when cursor holds one open task.

## Land it

- Commit with a plain message, e.g. `Dashboard: a Teams panel (profile, load, limit)`.
- Push the branch and open a PR to `main` with `gh pr create`. Body: what the panel shows, a line on
  the by-hand check, and that no API changed. Do **not** merge; the human merges.
- Update `CLAUDE.md`'s `ui.rs + ui.html` bullet with one clause: "a Teams panel reads `/v1/agents`".
- Commit this handoff file in the same PR.
