from tallyho.pricing.coupons import Coupon, CouponBook


def test_remove_accepts_codes_as_staff_type_them():
    book = CouponBook([Coupon("SAVE10", "percent", "10"), Coupon("Ship5", "fixed", "5")])
    book.remove(" save10 ")
    assert book.get("SAVE10") is None
    book.remove("SHIP5")
    assert book.get("ship5") is None


def test_removing_an_unknown_code_does_nothing():
    book = CouponBook([Coupon("SAVE10", "percent", "10")])
    book.remove("NOPE")
    assert book.get("save10") is not None
