"""Tables and the decision for M1 Part 2, from what the harness wrote. Changes nothing.

Reads, beside this file, `m1-p0/` and `m1-p1/` (`rows.jsonl` as the harness wrote it, and the
store exports `superseded.jsonl` and `records.jsonl` of PREREG.md's Build, "M1 Part 2"), and each
session's directory under RUNS (`<RUNS>/<run id>/<task>-av-r<rep>`; the run id is the `--out`
name without hyphens).

The harness calls both arms `av`. They are told apart by their results directory: `m1-p0` is
`av-p0` (build P0, `av` with Phase 0), `m1-p1` is `av-p01` (build P1, `av` with Phases 0 and 1).
Every row of both arms goes into every table and into the decision: nothing is filtered out
between the tables and the rule (M0's `analyze.py` passed its decision a filtered list, and its
guard line read no rows).

Reused, not copied: the Wilson interval and the event reader of the first wrong-lesson run
(`../../2026-11-wrong-lesson/analyze.py`). The decision rule is the plan's ("a difference is
claimed only if the Wilson intervals are disjoint"), as M0's PREREG wrote it out, with PREREG.md's
"No claim below n = 30 per arm". Committed with Build, before the first paid session.

Usage: python3 analyze.py [RUNS]   (default /m1/bench-work/runs)
"""

import importlib.util
import json
import re
import sys
import tomllib
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
RUNS = Path(sys.argv[1] if len(sys.argv) > 1 else "/m1/bench-work/runs")
TASKS = ["relay-quote-split", "relay-bus-resubscribe", "relay-tax-with-rate"]
ARMS = ["av-p0", "av-p01"]
OUTS = ["m1-p0", "m1-p1"]
POOL = ROOT / "bench/tasks-relay/pool/notes-silent.toml"
LABEL = {("m1-p0", "av"): "av-p0", ("m1-p1", "av"): "av-p01"}
HINT = "Claims by earlier agents"
# the hook pack cuts a record at the `handle` tier to about 70 characters; 40 always survive
PREFIX = 40


def _load(name: str, path: Path):
    spec = importlib.util.spec_from_file_location(name, path)
    mod = importlib.util.module_from_spec(spec)
    argv, sys.argv = sys.argv, sys.argv[:1]
    spec.loader.exec_module(mod)
    sys.argv = argv
    return mod


first = _load("first_run", HERE.parents[1] / "2026-11-wrong-lesson" / "analyze.py")
wilson, events, strings = first.wilson, first.events, first.strings

pool = tomllib.loads(POOL.read_text(encoding="utf-8"))["note"]
WRONG = {f"relay-{n['pair']}": n["summary"] for n in pool if n.get("no_commit")}
WRONG_SUMMARIES = set(WRONG.values())
DEAD = {}
for t in TASKS:
    spec = tomllib.loads((ROOT / "bench/tasks-m0" / f"{t.removeprefix('relay-')}.toml")
                        .read_text(encoding="utf-8"))
    DEAD[t] = re.compile(spec["dead_end"][0]["pattern"])


def run_id(out: str) -> str:
    return out.replace("-", "")


def session_dir(r: dict) -> Path:
    return RUNS / run_id(r["run"]) / f"{r['task']}-{r['arm']}-r{r['rep']}"


def delivery(r: dict) -> dict:
    """Where the task's wrong note reached this session, read from its transcript.

    - `start_handle`: the session-start hook's output has a line with the note's first
      PREFIX characters (any tier: `summary`, or `handle`, cut).
    - `start_whole`: that output has the whole summary (the `summary` tier).
    - `start_reverted`: the note's line there is labelled `reverted` (Phase 1's origin).
    - `pre_read_whole`: a pre-read answer (`pre-read.out`) has the whole summary.
    - `hint`: Phase 0's line is in the session-start hook's output.
    """
    note = WRONG[r["task"]]
    head = note[:PREFIX]
    out = {"start_handle": False, "start_whole": False, "start_reverted": False,
           "pre_read_whole": False, "hint": False}
    for e in events(session_dir(r) / "events.jsonl"):
        if not (e.get("type") == "system" and e.get("subtype") == "hook_response"
                and e.get("hook_event") == "SessionStart"):
            continue
        for s in strings(e):
            out["hint"] |= HINT in s
            out["start_whole"] |= note in s
            for line in s.splitlines():
                if head in line:
                    out["start_handle"] = True
                    out["start_reverted"] |= "reverted:" in line
    pre = session_dir(r) / "pre-read.out"
    if pre.exists():
        out["pre_read_whole"] = note in pre.read_text(encoding="utf-8", errors="replace")
    return out


