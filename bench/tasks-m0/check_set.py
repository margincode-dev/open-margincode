"""Free check of the M0 task set, without a model.

Always (no tallyho needed):

- `bus-resubscribe.toml` and `tax-with-rate.toml` are byte for byte the files of
  `../tasks-relay-silent`, so `../tasks-relay-silent/check_set.py` and everything measured or
  replayed on them there applies here;
- `quote-split.toml` is `../tasks-relay-wrong/quote-split.toml` with one field changed,
  `relay_pool`, now `notes-silent.toml`, so all three tasks are run under one pool of thirteen
  notes;
- in that pool each task's own note is the wrong one: a `decision` kept out of the commit log
  (`no_commit`), anchored to a symbol (the two silent notes in the file the task changes,
  `quote-split`'s on `Money.allocate`), and for `quote-split` the text of
  `notes-wrong.toml`, unchanged.

With a tallyho checkout and a Python with pytest as arguments, it also runs, for each of the three
pairs, `../tasks-relay/reference/check.py`'s silent proof: on the base tree the repo's own tests
pass and the hidden test fails; with the reference fix both pass; with the edit the wrong note
recommends (`dead` in `edits.py`) the own tests still pass and only the hidden test fails.
`check.py` itself proves that only for its `SILENT` pairs, which do not include `quote-split`.

Usage: python3 bench/tasks-m0/check_set.py [TALLYHO_REPO PYTHON]
"""

import sys
import tomllib
from pathlib import Path

HERE = Path(__file__).resolve().parent
BENCH = HERE.parent
RELAY = BENCH / "tasks-relay"
sys.path.insert(0, str(RELAY / "reference"))

TASKS = {"quote-split", "bus-resubscribe", "tax-with-rate"}
POOL = "../tasks-relay/pool/notes-silent.toml"

problems = []


def check(ok, what):
    if not ok:
        problems.append(what)
    return ok


def load(path):
    return tomllib.loads(path.read_text(encoding="utf-8"))


specs = {p.stem: load(p) for p in sorted(HERE.glob("*.toml"))}
check(set(specs) == TASKS, f"the set is {sorted(specs)}, want {sorted(TASKS)}")

for name in ("bus-resubscribe", "tax-with-rate"):
    same = (HERE / f"{name}.toml").read_bytes() == (BENCH / "tasks-relay-silent" / f"{name}.toml").read_bytes()
    check(same, f"{name}.toml differs from ../tasks-relay-silent/{name}.toml")
    print(f"{'ok ' if same else 'BAD'} {name}: the spec of ../tasks-relay-silent, byte for byte")

if "quote-split" in specs:
    old = load(BENCH / "tasks-relay-wrong" / "quote-split.toml")
    new = dict(specs["quote-split"])
    pool_ok = check(new.pop("relay_pool", None) == POOL, f"quote-split: relay_pool is not {POOL}")
    old.pop("relay_pool", None)
    rest_ok = check(new == old, "quote-split: a field other than relay_pool differs from ../tasks-relay-wrong")
    print(f"{'ok ' if pool_ok and rest_ok else 'BAD'} quote-split: the spec of ../tasks-relay-wrong, "
          "relay_pool set to notes-silent.toml")

pool = load(RELAY / "pool" / "notes-silent.toml")["note"]
wrong = {n["pair"]: n for n in load(RELAY / "pool" / "notes-wrong.toml")["note"]}
for name, spec in specs.items():
    before = len(problems)
    check(spec.get("relay_pool") == POOL and spec.get("relay_pair") == name, f"{name}: pool or pair")
    notes = [n for n in pool if n["pair"] == name]
    if check(len(notes) == 1, f"{name}: {len(notes)} notes in the pool, want 1"):
        n = notes[0]
        check(n["kind"] == "decision" and n.get("no_commit") is True and not n.get("noise"),
              f"{name}: the note is not a no_commit decision")
        # anchored where the pack finds it: the two silent notes in the file the task changes,
        # quote-split's on `Money.allocate` in money.py, the call the right fix makes
        check(n.get("path") and n.get("symbol"), f"{name}: the note is not anchored to a symbol")
        check(len(n["summary"]) <= 200, f"{name}: summary is {len(n['summary'])} characters")
        if name == "quote-split":
            check(n == wrong.get(name), "quote-split: the note differs from notes-wrong.toml's")
    print(f"{'ok ' if len(problems) == before else 'BAD'} {name}: its own note in notes-silent.toml "
          "is a wrong decision, no_commit, anchored")

if len(sys.argv) > 2:
    from check import silent  # noqa: E402

    repo, python = Path(sys.argv[1]), sys.argv[2]
    for name in sorted(specs):
        test = f"test_relay_{name.replace('-', '_')}.py"
        check(silent(repo, python, name, test), f"{name}: not silent")
else:
    print("(no tallyho checkout given: the silent proof was not run)")

for p in problems:
    print(f"    {p}")
print("M0 set ok" if not problems else f"{len(problems)} problem(s)")
sys.exit(1 if problems else 0)
