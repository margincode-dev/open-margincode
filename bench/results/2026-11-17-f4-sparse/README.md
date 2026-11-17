# F4: this repository's `.av/sparse`, before and after (2026-11-17)

This repository now lists `bench/results/` and `bench/retrieval/results/` in `.av/sparse`: 4,670 of
its 5,173 files. Sparse workspaces themselves are #42; this measures what the list does here, on
the layout the dogfood host uses (Docker Desktop, the daemon's workspaces on a Windows bind mount,
`AV_EXEC_BIT=ignore`).

## How

A throwaway daemon of image `margincode-dev:86d5ff0` (`main` after #42), workspaces bind-mounted
from D:, removed afterwards. Two repos imported into it from `main` at `86d5ff0`: `om-full` as it
is (the control) and `open-margincode` with this list committed on top. Five starts of each,
alternating, with `../2026-11-17-f4-storage/baseline.py 1` (`AV_REPO` names the repo); each start
is followed by a checkpoint of three edited files and an abandon. Raw rows: `runs.jsonl`.

## Numbers (median / worst of five)

| | without the list | with the list |
|---|---|---|
| start, client (s) | 89.95 / 91.42 | 6.54 / 6.96 |
| workspace copy (ms) | 86,405 / 87,197 | 5,975 / 6,383 |
| inline count (ms) | 3,627 / 4,146 | 468 / 489 |
| files / bytes in the workspace | 5,177 / 57.9 MB | 508 / 5.4 MB |
| checkpoint of 3 files (s) | 1.27 / 1.43 | 0.23 / 0.24 |

Every checkpoint and abandon succeeded. The copy is still about 85 files a second: it is per
file, and the list takes 90 % of the files away. On the bind mount this is now faster than the
volume layout of `../2026-11-17-f4-storage/` (15.88 s, nearly all of it the client cloning the
whole tree, which the list does not change), so the dogfood host stays on the bind mount.

## Checked in a sparse workspace, on the same daemon

- Listed paths are not on disk; `bench/retrieval/ci/`, `bench/retrieval/public.jsonl` and the
  `bench/tasks-*` the tests read are.
- The bar (fmt, clippy `-D warnings`, `cargo test --workspace`) passes in it: no test reads the
  archives.
- Task B adds a file under `bench/results/` and merges; task A syncs: B's file is not written to
  A's disk. A edits `README.md`, adds a file under `bench/results/`, records a note anchored to a
  listed file, builds a pack, checkpoints, and merges. `main` then has both new files, A's edit,
  and the old archives (`av show`); a full checkout of it has the control's 5,173 files plus
  exactly the list and the two new files.

## Limits

- One machine, one repository, n = 5 per arm.
- Until a workspace can be widened (16b), an agent in one cannot read or edit a file already
  under the list: read it with `av show PATH`, or write results to a new directory.
- Each snapshot works the patterns out from the commit's tree; at this size it is inside the
  0.23 s checkpoint.
