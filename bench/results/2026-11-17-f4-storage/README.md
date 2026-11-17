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

## What it means for the plan

The storage PRs that follow (workspaces on a volume, sparse workspaces, copying outside the lock)
are measured against these numbers: same script, same repository, five starts, median and worst.
A change counts when the copy (`ms`) and the client time both fall; `files` and `bytes` say
whether it did so by writing less.
