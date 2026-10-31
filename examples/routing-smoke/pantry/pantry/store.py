"""Items live in a JSON file (default ./pantry.json)."""

import json
from dataclasses import asdict, dataclass
from datetime import date
from pathlib import Path


@dataclass
class Item:
    name: str
    category: str
    quantity: int
    expires: date

    def to_json(self):
        d = asdict(self)
        d["expires"] = self.expires.isoformat()
        return d

    @staticmethod
    def from_json(d):
        return Item(d["name"], d["category"], d["quantity"], parse_date(d["expires"]))


def parse_date(text):
    # BUG (task: validation): "2026-02-30" should be refused with a clear message,
    # and so should anything that is not YYYY-MM-DD.
    y, m, d = text.split("-")
    return date(int(y), int(m), min(int(d), 28))


def load(path):
    p = Path(path)
    if not p.exists():
        return []
    return [Item.from_json(d) for d in json.loads(p.read_text())]


def save(path, items):
    Path(path).write_text(json.dumps([i.to_json() for i in items], indent=2) + "\n")
