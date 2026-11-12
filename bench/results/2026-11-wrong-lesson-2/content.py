"""Post hoc (written after the result was known, asked for by the review of the write-up).

Two things `leak.py` and the rows cannot show:

1. **Hidden-test content in a transcript.** `leak.py` classifies a call by the paths it names, so
   it misses a read through `~`, a variable, `..` or a path put together in the shell. This scan
   does not look at paths: it searches every event of every session, inputs and outputs, for the
   file name of a hidden test (`test_relay_`) and for the name of any test function defined in
   `bench/tasks-relay/files/`. A session that saw a hidden test by any file route would show one.
   It still cannot see what an agent learned from the daemon or the network.
2. **What each session that was not accepted last wrote** to the task's file: the text of its last
   Edit or Write there, and whether that text calls `.times(` (the note's approach) or
   `.allocate(` (the one that passes), so "the failure is the note's approach" can be read and
   not only counted by the harness's pattern.

Usage: python3 bench/results/2026-11-wrong-lesson-2/content.py [RUNS]   (default /bench/bench-work/runs)
"""

import json
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
RUNS = Path(sys.argv[1] if len(sys.argv) > 1 else "/bench/bench-work/runs")
OUTS = ["qs-cold", "qs-wrong", "silent-cold", "silent-wrong"]
FILES = {"relay-quote-split": "tallyho/pricing/quote.py"}

names = {"test_relay_"}
for p in (ROOT / "bench/tasks-relay/files").glob("test_relay_*.py"):
    names |= set(re.findall(r"def (test_\w+)", p.read_text(encoding="utf-8")))


def cell(text: str) -> str:
    return " ".join(text.split()).replace("|", "\\|")


rows = []
for out in OUTS:
    p = HERE / out / "rows.jsonl"
    if p.exists():
        rows += [json.loads(line) | {"out": out} for line in p.read_text().splitlines()]

print(f"## Hidden-test content in the transcripts ({len(names)} names searched)\n")
hits = 0
for r in rows:
    d = RUNS / r["out"].replace("-", "") / f"{r['task']}-{r['arm']}-r{r['rep']}"
    text = (d / "events.jsonl").read_text(encoding="utf-8", errors="replace")
    found = sorted(n for n in names if n in text)
    if found:
        hits += 1
        print(f"- {r['out']} {r['task']}-{r['arm']}-r{r['rep']}: {', '.join(found)}")
print(f"\n{len(rows)} sessions searched, {hits} with a hit.\n")

print("## Last edit of the task's file by each session that was not accepted\n")
print("| run | arm | rep | harness pattern matched | calls `.times(` | calls `.allocate(` | "
      "last text written to the file (300 chars) |")
print("|---|---|---|---|---|---|---|")
for r in rows:
    if r["success"] or r["task"] not in FILES:
        continue
    d = RUNS / r["out"].replace("-", "") / f"{r['task']}-{r['arm']}-r{r['rep']}"
    last = ""
    for line in (d / "events.jsonl").read_text(encoding="utf-8", errors="replace").splitlines():
        try:
            e = json.loads(line)
        except json.JSONDecodeError:
            continue
        if e.get("type") != "assistant":
            continue
        for c in (e.get("message") or {}).get("content") or []:
            if not (isinstance(c, dict) and c.get("type") == "tool_use"):
                continue
            i = c.get("input") or {}
            if c.get("name") in ("Edit", "Write", "MultiEdit") and str(
                    i.get("file_path", "")).endswith(FILES[r["task"]]):
                last = str(i.get("new_string") or i.get("content") or json.dumps(i.get("edits")))
    matched = (r.get("metrics") or {}).get("repeated_dead_ends", 0) >= 1
    yn = lambda b: "yes" if b else "no"  # noqa: E731
    print(f"| {r['out']} | {r['arm']} | {r['rep']} | {yn(matched)} | {yn('.times(' in last)} | "
          f"{yn('.allocate(' in last)} | `{cell(last)[:300]}` |")
