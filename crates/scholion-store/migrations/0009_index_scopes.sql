-- The change an index scope fully reflects. Any write to the scope deletes its row (the index
-- code does that in the same transaction); a complete index of one change writes it. Task start
-- copies `main`'s rows when main is recorded at the task's base change, instead of re-indexing.
CREATE TABLE index_scopes (
    repo      text NOT NULL,
    scope     text NOT NULL,
    change_id text NOT NULL,
    PRIMARY KEY (repo, scope)
);
