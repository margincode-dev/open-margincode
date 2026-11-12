## Stage 2: `relay-quote-split` on the build with the correction feature

| arm | sessions | valid | accepted | rate | 95 % Wilson | cost | cost per accepted | median turns |
|---|---|---|---|---|---|---|---|---|
| cold | 12 | 12 | 12 | 1.000 | 0.758 to 1.000 | $0.1112 | $0.0093 | 15.5 |
| notes | 30 | 30 | 28 | 0.933 | 0.787 to 0.982 | $0.4372 | $0.0156 | 20.0 |
| av | 30 | 30 | 12 | 0.400 | 0.246 to 0.577 | $0.4393 | $0.0366 | 21.0 |

Sessions skipped for budget: 0.

### Decision by the pre-registered rule

Reference, the second run on the build without the change: av 2/30 (0.018 to 0.213), notes 27/30 (0.744 to 0.965).

- Drift control, cold: 12 of 12 (at least 10 of 12).
- Drift control, notes: 28/30 (0.787 to 0.982) overlaps the reference interval.
- **R1 is reported**: av 12/30 (lower end 0.246) is above the reference's upper end (0.213).
- **R2, the gap remains**: av 12/30 (upper end 0.577) is below notes 28/30 (lower end 0.787).

### Without exposed sessions (0 left out)

| arm | sessions | valid | accepted | rate | 95 % Wilson | cost | cost per accepted | median turns |
|---|---|---|---|---|---|---|---|---|
| cold | 12 | 12 | 12 | 1.000 | 0.758 to 1.000 | $0.1112 | $0.0093 | 15.5 |
| notes | 30 | 30 | 28 | 0.933 | 0.787 to 0.982 | $0.4372 | $0.0156 | 20.0 |
| av | 30 | 30 | 12 | 0.400 | 0.246 to 0.577 | $0.4393 | $0.0366 | 21.0 |

Sessions skipped for budget: 0.

### The correction line and the command, `av` sessions

| valid | line in the session-start pack | wrong note's handle found | tried `--supersedes` | accepted by the daemon | superseded the wrong note |
|---|---|---|---|---|---|
| 30 | 30 | 30 | 16 | 16 | 12 |

| | accepted | not accepted |
|---|---|---|
| superseded the wrong note | 12 | 0 |
| did not | 0 | 18 |

| rep | accepted | wrote wrong edit | line | tried | accepted by daemon | superseded the wrong note |
|---|---|---|---|---|---|---|
| 1 | yes | no | yes | yes | yes | yes |
| 2 | no | yes | yes | no | no | no |
| 3 | no | yes | yes | yes | yes | no |
| 4 | yes | no | yes | yes | yes | yes |
| 5 | yes | no | yes | yes | yes | yes |
| 6 | no | yes | yes | no | no | no |
| 7 | no | yes | yes | no | no | no |
| 8 | yes | no | yes | yes | yes | yes |
| 9 | no | yes | yes | no | no | no |
| 10 | no | yes | yes | yes | yes | no |
| 11 | yes | no | yes | yes | yes | yes |
| 12 | yes | no | yes | yes | yes | yes |
| 13 | no | yes | yes | no | no | no |
| 14 | yes | no | yes | yes | yes | yes |
| 15 | no | yes | yes | no | no | no |
| 16 | no | yes | yes | yes | yes | no |
| 17 | no | yes | yes | no | no | no |
| 18 | no | yes | yes | yes | yes | no |
| 19 | no | yes | yes | no | no | no |
| 20 | no | yes | yes | no | no | no |
| 21 | yes | no | yes | yes | yes | yes |
| 22 | yes | no | yes | yes | yes | yes |
| 23 | no | yes | yes | no | no | no |
| 24 | yes | no | yes | yes | yes | yes |
| 25 | yes | no | yes | yes | yes | yes |
| 26 | no | yes | yes | no | no | no |
| 27 | no | yes | yes | no | no | no |
| 28 | no | yes | yes | no | no | no |
| 29 | no | yes | yes | no | no | no |
| 30 | yes | no | yes | yes | yes | yes |

### Sessions that wrote the wrong edit (`repeated_dead_ends` >= 1)

| arm | valid | wrote it | wrote it, accepted | wrote it, not accepted | never wrote it, not accepted |
|---|---|---|---|---|---|
| cold | 12 | 0 | 0 | 0 | 0 |
| notes | 30 | 2 | 0 | 2 | 0 |
| av | 30 | 18 | 0 | 18 | 0 |

