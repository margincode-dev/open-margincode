"""Leak scan for the re-run: which sessions named anything outside their own working copy?

Written while the primary cell was still running and before any of its rows was read. It is the
first run's `leak.py` (`../2026-11-wrong-lesson/leak.py`; its `WEB` and `DAEMON` patterns are
imported) with three changes the closed set-up needs:

- paths: the bench tree is `/bench/tree`, the source repos `/bench/bench-repos`, the work root
  `/bench/bench-work`, results `/bench/results`;
- another session is found by exact name. The first script dropped a name that contains the
  session's own (`…-r1` inside `…-r10`), which was harmless with six repetitions and is not with
  thirty;
- every absolute path a call names outside the session's own directories is listed, so a route
  nobody thought of still shows up.

Reads each session's `events.jsonl` under `<RUNS>/<run id>/<task>-<arm>-r<rep>` (the run id is the
`--out` name without hyphens) and classifies every tool call by what its input names, not by what
it printed; the closed harness is expected to refuse the read, so the answer each flagged call got
is printed beside it.

- `hidden test`: names a `test_relay_*` file (not in a session's copy until verify adds it).
- `other session`: names another session's directory or av workspace.
- `source tree`: names the bench tree, the source repos, the import copy, `/src` or `/root`.
- `results`: names `/bench/results`, `rows.jsonl` or `first-packs.jsonl`.
- `store`: names the daemon's store, `/bench/store`.
- `looked around`: names `/`, `/bench`, the work root or its `runs`/`ws` directory itself, or `..`.
- `tmp`: names `/tmp`.
- `web`: a WebSearch or WebFetch call, or `curl`/`wget` to another host.
- `daemon`: an HTTP call to the bench daemon that does not go through `av`.

A session is `exposed` if it has a `hidden test`, `other session`, `source tree`, `results` or
`store` call. The exposed sessions are written to `exposed.json` for `analyze.py`.

Usage: python3 bench/results/2026-11-wrong-lesson-2/leak.py [RUNS]   (default /bench/bench-work/runs)
"""

import collections
import importlib.util
import json
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
RUNS = Path(sys.argv[1] if len(sys.argv) > 1 else "/bench/bench-work/runs")
WORK = "/bench/bench-work"
ARMS = ["cold", "notes", "av"]
OUTS = ["qs-cold", "qs-wrong", "silent-cold", "silent-wrong"]
EXPOSING = {"hidden test", "other session", "source tree", "results", "store"}

_spec = importlib.util.spec_from_file_location(
    "first_leak", HERE.parent / "2026-11-wrong-lesson" / "leak.py")
_first = importlib.util.module_from_spec(_spec)
_argv, sys.argv = sys.argv, sys.argv[:1]
_spec.loader.exec_module(_first)
sys.argv = _argv
WEB, DAEMON = _first.WEB, _first.DAEMON

DIR = re.compile(r"^(relay-[a-z-]+)-(cold|notes|av)-r(\d+)$")
SESSION = re.compile(r"relay-[a-z-]+-(?:cold|notes|av)-r?\d+")
# an absolute path: a `/` that does not continue a word, a URL or a relative path
ABS = re.compile(r"(?<![\w.~/:-])/(?:[\w.@+-]+/)*[\w.@+-]*")
SOURCE = ("/bench/tree", "/bench/bench-repos", f"{WORK}/src", "/src", "/root")
AROUND = {"/", "/bench", WORK, f"{WORK}/runs", f"{WORK}/ws", "/home"}


def under(path: str, root: str) -> bool:
    return path == root or path.startswith(root + "/")


def kinds_of(text: str, own_roots: set, own_names: set):
    """The classes one tool input falls in, and the paths it names outside `own_roots`."""
    kinds, outside = set(), []
    if "test_relay_" in text:
        kinds.add("hidden test")
    if set(SESSION.findall(text)) - own_names:
        kinds.add("other session")
    if "bench/tasks-relay" in text:
        kinds.add("source tree")
    if "rows.jsonl" in text or "first-packs" in text:
        kinds.add("results")
    if "../" in text or text.rstrip('"} ').endswith(".."):
        kinds.add("looked around")
    for p in ABS.findall(text):
        p = p.rstrip("/") or "/"
        if any(under(p, r) for r in own_roots):
            continue
        outside.append(p)
        if any(under(p, s) for s in SOURCE):
            kinds.add("source tree")
        elif under(p, "/bench/results"):
            kinds.add("results")
        elif under(p, "/bench/store"):
            kinds.add("store")
        elif p in AROUND:
            kinds.add("looked around")
        elif under(p, f"{WORK}/runs") or under(p, f"{WORK}/ws"):
            # a run directory itself is a look around; anything inside one is a session's
            deep = len(Path(p).relative_to(WORK).parts) > 2
            kinds.add("other session" if deep else "looked around")
        elif under(p, "/tmp"):
            kinds.add("tmp")
    return kinds, outside


