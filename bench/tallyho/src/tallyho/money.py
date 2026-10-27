"""Fixed-point money amounts. Amounts are stored in minor units (cents)."""

from __future__ import annotations

from dataclasses import dataclass
from decimal import ROUND_HALF_EVEN, Decimal

from .errors import CurrencyMismatch, ValidationError

#: Minor units per major unit, by ISO currency code.
MINOR_UNITS = {"USD": 2, "EUR": 2, "GBP": 2, "JPY": 0, "KWD": 3}


def _exponent(currency: str) -> int:
    try:
        return MINOR_UNITS[currency]
    except KeyError:
        raise ValidationError(f"unknown currency {currency!r}", field="currency") from None


@dataclass(frozen=True, order=True)
class Money:
    """An amount of money in minor units of ``currency``."""

    minor: int
    currency: str = "USD"

    @classmethod
    def of(cls, amount: str | int | Decimal, currency: str = "USD") -> Money:
        """``Money.of("12.34")`` is 1234 cents. Rounds half to even."""
        exp = _exponent(currency)
        value = Decimal(str(amount)).scaleb(exp).quantize(Decimal(1), rounding=ROUND_HALF_EVEN)
        return cls(int(value), currency)

    @classmethod
    def zero(cls, currency: str = "USD") -> Money:
        return cls(0, currency)

    def _check(self, other: Money) -> None:
        if not isinstance(other, Money):
            raise TypeError(f"cannot combine Money and {type(other).__name__}")
        if other.currency != self.currency:
            raise CurrencyMismatch(f"{self.currency} vs {other.currency}")

    def __add__(self, other: Money) -> Money:
        self._check(other)
        return Money(self.minor + other.minor, self.currency)

    def __sub__(self, other: Money) -> Money:
        self._check(other)
        return Money(self.minor - other.minor, self.currency)

    def __neg__(self) -> Money:
        return Money(-self.minor, self.currency)

    def times(self, factor: int | Decimal | str) -> Money:
        """Multiply by ``factor``, rounding half to even to whole minor units."""
        value = (Decimal(self.minor) * Decimal(str(factor))).quantize(
            Decimal(1), rounding=ROUND_HALF_EVEN
        )
        return Money(int(value), self.currency)

    def allocate(self, parts: int) -> list[Money]:
        """Split into ``parts`` amounts that sum exactly to this one."""
        if parts <= 0:
            raise ValidationError("parts must be positive", field="parts")
        base, rest = divmod(self.minor, parts)
        return [Money(base + (1 if i < rest else 0), self.currency) for i in range(parts)]

    @property
    def amount(self) -> Decimal:
        return Decimal(self.minor).scaleb(-_exponent(self.currency))

    def is_zero(self) -> bool:
        return self.minor == 0

    def format(self) -> str:
        """``"12.34 USD"``."""
        exp = _exponent(self.currency)
        return f"{self.amount:.{exp}f} {self.currency}"

    def __str__(self) -> str:
        return self.format()


def total(amounts, currency: str = "USD") -> Money:
    """Sum of ``amounts``; ``Money.zero(currency)`` when empty."""
    result = Money.zero(currency)
    for a in amounts:
        result = result + a
    return result
