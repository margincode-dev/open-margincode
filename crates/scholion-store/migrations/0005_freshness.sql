-- Freshness of anchored knowledge (Plan 2b Task 2): hash of the anchored code when the record was written.
ALTER TABLE anchors ADD COLUMN content_hash text;
