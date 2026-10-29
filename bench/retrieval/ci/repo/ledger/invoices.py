"""Invoices: building them, discounting them and printing their numbers."""
from dataclasses import dataclass, field
from decimal import Decimal

from ledger.taxes import compute_tax, round_money


@dataclass
class Line:
    description: str
    quantity: int
    unit_price: Decimal

    def amount(self):
        return self.unit_price * self.quantity


@dataclass
class Invoice:
    number: str
    customer: str
    lines: list = field(default_factory=list)
    discount_pct: Decimal = Decimal(0)


def format_invoice_number(year, sequence):
    return "INV-%d-%05d" % (year, sequence)


def invoice_subtotal(invoice):
    return sum((line.amount() for line in invoice.lines), Decimal(0))


def apply_discount(total, pct):
    # percentage discount on the whole invoice
    return total * (Decimal(100) - pct) / Decimal(100)


def invoice_total(invoice, region):
    subtotal = invoice_subtotal(invoice)
    discounted = apply_discount(subtotal, invoice.discount_pct)
    return round_money(discounted + compute_tax(discounted, region))


def build_invoice(customer, lines, year, sequence):
    return Invoice(
        number=format_invoice_number(year, sequence),
        customer=customer,
        lines=list(lines),
    )
