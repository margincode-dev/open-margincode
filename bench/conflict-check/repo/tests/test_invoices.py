from datetime import datetime, timezone

from billing.discounts import apply_discount
from billing.invoices import due_date, invoice_total, next_invoice_number
from billing.money import format_amount, to_cents


def test_to_cents():
    assert to_cents("12.5") == 1250
    assert to_cents("3") == 300


def test_format_amount():
    assert format_amount(1250) == "12.50 EUR"


def test_invoice_total_applies_tax_once():
    assert invoice_total([{"price": 1000, "qty": 3}]) == 36.0


def test_discount():
    assert apply_discount([{"price": 1000, "qty": 1}], 0.1) == 9.0


def test_due_date_is_utc_midnight():
    issued = datetime(2026, 3, 1, 15, 30, tzinfo=timezone.utc)
    assert due_date(issued, 14) == datetime(2026, 3, 15, tzinfo=timezone.utc)


def test_invoice_numbers_count_per_year():
    assert next_invoice_number(2026, None) == "2026-0001"
    assert next_invoice_number(2026, "2026-0041") == "2026-0042"
