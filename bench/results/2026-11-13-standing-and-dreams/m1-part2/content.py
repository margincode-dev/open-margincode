"""M0's `content.py` (`../../2026-11-14-m0/content.py`) for M1 Part 2's two arms. Committed with
Build, before the first paid session.

1. **Hidden-test content in a transcript.** Every event of every session, inputs and outputs, is
   searched for the file name of a hidden test (`test_relay_`) and for the name of any test
   function defined in `bench/tasks-relay/files/`, whatever path the session used.
2. **What each session that was not accepted last wrote** to its task's file (the spec's
   `relevant_files`): the text of its last Edit or Write there, and whether that text holds the
   note's call, by the task's own `[[dead_end]]` pattern. This part reads which sessions were
   accepted, so it is run after the result; part 1 (`--hidden-only`) is run and committed before
   any row is read for accepted counts.

Usage: python3 content.py [RUNS] [--hidden-only]   (default /m1/bench-work/runs)
"""

import json
import re
import sys
import tomllib
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
args = [a for a in sys.argv[1:] if not a.startswith("--")]
RUNS = Path(args[0] if args else "/m1/bench-work/runs")
HIDDEN_ONLY = "--hidden-only" in sys.argv
OUTS = ["m1-p0", "m1-p1"]
LABEL = {("m1-p0", "av"): "av-p0", ("m1-p1", "av"): "av-p01"}
FILES, DEAD = {}, {}
for p in (ROOT / "bench/tasks-m0").glob("*.toml"):
    spec = tomllib.loads(p.read_text(encoding="utf-8"))
    FILES[spec["name"]] = spec["relevant_files"][0]
    DEAD[spec["name"]] = re.compile(spec["dead_end"][0]["pattern"])

names = {"test_relay_"}
for p in (ROOT / "bench/tasks-relay/files").glob("test_relay_*.py"):
    names |= set(re.findall(r"def (test_\w+)", p.read_text(encoding="utf-8")))


def cell(text: str) -> str:
    return " ".join(text.split()).replace("|", "\\|")


def session_dir(r: dict) -> Path:
    return RUNS / r["out"].replace("-", "") / f"{r['task']}-{r['arm']}-r{r['rep']}"


# Part 1 reads only which sessions exist, never `success`.
sessions = []
for out in OUTS:
    run = RUNS / out.replace("-", "")
    if run.is_dir():
        for d in sorted(run.iterdir()):
            m = re.match(r"^(relay-[a-z-]+)-(av)-r(\d+)$", d.name)
            if m:
                sessions.append({"out": out, "task": m[1], "arm": m[2], "rep": int(m[3])})

print(f"## Hidden-test content in the transcripts ({len(names)} names searched)\n")
hits = 0
for r in sessions:
    p = session_dir(r) / "events.jsonl"
    text = p.read_text(encoding="utf-8", errors="replace") if p.exists() else ""
    found = sorted(n for n in names if n in text)
    if found:
        hits += 1
        print(f"- {r['out']} {r['task']}-{r['arm']}-r{r['rep']}: {', '.join(found)}")
print(f"\n{len(sessions)} sessions searched, {hits} with a hit.\n")
if HIDDEN_ONLY:
    sys.exit(0)

rows = []
for out in OUTS:
    p = HERE / out / "rows.jsonl"
    if p.exists():
        rows += [json.loads(line) | {"out": out} for line in p.read_text().splitlines()]

print("## Last edit of the task's file by each session that was not accepted\n")
print("| run | arm | task | rep | harness pattern matched | last text holds the note's call | "
      "last text written to the file (300 chars) |")
print("|---|---|---|---|---|---|---|")
for r in rows:
    if r["success"] or r["task"] not in FILES:
        continue
    last = ""
    p = session_dir(r) / "events.jsonl"
    lines = p.read_text(encoding="utf-8", errors="replace").splitlines() if p.exists() else []
    for line in lines:
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
    label = LABEL.get((r["out"], r["arm"]), r["arm"])
    print(f"| {r['out']} | {label} | {r['task']} | {r['rep']} | {yn(matched)} | "
          f"{yn(bool(DEAD[r['task']].search(last)))} | `{cell(last)[:300]}` |")
