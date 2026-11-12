"""Post hoc (written after the result was known; not part of the pre-registration).

The registered counts say how many `av` sessions superseded the task's wrong note. They do not
say what else was superseded, or when. The pool holds five wrong notes (one per task of
`bench/tasks-relay-wrong`) and five true ones, so a session told to check earlier agents' claims
can also correct another task's wrong note, or "correct" a true one. This script reads every
`av` session's transcript and prints:

1. which record each accepted correction replaced: this task's wrong note, another task's wrong
   note, a true note, or a record the session wrote itself. A handle is matched to a pool note by
   the text the pack or a hook printed beside it in that session;
2. for sessions that superseded this task's wrong note, whether the correction came before or
   after the first edit of the task's file;
3. `av record` calls refused for the 200-character summary limit.

Usage: python3 bench/results/2026-11-wrong-lesson-3/supersedes.py [RUNS]   (default /b2/bench-work/runs)
"""

import collections
import json
import re
import sys
import tomllib
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
RUNS = Path(sys.argv[1] if len(sys.argv) > 1 else "/b2/bench-work/runs")
OUT, TASK, PAIR, FILE = "s2-wrong", "relay-quote-split", "quote-split", "tallyho/pricing/quote.py"

pool = tomllib.loads((ROOT / "bench/tasks-relay/pool/notes-wrong.toml").read_text(encoding="utf-8"))
NOTES = [(n["summary"], n["pair"], bool(n.get("no_commit"))) for n in pool["note"]]
SHOWN = re.compile(r"(rec:[0-9a-f]{8}) \w+: ([^\n\\]{20,})")
DONE = re.compile(r"recorded (rec:[0-9a-f]{8}) \(\w+\)(?:, supersedes (rec:[0-9a-f]{8}))?")


def strings(x):
    if isinstance(x, str):
        yield x
    elif isinstance(x, dict):
        for v in x.values():
            yield from strings(v)
    elif isinstance(x, list):
        for v in x:
            yield from strings(v)


def which(text: str) -> str:
    head = text.rstrip("….")[:40]
    for summary, pair, wrong in NOTES:
        if summary.startswith(head):
            if pair == PAIR:
                return "this task's wrong note"
            return f"another task's wrong note ({pair})" if wrong else f"a true note ({pair})"
    return "a record not in the pool"


rows = [json.loads(line) for line in (HERE / OUT / "rows.jsonl").read_text().splitlines()]
kinds = collections.Counter()
sessions = collections.defaultdict(set)
timing = []
limit_calls, limit_sessions = 0, 0
per = []
for r in rows:
    if r["arm"] != "av" or r["status"] != "ok":
        continue
    d = RUNS / OUT.replace("-", "") / f"{r['task']}-{r['arm']}-r{r['rep']}"
    shown, own, records = {}, set(), {}
    n = first_edit = first_fix = 0
    refused = 0
    replaced = []
    for line in (d / "events.jsonl").read_text(encoding="utf-8", errors="replace").splitlines():
        try:
            e = json.loads(line)
        except json.JSONDecodeError:
            continue
        if e.get("type") in ("system", "user"):
            for s in strings(e):
                for m in SHOWN.finditer(s):
                    shown.setdefault(m.group(1), m.group(2))
        content = (e.get("message") or {}).get("content")
        if not isinstance(content, list):
            continue
        for c in content:
            if not isinstance(c, dict):
                continue
            if e.get("type") == "assistant" and c.get("type") == "tool_use":
                n += 1
                i = c.get("input") or {}
                if c.get("name") in ("Edit", "Write", "MultiEdit") and \
                        str(i.get("file_path", "")).endswith(FILE) and not first_edit:
                    first_edit = n
                if c.get("name") == "Bash" and re.search(r"\b(av|scholion) record\b",
                                                         str(i.get("command", ""))):
                    records[c.get("id")] = n
            if e.get("type") == "user" and c.get("type") == "tool_result" \
                    and c.get("tool_use_id") in records:
                answer = " ".join(strings(c.get("content")))
                if re.search(r"summary is \d+ chars", answer):
                    refused += 1
                for m in DONE.finditer(answer):
                    if m.group(2):
                        old = m.group(2)
                        kind = "a record the session wrote itself" if old in own \
                            else which(shown.get(old, ""))
                        replaced.append(kind)
                        if kind == "this task's wrong note" and not first_fix:
                            first_fix = records[c["tool_use_id"]]
                    own.add(m.group(1))
    for k in replaced:
        kinds[k] += 1
        sessions[k].add(r["rep"])
    if first_fix:
        timing.append((r["rep"], first_edit, first_fix))
    limit_calls += refused
    limit_sessions += refused > 0
    per.append((r["rep"], r["success"], replaced, refused))

print("## What the accepted corrections replaced (`av` arm, post hoc)\n")
print("| record replaced | corrections | sessions | of them accepted |")
print("|---|---|---|---|")
ok = {r["rep"] for r in rows if r["arm"] == "av" and r["success"]}
for k, v in sorted(kinds.items(), key=lambda kv: -kv[1]):
    print(f"| {k} | {v} | {len(sessions[k])} | {len(sessions[k] & ok)} |")
print()
print("## When this task's wrong note was superseded\n")
after = sum(1 for _, edit, fix in timing if edit and fix > edit)
print(f"{len(timing)} sessions superseded it; in {after} the correction came after the first edit "
      f"of `{FILE}`, in {len(timing) - after} before it.\n")
print("| rep | first edit of the file (tool call no.) | correction (tool call no.) |")
print("|---|---|---|")
for rep, edit, fix in timing:
    print(f"| {rep} | {edit or '-'} | {fix} |")
print()
print("## `av record` calls refused for the summary limit\n")
print(f"{limit_calls} calls in {limit_sessions} of {len(per)} sessions.\n")
print("| rep | accepted | replaced | refused for length |")
print("|---|---|---|---|")
for rep, success, replaced, refused in per:
    print(f"| {rep} | {'yes' if success else 'no'} | {'; '.join(replaced) or '-'} | {refused} |")
