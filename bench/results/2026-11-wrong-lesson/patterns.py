"""Free check: each wrong-lesson task's `[[dead_end]]` pattern matches the text its pair's `dead`
edit writes (the edit the wrong note recommends) and none of the text its `good` edit writes.

Usage: python3 bench/results/2026-11-wrong-lesson/patterns.py   (from the repo root)
"""

import fnmatch
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "bench/tasks-relay/reference"))
from edits import PAIRS  # noqa: E402

bad = 0
for spec_path in sorted((ROOT / "bench/tasks-relay-wrong").glob("*.toml")):
    spec = tomllib.loads(spec_path.read_text(encoding="utf-8"))
    pair = spec["relay_pair"]
    for d in spec["dead_end"]:
        rx = re.compile(d["pattern"])

        def hit(side):
            return any(
                fnmatch.fnmatch(path, d["file_glob"]) and rx.search(new)
                for path, _old, new in PAIRS[pair][side]
            )

        dead, good = hit("dead"), hit("good")
        ok = dead and not good
        bad += not ok
        print(f"{'ok ' if ok else 'BAD'} {pair}: dead edit matches={dead}, good edit matches={good}")
print("all patterns ok" if not bad else f"{bad} pattern(s) wrong")
sys.exit(1 if bad else 0)
