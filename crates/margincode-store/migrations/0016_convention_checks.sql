-- Plan 5: a required convention may carry a check command; approval needs it to pass at the
-- head. Conventions can be removed (kept for history, hidden from packs and checks).
ALTER TABLE conventions
    ADD COLUMN check_command text,
    ADD COLUMN removed_at timestamptz;
