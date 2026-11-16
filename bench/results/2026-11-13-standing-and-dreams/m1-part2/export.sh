#!/bin/bash
# M1 Part 2 store export (PREREG.md, Build, "Part 2"): M0's two queries, read once from each
# daemon's database after command 2. Run from the host: export.sh <dir holding m1-p0/ and m1-p1/>
set -euo pipefail
OUT=${1:?usage: export.sh DIR}
export MSYS_NO_PATHCONV=1
SUP="SELECT r.id, t.repo, r.task_id, r.author_agent, r.kind, r.summary, r.created_at,
       r.supersedes, s.summary AS superseded_summary
FROM records r JOIN tasks t ON t.id = r.task_id JOIN records s ON s.id = r.supersedes
WHERE r.supersedes IS NOT NULL ORDER BY t.repo, r.created_at"
REC="SELECT r.id, t.repo, r.task_id, r.author_agent, r.kind, r.summary, r.body, r.created_at,
       r.supersedes, s.summary AS superseded_summary
FROM records r JOIN tasks t ON t.id = r.task_id LEFT JOIN records s ON s.id = r.supersedes
WHERE r.author_agent NOT IN ('agent-a', 'agent-r', 'bench-merger')
  AND r.kind IN ('decision', 'attempt', 'note')
ORDER BY t.repo, r.created_at"
for b in p0 p1; do
  docker exec lp-pg psql -U av -d "m1$b" -At -c "SELECT row_to_json(x) FROM ($SUP) x" \
    > "$OUT/m1-$b/superseded.jsonl"
  docker exec lp-pg psql -U av -d "m1$b" -At -c "SELECT row_to_json(x) FROM ($REC) x" \
    > "$OUT/m1-$b/records.jsonl"
  echo "m1-$b: $(wc -l < "$OUT/m1-$b/superseded.jsonl") superseded, $(wc -l < "$OUT/m1-$b/records.jsonl") records"
done
