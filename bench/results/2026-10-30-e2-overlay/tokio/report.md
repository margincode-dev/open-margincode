# Scale: tokio

890 files tracked, 814 code files, 814 indexed. Import and index: 104.2 s.
Daemon memory at the end: 152 MB (peak 152 MB).
Database size at the end: 183 MB.
Workspaces on disk: 1415 MB for 161 tasks (merged ones stay).

## Latency, one agent at a time (ms)

| op | n | p50 | p95 | max |
|---|---|---|---|---|
| checkpoint | 20 | 12 | 19 | 25 |
| checkpoint_again | 20 | 12 | 19 | 19 |
| context | 60 | 35 | 75 | 517 |
| start | 20 | 96 | 178 | 596 |
| why | 20 | 2 | 2 | 3 |
| why_cached | 20 | 1 | 1 | 2 |

## 1 agents at once: 1 merged in 0.6 s (1.57 merges/s)

| op | n | p50 | p95 | max |
|---|---|---|---|---|
| checkpoint | 1 | 3 | 3 | 3 |
| context | 1 | 507 | 507 | 507 |
| merge | 1 | 15 | 15 | 15 |
| record | 1 | 11 | 11 | 11 |
| review | 1 | 2 | 2 | 2 |
| start | 1 | 84 | 84 | 84 |
| submit | 1 | 7 | 7 | 7 |
| validate | 1 | 6 | 6 | 6 |

## 8 agents at once: 8 merged in 2.2 s (3.58 merges/s)

| op | n | p50 | p95 | max |
|---|---|---|---|---|
| checkpoint | 8 | 32 | 207 | 207 |
| context | 8 | 1046 | 1603 | 1603 |
| merge | 8 | 24 | 86 | 86 |
| record | 8 | 220 | 262 | 262 |
| review | 8 | 2 | 27 | 27 |
| start | 8 | 250 | 528 | 528 |
| submit | 8 | 19 | 50 | 50 |
| validate | 8 | 20 | 60 | 60 |

## 32 agents at once: 32 merged in 6.9 s (4.65 merges/s)

| op | n | p50 | p95 | max |
|---|---|---|---|---|
| checkpoint | 32 | 550 | 696 | 714 |
| context | 32 | 2023 | 2749 | 2845 |
| merge | 32 | 638 | 1484 | 1562 |
| record | 32 | 539 | 695 | 748 |
| review | 32 | 11 | 34 | 39 |
| start | 32 | 1165 | 3748 | 3867 |
| submit | 32 | 201 | 597 | 644 |
| validate | 32 | 114 | 650 | 674 |

## 100 agents at once: 100 merged in 27.4 s (3.65 merges/s)

| op | n | p50 | p95 | max |
|---|---|---|---|---|
| checkpoint | 100 | 898 | 1433 | 1926 |
| context | 100 | 3757 | 6356 | 7467 |
| merge | 100 | 7421 | 17805 | 17968 |
| record | 100 | 944 | 1484 | 2133 |
| review | 100 | 19 | 89 | 139 |
| start | 100 | 7961 | 21701 | 22589 |
| submit | 100 | 847 | 1499 | 2108 |
| validate | 100 | 818 | 1700 | 1902 |
