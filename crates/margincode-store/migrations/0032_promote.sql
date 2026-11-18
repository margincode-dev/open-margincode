-- Plan 16: a `promote` verdict proposes a repository convention from a decision. It carries the
-- convention's text and path glob. Only a person's acceptance gives it effect: a `conventions`
-- row whose `source` is `vd:<verdict id>`, which a retraction of the verdict removes
-- (`removed_at`). A convention written by hand keeps `source = ''` and is never touched.
ALTER TABLE record_verdicts
    ADD COLUMN convention_text text,
    ADD COLUMN scope_glob text,
    ADD CONSTRAINT record_verdicts_promote
        CHECK ((verdict = 'promote') = (convention_text IS NOT NULL AND scope_glob IS NOT NULL));
CREATE INDEX conventions_source ON conventions (source) WHERE source <> '';
