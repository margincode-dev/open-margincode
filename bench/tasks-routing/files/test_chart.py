"""Hidden check: assets/q3.json matches the chart's known series."""

import json
from pathlib import Path

got = json.loads(Path("assets/q3.json").read_text(encoding="utf-8"))
assert got == {
    "quarter": "Q3",
    "units": {"apples": 12, "pears": 7, "plums": 15},
}
print("ok")
