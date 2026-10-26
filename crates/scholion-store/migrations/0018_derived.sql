-- Plan 5: derived data (Scion's model): values computed from a commit and its parent's value,
-- keyed by the immutable commit id, so they never need invalidating (only pruning). The kind
-- carries a version (`blame_v1`) so a new algorithm never reads an old value.
CREATE TABLE derived (
    repo text NOT NULL,
    kind text NOT NULL,
    change text NOT NULL,
    key text NOT NULL,
    value jsonb NOT NULL,
    computed_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (repo, kind, change, key)
);
