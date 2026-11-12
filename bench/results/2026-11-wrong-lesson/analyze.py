"""Tables for the wrong-lesson cell, from what the harness wrote. Changes nothing.

Reads `cold/rows.jsonl` and `wrong/rows.jsonl` beside this file, and each session's kept
directory under RUNS (`<RUNS>/cold/...`, `<RUNS>/wrong/...`: `events.jsonl`, `pre-read.out`,
`outcome.json`, and for `notes` the kept copy `ws/`). Written before the first session of the
cell ran (see PREREG.md, "Metrics"); the smoke's three sessions were used to learn the layout.

Usage: python3 bench/results/2026-11-wrong-lesson/analyze.py [RUNS]   (default /root/bench-work/runs)
"""

import json
import math
import subprocess
import sys
import tomllib
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
RUNS = Path(sys.argv[1] if len(sys.argv) > 1 else "/root/bench-work/runs")
POOL = ROOT / "bench/tasks-relay/pool/notes-wrong.toml"
ARMS = ["cold", "notes", "av"]


def wilson(k: int, n: int):
    """95 % Wilson score interval, as `scholion_bench::stats::wilson_interval`."""
    if n == 0:
        return None
    z = 1.959963984540054
    p = k / n
    denom = 1 + z * z / n
    center = (p + z * z / (2 * n)) / denom
    margin = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / denom
    lo = 0.0 if k == 0 else center - margin
    hi = 1.0 if k == n else center + margin
    return lo, hi


def strings(x):
    """Every string anywhere in a JSON value."""
    if isinstance(x, str):
        yield x
    elif isinstance(x, dict):
        for v in x.values():
            yield from strings(v)
    elif isinstance(x, list):
        for v in x:
            yield from strings(v)


def events(path: Path):
    if not path.exists():
        return
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        try:
            yield json.loads(line)
        except json.JSONDecodeError:
            continue


def wrong_notes() -> dict:
    pool = tomllib.loads(POOL.read_text(encoding="utf-8"))
    return {f"relay-{n['pair']}": n["summary"] for n in pool["note"] if n.get("no_commit")}


def delivery(run: str, row: dict, note: str) -> dict:
    """Where the task's wrong note was put in front of this session's agent."""
    d = RUNS / run / f"{row['task']}-{row['arm']}-r{row['rep']}"
    out = {"dir": d.exists(), "start_hook": False, "pre_read": False, "tool_result": False,
           "read_notes_md": False, "anywhere": False, "in_git_log": None}
    for e in events(d / "events.jsonl"):
        has = any(note in s for s in strings(e))
        out["anywhere"] |= has
        if e.get("type") == "system" and e.get("subtype") == "hook_response":
            if e.get("hook_event") == "SessionStart" and has:
                out["start_hook"] = True
        if e.get("type") == "user" and has:
            out["tool_result"] = True
        if e.get("type") == "assistant":
            for c in (e.get("message") or {}).get("content") or []:
                if isinstance(c, dict) and c.get("type") == "tool_use" and c.get("name") == "Read":
                    if str((c.get("input") or {}).get("file_path", "")).endswith("NOTES.md"):
                        out["read_notes_md"] = True
    pre = d / "pre-read.out"
    if pre.exists():
        for line in pre.read_text(encoding="utf-8", errors="replace").splitlines():
            try:
                if any(note in s for s in strings(json.loads(line))):
                    out["pre_read"] = True
            except json.JSONDecodeError:
                out["pre_read"] |= note in line
    ws = d / "ws"
    if (ws / ".git").exists():
        log = subprocess.run(["git", "-C", str(ws), "log", "--format=%s%n%b"],
                             capture_output=True, text=True).stdout
        out["in_git_log"] = note in log
    if row["arm"] == "notes":
        out["delivered"] = out["tool_result"]
    elif row["arm"] == "av":
        out["delivered"] = out["start_hook"] or out["pre_read"]
    else:
        out["delivered"] = out["anywhere"]
    return out


