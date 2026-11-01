-- A team's limit on open tasks. An agent CLI that runs subagents can take work in parallel; with a
-- limit, `next` stops giving it new work once it holds this many open tasks. NULL: no limit.
ALTER TABLE agent_profiles
    ADD COLUMN max_open integer CHECK (max_open IS NULL OR max_open >= 1);
