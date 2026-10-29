"""Payments: charging cards, refunding them and retrying failed charges."""
import time
from decimal import Decimal


class PaymentError(Exception):
    pass


def validate_card_number(number):
    digits = [int(c) for c in number if c.isdigit()]
    if len(digits) < 12:
        return False
    checksum = 0
    for i, d in enumerate(reversed(digits)):
        if i % 2 == 1:
            d *= 2
            if d > 9:
                d -= 9
        checksum += d
    return checksum % 10 == 0


def charge_card(gateway, card, amount):
    if not validate_card_number(card):
        raise PaymentError("invalid card number")
    return gateway.capture(card, amount)


def retry_charge(gateway, card, amount, attempts=3, pause=1.0):
    last = None
    for attempt in range(attempts):
        try:
            return charge_card(gateway, card, amount)
        except PaymentError as err:
            last = err
            time.sleep(pause * (attempt + 1))
    raise last


def refund_payment(gateway, payment, amount):
    # a refund may be partial; the gateway rejects anything above what was captured
    if amount <= Decimal(0):
        raise PaymentError("nothing to refund")
    return gateway.refund(payment.id, amount)
