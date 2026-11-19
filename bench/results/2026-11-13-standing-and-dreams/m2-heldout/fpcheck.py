"""Free check 4 (and 5): in each first-packs dir, each task's hook pack names its own wrong note
(by the first 60 characters of its summary), no pack line contains --supersedes, no record line
carries an origin label. With a second (dreamed) dir: the record lines that differ, per task."""
import re
import sys
import tomllib
from pathlib import Path

tree = Path(sys.argv[1])
pool_path, raw = Path(sys.argv[2]), Path(sys.argv[3])
dreamed = Path(sys.argv[4]) if len(sys.argv) > 4 else None
pool = tomllib.loads((tree / pool_path).read_text(encoding="utf-8"))["note"]
wrong = {f"relay-{n['pair']}": n["summary"] for n in pool if n.get("no_commit")}
LABEL = re.compile(r"^\s*- rec:\w+ .*\((open|abandoned|reverted)\)", re.M)
bad = 0


def recs(text):
    out = {}
    for line in text.splitlines():
        m = re.match(r"^\s*- rec:\w+ (.*)$", line)
        if m:
            out[m.group(1)] = line
    return out


for task, summary in sorted(wrong.items()):
    if not (raw / "packs" / f"{task}-hook.md").exists():
        continue
    hook = (raw / "packs" / f"{task}-hook.md").read_text(encoding="utf-8")
    texts = [p.read_text(encoding="utf-8") for p in (raw / "packs").glob(f"{task}-*.md")]
    shown = summary[:60] in hook
    sup = any("--supersedes" in t for t in texts)
    lab = any(LABEL.search(t) for t in texts)
    ok = shown and not sup and not lab
    bad += not ok
    print(f"{'ok ' if ok else 'BAD'} {task}: wrong note in hook pack {shown}; --supersedes line {sup}; "
          f"origin label {lab}")
    if dreamed:
        d = (dreamed / "packs" / f"{task}-hook.md").read_text(encoding="utf-8")
        dtexts = [p.read_text(encoding="utf-8") for p in (dreamed / "packs").glob(f"{task}-*.md")]
        dsup = any("--supersedes" in t for t in dtexts)
        dlab = any(LABEL.search(t) for t in dtexts)
        marks = d.count("(disputed)")
        own = any(summary[:60] in line and "(disputed)" in line for line in d.splitlines())
        print(f"    dreamed: hook pack differs from raw {d != hook}; (disputed) marks {marks}; "
              f"own wrong note marked disputed in the hook pack {own}; --supersedes {dsup}; "
              f"origin label {dlab}")
        bad += dsup or dlab
print("all ok" if not bad else f"{bad} problem(s)")
sys.exit(1 if bad else 0)
