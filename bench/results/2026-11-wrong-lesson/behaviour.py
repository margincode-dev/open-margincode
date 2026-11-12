"""Post hoc: what the agents did with the wrong note, per task and arm, from the event streams.

- `edited NOTES.md`: an Edit/Write/MultiEdit call on `NOTES.md` (the notes arm can rewrite a note).
- `av record`: a Bash call that runs `av record` (the stop hook asks for it).
- `supersedes`: any tool call whose input mentions superseding a record.
- `recorded an attempt`: an `av record` call with a record of kind `attempt` (a dead end; on
  these tasks, usually the note's approach, rejected).
- `restates the wrong edit`: an `av record` call whose text matches the task's `[[dead_end]]`
  pattern, or an edit of `NOTES.md` that does (the wrong approach written down again).
- `final edit is the wrong one`: the last Edit/Write to the task's file matches the pattern.

Usage: python3 bench/results/2026-11-wrong-lesson/behaviour.py [RUNS]
"""

import collections
import fnmatch
import json
import re
import sys
import tomllib
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
RUNS = Path(sys.argv[1] if len(sys.argv) > 1 else "/root/bench-work/runs")
ARMS = ["cold", "notes", "av"]
COLS = ["n", "accepted", "edited NOTES.md", "av record", "supersedes", "recorded an attempt",
        "restates the wrong edit",
        "final edit is the wrong one"]

specs = {}
for p in (ROOT / "bench/tasks-relay-wrong").glob("*.toml"):
    s = tomllib.loads(p.read_text(encoding="utf-8"))
    specs[s["name"]] = [(d["file_glob"], re.compile(d["pattern"])) for d in s["dead_end"]]

stat = collections.defaultdict(collections.Counter)
for run in ("cold", "wrong"):
    for line in (HERE / run / "rows.jsonl").read_text(encoding="utf-8").splitlines():
        r = json.loads(line)
        d = RUNS / run / f"{r['task']}-{r['arm']}-r{r['rep']}"
        pats = specs[r["task"]]
        flags = collections.Counter()
        last_wrong = False
        for ev in (d / "events.jsonl").read_text(encoding="utf-8", errors="replace").splitlines():
            try:
                e = json.loads(ev)
            except json.JSONDecodeError:
                continue
            if e.get("type") != "assistant":
                continue
            for c in (e.get("message") or {}).get("content") or []:
                if not (isinstance(c, dict) and c.get("type") == "tool_use"):
                    continue
                i = c.get("input") or {}
                name = c.get("name")
                if "supersede" in json.dumps(i).lower():
                    flags["supersedes"] = 1
                if name in ("Edit", "Write", "MultiEdit"):
                    path = str(i.get("file_path", ""))
                    text = " ".join(str(x) for x in (i.get("new_string"), i.get("content"),
                                                    json.dumps(i.get("edits") or "")) if x)
                    if path.endswith("NOTES.md"):
                        flags["edited NOTES.md"] = 1
                        if any(rx.search(text) for _, rx in pats) and "not" not in text.lower():
                            flags["restates the wrong edit"] = 1
                    for glob, rx in pats:
                        if fnmatch.fnmatch(path, "*" + glob):
                            last_wrong = bool(rx.search(text)) if text else last_wrong
                elif name == "Bash":
                    cmd = str(i.get("command", ""))
                    if re.search(r"\bav record\b", cmd):
                        flags["av record"] = 1
                        if re.search(r"kind[^a-z]+attempt", cmd):
                            flags["recorded an attempt"] = 1
                        if any(rx.search(cmd) for _, rx in pats):
                            flags["restates the wrong edit"] = 1
        k = (r["task"], r["arm"])
        stat[k]["n"] += 1
        stat[k]["accepted"] += r["success"]
        stat[k]["final edit is the wrong one"] += last_wrong
        for f, v in flags.items():
            stat[k][f] += v

print("| task | arm | " + " | ".join(COLS) + " |")
print("|---|---|" + "---|" * len(COLS))
for task in sorted(specs):
    for arm in ARMS:
        s = stat[(task, arm)]
        print(f"| {task} | {arm} | " + " | ".join(str(s[c]) for c in COLS) + " |")
print("\n| arm | " + " | ".join(COLS) + " |")
print("|---|" + "---|" * len(COLS))
for arm in ARMS:
    tot = collections.Counter()
    for (t, a), s in stat.items():
        if a == arm:
            tot.update(s)
    print(f"| {arm} | " + " | ".join(str(tot[c]) for c in COLS) + " |")
