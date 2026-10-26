-- Plan 5: results of post-merge checks, posted by `av runner` (the daemon never runs commands).
-- One row per commit and command; a re-run replaces it.
CREATE TABLE checks (
    id bigserial PRIMARY KEY,
    repo text NOT NULL,
    change text NOT NULL,
    command text NOT NULL,
    exit_code integer NOT NULL,
    output_tail text NOT NULL DEFAULT '',
    runner text NOT NULL DEFAULT '',
    at timestamptz NOT NULL DEFAULT now(),
    UNIQUE (repo, change, command)
);
