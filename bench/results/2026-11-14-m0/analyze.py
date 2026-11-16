"""Tables and the decision for M0, from what the harness wrote. Changes nothing.

Reads, beside this file, `m0-cold/`, `m0-av-before/`, `m0-wrong/` and `m0-guard/` (`rows.jsonl`
as the harness wrote it), `m0-av-before/superseded.jsonl` and `m0-wrong/superseded.jsonl` (the
store export of PREREG.md, "The store export"), `m0-av-before/records.jsonl` and
`m0-wrong/records.jsonl` (every record the sessions' agent wrote, for secondary measure 5), and
each session's directory under RUNS (`<RUNS>/<run id>/<task>-<arm>-r<rep>`; the run id is the
`--out` name without hyphens).

The harness calls both `av` arms `av`; they are told apart by their results directory:
`m0-av-before` is `av-before` (build A), `m0-wrong`'s `av` is `av-phase0` (build B). The guard's
`av` rows (`m0-guard`) are `guard`.

Reused, not copied: the Wilson interval, the event reader and the per-session delivery check of
the first wrong-lesson run (`../2026-11-wrong-lesson/analyze.py`), and stage 2's transcript count
of `av record --supersedes` (`../2026-11-wrong-lesson-3/analyze.py`, `supersede_use`). The
decision rule is PREREG.md's, "Metrics and decision rule". Committed with "Build", before the
first paid session.

Usage: python3 bench/results/2026-11-14-m0/analyze.py [RUNS]   (default /m0/bench-work/runs)
"""

import importlib.util
import json
import re
import sys
import tomllib
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
RUNS = Path(sys.argv[1] if len(sys.argv) > 1 else "/m0/bench-work/runs")
TASKS = ["relay-quote-split", "relay-bus-resubscribe", "relay-tax-with-rate"]
ARMS = ["cold", "notes", "av-before", "av-phase0"]
POOL = ROOT / "bench/tasks-relay/pool/notes-silent.toml"
# (results directory, the harness's arm) -> the arm of PREREG.md
LABEL = {("m0-cold", "cold"): "cold", ("m0-av-before", "av"): "av-before",
         ("m0-wrong", "notes"): "notes", ("m0-wrong", "av"): "av-phase0",
         ("m0-guard", "av"): "guard"}
# stage 2, `relay-quote-split` on notes-wrong.toml: set beside, never compared (PREREG.md, 2)
STAGE2_QS = {"cold": (12, 12), "notes": (28, 30), "av-phase0": (12, 30), "av-before": (2, 30)}


def _load(name: str, path: Path):
    spec = importlib.util.spec_from_file_location(name, path)
    mod = importlib.util.module_from_spec(spec)
    argv, sys.argv = sys.argv, sys.argv[:1]
    spec.loader.exec_module(mod)
    sys.argv = argv
    return mod


first = _load("first_run", HERE.parent / "2026-11-wrong-lesson" / "analyze.py")
first.RUNS = RUNS
stage2 = _load("stage2", HERE.parent / "2026-11-wrong-lesson-3" / "analyze.py")
stage2.RUNS = RUNS
wilson = first.wilson

pool = tomllib.loads(POOL.read_text(encoding="utf-8"))["note"]
WRONG = {f"relay-{n['pair']}": n["summary"] for n in pool if n.get("no_commit")}
WRONG_SUMMARIES = set(WRONG.values())
POOL_SUMMARIES = {n["summary"] for n in pool}
DEAD = {}
for t in TASKS:
    spec = tomllib.loads((ROOT / "bench/tasks-m0" / f"{t.removeprefix('relay-')}.toml")
                        .read_text(encoding="utf-8"))
    DEAD[t] = re.compile(spec["dead_end"][0]["pattern"])


def run_id(out: str) -> str:
    return out.replace("-", "")


def load() -> list:
    rows = []
    for out in ("m0-cold", "m0-av-before", "m0-wrong", "m0-guard"):
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
            if r["task"] in WRONG:
                r["d"] = first.delivery(run_id(out), r, WRONG[r["task"]])
                if r["arm"] == "av":
                    r["u"] = stage2.supersede_use(r, WRONG[r["task"]])
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


def accepted_table(rows: list, labels: list, task: str | None = None) -> dict:
    out = {}
    print("| arm | sessions | valid | accepted | rate | 95 % Wilson | cost | cost per accepted | "
          "median session cost |")
    print("|---|---|---|---|---|---|---|---|---|")
    for label in labels:
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
                  and r["label"] in labels and (task is None or r["task"] == task))
    print(f"\nSessions skipped for budget: {skipped}.\n")
    return out


