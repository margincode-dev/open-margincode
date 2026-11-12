-- Plan 8 Task 3: routing. `agent_profiles` says what each agent is good at and what it costs.
-- `backlog` holds tasks waiting for an agent; a row becomes a normal task only when an agent takes
-- it. A handed-off task can carry the tags its next agent must have (`needs`, `prefer`), set by
-- `handoff` and cleared by `claim`.
CREATE TABLE agent_profiles (
    name      text PRIMARY KEY,
    best      text[] NOT NULL DEFAULT '{}',
    can       text[] NOT NULL DEFAULT '{}',
    cost_rank integer NOT NULL DEFAULT 5,
    aliases   text[] NOT NULL DEFAULT '{}'
);

CREATE TABLE backlog (
    id           uuid PRIMARY KEY,
    repo         text NOT NULL,
    title        text NOT NULL,
    goal         text NOT NULL,
    needs        text[] NOT NULL DEFAULT '{}',
    prefer       text,
    priority     integer NOT NULL DEFAULT 0,
    reserve_secs bigint NOT NULL DEFAULT 600,
    state        text NOT NULL DEFAULT 'queued' CHECK (state IN ('queued', 'taken', 'dropped')),
    task_id      uuid REFERENCES tasks (id),
    taken_by     text,
    posted_by    text NOT NULL,
    created_at   timestamptz NOT NULL DEFAULT now(),
    taken_at     timestamptz
);
CREATE INDEX backlog_by_state ON backlog (state, created_at);

ALTER TABLE tasks
    ADD COLUMN needs        text[] NOT NULL DEFAULT '{}',
    ADD COLUMN prefer       text,
    ADD COLUMN needs_set_at timestamptz;
