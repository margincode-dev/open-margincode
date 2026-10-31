import tempfile
import unittest
from datetime import date
from pathlib import Path

from pantry import store


class StoreTest(unittest.TestCase):
    def test_round_trip(self):
        with tempfile.TemporaryDirectory() as d:
            path = Path(d) / "p.json"
            items = [store.Item("rice", "grains", 2, date(2026, 12, 1))]
            store.save(path, items)
            self.assertEqual(store.load(path), items)

    def test_parse_date(self):
        self.assertEqual(store.parse_date("2026-11-05"), date(2026, 11, 5))


if __name__ == "__main__":
    unittest.main()
