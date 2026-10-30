# Scale: django

7094 files tracked, 2939 code files, 2939 indexed. Import and index: 228.6 s.
Daemon memory at the end: 230 MB (peak 230 MB).
Database size at the end: 428 MB.
Workspaces on disk: 4480 MB for 61 tasks (merged ones stay).

## Latency, one agent at a time (ms)

| op | n | p50 | p95 | max |
|---|---|---|---|---|
| checkpoint | 20 | 34 | 73 | 100 |
| checkpoint_again | 20 | 28 | 70 | 79 |
| context | 60 | 133 | 235 | 250 |
| start | 20 | 635 | 765 | 903 |
| why | 20 | 2 | 2 | 3 |
| why_cached | 20 | 1 | 2 | 2 |

## 1 agents at once: 1 merged in 1.8 s (0.56 merges/s)

| op | n | p50 | p95 | max |
|---|---|---|---|---|
| checkpoint | 1 | 13 | 13 | 13 |
| context | 1 | 1164 | 1164 | 1164 |
| merge | 1 | 36 | 36 | 36 |
| record | 1 | 29 | 29 | 29 |
| review | 1 | 2 | 2 | 2 |
| start | 1 | 523 | 523 | 523 |
| submit | 1 | 18 | 18 | 18 |
| validate | 1 | 15 | 15 | 15 |

## 8 agents at once: 8 merged in 9.3 s (0.86 merges/s)

| op | n | p50 | p95 | max |
|---|---|---|---|---|
| checkpoint | 8 | 250 | 1685 | 1685 |
| context | 8 | 4468 | 4930 | 4930 |
| merge | 8 | 143 | 256 | 256 |
| record | 8 | 1667 | 2146 | 2146 |
| review | 8 | 5 | 25 | 25 |
| start | 8 | 1959 | 4182 | 4182 |
| submit | 8 | 84 | 166 | 166 |
| validate | 8 | 66 | 159 | 159 |

## 32 agents at once: 32 merged in 26.6 s (1.20 merges/s)

| op | n | p50 | p95 | max |
|---|---|---|---|---|
| checkpoint | 32 | 2361 | 2602 | 2622 |
| context | 32 | 5605 | 6294 | 6833 |
| merge | 32 | 2709 | 5471 | 6079 |
| record | 32 | 2380 | 2625 | 2626 |
| review | 32 | 7 | 49 | 55 |
| start | 32 | 9139 | 18300 | 21856 |
| submit | 32 | 816 | 2620 | 2671 |
| validate | 32 | 604 | 2608 | 2627 |
