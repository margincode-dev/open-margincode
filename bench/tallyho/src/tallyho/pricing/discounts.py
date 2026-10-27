"""Discounts applied to an order subtotal.

A discount either takes a percentage off (``PercentDiscount``) or a fixed amount off
(``FixedDiscount``). ``apply_discounts`` combines several of them.
"""

from __future__ import annotations

from dataclasses import dataclass
from decimal import Decimal

from ..errors import ValidationError
from ..money import Money


@dataclass(frozen=True)
class Discount:
    """Base class. ``code`` identifies the discount on receipts."""

    code: str

    def amount_off(self, subtotal: Money) -> Money:  # pragma: no cover - abstract
        raise NotImplementedError


@dataclass(frozen=True)
class PercentDiscount(Discount):
    percent: Decimal = Decimal(0)

    def __post_init__(self) -> None:
        if not Decimal(0) <= Decimal(self.percent) <= Decimal(100):
            raise ValidationError("percent must be between 0 and 100", field="percent")

    def amount_off(self, subtotal: Money) -> Money:
        return subtotal.times(Decimal(self.percent) / 100)


@dataclass(frozen=True)
class FixedDiscount(Discount):
    off: Money = Money(0)

    def amount_off(self, subtotal: Money) -> Money:
        return self.off if self.off.minor <= subtotal.minor else subtotal


def apply_discounts(subtotal: Money, discounts: list[Discount]) -> tuple[Money, list[tuple[str, Money]]]:
    """Apply ``discounts`` to ``subtotal``.

    Returns the discounted subtotal and one ``(code, amount_off)`` entry per discount,
    in the order given. Percentages are added up and taken off the subtotal together;
    fixed discounts are then taken off what remains. The result never goes below zero.
    """
    applied: list[tuple[str, Money]] = []
    percent_total = Decimal(0)
    for d in discounts:
        if isinstance(d, PercentDiscount):
            percent_total += Decimal(d.percent)
            applied.append((d.code, subtotal.times(Decimal(d.percent) / 100)))
    remaining = subtotal - subtotal.times(min(percent_total, Decimal(100)) / 100)
    for d in discounts:
        if isinstance(d, FixedDiscount):
            off = d.amount_off(remaining)
            remaining = remaining - off
            applied.append((d.code, off))
    if remaining.minor < 0:
        remaining = Money.zero(subtotal.currency)
    return remaining, applied
