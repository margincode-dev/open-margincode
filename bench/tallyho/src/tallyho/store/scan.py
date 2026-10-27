"""Whole-collection scans on top of ``RecordStore.list_page``."""

from __future__ import annotations

from collections.abc import Callable, Iterator

from .base import RecordStore


def iter_records(
    store: RecordStore,
    collection: str,
    where: Callable[[dict], bool] | None = None,
    page_size: int = 100,
) -> Iterator[dict]:
    """Every record of ``collection`` in id order, optionally filtered by ``where``."""
    page = store.list_page(collection, None, page_size)
    for record in page.records:
        if where is None or where(record):
            yield record


def count_records(store: RecordStore, collection: str, where: Callable[[dict], bool] | None = None) -> int:
    return sum(1 for _ in iter_records(store, collection, where))
