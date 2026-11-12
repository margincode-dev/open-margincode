"""Post hoc (written after the run, when a transcript showed an agent reading the grading test):
which sessions looked outside their own working copy, and at what?

Reads each session's `events.jsonl` under RUNS and classifies every tool call's input:

- `hidden test`: it names a `test_relay_*` file. During a session that file is not in the
  session's own copy (the harness adds it for verify, afterwards), so the name can only come from
  another session's kept copy or from the bench tree.
- `other session`: it names another session's run directory or av repo.
- `source tree`: it names the bench tree (`/src`, `bench/tasks-relay`) or the source repo
  (`/root/bench-repos`).
- `looked around`: it lists or searches `/root/bench-work` or `/` without naming any of the above.
- `web`: a WebSearch or WebFetch call, or a `curl`/`wget` to a host other than this machine. The
  repository with the grading tests is public.
- `daemon`: an HTTP call to the bench daemon (`:7450`) that does not go through `av`.

A session is `exposed` if it has a `hidden test`, `other session` or `source tree` call. The
classes are by what the command names, not by what it printed.

Usage: python3 bench/results/2026-11-wrong-lesson/leak.py [RUNS]   (default /root/bench-work/runs)
"""

import collections
import json
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
RUNS = Path(sys.argv[1] if len(sys.argv) > 1 else "/root/bench-work/runs")
ARMS = ["cold", "notes", "av"]
SESSION = re.compile(r"relay-[a-z-]+-(?:cold|notes|av)-r?\d+")
WEB = re.compile(r"(?:curl|wget)\s[^|;]*https?://(?!127[.]0[.]0[.]1|localhost)")
DAEMON = re.compile(r"(?:curl|wget)\s[^|;]*(?:127[.]0[.]0[.]1|localhost)|:7450")
# `ls` or `find` whose path argument is the work root, one of its three top directories, or `/`
AROUND = re.compile(r"(?:^|[^a-z])(?:ls|find)\s+(?:-[A-Za-z0-9]+\s+)*(?:/root/bench-work(?:/runs|/ws|/src)?/?|/)(?=[\s;|&\\\"'])")


def classify(run: str, row: dict) -> set:
    name = f"{row['task']}-{row['arm']}-r{row['rep']}"
    repo = f"b{run}-{row['task']}-{row['arm']}-{row['rep']}"
    own = {name, repo}
    kinds = set()
    path = RUNS / run / name / "events.jsonl"
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        try:
            e = json.loads(line)
        except json.JSONDecodeError:
            continue
        if e.get("type") != "assistant":
            continue
        for c in (e.get("message") or {}).get("content") or []:
            if not (isinstance(c, dict) and c.get("type") == "tool_use"):
                continue
            s = json.dumps(c.get("input") or {})
            if c.get("name") in ("WebSearch", "WebFetch") or WEB.search(s):
                kinds.add("web")
            if DAEMON.search(s):
                kinds.add("daemon")
            if "test_relay_" in s:
                kinds.add("hidden test")
            others = {m for m in SESSION.findall(s)} - own
            others = {o for o in others if not any(o in x or x in o for x in own)}
            if others:
                kinds.add("other session")
            if re.search(r"/src/|bench/tasks-relay|/root/bench-repos", s):
                kinds.add("source tree")
            if AROUND.search(s):
                kinds.add("looked around")
    return kinds


def main() -> None:
    rows = []
    for run in ("cold", "wrong"):
        for line in (HERE / run / "rows.jsonl").read_text(encoding="utf-8").splitlines():
            r = json.loads(line)
            r["kinds"] = classify(run, r)
            r["exposed"] = bool(r["kinds"] & {"hidden test", "other session", "source tree"})
            rows.append(r)

    print("## Sessions that looked outside their own copy\n")
    print("| arm | sessions | exposed | names a hidden test | names another session | "
          "names the source tree | only looked around | web | daemon over HTTP |")
    print("|---|---|---|---|---|---|---|---|---|")
    for arm in ARMS:
        v = [r for r in rows if r["arm"] == arm]
        c = lambda k: sum(1 for r in v if k in r["kinds"])  # noqa: E731
        around = sum(1 for r in v if r["kinds"] == {"looked around"})
        print(f"| {arm} | {len(v)} | {sum(r['exposed'] for r in v)} | {c('hidden test')} | "
              f"{c('other session')} | {c('source tree')} | {around} | {c('web')} | {c('daemon')} |")

    print("\n## Exposed sessions per task\n")
    tasks = sorted({r["task"] for r in rows})
    print("| task | " + " | ".join(ARMS) + " |")
    print("|---|" + "---|" * len(ARMS))
    for t in tasks:
        cells = []
        for arm in ARMS:
            v = [r for r in rows if r["task"] == t and r["arm"] == arm]
            cells.append(f"{sum(r['exposed'] for r in v)}/{len(v)}")
        print(f"| {t} | " + " | ".join(cells) + " |")

    print("\n## Accepted, with exposed sessions set apart (post hoc, not a pre-registered measure)\n")
    print("| arm | all: accepted | not exposed: accepted | exposed: accepted |")
    print("|---|---|---|---|")
    for arm in ARMS:
        v = [r for r in rows if r["arm"] == arm]
        clean = [r for r in v if not r["exposed"]]
        exp = [r for r in v if r["exposed"]]
        f = lambda x: f"{sum(r['success'] for r in x)}/{len(x)}"  # noqa: E731
        print(f"| {arm} | {f(v)} | {f(clean)} | {f(exp)} |")

    print("\n## Accepted per task among sessions that were not exposed\n")
    print("| task | " + " | ".join(ARMS) + " |")
    print("|---|" + "---|" * len(ARMS))
    for t in tasks:
        cells = []
        for arm in ARMS:
            v = [r for r in rows if r["task"] == t and r["arm"] == arm and not r["exposed"]]
            cells.append(f"{sum(r['success'] for r in v)}/{len(v)}")
        print(f"| {t} | " + " | ".join(cells) + " |")

    print("\n## Every session with a call outside its copy\n")
    print("| task | arm | rep | accepted | what its calls name |")
    print("|---|---|---|---|---|")
    for r in rows:
        if r["kinds"]:
            print(f"| {r['task']} | {r['arm']} | {r['rep']} | {'yes' if r['success'] else 'no'} | "
                  f"{', '.join(sorted(r['kinds']))} |")


if __name__ == "__main__":
    main()
