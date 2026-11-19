"""Leak scan for M0's second model (amendment 2): `leak.py` of this directory, which is the second
run's scan (`../2026-11-wrong-lesson-2/leak.py`), with this run's directories.

This run lives under `/m2b/m0s` in `m2b-harness`, which also holds the M1/M2 runs (everything
else under `/m2b`), a read-only copy of an older home (`/m2home`), build A's and build B's source
(`/m0/srcA`, `/b2/tree`) and the cargo targets (`/target`). All of that, and this run's own trees,
task repository, import copy, checks, results and both stores, count as `source tree`
(exposing). A session's own directories are `/m2b/m0s/bench-work/runs/<run id>/<session>` and,
for `av`, its workspace `b<run id>-<task>-<arm>-<rep>` under its build's workspace root
(`ws-a`, `ws-b`); both roots are read as one `ws` root, as `leak.py` does. A path under
`/m2b/m0s/A/bin` or `/m2b/m0s/B/bin` (the `av` on the command's PATH) is listed and not exposing.
Naming `bench/tasks-m0` counts as `source tree`. The exposed sessions are written to
`checks-sonnet/exposed.json`, not this directory's `exposed.json` (the Haiku run's).

Usage: python3 sonnet_leak.py [RUNS]   (default /m2b/m0s/bench-work/runs)
"""

import importlib.util
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
RUNS = Path(sys.argv[1] if len(sys.argv) > 1 else "/m2b/m0s/bench-work/runs")

_spec = importlib.util.spec_from_file_location("m0_leak", HERE / "leak.py")
m0 = importlib.util.module_from_spec(_spec)
_argv, sys.argv = sys.argv, [sys.argv[0], str(RUNS)]
_spec.loader.exec_module(m0)
sys.argv = _argv
scan = m0.scan

R = "/m2b/m0s"
WORK = f"{R}/bench-work"
# every entry of /m2b in m2b-harness other than this run's root, as listed before the run
M2B = (".claude", ".claude.json", "H", "bench-repos", "bench-work", "bin", "build.done",
       "build.log", "checks", "daemon.log", "dreamer.sh", "m2bchain.sh", "m2bcmd.sh",
       "main-21f30dc", "pilot-dreammd", "pilot-dreammd-2", "pilot-dreammd-3", "r1", "r2", "r3",
       "results", "src", "store", "tree", "tree-h", "verifier.sh")
OUT = HERE / "checks-sonnet"
OUT.mkdir(exist_ok=True)
scan.HERE = OUT
scan.RUNS = RUNS
scan.WORK = WORK
scan.OUTS = ["m0s-av-before", "m0s-phase0"]
scan.SOURCE = (f"{R}/tree", f"{R}/tree-s", f"{R}/bench-repos", f"{WORK}/src", f"{R}/results",
               f"{R}/checks", f"{R}/A/store", f"{R}/B/store",
               *(f"/m2b/{x}" for x in M2B), "/m2home", "/m0", "/b2", "/target", "/src", "/root")
scan.AROUND = {"/", "/m2b", R, f"{R}/A", f"{R}/B", WORK, f"{WORK}/runs", f"{WORK}/ws", "/home"}
m0.WORK = WORK
m0.WS = m0.re.compile(m0.re.escape(WORK) + r"/ws-[ab](?=/|\b|$)")

if __name__ == "__main__":
    scan.main()
