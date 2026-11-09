# Offline pack retrieval (Plan 6, E1)

42 tasks (15 whose goal names a gold file or symbol). Semantic layer: on. Methods get the same cl100k budget; every text is scored by the same line matching against the fix's gold files and symbols.

Packs marked degraded (semantic search failed or timed out): 7 of 168.

Metrics (higher is better, means over tasks):

- `files_code`: gold files with code shown (a gold signature, or three lines unique to the file)
- `symbols_sig`: gold symbols whose signature line is shown
- `symbols_body`: gold symbols with at least half their lines shown
- `files_named`: gold file paths mentioned
- `gold_share`: share of the text's lines that are gold code (precision)

## All tasks (n = 42)

### Budget 800

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 693 | 0.69 | 0.47 | 0.06 | 0.95 | 0.05 |
| pack_sig | 684 | 0.64 | 0.47 | 0.01 | 0.95 | 0.02 |
| bm25_files | 793 | 0.36 | 0.12 | 0.08 | 0.36 | 0.30 |
| bm25_symbols | 792 | 0.76 | 0.44 | 0.35 | 0.81 | 0.45 |
| repo_map | 789 | 0.47 | 0.42 | 0.02 | 0.50 | 0.00 |

### Budget 2000

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 1740 | 0.80 | 0.57 | 0.33 | 0.95 | 0.20 |
| pack_sig | 1454 | 0.69 | 0.54 | 0.01 | 0.96 | 0.01 |
| bm25_files | 1965 | 0.36 | 0.17 | 0.14 | 0.36 | 0.31 |
| bm25_symbols | 1994 | 0.83 | 0.55 | 0.49 | 0.86 | 0.40 |
| repo_map | 1991 | 0.60 | 0.60 | 0.02 | 0.60 | 0.00 |

### Budget 4000

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 3571 | 0.86 | 0.76 | 0.52 | 0.96 | 0.23 |
| pack_sig | 2570 | 0.76 | 0.63 | 0.01 | 0.96 | 0.01 |
| bm25_files | 3964 | 0.44 | 0.26 | 0.22 | 0.44 | 0.32 |
| bm25_symbols | 3994 | 0.89 | 0.64 | 0.60 | 0.92 | 0.34 |
| repo_map | 3990 | 0.65 | 0.67 | 0.02 | 0.67 | 0.00 |

### Budget 8000

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 7182 | 0.93 | 0.83 | 0.67 | 0.98 | 0.25 |
| pack_sig | 5000 | 0.83 | 0.71 | 0.01 | 0.98 | 0.01 |
| bm25_files | 7994 | 0.44 | 0.37 | 0.36 | 0.44 | 0.29 |
| bm25_symbols | 7992 | 0.93 | 0.76 | 0.75 | 0.93 | 0.31 |
| repo_map | 7095 | 0.80 | 0.81 | 0.04 | 0.81 | 0.00 |

## Goal names no gold file or symbol: tasks (n = 27)

### Budget 800

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 688 | 0.68 | 0.49 | 0.05 | 0.96 | 0.04 |
| pack_sig | 677 | 0.60 | 0.49 | 0.00 | 0.96 | 0.02 |
| bm25_files | 793 | 0.28 | 0.11 | 0.04 | 0.28 | 0.23 |
| bm25_symbols | 794 | 0.68 | 0.39 | 0.34 | 0.75 | 0.42 |
| repo_map | 792 | 0.45 | 0.41 | 0.04 | 0.51 | 0.00 |

### Budget 2000

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 1724 | 0.76 | 0.57 | 0.35 | 0.96 | 0.21 |
| pack_sig | 1396 | 0.62 | 0.53 | 0.00 | 0.96 | 0.01 |
| bm25_files | 1993 | 0.28 | 0.13 | 0.07 | 0.28 | 0.25 |
| bm25_symbols | 1994 | 0.78 | 0.48 | 0.44 | 0.81 | 0.38 |
| repo_map | 1990 | 0.61 | 0.62 | 0.04 | 0.61 | 0.00 |

### Budget 4000

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 3569 | 0.80 | 0.72 | 0.50 | 0.96 | 0.23 |
| pack_sig | 2385 | 0.68 | 0.58 | 0.00 | 0.96 | 0.01 |
| bm25_files | 3990 | 0.41 | 0.23 | 0.17 | 0.41 | 0.28 |
| bm25_symbols | 3993 | 0.85 | 0.61 | 0.57 | 0.89 | 0.35 |
| repo_map | 3990 | 0.65 | 0.67 | 0.04 | 0.67 | 0.00 |

