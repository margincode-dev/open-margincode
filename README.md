# Margincode

**One memory for all the coding agents you pay for, and it stays right as it grows.** Margincode
pins one-line notes (decisions, and what didn't work) to the code they are about, and hands
whichever agent comes next a small brief built from them. What it measured:

- **Lessons reach the next agent.** With lessons an earlier task left (written for the
  benchmark), 30 of 30 runs were accepted against 18 of 30 without them, and cost per accepted
  task was 42.2 % lower (Claude Code on DeepSeek Flash, 10 tasks × 3 runs). A hand-kept
  `NOTES.md` did as well at this size ([run](bench/results/2026-10-29-relay-lean/)).
- **It holds up as memory grows.** With 500 unrelated notes beside the ten that matter, cost per
  accepted task was 39.9 % lower than with a flat `NOTES.md`: $0.0143 against $0.0237, same
  tasks and model, both 30 of 30 accepted ([run](bench/results/2026-11-e4-scale/)).
- **Wrong notes get corrected.** With a wrong note in memory, 30 of 30 runs were accepted on the
  build where agents can correct records, against 11 of 30 on the build before (Claude Haiku 5.5,
  3 tasks × 10 runs, all of the difference on two of them; 30 against 10 on Sonnet 5.5). The same note in a `NOTES.md` got 12 of 30,
  and no note at all 30 of 30 ([run](bench/results/2026-11-14-m0/)). On DeepSeek Flash, on one
  task, the same change went from 2 of 30 to 12 of 30, still below a `NOTES.md`'s 28 of 30
  ([run](bench/results/2026-11-wrong-lesson-3/)).

For one agent alone, it is not shown to cost less ([Does it help?](#does-it-help)).

The memory crosses vendors: Claude Code starts a job, Codex picks it up from the brief, Gemini
reviews it, and the reviewer merges.

*σχόλιον (margincode): a note written in the margin of a manuscript. Margincode keeps notes in the
margins of your code, so your AI agents stop forgetting.*

[![GitHub stars](https://img.shields.io/github/stars/margincode-dev/open-margincode?style=social)](https://github.com/margincode-dev/open-margincode/stargazers)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Release](https://img.shields.io/github/v/release/margincode-dev/open-margincode?include_prereleases)](https://github.com/margincode-dev/open-margincode/releases)

**New in 0.0.5:** two security fixes. A daemon without `AV_TOKEN` refuses requests a web page on
another site could send (a `Host` that is an unknown dotted name, a foreign `Origin`); a client
that reaches it by such a name needs `AV_TOKEN` or `AV_ALLOWED_HOSTS`. A backup remote's
credentials stay out of logs, stored errors and answers. **In 0.0.4**, the first release from
this repository: agents can correct a stored record (`av record --supersedes`), packs say when a
record comes from reverted, abandoned or open work, `av dream` and `av verdict` give records a
second look, and `av init --for antigravity` connects Google's Antigravity CLI
([changelog](CHANGELOG.md)).

## A short story

**Monday.** Claude Code spends an hour on a flaky network call. First it tries the obvious fix,
a retry library, and finds out the library isn't installed in production. Then it finds the real
fix, gets halfway, and runs out of context.

**Tuesday.** Codex picks the job up. Git shows it a diff and nothing else. So it reads the whole
repo again, reaches for the same retry library, hits the same wall, and burns the same hour.

**Tuesday, with Margincode.** Codex opens the task and gets a one-page brief: the goal, what's done,
*"tried the retry library: not installed in prod, don't"*, and the three files that matter. It
starts where Claude stopped. Gemini reviews the work, and the reviewer merges it.

Git remembers **what** changed. Margincode also remembers **why**, and **what didn't work**, and
hands it to whichever agent comes next.

[![A recorded run: Codex's brief leads with Claude's decision and dead end](docs/img/demo-replay.png)](https://open.margincode.dev/demo)

**[Watch the recorded run](https://open.margincode.dev/demo)**

## Why it exists

Many developers now pay for more than one agent subscription, such as Claude Code, Codex, Gemini, Grok, Cursor or Antigravity, because each model is better at something different. Today those agents cannot share what they learned about your code, so every handoff between them is a diff with no reasoning, and each one starts from zero and repeats the dead ends of the others. Margincode gives all your subscriptions one repository, one backlog and one shared memory, so the next agent, from any vendor, starts where the last one stopped.

## What you get

- **Notes in the margin.** Agents leave one-line notes (decisions, dead ends) pinned to the code
  they touched. When any agent opens that file later, the note comes with it.
- **A brief instead of a re-read.** Every agent starts from a small brief, sized to a token
  budget: the goal, the history that matters, and the code to look at.
- **One backlog for all your agents.** Claude Code, Codex, Gemini, Cursor, Grok, Antigravity and OpenCode share it. Each
  takes the work it's best at, another agent reviews it, and the reviewer merges.
- **A dashboard** at `http://localhost:7420/ui` to watch them work.
- **A disk that stays lean.** Workspaces of merged and abandoned tasks are removed after an hour
  with their build output, a Rust repo's tasks build lean (a fifth of the disk of a default
  `cargo test`, each task in its own target dir), and the hooks tell an agent what it left outside
  its workspace. `av disk` shows where the space went; `av gc` reclaims it.

It runs on your machine. No Docker and no account, and it never sees your model keys.

## How it compares

| | Margincode | Claude Code memory | claude-mem | Beads | Entire | `NOTES.md` by hand |
|---|---|---|---|---|---|---|
| Agents from other vendors share it | yes: an MCP server, plus hooks for Claude Code | `CLAUDE.md` can be an `AGENTS.md` other agents read; auto memory is Claude Code's and machine-local | yes: Claude Code, Codex, Cursor, OpenCode and more | yes: `bd setup` for Codex, Claude, Cursor and more | records Claude Code, Codex, Cursor and more; resuming in another agent not stated | yes, if each agent is told to read it |
| Notes anchored to code | yes: a path or `path:Symbol`, marked when that code changes | rules scoped by path glob; auto memory not stated | not stated | not stated | to commits, by a trailer | no |
| A wrong note is corrected or retired | yes: `av record --supersedes` | you or Claude edit or delete the files | not stated | `bd close`, `bd update`, a `supersedes` link | not stated | edit the file |
| Brief cut to a token budget, no model call | yes: 800 tokens by default, deterministic | first 200 lines or 25 KB of `MEMORY.md`; Claude writes auto memory | a second agent writes observations and summaries; a count limit, no token budget stated | `bd prime`; no token budget stated | not stated | no: the agent reads the whole file |
| Works with no daemon or server | **no**: a daemon with its own Postgres | yes: plain files, built in | no: a local HTTP worker and SQLite | yes: Dolt runs in-process (a server mode exists) | hooks and git; no daemon mentioned | yes |
| Setup | install script, then `av init` per repo | none, built in | `npx claude-mem install` | `brew install beads`, then `bd init` | `brew install --cask entireio/tap/entire`, then `entire enable` | none |
| Changes how you use git | **yes**: each job is an av task in its own workspace, and a merge moves Margincode's `main`, not your checkout | `CLAUDE.md` is a file in the repo, shared through source control; auto memory lives outside it | not stated | `bd init` writes `AGENTS.md`; `--stealth` turns off its git hooks; syncs over `refs/dolt/data` on your remote | git hooks, a commit trailer, checkpoint refs pushed with your code; no commits on your branch | a file in the repo |
| One agent alone costs less | **not shown**: 7 % more in the tracker run of 2026-11-21 ([Does it help?](#does-it-help)) | not measured here | not measured here | not measured here | not measured here | not measured here |

From each project's own page, read on 2026-11-28:
[Claude Code memory](https://code.claude.com/docs/en/memory),
[claude-mem](https://github.com/thedotmack/claude-mem),
[Beads](https://github.com/gastownhall/beads), [Entire](https://github.com/entireio/cli).
"Not stated" means that page does not say; "not measured here" means this repository has no run
of it.

## Does it help?

Measured so far, including where it didn't help:

- On a small Python repo, Claude Code needed **12.9 % fewer turns per task at the same cost**
  (October 2026, [benchmark](bench/results/2026-10-27-haiku/)).
- On the openevaluations tracker's run of 2026-11-20, on the same four tasks and model, release
  0.0.2 cost more than plain Claude Code: $0.0679 against $0.0540 for a leg of 12 sessions, too
  few for a claim. In that run every session was stopped once after its final answer and asked to
  record what it had changed, and its first record was refused for length. Both are changed in
  release 0.0.4
  ([report](https://github.com/vinhqbui/openevaluations/blob/main/docs/results/2026-11-20-tracker-haiku.md)).
  On the run of 2026-11-21, release 0.0.4 cost $0.0580 against $0.0545 for plain Claude Code on
  the same day, again a leg of 12 sessions and too few for a claim: 7 % more when paired by task
  (95 % interval 1 % to 14 %), with 20 % more billed tokens and 10 % fewer turns, and no session
  was stopped. The two runs differ in day and in Claude Code version
  ([report](https://github.com/vinhqbui/openevaluations/blob/main/docs/results/2026-11-21-tracker-haiku-margincode-0.0.4.md)).
- When one agent's lessons were passed on to the next, cost per solved task dropped by about
  **42 %**, and 30 of 30 tasks were solved instead of 18. Margincode did as well as a hand-written
  `NOTES.md`, without anyone having to write it.
- With 500 unrelated notes next to the ten that matter, cost per accepted task on DeepSeek Flash
  was **$0.0143 with Margincode against $0.0237 with a flat `NOTES.md`, 39.9 % less**. Without
  the extra notes the two cost the same ($0.0125 and $0.0126). Both solved 30 of 30 tasks, so
  this shows cost, not solve rate ([results](bench/results/2026-11-e4-scale/)).
- Not measured yet: large repositories.

## Try it

**The easy way: ask Claude.** Add the Margincode plugin to Claude Code once:

```bash
claude plugin marketplace add margincode-dev/open-margincode
claude plugin install margincode@margincode
```

Then open Claude Code in your repository and say *"set up Margincode for this repo"* (or run
`/margincode:setup`). Claude installs it, connects your agents and checks that everything works.
`/margincode:team` sets up several agents to share one backlog, and `/margincode:doctor` fixes a setup
that isn't working.

**By hand** (Linux, macOS on Apple silicon, or Windows through WSL):

```bash
curl -fsSL https://raw.githubusercontent.com/margincode-dev/open-margincode/main/scripts/install.sh | sh
cd your-repo && av init                       # starts Margincode, adds this repo, connects Claude Code
av init --for codex --for gemini --for cursor # the same for other agents (also grok, antigravity, opencode)
```

Restart your agent, then ask it to *"start an av task to …"*. Open `http://localhost:7420/ui` to
watch. A merge moves Margincode's own `main`, not your checkout's: `av repo backup NAME --remote URL`
pushes it to a git remote after every merge (a bare repo or GitHub; add `--branch av-main` to get
a branch you can open a pull request from), and `av log --repo NAME` shows it.

**Let the agent lead a team.** In our own use, one Claude Code or Grok session pointed at one av
task at a time, carried by the hooks, does the task but loses the thread between tasks. What
works better is to ask the session to run the backlog as a lead: *"take the av backlog: start a
subagent per task, monitor them, review what comes back and keep going"*. The session and its
subagents count as one agent to Margincode (the same name on every call), so it never reviews its
own work; another agent does. Size backlog items as whole jobs with a definition of done, not
single steps. We have not benchmarked this; it is what the project itself is built with.

Other channels ([details](docs/install.md)):

```bash
cargo binstall --git https://github.com/margincode-dev/open-margincode margincode-cli   # prebuilt, no compile
brew tap margincode-dev/open-margincode https://github.com/margincode-dev/open-margincode && brew install margincode
docker run -d -p 127.0.0.1:7420:7420 -e AV_LISTEN_OPEN=1 -v margincode:/data ghcr.io/margincode-dev/open-margincode
```

## ⭐ If it helps, star it

Margincode is a small open-source project. If it saved your agents a re-read, a
[star](https://github.com/margincode-dev/open-margincode/stargazers) is how other people find it.
Bugs and ideas are welcome in [issues](https://github.com/margincode-dev/open-margincode/issues).

## Learn more

- **[Reference](docs/reference.md)**: every command, configuration, the Docker stack, design,
  architecture and the full measurements.
- [Changelog](CHANGELOG.md) · [Contributing](CONTRIBUTING.md) · [Benchmarks](bench/README.md)
- The CLI is `margincode`, with the short alias `av`.

**How it was built.** Margincode is vibe-coded: AI coding agents (mostly Claude Code) wrote almost all
of the code, tests and docs, with one human steering and reviewing. Treat it as an experiment, not
as audited infrastructure. Some of the design borrows ideas from Anthropic's public article
[Redesigning source control](https://www.anthropic.com/research/redesigning-source-control).
Margincode is an independent project, **not affiliated with or endorsed by Anthropic**.

**Editions.** This repository is the free Community edition ([MIT](LICENSE)). It works with any
number of agents, with no seat limit. A hosted Commercial edition, **Margincode Cloud**, is planned but not available yet.
