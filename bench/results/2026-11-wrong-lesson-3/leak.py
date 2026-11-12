"""Leak scan for stage 2: the second run's `leak.py` (`../2026-11-wrong-lesson-2/leak.py`), with
this run's directories. Committed with PREREG.md, before the first paid session.

This run lives under `/b2`; the second run's set-up is still under `/bench` in the same container.
So besides this run's bench tree, source repos and import copy, everything under `/bench` and this
run's results and store count as the `source tree` class (exposing): a session of this run has no
business there. Its own directories are `/b2/bench-work/runs/<run id>/<session>` and, for `av`,
`/b2/bench-work/ws/b<run id>-<task>-<arm>-<rep>`.

What the scan can miss is said in the second run's README ("Leak check"); `content.py` is the
check that does not depend on paths.

Usage: python3 bench/results/2026-11-wrong-lesson-3/leak.py [RUNS]   (default /b2/bench-work/runs)
"""

import importlib.util
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
RUNS = Path(sys.argv[1] if len(sys.argv) > 1 else "/b2/bench-work/runs")

_spec = importlib.util.spec_from_file_location(
    "second_leak", HERE.parent / "2026-11-wrong-lesson-2" / "leak.py")
scan = importlib.util.module_from_spec(_spec)
_argv, sys.argv = sys.argv, sys.argv[:1]
_spec.loader.exec_module(scan)
sys.argv = _argv

WORK = "/b2/bench-work"
scan.HERE = HERE
scan.RUNS = RUNS
scan.WORK = WORK
scan.OUTS = ["s2-cold", "s2-wrong"]
scan.SOURCE = ("/b2/tree", "/b2/bench-repos", f"{WORK}/src", "/b2/results", "/b2/store",
               "/bench", "/src", "/root")
scan.AROUND = {"/", "/b2", WORK, f"{WORK}/runs", f"{WORK}/ws", "/home"}

if __name__ == "__main__":
    scan.main()
