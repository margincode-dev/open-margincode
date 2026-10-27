# Offline pack retrieval (Plan 6, E1)

42 tasks (15 whose goal names a gold file or symbol). Semantic layer: off (full-text packs). Methods get the same cl100k budget; every text is scored by the same line matching against the fix's gold files and symbols.

Packs marked degraded (semantic search failed or timed out): 0 of 126.

Metrics (higher is better, means over tasks):

- `files_code`: gold files with code shown (a gold signature, or three lines unique to the file)
- `symbols_sig`: gold symbols whose signature line is shown
- `symbols_body`: gold symbols with at least half their lines shown
- `files_named`: gold file paths mentioned
- `gold_share`: share of the text's lines that are gold code (precision)

## All tasks (n = 42)

### Budget 2000

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 1531 | 0.80 | 0.40 | 0.34 | 0.94 | 0.24 |
| bm25_files | 1965 | 0.36 | 0.17 | 0.14 | 0.36 | 0.31 |
| bm25_symbols | 1994 | 0.83 | 0.55 | 0.49 | 0.83 | 0.40 |
| repo_map | 1992 | 0.60 | 0.60 | 0.02 | 0.60 | 0.00 |

### Budget 4000

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 3190 | 0.88 | 0.56 | 0.48 | 0.98 | 0.23 |
| bm25_files | 3964 | 0.44 | 0.25 | 0.22 | 0.44 | 0.32 |
| bm25_symbols | 3994 | 0.89 | 0.62 | 0.58 | 0.89 | 0.33 |
| repo_map | 3990 | 0.68 | 0.69 | 0.04 | 0.69 | 0.00 |

### Budget 8000

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 6442 | 0.92 | 0.66 | 0.63 | 0.98 | 0.24 |
| bm25_files | 7994 | 0.44 | 0.36 | 0.36 | 0.44 | 0.29 |
| bm25_symbols | 7993 | 0.93 | 0.75 | 0.75 | 0.93 | 0.31 |
| repo_map | 7004 | 0.80 | 0.81 | 0.04 | 0.81 | 0.00 |

## Goal names no gold file or symbol: tasks (n = 27)

### Budget 2000

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 1519 | 0.77 | 0.42 | 0.36 | 0.96 | 0.26 |
| bm25_files | 1993 | 0.28 | 0.13 | 0.07 | 0.28 | 0.25 |
| bm25_symbols | 1993 | 0.78 | 0.48 | 0.44 | 0.78 | 0.38 |
| repo_map | 1990 | 0.61 | 0.62 | 0.04 | 0.61 | 0.00 |

### Budget 4000

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 3178 | 0.88 | 0.52 | 0.47 | 0.96 | 0.23 |
| bm25_files | 3990 | 0.41 | 0.21 | 0.17 | 0.41 | 0.28 |
| bm25_symbols | 3993 | 0.85 | 0.60 | 0.57 | 0.85 | 0.35 |
| repo_map | 3989 | 0.65 | 0.67 | 0.04 | 0.67 | 0.00 |

### Budget 8000

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 6424 | 0.91 | 0.59 | 0.58 | 0.96 | 0.24 |
| bm25_files | 7994 | 0.41 | 0.31 | 0.32 | 0.41 | 0.28 |
| bm25_symbols | 7993 | 0.89 | 0.75 | 0.76 | 0.89 | 0.33 |
| repo_map | 6782 | 0.80 | 0.81 | 0.04 | 0.81 | 0.00 |

## Pack against each baseline, paired by task

Mean difference (pack minus baseline) with a 95 % bootstrap interval, and wins/ties/losses over tasks.

| budget | baseline | metric | diff | 95 % CI | W/T/L |
|---|---|---|---|---|---|
| 2000 | bm25_files | files_code | +0.44 | [+0.28, +0.62] | 22/18/2 |
| 2000 | bm25_files | symbols_sig | +0.23 | [+0.07, +0.39] | 20/17/5 |
| 2000 | bm25_symbols | files_code | -0.03 | [-0.13, +0.07] | 2/35/5 |
| 2000 | bm25_symbols | symbols_sig | -0.14 | [-0.28, -0.01] | 5/21/16 |
| 2000 | repo_map | files_code | +0.21 | [+0.06, +0.36] | 12/26/4 |
| 2000 | repo_map | symbols_sig | -0.19 | [-0.36, -0.01] | 8/15/19 |
| 4000 | bm25_files | files_code | +0.44 | [+0.30, +0.60] | 20/21/1 |
| 4000 | bm25_files | symbols_sig | +0.31 | [+0.18, +0.44] | 20/21/1 |
| 4000 | bm25_symbols | files_code | -0.01 | [-0.07, +0.06] | 2/36/4 |
| 4000 | bm25_symbols | symbols_sig | -0.06 | [-0.19, +0.06] | 7/25/10 |
| 4000 | repo_map | files_code | +0.21 | [+0.08, +0.34] | 11/28/3 |
| 4000 | repo_map | symbols_sig | -0.13 | [-0.29, +0.04] | 7/18/17 |
| 8000 | bm25_files | files_code | +0.48 | [+0.34, +0.63] | 21/21/0 |
| 8000 | bm25_files | symbols_sig | +0.30 | [+0.18, +0.44] | 18/21/3 |
| 8000 | bm25_symbols | files_code | -0.01 | [-0.08, +0.07] | 2/37/3 |
| 8000 | bm25_symbols | symbols_sig | -0.09 | [-0.18, -0.02] | 2/30/10 |
| 8000 | repo_map | files_code | +0.12 | [+0.02, +0.25] | 7/32/3 |
| 8000 | repo_map | symbols_sig | -0.15 | [-0.29, +0.00] | 4/22/16 |

## Per task at budget 4000: `files_code` / `symbols_sig`

