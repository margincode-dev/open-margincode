def to_cents(text):
    """Parse an amount a person typed, such as "12.50", into cents."""
    whole, _, frac = text.strip().partition(".")
    frac = (frac + "00")[:2]
    return int(whole or "0") * 100 + int(frac)


def format_amount(cents, currency="EUR"):
    return f"{cents / 100:.2f} {currency}"
