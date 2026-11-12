-- Propose mode: an item an agent posts but is not sure of waits as `proposed` until a human
-- accepts it (then `queued`, and `next` may offer it). `why` says why the poster was unsure.
-- `repo_settings.agent_posts` = 'propose' makes every agent's post land as `proposed`.
ALTER TABLE backlog
    DROP CONSTRAINT backlog_state_check,
    ADD CONSTRAINT backlog_state_check
        CHECK (state IN ('proposed', 'queued', 'taken', 'dropped')),
    ADD COLUMN why text CHECK (char_length(why) <= 300),
    ADD COLUMN accepted_by text,
    ADD COLUMN accepted_at timestamptz;

ALTER TABLE repo_settings
    ADD COLUMN agent_posts text NOT NULL DEFAULT 'queue' CHECK (agent_posts IN ('queue', 'propose'));
