-- Dreams (Plan 16 Phase 2): an offline pass in which an agent looks again at records the daemon
-- chose and proposes verdicts on them. A verdict judges a record and never edits it; it has no
-- effect until a review by another run, or a person, agrees. Everything here is append-only
-- but a dream's own status: a verdict is taken back by a `retracted` row that names it.
CREATE TABLE dreams (
    id               uuid PRIMARY KEY,
    repo             text NOT NULL,
    mode             text NOT NULL CHECK (mode IN ('review', 'verify', 'pages')),
    -- a path prefix; '' is the whole repository
    area             text NOT NULL DEFAULT '',
    agent            text NOT NULL,
    instructions_sha text,
    status           text NOT NULL DEFAULT 'running'
                     CHECK (status IN ('running', 'done', 'failed', 'undone')),
    -- what the run may judge: {commit, records: [id]} for review, {verdicts: [id]} for verify
    worklist         jsonb NOT NULL,
    counts           jsonb NOT NULL DEFAULT '{}',
    started_at       timestamptz NOT NULL DEFAULT now(),
    finished_at      timestamptz,
    lease_expires    timestamptz NOT NULL
);
CREATE INDEX dreams_repo ON dreams (repo, started_at);
CREATE UNIQUE INDEX dreams_running_area ON dreams (repo, mode, area) WHERE status = 'running';

CREATE TABLE record_verdicts (
    id            uuid PRIMARY KEY,
    record_id     uuid NOT NULL REFERENCES records (id) ON DELETE CASCADE,
    verdict       text NOT NULL CHECK (verdict IN
                  ('confirmed', 'disputed', 'duplicate', 'outdated', 'promote', 'retracted')),
    reason        text NOT NULL DEFAULT '' CHECK (char_length(reason) <= 200),
    -- for 'duplicate': the live record that says the same and is shown in its place
    canonical     uuid REFERENCES records (id) ON DELETE CASCADE,
    -- for 'retracted': the verdict taken back
    retracts      uuid REFERENCES record_verdicts (id) ON DELETE CASCADE,
    evidence      jsonb NOT NULL DEFAULT '[]',
    -- for 'confirmed': per anchor of the record, the hash of the anchored code the verdict saw
    anchor_hashes jsonb NOT NULL DEFAULT '[]',
    author_agent  text NOT NULL,
    dream_id      uuid REFERENCES dreams (id) ON DELETE CASCADE,
    created_at    timestamptz NOT NULL DEFAULT now(),
    CHECK ((verdict = 'retracted') = (retracts IS NOT NULL)),
    CHECK ((verdict = 'duplicate') = (canonical IS NOT NULL))
);
CREATE INDEX record_verdicts_record ON record_verdicts (record_id, created_at);
CREATE INDEX record_verdicts_dream ON record_verdicts (dream_id);
CREATE UNIQUE INDEX record_verdicts_retracts ON record_verdicts (retracts) WHERE retracts IS NOT NULL;

CREATE TABLE verdict_reviews (
    id         bigserial PRIMARY KEY,
    verdict_id uuid NOT NULL REFERENCES record_verdicts (id) ON DELETE CASCADE,
    reviewer   text NOT NULL,
    -- the verify run that reviewed it; NULL when a person did
    dream_id   uuid REFERENCES dreams (id) ON DELETE CASCADE,
    agree      boolean NOT NULL,
    evidence   jsonb NOT NULL DEFAULT '[]',
    created_at timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX verdict_reviews_verdict ON verdict_reviews (verdict_id, id);
