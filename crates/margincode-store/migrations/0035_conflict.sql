-- A `conflict` verdict says a record tells future work the opposite of a rule in force: an
-- article (`art:N`, `org:N`) or a convention that covers its anchors. `against` is the rule's
-- `conventions` row, not its handle: an amendment retires that row and a removal sets its
-- `removed_at`, so a conflict counts only while its row is in force (`VERDICT_STATE` calls it
-- `lapsed` after that) and no write path has to remember it. No other verdict names a rule.
ALTER TABLE record_verdicts
    DROP CONSTRAINT record_verdicts_verdict_check,
    ADD CONSTRAINT record_verdicts_verdict_check CHECK (verdict IN
        ('confirmed', 'disputed', 'duplicate', 'outdated', 'promote', 'conflict', 'retracted')),
    ADD COLUMN against bigint REFERENCES conventions,
    ADD CONSTRAINT record_verdicts_conflict CHECK ((verdict = 'conflict') = (against IS NOT NULL));
CREATE INDEX record_verdicts_against ON record_verdicts (against) WHERE against IS NOT NULL;
