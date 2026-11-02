-- Who owns an agent profile. NULL: the user, through `av agent set` / `PUT /v1/agents/{name}`.
-- A name: an extension writes the profile (`AppState::put_profile`) and the API refuses to
-- overwrite it, so the extension's own notion of the agent is never lost to a stray put.
ALTER TABLE agent_profiles ADD COLUMN managed_by text;