def scan(run: str, task: str, arm: str, rep: str) -> dict:
    name = f"{task}-{arm}-r{rep}"
    own_roots = {f"{WORK}/runs/{run}/{name}", f"{WORK}/ws/b{run}-{task}-{arm}-{rep}"}
    own_names = {name, f"{task}-{arm}-{rep}"}
    calls, answers = [], {}
    kinds, outside = set(), collections.Counter()
    path = RUNS / run / name / "events.jsonl"
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        try:
            e = json.loads(line)
        except json.JSONDecodeError:
            continue
        content = (e.get("message") or {}).get("content")
        if not isinstance(content, list):
            continue
        for c in content:
            if not isinstance(c, dict):
                continue
            if e.get("type") == "user" and c.get("type") == "tool_result":
                answers[c.get("tool_use_id")] = json.dumps(c.get("content"))[:200]
            if e.get("type") != "assistant" or c.get("type") != "tool_use":
                continue
            s = json.dumps(c.get("input") or {})
            k, out = kinds_of(s, own_roots, own_names)
            if c.get("name") in ("WebSearch", "WebFetch") or WEB.search(s):
                k.add("web")
            if DAEMON.search(s):
                k.add("daemon")
            kinds |= k
            outside.update(out)
            if k & EXPOSING or k & {"web", "daemon"}:
                calls.append((sorted(k), c.get("name"), s[:240], c.get("id")))
    return {
        "run": run, "task": task, "arm": arm, "rep": int(rep), "kinds": kinds,
        "exposed": bool(kinds & EXPOSING), "outside": outside,
        "calls": [(k, n, s, answers.get(i, "")) for k, n, s, i in calls],
    }


def cell(text: str) -> str:
    return text.replace("|", "\\|").replace("\n", " ")


def main() -> None:
    rows = []
    for out in OUTS:
        run = out.replace("-", "")
        if not (RUNS / run).is_dir():
            continue
        for d in sorted((RUNS / run).iterdir()):
            m = DIR.match(d.name)
            if m and (d / "events.jsonl").exists():
                rows.append(scan(run, *m.groups()) | {"out": out})

    print("## Sessions that named something outside their own copy\n")
    print("| run | arm | sessions | exposed | hidden test | other session | source tree | "
          "results | store | looked around | tmp | web | daemon over HTTP |")
    print("|---|---|---|---|---|---|---|---|---|---|---|---|---|")
    for out in OUTS:
        for arm in ARMS:
            v = [r for r in rows if r["out"] == out and r["arm"] == arm]
            if not v:
                continue
            c = lambda k: sum(1 for r in v if k in r["kinds"])  # noqa: E731
            print(f"| {out} | {arm} | {len(v)} | {sum(r['exposed'] for r in v)} | "
                  + " | ".join(str(c(k)) for k in (
                      "hidden test", "other session", "source tree", "results", "store",
                      "looked around", "tmp", "web", "daemon")) + " |")

    print("\n## Every flagged call, and the answer it got\n")
    print("| run | session | classes | tool | input (240 chars) | answer (200 chars) |")
    print("|---|---|---|---|---|---|")
    n = 0
    for r in rows:
        for k, tool, s, answer in r["calls"]:
            n += 1
            print(f"| {r['out']} | {r['task']}-{r['arm']}-r{r['rep']} | {', '.join(k)} | {tool} | "
                  f"`{cell(s)}` | `{cell(answer)}` |")
    if not n:
        print("| (none) | | | | | |")

    print("\n## Absolute paths named outside the session's own directories\n")
    print("By the first three components; the number of calls across all sessions.\n")
    print("| path | calls | sessions |")
    print("|---|---|---|")
    calls, sessions = collections.Counter(), collections.Counter()
    for r in rows:
        seen = set()
        for p, c in r["outside"].items():
            top = "/" + "/".join(Path(p).parts[1:4])
            calls[top] += c
            seen.add(top)
        sessions.update(seen)
    for top, c in sorted(calls.items()):
        print(f"| `{top}` | {c} | {sessions[top]} |")

    exposed = [[r["out"], r["task"], r["arm"], r["rep"]] for r in rows if r["exposed"]]
    (HERE / "exposed.json").write_text(json.dumps(exposed, indent=1) + "\n", encoding="utf-8")
    print(f"\n{len(rows)} sessions scanned, {len(exposed)} exposed (written to exposed.json).")


if __name__ == "__main__":
    main()
