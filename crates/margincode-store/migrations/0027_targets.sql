-- Plan 10 A3: targets. A target is something a team works toward in a repo (a release, a
-- milestone). A backlog item counts toward at most one; a target's progress is what became of
-- its items. Targets are never deleted: they are done or dropped, and can be made active again.
CREATE TABLE targets (
    id         uuid PRIMARY KEY,
    repo       text NOT NULL,
    title      text NOT NULL CHECK (char_length(title) BETWEEN 1 AND 200),
    detail     text NOT NULL DEFAULT '',
    status     text NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'done', 'dropped')),
    due        date,
    created_by text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX targets_repo ON targets (repo, created_at);

ALTER TABLE backlog
    ADD COLUMN target_id uuid REFERENCES targets (id);
CREATE INDEX backlog_target ON backlog (target_id);
