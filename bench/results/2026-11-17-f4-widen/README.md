# F4: `av widen` on this repository (2026-11-17)

A throwaway daemon built from this change, workspaces on a Windows bind mount (Docker Desktop,
`AV_EXEC_BIT=ignore`), this repository at `95efa42` with its `.av/sparse`. One task; three
checkpoints of an edited `README.md` after each step; `POST /v1/tasks/{id}/widen` timed from the
client. One run (n = 1).

| step | widen | written | files on disk | checkpoints (ms) |
|---|---|---|---|---|
| start | | | 505 | 355, 384, 353 |
| widen `bench/results/2026-11-first-pack` | 14.9 s | 697 | 1,202 | 1,287, 930, 934 |
| widen `bench/results` | 9.5 s | 573 | 1,775 | |
| widen `bench/retrieval/results` | 64.8 s | 3,402 | 5,177 | 4,123, 4,704, 4,215 |

- The first pack said `Not on disk: bench/results/, bench/retrieval/results/ (...)`; each widen
  answered the prefixes still off disk (both, then one, then none), and after the last the pack
  had no such line.
- A widen writes about 47 files a second, the bind mount's per-file cost (the start's copy is
  about 85 a second). It runs under the task's lock.
- With one result directory widened a checkpoint stays under a second. With everything widened it
  is 4.2 s, against 1.27 s for a workspace with no list (`../2026-11-17-f4-sparse/`): before each
  snapshot the patterns are worked out again, and every file on disk under a listed prefix is
  walked and becomes its own pattern, even where a widened directory already covers it. Skipping
  what a current pattern covers would remove that walk; not done here.
