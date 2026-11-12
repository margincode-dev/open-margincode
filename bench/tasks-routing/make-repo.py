#!/usr/bin/env python3
"""Build the E-G1 pilot repo (one task per tag) at DEST (default: ./repo next to this file).

The image task is chart-to-JSON, not the Chromium SSIM mockup: the pilot does not need a
browser. assets/q3.svg is written from SERIES below (text, not a binary). The reference fix
for that task writes assets/q3.json from the same numbers. Hidden checks live in files/ and
are not copied into the repo, so a session does not see them.

Usage: make-repo.py [DEST]
Refuses to replace an existing DEST. Checks that each fix passes its hidden test and that
the base tree fails the tests that should fail there.
"""

import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
FILES = HERE / "files"
SERIES = (("apples", 12), ("pears", 7), ("plums", 15))
QUARTER = "Q3"

SLUG_BUG = '''\
def slugify(text):
    out = []
    for ch in text.lower():
        if ch.isascii() and ch.isalnum():
            out.append(ch)
        elif ch in " -_":
            out.append("-")
    return "".join(out)
'''

SLUG_FIX = '''\
def slugify(text):
    out = []
    hyphen = False
    for ch in text.lower():
        if ch.isascii() and ch.isalnum():
            out.append(ch)
            hyphen = False
        elif out and not hyphen:
            out.append("-")
            hyphen = True
    if out and out[-1] == "-":
        out.pop()
    return "".join(out)
'''

NUM_BUG = '''\
def gcd(a, b):
    a, b = abs(int(a)), abs(int(b))
    while b:
        a, b = b, a % b
    return a


def lcm(a, b):
    a, b = int(a), int(b)
    if a == 0 or b == 0:
        return 0
    return abs(a * b)
'''

NUM_FIX = '''\
def gcd(a, b):
    a, b = abs(int(a)), abs(int(b))
    while b:
        a, b = b, a % b
    return a


def lcm(a, b):
    a, b = int(a), int(b)
    if a == 0 or b == 0:
        return 0
    return abs(a * b) // gcd(a, b)
'''

VERSIONS = '''\
def parse_version(text):
    parts = text.split(".")
    if len(parts) != 3 or not all(part.isdigit() and part != "" for part in parts):
        raise ValueError(text)
    return tuple(int(part) for part in parts)
'''

TEST_VERSIONS_BUG = '''\
from versions import parse_version

assert parse_version("1.2.3") == (1, 2)
print("ok")
'''

TEST_VERSIONS_FIX = '''\
from versions import parse_version

assert parse_version("1.2.3") == (1, 2, 3)
print("ok")
'''

CLI = '''\
EXIT_OK = 0
EXIT_USAGE = 2
EXIT_MISSING = 3
'''

README_BUG = '''\
# routing-pilot

A tiny sample for the routing benchmark.

## Exit codes

- 0 success
- 2 usage error
- 1 a required file is missing
'''

README_FIX = '''\
# routing-pilot

A tiny sample for the routing benchmark.

## Exit codes

success: 0
usage: 2
missing: 3
'''

PAGE_BUG = '''\
<!DOCTYPE html>
<html lang="en">
<head><meta charset="utf-8"><title>Settings</title></head>
<body>
  <form id="profile">
    <label for="nick">Nickname</label>
    <input id="nick" name="nick">
    <a href="#save">Save</a>
  </form>
</body>
</html>
'''

PAGE_FIX = '''\
<!DOCTYPE html>
<html lang="en">
<head><meta charset="utf-8"><title>Settings</title></head>
<body>
  <form id="profile">
    <label for="display-name">Display name</label>
    <input id="display-name" name="display-name">
    <button type="submit" id="save">Save</button>
  </form>
</body>
</html>
'''


def chart_svg():
    width, height = 320, 200
    bar_w, gap = 40, 36
    max_v = max(value for _, value in SERIES)
    left, base = 48, 160
    parts = [
        '<?xml version="1.0" encoding="UTF-8"?>',
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" '
        f'viewBox="0 0 {width} {height}">',
        '<rect width="100%" height="100%" fill="white"/>',
        f'<text x="160" y="24" text-anchor="middle" font-size="16">{QUARTER} units</text>',
    ]
    for i, (name, value) in enumerate(SERIES):
        h = int(120 * value / max_v)
        x = left + i * (bar_w + gap)
        y = base - h
        parts.append(
            f'<rect x="{x}" y="{y}" width="{bar_w}" height="{h}" fill="#336699"/>'
        )
        parts.append(
            f'<text x="{x + bar_w / 2:.0f}" y="{base + 18}" text-anchor="middle" '
            f'font-size="12">{name}</text>'
        )
        parts.append(
            f'<text x="{x + bar_w / 2:.0f}" y="{y - 4}" text-anchor="middle" '
            f'font-size="12">{value}</text>'
        )
    parts.append("</svg>")
    return "\n".join(parts) + "\n"


