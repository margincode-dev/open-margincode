"""Money spent so far by the Sonnet run (amendment 2's money rule). Reads only costs, never an
accepted count: each row's `cost_usd` (the harness figure at the registered prices) and Claude
Code's own `total_cost_usd` from the session's `events.jsonl`. A session counts at the larger of
the two; the run's spend is the sum of those.

With --detail it also prints, per session, both figures, the largest single request's prompt
(input + cache write + cache read tokens of one assistant message), the models that answered,
and how Claude Code's result ended (`subtype`, which names a session stopped at its cap). This is
what the smoke reports.

Usage: python3 sonnet_spend.py ROOT [--detail] [--ceiling 7.00] [OUT ...]
       (ROOT /m2b/m0s; OUT defaults to the four directories of amendment 2)
"""

import json
import sys
from pathlib import Path

OUTS = ["smoke-s-a", "smoke-s-b", "m0s-av-before", "m0s-phase0"]


def events(path: Path):
    if not path.exists():
        return
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        try:
            yield json.loads(line)
        except json.JSONDecodeError:
            continue


def session(d: Path) -> dict:
    out = {"cc": None, "prompt": 0, "models": set(), "end": None}
    seen = set()
    for e in events(d / "events.jsonl"):
        if e.get("type") == "result":
            out["cc"] = e.get("total_cost_usd")
            out["end"] = e.get("subtype")
        m = e.get("message") or {}
        if e.get("type") == "assistant" and isinstance(m, dict):
            if m.get("model"):
                out["models"].add(m["model"])
            # one API response is split over several events with the same id
            if m.get("id") in seen:
                continue
            seen.add(m.get("id"))
            u = m.get("usage") or {}
            p = sum(u.get(k) or 0 for k in
                    ("input_tokens", "cache_creation_input_tokens", "cache_read_input_tokens"))
            out["prompt"] = max(out["prompt"], p)
    return out


def main() -> None:
    args = sys.argv[1:]
    root = Path(args.pop(0))
    detail = "--detail" in args
    ceiling = 7.00
    if "--ceiling" in args:
        ceiling = float(args[args.index("--ceiling") + 1])
    outs = [a for a in args if not a.startswith("--") and not a[0].isdigit()] or OUTS
    total = 0.0
    for out in outs:
        rows_path = root / "results" / out / "rows.jsonl"
        if not rows_path.exists():
            continue
        run = root / "bench-work" / "runs" / out.replace("-", "")
        h = c = t = 0.0
        n = 0
        for line in rows_path.read_text(encoding="utf-8").splitlines():
            if not line:
                continue
            r = json.loads(line)
            n += 1
            hc = r.get("cost_usd") or 0.0
            s = session(run / f"{r['task']}-{r['arm']}-r{r['rep']}")
            cc = s["cc"] or 0.0
            h, c, t = h + hc, c + cc, t + max(hc, cc)
            if detail:
                print(f"  {out} {r['task']} {r['arm']} r{r['rep']} {r['status']}: harness "
                      f"${hc:.4f}, claude ${cc:.4f}, largest prompt {s['prompt']} tokens, "
                      f"models {sorted(s['models'])}, end {s['end']}")
        print(f"{out}: {n} rows, harness ${h:.4f}, claude ${c:.4f}, counted ${t:.4f}")
        total += t
    print(f"total counted: ${total:.4f}; left under the ${ceiling:.2f} ceiling: "
          f"${ceiling - total:.4f}")


if __name__ == "__main__":
    main()
