-- Plan 7: a task's index scope becomes an overlay on `main`, holding rows only for the paths the
-- task's tree has differently. A path the task deleted keeps a tombstone file row and no symbols.
-- `main` never has tombstones. `scope_overlays.main_at` is the change of `main` the overlay was
-- last reconciled against.
ALTER TABLE index_files ADD COLUMN deleted boolean NOT NULL DEFAULT false;

CREATE TABLE scope_overlays (
    repo    text NOT NULL,
    scope   text NOT NULL,
    main_at text NOT NULL,
    PRIMARY KEY (repo, scope)
);
