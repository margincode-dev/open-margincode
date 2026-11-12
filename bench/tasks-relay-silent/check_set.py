"""Free check of this task set, without a model and without running tallyho's tests:

- the pool `notes-silent.toml` is `notes-wrong.toml` (its ten notes unchanged, in the same order)
  followed by exactly one wrong note per task here: a `decision` kept out of the commit log
  (`no_commit`), anchored with a path and a symbol in the file the task changes, summary of at
  most 200 characters, with a body;
- each spec has the fields of the wrong-lesson set, a goal that does not mention notes, and a
  hidden test that exists;
- each `[[dead_end]]` pattern matches the text its pair's `dead` edit writes (the edit the wrong
  note recommends) and none of the text its `good` edit writes, as
  `../results/2026-11-wrong-lesson/patterns.py` checks for the wrong-lesson set.

With a tallyho checkout as argument it also checks that each new note's symbol is defined in the
file it is anchored to. What the edits do to the tests is `../tasks-relay/reference/check.py`.

Usage: python3 bench/tasks-relay-silent/check_set.py [TALLYHO_REPO]
"""

import fnmatch
import re
import sys
import tomllib
from pathlib import Path

HERE = Path(__file__).resolve().parent
RELAY = HERE.parent / "tasks-relay"
sys.path.insert(0, str(RELAY / "reference"))
from edits import PAIRS, SILENT  # noqa: E402

FIXED = {
    "repo_path": "~/bench-repos/tallyho",
    "base_rev": "main",
    "verify_cmd": "~/bench-work/venv/bin/python -m pytest -q -p no:cacheprovider tests",
    "path_prepend": ["~/bench-work/venv/bin"],
    "max_turns": 30,
    "model": "deepseek-flash",
    "session_timeout_secs": 900,
    "relay_pool": "../tasks-relay/pool/notes-silent.toml",
}

problems = []


def check(ok, what):
    if not ok:
        problems.append(what)
    return ok


def load(path):
    return tomllib.loads(path.read_text(encoding="utf-8"))


wrong = load(RELAY / "pool/notes-wrong.toml")["note"]
pool = load(RELAY / "pool/notes-silent.toml")["note"]
specs = {p.stem: load(p) for p in sorted(HERE.glob("*.toml"))}
repo = Path(sys.argv[1]) if len(sys.argv) > 1 else None

same = pool[: len(wrong)] == wrong
check(same, "the first notes of notes-silent.toml are not notes-wrong.toml unchanged")
print(f"{'ok ' if same else 'BAD'} pool: the {len(wrong)} notes of notes-wrong.toml, unchanged and in order")
new = pool[len(wrong):]
check(sorted(n["pair"] for n in new) == sorted(specs), "the notes after them are not one per task of this set")
check(sorted(specs) == sorted(SILENT), "edits.SILENT does not name the tasks of this set")

for pair, spec in specs.items():
    before = len(problems)
    notes = [n for n in new if n["pair"] == pair]
    check(len(notes) == 1, f"{pair}: {len(notes)} notes after the wrong pool, want 1")
    for n in notes:
        check(n["kind"] == "decision", f"{pair}: the note is not a decision")
        check(n.get("no_commit") is True and not n.get("noise"), f"{pair}: the note is not no_commit")
        check(len(n["summary"]) <= 200, f"{pair}: summary is {len(n['summary'])} characters")
        check(n.get("body", "").strip(), f"{pair}: the note has no body")
        check(n["path"] in spec["relevant_files"] and n.get("symbol"),
              f"{pair}: the note is not anchored to a symbol in a file the task changes")
        check(all(path in spec["relevant_files"] for side in PAIRS[pair].values() for path, _, _ in side),
              f"{pair}: an edit touches a file outside relevant_files")
        if repo and n.get("symbol"):
            name = re.escape(n["symbol"].rsplit(".", 1)[-1])
            text = (repo / n["path"]).read_text(encoding="utf-8")
            check(re.search(rf"^\s*(class|def) {name}\b", text, re.M),
                  f"{pair}: {n['symbol']} is not defined in {n['path']}")
    for key, value in FIXED.items():
        check(spec.get(key) == value, f"{pair}: {key} is {spec.get(key)!r}")
    check(spec["name"] == f"relay-{pair}" and spec["relay_pair"] == pair, f"{pair}: name or relay_pair")
    test = f"test_relay_{pair.replace('-', '_')}.py"
    check(spec["local_tests"] == [{"path": f"tests/{test}", "source": f"../tasks-relay/files/{test}"}],
          f"{pair}: local_tests")
    check((RELAY / "files" / test).is_file(), f"{pair}: {test} is missing")
    check(not re.search(r"note|record|lesson", spec["goal"], re.I), f"{pair}: the goal mentions notes")
    dead = good = in_goal = False
    for d in spec["dead_end"]:
        rx = re.compile(d["pattern"])

        def hit(side):
            return any(fnmatch.fnmatch(path, d["file_glob"]) and rx.search(text)
                       for path, _old, text in PAIRS[pair][side])

        dead, good, in_goal = dead or hit("dead"), good or hit("good"), in_goal or bool(rx.search(spec["goal"]))
    check(dead and not good, f"{pair}: pattern: dead edit matches={dead}, good edit matches={good}")
    check(not in_goal, f"{pair}: the goal contains the dead-end pattern")
    summary = len(notes[0]["summary"]) if notes else 0
    print(f"{'ok ' if len(problems) == before else 'BAD'} {pair}: note of {summary} characters; "
          f"dead edit matches={dead}, good edit matches={good}")

for p in problems:
    print(f"    {p}")
print("silent set ok" if not problems else f"{len(problems)} problem(s)")
sys.exit(1 if problems else 0)
