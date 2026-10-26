-- Semantic outbox and code-chunk text. Postgres stays the source of truth;
-- a worker drains `outbox` into Qdrant. `code_chunks` is filled by the indexer.

CREATE TABLE outbox (
    id            bigserial PRIMARY KEY,
    kind          text NOT NULL CHECK (kind IN ('record','convention','code_chunk')),
    op            text NOT NULL CHECK (op IN ('upsert','delete','supersede')),
    entity_id     text NOT NULL,
    repo          text NOT NULL,
    task_id       uuid,
    record_id     uuid,
    symbol_id     bigint,
    convention_id bigint,
    path          text,
    change_id     text,
    superseded    boolean NOT NULL DEFAULT false,
    text          text,
    attempts      integer NOT NULL DEFAULT 0,
    last_error    text,
    locked_until  timestamptz,
    created_at    timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX outbox_claim ON outbox (id);

CREATE TABLE code_chunks (
    symbol_id bigint PRIMARY KEY REFERENCES symbols(id) ON DELETE CASCADE,
    repo      text NOT NULL,
    scope     text NOT NULL,
    path      text NOT NULL,
    task_id   uuid,
    change_id text,
    text      text NOT NULL
);
CREATE INDEX code_chunks_repo ON code_chunks (repo);
