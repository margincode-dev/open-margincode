#!/bin/bash
# Amendment 2: the store export of PREREG.md ("The store export", and the records.jsonl query of
# its "Build"), one JSON object per line, from one build's database on lp-pg.
# usage: sonnet-export.sh DB OUTDIR   (DB m0sa or m0sb; run where `docker exec lp-pg` works)
set -euo pipefail
db=$1 out=$2
q() { docker exec lp-pg psql -U av -d "$db" -Atq -c "SELECT row_to_json(x) FROM ($1) x"; }
q "SELECT r.id, t.repo, r.task_id, r.author_agent, r.kind, r.summary, r.created_at,
          r.supersedes, s.summary AS superseded_summary
   FROM records r JOIN tasks t ON t.id = r.task_id JOIN records s ON s.id = r.supersedes
   WHERE r.supersedes IS NOT NULL ORDER BY t.repo, r.created_at" > "$out/superseded.jsonl"
q "SELECT r.id, t.repo, r.task_id, r.author_agent, r.kind, r.summary, r.body, r.created_at,
          r.supersedes, s.summary AS superseded_summary
   FROM records r JOIN tasks t ON t.id = r.task_id LEFT JOIN records s ON s.id = r.supersedes
   WHERE r.author_agent <> 'agent-a' AND r.kind IN ('decision', 'attempt', 'note')
   ORDER BY t.repo, r.created_at" > "$out/records.jsonl"
wc -l "$out/superseded.jsonl" "$out/records.jsonl"
