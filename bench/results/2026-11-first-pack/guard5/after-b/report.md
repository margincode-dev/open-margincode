# Offline pack retrieval (Plan 6, E1)

9 tasks (2 whose goal names a gold file or symbol). Semantic layer: on. Methods get the same cl100k budget; every text is scored by the same line matching against the fix's gold files and symbols.

Packs marked degraded (semantic search failed or timed out): 0 of 27.

Metrics (higher is better, means over tasks):

- `files_code`: gold files with code shown (a gold signature, or three lines unique to the file)
- `symbols_sig`: gold symbols whose signature line is shown
- `symbols_body`: gold symbols with at least half their lines shown
- `files_named`: gold file paths mentioned
- `gold_share`: share of the text's lines that are gold code (precision)

## All tasks (n = 9)

### Budget 300

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 244 | 0.22 | 0.22 | 0.00 | 1.00 | 0.02 |
| pack_sig | 244 | 0.22 | 0.22 | 0.00 | 1.00 | 0.02 |
| bm25_files | 291 | 1.00 | 0.94 | 0.83 | 1.00 | 0.85 |
| bm25_symbols | 257 | 0.89 | 0.56 | 0.56 | 0.89 | 0.52 |
| repo_map | 285 | 0.89 | 0.89 | 0.17 | 0.89 | 0.00 |

### Budget 450

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 369 | 0.61 | 0.44 | 0.11 | 1.00 | 0.10 |
| pack_sig | 369 | 0.44 | 0.44 | 0.00 | 1.00 | 0.03 |
| bm25_files | 428 | 1.00 | 1.00 | 1.00 | 1.00 | 0.65 |
| bm25_symbols | 346 | 0.89 | 0.67 | 0.67 | 0.89 | 0.51 |
| repo_map | 438 | 0.89 | 0.89 | 0.17 | 0.89 | 0.00 |

### Budget 600

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 504 | 1.00 | 0.89 | 0.56 | 1.00 | 0.21 |
| pack_sig | 454 | 0.67 | 0.67 | 0.00 | 1.00 | 0.04 |
| bm25_files | 554 | 1.00 | 1.00 | 1.00 | 1.00 | 0.55 |
| bm25_symbols | 416 | 1.00 | 0.83 | 0.83 | 1.00 | 0.50 |
| repo_map | 588 | 0.89 | 0.89 | 0.17 | 0.89 | 0.00 |

## Goal names no gold file or symbol: tasks (n = 7)

### Budget 300

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 244 | 0.29 | 0.29 | 0.00 | 1.00 | 0.02 |
| pack_sig | 244 | 0.29 | 0.29 | 0.00 | 1.00 | 0.02 |
| bm25_files | 291 | 1.00 | 1.00 | 0.86 | 1.00 | 0.85 |
| bm25_symbols | 267 | 0.86 | 0.57 | 0.57 | 0.86 | 0.48 |
| repo_map | 288 | 0.86 | 0.86 | 0.14 | 0.86 | 0.00 |

### Budget 450

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 369 | 0.57 | 0.43 | 0.14 | 1.00 | 0.10 |
| pack_sig | 369 | 0.43 | 0.43 | 0.00 | 1.00 | 0.04 |
| bm25_files | 446 | 1.00 | 1.00 | 1.00 | 1.00 | 0.59 |
| bm25_symbols | 360 | 0.86 | 0.71 | 0.71 | 0.86 | 0.44 |
| repo_map | 437 | 0.86 | 0.86 | 0.14 | 0.86 | 0.00 |

### Budget 600

| method | tokens | files_code | symbols_sig | symbols_body | files_named | gold_share |
|---|---|---|---|---|---|---|
| pack | 504 | 1.00 | 1.00 | 0.57 | 1.00 | 0.18 |
| pack_sig | 463 | 0.71 | 0.71 | 0.00 | 1.00 | 0.05 |
| bm25_files | 590 | 1.00 | 1.00 | 1.00 | 1.00 | 0.46 |
| bm25_symbols | 427 | 1.00 | 0.86 | 0.86 | 1.00 | 0.42 |
| repo_map | 588 | 0.86 | 0.86 | 0.14 | 0.86 | 0.00 |

## Pack against each baseline, paired by task

Mean difference (pack minus baseline) with a 95 % bootstrap interval, and wins/ties/losses over tasks.

