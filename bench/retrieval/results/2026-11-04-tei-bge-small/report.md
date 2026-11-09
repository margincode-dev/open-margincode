# Offline pack retrieval (Plan 6, E1)

42 tasks (15 whose goal names a gold file or symbol). Semantic layer: on. Methods get the same cl100k budget; every text is scored by the same line matching against the fix's gold files and symbols.

Packs marked degraded (semantic search failed or timed out): 0 of 168.

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
| pack | 692 | 0.62 | 0.43 | 0.05 | 0.95 | 0.05 |
| pack_sig | 687 | 0.63 | 0.48 | 0.01 | 0.95 | 0.02 |
| bm25_files | 793 | 0.36 | 0.12 | 0.08 | 0.36 | 0.30 |
| bm25_symbols | 792 | 0.76 | 0.44 | 0.35 | 0.81 | 0.45 |
| repo_map | 789 | 0.47 | 0.42 | 0.02 | 0.50 | 0.00 |

### Budget 2000

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 1750 | 0.78 | 0.57 | 0.34 | 0.95 | 0.18 |
| pack_sig | 1457 | 0.72 | 0.57 | 0.01 | 0.95 | 0.01 |
| bm25_files | 1965 | 0.36 | 0.17 | 0.14 | 0.36 | 0.31 |
| bm25_symbols | 1994 | 0.83 | 0.55 | 0.49 | 0.86 | 0.40 |
| repo_map | 1991 | 0.60 | 0.60 | 0.02 | 0.60 | 0.00 |

### Budget 4000

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 3570 | 0.91 | 0.74 | 0.58 | 0.98 | 0.23 |
| pack_sig | 2622 | 0.75 | 0.62 | 0.01 | 0.98 | 0.02 |
| bm25_files | 3964 | 0.44 | 0.26 | 0.22 | 0.44 | 0.32 |
| bm25_symbols | 3994 | 0.89 | 0.64 | 0.60 | 0.92 | 0.34 |
| repo_map | 3990 | 0.65 | 0.67 | 0.02 | 0.67 | 0.00 |

### Budget 8000

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 7191 | 0.92 | 0.80 | 0.64 | 0.98 | 0.24 |
| pack_sig | 4991 | 0.81 | 0.68 | 0.01 | 0.98 | 0.01 |
| bm25_files | 7994 | 0.44 | 0.37 | 0.36 | 0.44 | 0.29 |
| bm25_symbols | 7992 | 0.93 | 0.76 | 0.75 | 0.93 | 0.31 |
| repo_map | 7095 | 0.80 | 0.81 | 0.04 | 0.81 | 0.00 |

## Goal names no gold file or symbol: tasks (n = 27)

### Budget 800

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 688 | 0.60 | 0.43 | 0.04 | 0.96 | 0.04 |
| pack_sig | 684 | 0.62 | 0.52 | 0.00 | 0.96 | 0.01 |
| bm25_files | 793 | 0.28 | 0.11 | 0.04 | 0.28 | 0.23 |
| bm25_symbols | 794 | 0.68 | 0.39 | 0.34 | 0.75 | 0.42 |
| repo_map | 792 | 0.45 | 0.41 | 0.04 | 0.51 | 0.00 |

### Budget 2000

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 1741 | 0.77 | 0.60 | 0.38 | 0.96 | 0.21 |
| pack_sig | 1403 | 0.66 | 0.57 | 0.00 | 0.96 | 0.01 |
| bm25_files | 1993 | 0.28 | 0.13 | 0.07 | 0.28 | 0.25 |
| bm25_symbols | 1994 | 0.78 | 0.48 | 0.44 | 0.81 | 0.38 |
| repo_map | 1990 | 0.61 | 0.62 | 0.04 | 0.61 | 0.00 |

### Budget 4000

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 3567 | 0.88 | 0.72 | 0.58 | 0.96 | 0.24 |
| pack_sig | 2480 | 0.68 | 0.58 | 0.00 | 0.96 | 0.01 |
| bm25_files | 3990 | 0.41 | 0.23 | 0.17 | 0.41 | 0.28 |
| bm25_symbols | 3993 | 0.85 | 0.61 | 0.57 | 0.89 | 0.35 |
| repo_map | 3990 | 0.65 | 0.67 | 0.04 | 0.67 | 0.00 |

