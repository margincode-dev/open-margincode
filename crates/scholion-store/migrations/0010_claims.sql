-- Plan 3 Task 3: claims. `generation` fences writes: every claim increments it, and a write that
-- carries an older one is refused. `lease_expires` lets another agent take over an open task whose
-- owner stopped renewing it.
ALTER TABLE tasks
    ADD COLUMN generation    bigint NOT NULL DEFAULT 1,
    ADD COLUMN lease_expires timestamptz;
