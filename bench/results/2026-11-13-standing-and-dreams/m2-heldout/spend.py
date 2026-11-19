"""Money spent so far by this run (amendment 3's money rule): the freeze's Claude Code
`total_cost_usd` over every `claude-result-*.json` under its dream directories, plus each
command's session costs from its `rows.jsonl`. Prints one line per part and the total.

Usage: python3 spend.py /m2b/r3
"""

import json
import sys
from pathlib import Path

root = Path(sys.argv[1])
total = 0.0
calls = []
for f in sorted((root / "bench-work").glob("df*/*/dreams/*/claude-result-*.json")):
    calls.append(json.loads(f.read_text())["total_cost_usd"])
if calls:
    print(f"freeze: ${sum(calls):.4f} over {len(calls)} calls ({', '.join(f'{c:.4f}' for c in calls)})")
    total += sum(calls)
for out in ["smoke-r3", "m2-raw-ho", "m2-dreamed-ho", "m2-raw-m0c", "m2-dreamed-m0c", "m2-cold-ho"]:
    rows = root / "results" / out / "rows.jsonl"
    if rows.exists():
        cost = [json.loads(l).get("cost_usd") or 0.0 for l in rows.read_text().splitlines() if l]
        print(f"{out}: ${sum(cost):.4f} over {len(cost)} rows")
        total += sum(cost)
print(f"total: ${total:.4f}; left under the $5.00 ceiling: ${5.0 - total:.4f}")
