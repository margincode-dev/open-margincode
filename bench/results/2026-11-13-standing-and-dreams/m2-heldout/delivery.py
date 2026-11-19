"""Per arm and task: sessions whose transcript (`events.jsonl`) holds the first 60 characters of
the task's own wrong note (it reached the agent, in a pack or a hook answer), and the same with
its `(disputed` mark. With `--edits TASK`, prints what each failed session of that task wrote for
its new method (the Edit/Write inputs that contain `def NAME`).

Usage: python3 delivery.py RUNS_DIR POOL [--edits TASK NAME]
"""

import json
import sys
import tomllib
from collections import Counter
from pathlib import Path

runs, pool = Path(sys.argv[1]), Path(sys.argv[2])
wrong = {f"relay-{n['pair']}": n["summary"][:60]
         for n in tomllib.loads(pool.read_text())["note"] if n.get("no_commit")}
edits = sys.argv[sys.argv.index("--edits") + 1:] if "--edits" in sys.argv else None

for run_id in ["m2rawho", "m2dreamedho"]:
    seen, marked, n = Counter(), Counter(), Counter()
    for d in sorted((runs / run_id).iterdir()):
        task = d.name.rsplit("-av-", 1)[0]
        text = (d / "events.jsonl").read_text()
        n[task] += 1
        # the transcript is JSON, so quotes inside a summary are escaped
        needle = json.dumps(wrong[task])[1:-1]
        seen[task] += needle in text
        marked[task] += any(needle in line and "(disputed" in line for line in text.splitlines())
        if edits and task == edits[0]:
            outcome = json.loads((d / "outcome.json").read_text())
            if outcome.get("verify_exit") != 0:
                for line in text.splitlines():
                    e = json.loads(line)
                    if e.get("type") != "assistant":
                        continue
                    for c in e["message"].get("content") or []:
                        if c.get("type") == "tool_use" and c.get("name") in ("Edit", "Write", "MultiEdit"):
                            s = json.dumps(c["input"])
                            if f"def {edits[1]}" in s:
                                body = c["input"].get("new_string") or c["input"].get("content") or s
                                i = body.find(f"def {edits[1]}")
                                print(f"--- {run_id} {d.name}:\n{body[i:i + 700]}")
    for task in sorted(n):
        print(f"{run_id} {task}: wrong note in the transcript {seen[task]}/{n[task]}, "
              f"with its disputed mark {marked[task]}/{n[task]}")
