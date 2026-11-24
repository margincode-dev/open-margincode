-- The constitution: a short numbered list of rules (articles) that outrank conventions and
-- records, amended only by a person. An article is a required, repository-wide `conventions` row
-- with `article` (its number) and `title` set, so the packer, the checks at approval and the
-- review bundle read one type. `repo = ''` is the shared (organisation) level, which only an
-- extension writes; every other row with an empty repo is refused. Nothing is edited in place:
-- an amendment retires the row in force (`removed_at`) and inserts the next one, and every
-- change is a `constitution_amendments` row. A level's version is the count of its amendments,
-- and an article's number is never given again (the next one comes from the amendments).
ALTER TABLE conventions
    ADD COLUMN article integer CHECK (article IS NULL OR article > 0),
    ADD COLUMN title   text CHECK (title IS NULL OR char_length(title) <= 60);
ALTER TABLE conventions ADD CONSTRAINT article_shape CHECK (
    article IS NULL
    OR (required AND scope_glob = '**' AND char_length(title) BETWEEN 1 AND 60
        AND char_length(text) BETWEEN 1 AND 400));
ALTER TABLE conventions ADD CONSTRAINT shared_rows_are_articles CHECK (
    repo <> '' OR article IS NOT NULL);
CREATE UNIQUE INDEX conventions_article ON conventions (repo, article)
    WHERE article IS NOT NULL AND removed_at IS NULL;

CREATE TABLE constitution_amendments (
    repo       text NOT NULL,
    version    integer NOT NULL CHECK (version > 0),
    at         timestamptz NOT NULL DEFAULT now(),
    amended_by text NOT NULL CHECK (char_length(amended_by) > 0),
    action     text NOT NULL CHECK (action IN ('add', 'amend', 'repeal')),
    article    integer NOT NULL CHECK (article > 0),
    -- the row in force after it; NULL for a repeal
    convention bigint REFERENCES conventions,
    reason     text NOT NULL CHECK (char_length(reason) BETWEEN 1 AND 200),
    PRIMARY KEY (repo, version),
    CHECK ((action = 'repeal') = (convention IS NULL))
);
CREATE INDEX constitution_amendments_article ON constitution_amendments (repo, article, version);
