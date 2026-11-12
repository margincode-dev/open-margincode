-- Plan 5: an append-only feed of task lifecycle events. Each row is written in the transaction of
-- the change it reports, under one advisory lock, so ids commit in order and `since` never skips.
CREATE TABLE events (
    id bigserial PRIMARY KEY,
    repo text NOT NULL,
    kind text NOT NULL,
    task_id uuid REFERENCES tasks (id) ON DELETE CASCADE,
    change text,
    payload jsonb NOT NULL DEFAULT '{}',
    at timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX events_repo_id ON events (repo, id);
