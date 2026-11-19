"""Proves every relay pair is a fair test: its hidden test passes on the reference fix and fails
on the dead end, and the base tree's own tests still pass with the reference fix applied.

For the pairs in `edits.SILENT` it proves more, with the repo's own tests and the hidden test run
apart: on the base tree the own tests pass and the hidden test fails; with the reference fix both
pass; with the dead end the own tests still pass and only the hidden test fails. That last line is
what makes a dead end silent: nothing the agent can run says the approach is wrong.

Usage: python check.py [TALLYHO_REPO] [PYTHON]
(defaults: ~/bench-repos/tallyho at `main`, ~/bench-work/venv/bin/python)
"""

import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

from edits import PAIRS, SILENT

HERE = Path(__file__).resolve().parent
FILES = HERE.parent / "files"


def apply(root: Path, edits) -> None:
    for path, old, new in edits:
        p = root / path
        text = p.read_text()
        if text.count(old) != 1:
            raise SystemExit(f"{path}: edit anchor found {text.count(old)} times: {old[:60]!r}")
        p.write_text(text.replace(old, new))


def run(root: Path, python: str, target: str = "tests") -> bool:
    r = subprocess.run([python, "-m", "pytest", "-q", "-p", "no:cacheprovider", target],
                       cwd=root, capture_output=True, text=True)
    return r.returncode == 0


def clone(repo: Path, tmp: str, edits) -> Path:
    root = Path(tmp) / "tallyho"
    subprocess.run(["git", "clone", "-q", "--no-hardlinks", str(repo), str(root)], check=True)
    subprocess.run(["git", "-C", str(root), "checkout", "-q", "main"], check=True)
    apply(root, edits)
    return root


def silent(repo: Path, python: str, name: str, test: str) -> bool:
    """The repo's own tests and the hidden test, run apart, on the base, the fix and the dead end."""
    want = {"base": (True, False), "good": (True, True), "dead": (True, False)}
    label = {"base": "base", "good": "reference", "dead": "dead end"}
    got = {}
    for side in want:
        with tempfile.TemporaryDirectory() as tmp:
            root = clone(repo, tmp, PAIRS[name][side] if side != "base" else [])
            own = run(root, python)
            shutil.copy(FILES / test, root / "tests" / test)
            got[side] = (own, run(root, python, f"tests/{test}"))
    ok = got == want
    words = {True: "pass", False: "FAIL"}
    hidden = {True: "passes", False: "fails"}
    print(f"{'ok ' if ok else 'BAD'} {name} (silent): " + "; ".join(
        f"{label[s]}: own tests {words[got[s][0]]}, hidden test {hidden[got[s][1]]}" for s in want))
    return ok


def main() -> None:
    repo = Path(sys.argv[1] if len(sys.argv) > 1 else os.path.expanduser("~/bench-repos/tallyho"))
    python = sys.argv[2] if len(sys.argv) > 2 else os.path.expanduser("~/bench-work/venv/bin/python")
    unknown = [name for name in SILENT if name not in PAIRS]
    if unknown:
        raise SystemExit(f"SILENT names pairs without edits: {', '.join(unknown)}")
    bad = []
    for name, sides in PAIRS.items():
        test = f"test_relay_{name.replace('-', '_')}.py"
        results = {}
        for side in ("good", "dead"):
            with tempfile.TemporaryDirectory() as tmp:
                root = clone(repo, tmp, sides[side])
                shutil.copy(FILES / test, root / "tests" / test)
                results[side] = run(root, python)
        ok = results["good"] and not results["dead"]
        print(f"{'ok ' if ok else 'BAD'} {name}: reference {'passes' if results['good'] else 'FAILS'}, "
              f"dead end {'fails' if not results['dead'] else 'PASSES'}")
        if name in SILENT:
            ok = silent(repo, python, name, test) and ok
        if not ok:
            bad.append(name)
    if bad:
        raise SystemExit(f"not a fair test: {', '.join(bad)}")
    print(f"all {len(PAIRS)} pairs ok ({len(SILENT)} of them silent)")


if __name__ == "__main__":
    main()
