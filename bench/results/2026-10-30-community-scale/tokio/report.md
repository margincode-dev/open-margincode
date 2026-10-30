# Scale: tokio

890 files tracked, 814 code files, 814 indexed. Import and index: 64.4 s.
Daemon memory at the end: 228 MB (peak 228 MB).
Database size at the end: 8172 MB.
Workspaces on disk: 532 MB for 61 tasks (merged ones stay).

## Latency, one agent at a time (ms)

| op | n | p50 | p95 | max |
|---|---|---|---|---|
| checkpoint | 20 | 5426 | 13134 | 16236 |
| checkpoint_again | 20 | 5635 | 10530 | 11025 |
| context | 60 | 88 | 178 | 654 |
| start | 20 | 13528 | 14369 | 14482 |
| why | 20 | 4 | 6 | 6 |
| why_cached | 20 | 2 | 3 | 4 |

## 1 agents at once: 1 merged in 14.2 s (0.07 merges/s)

| op | n | p50 | p95 | max |
|---|---|---|---|---|
| checkpoint | 1 | 8 | 8 | 8 |
| context | 1 | 55 | 55 | 55 |
| merge | 1 | 934 | 934 | 934 |
| record | 1 | 23 | 23 | 23 |
| review | 1 | 5 | 5 | 5 |
| start | 1 | 13197 | 13197 | 13197 |
| submit | 1 | 12 | 12 | 12 |
| validate | 1 | 11 | 11 | 11 |

## 8 agents at once: 8 merged in 103.1 s (0.08 merges/s)

| op | n | p50 | p95 | max |
|---|---|---|---|---|
| checkpoint | 8 | 8 | 46 | 46 |
| context | 8 | 108 | 208 | 208 |
| merge | 8 | 3836 | 10158 | 10158 |
| record | 8 | 33 | 172 | 172 |
| review | 8 | 5 | 27 | 27 |
| start | 8 | 50943 | 101015 | 101015 |
| submit | 8 | 23 | 308 | 308 |
| validate | 8 | 12 | 62 | 62 |

## 32 agents at once: 32 merged in 404.9 s (0.08 merges/s)

| op | n | p50 | p95 | max |
|---|---|---|---|---|
| checkpoint | 32 | 14 | 63 | 72 |
| context | 32 | 266 | 385 | 421 |
| merge | 32 | 5101 | 11862 | 11968 |
| record | 32 | 60 | 100 | 244 |
| review | 32 | 10 | 28 | 129 |
| start | 32 | 215960 | 401354 | 401358 |
| submit | 32 | 31 | 50 | 97 |
| validate | 32 | 23 | 59 | 59 |
