"""Tables for the re-run of the wrong-lesson cell, from what the harness wrote. Changes nothing.

Reads `qs-cold/rows.jsonl` and `qs-wrong/rows.jsonl` (the primary cell) and, when they exist,
`silent-cold/rows.jsonl` and `silent-wrong/rows.jsonl` (the secondary cell) beside this file, and
each session's directory under RUNS (`<RUNS>/<run>/<task>-<arm>-r<rep>`: `events.jsonl`,
`pre-read.out`). The Wilson interval and the per-session delivery check are the first run's
(`../2026-11-wrong-lesson/analyze.py`), imported, not copied. Written before the first session of
the primary cell ran; the decision rule is PREREG.md's, "Metrics and decision rule".

Changed twice after the primary cell ran and before any of its rows was read: a session's
directory is under the run id, which is the `--out` name without hyphens (`qs-cold` is
`<RUNS>/qscold`), and the accepted table is printed a second time without the sessions `leak.py`
lists in `exposed.json`, as PREREG.md's leak check says.

Usage: python3 bench/results/2026-11-wrong-lesson-2/analyze.py [RUNS]   (default /bench/bench-work/runs)
"""

import importlib.util
import json
import sys
import tomllib
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
RUNS = Path(sys.argv[1] if len(sys.argv) > 1 else "/bench/bench-work/runs")
ARMS = ["cold", "notes", "av"]
PRIMARY = "relay-quote-split"
CELLS = {
    "primary": (["qs-cold", "qs-wrong"], "bench/tasks-relay/pool/notes-wrong.toml"),
    "secondary": (["silent-cold", "silent-wrong"], "bench/tasks-relay/pool/notes-silent.toml"),
}

_spec = importlib.util.spec_from_file_location(
    "first_run", HERE.parent / "2026-11-wrong-lesson" / "analyze.py")
first = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(first)
first.RUNS = RUNS
wilson = first.wilson


def wrong_notes(pool: str) -> dict:
    p = tomllib.loads((ROOT / pool).read_text(encoding="utf-8"))
    return {f"relay-{n['pair']}": n["summary"] for n in p["note"] if n.get("no_commit")}


def load(cell: str) -> list:
    runs, pool = CELLS[cell]
    notes = wrong_notes(pool)
    rows = []
    for run in runs:
        p = HERE / run / "rows.jsonl"
        if not p.exists():
            continue
        for line in p.read_text(encoding="utf-8").splitlines():
            r = json.loads(line)
            r["run"] = run
            r["valid"] = r["status"] == "ok"
            r["metrics"] = r.get("metrics") or {"repeated_dead_ends": 0, "turns": 0}
            r["wrote_wrong"] = r["metrics"]["repeated_dead_ends"] >= 1
            r["d"] = first.delivery(run.replace("-", ""), r, notes[r["task"]])
            rows.append(r)
    return rows


def arm_line(rows: list, arm: str):
    rs = [r for r in rows if r["arm"] == arm and r["status"] != "skipped_budget"]
    v = [r for r in rs if r["valid"]]
    k = sum(1 for r in v if r["success"])
    return rs, v, k, wilson(k, len(v))


