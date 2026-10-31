# pantry

A tiny comand-line tool that tracks what is in your kitchen and when it expires.

    python -m pantry add rice --category grains --quantity 2 --expires 2026-12-01
    python -m pantry list
    python -m pantry report --out report.html

Run the tests with `python3 -m unittest`. It has no dependancies beyond Python 3.
