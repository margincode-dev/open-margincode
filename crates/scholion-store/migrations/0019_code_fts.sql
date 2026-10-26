-- Lexical code search (Plan 6): the goal's words find code chunks even without the semantic
-- layer. Non-alphanumerics become spaces (so paths and snake_case split into words), and a second
-- copy splits camelCase, so both `readurl` and `read`/`url` match `readUrl`.
ALTER TABLE code_chunks ADD COLUMN tsv tsvector GENERATED ALWAYS AS (
    to_tsvector('simple'::regconfig,
        regexp_replace(path || ' ' || text, '[^[:alnum:]]+', ' ', 'g'))
    || to_tsvector('simple'::regconfig,
        regexp_replace(regexp_replace(path || ' ' || text, '[^[:alnum:]]+', ' ', 'g'),
                       '([[:lower:][:digit:]])([[:upper:]])', '\1 \2', 'g'))
) STORED;

CREATE INDEX code_chunks_tsv ON code_chunks USING gin (tsv);
