# Guard 5: the retrieval runs behind the table in `../README.md`

The output of `scholion-bench retrieval` on `bench/retrieval/ci` (semantic layer on, budgets 300,
450 and 600), as the harness wrote it: `report.md`, `rows.jsonl`, `meta.json`, `gold.jsonl` and
the run's log. The `texts/` directories (every scored text, 0.6 MB a run) are left out. The files
were added on 2026-11-11 after a reviewer noted that the table had no file behind it; they were
copied unchanged from the build container, where the runs had been left.

| directory | packer | import | run (UTC, 2026-11-11) |
|---|---|---|---|
| `before/` | `cc8e8e3` (before both changes) | fresh, run `f48fdc` | 15:55 |
| `after-b/` | change B | fresh, run `f4907b` | 15:58 |
| `after-b-reuse-run/` | change B | `--reuse-run f48fdc` | 15:57 |
| `before-packer-reuse-run/` | `cc8e8e3` | `--reuse-run f4907b` | 16:42 |

`before/` and `after-b/` are the two columns of the guard's table. The two `reuse-run`
directories are the passes `../README.md` describes under "`--reuse-run` is not a fair after":
both packers fall to 0.00 `files_code` at 300 tokens when the goals are posted a second time on
the same repos.

One thing the times show: `after-b/` ran at 15:58 UTC and `bd9cef5` was committed at 16:17 UTC,
so that pass used the working tree that became the commit, not a build of the commit itself.
The first-packs replays in `../after-b/` were run after the commit.
