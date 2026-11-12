"""What the agents did with the wrong note, counted by the first run's `behaviour.py`, unchanged.

That script reads `cold/rows.jsonl` and `wrong/rows.jsonl` beside itself, the specs in
`bench/tasks-relay-wrong` and the sessions under `<RUNS>/cold` and `<RUNS>/wrong`. This one copies
it into a temporary tree where those names point at this run's (`qs-cold`, `qs-wrong`,
`<RUNS>/qscold`, `<RUNS>/qswrong`, or the `silent-*` ones with `bench/tasks-relay-silent`) and
runs it there. Written before any row of the primary cell was read. Needs symlinks (Linux).

Usage: python3 bench/results/2026-11-wrong-lesson-2/behaviour.py [RUNS] [primary|secondary]
"""

import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
RUNS = Path(sys.argv[1] if len(sys.argv) > 1 else "/bench/bench-work/runs")
CELL = sys.argv[2] if len(sys.argv) > 2 else "primary"
OUTS, SPECS = {
    "primary": ({"cold": "qs-cold", "wrong": "qs-wrong"}, "bench/tasks-relay-wrong"),
    "secondary": ({"cold": "silent-cold", "wrong": "silent-wrong"}, "bench/tasks-relay-silent"),
}[CELL]

with tempfile.TemporaryDirectory() as tmp:
    t = Path(tmp)
    shim = t / "bench/results/shim"
    shim.mkdir(parents=True)
    (t / "runs").mkdir()
    (t / "bench/tasks-relay-wrong").symlink_to(ROOT / SPECS, target_is_directory=True)
    shutil.copy(HERE.parent / "2026-11-wrong-lesson" / "behaviour.py", shim / "behaviour.py")
    for name, out in OUTS.items():
        (shim / name).symlink_to(HERE / out, target_is_directory=True)
        (t / "runs" / name).symlink_to(RUNS / out.replace("-", ""), target_is_directory=True)
    sys.exit(subprocess.run([sys.executable, str(shim / "behaviour.py"), str(t / "runs")]).returncode)
