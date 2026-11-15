import json, glob, os, subprocess
repo = os.path.expanduser("~/bench-repos/tallyho")
def show(path):
    return subprocess.run(["git", "-C", repo, "show", f"main:{path}"], capture_output=True, text=True).stdout
# every record text of stage A's pool, from the stub run's bundles (same seeds)
stub = {}
for b in glob.glob("/root/bench-work/df4257/relay-quote-split/dreams/*/bundle.json"):
    for it in json.load(open(b))["items"]:
        r = it["record"]; stub[r["summary"]] = r
for i in range(1, 7):
    t = list(json.load(open(f"/root/m2/stage-a/dreamed-{i}.json"))["tasks"].values())[0]
    ok = bad = other = 0; fails = []
    for v in t["verdicts"]:
        evs = [("verdict", e) for e in v["evidence"]] + [("review", e) for e in (v.get("review") or {}).get("evidence", [])]
        for who, e in evs:
            if "code" in e:
                good = e["quote"] in show(e["code"])
            elif "record" in e:
                rec = v["record"]; good = e["quote"] in rec["summary"] or e["quote"] in (stub.get(rec["summary"], {}).get("body") or "")
            else:
                other += 1; continue
            ok += good; bad += not good
            if not good: fails.append((v["note"], who, e))
    n1 = [e["quote"] for v in t["verdicts"] if v["note"] == 1 for e in v["evidence"] + v["review"]["evidence"]]
    print(f"pass {i}: quotes re-check {ok} ok, {bad} fail, {other} event refs; note 1 evidence: {n1}")
    for f in fails: print("   FAIL", f)
    # records never handed: stub's 61 summaries minus this pass's handed ones
    d = [l for l in open(f"/root/m2/stage-a/pass-{i}.log") if l.startswith("dir ")][0][4:].strip()
    handed = set()
    for b in glob.glob(d + "/dreams/*/bundle.json"):
        j = json.load(open(b))
        if j["dream"]["mode"] == "review":
            handed |= {it["record"]["summary"] for it in j["items"]}
    print("   never handed:", [s[:50] for s in stub if s not in handed])
