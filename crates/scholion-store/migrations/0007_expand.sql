-- Plan 2b Task 4: expand is charged per session.
CREATE TABLE ledger_sessions (
    session       text PRIMARY KEY,
    expand_tokens bigint NOT NULL DEFAULT 0,
    last_seen     timestamptz NOT NULL DEFAULT now()
);
