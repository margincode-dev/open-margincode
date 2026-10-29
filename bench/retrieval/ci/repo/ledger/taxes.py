"""Sales tax lookups and money rounding."""
from decimal import ROUND_HALF_UP, Decimal

RATES = {
    "CA": Decimal("0.0725"),
    "NY": Decimal("0.04"),
    "TX": Decimal("0.0625"),
    "OR": Decimal("0"),
}


def tax_rate_for_region(region):
    return RATES.get(region.upper(), Decimal("0.05"))


def compute_tax(amount, region):
    return amount * tax_rate_for_region(region)


def round_money(value):
    return value.quantize(Decimal("0.01"), rounding=ROUND_HALF_UP)
