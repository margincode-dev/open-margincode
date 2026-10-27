"""A double-entry ledger built from paid orders."""

from __future__ import annotations

from collections import defaultdict
from dataclasses import dataclass, field
from datetime import date

from .errors import ValidationError
from .money import Money
from .pricing import Quote


@dataclass(frozen=True)
class Entry:
    on: date
    account: str
    debit: Money
    credit: Money
    memo: str = ""


@dataclass
class Ledger:
    currency: str = "USD"
    entries: list[Entry] = field(default_factory=list)

    def post(self, on: date, memo: str, *legs: tuple[str, Money, Money]) -> None:
        """Post a balanced transaction: legs of ``(account, debit, credit)``."""
        debits = sum(l[1].minor for l in legs)
        credits = sum(l[2].minor for l in legs)
        if debits != credits:
            raise ValidationError(f"unbalanced: {debits} != {credits}", field="legs")
        for account, dr, cr in legs:
            self.entries.append(Entry(on, account, dr, cr, memo))

    def record_sale(self, on: date, order_id: str, quote: Quote) -> None:
        z = Money.zero(self.currency)
        self.post(
            on, f"order {order_id}",
            ("cash", quote.total, z),
            ("revenue", z, quote.discounted),
            ("tax_payable", z, quote.tax),
        )

    def balances(self) -> dict[str, Money]:
        out: dict[str, int] = defaultdict(int)
        for e in self.entries:
            out[e.account] += e.debit.minor - e.credit.minor
        return {k: Money(v, self.currency) for k, v in sorted(out.items())}

    def trial_balance_ok(self) -> bool:
        return sum(m.minor for m in self.balances().values()) == 0
