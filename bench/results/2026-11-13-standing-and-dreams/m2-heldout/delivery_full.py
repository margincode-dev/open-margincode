"""Per arm and task: sessions whose transcript holds the last 60 characters of the task's own
wrong note, the advice part a handle line cuts off (so the note reached the agent in full).

Usage: python3 delivery_full.py RUNS_DIR POOL
"""

import json
import sys
import tomllib
from pathlib import Path

runs, pool = Path(sys.argv[1]), Path(sys.argv[2])
wrong = {"relay-" + n["pair"]: n["summary"]
         for n in tomllib.loads(pool.read_text())["note"] if n.get("no_commit")}
for run_id in ["m2rawho", "m2dreamedho"]:
    for task, summary in sorted(wrong.items()):
        tail = json.dumps(summary[-60:])[1:-1]
        dirs = [d for d in (runs / run_id).iterdir() if d.name.startswith(task + "-av-")]
        k = sum(tail in (d / "events.jsonl").read_text() for d in dirs)
        print(f"{run_id} {task}: the note's last 60 characters in the transcript {k}/{len(dirs)}")
