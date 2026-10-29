from tallyho import Money


def test_plain_amounts_use_each_currencys_decimals():
    assert Money.of("12.34").format_plain() == "12.34"
    assert Money.of("1200", "JPY").format_plain() == "1200"
    assert Money.of("1.234", "KWD").format_plain() == "1.234"
    assert Money.of("7", "EUR").format_plain() == "7.00"


def test_negative_amounts_keep_their_sign():
    assert Money(-5, "USD").format_plain() == "-0.05"
