-- A backlog item can start after other items: `after_ids` lists the backlog items that must be
-- merged before `next` offers it. Resolved to ids when the item is posted, so every id exists and
-- no cycle is possible.
ALTER TABLE backlog
    ADD COLUMN after_ids uuid[] NOT NULL DEFAULT '{}';
