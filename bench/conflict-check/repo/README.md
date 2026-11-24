# tidepool

A small invoicing service: invoices with lines, discounts, refunds from the payment provider,
reminder emails and a nightly export for accounting.

- `api/`: the HTTP handlers and token checks.
- `billing/`: amounts, invoices, discounts, refunds.
- `jobs/`: the reminder job and the nightly export.
- `storage/`: every query to Postgres.

Run the tests with `pytest -q` from this directory.
