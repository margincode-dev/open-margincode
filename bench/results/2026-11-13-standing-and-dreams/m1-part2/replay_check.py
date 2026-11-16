"""Free check 3 of PREREG.md's Build, "Part 2": reads the two replays' hook packs and prints, per
build and task, where the task's wrong note stands and how every seeded wrong note is labelled.
Where a pre-read answer carries the note is in each replay's own `first-packs.md`.

Usage: python3 replay_check.py RESULTS POOL   (RESULTS holds first-packs-p0/ and first-packs-p1/)
"""

import re
import sys
import tomllib
from pathlib import Path

RESULTS, POOL = Path(sys.argv[1]), Path(sys.argv[2])
TASKS = ["relay-quote-split", "relay-bus-resubscribe", "relay-tax-with-rate"]
HINT = "Claims by earlier agents"
HEAD = 40  # a `handle`-tier line keeps about 70 characters of the summary
ORIGIN = re.compile(r"rec:\w+ \w+, (open|reverted|abandoned|merged):")

pool = tomllib.loads(POOL.read_text(encoding="utf-8"))["note"]
wrong = {f"relay-{n['pair']}": n["summary"] for n in pool if n.get("no_commit")}
print(f"{len(wrong)} seeded wrong notes in the pool\n")
ok = True
print("| build | task | task's wrong note in the hook pack | its line has `rec:` | its label | "
      "lines with a seeded wrong note | of them `reverted` | with any origin | Phase 0's line "
      "under \"Decisions & dead ends\" |")
print("|---|---|---|---|---|---|---|---|---|")
for b in ("p0", "p1"):
    for t in TASKS:
        hook = (RESULTS / f"first-packs-{b}" / "packs" / f"{t}-hook.md").read_text(encoding="utf-8")
        lines = hook.splitlines()
        own = [ln for ln in lines if wrong[t][:HEAD] in ln]
        tier = "absent" if not own else "summary" if wrong[t] in hook else "handle"
        m = ORIGIN.search(own[0]) if own else None
        label = m.group(1) if m else "none"
        seeded = [ln for ln in lines if any(s[:HEAD] in ln for s in wrong.values())]
        rev = sum("reverted:" in ln for ln in seeded)
        origin = sum(bool(ORIGIN.search(ln)) for ln in seeded)
        dd = next((i for i, ln in enumerate(lines) if "Decisions & dead ends" in ln), None)
        hint = dd is not None and any(HINT in ln for ln in lines[dd + 1:dd + 3])
        has_rec = bool(own) and "rec:" in own[0]
        if b == "p0":
            passed = tier != "absent" and has_rec and origin == 0 and hint
        else:
            passed = hint and rev == len(seeded)
        ok &= passed
        print(f"| {b} | {t} | {tier} | {'yes' if has_rec else 'no'} | {label} | {len(seeded)} | "
              f"{rev} | {origin} | {'yes' if hint else 'no'} |")
print(f"\nReplay conditions: {'pass' if ok else 'FAIL'}.")
sys.exit(0 if ok else 1)
