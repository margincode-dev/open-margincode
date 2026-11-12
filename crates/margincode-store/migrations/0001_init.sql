CREATE TABLE tasks (
    id            uuid PRIMARY KEY,
    repo          text NOT NULL,
    slug          text NOT NULL,
    title         text NOT NULL,
    goal          text NOT NULL,
    status        text NOT NULL DEFAULT 'open'
                  CHECK (status IN ('open','handed_off','in_review','conflicted','merged','abandoned')),
    owner_agent   text NOT NULL,
    base_change   text NOT NULL DEFAULT '',
    head_change   text NOT NULL DEFAULT '',
    workspace_path text NOT NULL DEFAULT '',
    created_at    timestamptz NOT NULL DEFAULT now(),
    UNIQUE (repo, slug)
);

CREATE TABLE task_touched (
    task_id uuid NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    path    text NOT NULL,
    PRIMARY KEY (task_id, path)
);
CREATE INDEX task_touched_path ON task_touched (path);

CREATE TABLE records (
    id           uuid PRIMARY KEY,
    task_id      uuid NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    kind         text NOT NULL
                 CHECK (kind IN ('intent','decision','attempt','note','handoff','review')),
    summary      text NOT NULL CHECK (char_length(summary) <= 200),
    body         text NOT NULL DEFAULT '',
    change_id    text,
    supersedes   uuid REFERENCES records(id),
    author_agent text NOT NULL,
    created_at   timestamptz NOT NULL DEFAULT now(),
    -- full-text fallback for the semantic layer (used by Plan 2's degraded mode)
    tsv          tsvector GENERATED ALWAYS AS (to_tsvector('english', summary || ' ' || body)) STORED
);
CREATE INDEX records_task ON records (task_id);
CREATE INDEX records_supersedes ON records (supersedes);
CREATE INDEX records_tsv ON records USING gin (tsv);

CREATE TABLE anchors (
    record_id uuid NOT NULL REFERENCES records(id) ON DELETE CASCADE,
    path      text NOT NULL,
    symbol    text
);
CREATE INDEX anchors_record ON anchors (record_id);
CREATE INDEX anchors_path ON anchors (path);
CREATE INDEX anchors_symbol ON anchors (symbol) WHERE symbol IS NOT NULL;

CREATE TABLE conventions (
    id         bigserial PRIMARY KEY,
    repo       text NOT NULL,
    scope_glob text NOT NULL,
    text       text NOT NULL,
    source     text NOT NULL DEFAULT ''
);
CREATE INDEX conventions_repo ON conventions (repo);

CREATE TABLE ledger (
    session text NOT NULL,
    item_id text NOT NULL,
    tier    text NOT NULL CHECK (tier IN ('handle','summary','full')),
    PRIMARY KEY (session, item_id)
);
