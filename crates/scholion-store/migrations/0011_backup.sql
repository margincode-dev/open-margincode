-- Plan 3 Task 6: git backup push of each repo's trunk. A failed push never fails a merge; it is
-- recorded here and retried with backoff from `next_retry`.
CREATE TABLE backup_state (
    repo               text PRIMARY KEY,
    remote             text,
    last_pushed_change text,
    failures           integer NOT NULL DEFAULT 0,
    last_error         text,
    next_retry         timestamptz,
    updated_at         timestamptz NOT NULL DEFAULT now()
);
