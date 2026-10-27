"""A JSON-file backend: one file per collection under a directory."""

from __future__ import annotations

import json
import os
import tempfile
from pathlib import Path

from ..errors import NotFound, ValidationError
from .base import Page, RecordStore


class JsonFileStore(RecordStore):
    def __init__(self, root: str | os.PathLike, max_page_size: int = 100) -> None:
        self.root = Path(root)
        self.root.mkdir(parents=True, exist_ok=True)
        self.max_page_size = max_page_size

    def _path(self, collection: str) -> Path:
        if not collection.isidentifier():
            raise ValidationError(f"bad collection name {collection!r}", field="collection")
        return self.root / f"{collection}.json"

    def _load(self, collection: str) -> dict[str, dict]:
        path = self._path(collection)
        if not path.exists():
            return {}
        with path.open(encoding="utf-8") as f:
            return json.load(f)

    def _save(self, collection: str, data: dict[str, dict]) -> None:
        path = self._path(collection)
        fd, tmp = tempfile.mkstemp(dir=self.root, suffix=".tmp")
        with os.fdopen(fd, "w", encoding="utf-8") as f:
            json.dump(data, f, sort_keys=True, indent=1)
        os.replace(tmp, path)

    def put(self, collection: str, record: dict) -> None:
        rid = record.get("id")
        if not isinstance(rid, str) or not rid:
            raise ValidationError("record needs a string id", field="id")
        data = self._load(collection)
        data[rid] = record
        self._save(collection, data)

    def put_many(self, collection: str, records: list[dict]) -> None:
        data = self._load(collection)
        for r in records:
            rid = r.get("id")
            if not isinstance(rid, str) or not rid:
                raise ValidationError("record needs a string id", field="id")
            data[rid] = r
        self._save(collection, data)

    def get(self, collection: str, record_id: str) -> dict:
        try:
            return self._load(collection)[record_id]
        except KeyError:
            raise NotFound(f"{collection}/{record_id}") from None

    def delete(self, collection: str, record_id: str) -> None:
        data = self._load(collection)
        if data.pop(record_id, None) is not None:
            self._save(collection, data)

    def list_page(self, collection: str, cursor: str | None = None, limit: int = 50) -> Page:
        data = self._load(collection)
        keys = sorted(k for k in data if cursor is None or k > cursor)
        size = max(1, min(limit, self.max_page_size))
        chunk = keys[:size]
        return Page([data[k] for k in chunk], chunk[-1] if len(keys) > size else None)
