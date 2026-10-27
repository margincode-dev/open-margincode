from datetime import date
from decimal import Decimal

from tallyho.money import Money
from tallyho.pricing import FixedDiscount, PercentDiscount, apply_discounts, quote_order
from tallyho.pricing.coupons import Coupon, CouponBook


def test_single_percent():
    rest, applied = apply_discounts(Money(1000), [PercentDiscount("TEN", Decimal(10))])
    assert rest.minor == 900
    assert applied == [("TEN", Money(100))]


def test_fixed_never_below_zero():
    rest, _ = apply_discounts(Money(500), [FixedDiscount("BIG", Money(900))])
    assert rest.minor == 0


def test_quote_taxes_only_taxable(order):
    q = quote_order(order, today=date(2026, 1, 1))
    # mugs are taxable (25.00 at 7.25 %), books are exempt
    assert q.tax.minor == 181
    assert q.total.minor == 4500 + 181


def test_coupon_book_skips_unknown(order):
    book = CouponBook([Coupon("SAVE5", "fixed", "5.00")])
    order.coupon_codes = ["nope", "save5"]
    q = quote_order(order, book, today=date(2026, 1, 1))
    assert q.discounts == [("SAVE5", Money(500))]
