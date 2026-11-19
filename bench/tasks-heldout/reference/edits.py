"""Reference fixes and dead ends for the held-out pairs, as (path, old, new) text edits on tallyho
`main`, in the form of `../../tasks-relay/reference/edits.py`. `dead` is the edit the pair's wrong
note recommends. `check.py` (a copy of `../../tasks-relay/reference/check.py`) proves for each
pair that the hidden test passes on `good` and fails on `dead`, and, run apart, that the repo's own
tests pass on the base tree, on `good` and on `dead`, while the hidden test fails on the base tree.
"""

_NOT_FOUND = ("tallyho/repository.py", "from .models import Customer",
              "from .errors import NotFound\nfrom .models import Customer")

PAIRS = {
    "order-can-cancel": {
        "good": [
            ("tallyho/models.py", "    def remove(self, sku: str) -> None:",
             "    def can_cancel(self) -> bool:\n"
             "        return self.status in (OrderStatus.DRAFT, OrderStatus.PLACED)\n\n"
             "    def remove(self, sku: str) -> None:"),
        ],
        "dead": [
            ("tallyho/models.py", "    def remove(self, sku: str) -> None:",
             "    def can_cancel(self) -> bool:\n"
             "        return self.status is OrderStatus.DRAFT\n\n"
             "    def remove(self, sku: str) -> None:"),
        ],
    },
    "delete-order": {
        "good": [
            _NOT_FOUND,
            ("tallyho/repository.py", "    def _order_from_record(self, r: dict) -> Order:",
             "    def delete_order(self, oid: str) -> bool:\n"
             "        try:\n"
             "            self.store.get(ORDERS, oid)\n"
             "        except NotFound:\n"
             "            return False\n"
             "        self.store.delete(ORDERS, oid)\n"
             "        return True\n\n"
             "    def _order_from_record(self, r: dict) -> Order:"),
        ],
        "dead": [
            _NOT_FOUND,
            ("tallyho/repository.py", "    def _order_from_record(self, r: dict) -> Order:",
             "    def delete_order(self, oid: str) -> bool:\n"
             "        try:\n"
             "            self.store.delete(ORDERS, oid)\n"
             "        except NotFound:\n"
             "            return False\n"
             "        return True\n\n"
             "    def _order_from_record(self, r: dict) -> Order:"),
        ],
    },
    "order-comments": {
        "good": [
            _NOT_FOUND,
            ("tallyho/repository.py", 'PRODUCTS, CUSTOMERS, ORDERS = "products", "customers", "orders"\n',
             'PRODUCTS, CUSTOMERS, ORDERS = "products", "customers", "orders"\n'
             'COMMENTS = "order_comments"\n'),
            ("tallyho/repository.py", "    def _order_from_record(self, r: dict) -> Order:",
             "    def add_order_comment(self, oid: str, text: str) -> None:\n"
             "        try:\n"
             "            r = self.store.get(COMMENTS, oid)\n"
             "        except NotFound:\n"
             "            r = {\"id\": oid, \"comments\": []}\n"
             "        r[\"comments\"].append(text)\n"
             "        self.store.put(COMMENTS, r)\n\n"
             "    def order_comments(self, oid: str) -> list[str]:\n"
             "        try:\n"
             "            return list(self.store.get(COMMENTS, oid)[\"comments\"])\n"
             "        except NotFound:\n"
             "            return []\n\n"
             "    def _order_from_record(self, r: dict) -> Order:"),
        ],
        "dead": [
            _NOT_FOUND,
            ("tallyho/repository.py", 'PRODUCTS, CUSTOMERS, ORDERS = "products", "customers", "orders"\n',
             'PRODUCTS, CUSTOMERS, ORDERS = "products", "customers", "orders"\n'
             'COMMENTS = "order-comments"\n'),
            ("tallyho/repository.py", "    def _order_from_record(self, r: dict) -> Order:",
             "    def add_order_comment(self, oid: str, text: str) -> None:\n"
             "        try:\n"
             "            r = self.store.get(COMMENTS, oid)\n"
             "        except NotFound:\n"
             "            r = {\"id\": oid, \"comments\": []}\n"
             "        r[\"comments\"].append(text)\n"
             "        self.store.put(COMMENTS, r)\n\n"
             "    def order_comments(self, oid: str) -> list[str]:\n"
             "        try:\n"
             "            return list(self.store.get(COMMENTS, oid)[\"comments\"])\n"
             "        except NotFound:\n"
             "            return []\n\n"
             "    def _order_from_record(self, r: dict) -> Order:"),
        ],
    },
    "ledger-revenue": {
        "good": [
            ("tallyho/ledger.py", "    def trial_balance_ok(self) -> bool:",
             "    def revenue(self) -> Money:\n"
             "        return -self.balances().get(\"revenue\", Money.zero(self.currency))\n\n"
             "    def trial_balance_ok(self) -> bool:"),
        ],
        "dead": [
            ("tallyho/ledger.py", "    def trial_balance_ok(self) -> bool:",
             "    def revenue(self) -> Money:\n"
             "        return self.balances().get(\"revenue\", Money.zero(self.currency))\n\n"
             "    def trial_balance_ok(self) -> bool:"),
        ],
    },
    "coupon-days-left": {
        "good": [
            ("tallyho/pricing/coupons.py", "        return True\n\n\nclass CouponBook:",
             "        return True\n\n"
             "    def days_left(self, today: date) -> int | None:\n"
             "        if self.expires is None:\n"
             "            return None\n"
             "        return max((self.expires - today).days + 1, 0)\n\n\n"
             "class CouponBook:"),
        ],
        "dead": [
            ("tallyho/pricing/coupons.py", "        return True\n\n\nclass CouponBook:",
             "        return True\n\n"
             "    def days_left(self, today: date) -> int | None:\n"
             "        if self.expires is None:\n"
             "            return None\n"
             "        return max((self.expires - today).days, 0)\n\n\n"
             "class CouponBook:"),
        ],
    },
}

# Every held-out pair is silent: with the dead-end edit the repo's own tests still pass and only
# the hidden test fails.
SILENT = tuple(PAIRS)
