# Scale: django

7094 files tracked, 2939 code files, 2939 indexed. Import and index: 128.7 s.
Daemon memory at the end: 275 MB (peak 275 MB).
Database size at the end: 2259 MB.
Workspaces on disk: 587 MB for 8 tasks (merged ones stay).

## Latency, one agent at a time (ms)

| op | n | p50 | p95 | max |
|---|---|---|---|---|
| checkpoint | 3 | 205 | 6893 | 6893 |
| checkpoint_again | 3 | 182 | 6361 | 6361 |
| context | 9 | 268 | 317 | 317 |
| start | 3 | 24455 | 24977 | 24977 |
| why | 3 | 4 | 4 | 4 |
| why_cached | 3 | 2 | 2 | 2 |

## 1 agents at once: 1 merged in 28.2 s (0.04 merges/s)

| op | n | p50 | p95 | max |
|---|---|---|---|---|
| checkpoint | 1 | 27 | 27 | 27 |
| context | 1 | 161 | 161 | 161 |
| merge | 1 | 2655 | 2655 | 2655 |
| record | 1 | 94 | 94 | 94 |
| review | 1 | 5 | 5 | 5 |
| start | 1 | 25220 | 25220 | 25220 |
| submit | 1 | 40 | 40 | 40 |
| validate | 1 | 29 | 29 | 29 |

## 4 agents at once: 0 merged in 612.6 s (0.00 merges/s)

| op | n | p50 | p95 | max |
|---|---|---|---|---|

error: agent 1: start: daemon at http://127.0.0.1:7431 unreachable: http://127.0.0.1:7431/v1/tasks: Network Error: Network Error: Error encountered in a header: timed out reading response

error: agent 0: start: daemon at http://127.0.0.1:7431 unreachable: http://127.0.0.1:7431/v1/tasks: Network Error: Network Error: Error encountered in the status line: timed out reading response

error: agent 3: start: daemon at http://127.0.0.1:7431 unreachable: http://127.0.0.1:7431/v1/tasks: Network Error: Network Error: Error encountered in the status line: timed out reading response

error: agent 2: start: daemon at http://127.0.0.1:7431 unreachable: http://127.0.0.1:7431/v1/tasks: Network Error: Network Error: Error encountered in the status line: timed out reading response
