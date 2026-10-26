-- Plan 3 Tasks 4-5: review verdicts, validation records, merge state.
ALTER TABLE records DROP CONSTRAINT records_kind_check;
ALTER TABLE records ADD CONSTRAINT records_kind_check
    CHECK (kind IN ('intent','decision','attempt','note','handoff','review','validation'));

-- Append-only. Merge is authorized only when the latest verdict at the task's current head is 'approve'.
CREATE TABLE review_verdicts (
    id          bigserial PRIMARY KEY,
    task_id     uuid NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    reviewer    text NOT NULL,
    verdict     text NOT NULL CHECK (verdict IN ('approve','request_changes')),
    head_change text NOT NULL,
    created_at  timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX review_verdicts_task ON review_verdicts (task_id, head_change, id);

ALTER TABLE tasks
    ADD COLUMN pre_merge_commit text,
    ADD COLUMN merge_commit     text,
    ADD COLUMN merged_by        text,
    ADD COLUMN merged_at        timestamptz,
    ADD COLUMN conflicts        text[] NOT NULL DEFAULT '{}';
