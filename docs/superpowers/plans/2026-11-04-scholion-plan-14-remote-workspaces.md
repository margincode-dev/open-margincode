# Scholion Plan 14: Remote workspaces by checkpoint-by-patch

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task.

**Goal:** An agent on another host than the daemon works in a local checkout of its task and
sends the daemon one patch per checkpoint. The daemon applies it to the workspace it still owns,
snapshots as usual, and answers with what the agent needs to stay in sync. This is Plan 13's
Phase 3 (`agentvcs/docs/superpowers/plans/2026-11-04-agentvcs-plan-13-rollout.md`): hosted Teams
needs it, and so does a customer-hosted runner with a hosted brain.

**Status (2026-11-04):** Tasks 1–8 done. Measured: a remote checkpoint's median round trip
is 1.5× a local one (31.5 ms vs 21 ms), under the 2× target. Merged to `main` (#101); the
branch `remote-checkpoint` is gone.

## The problem

Today every task workspace is a directory under the daemon's `ws_root`: `task start` creates it,
`checkpoint` snapshots it (jj-lib, in process), and the `.av-task` file the CLI writes there is how
every later command finds its task. An agent must therefore run on the daemon's host, or on a
filesystem the daemon can read. A laptop agent against a daemon in a hosted cell, or a runner on a
customer's machine against a hosted daemon, cannot work at all.

## The invariant

Nothing about who owns what changes:

- **The daemon owns and snapshots every workspace.** A remote checkpoint ends in the same
  `snapshot_and_index` as a local one: the same jj commit, overlay update, `set_head` and
  `checkpoint` event. Records, packs, submit, review and merge read the daemon's workspace and
  store exactly as before.
- **The daemon runs no new repository command.** Today it runs git only against its own bare
  store (`clone` at import, `push` for backups) and never in a workspace; a checkpoint is jj-lib in
  process. The patch is applied in process too (`scholion-vcs/src/patch.rs`: parse, check, compute
  every file's new content, then write); it never shells out to `git apply`. The patch for a pull
  is rendered from the store by jj-lib (`diff_presentation`) plus our own binary encoding.
- **Community is not made worse.** A local checkpoint is unchanged (no `base`, the body is
  ignored as before); no limit, seat or cost check is added. Remote checkouts are Community.
- **Auth is the existing `AV_TOKEN`**, fencing the existing `X-Av-Generation`.

The checkout runs git on the agent's host (the CLI already starts processes there: `validate`,
`watch`, `crates_scope`).

## API

### `POST /v1/tasks/{id}/checkpoint?base=<change>`

Body: a patch as `git diff --binary --full-index --no-renames --src-prefix=a/ --dst-prefix=b/`
writes it, content type `text/x-diff` (raw bytes: a text file need not be UTF-8, so not JSON).
`base` is the daemon head the patch was made against.

Under `lock_task` and `fence`:

1. **Snapshot first.** Edits made in the workspace on the daemon's host since the last checkpoint
   are recorded (`api::index_head`), so they make the base stale instead of being mixed in.
2. **Base check.** The base is current when it is the head or has the head's files
   (`Vcs::tree_id` equal; a fresh task's first base is its trunk commit and the workspace's commit
   is an empty child of it). Otherwise **`409`** with `{error, head_change, base, tree}` and header
   `X-Av-Head`. An unknown base is `404` ("clone the task again").
3. **Apply** (`Vcs::apply_patch`, under the per-repo jj lock): every file of the patch is worked
   out against the files on disk before any is written; a patch applies whole or not at all.
   Strict: no fuzz or offsets (the base was checked), `index` blob ids must match, a new file
   must not exist and a changed one must. Refused with **`400`**: malformed patches, renames and
   copies (send `--no-renames`), symlink or submodule changes, a `Binary files … differ` notice
   (send `--binary`), a path twice, paths that are absolute, contain `..`, `.jj`, `.git`, or are
   `.av-task`, and any write through a symlink or into a non-directory. Text, binary `literal` and
   `delta` hunks, new and deleted files and `old mode`/`new mode` (the executable bit) are
   supported; an emptied directory is removed.
4. **Checkpoint** as a local one does. `200`: `{head_change, changed, warnings}` (a local
   checkpoint's answer) plus `applied` (paths written or deleted, in patch order) and `tree` (the
   git tree id of the new head, `null` when conflicted), which the client compares with its own
   `git write-tree` to know it holds exactly the daemon's files.

**Size:** at most `MAX_PATCH_BYTES` = 64 MiB per body (`413` above, axum's `DefaultBodyLimit` on
that route only); a binary hunk inflates to at most 256 MiB (the snapshot's own cap on new files).

Without `base` the route is the local checkpoint, unchanged (the MCP tool and old CLIs post `{}`).

### `GET /v1/tasks/{id}/patch?from=<change>`

The task's recorded head as a patch from `from` (`main`, `task:<slug>` or a commit id; without
`from` the empty tree, which is a whole checkout). `git apply` accepts it (tested); conflicted
files carry the markers the working copy writes (same marker style and length). Headers
`X-Av-Head` and `X-Av-Tree`. Reads immutable commits only: no lock, no snapshot, so a reader (a
reviewer on another host) changes nothing. jj's commit ids are git commit ids and its tree ids
git tree ids, which is what lets the client check its trees against the daemon's.

### On a `409`, the client

1. fetches `GET …/patch?from=<its base>` and builds the new base commit from its old one in a
   separate index (`git apply --cached`), checking the tree against `X-Av-Tree`;
2. saves its local edits (as a commit and as a patch), moves the checkout onto the new base
   (`reset --hard`), and puts the edits back with `git apply --3way`;
3. sends the checkpoint again, once. Lines both sides changed get conflict markers; the command
   exits non-zero naming the files and the agent resolves them and checkpoints again (the same
   path as `av sync`'s conflicts). If the edits do not go back on at all, the message names the
   commit that holds them.

What makes a base stale: a checkpoint on the daemon's host, `av sync` (the rebase moves the
head), `undo`/`revert` restoring the workspace, a claim by another agent (whose writes the fence
refuses anyway), or a second remote writer.

## CLI

- `av remote clone TASK [DIR]`: `git init` in DIR (default: the slug), `.git/info/exclude` with
  `/.av-task` and `/target/` (as `AGENT_IGNORES` in a workspace), `.git/info/attributes`
  `* -text -filter -ident` (the checkout's bytes are the daemon's; also `core.autocrlf=false` on
  every git call), the whole head from `GET …/patch`, one commit, `.av-task` with
  `remote: {base, commit}`.
- `av task start --checkout DIR`: start, then clone, instead of writing `.av-task` into a
  workspace path that does not exist on this host.
- `av checkpoint` in a checkout: `git add -A`, `git diff --cached … <commit>`, post with the
  `.av-task` generation; on `200` the index's tree becomes the new local commit (when it is the
  daemon's `tree`; otherwise the commit is rebuilt from the daemon's patch and the difference
  stays a local edit); on `409` the pull above, then once more.
