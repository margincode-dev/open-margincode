"""Leak scan for M1 Part 2: the second wrong-lesson run's `leak.py`
(`../../2026-11-wrong-lesson-2/leak.py`) with this run's directories, as M0's
(`../../2026-11-14-m0/leak.py`) did. Committed with Build, before the first paid session.

This run lives under `/m1` in the container that also holds M0 (`/m0`), the second run
(`/bench`) and stage 2 (`/b2`). Everything of theirs, and this run's tree, both build sources,
the harness's directory, source repos, import copy, checks, results and both stores, count as
`source tree` (exposing): a session of this run has no business there. A session's own
directories are `/m1/bench-work/runs/<run id>/<session>` and its workspace
`b<run id>-<task>-av-<rep>` under its daemon's workspace root (`/m1/bench-work/ws-p0` for build
P0, `ws-p1` for build P1). Both roots are read as the second run's one `ws` root, so another
session's workspace (or the seeded earlier tasks' workspaces) under either is `other session`,
and either root itself is `looked around`. A path under `/m1/P0/bin` or `/m1/P1/bin` (the `av`
the hooks run) is listed and not exposing. Naming `bench/tasks-m0` counts as `source tree`.

Usage: python3 leak.py [RUNS]   (default /m1/bench-work/runs)
"""

import importlib.util
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
RUNS = Path(sys.argv[1] if len(sys.argv) > 1 else "/m1/bench-work/runs")

_spec = importlib.util.spec_from_file_location(
    "second_leak", HERE.parents[1] / "2026-11-wrong-lesson-2" / "leak.py")
scan = importlib.util.module_from_spec(_spec)
_argv, sys.argv = sys.argv, sys.argv[:1]
_spec.loader.exec_module(scan)
sys.argv = _argv

WORK = "/m1/bench-work"
scan.HERE = HERE
scan.RUNS = RUNS
scan.WORK = WORK
scan.ARMS = ["av"]
scan.OUTS = ["m1-p0", "m1-p1"]
scan.SOURCE = ("/m1/tree-h", "/m1/srcP1", "/m1/srcH", "/m1/H", "/m1/bench-repos", f"{WORK}/src",
               "/m1/results", "/m1/checks", "/m1/P0/store", "/m1/P1/store", "/m0", "/b2",
               "/bench", "/src", "/root")
scan.AROUND = {"/", "/m1", "/m1/P0", "/m1/P1", WORK, f"{WORK}/runs", f"{WORK}/ws", "/home"}

# this run's daemons listen on 7481 (P0) and 7482 (P1), not the first run's 7450
scan.DAEMON = re.compile(r"(?:curl|wget)\s[^|;]*(?:127[.]0[.]0[.]1|localhost)|:748[12]\b")

WS = re.compile(re.escape(WORK) + r"/ws-p[01](?=/|\b|$)")
_kinds_of = scan.kinds_of


def kinds_of(text: str, own_roots: set, own_names: set):
    kinds, outside = _kinds_of(WS.sub(f"{WORK}/ws", text), own_roots, own_names)
    if "bench/tasks-m0" in text:
        kinds.add("source tree")
    return kinds, outside


scan.kinds_of = kinds_of

if __name__ == "__main__":
    scan.main()
