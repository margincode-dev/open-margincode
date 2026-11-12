-- Every imported repo: where it was imported from (the path as the daemon saw it) and when.
-- The jj store on disk holds the repo itself; this table is what `GET /v1/repos` lists. Repos
-- imported before this migration get their row when the daemon starts (`sync_repos`), with no
-- source.
CREATE TABLE repos (
    name       text PRIMARY KEY,
    git_source text,
    added_at   timestamptz NOT NULL DEFAULT now()
);
