# E2, 2026-10-30: scale and latency

Plan 6 E2, the scale part. `scholion-bench scale` imports one repo into a fresh bench daemon
(release build, Postgres 16 on the same 4-core machine, no semantic layer), times what an agent
waits on, then runs whole tasks with N agents at once: start, write a file, record, checkpoint,
context, submit, validate, review, merge. Raw numbers: `tokio/`, `django/` (`scale.json`,
`report.md`).

## Summary

| repo | code files | import + index | start (1 agent) | context p50 | why | merge (1 agent) | DB per live scope |
|---|---|---|---|---|---|---|---|
| tokio | 814 | 58 s | 13.5 s | 94 ms | 3 ms | 1.2 s | ≤ 200 MB |
| django | 2,939 | 129 s | 24.5 s | 268 ms | 4 ms | 2.7 s | ≈ 280 MB |

Under load (tokio, all agents released at once):

| agents | merged | wall | start p50 | merge p50 |
|---|---|---|---|---|
| 1 | 1 / 1 | 15 s | 14 s | 1.2 s |
| 8 | 8 / 8 | 127 s | 114 s | 5.8 s |
| 32 | **16 / 32** | 300 s | 265 s | 20 s |
| 100 | **17 / 100** | 329 s | 271 s | 28 s |

Django with 4 agents at once merged none: every start ran past the client's 600 s timeout.

## What limits it: every task copies the whole index

Starting a task copies `main`'s index into a scope of its own (`clone_scope`: symbols, the
reference graph, code chunks). That cost grows with the repo, not with what the task changes:

- **Time.** One start is 13.5 s on tokio and 24.5 s on django, nearly all of it this copy. Copies
  run in parallel transactions over the same tables and slow each other down: on django the
  `symbol_refs` insert took about 90 s per task with four at once.
- **Connections.** Each copy holds a pooled connection (16 by default) for its whole length. At 32
  and 100 agents the other requests waited over 30 s for one and failed with `pool timed out`.
  The daemon marks such a task abandoned, so these are lost starts, not slow ones.
- **Space.** The reference graph joins references by name, so common names (`new`, `poll`, `get`)
  fan out: tokio's 814 files hold about 900k edges per scope. A merged or abandoned task's
  scope is dropped, but Postgres keeps the space until a vacuum: the tokio database reached
  11 GB with 57 scopes live (`main` and the open tasks) and 104 dropped. Django's reached 2.3 GB
  with 8 live scopes, about 280 MB each.

Everything else is fast enough: context packs 94–268 ms, `why` 2–4 ms, record, submit, validate
and review in tens of milliseconds. Merges take a global lock, so N merges queue (p50 28 s at
100 agents), but each is 1–3 s of work.

Checkpoints that edit an existing file took 0.2–3 s at p50 and up to 13 s at p95 (re-resolving
that file's references against the graph); a checkpoint that only adds a file takes about 10 ms.

Workspaces are full checkouts and stay after merge: 1.4 GB for 161 tokio tasks, 590 MB for 8
django tasks.

## What this means for the claims

- C3 ("many agents share one repo") holds at 8 agents on a small repo and fails beyond that.
  The eight-agent stress runs used a small Python app, where the copy is cheap, so they could not
  show this.
- Packs, records and review are not the problem; task start is.

## Suggested fix (not in this change)

Make a task scope an overlay on `main`: a task stores rows only for the files it changed, and
reads fall through to `main` for the rest (excluding the paths it changed or deleted). Starting
a task then writes nothing to the index, and its size follows the task's own diff. That changes
every index query (symbols, references, chunks, lexical search), so it is its own plan with E1
and the determinism tests as guards. Smaller steps that help before it: cap concurrent copies
with a semaphore so starts queue instead of failing, and look at name-joined references (an edge
to every symbol of that name), which make up most of each copy.

## Not measured

- **The ~50k-file tier.** rust-lang/rust (64k files, 39.5k code files, shallow clone) did not
  finish importing. After 10 minutes 2,000 of its code files were indexed, with 5.75 million
  reference edges and a 1.4 GB database. Then the client's 600 s timeout closed the connection,
  and the daemon dropped the import with it: `POST /v1/repos` runs inside the request, so a
  disconnect cancels it, half done and without an error in the log. A large import should run
  as a job the client polls.
- **Token estimate accuracy per model family.** Claude's tokenizer is not public (it needs an
  API key for `count_tokens`), and this environment's proxy blocks downloading the DeepSeek
  tokenizer. The `calibrate` command does it with a key.
- **The semantic layer.** No embedder here; the outbox worker adds load that this does not show.
