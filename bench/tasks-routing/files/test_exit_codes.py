"""Hidden check: README lists each exit constant's value on its own line."""

import re
from pathlib import Path

cli = Path("cli.py").read_text(encoding="utf-8")
readme = Path("README.md").read_text(encoding="utf-8")
codes = dict(re.findall(r"^(EXIT_\w+)\s*=\s*(\d+)\s*$", cli, re.M))
lines = set(readme.splitlines())
for name, key in (
    ("success", "EXIT_OK"),
    ("usage", "EXIT_USAGE"),
    ("missing", "EXIT_MISSING"),
):
    want = f"{name}: {codes[key]}"
    if want not in lines:
        raise SystemExit(f"missing {want!r}")
print("ok")
