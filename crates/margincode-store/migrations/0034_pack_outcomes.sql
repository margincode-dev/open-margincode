-- Learn from use: how good each merged task's packs were (`margincode_core::outcome`), and what
-- its agent opened through the daemon. A row of a merge that was later reverted or undone stays;
-- it leaves the totals while its task is no longer merged with this merge commit.
CREATE TABLE pack_outcomes (
    task_id               uuid PRIMARY KEY REFERENCES tasks(id) ON DELETE CASCADE,
    repo                  text NOT NULL,
    merge_commit          text,
    -- where the changed paths came from: the merge's diff, or task_touched when the VCS could
    -- no longer give it (a backfilled task)
    changed_source        text NOT NULL CHECK (changed_source IN ('vcs', 'touched')),
    packer_version        integer,
    first_budget          integer,
    packs                 integer NOT NULL,
    code_tokens           bigint NOT NULL,
    code_tokens_changed   bigint NOT NULL,
    code_tokens_used      bigint NOT NULL,
    files_changed         integer NOT NULL,
    files_missed          integer NOT NULL,
    missed_paths          text[] NOT NULL,
    files_named_only      integer NOT NULL,
    files_opened          integer NOT NULL,
    opened_not_sent       integer NOT NULL,
    opened_not_sent_paths text[] NOT NULL,
    computed_at           timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX pack_outcomes_repo ON pack_outcomes (repo, packer_version);

CREATE TABLE task_reads (
    task_id  uuid NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    path     text NOT NULL,
    reads    integer NOT NULL DEFAULT 1,
    first_at timestamptz NOT NULL DEFAULT now(),
    expanded boolean NOT NULL DEFAULT false,
    PRIMARY KEY (task_id, path)
);
