"""Tables and the decision for stage 2, from what the harness wrote. Changes nothing.

Reads `s2-cold/rows.jsonl` and `s2-wrong/rows.jsonl` beside this file and each session's
directory under RUNS (`<RUNS>/<run id>/<task>-<arm>-r<rep>`; the run id is the `--out` name
without hyphens). The tables are the second run's (`../2026-11-wrong-lesson-2/analyze.py`,
imported); the decision is PREREG.md's of this directory, and two counts are new: whether the
pack's line about correcting records reached each `av` session, and what each `av` session did
with `av record --supersedes`. Committed with PREREG.md, before the first paid session.

Usage: python3 bench/results/2026-11-wrong-lesson-3/analyze.py [RUNS]   (default /b2/bench-work/runs)
"""

import importlib.util
import json
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
RUNS = Path(sys.argv[1] if len(sys.argv) > 1 else "/b2/bench-work/runs")
PRIMARY = "relay-quote-split"
# the first words of `render::CORRECT_HINT`, the line under the pack's records heading
HINT = "Claims by earlier agents: check against the code"
# the second run's primary cell, the reference for R1 and the drift controls
S1 = {"av": (2, 30), "notes": (27, 30), "cold": (12, 12)}

_spec = importlib.util.spec_from_file_location(
    "second_run", HERE.parent / "2026-11-wrong-lesson-2" / "analyze.py")
s1 = importlib.util.module_from_spec(_spec)
_argv, sys.argv = sys.argv, sys.argv[:1]
_spec.loader.exec_module(s1)
sys.argv = _argv
s1.HERE = HERE
s1.first.RUNS = RUNS
s1.CELLS = {"primary": (["s2-cold", "s2-wrong"], "bench/tasks-relay/pool/notes-wrong.toml")}
wilson = s1.wilson
events, strings = s1.first.events, s1.first.strings

HANDLE = re.compile(r"rec:[0-9a-f]{8}")


def session_dir(r: dict) -> Path:
    return RUNS / r["run"].replace("-", "") / f"{r['task']}-{r['arm']}-r{r['rep']}"


def supersede_use(r: dict, note: str) -> dict:
    """What an `av` session did about the wrong record, read from its transcript.

    - `hint`: the pack line (`HINT`) is in the session-start hook's output.
    - `wrong`: the handle the pack printed for the task's wrong note (the `rec:` id on the line
      that holds the note's text in the session-start hook's output).
    - `tried`: a Bash call that runs `av record` (or `scholion record`) and names `supersedes`,
      or one whose answer shows a record was superseded (a `--json` file names it out of sight).
    - `done`: an `av record` call was answered with `supersedes rec:…` (the CLI's line for a
      record the daemon accepted as a correction).
    - `hit`: one accepted call superseded the wrong note's handle.
    """
    out = {"hint": False, "wrong": None, "tried": False, "done": False, "hit": False}
    calls = {}
    for e in events(session_dir(r) / "events.jsonl"):
        if e.get("type") == "system" and e.get("subtype") == "hook_response" \
                and e.get("hook_event") == "SessionStart":
            for s in strings(e):
                out["hint"] |= HINT in s
                for line in s.splitlines():
                    if note in line and out["wrong"] is None:
                        m = HANDLE.search(line)
                        out["wrong"] = m.group(0) if m else None
        content = (e.get("message") or {}).get("content")
        if not isinstance(content, list):
            continue
        for c in content:
            if not isinstance(c, dict):
                continue
            if e.get("type") == "assistant" and c.get("type") == "tool_use" \
                    and c.get("name") == "Bash":
                cmd = str((c.get("input") or {}).get("command", ""))
                if re.search(r"\b(av|scholion) record\b", cmd):
                    out["tried"] |= "supersedes" in cmd
                    calls[c.get("id")] = cmd
            if e.get("type") == "user" and c.get("type") == "tool_result" \
                    and c.get("tool_use_id") in calls:
                answer = json.dumps(c.get("content"))
                for m in re.finditer(r"supersedes (rec:[0-9a-f]{8})", answer):
                    out["tried"] = out["done"] = True
                    out["hit"] |= m.group(1) == out["wrong"]
    return out