| task | gold files | gold symbols | leak | pack | bm25_files | bm25_symbols | repo_map |
|---|---|---|---|---|---|---|---|
| click-05f6fd0 | 1 | 2 |  | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| click-22177f8 | 1 | 4 | yes | 1.00 / 0.75 | 0.00 / 0.00 | 1.00 / 0.25 | 1.00 / 1.00 |
| click-405a548 | 1 | 8 | yes | 1.00 / 0.88 | 0.00 / 0.00 | 1.00 / 0.75 | 1.00 / 1.00 |
| click-9f0bcdf | 1 | 9 | yes | 1.00 / 0.67 | 1.00 / 0.22 | 1.00 / 0.78 | 1.00 / 1.00 |
| click-b90faad | 1 | 4 | yes | 1.00 / 0.75 | 0.00 / 0.00 | 1.00 / 0.50 | 1.00 / 1.00 |
| click-e2bddbc | 1 | 4 | yes | 1.00 / 0.00 | 0.00 / 0.00 | 1.00 / 0.00 | 1.00 / 1.00 |
| click-f25f697 | 1 | 1 |  | 1.00 / 1.00 | 1.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| click-f67c2bb | 1 | 1 |  | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 1.00 | 0.00 / 0.00 |
| fd-3b3baea | 1 | 2 | yes | 1.00 / 1.00 | 1.00 / 0.50 | 1.00 / 0.50 | 1.00 / 1.00 |
| fd-4a89aff | 1 | 1 |  | 1.00 / 1.00 | 1.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| fd-68edfde | 1 | 8 | yes | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| fd-a3942db | 1 | 1 | yes | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| fd-b3a8ff6 | 2 | 6 | yes | 1.00 / 0.67 | 0.50 / 0.17 | 1.00 / 0.33 | 1.00 / 1.00 |
| fd-b422e5d | 1 | 1 |  | 1.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 | 0.00 / 0.00 |
| fd-de2137e | 2 | 2 | yes | 0.50 / 0.50 | 1.00 / 0.50 | 1.00 / 1.00 | 0.00 / 0.00 |
| fd-e422fc3 | 1 | 3 |  | 1.00 / 0.67 | 0.00 / 0.00 | 1.00 / 0.67 | 0.00 / 0.00 |
| hono-06a45bc | 1 | 2 | yes | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| hono-08807ce | 1 | 1 |  | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 0.00 | 0.00 / 0.00 |
| hono-5ca49b0 | 1 | 1 |  | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| hono-ae401f0 | 1 | 1 |  | 1.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 | 1.00 / 1.00 |
| hono-d54720d | 1 | 1 |  | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| hono-e77c7c8 | 1 | 1 | yes | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| hono-e9c0107 | 2 | 1 |  | 0.50 / 1.00 | 0.50 / 1.00 | 0.50 / 1.00 | 0.50 / 1.00 |
| hono-eb78eca | 1 | 1 |  | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| httpx-336204f | 1 | 1 |  | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 | 0.00 / 0.00 |
| httpx-ae1b9f6 | 2 | 0 |  | 0.50 / 0.00 | 0.50 / 0.00 | 0.50 / 0.00 | 0.00 / 0.00 |
| ky-01b0839 | 2 | 2 |  | 0.50 / 0.50 | 0.50 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| ky-0b1f462 | 3 | 6 |  | 0.67 / 0.33 | 0.00 / 0.00 | 1.00 / 0.33 | 1.00 / 1.00 |
| ky-33462fc | 2 | 3 |  | 0.50 / 0.33 | 0.50 / 0.33 | 0.50 / 0.33 | 1.00 / 1.00 |
| ky-4cc6698 | 1 | 3 |  | 1.00 / 0.33 | 0.00 / 0.00 | 1.00 / 0.67 | 1.00 / 1.00 |
| ky-619c427 | 1 | 8 |  | 1.00 / 0.38 | 1.00 / 0.12 | 1.00 / 0.38 | 1.00 / 1.00 |
| ky-6e27594 | 1 | 2 |  | 1.00 / 0.00 | 0.00 / 0.00 | 1.00 / 0.50 | 1.00 / 1.00 |
| ky-74224d8 | 1 | 3 |  | 1.00 / 0.33 | 1.00 / 0.33 | 1.00 / 1.00 | 1.00 / 1.00 |
| ky-8a3a9cf | 1 | 1 |  | 1.00 / 0.00 | 0.00 / 0.00 | 1.00 / 0.00 | 1.00 / 1.00 |
| ripgrep-020687a | 2 | 3 | yes | 0.00 / 0.00 | 0.00 / 0.00 | 0.50 / 0.00 | 0.00 / 0.00 |
| ripgrep-2d4fa85 | 1 | 5 |  | 1.00 / 0.20 | 0.00 / 0.00 | 1.00 / 0.20 | 1.00 / 1.00 |
| ripgrep-435f59f | 1 | 3 |  | 1.00 / 0.00 | 1.00 / 0.00 | 1.00 / 0.00 | 0.00 / 0.00 |
| ripgrep-6892bc2 | 1 | 5 | yes | 1.00 / 0.40 | 1.00 / 0.40 | 1.00 / 1.00 | 0.00 / 0.00 |
| ripgrep-7155f64 | 2 | 1 |  | 1.00 / 1.00 | 0.00 / 0.00 | 0.50 / 0.00 | 0.00 / 0.00 |
| ripgrep-b5611fc | 1 | 1 |  | 1.00 / 0.00 | 0.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| ripgrep-c5e744c | 1 | 1 | yes | 1.00 / 0.00 | 0.00 / 0.00 | 1.00 / 1.00 | 0.00 / 0.00 |
| ripgrep-d99ac34 | 1 | 0 |  | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
