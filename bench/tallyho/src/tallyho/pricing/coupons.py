"""Coupon codes and the discounts they grant."""

from __future__ import annotations

from dataclasses import dataclass
from datetime import date
from decimal import Decimal

from ..errors import ValidationError
from ..money import Money
from .discounts import Discount, FixedDiscount, PercentDiscount


@dataclass(frozen=True)
class Coupon:
    code: str
    kind: str  # "percent" or "fixed"
    value: str
    expires: date | None = None
    min_subtotal: Money | None = None
    tiers: tuple[str, ...] = ()

    def discount(self, currency: str = "USD") -> Discount:
        if self.kind == "percent":
            return PercentDiscount(self.code, Decimal(self.value))
        if self.kind == "fixed":
            return FixedDiscount(self.code, Money.of(self.value, currency))
        raise ValidationError(f"unknown coupon kind {self.kind!r}", field="kind")

    def usable(self, subtotal: Money, tier: str, today: date) -> bool:
        if self.expires is not None and today > self.expires:
            return False
        if self.min_subtotal is not None and subtotal < self.min_subtotal:
            return False
        if self.tiers and tier not in self.tiers:
            return False
        return True


class CouponBook:
    """The coupons a shop accepts, by code (case-insensitive)."""

    def __init__(self, coupons: list[Coupon] | None = None) -> None:
        self._by_code = {c.code.upper(): c for c in coupons or []}

    def add(self, coupon: Coupon) -> None:
        self._by_code[coupon.code.upper()] = coupon

    def get(self, code: str) -> Coupon | None:
        return self._by_code.get(code.strip().upper())

    def discounts_for(self, codes: list[str], subtotal: Money, tier: str, today: date) -> list[Discount]:
        """Discounts of the usable coupons among ``codes``; unknown or unusable codes are skipped."""
        out: list[Discount] = []
        seen: set[str] = set()
        for code in codes:
            c = self.get(code)
            if c is None or c.code.upper() in seen:
                continue
            seen.add(c.code.upper())
            if c.usable(subtotal, tier, today):
                out.append(c.discount(subtotal.currency))
        return out
