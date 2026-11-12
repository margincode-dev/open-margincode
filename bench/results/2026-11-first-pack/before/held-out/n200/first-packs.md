# First packs: is the task's own lesson in them?

Noise notes: 200. Tasks: 10. No agent ran.

| task | records seeded | pack at task start | hook pack | records in hook pack | omitted | relevant files named | pre-read (file: lesson, records) |
|---|---|---|---|---|---|---|---|
| relay-coupon-remove | 210 | absent | handle | 7 | 0 | 1/1 | tallyho/pricing/coupons.py: yes, 7 |
| relay-customer-region | 210 | absent | handle | 4 | 83 | 2/2 | tallyho/models.py: no, 7; tallyho/validate.py: yes, 4 |
| relay-ledger-refund | 210 | handle | summary | 4 | 110 | 1/1 | tallyho/ledger.py: no, 7 |
| relay-line-taxes | 210 | summary | summary | 4 | 87 | 1/1 | tallyho/pricing/quote.py: no, 6 |
| relay-money-plain | 210 | absent | handle | 6 | 6 | 1/1 | tallyho/money.py: yes, 6 |
| relay-order-ids | 210 | absent | handle | 7 | 0 | 1/1 | tallyho/repository.py: yes, 8 |
| relay-product-slug | 210 | absent | handle | 5 | 6 | 1/1 | tallyho/models.py: yes, 7 |
| relay-quote-split | 210 | omitted #6 | handle | 3 | 88 | 1/1 | tallyho/pricing/quote.py: no, 7 |
| relay-refund-event | 210 | absent | handle | 4 | 94 | 2/2 | tallyho/workflow.py: yes, 7; tallyho/models.py: no, 7 |
| relay-store-count | 210 | absent | omitted #1 | 4 | 55 | 3/3 | tallyho/store/base.py: yes, 7; tallyho/store/memory.py: no, 5; tallyho/store/jsonfile.py: no, 7 |

Lesson shown in the pack at task start: 2 of 10. In the hook pack: 9 of 10. In the hook pack or a pre-read answer: 10 of 10.

Lesson shown in full (the hook pack at the summary or full tier, or a pre-read answer): 9 of 10. Relevant files named in the hook pack: 14 of 14. Packs marked degraded: 0.
