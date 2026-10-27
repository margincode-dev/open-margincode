"""The ``tallyho`` command: list products and orders, quote an order, export CSV."""

from __future__ import annotations

import argparse
import sys

from .config import Config, load_config
from .report import orders_to_csv, render_table
from .repository import Repository
from .store import JsonFileStore, MemoryStore
from .text import summarize


def _repo(cfg: Config, dry_run: bool) -> Repository:
    store = MemoryStore(cfg.page_size) if dry_run else JsonFileStore(cfg.store_dir, cfg.page_size)
    return Repository(store)


def cmd_products(repo: Repository, args: argparse.Namespace) -> int:
    rows = [[p.sku, summarize(p.name, 30), str(p.price)] for p in repo.products(args.category)]
    print(render_table(["SKU", "Name", "Price"], rows, ["left", "left", "right"]))
    return 0


def cmd_export(repo: Repository, args: argparse.Namespace) -> int:
    sys.stdout.write(orders_to_csv(repo.orders()))
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(prog="tallyho")
    parser.add_argument("--config")
    parser.add_argument("--dry-run", action="store_true")
    sub = parser.add_subparsers(dest="cmd", required=True)
    p = sub.add_parser("products")
    p.add_argument("--category")
    sub.add_parser("export")
    args = parser.parse_args(argv)
    cfg = load_config(args.config) if args.config else Config()
    repo = _repo(cfg, args.dry_run)
    return {"products": cmd_products, "export": cmd_export}[args.cmd](repo, args)


if __name__ == "__main__":
    raise SystemExit(main())
