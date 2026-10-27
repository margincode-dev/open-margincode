"""paired.py BASE_ROWS[,..] NAME=ROWS[,..] ...: per metric, the mean over tasks of arm/baseline - 1,
with a task-stratified bootstrap 95% CI (sessions resampled within each task). ROWS is a rows.jsonl,
optionally `rows.jsonl:arm` to take one arm of a multi-arm run. Truncated sessions are skipped."""
import json, random, sys, statistics as st, collections

def load(paths):
    by = collections.defaultdict(list)
    for spec in paths.split(","):
        p, _, only = spec.partition(":")
        for l in open(p):
            r = json.loads(l)
            if only and r["arm"] != only:
                continue
            if r.get("status") != "ok" or (r.get("metrics") or {}).get("truncated"):
                continue
            by[r["task"]].append(r)
    return by

METRICS = {
    "cost": lambda r: r["cost_usd"],
    "tokens": lambda r: r["metrics"]["tokens_total"],
    "turns": lambda r: r["metrics"]["turns"],
    "wall": lambda r: r["wall_secs"],
    "tok_first_edit": lambda r: r["metrics"]["tokens_to_first_edit"],
}

def rel(base, arm, f, rng=None):
    out = []
    for t in base:
        if t not in arm:
            continue
        b, a = base[t], arm[t]
        if rng:
            b = [rng.choice(b) for _ in b]
            a = [rng.choice(a) for _ in a]
        av = [v for v in map(f, a) if v is not None]
        bv = [v for v in map(f, b) if v is not None]
        if av and bv:
            out.append(st.mean(av) / st.mean(bv) - 1)
    return st.mean(out)

base = load(sys.argv[1])
rng = random.Random(1)
print(f"baseline: {sum(map(len, base.values()))} sessions over {len(base)} tasks")
for spec in sys.argv[2:]:
    name, paths = spec.split("=", 1)
    arm = load(paths)
    print(f"{name}: {sum(map(len, arm.values()))} sessions, accepted {sum(r['success'] for v in arm.values() for r in v)}")
    for m, f in METRICS.items():
        point = rel(base, arm, f)
        boots = sorted(rel(base, arm, f, rng) for _ in range(2000))
        print(f"   {m:15} {point:+6.1%}  95% CI [{boots[50]:+6.1%}, {boots[1949]:+6.1%}]")
