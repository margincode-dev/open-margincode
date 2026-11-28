# Hooks against a daemon that does not answer (2026-11-28)

What each `av hook` costs the agent when the daemon is down, hung or slow, before and after the
hook budgets (`AV_HOOK_*_SECS`, "When the daemon does not answer" in `docs/reference.md`). Debug
build of `av` from `main` at `0f4d4da` (before) and from branch `hooks-fail-open` (after), in a
`rust:1.97.1` container, each run with its own scratch `HOME` and a workspace holding only a
`.av-task` for task `t-1` and `app.py`. No real daemon in the failure cases: `fake.py` stands in.

Conditions (`audit.sh`):

- **refused**: nothing listens on the address.
- **hang**: `fake.py hang` accepts and never answers.
- **drop**: `192.0.2.1:7420` with `iptables -A OUTPUT -d 192.0.2.1 -j DROP` (the connect times out).
- **500**, **401**: `fake.py 500` / `fake.py 401` answer every request with that status at once.
- **slow**: `fake.py slow:10` answers `{}` with 200 after 10 s, every request. The Stop hook ran
  with `AV_HOOK_WAIT_SECS=0` here (a daemon that answers "nothing waiting" legitimately keeps the
  wait going); in every other condition with the default 600.

Wrappers: **cli** is `av hook NAME` with the host's JSON on stdin; **claude** is the command
`av init` installs for Claude Code (`command -v av >/dev/null 2>&1 && av hook NAME; exit 0`),
which Grok's hooks use too; **opencode** is `integrations/opencode/av.js` under the plugin harness
(`crates/margincode-cli/tests/opencode/harness.mjs`, node 20), timing the plugin's whole hook.

## Seconds per hook, before -> after

| condition | wrapper | session-start | pre-read | post-edit | stop |
|---|---|---|---|---|---|
| refused | cli, claude | 0.0 -> 0.0 | 0.0 -> 0.0 | 0.0 -> 0.0 | 0.0 -> 0.0 |
| refused | opencode | 0.3 -> 0.2 | 0.3 -> 0.3 | 0.3 -> 0.3 | 0.3 -> 0.3 |
| hang | cli, claude | 613.7 -> 15.1 | 300.4 -> 3.1 | 300.4 -> 5.1 | 611.7 -> 10.3 |
| hang | opencode | 60.3 -> 15.4 | 60.4 -> 3.3 | 60.4 -> 5.4 | 61.4 -> 10.5 |
| drop | cli, claude | 15.0 -> 5.0 | 5.0 -> 3.0 | 5.0 -> 5.0 | 10.1 -> 5.0 |
| drop | opencode | 15.3 -> 5.2 | 5.3 -> 3.3 | 5.3 -> 5.2 | 10.3 -> 5.3 |
| 500 | cli, claude | 0.1 -> 0.0 | 0.1 -> 0.0 | 0.1 -> 0.0 | 0.1 -> 0.0 |
| 500 | opencode | 0.3 -> 0.2 | 0.3 -> 0.2 | 0.3 -> 0.3 | 0.3 -> 0.3 |
| 401 | cli, claude | 0.1 -> 0.1 | 0.1 -> 0.0 | 0.1 -> 0.1 | 0.1 -> 0.0 |
| 401 | opencode | 0.3 -> 0.3 | 0.3 -> 0.2 | 0.3 -> 0.2 | 0.3 -> 0.2 |
| slow (10 s) | cli, claude | 22.0 -> 15.3 | 10.0 -> 3.1 | 10.1 -> 5.0 | 20.1 -> 10.0 |
| slow (10 s) | opencode | 22.3 -> 15.3 | 10.3 -> 3.3 | 10.3 -> 5.3 | 20.3 -> 10.2 |

The cli and claude rows were within 0.1 s of each other in every cell; the table shows the cli
number. Every run, before and after, exited 0. stdout was empty in every failure run except
**slow** session-start, which got the context answer (`{}`, an empty pack) and printed the line
that asks for records before the final message, before and after: that is an answer, not a
failure. stderr was one line (`av: hook NAME: …`). Under OpenCode every plugin hook returned
without throwing, so the tool call, prompt or turn went on; its log got one `av plugin: …` line.

Before, a hung daemon held a Claude Code or Grok read or edit for the CLI's 300 s overall timeout,
a session start for 614 s (the pack and the task lookup, 300 s each, then the health check;
longer than the 600 s Claude Code allows a hook by default) and a stop for 612 s (the checkpoint
and the task lookup, 300 s each; its own timeout is the wait + 60 s, 660 s), and an OpenCode hook for the plugin's 60 s.
A stopped daemon (refused) never cost more than 0.3 s.

`av-wrap.sh true` before (it is not a hook and is unchanged): refused 0.0 s, 500 0.0 s, drop 10.0 s,
hang 623.9 s (see `docs/backlog.md`, 2026-11-28).

## A session against a hung daemon (`session.sh`)

One `session-start`, then 30 times a `pre-read` and a `post-edit` 2 s apart, then a `stop`, in one
`HOME` (about 75 s of session), after the change: 24.4 s in hooks in all (session-start 15.0 s,
two `pre-read`s at 3.1 s when the 30 s skip ran out, the rest under 0.9 s each). Without the skip
the same session would pay every budget: 15 + 30 x 3 + 30 x 5 + 10 = 265 s; before the change
614 + 30 x 300 + 30 x 300 + 612, about 19,200 s under Claude Code.

## Healthy hooks (`healthy.sh`)

Against a real daemon (debug build, one agent, a repository of this repo's `crates/`, one record
of another task anchored to the file read), 20 runs each, after the change:

| hook | median | p95 | max |
|---|---|---|---|
| session-start | 85 ms | 106 ms | 127 ms |
| pre-read | 9 ms | 12 ms | 13 ms |
| post-edit (a checkpoint each) | 86 ms | 258 ms | 324 ms |
| stop (checkpoint + unrecorded, no wait) | 24 ms | 27 ms | 29 ms |

The default budgets (15, 3, 5, 10 s) sit far above these and about twice the slowest answers at
100 agents at once in `bench/results/2026-10-30-e2-overlay/tokio/report.md` (context 7467 ms,
checkpoint 1926 ms, record 2133 ms). Not measured: a hook against a daemon under that load.
