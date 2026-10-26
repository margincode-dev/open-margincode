-- Plan 5: approvals that survive a rebase. A verdict remembers the base it was given against, and
-- a carried approval (reviewer 'av') names the human-approved head it was carried from.
ALTER TABLE review_verdicts
    ADD COLUMN base_change text,
    ADD COLUMN carried_from text;

-- Per-repo policy: which files may differ after a conflict resolution without a new review, and
-- the post-merge check command (Plan 5 Task 6).
CREATE TABLE repo_settings (
    repo text PRIMARY KEY,
    carry_globs text[] NOT NULL DEFAULT '{**/*.md}',
    check_command text
);
