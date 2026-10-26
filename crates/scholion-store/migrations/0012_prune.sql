-- Plan 3 Task 9: prune idle session ledgers. `ledger_put` stamps the rows it writes.
ALTER TABLE ledger ADD COLUMN last_seen timestamptz NOT NULL DEFAULT now();