### Budget 8000

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 7179 | 0.91 | 0.79 | 0.65 | 0.96 | 0.25 |
| pack_sig | 4792 | 0.79 | 0.65 | 0.00 | 0.96 | 0.01 |
| bm25_files | 7994 | 0.41 | 0.32 | 0.32 | 0.41 | 0.28 |
| bm25_symbols | 7992 | 0.89 | 0.76 | 0.76 | 0.89 | 0.33 |
| repo_map | 6860 | 0.80 | 0.81 | 0.04 | 0.81 | 0.00 |

## Pack against each baseline, paired by task

Mean difference (pack minus baseline) with a 95 % bootstrap interval, and wins/ties/losses over tasks.

| budget | pack | baseline | metric | diff | 95 % CI | W/T/L |
|---|---|---|---|---|---|---|
| 800 | pack | bm25_files | files_code | +0.33 | [+0.15, +0.52] | 20/17/5 |
| 800 | pack | bm25_files | symbols_sig | +0.35 | [+0.19, +0.50] | 25/15/2 |
| 800 | pack | bm25_symbols | files_code | -0.07 | [-0.21, +0.10] | 6/26/10 |
| 800 | pack | bm25_symbols | symbols_sig | +0.04 | [-0.11, +0.19] | 11/21/10 |
| 800 | pack | repo_map | files_code | +0.22 | [+0.04, +0.42] | 16/19/7 |
| 800 | pack | repo_map | symbols_sig | +0.05 | [-0.14, +0.25] | 17/11/14 |
| 800 | pack_sig | bm25_files | files_code | +0.28 | [+0.10, +0.46] | 18/19/5 |
| 800 | pack_sig | bm25_files | symbols_sig | +0.35 | [+0.19, +0.50] | 25/15/2 |
| 800 | pack_sig | bm25_symbols | files_code | -0.12 | [-0.26, +0.05] | 5/26/11 |
| 800 | pack_sig | bm25_symbols | symbols_sig | +0.04 | [-0.11, +0.19] | 11/21/10 |
| 800 | pack_sig | repo_map | files_code | +0.17 | [-0.02, +0.38] | 16/17/9 |
| 800 | pack_sig | repo_map | symbols_sig | +0.05 | [-0.14, +0.25] | 17/11/14 |
| 2000 | pack | bm25_files | files_code | +0.44 | [+0.26, +0.63] | 24/14/4 |
| 2000 | pack | bm25_files | symbols_sig | +0.39 | [+0.22, +0.56] | 27/10/5 |
| 2000 | pack | bm25_symbols | files_code | -0.04 | [-0.14, +0.07] | 3/34/5 |
| 2000 | pack | bm25_symbols | symbols_sig | +0.02 | [-0.11, +0.15] | 9/24/9 |
| 2000 | pack | repo_map | files_code | +0.20 | [+0.06, +0.36] | 13/25/4 |
| 2000 | pack | repo_map | symbols_sig | -0.03 | [-0.22, +0.16] | 12/13/17 |
| 2000 | pack_sig | bm25_files | files_code | +0.33 | [+0.15, +0.51] | 20/18/4 |
| 2000 | pack_sig | bm25_files | symbols_sig | +0.37 | [+0.20, +0.53] | 27/10/5 |
| 2000 | pack_sig | bm25_symbols | files_code | -0.15 | [-0.27, -0.02] | 2/31/9 |
| 2000 | pack_sig | bm25_symbols | symbols_sig | -0.01 | [-0.15, +0.13] | 8/24/10 |
| 2000 | pack_sig | repo_map | files_code | +0.09 | [-0.10, +0.29] | 13/20/9 |
| 2000 | pack_sig | repo_map | symbols_sig | -0.06 | [-0.25, +0.14] | 12/11/19 |
| 4000 | pack | bm25_files | files_code | +0.42 | [+0.26, +0.57] | 20/21/1 |
| 4000 | pack | bm25_files | symbols_sig | +0.50 | [+0.38, +0.63] | 28/13/1 |
| 4000 | pack | bm25_symbols | files_code | -0.04 | [-0.12, +0.05] | 1/38/3 |
| 4000 | pack | bm25_symbols | symbols_sig | +0.12 | [+0.03, +0.21] | 12/26/4 |
| 4000 | pack | repo_map | files_code | +0.20 | [+0.06, +0.33] | 12/27/3 |
| 4000 | pack | repo_map | symbols_sig | +0.10 | [-0.07, +0.26] | 11/19/12 |
| 4000 | pack_sig | bm25_files | files_code | +0.32 | [+0.14, +0.50] | 19/19/4 |
| 4000 | pack_sig | bm25_files | symbols_sig | +0.37 | [+0.19, +0.54] | 27/9/6 |
| 4000 | pack_sig | bm25_symbols | files_code | -0.13 | [-0.23, -0.05] | 0/35/7 |
| 4000 | pack_sig | bm25_symbols | symbols_sig | -0.01 | [-0.15, +0.12] | 9/23/10 |
| 4000 | pack_sig | repo_map | files_code | +0.10 | [-0.06, +0.28] | 12/22/8 |
| 4000 | pack_sig | repo_map | symbols_sig | -0.04 | [-0.22, +0.16] | 11/14/17 |
| 8000 | pack | bm25_files | files_code | +0.49 | [+0.35, +0.63] | 23/19/0 |
| 8000 | pack | bm25_files | symbols_sig | +0.46 | [+0.33, +0.60] | 24/17/1 |
| 8000 | pack | bm25_symbols | files_code | +0.00 | [-0.04, +0.04] | 1/40/1 |
| 8000 | pack | bm25_symbols | symbols_sig | +0.07 | [+0.01, +0.14] | 12/27/3 |
| 8000 | pack | repo_map | files_code | +0.13 | [+0.05, +0.23] | 7/35/0 |
| 8000 | pack | repo_map | symbols_sig | +0.02 | [-0.09, +0.14] | 6/28/8 |
| 8000 | pack_sig | bm25_files | files_code | +0.39 | [+0.23, +0.56] | 21/18/3 |
| 8000 | pack_sig | bm25_files | symbols_sig | +0.34 | [+0.14, +0.52] | 23/13/6 |
| 8000 | pack_sig | bm25_symbols | files_code | -0.10 | [-0.18, -0.02] | 0/36/6 |
| 8000 | pack_sig | bm25_symbols | symbols_sig | -0.05 | [-0.17, +0.06] | 10/23/9 |
| 8000 | pack_sig | repo_map | files_code | +0.03 | [-0.10, +0.17] | 7/29/6 |
| 8000 | pack_sig | repo_map | symbols_sig | -0.10 | [-0.25, +0.05] | 6/23/13 |