def accepted_table(rows: list) -> dict:
    out = {}
    print("| arm | sessions | valid | accepted | rate | 95 % Wilson | cost | cost per accepted | median turns |")
    print("|---|---|---|---|---|---|---|---|---|")
    for arm in ARMS:
        rs, v, k, w = arm_line(rows, arm)
        out[arm] = (len(v), k, w)
        cost = sum(r["cost_usd"] for r in rs)
        turns = sorted(r["metrics"]["turns"] for r in v)
        n = len(turns)
        med = 0 if not n else turns[n // 2] if n % 2 else (turns[n // 2 - 1] + turns[n // 2]) / 2
        iv = f"{w[0]:.3f} to {w[1]:.3f}" if w else "n/a"
        rate = f"{k / len(v):.3f}" if v else "n/a"
        per = f"${cost / k:.4f}" if k else "n/a"
        print(f"| {arm} | {len(rs)} | {len(v)} | {k} | {rate} | {iv} | ${cost:.4f} | {per} | {med} |")
    skipped = sum(1 for r in rows if r["status"] == "skipped_budget")
    print(f"\nSessions skipped for budget: {skipped}.\n")
    return out


def without_exposed(rows: list) -> None:
    """PREREG.md, leak check: the same table without the sessions `leak.py` found exposed."""
    p = HERE / "exposed.json"
    if not p.exists():
        print("### Without exposed sessions\n\n`exposed.json` is missing: run `leak.py` first.\n")
        return
    exposed = {tuple(x) for x in json.loads(p.read_text(encoding="utf-8"))}
    kept = [r for r in rows if (r["run"], r["task"], r["arm"], r["rep"]) not in exposed]
    print(f"### Without exposed sessions ({len(rows) - len(kept)} left out)\n")
    accepted_table(kept)


def wrote_and_delivery(rows: list) -> None:
    print("### Sessions that wrote the wrong edit (`repeated_dead_ends` >= 1)\n")
    print("| arm | valid | wrote it | wrote it, accepted | wrote it, not accepted | never wrote it, not accepted |")
    print("|---|---|---|---|---|---|")
    for arm in ARMS:
        v = [r for r in rows if r["arm"] == arm and r["valid"]]
        wr = [r for r in v if r["wrote_wrong"]]
        print(f"| {arm} | {len(v)} | {len(wr)} | {sum(1 for r in wr if r['success'])} | "
              f"{sum(1 for r in wr if not r['success'])} | "
              f"{sum(1 for r in v if not r['wrote_wrong'] and not r['success'])} |")
    print("\n### Delivery of the task's wrong note, per session\n")
    print("| arm | valid | session dir found | delivered | session-start hook | pre-read hook | "
          "in a tool result | read NOTES.md |")
    print("|---|---|---|---|---|---|---|---|")
    for arm in ARMS:
        v = [r for r in rows if r["arm"] == arm and r["valid"]]
        c = lambda key: sum(1 for r in v if r["d"][key])  # noqa: E731
        print(f"| {arm} | {len(v)} | {c('dir')} | {c('delivered')} | {c('start_hook')} | "
              f"{c('pre_read')} | {c('tool_result')} | {c('read_notes_md')} |")
    print("\n`delivered`: for `notes`, the note's whole summary is in a tool result (the read of "
          "`NOTES.md`); for `av`, it is in the session-start hook's output or a pre-read answer; "
          "for `cold`, it is anywhere in the event stream (expected 0).\n")


def per_session(rows: list) -> None:
    print("### Per session\n")
    print("| run | task | arm | rep | status | accepted | wrote wrong edit | delivered | turns | cost |")
    print("|---|---|---|---|---|---|---|---|---|---|")
    for r in rows:
        print(f"| {r['run']} | {r['task']} | {r['arm']} | {r['rep']} | {r['status']} | "
              f"{'yes' if r['success'] else 'no'} | {'yes' if r['wrote_wrong'] else 'no'} | "
              f"{'yes' if r['d']['delivered'] else 'no'} | {r['metrics']['turns']} | ${r['cost_usd']:.4f} |")
    print()


def decision(t: dict) -> None:
    """PREREG.md: H1 only if av and notes each have 30 valid sessions and av's interval lies
    below notes's; cold below 10 of 12 means the comparison is confounded."""
    (nn, nk, nw), (an, ak, aw), (cn, ck, _) = t["notes"], t["av"], t["cold"]
    print("### Decision by the pre-registered rule\n")
    if nn < 30 or an < 30:
        print(f"- No claim: fewer than 30 valid sessions in an arm (notes {nn}, av {an}).")
    elif aw[1] < nw[0]:
        print(f"- **H1 is reported**: av {ak}/{an} (upper end {aw[1]:.3f}) is below notes "
              f"{nk}/{nn} (lower end {nw[0]:.3f}).")
    else:
        print(f"- **H0 is reported**: the intervals overlap or av is not below notes "
              f"(av {ak}/{an}, {aw[0]:.3f} to {aw[1]:.3f}; notes {nk}/{nn}, {nw[0]:.3f} to {nw[1]:.3f}).")
    if cn:
        if ck * 12 < 10 * cn:
            print(f"- cold solved {ck} of {cn}, below 10 of 12: the comparison is confounded.")
        else:
            print(f"- cold solved {ck} of {cn}: the task is still solved without any note.")
    print()


def main() -> None:
    rows = load("primary")
    if not rows:
        sys.exit("no rows yet")
    print(f"## Primary: `{PRIMARY}`\n")
    other = sorted({r["task"] for r in rows} - {PRIMARY})
    if other:
        print(f"Unexpected tasks in the primary runs: {other}\n")
    t = accepted_table(rows)
    decision(t)
    without_exposed(rows)
    wrote_and_delivery(rows)
    per_session(rows)

    rows2 = load("secondary")
    if not rows2:
        print("## Secondary: the three silent tasks\n\nNot run yet.")
        return
    print("## Secondary: the three silent tasks (not part of the decision)\n")
    print("### Pooled over the three tasks\n")
    accepted_table(rows2)
    without_exposed(rows2)
    for task in sorted({r["task"] for r in rows2}):
        print(f"### `{task}`\n")
        accepted_table([r for r in rows2 if r["task"] == task])
    wrote_and_delivery(rows2)
    per_session(rows2)


if __name__ == "__main__":
    main()
