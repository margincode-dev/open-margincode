from decimal import Decimal

from tallyho.money import Money
from tallyho.pricing import FixedDiscount, PercentDiscount, apply_discounts


def test_percent_discounts_compound():
    rest, applied = apply_discounts(
        Money(10000), [PercentDiscount("TEN", Decimal(10)), PercentDiscount("TWENTY", Decimal(20))]
    )
    # 10 % of 100.00, then 20 % of the remaining 90.00
    assert rest.minor == 7200
    assert applied == [("TEN", Money(1000)), ("TWENTY", Money(1800))]


def test_two_fifty_percent_never_free():
    rest, _ = apply_discounts(
        Money(1000), [PercentDiscount("A", Decimal(50)), PercentDiscount("B", Decimal(50))]
    )
    assert rest.minor == 250


def test_fixed_applies_after_percent_in_given_order():
    rest, applied = apply_discounts(
        Money(10000), [FixedDiscount("FIVE", Money(500)), PercentDiscount("TEN", Decimal(10))]
    )
    assert rest.minor == 8500
    # entries keep the order the discounts were given in
    assert applied == [("FIVE", Money(500)), ("TEN", Money(1000))]


def test_applied_amounts_sum_to_reduction():
    subtotal = Money(9999)
    rest, applied = apply_discounts(
        subtotal,
        [PercentDiscount("A", Decimal("12.5")), FixedDiscount("B", Money(100)), PercentDiscount("C", Decimal(5))],
    )
    assert sum(a.minor for _, a in applied) == subtotal.minor - rest.minor


def test_fixed_bigger_than_rest_clamps_to_zero():
    rest, applied = apply_discounts(
        Money(1000), [PercentDiscount("HALF", Decimal(50)), FixedDiscount("BIG", Money(900))]
    )
    assert rest.minor == 0
    assert applied == [("HALF", Money(500)), ("BIG", Money(500))]
