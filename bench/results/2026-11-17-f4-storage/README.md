# Where a task start's time goes (F4 baseline)

2026-11-17. No agent ran and nothing was spent. This is the baseline the dogfood fix plan asks for
before any change to how workspaces are made (F4, F21): a start of this repository took about 90 s
on the dogfood host, and until `workspace:ready` (#39) nothing said where the time went.

## Question

Of a start's wall time, how much is the workspace copy (`Vcs::create_workspace`, jj's checkout of
`main`), how much the task's index overlay, and how much the count that `workspace:ready` adds?
And what does a checkpoint cost on the same layout?

## Method

`baseline.py` runs five starts of `open-margincode` against the dogfood daemon, one after another.
Per run it:

1. times `av task start` on the client (the whole request, as an agent sees it);
2. reads that start's `workspace:ready` event from `GET /v1/events`: `ms` (the copy),
   `overlay_ms`, `files` and `bytes` (what the workspace holds, `.jj/` in, `target/` out) and
   `count_ms` (what counting them took);
3. appends a line to `CHANGELOG.md`, `README.md` and `CONTRIBUTING.md` and times `av checkpoint`;
4. abandons the task, so nothing is kept.

Setup, layout (a) of the plan, the one the dogfood host runs today:

- daemon: image `margincode-dev:6421954` (`main` at #39), lite mode, one container on Docker
  Desktop (WSL 2 backend, kernel 6.6, 16 CPUs, 12 GB);
- workspace root: a folder on the Windows host's NTFS volume, bind-mounted into the container
  (`AV_WS_ROOT=/ws`, `AV_WS_HOST_ROOT` set so the client gets a Windows path), `AV_EXEC_BIT=ignore`;
- client: native `av.exe` on the same host;
- the repository at `main`: 5,171 files and 57.8 MB once checked out with `.jj/`.

```bash
python baseline.py 5 <a directory av resolves the repo from>   # AV_URL defaults to http://127.0.0.1:7420
```

## Result

`baseline.jsonl` holds the five raw lines and the summary line.

| | median | worst (of 5) |
|---|---|---|
| `av task start`, client wall time | 92.99 s | 93.45 s |
| workspace copy (`ms`) | 86,633 ms | 87,282 ms |
| index overlay (`overlay_ms`) | 1 ms | 2 ms |
| counting the workspace (`count_ms`) | 3,591 ms | 3,825 ms |
| files / bytes in the workspace | 5,171 / 57,810,890 | same in every run |
| `av checkpoint` of 3 changed files | 3.13 s | 3.32 s |

All five checkpoints and abandons succeeded.

- **The copy is the start.** 86.6 of 93.0 s (93%) is jj writing the checkout into the bind mount:
  about 60 files or 0.7 MB a second. At that rate the cost is per file, not per byte; whether it
  is the bind mount's is for layout (b), workspaces on a volume, to measure (not measured here).
- **The overlay is free** (1 ms): the task's index starts empty and shares the repo's.
- **The count is not free on this layout.** Walking the new workspace takes 3.6 s, because every
  `stat` crosses the same bind mount. It is about 4% of the start and stays inline so the
  event's place in the feed is fixed; a change that makes the copy cheap should recheck it.
- What is left (about 2.8 s) is the request itself, jj's workspace bookkeeping and the client.
- A checkpoint of three small files takes 3.1 s: jj snapshots the workspace, which also walks
  the bind mount.

## Layout (b): workspaces on a volume, the client in its own checkout

Measured the same day for the Windows recipe (`docs/reference.md`, "Workspaces on a volume"),
with `av next --checkout` in the same change. A second daemon from the same image, on another
port, with its workspace root inside its data volume (ext4 in Docker's VM) instead of a bind
mount, imported `main` at `719761f` (5,174 files and 57.8 MB in a workspace). The client starts
each task with `--checkout`, so `av task start` also makes the Windows checkout (`av remote
clone`), and edits and checkpoints there (the checkpoint goes to the daemon as a patch):

```bash
AV_URL=http://127.0.0.1:7421 AV_CHECKOUT_ROOT=<an empty Windows folder> python baseline.py 5 .
```

`volume-checkout.jsonl` holds the raw lines. Median / worst of 5:

| | (a) bind mount | (b) volume + checkout |
|---|---|---|
| `av task start`, client wall time | 92.99 / 93.45 s | 15.88 / 16.65 s |
| workspace copy (`ms`) | 86,633 / 87,282 ms | 376 / 407 ms |
| counting the workspace (`count_ms`) | 3,591 / 3,825 ms | 8 / 8 ms |
| `av checkpoint` of 3 changed files | 3.13 / 3.32 s | 0.60 / 0.86 s |

- The copy is 230 times faster on the volume: the bind mount's per-file cost was the start.
- Of the 15.9 s left, the daemon's part is under a second. Timed apart once: a start without
  `--checkout` 0.58 s, the whole tree as a patch (`GET /v1/tasks/{id}/patch`) 0.63 s, and
  `av remote clone` 13.4 s, so about 12.8 s is git on Windows writing the 5,174 files of the new
  checkout. That is now the cost to cut. Sparse workspaces (`.av/sparse`) do not cut it by
  themselves: they leave paths off the daemon's workspace, and the client's checkout is still
  cloned from the whole tree (corrected; this line said they did).
- With the copy this cheap, the count stays inline.

## What it means for the plan

The storage PRs that follow (workspaces on a volume, sparse workspaces, copying outside the lock)
are measured against these numbers: same script, same repository, five starts, median and worst.
A change counts when the copy (`ms`) and the client time both fall; `files` and `bytes` say
whether it did so by writing less.
