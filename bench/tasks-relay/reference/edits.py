"""Reference fixes and dead ends for the relay pairs, as (path, old, new) text edits on tallyho
`main`. `check.py` proves each pair's hidden test passes on `good` and fails on `dead`; `dead`
is the obvious approach the pair's note warns about (traps) or the guess its decision rules out.
"""

PAIRS = {
    "quote-split": {
        "good": [
            ("tallyho/pricing/quote.py", "    def lines(self) -> list[str]:",
             "    def split(self, parts: int) -> list[Money]:\n"
             "        return self.total.allocate(parts)\n\n"
             "    def lines(self) -> list[str]:"),
        ],
        "dead": [
            ("tallyho/pricing/quote.py", "from datetime import date\n",
             "from datetime import date\nfrom decimal import Decimal\n\nfrom ..errors import ValidationError\n"),
            ("tallyho/pricing/quote.py", "    def lines(self) -> list[str]:",
             "    def split(self, parts: int) -> list[Money]:\n"
             "        if parts < 1:\n"
             "            raise ValidationError('parts must be positive', field='parts')\n"
             "        return [self.total.times(Decimal(1) / parts) for _ in range(parts)]\n\n"
             "    def lines(self) -> list[str]:"),
        ],
    },
    "store-count": {
        "good": [
            ("tallyho/store/base.py", "    def put_many(",
             "    def count(self, collection: str) -> int:\n"
             "        raise NotImplementedError\n\n"
             "    def put_many("),
            ("tallyho/store/memory.py", "    def list_page(",
             "    def count(self, collection: str) -> int:\n"
             "        return len(self._keys.get(collection, []))\n\n"
             "    def list_page("),
            ("tallyho/store/jsonfile.py", "    def list_page(",
             "    def count(self, collection: str) -> int:\n"
             "        return len(self._load(collection))\n\n"
             "    def list_page("),
        ],
        "dead": [
            ("tallyho/store/base.py", "    def put_many(",
             "    def count(self, collection: str) -> int:\n"
             "        return len(self.list_page(collection, None, 10**9).records)\n\n"
             "    def put_many("),
        ],
    },
    "line-taxes": {
        "good": [
            ("tallyho/pricing/quote.py", "    total: Money\n",
             "    total: Money\n    line_taxes: list[tuple[str, Money]] = ()\n"),
            ("tallyho/pricing/quote.py",
             "    tax = taxes.tax(taxable, order.customer.region)\n"
             "    return Quote(subtotal, applied, discounted, tax, discounted + tax)",
             "    tax = taxes.tax(taxable, order.customer.region)\n"
             "    lines = [l for l in order.lines\n"
             "             if l.product.taxable and l.product.category not in taxes.exempt_categories]\n"
             "    base = sum(l.subtotal.minor for l in lines)\n"
             "    shares = [(tax.minor * l.subtotal.minor // base if base else 0) for l in lines]\n"
             "    rest = tax.minor - sum(shares)\n"
             "    order_ = sorted(range(len(lines)), key=lambda i: -((tax.minor * lines[i].subtotal.minor) % base if base else 0))\n"
             "    for i in order_[:rest]:\n"
             "        shares[i] += 1\n"
             "    line_taxes = [(l.product.sku, Money(s, order.currency)) for l, s in zip(lines, shares)]\n"
             "    return Quote(subtotal, applied, discounted, tax, discounted + tax, line_taxes)"),
        ],
        "dead": [
            ("tallyho/pricing/quote.py", "    total: Money\n",
             "    total: Money\n    line_taxes: list[tuple[str, Money]] = ()\n"),
            ("tallyho/pricing/quote.py",
             "    tax = taxes.tax(taxable, order.customer.region)\n"
             "    return Quote(subtotal, applied, discounted, tax, discounted + tax)",
             "    tax = taxes.tax(taxable, order.customer.region)\n"
             "    line_taxes = [(l.product.sku, taxes.tax(l.subtotal, order.customer.region))\n"
             "                  for l in order.lines\n"
             "                  if l.product.taxable and l.product.category not in taxes.exempt_categories]\n"
             "    return Quote(subtotal, applied, discounted, tax, discounted + tax, line_taxes)"),
        ],
    },
    "coupon-remove": {
        "good": [
            ("tallyho/pricing/coupons.py", "    def get(self, code: str)",
             "    def remove(self, code: str) -> None:\n"
             "        self._by_code.pop(code.strip().upper(), None)\n\n"
             "    def get(self, code: str)"),
        ],
        "dead": [
            ("tallyho/pricing/coupons.py", "    def get(self, code: str)",
             "    def remove(self, code: str) -> None:\n"
             "        self._by_code.pop(code, None)\n\n"
             "    def get(self, code: str)"),
        ],
    },
    "money-plain": {
        "good": [
            ("tallyho/money.py", "    def __str__(self) -> str:",
             "    def format_plain(self) -> str:\n"
             "        return f\"{self.amount:.{_exponent(self.currency)}f}\"\n\n"
             "    def __str__(self) -> str:"),
        ],
        "dead": [
            ("tallyho/money.py", "    def __str__(self) -> str:",
             "    def format_plain(self) -> str:\n"
             "        return f\"{self.minor / 100:.2f}\"\n\n"
             "    def __str__(self) -> str:"),
        ],
    },
    "customer-region": {
        "good": [
            ("tallyho/validate.py",
             "        raise ValidationError(f\"invalid region {region!r}\", field=\"region\")",
             "        raise ValidationError(\"invalid region\", field=\"region\")"),
            ("tallyho/models.py", "from .validate import check_email, check_sku, require_positive",
             "from .validate import check_email, check_region, check_sku, require_positive"),
            ("tallyho/models.py", "        check_email(self.email)\n",
             "        check_email(self.email)\n        check_region(self.region)\n"),
        ],
        "dead": [
            ("tallyho/models.py", "from .validate import check_email, check_sku, require_positive",
             "from .validate import check_email, check_region, check_sku, require_positive"),
            ("tallyho/models.py", "        check_email(self.email)\n",
             "        check_email(self.email)\n        check_region(self.region)\n"),
        ],
    },
    "refund-event": {
        "good": [
            ("tallyho/models.py", '    CANCELLED = "cancelled"\n',
             '    CANCELLED = "cancelled"\n    REFUNDED = "refunded"\n'),
            ("tallyho/workflow.py", "    OrderStatus.PAID: {OrderStatus.SHIPPED},",
             "    OrderStatus.PAID: {OrderStatus.SHIPPED, OrderStatus.REFUNDED},\n"
             "    OrderStatus.REFUNDED: set(),"),
            ("tallyho/workflow.py", "    def ship(self, order: Order) -> None:",
             "    def refund(self, order: Order, on: date) -> None:\n"
             "        if order.status is not OrderStatus.PAID:\n"
             "            raise ValidationError('only paid orders can be refunded', field='status')\n"
             "        q = quote_order(order, self.coupons, today=on)\n"
             "        order.status = OrderStatus.REFUNDED\n"
             "        self.bus.publish('order.refunded', {'id': order.id, 'amount_minor': q.total.minor,\n"
             "                                            'currency': q.total.currency})\n\n"
             "    def ship(self, order: Order) -> None:"),
        ],
        "dead": [
            ("tallyho/models.py", '    CANCELLED = "cancelled"\n',
             '    CANCELLED = "cancelled"\n    REFUNDED = "refunded"\n'),
            ("tallyho/workflow.py", "    OrderStatus.PAID: {OrderStatus.SHIPPED},",
             "    OrderStatus.PAID: {OrderStatus.SHIPPED, OrderStatus.REFUNDED},\n"
             "    OrderStatus.REFUNDED: set(),"),
            ("tallyho/workflow.py", "    def ship(self, order: Order) -> None:",
             "    def refund(self, order: Order, on: date) -> None:\n"
             "        if order.status is not OrderStatus.PAID:\n"
             "            raise ValidationError('only paid orders can be refunded', field='status')\n"
             "        q = quote_order(order, self.coupons, today=on)\n"
             "        order.status = OrderStatus.REFUNDED\n"
             "        self.bus.publish('order.refunded', {'id': order.id, 'amount': str(q.total.amount)})\n\n"
             "    def ship(self, order: Order) -> None:"),
        ],
    },
    "product-slug": {
        "good": [
            ("tallyho/models.py", "from .money import Money, total\n",
             "from .money import Money, total\nfrom .text import slugify\n"),
            ("tallyho/models.py", "\n\n@dataclass(frozen=True)\nclass Customer:",
             "\n    @property\n    def slug(self) -> str:\n"
             "        return slugify(f\"{self.sku} {self.name}\")\n"
             "\n\n@dataclass(frozen=True)\nclass Customer:"),
        ],
        "dead": [
            ("tallyho/models.py", "from .money import Money, total\n",
             "from .money import Money, total\nfrom .text import slugify\n"),
            ("tallyho/models.py", "\n\n@dataclass(frozen=True)\nclass Customer:",
             "\n    @property\n    def slug(self) -> str:\n"
             "        return slugify(self.name)\n"
             "\n\n@dataclass(frozen=True)\nclass Customer:"),
        ],
    },
    "ledger-refund": {
        "good": [
            ("tallyho/ledger.py", "    def balances(self)",
             "    def record_refund(self, on: date, order_id: str, quote: Quote) -> None:\n"
             "        z = Money.zero(self.currency)\n"
             "        self.post(\n"
             "            on, f\"refund {order_id}\",\n"
             "            (\"cash\", z, quote.total),\n"
             "            (\"revenue\", quote.discounted, z),\n"
             "            (\"tax_payable\", quote.tax, z),\n"
             "        )\n\n"
             "    def balances(self)"),
        ],
        "dead": [
            ("tallyho/ledger.py", "    def balances(self)",
             "    def record_refund(self, on: date, order_id: str, quote: Quote) -> None:\n"
             "        z = Money.zero(self.currency)\n"
             "        self.post(\n"
             "            on, f\"refund of order {order_id}\",\n"
             "            (\"cash\", z, quote.total),\n"
             "            (\"revenue\", quote.discounted, z),\n"
             "            (\"tax_payable\", quote.tax, z),\n"
             "        )\n\n"
             "    def balances(self)"),
        ],
    },
    "order-ids": {
        "good": [
            ("tallyho/repository.py", "    def save_order(self, o: Order) -> None:",
             "    def next_order_id(self) -> str:\n"
             "        n = sum(1 for _ in iter_records(self.store, ORDERS))\n"
             "        return f\"o{n + 1:06d}\"\n\n"
             "    def save_order(self, o: Order) -> None:"),
        ],
        "dead": [
            ("tallyho/repository.py", "    def save_order(self, o: Order) -> None:",
             "    def next_order_id(self) -> str:\n"
             "        import uuid\n"
             "        return f\"o-{uuid.uuid4().hex[:12]}\"\n\n"
             "    def save_order(self, o: Order) -> None:"),
        ],
    },
}
