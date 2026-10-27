"""Small validators shared by the models and the CLI."""

from __future__ import annotations

import re

from .errors import ValidationError

_SKU = re.compile(r"^[A-Z0-9][A-Z0-9-]{1,31}$")
_EMAIL = re.compile(r"^[^@\s]+@[^@\s]+\.[^@\s]+$")
_REGION = re.compile(r"^[A-Z]{2}(-[A-Z0-9]{1,3})?$")


def check_sku(sku: str) -> None:
    if not _SKU.match(sku):
        raise ValidationError(f"invalid SKU {sku!r}", field="sku")


def check_email(email: str) -> None:
    if not _EMAIL.match(email):
        raise ValidationError(f"invalid email {email!r}", field="email")


def check_region(region: str) -> None:
    if not _REGION.match(region):
        raise ValidationError(f"invalid region {region!r}", field="region")


def require_positive(value: int, field: str) -> None:
    if not isinstance(value, int) or value <= 0:
        raise ValidationError(f"must be a positive integer, got {value!r}", field=field)


def parse_quantity(text: str) -> int:
    """Parse a CLI quantity such as ``"3"`` or ``"x3"``."""
    text = text.strip().lower().removeprefix("x")
    try:
        value = int(text)
    except ValueError:
        raise ValidationError(f"not a quantity: {text!r}", field="quantity") from None
    require_positive(value, "quantity")
    return value
