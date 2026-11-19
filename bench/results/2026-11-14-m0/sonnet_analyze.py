"""Tables and the decision for M0's second model (amendment 2, Claude Sonnet 5.5). Changes nothing.

The primary pair only: `m0s-av-before/` (build A, `av-before`) and `m0s-phase0/` (build B,
`av-phase0`), as the harness wrote them, with each directory's store export
(`superseded.jsonl`, `records.jsonl`) and each session's directory under RUNS. Everything that
applies is taken from `analyze.py` as committed (the Wilson formula of
`margincode_bench::stats::wilson_interval`, the per-session delivery check, the transcript count
of `av record --supersedes`, the tables); only the directory names and the arms differ, and the
guard and the `cold` and `notes` arms are not part of this run. The decision rule is PREREG.md's,
"Metrics and decision rule".

Usage: python3 sonnet_analyze.py [RUNS] [--dirs OUT=ARM,OUT=ARM]
       (default RUNS /m2b/m0s/bench-work/runs, dirs m0s-av-before=av-before,m0s-phase0=av-phase0)
--dirs exists to test the script on the Haiku rows (m0-av-before=av-before,m0-wrong=av-phase0).
"""

import importlib.util
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
args = sys.argv[1:]
DIRS = {"m0s-av-before": "av-before", "m0s-phase0": "av-phase0"}
if "--dirs" in args:
    i = args.index("--dirs")
    DIRS = dict(x.split("=") for x in args[i + 1].split(","))
    del args[i:i + 2]
RUNS = Path(args[0] if args else "/m2b/m0s/bench-work/runs")
ARMS = ["av-before", "av-phase0"]

_spec = importlib.util.spec_from_file_location("m0_analyze", HERE / "analyze.py")
an = importlib.util.module_from_spec(_spec)
_argv, sys.argv = sys.argv, [sys.argv[0], str(RUNS)]
_spec.loader.exec_module(an)
sys.argv = _argv
an.ARMS = ARMS
WRONG, TASKS = an.WRONG, an.TASKS


def load() -> list:
    rows = []
    for out, label in DIRS.items():
        p = HERE / out / "rows.jsonl"
        if not p.exists():
            continue
        for line in p.read_text(encoding="utf-8").splitlines():
            r = json.loads(line)
            if r["arm"] != "av":
                continue
            r["run"] = out
            r["label"] = label
            r["valid"] = r["status"] == "ok"
            r["metrics"] = r.get("metrics") or {"repeated_dead_ends": 0, "turns": 0}
            r["wrote_wrong"] = r["metrics"]["repeated_dead_ends"] >= 1
            r["d"] = an.first.delivery(an.run_id(out), r, WRONG[r["task"]])
            r["u"] = an.stage2.supersede_use(r, WRONG[r["task"]])
            rows.append(r)
    return rows


def decision(t: dict) -> None:
    (bn, bk, bw), (pn, pk, pw) = t["av-before"], t["av-phase0"]
    print("### Decision by the pre-registered rule (primary: av-phase0 against av-before)\n")
    per_task = all(len(an.arm(rows, a, x)[1]) >= 10 for a in ARMS for x in TASKS)
    if bn < 30 or pn < 30 or not per_task:
        print(f"- **No claim**: an `av` arm has fewer than 30 valid sessions, or fewer than 10 on "
              f"a task (av-before {bn}, av-phase0 {pn}).\n")
    elif pw[0] > bw[1]:
        print(f"- **Phase 0 helps**: av-phase0 {pk}/{pn} (lower end {pw[0]:.3f}) is above "
              f"av-before {bk}/{bn} (upper end {bw[1]:.3f}).\n")
    elif pw[1] < bw[0]:
        print(f"- **Phase 0 hurts**: av-phase0 {pk}/{pn} (upper end {pw[1]:.3f}) is below "
              f"av-before {bk}/{bn} (lower end {bw[0]:.3f}).\n")
    else:
        print(f"- **Not shown at this size**: av-phase0 {pk}/{pn} ({an.fmt_iv(pw)}), av-before "
              f"{bk}/{bn} ({an.fmt_iv(bw)}).\n")


