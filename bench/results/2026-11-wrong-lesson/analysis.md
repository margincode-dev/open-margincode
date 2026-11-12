## Accepted

| arm | sessions | valid | accepted | rate | 95 % Wilson | cost | cost per accepted | median turns |
|---|---|---|---|---|---|---|---|---|
| cold | 30 | 30 | 27 | 0.900 | 0.744 to 0.965 | $0.3214 | $0.0119 | 15.0 |
| notes | 30 | 30 | 30 | 1.000 | 0.886 to 1.000 | $0.4785 | $0.0160 | 18.0 |
| av | 30 | 30 | 24 | 0.800 | 0.627 to 0.905 | $0.4134 | $0.0172 | 16.0 |

Sessions skipped for budget: 0.

## Accepted per task (of the valid sessions)

| task | cold | notes | av |
|---|---|---|---|
| relay-coupon-remove | 6/6 | 6/6 | 6/6 |
| relay-line-taxes | 3/6 | 6/6 | 6/6 |
| relay-money-plain | 6/6 | 6/6 | 6/6 |
| relay-quote-split | 6/6 | 6/6 | 0/6 |
| relay-store-count | 6/6 | 6/6 | 6/6 |

## Sessions that wrote the wrong edit (`repeated_dead_ends` >= 1)

| arm | valid | wrote it | wrote it, accepted | wrote it, not accepted | never wrote it, not accepted |
|---|---|---|---|---|---|
| cold | 30 | 3 | 0 | 3 | 0 |
| notes | 30 | 0 | 0 | 0 | 0 |
| av | 30 | 6 | 2 | 4 | 2 |

## Delivery of the task's wrong note, per session

| arm | valid | session dir found | delivered | session-start hook | pre-read hook | in a tool result | read NOTES.md | in the kept copy's git log |
|---|---|---|---|---|---|---|---|---|
| cold | 30 | 30 | 0 | 0 | 0 | 0 | 0 | 0 of 30 checked |
| notes | 30 | 30 | 30 | 0 | 0 | 30 | 28 | 0 of 30 checked |
| av | 30 | 30 | 30 | 30 | 0 | 7 | 0 | 0 of 0 checked |

`delivered`: for `notes`, the note's whole summary is in a tool result (the read of `NOTES.md`); for `av`, it is in the session-start hook's output or a pre-read answer; for `cold`, it is anywhere in the event stream (expected 0).

## Accepted, by delivery (valid sessions)

| arm | delivered: accepted | not delivered: accepted |
|---|---|---|
| cold | 0/0 | 27/30 |
| notes | 30/30 | 0/0 |
| av | 24/30 | 0/0 |

## Per session

