# Backlog of observations

Errors and anything else that did not fit, written down when they were seen so they can be taken
up later (`constitution.md`, rule 5). Newest first. An entry stays when it is fixed, marked with
the pull request that fixed it. No secrets, keys or local paths: this repository is public.

Each entry: the date; where (command, test, commit); what was seen, exactly; what was ruled out;
the suspected cause, marked as a guess; the status.

## 2026-11-18: M2 stage B, three stops before the first session

- **Where.** M2 stage B (`bench/results/2026-11-13-standing-and-dreams/m2-stage-b/checks/`).
- **Seen.** (1) The first `dream-freeze` stopped at its first agent call: Claude Code answered
  "Invalid API key" and a direct models request returned 401; the owner replaced the key and the
  freeze ran again from a fresh seed ($0 spent). (2) The smoke's preflight refused twice:
  `first-packs` and the freeze leave run and workspace directories open to the agent user, and
  `useradd -m` gives `benchagent` a home it can write. Closed with `chmod 700` and
  `usermod -d /nonexistent benchagent`; `m2bchain.sh` now closes `ws/*` too between the arms.
- **Ruled out.** No session ran in either state; the harness refused before launching.
- **Guess.** (2) is setup for a new container: M0's container had been set up that way already.
- **Status.** Worked around. A harness recipe for a new container could name both steps.

## 2026-11-18: the Workers Builds preview of #49 failed

- **Where.** The Cloudflare Workers Builds preview check on #49 (`constitution.md` and this file;
  build `63832ff8`).
- **Seen.** The preview build failed. `main`'s production build after #49 merged passed.
- **Ruled out.** Not the change's content, as far as can be told without the log: #49 touched only
  Markdown, and the same tree built on `main`.
- **Guess.** Something in the preview environment, not the repository. The build log was not read.
- **Status.** Open. Read the build's log before the next preview fails the same way.

## 2026-11-18: which retrieval task lost a signature at 600 tokens

- **Where.** `margincode-bench` `tests/retrieval_gate.rs`, on #48 (F17).
- **Seen.** The pack's `symbols_sig` mean at budget 600 fell from 0.833 to 0.778 (half of one
  task's gold signatures, of 9), while at 300 it rose from 0.333 to 0.444 and `files_code` from
  0.444 to 0.556. The floors were re-set from the new means.
- **Ruled out.** Not the goals' length: the fixture's goals are one line each.
- **Guess.** The task handle is now reserved before the section shares, which moves a few tokens
  between sections at tight budgets.
- **Status.** Open. The gate prints means only; run `margincode-bench retrieval` on the fixture to
  see which task changed.

## 2026-11-18: `av` on Windows does not read a Git Bash path

- **Where.** `av record --json FILE` in Git Bash with `av.exe`.
- **Seen.** `av: cannot read /c/Users/.../records.json: The system cannot find the path
  specified. (os error 3)`. The same file passed as a Windows path (`cygpath -w`) was read.
- **Guess.** `av.exe` is a Windows program and Git Bash converts paths only for arguments it
  recognises as paths; with `MSYS_NO_PATHCONV=1` (which `docs/reference.md` recommends for `/import/...`)
  it converts none.
- **Status.** Open. Related to the Git Bash hint in the fix plan's PR 23 (F2, F13).

## 2026-11-17: a checkpoint during another task's start took 2.2 s once

- **Where.** The measurement for #46 (`bench/results/2026-11-17-f4-lock/`), on a throwaway daemon
  on the Windows bind mount.
- **Seen.** With the write outside the repo lock, seven of eight checkpoints during another start
  took 370-378 ms; one took 2,236 ms. On `main` before the change they took 4.7-9.7 s.
- **Ruled out.** Not reproduced in five more runs, or on the dogfood daemon (369-381 ms).
- **Guess.** The host's disk, or the snapshot meeting the start's index overlay.
- **Status.** Open.

## 2026-11-17: checkpoints after widening every archive take 4.2 s

- **Where.** `av widen` on a sparse workspace of this repository (#45).
- **Seen.** Widening one result directory took 14.9 s and checkpoints after it stayed under a
  second. Widening all of `bench/results/` made each checkpoint 4.2 s.
- **Guess.** The snapshot walks every widened file on disk; a path that a sparse pattern already
  covers could skip the walk.
- **Status.** Open.