| budget | pack | baseline | metric | diff | 95 % CI | W/T/L |
|---|---|---|---|---|---|---|
| 300 | pack | bm25_files | files_code | -0.78 | [-1.00, -0.44] | 0/2/7 |
| 300 | pack | bm25_files | symbols_sig | -0.72 | [-1.00, -0.39] | 0/2/7 |
| 300 | pack | bm25_symbols | files_code | -0.67 | [-0.89, -0.33] | 0/3/6 |
| 300 | pack | bm25_symbols | symbols_sig | -0.33 | [-0.67, +0.00] | 0/6/3 |
| 300 | pack | repo_map | files_code | -0.67 | [-0.89, -0.33] | 0/3/6 |
| 300 | pack | repo_map | symbols_sig | -0.67 | [-0.89, -0.33] | 0/3/6 |
| 300 | pack_sig | bm25_files | files_code | -0.78 | [-1.00, -0.44] | 0/2/7 |
| 300 | pack_sig | bm25_files | symbols_sig | -0.72 | [-1.00, -0.39] | 0/2/7 |
| 300 | pack_sig | bm25_symbols | files_code | -0.67 | [-0.89, -0.33] | 0/3/6 |
| 300 | pack_sig | bm25_symbols | symbols_sig | -0.33 | [-0.67, +0.00] | 0/6/3 |
| 300 | pack_sig | repo_map | files_code | -0.67 | [-0.89, -0.33] | 0/3/6 |
| 300 | pack_sig | repo_map | symbols_sig | -0.67 | [-0.89, -0.33] | 0/3/6 |
| 450 | pack | bm25_files | files_code | -0.39 | [-0.67, -0.11] | 0/5/4 |
| 450 | pack | bm25_files | symbols_sig | -0.56 | [-0.89, -0.22] | 0/4/5 |
| 450 | pack | bm25_symbols | files_code | -0.28 | [-0.56, +0.00] | 0/6/3 |
| 450 | pack | bm25_symbols | symbols_sig | -0.22 | [-0.56, +0.00] | 0/7/2 |
| 450 | pack | repo_map | files_code | -0.28 | [-0.56, +0.00] | 0/6/3 |
| 450 | pack | repo_map | symbols_sig | -0.44 | [-0.78, -0.11] | 0/5/4 |
| 450 | pack_sig | bm25_files | files_code | -0.56 | [-0.89, -0.22] | 0/4/5 |
| 450 | pack_sig | bm25_files | symbols_sig | -0.56 | [-0.89, -0.22] | 0/4/5 |
| 450 | pack_sig | bm25_symbols | files_code | -0.44 | [-0.78, -0.11] | 0/5/4 |
| 450 | pack_sig | bm25_symbols | symbols_sig | -0.22 | [-0.56, +0.00] | 0/7/2 |
| 450 | pack_sig | repo_map | files_code | -0.44 | [-0.78, -0.11] | 0/5/4 |
| 450 | pack_sig | repo_map | symbols_sig | -0.44 | [-0.78, -0.11] | 0/5/4 |
| 600 | pack | bm25_files | files_code | +0.00 | [+0.00, +0.00] | 0/9/0 |
| 600 | pack | bm25_files | symbols_sig | -0.11 | [-0.33, +0.00] | 0/8/1 |
| 600 | pack | bm25_symbols | files_code | +0.00 | [+0.00, +0.00] | 0/9/0 |
| 600 | pack | bm25_symbols | symbols_sig | +0.06 | [-0.17, +0.33] | 1/7/1 |
| 600 | pack | repo_map | files_code | +0.11 | [+0.00, +0.33] | 1/8/0 |
| 600 | pack | repo_map | symbols_sig | +0.00 | [-0.33, +0.33] | 1/7/1 |
| 600 | pack_sig | bm25_files | files_code | -0.33 | [-0.67, -0.11] | 0/6/3 |
| 600 | pack_sig | bm25_files | symbols_sig | -0.33 | [-0.67, -0.11] | 0/6/3 |
| 600 | pack_sig | bm25_symbols | files_code | -0.33 | [-0.67, -0.11] | 0/6/3 |
| 600 | pack_sig | bm25_symbols | symbols_sig | -0.17 | [-0.39, +0.00] | 0/7/2 |
| 600 | pack_sig | repo_map | files_code | -0.22 | [-0.56, +0.22] | 1/5/3 |
| 600 | pack_sig | repo_map | symbols_sig | -0.22 | [-0.56, +0.22] | 1/5/3 |

## Per task at budget 450: `files_code` / `symbols_sig`

| task | gold files | gold symbols | leak | pack | pack_sig | bm25_files | bm25_symbols | repo_map |
|---|---|---|---|---|---|---|---|---|
| aging-due-today | 1 | 1 |  | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| backoff-overflow | 1 | 1 |  | 0.00 / 0.00 | 0.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| cache-key-order | 2 | 2 | yes | 0.50 / 0.00 | 0.00 / 0.00 | 1.00 / 1.00 | 1.00 / 0.00 | 1.00 / 1.00 |
| discount-cap | 1 | 1 |  | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| percentile-rank | 1 | 1 | yes | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| refund-over-captured | 1 | 1 |  | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| requeue-attempts | 1 | 1 |  | 0.00 / 0.00 | 0.00 / 0.00 | 1.00 / 1.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| retry-after-date | 1 | 1 |  | 0.00 / 0.00 | 0.00 / 0.00 | 1.00 / 1.00 | 1.00 / 1.00 | 1.00 / 1.00 |
| token-skew | 1 | 1 |  | 1.00 / 0.00 | 0.00 / 0.00 | 1.00 / 1.00 | 1.00 / 0.00 | 1.00 / 1.00 |