### Delivery of the task's wrong note, per session

| arm | valid | session dir found | delivered | session-start hook | pre-read hook | in a tool result | read NOTES.md |
|---|---|---|---|---|---|---|---|
| cold | 12 | 12 | 0 | 0 | 0 | 0 | 0 |
| notes | 30 | 30 | 30 | 0 | 0 | 30 | 30 |
| av | 30 | 30 | 30 | 30 | 0 | 19 | 0 |

`delivered`: for `notes`, the note's whole summary is in a tool result (the read of `NOTES.md`); for `av`, it is in the session-start hook's output or a pre-read answer; for `cold`, it is anywhere in the event stream (expected 0).

### Per session

| run | task | arm | rep | status | accepted | wrote wrong edit | delivered | turns | cost |
|---|---|---|---|---|---|---|---|---|---|
| s2-cold | relay-quote-split | cold | 1 | ok | yes | no | no | 16 | $0.0096 |
| s2-cold | relay-quote-split | cold | 2 | ok | yes | no | no | 15 | $0.0085 |
| s2-cold | relay-quote-split | cold | 3 | ok | yes | no | no | 14 | $0.0087 |
| s2-cold | relay-quote-split | cold | 4 | ok | yes | no | no | 14 | $0.0088 |
| s2-cold | relay-quote-split | cold | 5 | ok | yes | no | no | 18 | $0.0100 |
| s2-cold | relay-quote-split | cold | 6 | ok | yes | no | no | 17 | $0.0096 |
| s2-cold | relay-quote-split | cold | 7 | ok | yes | no | no | 14 | $0.0085 |
| s2-cold | relay-quote-split | cold | 8 | ok | yes | no | no | 16 | $0.0093 |
| s2-cold | relay-quote-split | cold | 9 | ok | yes | no | no | 14 | $0.0093 |
| s2-cold | relay-quote-split | cold | 10 | ok | yes | no | no | 16 | $0.0098 |
| s2-cold | relay-quote-split | cold | 11 | ok | yes | no | no | 15 | $0.0081 |
| s2-cold | relay-quote-split | cold | 12 | ok | yes | no | no | 20 | $0.0111 |
| s2-wrong | relay-quote-split | av | 1 | ok | yes | no | yes | 20 | $0.0146 |
| s2-wrong | relay-quote-split | notes | 1 | ok | yes | no | yes | 20 | $0.0144 |
| s2-wrong | relay-quote-split | notes | 2 | ok | yes | no | yes | 20 | $0.0147 |
| s2-wrong | relay-quote-split | av | 2 | ok | no | yes | yes | 21 | $0.0128 |
| s2-wrong | relay-quote-split | av | 3 | ok | no | yes | yes | 23 | $0.0146 |
| s2-wrong | relay-quote-split | notes | 3 | ok | yes | no | yes | 18 | $0.0133 |
| s2-wrong | relay-quote-split | notes | 4 | ok | yes | no | yes | 17 | $0.0120 |
| s2-wrong | relay-quote-split | av | 4 | ok | yes | no | yes | 21 | $0.0145 |
| s2-wrong | relay-quote-split | notes | 5 | ok | no | yes | yes | 17 | $0.0112 |
| s2-wrong | relay-quote-split | av | 5 | ok | yes | no | yes | 22 | $0.0184 |
| s2-wrong | relay-quote-split | av | 6 | ok | no | yes | yes | 16 | $0.0098 |
| s2-wrong | relay-quote-split | notes | 6 | ok | yes | no | yes | 25 | $0.0179 |
| s2-wrong | relay-quote-split | notes | 7 | ok | yes | no | yes | 21 | $0.0136 |
| s2-wrong | relay-quote-split | av | 7 | ok | no | yes | yes | 21 | $0.0133 |
| s2-wrong | relay-quote-split | av | 8 | ok | yes | no | yes | 31 | $0.0302 |
| s2-wrong | relay-quote-split | notes | 8 | ok | yes | no | yes | 25 | $0.0178 |
| s2-wrong | relay-quote-split | av | 9 | ok | no | yes | yes | 21 | $0.0130 |
| s2-wrong | relay-quote-split | notes | 9 | ok | yes | no | yes | 21 | $0.0152 |
| s2-wrong | relay-quote-split | notes | 10 | ok | yes | no | yes | 18 | $0.0126 |
| s2-wrong | relay-quote-split | av | 10 | ok | no | yes | yes | 24 | $0.0147 |
| s2-wrong | relay-quote-split | av | 11 | ok | yes | no | yes | 19 | $0.0135 |
| s2-wrong | relay-quote-split | notes | 11 | ok | yes | no | yes | 19 | $0.0140 |
| s2-wrong | relay-quote-split | notes | 12 | ok | yes | no | yes | 25 | $0.0178 |
| s2-wrong | relay-quote-split | av | 12 | ok | yes | no | yes | 29 | $0.0225 |
| s2-wrong | relay-quote-split | av | 13 | ok | no | yes | yes | 14 | $0.0103 |
| s2-wrong | relay-quote-split | notes | 13 | ok | yes | no | yes | 22 | $0.0130 |
| s2-wrong | relay-quote-split | notes | 14 | ok | no | yes | yes | 15 | $0.0096 |
| s2-wrong | relay-quote-split | av | 14 | ok | yes | no | yes | 23 | $0.0133 |
| s2-wrong | relay-quote-split | notes | 15 | ok | yes | no | yes | 20 | $0.0138 |
| s2-wrong | relay-quote-split | av | 15 | ok | no | yes | yes | 20 | $0.0117 |
| s2-wrong | relay-quote-split | notes | 16 | ok | yes | no | yes | 18 | $0.0136 |
| s2-wrong | relay-quote-split | av | 16 | ok | no | yes | yes | 30 | $0.0207 |
| s2-wrong | relay-quote-split | av | 17 | ok | no | yes | yes | 18 | $0.0112 |
| s2-wrong | relay-quote-split | notes | 17 | ok | yes | no | yes | 22 | $0.0132 |
| s2-wrong | relay-quote-split | notes | 18 | ok | yes | no | yes | 19 | $0.0146 |
| s2-wrong | relay-quote-split | av | 18 | ok | no | yes | yes | 26 | $0.0150 |
| s2-wrong | relay-quote-split | notes | 19 | ok | yes | no | yes | 19 | $0.0143 |
| s2-wrong | relay-quote-split | av | 19 | ok | no | yes | yes | 17 | $0.0106 |
| s2-wrong | relay-quote-split | av | 20 | ok | no | yes | yes | 19 | $0.0114 |
| s2-wrong | relay-quote-split | notes | 20 | ok | yes | no | yes | 20 | $0.0145 |
| s2-wrong | relay-quote-split | av | 21 | ok | yes | no | yes | 21 | $0.0146 |
| s2-wrong | relay-quote-split | notes | 21 | ok | yes | no | yes | 21 | $0.0177 |
| s2-wrong | relay-quote-split | av | 22 | ok | yes | no | yes | 27 | $0.0161 |
| s2-wrong | relay-quote-split | notes | 22 | ok | yes | no | yes | 20 | $0.0155 |
| s2-wrong | relay-quote-split | notes | 23 | ok | yes | no | yes | 21 | $0.0150 |
| s2-wrong | relay-quote-split | av | 23 | ok | no | yes | yes | 20 | $0.0132 |
| s2-wrong | relay-quote-split | av | 24 | ok | yes | no | yes | 24 | $0.0179 |
| s2-wrong | relay-quote-split | notes | 24 | ok | yes | no | yes | 17 | $0.0157 |
| s2-wrong | relay-quote-split | notes | 25 | ok | yes | no | yes | 21 | $0.0157 |
| s2-wrong | relay-quote-split | av | 25 | ok | yes | no | yes | 24 | $0.0181 |
| s2-wrong | relay-quote-split | av | 26 | ok | no | yes | yes | 13 | $0.0106 |
| s2-wrong | relay-quote-split | notes | 26 | ok | yes | no | yes | 21 | $0.0145 |
| s2-wrong | relay-quote-split | notes | 27 | ok | yes | no | yes | 18 | $0.0125 |
| s2-wrong | relay-quote-split | av | 27 | ok | no | yes | yes | 15 | $0.0107 |
| s2-wrong | relay-quote-split | av | 28 | ok | no | yes | yes | 16 | $0.0112 |
| s2-wrong | relay-quote-split | notes | 28 | ok | yes | no | yes | 19 | $0.0141 |
| s2-wrong | relay-quote-split | av | 29 | ok | no | yes | yes | 20 | $0.0120 |
| s2-wrong | relay-quote-split | notes | 29 | ok | yes | no | yes | 25 | $0.0182 |
| s2-wrong | relay-quote-split | notes | 30 | ok | yes | no | yes | 24 | $0.0173 |
| s2-wrong | relay-quote-split | av | 30 | ok | yes | no | yes | 25 | $0.0186 |

