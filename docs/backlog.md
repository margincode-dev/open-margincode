# Backlog of observations

Errors and anything else that did not fit, written down when they were seen so they can be taken
up later (`constitution.md`, rule 5). Newest first. An entry stays when it is fixed, marked with
the pull request that fixed it. No secrets, keys or local paths: this repository is public.

Each entry: the date; where (command, test, commit); what was seen, exactly; what was ruled out;
the suspected cause, marked as a guess; the status.

## 2026-11-18: two result files were stored with CRLF line endings

- **Where.** `bench/results/2026-11-17-f4-storage/baseline.jsonl` and `volume-checkout.jsonl`
  (#46), found by the Cloud's vendor sync to `d0102c0`.
- **Seen.** Both blobs on `main` had CRLF endings (6 lines each). `.gitattributes` asks for LF,
  so the Cloud's `git add` stored them LF, and `scripts/sync-margincode.sh --check` then found the
  vendored tree's sha256 different from the one it recorded.
- **Ruled out.** Not the sync script: `git archive` of `main` gives the CRLF bytes, and they are
  the only two blobs on `main` with a CR.
- **Guess.** They were written on Windows and committed through `av` (jj), which does not apply
  `.gitattributes`, so nothing normalized them.
- **Status.** Fixed in #64 (both stored LF). `av diff --check` now reports an added CRLF line
  (branch `claude/cloud-capabilities-offline-22d48v`); a commit that runs no check can still
  carry one.

## 2026-11-18: the patch route answered with the previous head

- **Where.** `GET /v1/tasks/{id}/patch?from=main`, called by hand right after `av validate` or
  `av checkpoint` returned, while preparing fix-plan PR 22 (F29) for review.
- **Seen.** On two occasions the patch did not hold the edit the checkpoint had just taken: it was
  the previous head's. The same request a few seconds later held it. `av validate` had already
  printed the new head.
- **Ruled out.** Nothing yet: not reproduced on purpose, and no test reads the patch right after a
  checkpoint.
- **Guess.** The route reads the task's head, or the workspace's files, before the checkpoint's
  write is visible to it.
- **Status.** Open. A wait of a few seconds before reading the patch has avoided it since.
  Not reproduced in the daemon itself (2026-11-18): a throwaway test in `tests/remote.rs` that
  edited a file, then called `checkpoint` or `validate` and at once `GET .../patch?from=main`,
  60 rounds, three runs, always got the new head (`X-Av-Head`) and the edit. Both routes write
  the head in one transaction before they answer. So the cause is more likely outside one
  daemon process's ordering: the client (which head it read the patch for), or the Windows bind
  mount the edits were made on (a write the snapshot did not see yet).

## 2026-11-18: the Docker image cannot push a backup over SSH

- **Where.** `deploy/Dockerfile` (the apt line), `deploy/docker-compose.yml` (the SSH key and
  `AV_BACKUP_SSH_COMMAND` comments), found reviewing fix-plan PR 24 (F7, F8, F19, F20).
- **Seen.** The image installs `git ca-certificates curl` with `--no-install-recommends`, and
  `ssh-client` is only a Recommends of Debian's `git`: `which ssh` finds nothing in the image. With
  `openssh-client` added, ssh still exits "No user exists for uid 4242" for a UID with no passwd
  entry, and compose runs the daemon as the host `${UID}` (its own comment says that UID has no
  passwd entry). So the compose file's SSH backup comments describe a setup that cannot work.
- **Ruled out.** Lite mode: the daemon runs on the host and uses the host's ssh.
- **Done for now.** `docs/reference.md` ("Getting main to GitHub (self-hosted)") sends the Docker
  stack to a bare repository on a mount and a push from the host.
- **Guess at a fix.** Add `openssh-client`, and give ssh a user it accepts (for example a passwd
  entry written at start, or `nss_wrapper`), then a test that pushes a backup over SSH from the
  image. Or drop the compose comments.
- **Status.** Fixed on `claude/cloud-capabilities-offline-22d48v`: the image installs
  `openssh-client` and `libnss-wrapper`, and its entrypoint (`deploy/entrypoint.sh`) gives a UID
  without a passwd entry one through nss_wrapper before it starts the daemon. Checked on the
  image's runtime layer (same base and packages, a stand-in daemon): as UID 4242, `ssh` without
  the entrypoint exits "No user exists for uid 4242"; with it, a `git push` over SSH to an sshd
  container succeeded. Not checked: the full compose stack (`scripts/smoke.sh`), whose images
  Docker Hub rate-limited in the session.

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

## 2026-11-18: a two-second lease ran out inside a routing test under load

- **Where.** `margincode-daemon` `tests/routing.rs`
  `the_reviewer_who_requested_changes_does_not_rescue_the_task` (line 590), in the full
  `cargo test --workspace` run for fix-plan PR 21 (F6).
- **Seen.** Right after grok's `request_changes`, which renews the owner's lease, `next` for
  `codex` returned the task `from: abandoned` where the test expects nothing offered. Alone the
  test passed five runs out of five (5.9 s each).
- **Ruled out.** Not F6's change: F6 touches the merge message and the commit author, not
  leases, reviews or `next`.
- **Guess.** The test's lease is 2 s; with the whole workspace's tests running, the time from the
  renewal to the two `next` calls passed 2 s. A longer lease with the sleeps scaled to it would
  hold.
- **Status.** Fixed on `claude/cloud-capabilities-offline-22d48v`: the test's lease is 5 s and
  its sleeps 5.5 s, so a loaded run has 5 s from the renewal to the two `next` calls.

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
- **Status.** Answered (2026-11-18, the gate with a per-task print at #48's parent `6721a56` and
  at `e515440`): at 600, `cache-key-order` lost its one signature of two (0.5 to 0; every other
  task unchanged, `token-skew` was already 0); at 300, `aging-due-today` gained its file and its
  signature (0 to 1). Why `cache-key-order`'s signature no longer fits at 600 is not looked at
  yet. A per-task line in the gate's report would answer the next such question at once.

## 2026-11-18: `av` on Windows does not read a Git Bash path

- **Where.** `av record --json FILE` in Git Bash with `av.exe`.
- **Seen.** `av: cannot read /c/Users/.../records.json: The system cannot find the path
  specified. (os error 3)`. The same file passed as a Windows path (`cygpath -w`) was read.
- **Guess.** `av.exe` is a Windows program and Git Bash converts paths only for arguments it
  recognises as paths; with `MSYS_NO_PATHCONV=1` (which `docs/reference.md` recommends for `/import/...`)
  it converts none.
- **Status.** Documented in #60 (F2, F13): `docs/reference.md` says Git Bash converts only
  arguments and that a Windows program needs `C:/...` (`cygpath -m`). `av` still reads such a path
  as given.

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