def gate(rows: list) -> bool:
    """PREREG.md, "The gate after the control": cold accepted at least 8 of 10 on each task."""
    print("### The gate after the control\n")
    ok = True
    for t in TASKS:
        _, v, k, _ = arm(rows, "cold", t)
        passed = len(v) >= 10 and k >= 8
        ok &= passed
        print(f"- `{t}`: cold {k} of {len(v)} valid: {'passes' if passed else 'misses'} the gate.")
    print(f"\n**The gate {'passes' if ok else 'is missed'}.**"
          + ("" if ok else " Commands 2 to 4 do not run, and the run stops.") + "\n")
    return ok


def decision(rows: list, t: dict, guard_rows: list) -> None:
    (bn, bk, bw), (pn, pk, pw) = t["av-before"], t["av-phase0"]
    print("### Decision by the pre-registered rule (primary: av-phase0 against av-before)\n")
    per_task = all(len(arm(rows, a, x)[1]) >= 10 for a in ("av-before", "av-phase0")
                   for x in TASKS)
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
        print(f"- **Not shown at this size**: av-phase0 {pk}/{pn} ({fmt_iv(pw)}), av-before "
              f"{bk}/{bn} ({fmt_iv(bw)}).\n")
    _, gv, gk, gw = arm(guard_rows, "guard")
    if len(gv) < 30:
        print(f"- **Guard not assessed**: {len(gv)} valid sessions of 30.\n")
    else:
        print(f"- **Guard {'holds' if gk >= 27 else 'fails'}**: {gk} of {len(gv)} accepted "
              f"({fmt_iv(gw)}); the bar is at least 27 of 30.\n")
    (nn, nk, nw) = t["notes"]
    print("### Secondary 1: the gap to the notes file (av-phase0 against notes)\n")
    if not pw or not nw:
        print("- Not assessed: an arm has no valid sessions.\n")
    elif pw[1] < nw[0]:
        print(f"- **The gap remains**: av-phase0 {pk}/{pn} (upper end {pw[1]:.3f}) is below notes "
              f"{nk}/{nn} (lower end {nw[0]:.3f}).\n")
    else:
        print(f"- **av is not shown worse than notes at this size**: av-phase0 {pk}/{pn} "
              f"({fmt_iv(pw)}), notes {nk}/{nn} ({fmt_iv(nw)}).\n")


def without_exposed(rows: list) -> None:
    p = HERE / "exposed.json"
    if not p.exists():
        print("### Without exposed sessions\n\n`exposed.json` is missing: run `leak.py` first.\n")
        return
    exposed = {tuple(x) for x in json.loads(p.read_text(encoding="utf-8"))}
    kept = [r for r in rows if (r["run"], r["task"], r["arm"], r["rep"]) not in exposed]
    print(f"### Without exposed sessions ({len(rows) - len(kept)} left out)\n")
    accepted_table([r for r in kept if r["task"] in WRONG], ARMS)


def per_task(rows: list) -> None:
    print("### Secondary 2: per task\n")
    for t in TASKS:
        print(f"#### `{t}`\n")
        accepted_table(rows, ARMS, t)
    print("Stage 2 on `relay-quote-split` (another pool, another run; a description, not a "
          "comparison): " + ", ".join(f"{a} {k}/{n}" for a, (k, n) in STAGE2_QS.items()) + ".\n")


def session_repo(r: dict) -> str:
    """`b<run id>-<task>-<arm short>-<rep>`, `runner.rs` at both build commits."""
    return f"b{run_id(r['run'])}-{r['task']}-{r['arm']}-{r['rep']}"


def export(out: str, name: str) -> list:
    p = HERE / out / name
    if not p.exists():
        return []
    return [json.loads(line) for line in p.read_text(encoding="utf-8").splitlines() if line]


def superseded(rows: list) -> None:
    print("### Secondary 3: superseded records, read from the store\n")
    by_repo = {}
    for out in ("m0-av-before", "m0-wrong"):
        for x in export(out, "superseded.jsonl"):
            by_repo.setdefault(x["repo"], []).append(x)
    print("| arm | valid | superseded its task's wrong note | another task's wrong note | "
          "a true note | transcript: superseded the wrong note (stage 2's count) |")
    print("|---|---|---|---|---|---|")
    for label in ("av-before", "av-phase0"):
        v = [r for r in rows if r["label"] == label and r["valid"]]
        own = other = true = 0
        for r in v:
            xs = by_repo.get(session_repo(r), [])
            s = {x["superseded_summary"] for x in xs}
            r["sup_own"] = WRONG[r["task"]] in s
            own += r["sup_own"]
            other += any(x in WRONG_SUMMARIES and x != WRONG[r["task"]] for x in s)
            true += any(x not in WRONG_SUMMARIES for x in s)
        hits = sum(1 for r in v if r.get("u", {}).get("hit"))
        print(f"| {label} | {len(v)} | {own} | {other} | {true} | {hits} |")
    print("\n| av-phase0 | accepted | not accepted |")
    print("|---|---|---|")
    v = [r for r in rows if r["label"] == "av-phase0" and r["valid"]]
    for name, want in (("superseded the task's wrong note", True), ("did not", False)):
        g = [r for r in v if bool(r.get("sup_own")) == want]
        print(f"| {name} | {sum(1 for r in g if r['success'])} | "
              f"{sum(1 for r in g if not r['success'])} |")
    print()


