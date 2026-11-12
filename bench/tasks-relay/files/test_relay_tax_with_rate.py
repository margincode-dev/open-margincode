from decimal import Decimal

from tallyho.money import Money
from tallyho.pricing.tax import TaxTable, default_tax_table


def test_the_copy_has_the_new_rate_and_the_table_keeps_its_own():
    table = default_tax_table()
    trial = table.with_rate("US-CA", Decimal("8"))
    assert trial.rate("US-CA") == Decimal("8")
    assert trial.tax(Money.of("100.00"), "US-CA") == Money.of("8.00")
    assert table.rate("US-CA") == Decimal("7.25")
    assert table.tax(Money.of("100.00"), "US-CA") == Money.of("7.25")


def test_a_region_the_table_did_not_list():
    table = default_tax_table()
    trial = table.with_rate("US-WA", Decimal("6.5"))
    assert trial.rate("US-WA") == Decimal("6.5")
    assert table.rate("US-WA") == Decimal(0)


def test_the_copy_keeps_everything_else():
    table = TaxTable({"GB": Decimal(20)}, default=Decimal(5), exempt_categories=frozenset({"food"}))
    trial = table.with_rate("DE", Decimal(19))
    assert isinstance(trial, TaxTable)
    assert trial.rate("DE") == Decimal(19)
    assert trial.rate("GB-LND") == Decimal(20)
    assert trial.rate("FR") == Decimal(5)
    assert trial.exempt_categories == frozenset({"food"})
