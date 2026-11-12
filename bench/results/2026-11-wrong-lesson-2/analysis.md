## Primary: `relay-quote-split`

| arm | sessions | valid | accepted | rate | 95 % Wilson | cost | cost per accepted | median turns |
|---|---|---|---|---|---|---|---|---|
| cold | 12 | 12 | 12 | 1.000 | 0.758 to 1.000 | $0.1051 | $0.0088 | 14.0 |
| notes | 30 | 30 | 27 | 0.900 | 0.744 to 0.965 | $0.4481 | $0.0166 | 21.0 |
| av | 30 | 30 | 2 | 0.067 | 0.018 to 0.213 | $0.3499 | $0.1750 | 17.5 |

Sessions skipped for budget: 0.

### Decision by the pre-registered rule

- **H1 is reported**: av 2/30 (upper end 0.213) is below notes 27/30 (lower end 0.744).
- cold solved 12 of 12: the task is still solved without any note.

### Without exposed sessions (1 left out)

| arm | sessions | valid | accepted | rate | 95 % Wilson | cost | cost per accepted | median turns |
|---|---|---|---|---|---|---|---|---|
| cold | 12 | 12 | 12 | 1.000 | 0.758 to 1.000 | $0.1051 | $0.0088 | 14.0 |
| notes | 30 | 30 | 27 | 0.900 | 0.744 to 0.965 | $0.4481 | $0.0166 | 21.0 |
| av | 29 | 29 | 2 | 0.069 | 0.019 to 0.220 | $0.3205 | $0.1602 | 17 |

Sessions skipped for budget: 0.

### Sessions that wrote the wrong edit (`repeated_dead_ends` >= 1)

| arm | valid | wrote it | wrote it, accepted | wrote it, not accepted | never wrote it, not accepted |
|---|---|---|---|---|---|
| cold | 12 | 0 | 0 | 0 | 0 |
| notes | 30 | 3 | 0 | 3 | 0 |
| av | 30 | 26 | 0 | 26 | 2 |

### Delivery of the task's wrong note, per session

| arm | valid | session dir found | delivered | session-start hook | pre-read hook | in a tool result | read NOTES.md |
|---|---|---|---|---|---|---|---|
| cold | 12 | 12 | 0 | 0 | 0 | 0 | 0 |
| notes | 30 | 30 | 30 | 0 | 0 | 30 | 29 |
| av | 30 | 30 | 30 | 30 | 0 | 5 | 0 |

`delivered`: for `notes`, the note's whole summary is in a tool result (the read of `NOTES.md`); for `av`, it is in the session-start hook's output or a pre-read answer; for `cold`, it is anywhere in the event stream (expected 0).

### Per session

