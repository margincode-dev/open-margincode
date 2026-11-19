from datetime import date, timedelta

from tallyho.money import Money
from tallyho.pricing.coupons import Coupon

END = date(2026, 3, 10)


def spring(expires=END):
    return Coupon("SPRING", "percent", "10", expires=expires)


def test_on_its_last_day_one_day_is_left():
    assert spring().days_left(END) == 1


def test_a_few_days_before():
    assert spring().days_left(date(2026, 3, 8)) == 3


def test_after_it_expired():
    assert spring().days_left(date(2026, 3, 11)) == 0
    assert spring().days_left(date(2026, 4, 1)) == 0


def test_a_coupon_that_never_expires():
    assert spring(None).days_left(END) is None


def test_days_left_agrees_with_usable():
    c = spring()
    for offset in range(-3, 4):
        day = END + timedelta(days=offset)
        assert (c.days_left(day) > 0) == c.usable(Money.of("50.00"), "standard", day)
