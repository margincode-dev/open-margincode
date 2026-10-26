-- Code index (owned by av-index, migrated here so one runner owns the schema).
-- A "scope" is "main" or a task slug; each scope is a complete, isolated view of the repo.

CREATE TABLE parse_cache (
    cache_key text PRIMARY KEY,          -- "<sha256 of content>:<file extension>"
    symbols   jsonb NOT NULL
);

CREATE TABLE index_files (
    repo         text NOT NULL,
    scope        text NOT NULL,
    path         text NOT NULL,
    content_hash text NOT NULL,
    PRIMARY KEY (repo, scope, path)
);

CREATE TABLE symbols (
    id         bigserial PRIMARY KEY,
    repo       text NOT NULL,
    scope      text NOT NULL,
    path       text NOT NULL,
    qname      text NOT NULL,
    name       text NOT NULL,            -- last segment of qname
    kind       text NOT NULL CHECK (kind IN ('function','method','type')),
    signature  text NOT NULL,
    start_line integer NOT NULL,
    end_line   integer NOT NULL,
    refs       text[] NOT NULL DEFAULT '{}',
    UNIQUE (repo, scope, path, qname)
);
CREATE INDEX symbols_by_name ON symbols (repo, scope, name);
CREATE INDEX symbols_refs_gin ON symbols USING gin (refs);

CREATE TABLE symbol_refs (
    src    bigint NOT NULL REFERENCES symbols(id) ON DELETE CASCADE,
    dst    bigint NOT NULL REFERENCES symbols(id) ON DELETE CASCADE,
    weight real   NOT NULL,
    PRIMARY KEY (src, dst)
);
CREATE INDEX symbol_refs_dst ON symbol_refs (dst);
