# GitHub pull requests: the task's records where reviewers read

A task's decisions, dead ends, notes and review findings, plus what earlier tasks recorded about
the files it changes, as one comment on the pull request:

```bash
av notes                    # print the block for the task of this workspace (Markdown)
av notes --pr               # post it on the current branch's pull request, or update it
av notes --pr 123           # a given pull request
av notes demo/atomic-save --pr 123 --gh-repo owner/name   # any task, from anywhere
av notes --format json      # the block plus the task id, record ids and files
```

`av notes` only reads the daemon (`GET /v1/tasks/{id}`, `/records`, and `/records/at` for each
changed file, with no session, so nothing is marked as sent). `--pr` goes through the
[GitHub CLI](https://cli.github.com/) (`gh`, logged in, or `GH_TOKEN`; `AV_GH` names another
program): it lists the pull request's comments, edits the one that already holds this task's
block, and posts a new one only when there is none. A block that has not changed is not sent
again, so running it after every submit, review or merge is safe. The task is `--task`, the
nearest `.av-task`, or the first argument: an id, a slug (`--repo` when two repos share it), or
`repo/slug`.

Exit codes: `0` printed, posted, updated or already up to date; `1` the daemon could not be
reached, the task was not found (or the slug is ambiguous), or `gh` failed (its message is
printed); `2` a usage error.

## Why the CLI posts, not a GitHub Action

The daemon runs on the developer's machine (`av up`, `127.0.0.1:7420`), so a GitHub-hosted
runner cannot reach it. Whoever can reach it posts: the developer after `av submit` (or an
agent, as the last step of its task), or a self-hosted runner on that machine or its network.
The comment is plain Markdown, so the pull request keeps it after the daemon is gone.

**By hand or from an agent** (no CI at all):

```bash
av submit
av notes --pr               # gh finds the pull request of the current branch
```

How a task's change reaches a pull request is up to you: push the task's work as a branch, or
let the daemon push main to its own branch and open the pull request from that
(`av repo backup NAME --remote URL --branch av-main`). For the workflow below, name the branch
after the task's slug or put `Scholion-Task: <id or repo/slug>` lines in the description (one
per task, when a pull request carries several merges).

**From CI on a self-hosted runner** that can reach the daemon and has `av` and `gh` installed:
copy [`scholion-notes.yml`](scholion-notes.yml) to `.github/workflows/`, label the runner
`scholion`, and set the repository variable `AV_URL` (and the secret `AV_TOKEN` when the daemon
requires one). It runs on every push to the pull request and on description edits, posts or
updates one comment per task, and skips pull requests from forks: never run a fork's code on a
self-hosted runner. A branch that names no task (and no `Scholion-Task:` line) gives a notice,
not a failure.

## The block's format

```markdown
<!-- scholion:notes v1 task=3f0c2b1a-0000-4000-8000-000000000001 -->
### Scholion: why this change

**Atomic save** (`pantry/atomic-save`, in review; approved by codex, owner claude)

> Write pantry.json atomically so a crash cannot truncate it.

**Decisions**
- Write to a temp file in the same dir, then os.replace (`rec:b2000000`, claude) · `pantry/store.py:save`
  > os.replace is atomic on POSIX and Windows

**Dead ends**
- tried os.rename; fails on Windows if the target exists (`rec:c3000000`, claude) · `pantry/store.py`

<details><summary>Recorded earlier about the changed files (1 of 3, 1 record)</summary>
...
</details>

<sub>Written by `av notes` from the task's records. Records: rec:b2000000 rec:c3000000. ...</sub>
<!-- /scholion:notes -->
```

- The block starts with `<!-- scholion:notes v1 task=<task uuid> -->` and ends with
  `<!-- /scholion:notes -->`. One block per task: a comment is found by its task's start marker,
  and only the text between the markers is replaced, so text around it and other tasks' blocks
  stay. `v1` changes only if the markers do.
- Sections, each only when it has records: Decisions, Dead ends (`attempt`), Notes, Review
  findings, Handoffs (done / remaining / broken / questions), Checks (`validation`). Superseded
  records are left out. A body shows as quoted lines, clipped to 600 characters
  (`av expand rec:…` prints the rest).
- "Recorded earlier about the changed files" is what other tasks anchored to the files this task
  changes (the first 20 files), the same lines the Claude Code `PreToolUse` hook shows an agent
  opening that file.
- The footer lists the record ids shown.
- Record text cannot close the markers (`<!--` and `-->` are escaped) and a `@name` in it does
  not notify anyone (a word joiner follows the `@`).
- The full example is `crates/scholion-cli/tests/golden/notes.md`.

## `av why` and pull requests

`av why path:line` is a blame of main: it answers once the task is merged, naming the task, its
goal and its decisions. Before the merge, a pull request's lines are not on main yet, so the
block's pre-merge answer to "why is this file the way it is" is the records earlier tasks
anchored to the changed files.
