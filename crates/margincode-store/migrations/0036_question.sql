-- A `question` record asks a person; a `decision` that supersedes it is the answer, a `note`
-- withdraws it, so an open question is one nothing supersedes and tasks gain no state.
ALTER TABLE records DROP CONSTRAINT records_kind_check;
ALTER TABLE records ADD CONSTRAINT records_kind_check
    CHECK (kind IN ('intent','decision','attempt','note','handoff','review','validation','question'));

-- How far the agent goes without an answer. NULL: the daemon's default (`AV_ASK_LEVEL`). Not part
-- of `put_profile`'s upsert, so a profile put keeps it.
ALTER TABLE agent_profiles
    ADD COLUMN ask_level text CHECK (ask_level IN ('ask_first', 'balanced', 'carry_on'));
