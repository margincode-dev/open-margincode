"""Amendment 3's counts, from the results directories as the harness wrote them.

Per arm: sessions, valid (`status` ok), accepted (`success`), the Wilson interval (printed by
`wilson`, built from `margincode_bench::stats` by `wilson.rs`), cost, repeated dead ends, median
wall time. Per task: accepted of valid in each arm. With the session directories (in the
container, `--runs`), also: sessions whose Bash commands contained `--supersedes`, sessions that
ran `av`, the largest single request (prompt tokens, cache included) and the largest session cost.

Usage: python3 analyze.py RESULTS_DIR [--runs RUNS_DIR] [--wilson PATH]
"""

import json
import statistics
import subprocess
import sys
from collections import Counter
from pathlib import Path

args = sys.argv[1:]
res = Path(args[0])
runs = Path(args[args.index("--runs") + 1]) if "--runs" in args else None
wilson_bin = args[args.index("--wilson") + 1] if "--wilson" in args else None
ARMS = [("raw", "m2-raw-ho"), ("dreamed", "m2-dreamed-ho"), ("raw M0 row", "m2-raw-m0c"),
        ("dreamed M0 row", "m2-dreamed-m0c"), ("cold", "m2-cold-ho"), ("smoke", "smoke-r3")]


def wilson(k, n):
    if not wilson_bin:
        return "-"
    out = subprocess.run([wilson_bin, f"{k}/{n}"], capture_output=True, text=True, check=True)
    return out.stdout.split("\t")[2].strip()


def rows_of(out):
    p = res / out / "rows.jsonl"
    return [json.loads(l) for l in p.read_text().splitlines() if l] if p.exists() else None


def session_facts(out, rows):
    run_id = out.replace("-", "")
    sup = ran_av = 0
    biggest = 0
    for r in rows:
        d = runs / run_id / f"{r['task']}-{r['arm']}-r{r['rep']}"
        ev = d / "events.jsonl"
        if not ev.exists():
            continue
        cmds = []
        for line in ev.read_text().splitlines():
            try:
                e = json.loads(line)
            except json.JSONDecodeError:
                continue
            if e.get("type") != "assistant":
                continue
            m = e["message"]
            u = m.get("usage") or {}
            biggest = max(biggest, u.get("input_tokens", 0) + u.get("cache_creation_input_tokens", 0)
                          + u.get("cache_read_input_tokens", 0))
            for c in m.get("content") or []:
                if c.get("type") == "tool_use" and c.get("name") == "Bash":
                    cmds.append(c["input"].get("command", ""))
        sup += any("--supersedes" in c for c in cmds)
        ran_av += any(c.lstrip().startswith("av ") or " av " in c or "&& av " in c for c in cmds)
    return sup, ran_av, biggest


print("| arm | out | sessions | valid | accepted | rate | 95 % Wilson | cost | repeated dead ends | median wall s |")
print("|---|---|---|---|---|---|---|---|---|---|")
per_task = {}
facts = {}
for label, out in ARMS:
    rows = rows_of(out)
    if rows is None:
        continue
    valid = [r for r in rows if r["status"] == "ok"]
    k = sum(bool(r["success"]) for r in valid)
    cost = sum(r.get("cost_usd") or 0.0 for r in rows)
    dead = sum((r.get("metrics") or {}).get("repeated_dead_ends") or 0 for r in valid)
    wall = statistics.median(r["wall_secs"] for r in valid) if valid else 0
    rate = f"{k / len(valid):.3f}" if valid else "-"
    print(f"| {label} | {out} | {len(rows)} | {len(valid)} | {k} | {rate} | {wilson(k, len(valid)) if valid else '-'} "
          f"| ${cost:.4f} | {dead} | {wall:.1f} |")
    t_n, t_k = Counter(), Counter()
    for r in valid:
        t_n[r["task"]] += 1
        t_k[r["task"]] += bool(r["success"])
    per_task[label] = (t_k, t_n)
    if runs:
        facts[label] = session_facts(out, rows) + (max((r.get("cost_usd") or 0.0) for r in rows),)

print()
print("| task | " + " | ".join(per_task) + " |")
print("|---|" + "---|" * len(per_task))
tasks = sorted({t for k, n in per_task.values() for t in n})
for t in tasks:
    cells = [f"{k[t]}/{n[t]}" if n[t] else "-" for k, n in per_task.values()]
    print(f"| {t} | " + " | ".join(cells) + " |")
if facts:
    print()
    print("| arm | tried --supersedes | ran av | largest request (prompt tokens) | largest session cost |")
    print("|---|---|---|---|---|")
    for label, (sup, ran_av, big, top) in facts.items():
        print(f"| {label} | {sup} | {ran_av} | {big} | ${top:.4f} |")
