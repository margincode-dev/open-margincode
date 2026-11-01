# Smoke test: Claude Code and Grok Build share one backlog

A hypothetical project for trying Plan 8 routing with real agents. `pantry/` here is a small Python
CLI that tracks kitchen items and expiry dates. It has a planted bug, an unstyled report, README
typos and no icon. Its tests run with `python3 -m unittest`, and it needs nothing but Python.

```bash
cargo build --release -p scholion-cli -p scholion-daemon
examples/routing-smoke/setup.sh        # SRC=… REPO=… AV_URL=… to change the defaults
```

`setup.sh` does the following:
- copies `pantry/` into a git repo at `~/projects/pantry` (unless one is there);
- starts the daemon if none answers (`av up`, http://127.0.0.1:7420, dashboard at `/ui`);
- registers the repo as `pantry`;
- sets the two agent profiles;
- posts seven tasks, once.

It spends nothing.

## Profiles

| agent  | best        | can           | cost (1 = cheapest) |
|--------|-------------|---------------|---------------------|
| claude | code, tests | docs, ui      | 4                   |
| grok   | image       | ui, docs, code| 2                   |

## Backlog and where each task should go

| # | task | needs | priority | should go to | why |
|---|------|-------|----------|--------------|-----|
| 1 | Refuse invalid dates | code,tests | 2 | claude | only claude has `tests` |
| 2 | Category totals, then the chart | code | 1 | claude, then **handoff needs [image] → grok** | Claude is `best` at code; tests the cross-vendor handoff |
| 3 | Add an expiring command | code | 0 | claude | `best` (2) beats grok's `can` (1) |
| 4 | Write docs/USAGE.md | docs | 0 | claude | `--prefer claude` overrides the cheaper grok |
| 5 | Draw the app icon | image | 1 | grok | only grok has `image` |
| 6 | Style the HTML report | ui | 0 | grok | both `can`; grok is cheaper |
| 7 | Fix the README typos | (none) | 0 | grok | untagged → the cheapest agent |

## Second project: Sunken Crown (if `~/projects/sunken-crown` exists)

`setup.sh` also registers the game repo as `sunken-crown` and posts five tasks for it. The game is
Godot/GDScript with Python tools. av indexes only the Python, so GDScript files get records and
packs but no symbols. Every goal tells the agent to read `HANDOFF.md` and leave it unedited, and
not to push. av imports a copy of the repo, so the original checkout is never changed.

| task | needs | priority | should go to |
|------|-------|----------|--------------|
| Validator: every art path must exist | code,tests | 1 | claude (Grok lacks `tests`) |
| Remove the duplicate `_is_enemy` | code | 0 | claude |
| Review the shard icons at 48 px | image | 1 | grok |
| Check the weak spots of art batches 6-10 | image | 0 | grok |
| Explain the tools folder | (none) | 0 | grok (cheapest) |

`start_task` with no arguments draws from every repo, so one agent session may work on both
projects in turn. Pass `repo` to keep it on one project; `av next --repo` does the same on the
CLI.

## Reservations

Each task is reserved for its top agent for 10 minutes after it was posted. After that, any agent
that can do it may take it. So if Claude is slow, Grok (which can do `code`) may pick up #2 or #3.
That is the quota-fallback rule working, not a bug. Grok can never take #1, which needs `tests`.
To get the reservation back, start a fresh daemon and run `setup.sh` again. On a daemon whose
backlog already has these items, `setup.sh` posts nothing and the timers keep running.

## Connect the agents (MCP, one URL)

Run these yourself. Both commands change your user-level config, and each keeps a backup.

```bash
# Claude Code (user scope). The header names it for routing.
claude mcp add --transport http --scope user av http://127.0.0.1:7420/mcp --header "X-Av-Agent: claude"

# Grok Build. Writes [mcp_servers.av] with X-Av-Agent = "grok" into ~/.grok/config.toml (keeps .bak).
cd ~/projects/pantry && <scholion>/target/release/av init --for grok --no-claude
grok mcp doctor av      # should list the av tools
```

An `av` installed before Plan 8 lacks `agent`, `post`, `backlog` and `next`. Use the one in
`target/release/`, or put that directory first on your PATH.

## Prompt to give each agent

Start each agent in `~/projects/pantry` and paste this prompt:

> You are working through the av MCP server. Loop: call `start_task` with no title or goal and do
> what it returns. A review: `review_task`, run `python3 -m unittest` in its workspace, `validate`
> with that command and its exit code, `post_review`, and if you approve, `merge`. A task (new or
> handed off): work ONLY in the workspace path it returns (git status there shows nothing; that is
> expected), read its pack, do the goal, run the tests, `validate`, then `submit` with at least one
> `decision` or `note` record explaining why. If the goal tells you to hand off, call `handoff` with
> the `needs` it names instead of submitting. Pass `task_id` (or the task's slug) on every call.
> When it returns no task, call it again with `wait` 50; stop after three empty waits and report
> what you did.

Instead of keeping each agent open, `av watch` can start it only when there is work for it. It
asks every 15 s what `start_task` would give the agent (taking nothing) and runs the command when
something fits:

```bash
av --agent claude watch --repo pantry -- claude -p "$(cat prompt.txt)"
av --agent grok watch --repo pantry -- grok -p "$(cat prompt.txt)"
```

`--once` checks a single time; `--max-runs N` stops after N runs. It runs on your machine; the
daemon still never starts a process.

For Grok, also set `PYTHONDONTWRITEBYTECODE=1`, otherwise `__pycache__` shows up in the changed
files. Its `--sandbox workspace` does not confine writes, so watch which paths it edits.

## What to check

- `av backlog --repo pantry`, or the dashboard's Backlog panel: who took each item (`taken_by`).
  The expected column is in the table above.
- When #2 is handed off, Grok claims it (generation 2) and draws `assets/chart.svg` from Claude's
  `totals_by_category`. Claude is not offered it again, because Claude lacks `image`.
- Each agent's later pack shows the other agent's live work and records.
- The review loop: once a task is submitted, the other agent's next `start_task` offers it for
  review before any new work, and the reviewer merges what it approves. An owner is never offered
  its own review. Grok reviews Claude's code work; Claude reviews Grok's image work (only Grok can
  do `image`, so any other agent may review it).

## Two agents on DeepSeek Flash (the v0.0.2 run)

`deepseek-demo.sh` runs the loop with no subscription: two Claude Code agents pointed at
DeepSeek's Anthropic-compatible API, `coder` (best `code,tests`) and `writer` (best `docs,ui`,
cheaper), each with `--max-open 1`. They take five tasks, review each other's work and merge it.
It uses its own daemon on :7441 with data in `~/scholion-demo`, so an `av up` daemon on :7420 is
left alone.

```bash
examples/routing-smoke/deepseek-demo.sh up
examples/routing-smoke/deepseek-demo.sh setup          # `setup pilot` posts one task only
examples/routing-smoke/deepseek-demo.sh agent coder    # terminal 1
examples/routing-smoke/deepseek-demo.sh agent writer   # terminal 2
examples/routing-smoke/deepseek-demo.sh down
```

It needs `~/.config/deepseek/key` (the key alone, `chmod 600`). On 2026-11-01 a pilot of one
task and its review cost about $0.01, and the five-task run about $0.03. Claude Code cannot price
DeepSeek, so the `costUSD` in its session logs uses Claude prices; check
`https://api.deepseek.com/user/balance` instead. Set `DEMO` and `AV_URL` the same for every
command when you change them.

## Reset

```bash
av down                 # stop the daemon. Data stays in ~/.scholion; workspaces are in ~/ws
rm -rf ~/.scholion ~/ws # only if you want a clean slate (deletes all av data on this machine)
```

`~/projects/pantry` itself is never changed: av imports it, and merges land on av's own copy
(`~/.scholion/store`). To see the merged result, open the workspace of a task started after the
merge, or use `av log` and `av show`.
