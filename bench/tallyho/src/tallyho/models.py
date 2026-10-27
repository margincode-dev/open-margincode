"""Domain models: products, customers, order lines and orders."""

from __future__ import annotations

from dataclasses import dataclass, field
from datetime import date
from enum import Enum

from .errors import ValidationError
from .money import Money, total
from .validate import check_email, check_sku, require_positive


class OrderStatus(str, Enum):
    DRAFT = "draft"
    PLACED = "placed"
    PAID = "paid"
    SHIPPED = "shipped"
    CANCELLED = "cancelled"


@dataclass(frozen=True)
class Product:
    sku: str
    name: str
    price: Money
    category: str = "general"
    taxable: bool = True

    def __post_init__(self) -> None:
        check_sku(self.sku)
        if not self.name.strip():
            raise ValidationError("name must not be empty", field="name")
        if self.price.minor < 0:
            raise ValidationError("price must not be negative", field="price")


@dataclass(frozen=True)
class Customer:
    id: str
    name: str
    email: str
    region: str = "US-CA"
    tier: str = "standard"

    def __post_init__(self) -> None:
        check_email(self.email)


@dataclass(frozen=True)
class OrderLine:
    product: Product
    quantity: int

    def __post_init__(self) -> None:
        require_positive(self.quantity, "quantity")

    @property
    def subtotal(self) -> Money:
        return self.product.price.times(self.quantity)


@dataclass
class Order:
    id: str
    customer: Customer
    lines: list[OrderLine] = field(default_factory=list)
    status: OrderStatus = OrderStatus.DRAFT
    placed_on: date | None = None
    coupon_codes: list[str] = field(default_factory=list)

    @property
    def currency(self) -> str:
        return self.lines[0].product.price.currency if self.lines else "USD"

    def add(self, product: Product, quantity: int = 1) -> None:
        """Add ``quantity`` of ``product``, merging with an existing line for the same SKU."""
        if self.status is not OrderStatus.DRAFT:
            raise ValidationError("only draft orders can change", field="status")
        for i, line in enumerate(self.lines):
            if line.product.sku == product.sku:
                self.lines[i] = OrderLine(product, line.quantity + quantity)
                return
        self.lines.append(OrderLine(product, quantity))

    def remove(self, sku: str) -> None:
        if self.status is not OrderStatus.DRAFT:
            raise ValidationError("only draft orders can change", field="status")
        self.lines = [line for line in self.lines if line.product.sku != sku]

    @property
    def subtotal(self) -> Money:
        return total((line.subtotal for line in self.lines), self.currency)

    def item_count(self) -> int:
        return sum(line.quantity for line in self.lines)

    def to_dict(self) -> dict:
        return {
            "id": self.id,
            "customer": self.customer.id,
            "status": self.status.value,
            "placed_on": self.placed_on.isoformat() if self.placed_on else None,
            "coupons": list(self.coupon_codes),
            "lines": [
                {"sku": l.product.sku, "quantity": l.quantity, "price": l.product.price.minor}
                for l in self.lines
            ],
        }