def export(name: str) -> list:
    xs = []
    for out in DIRS:
        p = HERE / out / name
        if p.exists():
            xs += [json.loads(line) for line in p.read_text(encoding="utf-8").splitlines() if line]
    return xs


def superseded() -> None:
    print("### Secondary 3: superseded records, read from the store\n")
    by_repo = {}
    for x in export("superseded.jsonl"):
        by_repo.setdefault(x["repo"], []).append(x)
    print("| arm | valid | superseded its task's wrong note | another task's wrong note | "
          "a true note | transcript: tried `--supersedes` | transcript: superseded the wrong "
          "note (stage 2's count) |")
    print("|---|---|---|---|---|---|---|")
    for label in ARMS:
        v = [r for r in rows if r["label"] == label and r["valid"]]
        own = other = true = 0
        for r in v:
            s = {x["superseded_summary"] for x in by_repo.get(an.session_repo(r), [])}
            r["sup_own"] = WRONG[r["task"]] in s
            own += r["sup_own"]
            other += any(x in an.WRONG_SUMMARIES and x != WRONG[r["task"]] for x in s)
            true += any(x not in an.WRONG_SUMMARIES for x in s)
        tried = sum(1 for r in v if r["u"]["tried"])
        hits = sum(1 for r in v if r["u"]["hit"])
        print(f"| {label} | {len(v)} | {own} | {other} | {true} | {tried} | {hits} |")
    print("\n| av-phase0 | accepted | not accepted |")
    print("|---|---|---|")
    v = [r for r in rows if r["label"] == "av-phase0" and r["valid"]]
    for name, want in (("superseded the task's wrong note", True), ("did not", False)):
        g = [r for r in v if bool(r.get("sup_own")) == want]
        print(f"| {name} | {sum(1 for r in g if r['success'])} | "
              f"{sum(1 for r in g if not r['success'])} |")
    print()


def restatements() -> None:
    print("### Secondary 5: records that restate the wrong edit\n")
    recs = {}
    for x in export("records.jsonl"):
        if x["summary"] not in an.POOL_SUMMARIES:
            recs.setdefault(x["repo"], []).append(x)
    print("| arm | valid | a record matching the pattern that supersedes the wrong note | "
          "one that does not (restatement, upper bound) |")
    print("|---|---|---|---|")
    for label in ARMS:
        v = [r for r in rows if r["label"] == label and r["valid"]]
        corr = rest = 0
        for r in v:
            xs = [x for x in recs.get(an.session_repo(r), [])
                  if an.DEAD[r["task"]].search(x["summary"] + "\n" + (x.get("body") or ""))]
            corr += any(x.get("superseded_summary") == WRONG[r["task"]] for x in xs)
            rest += any(x.get("superseded_summary") != WRONG[r["task"]] for x in xs)
        print(f"| {label} | {len(v)} | {corr} | {rest} |")
    print()


def without_exposed() -> None:
    p = HERE / "checks-sonnet" / "exposed.json"
    if not p.exists():
        print("### Without exposed sessions\n\n`checks-sonnet/exposed.json` is missing: run "
              "`sonnet_leak.py` first.\n")
        return
    exposed = {tuple(x) for x in json.loads(p.read_text(encoding="utf-8"))}
    kept = [r for r in rows if (r["run"], r["task"], r["arm"], r["rep"]) not in exposed]
    print(f"### Without exposed sessions ({len(rows) - len(kept)} left out)\n")
    an.accepted_table(kept, ARMS)


if __name__ == "__main__":
    rows = load()
    if not rows:
        sys.exit("no rows yet")
    print("## M0 on Claude Sonnet 5.5: the primary pair\n")
    print("### Pooled over the three tasks\n")
    t = an.accepted_table(rows, ARMS)
    decision(t)
    without_exposed()
    print("### Secondary 2: per task\n")
    for task in TASKS:
        print(f"#### `{task}`\n")
        an.accepted_table(rows, ARMS, task)
    superseded()
    an.wrong_edit(rows)
    restatements()
    an.delivery(rows)
    an.per_session(rows)
