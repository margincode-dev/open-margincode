"""Proves every relay pair is a fair test: its hidden test passes on the reference fix and fails
on the dead end, and the base tree's own tests still pass with the reference fix applied.

Usage: python check.py [TALLYHO_REPO] [PYTHON]
(defaults: ~/bench-repos/tallyho at `main`, ~/bench-work/venv/bin/python)
"""

import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

from edits import PAIRS

HERE = Path(__file__).resolve().parent
FILES = HERE.parent / "files"


def apply(root: Path, edits) -> None:
    for path, old, new in edits:
        p = root / path
        text = p.read_text()
        if text.count(old) != 1:
            raise SystemExit(f"{path}: edit anchor found {text.count(old)} times: {old[:60]!r}")
        p.write_text(text.replace(old, new))


def run(root: Path, python: str) -> bool:
    r = subprocess.run([python, "-m", "pytest", "-q", "-p", "no:cacheprovider", "tests"],
                       cwd=root, capture_output=True, text=True)
    return r.returncode == 0


def main() -> None:
    repo = Path(sys.argv[1] if len(sys.argv) > 1 else os.path.expanduser("~/bench-repos/tallyho"))
    python = sys.argv[2] if len(sys.argv) > 2 else os.path.expanduser("~/bench-work/venv/bin/python")
    bad = []
    for name, sides in PAIRS.items():
        test = f"test_relay_{name.replace('-', '_')}.py"
        results = {}
        for side in ("good", "dead"):
            with tempfile.TemporaryDirectory() as tmp:
                root = Path(tmp) / "tallyho"
                subprocess.run(["git", "clone", "-q", "--no-hardlinks", str(repo), str(root)], check=True)
                subprocess.run(["git", "-C", str(root), "checkout", "-q", "main"], check=True)
                apply(root, sides[side])
                shutil.copy(FILES / test, root / "tests" / test)
                results[side] = run(root, python)
        ok = results["good"] and not results["dead"]
        print(f"{'ok ' if ok else 'BAD'} {name}: reference {'passes' if results['good'] else 'FAILS'}, "
              f"dead end {'fails' if not results['dead'] else 'PASSES'}")
        if not ok:
            bad.append(name)
    if bad:
        raise SystemExit(f"not a fair test: {', '.join(bad)}")
    print(f"all {len(PAIRS)} pairs ok")


if __name__ == "__main__":
    main()