def decision(t: dict) -> None:
    """PREREG.md, "Metrics and decision rule"."""
    (nn, nk, nw), (an, ak, aw), (cn, ck, _) = t["notes"], t["av"], t["cold"]
    ref_av, ref_notes = wilson(*S1["av"]), wilson(*S1["notes"])
    print("### Decision by the pre-registered rule\n")
    print(f"Reference, the second run on the build without the change: av {S1['av'][0]}/"
          f"{S1['av'][1]} ({ref_av[0]:.3f} to {ref_av[1]:.3f}), notes {S1['notes'][0]}/"
          f"{S1['notes'][1]} ({ref_notes[0]:.3f} to {ref_notes[1]:.3f}).\n")
    if nn < 30 or an < 30:
        print(f"- No claim: fewer than 30 valid sessions in an arm (notes {nn}, av {an}).\n")
        return
    cold_ok = cn > 0 and ck * 12 >= 10 * cn
    notes_ok = nw[1] >= ref_notes[0] and nw[0] <= ref_notes[1]
    print(f"- Drift control, cold: {ck} of {cn} "
          f"({'at least' if cold_ok else 'below'} 10 of 12).")
    print(f"- Drift control, notes: {nk}/{nn} ({nw[0]:.3f} to {nw[1]:.3f}) "
          f"{'overlaps' if notes_ok else 'does not overlap'} the reference interval.")
    if not (cold_ok and notes_ok):
        print("- **R1 is not assessed**: a drift control failed, so this run is not compared "
              "with the second run.")
    elif aw[0] > ref_av[1]:
        print(f"- **R1 is reported**: av {ak}/{an} (lower end {aw[0]:.3f}) is above the "
              f"reference's upper end ({ref_av[1]:.3f}).")
    else:
        print(f"- **R1 is not reported**: av {ak}/{an} (lower end {aw[0]:.3f}) is not above the "
              f"reference's upper end ({ref_av[1]:.3f}).")
    if aw[1] < nw[0]:
        print(f"- **R2, the gap remains**: av {ak}/{an} (upper end {aw[1]:.3f}) is below notes "
              f"{nk}/{nn} (lower end {nw[0]:.3f}).")
    else:
        print(f"- **R2, av is not shown worse than notes at this size** (av {ak}/{an}, "
              f"{aw[0]:.3f} to {aw[1]:.3f}; notes {nk}/{nn}, {nw[0]:.3f} to {nw[1]:.3f}). "
              "This is not a finding that they are equal.")
    print()


def supersede_tables(rows: list) -> None:
    av = [r for r in rows if r["arm"] == "av" and r["valid"]]
    c = lambda key: sum(1 for r in av if r["u"][key])  # noqa: E731
    print("### The correction line and the command, `av` sessions\n")
    print("| valid | line in the session-start pack | wrong note's handle found | tried "
          "`--supersedes` | accepted by the daemon | superseded the wrong note |")
    print("|---|---|---|---|---|---|")
    print(f"| {len(av)} | {c('hint')} | {sum(1 for r in av if r['u']['wrong'])} | {c('tried')} | "
          f"{c('done')} | {c('hit')} |\n")
    print("| | accepted | not accepted |")
    print("|---|---|---|")
    for label, key, want in (("superseded the wrong note", "hit", True),
                             ("did not", "hit", False)):
        v = [r for r in av if bool(r["u"][key]) == want]
        print(f"| {label} | {sum(1 for r in v if r['success'])} | "
              f"{sum(1 for r in v if not r['success'])} |")
    print()
    print("| rep | accepted | wrote wrong edit | line | tried | accepted by daemon | "
          "superseded the wrong note |")
    print("|---|---|---|---|---|---|---|")
    yn = lambda b: "yes" if b else "no"  # noqa: E731
    for r in av:
        u = r["u"]
        print(f"| {r['rep']} | {yn(r['success'])} | {yn(r['wrote_wrong'])} | {yn(u['hint'])} | "
              f"{yn(u['tried'])} | {yn(u['done'])} | {yn(u['hit'])} |")
    print()


def main() -> None:
    rows = s1.load("primary")
    if not rows:
        sys.exit("no rows yet")
    notes = s1.wrong_notes(s1.CELLS["primary"][1])
    for r in rows:
        if r["arm"] == "av":
            r["u"] = supersede_use(r, notes[r["task"]])
    print(f"## Stage 2: `{PRIMARY}` on the build with the correction feature\n")
    other = sorted({r["task"] for r in rows} - {PRIMARY})
    if other:
        print(f"Unexpected tasks in the runs: {other}\n")
    t = s1.accepted_table(rows)
    decision(t)
    s1.without_exposed(rows)
    supersede_tables(rows)
    s1.wrote_and_delivery(rows)
    s1.per_session(rows)


if __name__ == "__main__":
    main()