### Budget 8000

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 7190 | 0.89 | 0.77 | 0.64 | 0.96 | 0.24 |
| pack_sig | 4776 | 0.75 | 0.63 | 0.00 | 0.96 | 0.01 |
| bm25_files | 7994 | 0.41 | 0.32 | 0.32 | 0.41 | 0.28 |
| bm25_symbols | 7992 | 0.89 | 0.76 | 0.76 | 0.89 | 0.33 |
| repo_map | 6860 | 0.80 | 0.81 | 0.04 | 0.81 | 0.00 |

## Pack against each baseline, paired by task

Mean difference (pack minus baseline) with a 95 % bootstrap interval, and wins/ties/losses over tasks.

| budget | pack | baseline | metric | diff | 95 % CI | W/T/L |
|---|---|---|---|---|---|---|
| 800 | pack | bm25_files | files_code | +0.26 | [+0.06, +0.46] | 19/17/6 |
| 800 | pack | bm25_files | symbols_sig | +0.30 | [+0.12, +0.47] | 24/14/4 |
| 800 | pack | bm25_symbols | files_code | -0.14 | [-0.30, +0.01] | 4/28/10 |
| 800 | pack | bm25_symbols | symbols_sig | -0.01 | [-0.17, +0.14] | 10/21/11 |
| 800 | pack | repo_map | files_code | +0.15 | [-0.04, +0.34] | 13/22/7 |
| 800 | pack | repo_map | symbols_sig | +0.01 | [-0.18, +0.19] | 13/16/13 |
| 800 | pack_sig | bm25_files | files_code | +0.27 | [+0.08, +0.46] | 18/19/5 |
| 800 | pack_sig | bm25_files | symbols_sig | +0.36 | [+0.19, +0.52] | 25/14/3 |
| 800 | pack_sig | bm25_symbols | files_code | -0.13 | [-0.29, +0.02] | 4/28/10 |
| 800 | pack_sig | bm25_symbols | symbols_sig | +0.05 | [-0.12, +0.22] | 13/18/11 |
| 800 | pack_sig | repo_map | files_code | +0.16 | [-0.03, +0.36] | 14/20/8 |
| 800 | pack_sig | repo_map | symbols_sig | +0.07 | [-0.13, +0.26] | 15/15/12 |
| 2000 | pack | bm25_files | files_code | +0.42 | [+0.23, +0.61] | 24/13/5 |
| 2000 | pack | bm25_files | symbols_sig | +0.40 | [+0.23, +0.57] | 27/10/5 |
| 2000 | pack | bm25_symbols | files_code | -0.06 | [-0.17, +0.06] | 3/32/7 |
| 2000 | pack | bm25_symbols | symbols_sig | +0.03 | [-0.14, +0.19] | 11/19/12 |
| 2000 | pack | repo_map | files_code | +0.18 | [+0.04, +0.33] | 12/25/5 |
| 2000 | pack | repo_map | symbols_sig | -0.03 | [-0.21, +0.16] | 11/16/15 |
| 2000 | pack_sig | bm25_files | files_code | +0.37 | [+0.18, +0.54] | 21/17/4 |
| 2000 | pack_sig | bm25_files | symbols_sig | +0.40 | [+0.22, +0.56] | 28/9/5 |
| 2000 | pack_sig | bm25_symbols | files_code | -0.11 | [-0.23, -0.00] | 2/32/8 |
| 2000 | pack_sig | bm25_symbols | symbols_sig | +0.02 | [-0.13, +0.17] | 9/22/11 |
| 2000 | pack_sig | repo_map | files_code | +0.13 | [-0.05, +0.31] | 12/23/7 |
| 2000 | pack_sig | repo_map | symbols_sig | -0.03 | [-0.21, +0.16] | 11/14/17 |
| 4000 | pack | bm25_files | files_code | +0.47 | [+0.33, +0.61] | 21/21/0 |
| 4000 | pack | bm25_files | symbols_sig | +0.48 | [+0.34, +0.62] | 28/13/1 |
| 4000 | pack | bm25_symbols | files_code | +0.02 | [-0.04, +0.08] | 2/38/2 |
| 4000 | pack | bm25_symbols | symbols_sig | +0.10 | [-0.01, +0.21] | 11/26/5 |
| 4000 | pack | repo_map | files_code | +0.25 | [+0.12, +0.40] | 13/26/3 |
| 4000 | pack | repo_map | symbols_sig | +0.07 | [-0.10, +0.25] | 11/17/14 |
| 4000 | pack_sig | bm25_files | files_code | +0.31 | [+0.13, +0.48] | 18/20/4 |
| 4000 | pack_sig | bm25_files | symbols_sig | +0.36 | [+0.18, +0.53] | 26/11/5 |
| 4000 | pack_sig | bm25_symbols | files_code | -0.15 | [-0.24, -0.06] | 0/34/8 |
| 4000 | pack_sig | bm25_symbols | symbols_sig | -0.03 | [-0.16, +0.10] | 8/23/11 |
| 4000 | pack_sig | repo_map | files_code | +0.09 | [-0.09, +0.28] | 12/21/9 |
| 4000 | pack_sig | repo_map | symbols_sig | -0.05 | [-0.23, +0.15] | 11/13/18 |
| 8000 | pack | bm25_files | files_code | +0.48 | [+0.33, +0.62] | 21/21/0 |
| 8000 | pack | bm25_files | symbols_sig | +0.43 | [+0.28, +0.58] | 23/17/2 |
| 8000 | pack | bm25_symbols | files_code | -0.01 | [-0.05, +0.02] | 1/39/2 |
| 8000 | pack | bm25_symbols | symbols_sig | +0.04 | [-0.05, +0.13] | 9/29/4 |
| 8000 | pack | repo_map | files_code | +0.12 | [+0.01, +0.23] | 7/33/2 |
| 8000 | pack | repo_map | symbols_sig | -0.01 | [-0.13, +0.11] | 5/26/11 |
| 8000 | pack_sig | bm25_files | files_code | +0.37 | [+0.20, +0.53] | 20/19/3 |
| 8000 | pack_sig | bm25_files | symbols_sig | +0.31 | [+0.13, +0.49] | 22/14/6 |
| 8000 | pack_sig | bm25_symbols | files_code | -0.12 | [-0.22, -0.04] | 0/35/7 |
| 8000 | pack_sig | bm25_symbols | symbols_sig | -0.08 | [-0.19, +0.03] | 7/26/9 |
| 8000 | pack_sig | repo_map | files_code | +0.01 | [-0.12, +0.15] | 7/28/7 |
| 8000 | pack_sig | repo_map | symbols_sig | -0.13 | [-0.28, +0.03] | 6/20/16 |

