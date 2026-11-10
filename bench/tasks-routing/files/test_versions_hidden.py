"""Hidden check: parse_version still returns a triple and rejects the rest."""

from versions import parse_version

assert parse_version("1.2.3") == (1, 2, 3)
assert parse_version("10.0.2") == (10, 0, 2)
for bad in ("1.2", "a.b.c", ""):
    try:
        parse_version(bad)
    except ValueError:
        continue
    raise SystemExit(f"{bad!r} should raise ValueError")
print("ok")
