-- Backup to a named branch: where main is pushed on the remote. NULL = the trunk's own name.
ALTER TABLE backup_state ADD COLUMN branch text;