def wrong_edit(rows: list) -> None:
    print("### Secondary 4: the wrong edit (`repeated_dead_ends` >= 1)\n")
    print("| arm | task | valid | wrote it | wrote it, accepted | wrote it, not accepted |")
    print("|---|---|---|---|---|---|")
    for label in ARMS:
        for t in TASKS:
            v = [r for r in rows if r["label"] == label and r["task"] == t and r["valid"]]
            w = [r for r in v if r["wrote_wrong"]]
            print(f"| {label} | {t} | {len(v)} | {len(w)} | {sum(1 for r in w if r['success'])} | "
                  f"{sum(1 for r in w if not r['success'])} |")
    print()


def restatements(rows: list) -> None:
    print("### Secondary 5: records that restate the wrong edit\n")
    recs = {}
    for out in ("m0-av-before", "m0-wrong"):
        for x in export(out, "records.jsonl"):
            if x["summary"] not in POOL_SUMMARIES:
                recs.setdefault(x["repo"], []).append(x)
    print("| arm | valid | a record matching the pattern that supersedes the wrong note | "
          "one that does not (restatement, upper bound) |")
    print("|---|---|---|---|")
    for label in ("av-before", "av-phase0"):
        v = [r for r in rows if r["label"] == label and r["valid"]]
        corr = rest = 0
        for r in v:
            xs = [x for x in recs.get(session_repo(r), [])
                  if DEAD[r["task"]].search(x["summary"] + "\n" + (x.get("body") or ""))]
            corr += any(x.get("superseded_summary") == WRONG[r["task"]] for x in xs)
            rest += any(x.get("superseded_summary") != WRONG[r["task"]] for x in xs)
        print(f"| {label} | {len(v)} | {corr} | {rest} |")
    print()


def delivery(rows: list) -> None:
    print("### Secondary 6: delivery of the task's wrong note\n")
    print("| arm | valid | delivered | session-start hook | read NOTES.md | line in the pack |")
    print("|---|---|---|---|---|---|")
    for label in ARMS:
        v = [r for r in rows if r["label"] == label and r["valid"]]
        c = lambda k: sum(1 for r in v if r["d"][k])  # noqa: E731
        hint = sum(1 for r in v if r.get("u", {}).get("hint"))
        print(f"| {label} | {len(v)} | {c('delivered')} | {c('start_hook')} | "
              f"{c('read_notes_md')} | {hint} |")
    print("\n`delivered`: `notes`, the whole summary in a tool result; the `av` arms, in the "
          "session-start hook's output or a pre-read answer; `cold`, anywhere (expected 0).\n")


def per_session(rows: list) -> None:
    print("### Per session\n")
    print("| run | task | arm | rep | status | accepted | wrote wrong edit | delivered | turns | "
          "cost |")
    print("|---|---|---|---|---|---|---|---|---|---|")
    for r in rows:
        d = r.get("d", {}).get("delivered")
        print(f"| {r['run']} | {r['task']} | {r['label']} | {r['rep']} | {r['status']} | "
              f"{'yes' if r['success'] else 'no'} | {'yes' if r['wrote_wrong'] else 'no'} | "
              f"{'-' if d is None else 'yes' if d else 'no'} | {r['metrics']['turns']} | "
              f"${r['cost_usd']:.4f} |")
    print()


def main() -> None:
    rows = load()
    if not rows:
        sys.exit("no rows yet")
    odd = sorted({r["label"] for r in rows if r["label"].startswith("?")})
    if odd:
        print(f"Unexpected arms in the runs: {odd}\n")
    m0 = [r for r in rows if r["task"] in WRONG and r["label"] != "guard"]
    print("## M0: correction on three silent tasks\n")
    print("### Pooled over the three tasks\n")
    t = accepted_table(m0, ARMS)
    if not gate(m0):
        per_task(m0)
        per_session(m0)
        return
    print("### The guard (ten true-lesson tasks, N = 0)\n")
    accepted_table(rows, ["guard"])
    decision(m0, t, rows)
    without_exposed(m0)
    per_task(m0)
    superseded(m0)
    wrong_edit(m0)
    restatements(m0)
    delivery(m0)
    per_session(rows)


if __name__ == "__main__":
    main()
