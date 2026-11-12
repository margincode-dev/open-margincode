-- Plan 4: `task_only` packs keep just the Task section (the benchmark's av_no_packs arm).
ALTER TABLE tasks ADD COLUMN pack_mode text NOT NULL DEFAULT 'full'
    CHECK (pack_mode IN ('full', 'task_only'));
