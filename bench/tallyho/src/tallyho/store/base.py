"""The store interface every backend implements."""

from __future__ import annotations

from abc import ABC, abstractmethod
from dataclasses import dataclass, field


@dataclass
class Page:
    """One page of a listing. ``next_cursor`` is ``None`` on the last page."""

    records: list[dict] = field(default_factory=list)
    next_cursor: str | None = None


class RecordStore(ABC):
    """A collection-scoped store of JSON-able dict records keyed by ``id``."""

    #: Largest page a backend returns, whatever ``limit`` asks for.
    max_page_size: int = 100

    @abstractmethod
    def put(self, collection: str, record: dict) -> None:
        """Insert or replace ``record`` (must have a string ``id``)."""

    @abstractmethod
    def get(self, collection: str, record_id: str) -> dict:
        """The record, or raise ``NotFound``."""

    @abstractmethod
    def delete(self, collection: str, record_id: str) -> None:
        """Remove the record; missing records are ignored."""

    @abstractmethod
    def list_page(self, collection: str, cursor: str | None = None, limit: int = 50) -> Page:
        """Records in id order, starting after ``cursor``, at most ``min(limit, max_page_size)``."""

    def put_many(self, collection: str, records: list[dict]) -> None:
        for r in records:
            self.put(collection, r)
