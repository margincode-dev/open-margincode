-- Plan 16: a `promote` verdict proposes a repository convention from a decision. It carries the
-- convention's text (at most 400 characters) and path glob; no other verdict carries either.
-- Only a person's acceptance gives it effect: a `conventions` row whose `source` is
-- `vd:<verdict id>`, at most one live per verdict, which a retraction of the verdict removes
-- (`removed_at`). A convention written by hand keeps `source = ''` and is never touched.
-- A run proposes at most one promote per record (the daemon holds every kind to one verdict
-- per record and run, under a lock on the record; this index backs it for promotes).
ALTER TABLE record_verdicts
    ADD COLUMN convention_text text CHECK (char_length(convention_text) <= 400),
    ADD COLUMN scope_glob text,
    ADD CONSTRAINT record_verdicts_promote CHECK (CASE WHEN verdict = 'promote'
        THEN convention_text IS NOT NULL AND scope_glob IS NOT NULL
        ELSE convention_text IS NULL AND scope_glob IS NULL END);
CREATE UNIQUE INDEX record_verdicts_promote_per_run ON record_verdicts (record_id, dream_id)
    WHERE verdict = 'promote';
CREATE UNIQUE INDEX conventions_promoted ON conventions (source)
    WHERE source <> '' AND removed_at IS NULL;