def load() -> list:
    rows = []
    for out in OUTS:
        p = HERE / out / "rows.jsonl"
        if not p.exists():
            continue
        for line in p.read_text(encoding="utf-8").splitlines():
            r = json.loads(line)
            r["run"] = out
            r["label"] = LABEL.get((out, r["arm"]), f"?{r['arm']}")
            r["valid"] = r["status"] == "ok"
            r["metrics"] = r.get("metrics") or {"repeated_dead_ends": 0, "turns": 0}
            r["wrote_wrong"] = r["metrics"]["repeated_dead_ends"] >= 1
            r["d"] = delivery(r) if r["task"] in WRONG else {}
            rows.append(r)
    return rows


def arm(rows: list, label: str, task: str | None = None):
    rs = [r for r in rows if r["label"] == label and r["status"] != "skipped_budget"
          and (task is None or r["task"] == task)]
    v = [r for r in rs if r["valid"]]
    k = sum(1 for r in v if r["success"])
    return rs, v, k, wilson(k, len(v))


def fmt_iv(w) -> str:
    return f"{w[0]:.3f} to {w[1]:.3f}" if w else "n/a"


def accepted_table(rows: list, task: str | None = None) -> dict:
    out = {}
    print("| arm | sessions | valid | accepted | rate | 95 % Wilson | cost | cost per accepted | "
          "median session cost |")
    print("|---|---|---|---|---|---|---|---|---|")
    for label in ARMS:
        rs, v, k, w = arm(rows, label, task)
        out[label] = (len(v), k, w)
        cost = sum(r["cost_usd"] for r in rs)
        costs = sorted(r["cost_usd"] for r in rs)
        n = len(costs)
        med = 0 if not n else costs[n // 2] if n % 2 else (costs[n // 2 - 1] + costs[n // 2]) / 2
        rate = f"{k / len(v):.3f}" if v else "n/a"
        per = f"${cost / k:.4f}" if k else "n/a"
        print(f"| {label} | {len(rs)} | {len(v)} | {k} | {rate} | {fmt_iv(w)} | ${cost:.4f} | "
              f"{per} | ${med:.4f} |")
    skipped = sum(1 for r in rows if r["status"] == "skipped_budget"
                  and (task is None or r["task"] == task))
    print(f"\nSessions skipped for budget: {skipped}.\n")
    return out


def decision(rows: list) -> str:
    """The registered rule, on `rows` (every row of both arms on the three tasks)."""
    _, v0, k0, w0 = arm(rows, "av-p0")
    _, v1, k1, w1 = arm(rows, "av-p01")
    n0, n1 = len(v0), len(v1)
    print("### Decision by the pre-registered rule (primary: av-p01 against av-p0)\n")
    per_task = all(len(arm(rows, a, x)[1]) >= 10 for a in ARMS for x in TASKS)
    if n0 < 30 or n1 < 30 or not per_task:
        verdict = "no claim"
        print(f"- **No claim**: an arm has fewer than 30 valid sessions, or fewer than 10 on a "
              f"task (av-p0 {n0}, av-p01 {n1}).\n")
    elif w1[0] > w0[1]:
        verdict = "helps"
        print(f"- **Phase 1 helps**: av-p01 {k1}/{n1} (lower end {w1[0]:.3f}) is above "
              f"av-p0 {k0}/{n0} (upper end {w0[1]:.3f}).\n")
    elif w1[1] < w0[0]:
        verdict = "hurts"
        print(f"- **Phase 1 hurts**: av-p01 {k1}/{n1} (upper end {w1[1]:.3f}) is below "
              f"av-p0 {k0}/{n0} (lower end {w0[0]:.3f}).\n")
    else:
        verdict = "not shown"
        print(f"- **Not shown at this size**: av-p01 {k1}/{n1} ({fmt_iv(w1)}), av-p0 "
              f"{k0}/{n0} ({fmt_iv(w0)}).\n")
    return verdict


def without_exposed(rows: list) -> None:
    p = HERE / "exposed.json"
    if not p.exists():
        print("### Without exposed sessions\n\n`exposed.json` is missing: run `leak.py` first.\n")
        return
    exposed = {tuple(x) for x in json.loads(p.read_text(encoding="utf-8"))}
    kept = [r for r in rows if (r["run"], r["task"], r["arm"], r["rep"]) not in exposed]
    print(f"### Without exposed sessions ({len(rows) - len(kept)} left out)\n")
    accepted_table(kept)


def per_task(rows: list) -> None:
    print("### Per task (described, not used for the decision)\n")
    for t in TASKS:
        print(f"#### `{t}`\n")
        accepted_table(rows, t)


def session_repo(r: dict) -> str:
    """`b<run id>-<task>-<arm short>-<rep>`, `runner.rs` of the harness."""
    return f"b{run_id(r['run'])}-{r['task']}-{r['arm']}-{r['rep']}"


def export(out: str, name: str) -> list:
    p = HERE / out / name
    if not p.exists():
        return []
    return [json.loads(line) for line in p.read_text(encoding="utf-8").splitlines() if line]


def superseded(rows: list) -> None:
    print("### Superseded records, read from the store (described)\n")
    by_repo = {}
    for out in OUTS:
        for x in export(out, "superseded.jsonl"):
            by_repo.setdefault(x["repo"], []).append(x)
    print("| arm | valid | superseded its task's wrong note | another task's wrong note | "
          "a true note |")
    print("|---|---|---|---|---|")
    for label in ARMS:
        v = [r for r in rows if r["label"] == label and r["valid"]]
        own = other = true = 0
        for r in v:
            s = {x["superseded_summary"] for x in by_repo.get(session_repo(r), [])}
            r["sup_own"] = WRONG[r["task"]] in s
            own += r["sup_own"]
            other += any(x in WRONG_SUMMARIES and x != WRONG[r["task"]] for x in s)
            true += any(x not in WRONG_SUMMARIES for x in s)
        print(f"| {label} | {len(v)} | {own} | {other} | {true} |")
    print("\n| arm | superseded its task's wrong note: accepted / not | did not: accepted / not |")
    print("|---|---|---|")
    for label in ARMS:
        v = [r for r in rows if r["label"] == label and r["valid"]]
        cells = []
        for want in (True, False):
            g = [r for r in v if bool(r.get("sup_own")) == want]
            cells.append(f"{sum(1 for r in g if r['success'])} / "
                         f"{sum(1 for r in g if not r['success'])}")
        print(f"| {label} | {cells[0]} | {cells[1]} |")
    print()


def wrong_edit(rows: list) -> None:
    print("### The wrong edit (`repeated_dead_ends` >= 1, described)\n")
    print("| arm | task | valid | wrote it | wrote it, accepted | wrote it, not accepted |")
    print("|---|---|---|---|---|---|")
    for label in ARMS:
        for t in TASKS:
            v = [r for r in rows if r["label"] == label and r["task"] == t and r["valid"]]
            w = [r for r in v if r["wrote_wrong"]]
            print(f"| {label} | {t} | {len(v)} | {len(w)} | {sum(1 for r in w if r['success'])} | "
                  f"{sum(1 for r in w if not r['success'])} |")
    print()


def delivery_table(rows: list) -> None:
    print("### Delivery of the task's wrong note (described)\n")
    print("| arm | valid | in the session-start hook (any tier) | there whole (`summary`) | "
          "labelled `reverted` there | whole in a pre-read answer | Phase 0's line |")
    print("|---|---|---|---|---|---|---|")
    for label in ARMS:
        v = [r for r in rows if r["label"] == label and r["valid"]]
        c = lambda k: sum(1 for r in v if r["d"].get(k))  # noqa: E731
        print(f"| {label} | {len(v)} | {c('start_handle')} | {c('start_whole')} | "
              f"{c('start_reverted')} | {c('pre_read_whole')} | {c('hint')} |")
    print()


def per_session(rows: list) -> None:
    print("### Per session\n")
    print("| run | task | arm | rep | status | accepted | wrote wrong edit | hook: any / whole / "
          "reverted | turns | cost |")
    print("|---|---|---|---|---|---|---|---|---|---|")
    yn = lambda b: "yes" if b else "no"  # noqa: E731
    for r in rows:
        d = r.get("d") or {}
        hook = "/".join(yn(d.get(k)) for k in ("start_handle", "start_whole", "start_reverted"))
        print(f"| {r['run']} | {r['task']} | {r['label']} | {r['rep']} | {r['status']} | "
              f"{yn(r['success'])} | {yn(r['wrote_wrong'])} | {hook} | "
              f"{r['metrics']['turns']} | ${r['cost_usd']:.4f} |")
    print()


def main() -> None:
    rows = [r for r in load() if r["task"] in WRONG]
    if not rows:
        sys.exit("no rows yet")
    odd = sorted({r["label"] for r in rows if r["label"].startswith("?")})
    if odd:
        print(f"Unexpected arms in the runs: {odd}\n")
    print("## M1 Part 2: standing on the M0 tasks, the wrong note from a reverted task\n")
    print("### Pooled over the three tasks\n")
    accepted_table(rows)
    decision(rows)
    without_exposed(rows)
    per_task(rows)
    delivery_table(rows)
    wrong_edit(rows)
    superseded(rows)
    per_session(rows)


if __name__ == "__main__":
    main()
