"""Free check of the held-out task set (AMENDMENT-3), without a model and without running tallyho's tests:

- the pool `pool/notes-heldout.toml` holds exactly one wrong note per task here (a `decision` kept
  out of the commit log, anchored with a path and a symbol in the file its task changes, summary of
  at most 200 characters, with a body) and five true lessons (no `no_commit`, no `noise`, a pair no
  task selects);
- no note of the pool is a note of `../tasks-relay/pool/notes-silent.toml` (the pool the revised
  `DREAM.md` was fitted on), by pair or by summary;
- each spec has the fixed fields, a goal that does not mention notes, records or lessons, and a
  hidden test that exists;
- each `[[dead_end]]` pattern matches the text its pair's `dead` edit writes and none of the text
  its `good` edit writes, and not the goal.

With a tallyho checkout as argument it also checks that every note's symbol is defined in the file
it is anchored to. What the edits do to the tests is `reference/check.py`.

Usage: python3 bench/tasks-heldout/check_set.py [TALLYHO_REPO]
"""

import fnmatch
import re
import sys
import tomllib
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE / "reference"))
from edits import PAIRS, SILENT  # noqa: E402

FIXED = {
    "repo_path": "~/bench-repos/tallyho",
    "base_rev": "main",
    "verify_cmd": "~/bench-work/venv/bin/python -m pytest -q -p no:cacheprovider tests",
    "path_prepend": ["~/bench-work/venv/bin"],
    "max_turns": 30,
    "model": "claude-haiku-5-5",
    "session_timeout_secs": 900,
    "relay_pool": "pool/notes-heldout.toml",
}

problems = []


def check(ok, what):
    if not ok:
        problems.append(what)
    return ok


def load(path):
    return tomllib.loads(path.read_text(encoding="utf-8"))


pool = load(HERE / "pool/notes-heldout.toml")["note"]
fitted = load(HERE.parent / "tasks-relay/pool/notes-silent.toml")["note"]
specs = {load(p)["relay_pair"]: load(p) for p in sorted(HERE.glob("*.toml"))}
repo = Path(sys.argv[1]) if len(sys.argv) > 1 else None

wrong = [n for n in pool if n.get("no_commit")]
true = [n for n in pool if not n.get("no_commit")]
check(sorted(n["pair"] for n in wrong) == sorted(specs), "the wrong notes are not one per task")
check(sorted(specs) == sorted(SILENT) == sorted(PAIRS), "edits.PAIRS / SILENT do not name the tasks")
check(len(true) == 5 and not any(n.get("noise") or n["pair"] in specs for n in true),
      "the true lessons are not five notes that no task selects")
old = {n["pair"] for n in fitted} | {n["summary"] for n in fitted}
check(not any(n["pair"] in old or n["summary"] in old for n in pool), "a note is in notes-silent.toml")
print(f"{'ok ' if not problems else 'BAD'} pool: {len(wrong)} wrong, {len(true)} true, none fitted")

for n in pool:
    check(len(n["summary"]) <= 200, f"{n['pair']}: summary is {len(n['summary'])} characters")
    check(n["kind"] == "decision" and n.get("symbol") and n.get("body", "").strip(),
          f"{n['pair']}: not an anchored decision with a body")
    if repo:
        name = re.escape(n["symbol"].rsplit(".", 1)[-1])
        text = (repo / n["path"]).read_text(encoding="utf-8")
        check(re.search(rf"^\s*(class|def) {name}\b", text, re.M),
              f"{n['pair']}: {n['symbol']} is not defined in {n['path']}")

for pair, spec in specs.items():
    before = len(problems)
    note = next(n for n in wrong if n["pair"] == pair)
    for key, value in FIXED.items():
        check(spec.get(key) == value, f"{pair}: {key} is {spec.get(key)!r}")
    check(spec["name"] == f"relay-{pair}", f"{pair}: name")
    check(note["path"] in spec["relevant_files"], f"{pair}: the note is not in a file the task changes")
    check(all(path in spec["relevant_files"] for side in PAIRS[pair].values() for path, _, _ in side),
          f"{pair}: an edit touches a file outside relevant_files")
    test = f"test_relay_{pair.replace('-', '_')}.py"
    check(spec["local_tests"] == [{"path": f"tests/{test}", "source": f"files/{test}"}],
          f"{pair}: local_tests")
    check((HERE / "files" / test).is_file(), f"{pair}: {test} is missing")
    check(not re.search(r"note|record|lesson", spec["goal"], re.I), f"{pair}: the goal mentions notes")
    dead = good = in_goal = False
    for d in spec["dead_end"]:
        rx = re.compile(d["pattern"])

        def hit(side):
            return any(fnmatch.fnmatch(path, d["file_glob"]) and rx.search(text)
                       for path, _old, text in PAIRS[pair][side])

        dead, good, in_goal = dead or hit("dead"), good or hit("good"), in_goal or bool(rx.search(spec["goal"]))
    check(dead and not good and not in_goal,
          f"{pair}: pattern: dead matches={dead}, good matches={good}, goal matches={in_goal}")
    print(f"{'ok ' if len(problems) == before else 'BAD'} {pair}: note of {len(note['summary'])} "
          f"characters; dead edit matches={dead}, good edit matches={good}")

for p in problems:
    print(f"    {p}")
print("held-out set ok" if not problems else f"{len(problems)} problem(s)")
sys.exit(1 if problems else 0)
