# Scale: tokio

890 files tracked, 814 code files, 814 indexed. Import and index: 58.2 s.
Daemon memory at the end: 515 MB (peak 575 MB).
Database size at the end: 11220 MB.
Workspaces on disk: 1407 MB for 161 tasks (merged ones stay).

## Latency, one agent at a time (ms)

| op | n | p50 | p95 | max |
|---|---|---|---|---|
| checkpoint | 20 | 3267 | 13502 | 15455 |
| checkpoint_again | 20 | 2827 | 11029 | 12929 |
| context | 60 | 94 | 173 | 497 |
| start | 20 | 13458 | 14119 | 14435 |
| why | 20 | 3 | 7 | 7 |
| why_cached | 20 | 2 | 3 | 3 |

## 1 agents at once: 1 merged in 15.3 s (0.07 merges/s)

| op | n | p50 | p95 | max |
|---|---|---|---|---|
| checkpoint | 1 | 9 | 9 | 9 |
| context | 1 | 70 | 70 | 70 |
| merge | 1 | 1164 | 1164 | 1164 |
| record | 1 | 21 | 21 | 21 |
| review | 1 | 5 | 5 | 5 |
| start | 1 | 14020 | 14020 | 14020 |
| submit | 1 | 15 | 15 | 15 |
| validate | 1 | 9 | 9 | 9 |

## 8 agents at once: 8 merged in 127.4 s (0.06 merges/s)

| op | n | p50 | p95 | max |
|---|---|---|---|---|
| checkpoint | 8 | 9 | 36 | 36 |
| context | 8 | 125 | 293 | 293 |
| merge | 8 | 5813 | 11797 | 11797 |
| record | 8 | 55 | 200 | 200 |
| review | 8 | 7 | 35 | 35 |
| start | 8 | 113500 | 118859 | 118859 |
| submit | 8 | 19 | 47 | 47 |
| validate | 8 | 14 | 44 | 44 |

## 32 agents at once: 16 merged in 299.6 s (0.05 merges/s)

| op | n | p50 | p95 | max |
|---|---|---|---|---|
| checkpoint | 16 | 20 | 49 | 49 |
| context | 16 | 432 | 662 | 662 |
| merge | 16 | 20442 | 28757 | 28757 |
| record | 16 | 71 | 136 | 136 |
| review | 16 | 14 | 44 | 44 |
| start | 16 | 265096 | 270638 | 270638 |
| submit | 16 | 29 | 83 | 83 |
| validate | 16 | 18 | 72 | 72 |

error: agent 26: start: daemon answered 500: {"error":"index error: pool timed out while waiting for an open connection"}

error: agent 15: start: daemon answered 500: {"error":"index error: pool timed out while waiting for an open connection"}

error: agent 25: start: daemon answered 500: {"error":"index error: pool timed out while waiting for an open connection"}

error: agent 20: start: daemon answered 500: {"error":"index error: pool timed out while waiting for an open connection"}

error: agent 30: start: daemon answered 500: {"error":"index error: pool timed out while waiting for an open connection"}

error: agent 31: start: daemon answered 500: {"error":"index error: pool timed out while waiting for an open connection"}

error: agent 23: start: daemon answered 500: {"error":"index error: pool timed out while waiting for an open connection"}

error: agent 24: start: daemon answered 500: {"error":"index error: pool timed out while waiting for an open connection"}

error: agent 22: start: daemon answered 500: {"error":"index error: pool timed out while waiting for an open connection"}

error: agent 29: start: daemon answered 500: {"error":"index error: pool timed out while waiting for an open connection"}

## 100 agents at once: 17 merged in 329.0 s (0.05 merges/s)

| op | n | p50 | p95 | max |
|---|---|---|---|---|
| checkpoint | 17 | 30 | 76 | 76 |
| context | 17 | 448 | 1200 | 1200 |
| merge | 17 | 27734 | 42542 | 42542 |
| record | 17 | 81 | 403 | 403 |
| review | 17 | 18 | 56 | 56 |
| start | 17 | 271188 | 302632 | 302632 |
| submit | 17 | 40 | 247 | 247 |
| validate | 17 | 34 | 209 | 209 |

error: agent 89: start: daemon answered 500: {"error":"index error: pool timed out while waiting for an open connection"}

error: agent 57: start: daemon answered 500: {"error":"index error: pool timed out while waiting for an open connection"}

error: agent 25: start: daemon answered 500: {"error":"index error: pool timed out while waiting for an open connection"}

error: agent 75: start: daemon answered 500: {"error":"index error: pool timed out while waiting for an open connection"}

error: agent 32: start: daemon answered 500: {"error":"index error: pool timed out while waiting for an open connection"}

error: agent 76: start: daemon answered 500: {"error":"index error: pool timed out while waiting for an open connection"}

error: agent 37: start: daemon answered 500: {"error":"index error: pool timed out while waiting for an open connection"}

error: agent 74: start: daemon answered 500: {"error":"index error: pool timed out while waiting for an open connection"}

error: agent 68: start: daemon answered 500: {"error":"index error: pool timed out while waiting for an open connection"}

error: agent 93: start: daemon answered 500: {"error":"index error: pool timed out while waiting for an open connection"}
