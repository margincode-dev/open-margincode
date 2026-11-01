-- An extension's first migration (test fixture): an unqualified table lands in the extension's
-- schema, and `tasks` still resolves to the daemon's table in `public`.
CREATE TABLE board (
    id   integer PRIMARY KEY,
    task uuid REFERENCES tasks (id) ON DELETE CASCADE
);
