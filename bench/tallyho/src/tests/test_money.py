from decimal import Decimal

import pytest

from tallyho.errors import CurrencyMismatch
from tallyho.money import Money, total


def test_of_and_format():
    assert Money.of("12.345").minor == 1234
    assert Money.of("1", "JPY").format() == "1 JPY"
    assert str(Money.of("3.5")) == "3.50 USD"


def test_arithmetic_and_mismatch():
    assert Money.of("1.10") + Money.of("2.20") == Money.of("3.30")
    with pytest.raises(CurrencyMismatch):
        Money.of("1") + Money.of("1", "EUR")


def test_allocate_sums_exactly():
    parts = Money(100).allocate(3)
    assert [p.minor for p in parts] == [34, 33, 33]
    assert total(parts).minor == 100


def test_times_rounds_half_even():
    assert Money(5).times(Decimal("0.5")).minor == 2
