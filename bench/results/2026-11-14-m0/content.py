"""Stage 2's `content.py` (`../2026-11-wrong-lesson-3/content.py`) for M0's three tasks and four
arms. Committed with "Build" of PREREG.md, before the first paid session.

1. **Hidden-test content in a transcript.** Every event of every session, inputs and outputs, is
   searched for the file name of a hidden test (`test_relay_`) and for the name of any test
   function defined in `bench/tasks-relay/files/`, whatever path the session used.
2. **What each session that was not accepted last wrote** to its task's file (the spec's
   `relevant_files`): the text of its last Edit or Write there, and whether that text holds the
   note's call, by the task's own `[[dead_end]]` pattern (the one the harness counts as
   `repeated_dead_ends`). This part reads which sessions were accepted, so it is run after the
   result; part 1 is printed first and kept before any row is read for accepted counts.

Usage: python3 bench/results/2026-11-14-m0/content.py [RUNS] [--hidden-only]
       (default /m0/bench-work/runs)
"""

import json
import re
import sys
import tomllib
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
args = [a for a in sys.argv[1:] if not a.startswith("--")]
RUNS = Path(args[0] if args else "/m0/bench-work/runs")
HIDDEN_ONLY = "--hidden-only" in sys.argv
OUTS = ["m0-cold", "m0-av-before", "m0-wrong", "m0-guard"]
LABEL = {("m0-cold", "cold"): "cold", ("m0-av-before", "av"): "av-before",
         ("m0-wrong", "notes"): "notes", ("m0-wrong", "av"): "av-phase0",
         ("m0-guard", "av"): "guard"}
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


rows = []
for out in OUTS:
    p = HERE / out / "rows.jsonl"
    if p.exists():
        rows += [json.loads(line) | {"out": out} for line in p.read_text().splitlines()]

print(f"## Hidden-test content in the transcripts ({len(names)} names searched)\n")
hits = 0
for r in rows:
    p = session_dir(r) / "events.jsonl"
    text = p.read_text(encoding="utf-8", errors="replace") if p.exists() else ""
    found = sorted(n for n in names if n in text)
    if found:
        hits += 1
        print(f"- {r['out']} {r['task']}-{r['arm']}-r{r['rep']}: {', '.join(found)}")
print(f"\n{len(rows)} sessions searched, {hits} with a hit.\n")
if HIDDEN_ONLY:
    sys.exit(0)

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
