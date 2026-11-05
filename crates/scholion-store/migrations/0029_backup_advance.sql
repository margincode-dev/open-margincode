-- Plan 15: advance trunk from the backup remote's main before each push. Off unless asked.
ALTER TABLE backup_state ADD COLUMN advance boolean NOT NULL DEFAULT false;