| run | task | arm | rep | status | accepted | wrote wrong edit | delivered | turns | cost |
|---|---|---|---|---|---|---|---|---|---|
| cold | relay-coupon-remove | cold | 1 | ok | yes | no | no | 8 | $0.0069 |
| cold | relay-line-taxes | cold | 1 | ok | yes | no | no | 23 | $0.0216 |
| cold | relay-money-plain | cold | 1 | ok | yes | no | no | 10 | $0.0068 |
| cold | relay-quote-split | cold | 1 | ok | yes | no | no | 15 | $0.0095 |
| cold | relay-store-count | cold | 1 | ok | yes | no | no | 19 | $0.0101 |
| cold | relay-coupon-remove | cold | 2 | ok | yes | no | no | 10 | $0.0069 |
| cold | relay-line-taxes | cold | 2 | ok | yes | no | no | 29 | $0.0198 |
| cold | relay-money-plain | cold | 2 | ok | yes | no | no | 14 | $0.0084 |
| cold | relay-quote-split | cold | 2 | ok | yes | no | no | 14 | $0.0095 |
| cold | relay-store-count | cold | 2 | ok | yes | no | no | 16 | $0.0091 |
| cold | relay-coupon-remove | cold | 3 | ok | yes | no | no | 8 | $0.0062 |
| cold | relay-line-taxes | cold | 3 | ok | no | yes | no | 24 | $0.0182 |
| cold | relay-money-plain | cold | 3 | ok | yes | no | no | 7 | $0.0062 |
| cold | relay-quote-split | cold | 3 | ok | yes | no | no | 15 | $0.0088 |
| cold | relay-store-count | cold | 3 | ok | yes | no | no | 16 | $0.0087 |
| cold | relay-coupon-remove | cold | 4 | ok | yes | no | no | 10 | $0.0069 |
| cold | relay-line-taxes | cold | 4 | ok | yes | no | no | 39 | $0.0294 |
| cold | relay-money-plain | cold | 4 | ok | yes | no | no | 14 | $0.0082 |
| cold | relay-quote-split | cold | 4 | ok | yes | no | no | 16 | $0.0092 |
| cold | relay-store-count | cold | 4 | ok | yes | no | no | 18 | $0.0094 |
| cold | relay-coupon-remove | cold | 5 | ok | yes | no | no | 7 | $0.0063 |
| cold | relay-line-taxes | cold | 5 | ok | no | yes | no | 24 | $0.0173 |
| cold | relay-money-plain | cold | 5 | ok | yes | no | no | 9 | $0.0066 |
| cold | relay-quote-split | cold | 5 | ok | yes | no | no | 15 | $0.0095 |
| cold | relay-store-count | cold | 5 | ok | yes | no | no | 15 | $0.0087 |
| cold | relay-coupon-remove | cold | 6 | ok | yes | no | no | 11 | $0.0071 |
| cold | relay-line-taxes | cold | 6 | ok | no | yes | no | 28 | $0.0199 |
| cold | relay-money-plain | cold | 6 | ok | yes | no | no | 11 | $0.0069 |
| cold | relay-quote-split | cold | 6 | ok | yes | no | no | 19 | $0.0098 |
| cold | relay-store-count | cold | 6 | ok | yes | no | no | 17 | $0.0096 |
| wrong | relay-coupon-remove | av | 1 | ok | yes | no | yes | 7 | $0.0063 |
| wrong | relay-coupon-remove | notes | 1 | ok | yes | no | yes | 10 | $0.0084 |
| wrong | relay-line-taxes | notes | 1 | ok | yes | no | yes | 40 | $0.0371 |
| wrong | relay-line-taxes | av | 1 | ok | yes | no | yes | 35 | $0.0320 |
| wrong | relay-money-plain | av | 1 | ok | yes | no | yes | 18 | $0.0114 |
| wrong | relay-money-plain | notes | 1 | ok | yes | no | yes | 16 | $0.0119 |
| wrong | relay-quote-split | notes | 1 | ok | yes | no | yes | 19 | $0.0127 |
| wrong | relay-quote-split | av | 1 | ok | no | yes | yes | 19 | $0.0122 |
| wrong | relay-store-count | notes | 1 | ok | yes | no | yes | 18 | $0.0132 |
| wrong | relay-store-count | av | 1 | ok | yes | no | yes | 19 | $0.0124 |
| wrong | relay-coupon-remove | av | 2 | ok | yes | no | yes | 9 | $0.0074 |
| wrong | relay-coupon-remove | notes | 2 | ok | yes | no | yes | 11 | $0.0089 |
| wrong | relay-line-taxes | notes | 2 | ok | yes | no | yes | 31 | $0.0255 |
| wrong | relay-line-taxes | av | 2 | ok | yes | no | yes | 31 | $0.0418 |
| wrong | relay-money-plain | av | 2 | ok | yes | no | yes | 15 | $0.0103 |
| wrong | relay-money-plain | notes | 2 | ok | yes | no | yes | 15 | $0.0117 |
| wrong | relay-quote-split | av | 2 | ok | no | yes | yes | 16 | $0.0107 |
| wrong | relay-quote-split | notes | 2 | ok | yes | no | yes | 19 | $0.0162 |
| wrong | relay-store-count | notes | 2 | ok | yes | no | yes | 15 | $0.0118 |
| wrong | relay-store-count | av | 2 | ok | yes | no | yes | 16 | $0.0108 |
| wrong | relay-coupon-remove | av | 3 | ok | yes | no | yes | 8 | $0.0068 |
| wrong | relay-coupon-remove | notes | 3 | ok | yes | no | yes | 9 | $0.0078 |
| wrong | relay-line-taxes | notes | 3 | ok | yes | no | yes | 42 | $0.0364 |
| wrong | relay-line-taxes | av | 3 | ok | yes | yes | yes | 32 | $0.0263 |
| wrong | relay-money-plain | av | 3 | ok | yes | no | yes | 13 | $0.0093 |
| wrong | relay-money-plain | notes | 3 | ok | yes | no | yes | 9 | $0.0090 |
| wrong | relay-quote-split | notes | 3 | ok | yes | no | yes | 17 | $0.0131 |
| wrong | relay-quote-split | av | 3 | ok | no | no | yes | 18 | $0.0121 |
| wrong | relay-store-count | notes | 3 | ok | yes | no | yes | 21 | $0.0124 |
| wrong | relay-store-count | av | 3 | ok | yes | no | yes | 17 | $0.0119 |
| wrong | relay-coupon-remove | notes | 4 | ok | yes | no | yes | 10 | $0.0078 |
| wrong | relay-coupon-remove | av | 4 | ok | yes | no | yes | 9 | $0.0071 |
| wrong | relay-line-taxes | av | 4 | ok | yes | no | yes | 24 | $0.0178 |
| wrong | relay-line-taxes | notes | 4 | ok | yes | no | yes | 37 | $0.0264 |
| wrong | relay-money-plain | notes | 4 | ok | yes | no | yes | 14 | $0.0095 |
| wrong | relay-money-plain | av | 4 | ok | yes | no | yes | 12 | $0.0086 |
| wrong | relay-quote-split | notes | 4 | ok | yes | no | yes | 23 | $0.0155 |
| wrong | relay-quote-split | av | 4 | ok | no | no | yes | 12 | $0.0087 |
| wrong | relay-store-count | av | 4 | ok | yes | no | yes | 17 | $0.0114 |
| wrong | relay-store-count | notes | 4 | ok | yes | no | yes | 21 | $0.0159 |
| wrong | relay-coupon-remove | av | 5 | ok | yes | no | yes | 9 | $0.0077 |
| wrong | relay-coupon-remove | notes | 5 | ok | yes | no | yes | 9 | $0.0090 |
| wrong | relay-line-taxes | av | 5 | ok | yes | yes | yes | 42 | $0.0333 |
| wrong | relay-line-taxes | notes | 5 | ok | yes | no | yes | 40 | $0.0435 |
| wrong | relay-money-plain | notes | 5 | ok | yes | no | yes | 11 | $0.0082 |
| wrong | relay-money-plain | av | 5 | ok | yes | no | yes | 12 | $0.0101 |
| wrong | relay-quote-split | av | 5 | ok | no | yes | yes | 13 | $0.0097 |
| wrong | relay-quote-split | notes | 5 | ok | yes | no | yes | 18 | $0.0130 |
| wrong | relay-store-count | notes | 5 | ok | yes | no | yes | 18 | $0.0125 |
| wrong | relay-store-count | av | 5 | ok | yes | no | yes | 17 | $0.0105 |
| wrong | relay-coupon-remove | av | 6 | ok | yes | no | yes | 9 | $0.0075 |
| wrong | relay-coupon-remove | notes | 6 | ok | yes | no | yes | 13 | $0.0096 |
| wrong | relay-line-taxes | notes | 6 | ok | yes | no | yes | 40 | $0.0308 |
| wrong | relay-line-taxes | av | 6 | ok | yes | no | yes | 37 | $0.0300 |
| wrong | relay-money-plain | av | 6 | ok | yes | no | yes | 10 | $0.0075 |
| wrong | relay-money-plain | notes | 6 | ok | yes | no | yes | 12 | $0.0095 |
| wrong | relay-quote-split | av | 6 | ok | no | yes | yes | 15 | $0.0096 |
| wrong | relay-quote-split | notes | 6 | ok | yes | no | yes | 21 | $0.0167 |
| wrong | relay-store-count | notes | 6 | ok | yes | no | yes | 19 | $0.0144 |
| wrong | relay-store-count | av | 6 | ok | yes | no | yes | 19 | $0.0123 |
