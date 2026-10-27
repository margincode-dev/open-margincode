"""In-memory backend, for tests and the CLI's --dry-run."""

from __future__ import annotations

import bisect
import copy

from ..errors import NotFound, ValidationError
from .base import Page, RecordStore


class MemoryStore(RecordStore):
    def __init__(self, max_page_size: int = 100) -> None:
        self.max_page_size = max_page_size
        self._data: dict[str, dict[str, dict]] = {}
        self._keys: dict[str, list[str]] = {}

    def put(self, collection: str, record: dict) -> None:
        rid = record.get("id")
        if not isinstance(rid, str) or not rid:
            raise ValidationError("record needs a string id", field="id")
        coll = self._data.setdefault(collection, {})
        keys = self._keys.setdefault(collection, [])
        if rid not in coll:
            bisect.insort(keys, rid)
        coll[rid] = copy.deepcopy(record)

    def get(self, collection: str, record_id: str) -> dict:
        try:
            return copy.deepcopy(self._data[collection][record_id])
        except KeyError:
            raise NotFound(f"{collection}/{record_id}") from None

    def delete(self, collection: str, record_id: str) -> None:
        coll = self._data.get(collection, {})
        if coll.pop(record_id, None) is not None:
            self._keys[collection].remove(record_id)

    def list_page(self, collection: str, cursor: str | None = None, limit: int = 50) -> Page:
        keys = self._keys.get(collection, [])
        start = bisect.bisect_right(keys, cursor) if cursor else 0
        size = max(1, min(limit, self.max_page_size))
        chunk = keys[start : start + size]
        more = start + size < len(keys)
        records = [copy.deepcopy(self._data[collection][k]) for k in chunk]
        return Page(records, chunk[-1] if more and chunk else None)
