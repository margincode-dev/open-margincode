"""Sales tax by region."""

from __future__ import annotations

from dataclasses import dataclass, field
from decimal import Decimal

from ..money import Money


@dataclass
class TaxTable:
    """Tax rates in percent by region code (``"US-CA"``). ``default`` covers the rest."""

    rates: dict[str, Decimal] = field(default_factory=dict)
    default: Decimal = Decimal(0)
    exempt_categories: frozenset[str] = frozenset({"food", "books"})

    def rate(self, region: str) -> Decimal:
        if region in self.rates:
            return self.rates[region]
        country = region.split("-", 1)[0]
        return self.rates.get(country, self.default)

    def tax(self, amount: Money, region: str) -> Money:
        return amount.times(self.rate(region) / 100)


def default_tax_table() -> TaxTable:
    return TaxTable(
        rates={
            "US-CA": Decimal("7.25"),
            "US-NY": Decimal("4"),
            "US-TX": Decimal("6.25"),
            "US-OR": Decimal(0),
            "GB": Decimal(20),
            "DE": Decimal(19),
        }
    )