- `av remote pull`: the pull alone ("up to date" when nothing moved).
- The same checkpoint runs in the `post-edit` and `stop` hooks and in `av validate`, and before
  `submit`, `record` (anchors are hashed on the daemon), `handoff` and `sync` (which then pulls the
  rebased head, markers included). The hooks need no change in how they are installed.
- One checkout lock (`.git/av-lock`, `scholion_agentrun::slots`), since hooks checkpoint while
  commands run. `CliError::Api` keeps the daemon's JSON body, for the `409`'s head.

## Out of scope

- **Multi-host dispatch**: choosing a host for a task, starting agents remotely, moving a task's
  workspace between daemons. `av watch` on the remote host plus `--checkout` is the manual form.
- Two writers on one task from different hosts (works, but each keeps pulling the other's head).
- Symlink changes from a checkout, submodules, renames as renames (they arrive as delete + add).
- `av claim` and `av next` writing a checkout (they still write `.av-task` into the daemon's
  workspace path); MCP tools for remote checkouts (MCP agents on the remote host use the hooks).
- Large initial checkouts: the first patch holds the whole tree in memory on both sides. A later
  step can start from a git clone the host already has (`from=<base>` works with any commit the
  daemon knows, and jj's commit ids are git's).

## Tasks (TDD each: failing test, code, `cargo fmt`, `clippy -D warnings`, `cargo test --workspace`)

1. **Patch module** `scholion-vcs/src/patch.rs`: `render`, `apply`, `git_blob_id`. Tests in
   `crates/scholion-vcs/tests/patch.rs` use git as the oracle: a `git diff --binary` (with a
   delta, an exec bit, CRLF, Latin-1, no final newline, empty files, a quoted path) applies to the
   files it was made from; a rendered patch applies with `git apply` and its tree is git's; from
   nothing; refusals (unsafe paths, renames, symlinks, bare binary notices, mismatches, garbage).
2. **Vcs methods** (`scholion-core/src/traits.rs`, defaults refuse): `apply_patch`, `diff_patch`,
   `tree_id`, in `JjVcs`. Tests in `tests/vcs.rs`: a patch lands and a snapshot records it (exec
   bit, emptied directory removed); whole or nothing; never through a symlink; `diff_patch` from
   nothing rebuilds the source's git tree; between two heads it applies with git.
3. **Routes** (`scholion-daemon/src/remote.rs`, `api::checkpoint`, `api::index_head`). Tests in
   `tests/remote.rs`: a patch checkpoint lands like a local one (head, changed, file at
   `task:<slug>`, a later local checkpoint is a no-op, the `checkpoint` event); an empty patch on a
   fresh task is accepted; a stale base is `409` with the head, and so is drift on disk; the patch
   route starts and resyncs a checkout; bad patches are `400` and write nothing; unknown base
   `404`; over the limit `413`; a lost claim `422`.
4. **CLI** `scholion-cli/src/remote.rs`, `av remote clone|pull`, `task start --checkout`, the
   checkpoint dispatch in `main.rs` and `hook.rs`. Tests in `tests/cli.rs`, each driving `av`
   from a directory of its own against a daemon whose workspace is elsewhere: clone, edit (text,
   new, deleted, binary; `target/` left out), checkpoint, the daemon's workspace and `av show` have
   it, an incremental checkpoint, a same-host checkpoint then a remote one (409, pull, resend),
   `remote pull` up to date; same-line edits leave markers and a resolved checkpoint lands;
   `submit` sends the edits first; `task start --checkout`.
5. **Docs**: `docs/reference.md` ("An agent on another host", the main path), `CLAUDE.md`,
   `CHANGELOG.md`, `HANDOFF.md`.
6. **Measurement script** `scripts/checkpoint-timing.sh DIR FILE [N]`.
7. **fmt, clippy, the workspace suite.**
8. **Measure** (below).

## Measurement

**Exit (Plan 13 Phase 3):** one checkpoint round trip from another host, measured; target under
2× the local checkpoint time.

Method: the daemon (release build) runs in one container with Postgres in a second; the remote
checkout lives in a third container on the same Docker network, so the round trip crosses a
network stack (not a WAN). The repo is this one (Scholion's 531 tracked files, 7.2 MB, imported as a fresh one-commit repo). Each side
edits one file and runs `av checkpoint` ten times (`scripts/checkpoint-timing.sh`); the local
runs are in the task's workspace on the daemon's container, the remote runs in the checkout.

Results (2026-11-04, release build, all three containers on one Windows 11 / Docker Desktop
machine; each run appends one line to `README.md`):

| | runs | median | notes |
|---|---|---|---|
| local, `av checkpoint` in the workspace | 10 | 21.5 ms | first run 6089 ms (cold start: the daemon's first checkpoint on the repo, run before the remote one), then 21–46 ms |
| remote, `av checkpoint` in the checkout | 10 | 32 ms | first run 54 ms, then 30–35 ms |
| remote, second round (run first) | 20 | 31.5 ms | |
| local, second round | 20 | 21 ms | |
| remote after a same-host edit (`409`, pull, resend) | 5 | 77 ms | first 524 ms; three round trips plus `git apply --3way` |

**Ratio: 31.5 / 21 = 1.5×, under the 2× target.** The extra ~10 ms is the client's git work
(`add -A`, `write-tree`, `diff --cached`, `commit-tree` over 531 files) plus one HTTP round trip
and the in-process apply; the daemon's snapshot is the same as the local one. `av remote clone`
of the whole tree took 192 ms. Not measured: a WAN round trip (add its latency once per
checkpoint, three times on a `409`) and large trees (the clone patch is in memory on both sides).
The measurement also found a bug, fixed with a test: `av remote clone` and `task start --checkout`
with a relative `DIR` built the separate index at a doubled path.

## Verification

- `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings &&
  cargo test --workspace` green.
- By hand: `av up` on one machine, `AV_URL=http://that-host:7420 av remote clone <task> co` on
  another, edit, `av checkpoint`, then `av show <path> --at task:<slug>` shows the edit.
