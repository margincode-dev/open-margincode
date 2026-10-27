"""Persisting orders, products and customers in a ``RecordStore``."""

from __future__ import annotations

from datetime import date

from .models import Customer, Order, OrderLine, OrderStatus, Product
from .money import Money
from .store import RecordStore, iter_records

PRODUCTS, CUSTOMERS, ORDERS = "products", "customers", "orders"


def product_to_record(p: Product) -> dict:
    return {"id": p.sku, "name": p.name, "price": p.price.minor, "currency": p.price.currency,
            "category": p.category, "taxable": p.taxable}


def product_from_record(r: dict) -> Product:
    return Product(r["id"], r["name"], Money(r["price"], r.get("currency", "USD")),
                   r.get("category", "general"), r.get("taxable", True))


def customer_to_record(c: Customer) -> dict:
    return {"id": c.id, "name": c.name, "email": c.email, "region": c.region, "tier": c.tier}


def customer_from_record(r: dict) -> Customer:
    return Customer(r["id"], r["name"], r["email"], r.get("region", "US-CA"), r.get("tier", "standard"))


class Repository:
    """Typed access to the shop's records."""

    def __init__(self, store: RecordStore) -> None:
        self.store = store

    def save_product(self, p: Product) -> None:
        self.store.put(PRODUCTS, product_to_record(p))

    def product(self, sku: str) -> Product:
        return product_from_record(self.store.get(PRODUCTS, sku))

    def products(self, category: str | None = None) -> list[Product]:
        where = (lambda r: r.get("category") == category) if category else None
        return [product_from_record(r) for r in iter_records(self.store, PRODUCTS, where)]

    def save_customer(self, c: Customer) -> None:
        self.store.put(CUSTOMERS, customer_to_record(c))

    def customer(self, cid: str) -> Customer:
        return customer_from_record(self.store.get(CUSTOMERS, cid))

    def save_order(self, o: Order) -> None:
        self.store.put(ORDERS, o.to_dict())

    def order(self, oid: str) -> Order:
        r = self.store.get(ORDERS, oid)
        return self._order_from_record(r)

    def _order_from_record(self, r: dict) -> Order:
        lines = []
        for l in r["lines"]:
            p = self.product(l["sku"])
            lines.append(OrderLine(p, l["quantity"]))
        return Order(
            r["id"], self.customer(r["customer"]), lines, OrderStatus(r["status"]),
            date.fromisoformat(r["placed_on"]) if r.get("placed_on") else None,
            list(r.get("coupons", [])),
        )

    def orders(self, status: OrderStatus | None = None) -> list[Order]:
        where = (lambda r: r["status"] == status.value) if status else None
        return [self._order_from_record(r) for r in iter_records(self.store, ORDERS, where)]
