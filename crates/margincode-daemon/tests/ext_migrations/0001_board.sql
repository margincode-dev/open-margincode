-- An extension's first migration (test fixture), run in the extension's own schema.
CREATE TABLE board (
    task uuid PRIMARY KEY REFERENCES tasks (id) ON DELETE CASCADE,
    note text NOT NULL
);
