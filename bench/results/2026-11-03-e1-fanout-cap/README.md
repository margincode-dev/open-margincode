# E1 under a name fan-out cap, 2026-11-03

A call names a function by its bare name, and the index links it to every symbol of that name
(`new`, `get`, `constructor`): one name shared by k symbols gives each caller k edges of weight
1/k. On tokio that is 896,182 edges, 499,681 of them from `new` alone (k = 289); 633k come from
names more than 64 symbols share. rust-lang/rust's import did not finish at about 4 GB of index.
Plan 7's Risks section left "capping fan-out" to be measured with E1; this is that measurement.

The cap: a name that more than N other symbols of the scope define links to none of them
(`AV_INDEX_MAX_FANOUT=N`, unset = no cap, the default). In-degree already counts only names with
one definition (`MAX_SHARED_NAME = 1`), so the cap changes what the `neighbors` walk reaches, not
the in-degree score.

## Run

`scholion-bench retrieval bench/retrieval/public.jsonl` (42 tasks on click, httpx, ripgrep, fd,
ky, hono), budgets 800/2000/4000/8000, semantic layer off (`--allow-no-semantic`; full-text
packs), each cap on its own fresh database and daemon: no cap, N = 64, N = 16. Debug build of
`main` at 12e1e4b plus the cap (the experiment read it from an env var in `rebuild_edges`; the
change that ships also applies it in overlay views). Reports: [`report-none.md`](report-none.md),
[`report-64.md`](report-64.md), [`report-16.md`](report-16.md).

## Result

Pack quality does not change. The `pack` row, all 42 tasks:

| budget | cap | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| 800 | none / 64 / 16 | 0.69 / 0.69 / 0.69 | 0.44 / 0.44 / 0.44 | 0.09 / 0.09 / 0.09 | 0.94 / 0.94 / 0.94 | 0.09 / 0.09 / 0.09 |
| 2000 | none / 64 / 16 | 0.83 / 0.83 / 0.83 | 0.55 / 0.53 / 0.53 | 0.33 / 0.33 / 0.33 | 0.94 / 0.94 / 0.94 | 0.21 / 0.21 / 0.21 |
| 4000 | none / 64 / 16 | 0.87 / 0.87 / 0.87 | 0.74 / 0.74 / 0.74 | 0.52 / 0.52 / 0.52 | 0.96 / 0.96 / 0.96 | 0.23 / 0.23 / 0.23 |
| 8000 | none / 64 / 16 | 0.95 / 0.95 / 0.95 | 0.79 / 0.79 / 0.79 | 0.65 / 0.65 / 0.65 | 0.98 / 0.98 / 0.98 | 0.24 / 0.24 / 0.24 |

Per task: of 168 `pack` rows (42 tasks × 4 budgets), 15 change under cap 64 and 17 under cap 16,
all but one only in `gold_share` (±0.01–0.04, both directions).

**The one regression:** `ky-619c427` at 2000 tokens, `symbols_sig` 0.88 → 0.25 under both caps
(the 2000 row's 0.55 → 0.53). With the cap the pack spends its budget on the class and
constructor lines of five other files (`ForceRetryError`, `HTTPError`, `NetworkError`, `NonError`,
`createInstance`) and drops eight members of `Ky.ts`, where the gold symbols are. At 4000 and
8000 the members are back and the task scores the same as without a cap.

**Storage**, the same 42 imports in each database:

| cap | edges (`symbol_refs`) | table | database |
|---|---|---|---|
| none | 1,049,522 | 105 MB | 272 MB |
| 64 | 632,576 (−40%) | 63 MB | 230 MB |
| 16 | 486,651 (−54%) | 49 MB | 216 MB |

On tokio (measured by edge weight in an uncapped import), a cap of 64 removes 633k of 896k edges
(71%), a cap of 128 502k (56%).

## Not decided here

The default stays off. Which N, and whether to turn it on by default, is the owner's call; 64 lost
nothing measurable here except the one task above. This run did not measure rust-lang/rust's import
under a cap, nor the semantic layer's packs (semantic was off).
