# tallyho

Orders, pricing and ledgers for small shops.

- `tallyho.models`: products, customers, order lines, orders.
- `tallyho.pricing`: price rules, discounts and taxes.
- `tallyho.store`: a paginated record store with in-memory and JSON-file backends.
- `tallyho.ledger`: double-entry ledger built from orders.
- `tallyho.report`: text and CSV reports.
- `tallyho.cli`: the `tallyho` command.

Run the tests with `python -m pytest -q`.
