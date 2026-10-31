"""Command line: python -m pantry add|list|report."""

import argparse
import sys

from . import report, store


def main(argv=None):
    ap = argparse.ArgumentParser(prog="pantry")
    ap.add_argument("--file", default="pantry.json")
    sub = ap.add_subparsers(dest="cmd", required=True)
    a = sub.add_parser("add")
    a.add_argument("name")
    a.add_argument("--category", default="other")
    a.add_argument("--quantity", type=int, default=1)
    a.add_argument("--expires", required=True, help="YYYY-MM-DD")
    sub.add_parser("list")
    r = sub.add_parser("report")
    r.add_argument("--out", default="report.html")
    args = ap.parse_args(argv)

    items = store.load(args.file)
    if args.cmd == "add":
        items.append(
            store.Item(args.name, args.category, args.quantity, store.parse_date(args.expires))
        )
        store.save(args.file, items)
    elif args.cmd == "list":
        for i in sorted(items, key=lambda i: i.expires):
            print(f"{i.expires}  {i.quantity:>3}  {i.name}  ({i.category})")
    elif args.cmd == "report":
        with open(args.out, "w") as f:
            f.write(report.render(items))
    return 0


if __name__ == "__main__":
    sys.exit(main())