def main() -> None:
    notes = wrong_notes()
    rows = []
    for run in ("cold", "wrong"):
        p = HERE / run / "rows.jsonl"
        if p.exists():
            for line in p.read_text(encoding="utf-8").splitlines():
                r = json.loads(line)
                r["run"] = run
                rows.append(r)
    if not rows:
        sys.exit("no rows yet")
    for r in rows:
        r["d"] = delivery(r["run"], r, notes[r["task"]])
        r["valid"] = r["status"] == "ok"
        r["metrics"] = r.get("metrics") or {"repeated_dead_ends": 0, "turns": 0}
        r["wrote_wrong"] = r["metrics"]["repeated_dead_ends"] >= 1

    print("## Accepted\n")
    print("| arm | sessions | valid | accepted | rate | 95 % Wilson | cost | cost per accepted | median turns |")
    print("|---|---|---|---|---|---|---|---|---|")
    for arm in ARMS:
        rs = [r for r in rows if r["arm"] == arm and r["status"] != "skipped_budget"]
        v = [r for r in rs if r["valid"]]
        k = sum(1 for r in v if r["success"])
        w = wilson(k, len(v))
        cost = sum(r["cost_usd"] for r in rs)
        turns = sorted(r["metrics"]["turns"] for r in v)
        med = turns[len(turns) // 2] if len(turns) % 2 else (
            (turns[len(turns) // 2 - 1] + turns[len(turns) // 2]) / 2 if turns else 0)
        iv = f"{w[0]:.3f} to {w[1]:.3f}" if w else "n/a"
        rate = f"{k / len(v):.3f}" if v else "n/a"
        per = f"${cost / k:.4f}" if k else "n/a"
        print(f"| {arm} | {len(rs)} | {len(v)} | {k} | {rate} | {iv} | ${cost:.4f} | {per} | {med} |")
    skipped = sum(1 for r in rows if r["status"] == "skipped_budget")
    print(f"\nSessions skipped for budget: {skipped}.\n")

    print("## Accepted per task (of the valid sessions)\n")
    tasks = sorted({r["task"] for r in rows})
    print("| task | " + " | ".join(ARMS) + " |")
    print("|---|" + "---|" * len(ARMS))
    for t in tasks:
        cells = []
        for arm in ARMS:
            v = [r for r in rows if r["task"] == t and r["arm"] == arm and r["valid"]]
            cells.append(f"{sum(1 for r in v if r['success'])}/{len(v)}")
        print(f"| {t} | " + " | ".join(cells) + " |")

    print("\n## Sessions that wrote the wrong edit (`repeated_dead_ends` >= 1)\n")
    print("| arm | valid | wrote it | wrote it, accepted | wrote it, not accepted | never wrote it, not accepted |")
    print("|---|---|---|---|---|---|")
    for arm in ARMS:
        v = [r for r in rows if r["arm"] == arm and r["valid"]]
        wr = [r for r in v if r["wrote_wrong"]]
        print(f"| {arm} | {len(v)} | {len(wr)} | {sum(1 for r in wr if r['success'])} | "
              f"{sum(1 for r in wr if not r['success'])} | "
              f"{sum(1 for r in v if not r['wrote_wrong'] and not r['success'])} |")

    print("\n## Delivery of the task's wrong note, per session\n")
    print("| arm | valid | session dir found | delivered | session-start hook | pre-read hook | "
          "in a tool result | read NOTES.md | in the kept copy's git log |")
    print("|---|---|---|---|---|---|---|---|---|")
    for arm in ARMS:
        v = [r for r in rows if r["arm"] == arm and r["valid"]]
        c = lambda key: sum(1 for r in v if r["d"][key])  # noqa: E731
        logs = [r["d"]["in_git_log"] for r in v if r["d"]["in_git_log"] is not None]
        print(f"| {arm} | {len(v)} | {c('dir')} | {c('delivered')} | {c('start_hook')} | "
              f"{c('pre_read')} | {c('tool_result')} | {c('read_notes_md')} | "
              f"{sum(logs)} of {len(logs)} checked |")
    print("\n`delivered`: for `notes`, the note's whole summary is in a tool result (the read of "
          "`NOTES.md`); for `av`, it is in the session-start hook's output or a pre-read answer; "
          "for `cold`, it is anywhere in the event stream (expected 0).")

    print("\n## Accepted, by delivery (valid sessions)\n")
    print("| arm | delivered: accepted | not delivered: accepted |")
    print("|---|---|---|")
    for arm in ARMS:
        v = [r for r in rows if r["arm"] == arm and r["valid"]]
        yes = [r for r in v if r["d"]["delivered"]]
        no = [r for r in v if not r["d"]["delivered"]]
        print(f"| {arm} | {sum(1 for r in yes if r['success'])}/{len(yes)} | "
              f"{sum(1 for r in no if r['success'])}/{len(no)} |")

    print("\n## Per session\n")
    print("| run | task | arm | rep | status | accepted | wrote wrong edit | delivered | turns | cost |")
    print("|---|---|---|---|---|---|---|---|---|---|")
    for r in rows:
        print(f"| {r['run']} | {r['task']} | {r['arm']} | {r['rep']} | {r['status']} | "
              f"{'yes' if r['success'] else 'no'} | {'yes' if r['wrote_wrong'] else 'no'} | "
              f"{'yes' if r['d']['delivered'] else 'no'} | {r['metrics']['turns']} | ${r['cost_usd']:.4f} |")


if __name__ == "__main__":
    main()