## Per task at budget 4000: `files_code` / `symbols_sig`

| task | gold files | gold symbols | leak | pack | pack_sig | bm25_files | bm25_symbols | repo_map |
|---|---|---|---|---|---|---|---|---|
| click-05f6fd0 | 1 | 2 |  | 1.00 / 1.00 | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| click-22177f8 | 1 | 4 | yes | 1.00 / 1.00 | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 0.25 | 1.00 / 1.00 |
| click-405a548 | 1 | 8 | yes | 1.00 / 1.00 | 1.00 / 0.62 | 0.00 / 0.00 | 1.00 / 0.75 | 1.00 / 1.00 |
| click-9f0bcdf | 1 | 9 | yes | 1.00 / 0.67 | 1.00 / 0.67 | 1.00 / 0.22 | 1.00 / 0.78 | 1.00 / 1.00 |
| click-b90faad | 1 | 4 | yes | 1.00 / 0.75 | 1.00 / 0.75 | 0.00 / 0.00 | 1.00 / 0.50 | 1.00 / 1.00 |
| click-e2bddbc | 1 | 4 | yes | 1.00 / 0.00 | 1.00 / 0.00 | 0.00 / 0.00 | 1.00 / 0.00 | 1.00 / 1.00 |
| click-f25f697 | 1 | 1 |  | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| click-f67c2bb | 1 | 1 |  | 1.00 / 1.00 | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 1.00 | 0.00 / 0.00 |
| fd-3b3baea | 1 | 2 | yes | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 0.50 | 1.00 / 1.00 | 1.00 / 1.00 |
| fd-4a89aff | 1 | 1 |  | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| fd-68edfde | 1 | 8 | yes | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| fd-a3942db | 1 | 1 | yes | 1.00 / 1.00 | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| fd-b3a8ff6 | 2 | 6 | yes | 1.00 / 0.83 | 1.00 / 0.83 | 0.50 / 0.17 | 1.00 / 0.33 | 1.00 / 1.00 |
| fd-b422e5d | 1 | 1 |  | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 | 0.00 / 0.00 |
| fd-de2137e | 2 | 2 | yes | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 0.50 | 1.00 / 1.00 | 0.00 / 0.00 |
| fd-e422fc3 | 1 | 3 |  | 1.00 / 0.67 | 1.00 / 0.67 | 0.00 / 0.00 | 1.00 / 0.67 | 0.00 / 0.00 |
| hono-06a45bc | 1 | 2 | yes | 1.00 / 1.00 | 1.00 / 0.50 | 1.00 / 1.00 | 1.00 / 1.00 | 0.00 / 0.00 |
| hono-08807ce | 1 | 1 |  | 1.00 / 1.00 | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 0.00 | 0.00 / 0.00 |
| hono-5ca49b0 | 1 | 1 |  | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| hono-ae401f0 | 1 | 1 |  | 1.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 | 1.00 / 1.00 |
| hono-d54720d | 1 | 1 |  | 1.00 / 1.00 | 1.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| hono-e77c7c8 | 1 | 1 | yes | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| hono-e9c0107 | 2 | 1 |  | 0.50 / 1.00 | 0.00 / 0.00 | 0.50 / 1.00 | 0.50 / 1.00 | 0.50 / 1.00 |
| hono-eb78eca | 1 | 1 |  | 1.00 / 1.00 | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| httpx-336204f | 1 | 1 |  | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 | 0.00 / 0.00 |
| httpx-ae1b9f6 | 2 | 0 |  | 0.50 / 0.00 | 0.50 / 0.00 | 0.50 / 0.00 | 0.50 / 0.00 | 0.00 / 0.00 |
| ky-01b0839 | 2 | 2 |  | 0.50 / 0.50 | 0.50 / 0.50 | 0.50 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| ky-0b1f462 | 3 | 6 |  | 1.00 / 0.67 | 0.33 / 0.33 | 0.00 / 0.00 | 1.00 / 0.33 | 1.00 / 1.00 |
| ky-33462fc | 2 | 3 |  | 0.50 / 0.67 | 0.50 / 0.33 | 0.50 / 0.33 | 0.50 / 0.33 | 1.00 / 1.00 |
| ky-4cc6698 | 1 | 3 |  | 1.00 / 0.67 | 1.00 / 0.67 | 0.00 / 0.00 | 1.00 / 0.67 | 1.00 / 1.00 |
| ky-619c427 | 1 | 8 |  | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 0.12 | 1.00 / 0.38 | 1.00 / 1.00 |
| ky-6e27594 | 1 | 2 |  | 1.00 / 1.00 | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 0.50 | 1.00 / 1.00 |
| ky-74224d8 | 1 | 3 |  | 1.00 / 0.67 | 1.00 / 0.67 | 1.00 / 0.33 | 1.00 / 1.00 | 1.00 / 1.00 |
| ky-8a3a9cf | 1 | 1 |  | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 | 1.00 / 0.00 | 1.00 / 1.00 |
| ripgrep-020687a | 2 | 3 | yes | 0.50 / 0.33 | 0.50 / 0.33 | 0.00 / 0.00 | 0.50 / 0.00 | 0.00 / 0.00 |
| ripgrep-2d4fa85 | 1 | 5 |  | 1.00 / 0.60 | 1.00 / 0.60 | 0.00 / 0.00 | 1.00 / 0.20 | 1.00 / 1.00 |
| ripgrep-435f59f | 1 | 3 |  | 0.00 / 0.00 | 0.00 / 0.00 | 1.00 / 0.33 | 1.00 / 0.33 | 0.00 / 0.00 |
| ripgrep-6892bc2 | 1 | 5 | yes | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 0.40 | 1.00 / 1.00 | 0.00 / 0.00 |
| ripgrep-7155f64 | 2 | 1 |  | 0.50 / 1.00 | 0.50 / 1.00 | 0.00 / 0.00 | 0.50 / 0.00 | 0.00 / 0.00 |
| ripgrep-b5611fc | 1 | 1 |  | 1.00 / 1.00 | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| ripgrep-c5e744c | 1 | 1 | yes | 1.00 / 1.00 | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 1.00 | 0.00 / 0.00 |
| ripgrep-d99ac34 | 1 | 0 |  | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
