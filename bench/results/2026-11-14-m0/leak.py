"""Leak scan for M0: the second run's `leak.py` (`../2026-11-wrong-lesson-2/leak.py`), with this
run's directories, as stage 2's (`../2026-11-wrong-lesson-3/leak.py`) did. Committed with "Build"
of PREREG.md, before the first paid session.

This run lives under `/m0` in the container that also holds the second run (`/bench`) and stage 2
(`/b2`). Everything of theirs, and this run's bench tree, build A's source, source repos, import
copy, checks, results and both stores, count as `source tree` (exposing): a session of this run has
no business there. A session's own directories are `/m0/bench-work/runs/<run id>/<session>` and,
for `av`, its workspace `b<run id>-<task>-<arm>-<rep>` under its daemon's workspace root
(`/m0/bench-work/ws-a` for build A, `ws-b` for build B). Both roots are read as the second run's
one `ws` root, so another session's workspace under either is `other session`, and either root
itself is `looked around`. A path under `/m0/A/bin` or `/m0/B/bin` (the `av` the hooks run) is
listed and not exposing. Naming `bench/tasks-m0` counts as `source tree`, like `bench/tasks-relay`.

Usage: python3 bench/results/2026-11-14-m0/leak.py [RUNS]   (default /m0/bench-work/runs)
"""

import importlib.util
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
RUNS = Path(sys.argv[1] if len(sys.argv) > 1 else "/m0/bench-work/runs")

_spec = importlib.util.spec_from_file_location(
    "second_leak", HERE.parent / "2026-11-wrong-lesson-2" / "leak.py")
scan = importlib.util.module_from_spec(_spec)
_argv, sys.argv = sys.argv, sys.argv[:1]
_spec.loader.exec_module(scan)
sys.argv = _argv

WORK = "/m0/bench-work"
scan.HERE = HERE
scan.RUNS = RUNS
scan.WORK = WORK
scan.OUTS = ["m0-cold", "m0-av-before", "m0-wrong", "m0-guard"]
scan.SOURCE = ("/m0/tree", "/m0/srcA", "/m0/bench-repos", f"{WORK}/src", "/m0/results",
               "/m0/checks", "/m0/A/store", "/m0/B/store", "/b2", "/bench", "/src", "/root")
scan.AROUND = {"/", "/m0", "/m0/A", "/m0/B", WORK, f"{WORK}/runs", f"{WORK}/ws", "/home"}

WS = re.compile(re.escape(WORK) + r"/ws-[ab](?=/|\b|$)")
_kinds_of = scan.kinds_of


def kinds_of(text: str, own_roots: set, own_names: set):
    kinds, outside = _kinds_of(WS.sub(f"{WORK}/ws", text), own_roots, own_names)
    if "bench/tasks-m0" in text:
        kinds.add("source tree")
    return kinds, outside


scan.kinds_of = kinds_of

if __name__ == "__main__":
    scan.main()
