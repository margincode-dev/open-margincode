-- Context receipts (Plan 2b): what each pack sent, and why.
CREATE TABLE context_receipts (
    id             uuid PRIMARY KEY,
    task_id        uuid NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    session        text NOT NULL,
    head_change    text NOT NULL,
    packer_version integer NOT NULL,
    budget         integer NOT NULL,
    tokens         integer NOT NULL,
    items          jsonb NOT NULL,
    omitted        jsonb NOT NULL,
    created_at     timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX context_receipts_task ON context_receipts (task_id, created_at);

-- A record may cite the receipt of the pack its author was working from.
ALTER TABLE records ADD COLUMN receipt_id uuid;
