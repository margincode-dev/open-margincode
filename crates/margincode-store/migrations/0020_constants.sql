-- Module-level constants are symbols too (`BULK_QTY = 10`, `const MAX: usize`), so code that uses
-- one links to the module defining it. Files parsed before this have none: drop the parse cache
-- and forget content hashes, so the next index of a scope (`av reindex` does main) parses again.
ALTER TABLE symbols DROP CONSTRAINT symbols_kind_check;
ALTER TABLE symbols ADD CONSTRAINT symbols_kind_check
    CHECK (kind IN ('function', 'method', 'type', 'constant'));
DELETE FROM parse_cache;
UPDATE index_files SET content_hash = '';
