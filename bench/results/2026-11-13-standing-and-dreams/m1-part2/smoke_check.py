"""The smokes' pass conditions (PREREG.md, Build, "Part 2"): one row per smoke directory.

Usage: python3 smoke_check.py RESULTS RUNS smoke-p0 smoke-p1 ...
"""

import json
import sys
from pathlib import Path

RESULTS, RUNS = Path(sys.argv[1]), Path(sys.argv[2])
ok = True
for out in sys.argv[3:]:
    meta = json.loads((RESULTS / out / "meta.json").read_text())
    iso = json.dumps(meta.get("agent_isolation"))
    for line in (RESULTS / out / "rows.jsonl").read_text().splitlines():
        r = json.loads(line)
        d = RUNS / out.replace("-", "") / f"{r['task']}-{r['arm']}-r{r['rep']}"
        models, edits, answered, top = set(), 0, False, 0
        for ev in (d / "events.jsonl").read_text(errors="replace").splitlines():
            try:
                e = json.loads(ev)
            except json.JSONDecodeError:
                continue
            if e.get("type") != "assistant":
                continue
            m = e.get("message") or {}
            models.add(m.get("model"))
            answered = True
            u = m.get("usage") or {}
            top = max(top, sum(u.get(k, 0) for k in (
                "input_tokens", "cache_creation_input_tokens", "cache_read_input_tokens")))
            for c in m.get("content") or []:
                if isinstance(c, dict) and c.get("type") == "tool_use" and c.get("name") in (
                        "Edit", "Write", "MultiEdit"):
                    edits += 1
        hooks = (d / "hook-count").read_text().strip() if (d / "hook-count").exists() else ""
        verify = (d / "verify.log").exists()
        conds = {
            "status ok": r["status"] == "ok",
            "answered": answered,
            "an edit": edits >= 1,
            "verify ran": verify,
            "hook-count": bool(hooks),
            "benchagent": "benchagent" in iso,
            "claude 2.1.318": "2.1.318" in json.dumps(meta),
            "models haiku": meta.get("models") == ["claude-haiku-5-5"]
            and models == {"claude-haiku-5-5"},
            "prompt <= 100K": top <= 100_000,
            "cost <= $0.04": r["cost_usd"] <= 0.04,
        }
        ok &= all(conds.values())
        print(f"{out} {r['task']} {r['arm']} r{r['rep']}: success={r['success']} edits={edits} "
              f"largest prompt={top} cost=${r['cost_usd']:.4f} hooks={hooks!r} "
              f"models={sorted(map(str, models))}")
        for k, v in conds.items():
            print(f"  {'ok ' if v else 'BAD'} {k}")
print(f"\nSmokes: {'pass' if ok else 'FAIL'}.")
sys.exit(0 if ok else 1)