| run | task | arm | rep | status | accepted | wrote wrong edit | delivered | turns | cost |
|---|---|---|---|---|---|---|---|---|---|
| qs-cold | relay-quote-split | cold | 1 | ok | yes | no | no | 13 | $0.0077 |
| qs-cold | relay-quote-split | cold | 2 | ok | yes | no | no | 10 | $0.0071 |
| qs-cold | relay-quote-split | cold | 3 | ok | yes | no | no | 10 | $0.0071 |
| qs-cold | relay-quote-split | cold | 4 | ok | yes | no | no | 16 | $0.0093 |
| qs-cold | relay-quote-split | cold | 5 | ok | yes | no | no | 15 | $0.0096 |
| qs-cold | relay-quote-split | cold | 6 | ok | yes | no | no | 14 | $0.0093 |
| qs-cold | relay-quote-split | cold | 7 | ok | yes | no | no | 16 | $0.0092 |
| qs-cold | relay-quote-split | cold | 8 | ok | yes | no | no | 15 | $0.0091 |
| qs-cold | relay-quote-split | cold | 9 | ok | yes | no | no | 12 | $0.0081 |
| qs-cold | relay-quote-split | cold | 10 | ok | yes | no | no | 12 | $0.0078 |
| qs-cold | relay-quote-split | cold | 11 | ok | yes | no | no | 20 | $0.0114 |
| qs-cold | relay-quote-split | cold | 12 | ok | yes | no | no | 14 | $0.0092 |
| qs-wrong | relay-quote-split | av | 1 | ok | no | yes | yes | 20 | $0.0113 |
| qs-wrong | relay-quote-split | notes | 1 | ok | yes | no | yes | 21 | $0.0167 |
| qs-wrong | relay-quote-split | notes | 2 | ok | yes | no | yes | 19 | $0.0158 |
| qs-wrong | relay-quote-split | av | 2 | ok | no | yes | yes | 22 | $0.0136 |
| qs-wrong | relay-quote-split | av | 3 | ok | no | yes | yes | 18 | $0.0116 |
| qs-wrong | relay-quote-split | notes | 3 | ok | yes | no | yes | 29 | $0.0183 |
| qs-wrong | relay-quote-split | notes | 4 | ok | yes | no | yes | 19 | $0.0122 |
| qs-wrong | relay-quote-split | av | 4 | ok | no | no | yes | 15 | $0.0088 |
| qs-wrong | relay-quote-split | notes | 5 | ok | yes | no | yes | 19 | $0.0134 |
| qs-wrong | relay-quote-split | av | 5 | ok | no | yes | yes | 17 | $0.0105 |
| qs-wrong | relay-quote-split | av | 6 | ok | no | yes | yes | 18 | $0.0106 |
| qs-wrong | relay-quote-split | notes | 6 | ok | yes | no | yes | 28 | $0.0175 |
| qs-wrong | relay-quote-split | notes | 7 | ok | yes | no | yes | 17 | $0.0114 |
| qs-wrong | relay-quote-split | av | 7 | ok | no | yes | yes | 15 | $0.0107 |
| qs-wrong | relay-quote-split | av | 8 | ok | no | yes | yes | 12 | $0.0085 |
| qs-wrong | relay-quote-split | notes | 8 | ok | yes | no | yes | 23 | $0.0161 |
| qs-wrong | relay-quote-split | av | 9 | ok | no | no | yes | 16 | $0.0096 |
| qs-wrong | relay-quote-split | notes | 9 | ok | yes | no | yes | 17 | $0.0123 |
| qs-wrong | relay-quote-split | notes | 10 | ok | yes | no | yes | 21 | $0.0139 |
| qs-wrong | relay-quote-split | av | 10 | ok | no | yes | yes | 18 | $0.0105 |
| qs-wrong | relay-quote-split | av | 11 | ok | no | yes | yes | 18 | $0.0101 |
| qs-wrong | relay-quote-split | notes | 11 | ok | yes | no | yes | 27 | $0.0173 |
| qs-wrong | relay-quote-split | notes | 12 | ok | yes | no | yes | 21 | $0.0157 |
| qs-wrong | relay-quote-split | av | 12 | ok | no | yes | yes | 11 | $0.0082 |
| qs-wrong | relay-quote-split | av | 13 | ok | no | yes | yes | 23 | $0.0134 |
| qs-wrong | relay-quote-split | notes | 13 | ok | no | yes | yes | 21 | $0.0130 |
| qs-wrong | relay-quote-split | notes | 14 | ok | no | yes | yes | 15 | $0.0107 |
| qs-wrong | relay-quote-split | av | 14 | ok | no | yes | yes | 14 | $0.0091 |
| qs-wrong | relay-quote-split | notes | 15 | ok | yes | no | yes | 23 | $0.0170 |
| qs-wrong | relay-quote-split | av | 15 | ok | no | yes | yes | 15 | $0.0092 |
| qs-wrong | relay-quote-split | notes | 16 | ok | yes | no | yes | 20 | $0.0142 |
| qs-wrong | relay-quote-split | av | 16 | ok | no | yes | yes | 17 | $0.0105 |
| qs-wrong | relay-quote-split | av | 17 | ok | no | yes | yes | 17 | $0.0099 |
| qs-wrong | relay-quote-split | notes | 17 | ok | no | yes | yes | 11 | $0.0094 |
| qs-wrong | relay-quote-split | notes | 18 | ok | yes | no | yes | 22 | $0.0154 |
| qs-wrong | relay-quote-split | av | 18 | ok | no | yes | yes | 21 | $0.0119 |
| qs-wrong | relay-quote-split | notes | 19 | ok | yes | no | yes | 24 | $0.0136 |
| qs-wrong | relay-quote-split | av | 19 | ok | yes | no | yes | 25 | $0.0179 |
| qs-wrong | relay-quote-split | av | 20 | ok | no | yes | yes | 19 | $0.0110 |
| qs-wrong | relay-quote-split | notes | 20 | ok | yes | no | yes | 18 | $0.0141 |
| qs-wrong | relay-quote-split | av | 21 | ok | no | yes | yes | 15 | $0.0095 |
| qs-wrong | relay-quote-split | notes | 21 | ok | yes | no | yes | 21 | $0.0160 |
| qs-wrong | relay-quote-split | av | 22 | ok | yes | no | yes | 16 | $0.0115 |
| qs-wrong | relay-quote-split | notes | 22 | ok | yes | no | yes | 14 | $0.0118 |
| qs-wrong | relay-quote-split | notes | 23 | ok | yes | no | yes | 23 | $0.0168 |
| qs-wrong | relay-quote-split | av | 23 | ok | no | yes | yes | 20 | $0.0129 |
| qs-wrong | relay-quote-split | av | 24 | ok | no | yes | yes | 23 | $0.0181 |
| qs-wrong | relay-quote-split | notes | 24 | ok | yes | no | yes | 23 | $0.0173 |
| qs-wrong | relay-quote-split | notes | 25 | ok | yes | no | yes | 22 | $0.0160 |
| qs-wrong | relay-quote-split | av | 25 | ok | no | yes | yes | 16 | $0.0099 |
| qs-wrong | relay-quote-split | av | 26 | ok | no | yes | yes | 35 | $0.0295 |
| qs-wrong | relay-quote-split | notes | 26 | ok | yes | no | yes | 16 | $0.0127 |
| qs-wrong | relay-quote-split | notes | 27 | ok | yes | no | yes | 28 | $0.0233 |
| qs-wrong | relay-quote-split | av | 27 | ok | no | yes | yes | 19 | $0.0113 |
| qs-wrong | relay-quote-split | av | 28 | ok | no | yes | yes | 14 | $0.0090 |
| qs-wrong | relay-quote-split | notes | 28 | ok | yes | no | yes | 18 | $0.0138 |
| qs-wrong | relay-quote-split | av | 29 | ok | no | yes | yes | 18 | $0.0111 |
| qs-wrong | relay-quote-split | notes | 29 | ok | yes | no | yes | 21 | $0.0174 |
| qs-wrong | relay-quote-split | notes | 30 | ok | yes | no | yes | 23 | $0.0151 |
| qs-wrong | relay-quote-split | av | 30 | ok | no | yes | yes | 17 | $0.0103 |

## Secondary: the three silent tasks

Not run yet.
