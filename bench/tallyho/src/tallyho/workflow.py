"""Order lifecycle: place, pay, ship, cancel."""

from __future__ import annotations

from datetime import date

from .errors import ValidationError
from .events import EventBus
from .ledger import Ledger
from .models import Order, OrderStatus
from .pricing import quote_order
from .pricing.coupons import CouponBook

_ALLOWED = {
    OrderStatus.DRAFT: {OrderStatus.PLACED, OrderStatus.CANCELLED},
    OrderStatus.PLACED: {OrderStatus.PAID, OrderStatus.CANCELLED},
    OrderStatus.PAID: {OrderStatus.SHIPPED},
    OrderStatus.SHIPPED: set(),
    OrderStatus.CANCELLED: set(),
}


class Workflow:
    def __init__(self, ledger: Ledger, bus: EventBus | None = None, coupons: CouponBook | None = None) -> None:
        self.ledger = ledger
        self.bus = bus or EventBus()
        self.coupons = coupons or CouponBook()

    def _move(self, order: Order, to: OrderStatus) -> None:
        if to not in _ALLOWED[order.status]:
            raise ValidationError(f"cannot go from {order.status.value} to {to.value}", field="status")
        order.status = to
        self.bus.publish(f"order.{to.value}", {"id": order.id})

    def place(self, order: Order, on: date) -> None:
        if not order.lines:
            raise ValidationError("an order needs at least one line", field="lines")
        order.placed_on = on
        self._move(order, OrderStatus.PLACED)

    def pay(self, order: Order, on: date) -> None:
        q = quote_order(order, self.coupons, today=on)
        self._move(order, OrderStatus.PAID)
        self.ledger.record_sale(on, order.id, q)

    def ship(self, order: Order) -> None:
        self._move(order, OrderStatus.SHIPPED)

    def cancel(self, order: Order) -> None:
        self._move(order, OrderStatus.CANCELLED)
