"""A tiny synchronous event bus for order lifecycle events."""

from __future__ import annotations

from collections import defaultdict
from collections.abc import Callable

Handler = Callable[[str, dict], None]


class EventBus:
    def __init__(self) -> None:
        self._handlers: dict[str, list[Handler]] = defaultdict(list)
        self.history: list[tuple[str, dict]] = []

    def subscribe(self, event: str, handler: Handler) -> None:
        self._handlers[event].append(handler)

    def publish(self, event: str, payload: dict) -> None:
        self.history.append((event, payload))
        for h in self._handlers.get(event, []) + self._handlers.get("*", []):
            h(event, payload)