def chart_json():
    return json.dumps(
        {"quarter": QUARTER, "units": {name: value for name, value in SERIES}},
        indent=2,
    ) + "\n"


def write(dest, rel, text):
    path = dest / rel
    path.parent.mkdir(parents=True, exist_ok=True)
    data = text.replace("\r\n", "\n").replace("\r", "\n").encode("utf-8")
    path.write_bytes(data)


def git(dest, *args):
    cmd = [
        "git",
        "-C",
        str(dest),
        "-c",
        "user.name=margincode-bench",
        "-c",
        "user.email=bench@localhost",
        "-c",
        "commit.gpgsign=false",
        "-c",
        "core.autocrlf=false",
        *args,
    ]
    subprocess.run(cmd, check=True)


def commit(dest, message):
    git(dest, "add", "-A")
    git(dest, "commit", "-qm", message)


def run_check(cwd, script, expect_ok):
    env = os.environ.copy()
    env["PYTHONDONTWRITEBYTECODE"] = "1"
    # The script lives outside the tree (or under tests/), so the repo root must be importable.
    env["PYTHONPATH"] = str(cwd)
    result = subprocess.run([sys.executable, str(script)], cwd=cwd, env=env)
    if expect_ok and result.returncode != 0:
        raise SystemExit(f"{script.name} should pass in {cwd}")
    if not expect_ok and result.returncode == 0:
        raise SystemExit(f"{script.name} should fail on the base tree")


def main():
    dest = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else HERE / "repo"
    if dest.exists():
        raise SystemExit(f"make-repo.py: {dest} exists; remove it or pass another DEST")
    dest.mkdir(parents=True)
    try:
        git(dest, "init", "-q", "-b", "main")
        base = {
            "assets/q3.svg": chart_svg(),
            "page/index.html": PAGE_BUG,
            "slugify.py": SLUG_BUG,
            "versions.py": VERSIONS,
            "tests/test_versions.py": TEST_VERSIONS_BUG,
            "cli.py": CLI,
            "README.md": README_BUG,
            "numtheory.py": NUM_BUG,
        }
        for rel, text in base.items():
            write(dest, rel, text)
        commit(dest, "base")
        fixes = {
            "fix-chart-json": {"assets/q3.json": chart_json()},
            "fix-settings-form": {"page/index.html": PAGE_FIX},
            "fix-slug-collapse": {"slugify.py": SLUG_FIX},
            "fix-version-tests": {"tests/test_versions.py": TEST_VERSIONS_FIX},
            "fix-exit-codes": {"README.md": README_FIX},
            "fix-least-multiple": {"numtheory.py": NUM_FIX},
        }
        for branch, files in fixes.items():
            git(dest, "checkout", "-q", "-b", branch, "main")
            for rel, text in files.items():
                write(dest, rel, text)
            commit(dest, branch)
            git(dest, "checkout", "-q", "main")
        # Hidden checks: the base fails where the bug is the code under test.
        # version-tests is the exception: the function is already right, the test file is not.
        git(dest, "checkout", "-q", "main")
        for name, fail_on_base in (
            ("test_chart.py", True),
            ("test_settings.py", True),
            ("test_slug.py", True),
            ("test_exit_codes.py", True),
            ("test_lcm.py", True),
            ("test_versions_hidden.py", False),
        ):
            run_check(dest, FILES / name, expect_ok=not fail_on_base)
        run_check(dest, dest / "tests" / "test_versions.py", expect_ok=False)
        for branch, script, extra in (
            ("fix-chart-json", "test_chart.py", None),
            ("fix-settings-form", "test_settings.py", None),
            ("fix-slug-collapse", "test_slug.py", None),
            ("fix-exit-codes", "test_exit_codes.py", None),
            ("fix-least-multiple", "test_lcm.py", None),
            ("fix-version-tests", "test_versions_hidden.py", "tests/test_versions.py"),
        ):
            git(dest, "checkout", "-q", branch)
            run_check(dest, FILES / script, expect_ok=True)
            if extra:
                run_check(dest, dest / extra, expect_ok=True)
        git(dest, "checkout", "-q", "main")
    except BaseException:
        shutil.rmtree(dest, ignore_errors=True)
        raise
    print(f"routing pilot repo: {dest}")


if __name__ == "__main__":
    main()
