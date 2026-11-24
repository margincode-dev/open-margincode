from decimal import Decimal


def refund_covers(invoice_amount, refund):
    """Whether the provider's refund (its "amount" is a string such as "12.50") covers
    `invoice_amount`, a string in the same form."""
    return Decimal(refund["amount"]) >= Decimal(invoice_amount)


def refund_total(refunds):
    return sum((Decimal(r["amount"]) for r in refunds), Decimal("0"))