## Per task at budget 4000: `files_code` / `symbols_sig`

| task | gold files | gold symbols | leak | pack | pack_sig | bm25_files | bm25_symbols | repo_map |
|---|---|---|---|---|---|---|---|---|
| click-05f6fd0 | 1 | 2 |  | 1.00 / 1.00 | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| click-22177f8 | 1 | 4 | yes | 1.00 / 1.00 | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 0.25 | 1.00 / 1.00 |
| click-405a548 | 1 | 8 | yes | 1.00 / 1.00 | 1.00 / 0.62 | 0.00 / 0.00 | 1.00 / 0.75 | 1.00 / 1.00 |
| click-9f0bcdf | 1 | 9 | yes | 1.00 / 0.67 | 1.00 / 0.67 | 1.00 / 0.22 | 1.00 / 0.78 | 1.00 / 1.00 |
| click-b90faad | 1 | 4 | yes | 1.00 / 0.50 | 1.00 / 0.75 | 0.00 / 0.00 | 1.00 / 0.50 | 1.00 / 1.00 |
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
| fd-e422fc3 | 1 | 3 |  | 1.00 / 1.00 | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 0.67 | 0.00 / 0.00 |
| hono-06a45bc | 1 | 2 | yes | 1.00 / 1.00 | 1.00 / 0.50 | 1.00 / 1.00 | 1.00 / 1.00 | 0.00 / 0.00 |
| hono-08807ce | 1 | 1 |  | 1.00 / 1.00 | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 0.00 | 0.00 / 0.00 |
| hono-5ca49b0 | 1 | 1 |  | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| hono-ae401f0 | 1 | 1 |  | 1.00 / 1.00 | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 | 1.00 / 1.00 |
| hono-d54720d | 1 | 1 |  | 1.00 / 0.00 | 0.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| hono-e77c7c8 | 1 | 1 | yes | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| hono-e9c0107 | 2 | 1 |  | 0.50 / 1.00 | 0.00 / 0.00 | 0.50 / 1.00 | 0.50 / 1.00 | 0.50 / 1.00 |
| hono-eb78eca | 1 | 1 |  | 1.00 / 1.00 | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| httpx-336204f | 1 | 1 |  | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 | 0.00 / 0.00 |
| httpx-ae1b9f6 | 2 | 0 |  | 0.50 / 0.00 | 0.50 / 0.00 | 0.50 / 0.00 | 0.50 / 0.00 | 0.00 / 0.00 |
| ky-01b0839 | 2 | 2 |  | 0.50 / 0.50 | 0.50 / 0.50 | 0.50 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| ky-0b1f462 | 3 | 6 |  | 0.67 / 0.50 | 0.33 / 0.17 | 0.00 / 0.00 | 1.00 / 0.33 | 1.00 / 1.00 |
| ky-33462fc | 2 | 3 |  | 0.50 / 0.67 | 0.50 / 0.33 | 0.50 / 0.33 | 0.50 / 0.33 | 1.00 / 1.00 |
| ky-4cc6698 | 1 | 3 |  | 1.00 / 0.67 | 1.00 / 0.67 | 0.00 / 0.00 | 1.00 / 0.67 | 1.00 / 1.00 |
| ky-619c427 | 1 | 8 |  | 1.00 / 0.88 | 1.00 / 1.00 | 1.00 / 0.12 | 1.00 / 0.38 | 1.00 / 1.00 |
| ky-6e27594 | 1 | 2 |  | 1.00 / 0.50 | 1.00 / 0.50 | 0.00 / 0.00 | 1.00 / 0.50 | 1.00 / 1.00 |
| ky-74224d8 | 1 | 3 |  | 1.00 / 0.67 | 1.00 / 0.67 | 1.00 / 0.33 | 1.00 / 1.00 | 1.00 / 1.00 |
| ky-8a3a9cf | 1 | 1 |  | 1.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 | 1.00 / 0.00 | 1.00 / 1.00 |
| ripgrep-020687a | 2 | 3 | yes | 0.50 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 | 0.50 / 0.00 | 0.00 / 0.00 |
| ripgrep-2d4fa85 | 1 | 5 |  | 1.00 / 0.60 | 1.00 / 0.60 | 0.00 / 0.00 | 1.00 / 0.20 | 1.00 / 1.00 |
| ripgrep-435f59f | 1 | 3 |  | 1.00 / 0.33 | 1.00 / 0.33 | 1.00 / 0.33 | 1.00 / 0.33 | 0.00 / 0.00 |
| ripgrep-6892bc2 | 1 | 5 | yes | 1.00 / 0.80 | 1.00 / 0.80 | 1.00 / 0.40 | 1.00 / 1.00 | 0.00 / 0.00 |
| ripgrep-7155f64 | 2 | 1 |  | 1.00 / 1.00 | 0.50 / 1.00 | 0.00 / 0.00 | 0.50 / 0.00 | 0.00 / 0.00 |
| ripgrep-b5611fc | 1 | 1 |  | 1.00 / 1.00 | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| ripgrep-c5e744c | 1 | 1 | yes | 1.00 / 1.00 | 1.00 / 1.00 | 0.00 / 0.00 | 1.00 / 1.00 | 0.00 / 0.00 |
| ripgrep-d99ac34 | 1 | 0 |  | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
