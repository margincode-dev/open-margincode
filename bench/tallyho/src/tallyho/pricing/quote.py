"""A priced quote for an order: subtotal, discounts, tax and total."""

from __future__ import annotations

from dataclasses import dataclass
from datetime import date

from ..models import Order
from ..money import Money, total
from .coupons import CouponBook
from .discounts import apply_discounts
from .tax import TaxTable, default_tax_table


@dataclass(frozen=True)
class Quote:
    subtotal: Money
    discounts: list[tuple[str, Money]]
    discounted: Money
    tax: Money
    total: Money

    def lines(self) -> list[str]:
        out = [f"Subtotal  {self.subtotal}"]
        out += [f"  {code}  -{amount}" for code, amount in self.discounts]
        out += [f"Tax       {self.tax}", f"Total     {self.total}"]
        return out


def quote_order(
    order: Order,
    coupons: CouponBook | None = None,
    taxes: TaxTable | None = None,
    today: date | None = None,
) -> Quote:
    """Price ``order``: coupons first, then tax on the taxable share of what remains."""
    taxes = taxes or default_tax_table()
    today = today or date.today()
    subtotal = order.subtotal
    discounts = (coupons or CouponBook()).discounts_for(
        order.coupon_codes, subtotal, order.customer.tier, today
    )
    discounted, applied = apply_discounts(subtotal, discounts)
    taxable = total(
        (l.subtotal for l in order.lines
         if l.product.taxable and l.product.category not in taxes.exempt_categories),
        order.currency,
    )
    # Discounts reduce the taxable base proportionally.
    if subtotal.minor:
        taxable = Money(taxable.minor * discounted.minor // subtotal.minor, order.currency)
    tax = taxes.tax(taxable, order.customer.region)
    return Quote(subtotal, applied, discounted, tax, discounted + tax)
