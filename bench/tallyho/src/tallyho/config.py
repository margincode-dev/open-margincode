"""Shop configuration from an INI-style file."""

from __future__ import annotations

import configparser
from dataclasses import dataclass, field
from decimal import Decimal, InvalidOperation
from pathlib import Path

from .errors import ConfigError
from .pricing.tax import TaxTable, default_tax_table


@dataclass
class Config:
    shop_name: str = "tallyho shop"
    currency: str = "USD"
    store_dir: Path = Path("./data")
    page_size: int = 100
    taxes: TaxTable = field(default_factory=default_tax_table)


def load_config(path: str | Path) -> Config:
    """Read ``[shop]`` and ``[tax]`` sections; missing keys keep their defaults."""
    parser = configparser.ConfigParser()
    try:
        with open(path, encoding="utf-8") as f:
            parser.read_file(f)
    except OSError as e:
        raise ConfigError(f"cannot read {path}: {e}") from e
    cfg = Config()
    if parser.has_section("shop"):
        s = parser["shop"]
        cfg.shop_name = s.get("name", cfg.shop_name)
        cfg.currency = s.get("currency", cfg.currency).upper()
        cfg.store_dir = Path(s.get("store_dir", str(cfg.store_dir)))
        try:
            cfg.page_size = s.getint("page_size", cfg.page_size)
        except ValueError as e:
            raise ConfigError(f"page_size: {e}") from e
    if parser.has_section("tax"):
        for region, rate in parser["tax"].items():
            try:
                cfg.taxes.rates[region.upper()] = Decimal(rate)
            except InvalidOperation:
                raise ConfigError(f"tax rate for {region}: not a number: {rate!r}") from None
    return cfg
