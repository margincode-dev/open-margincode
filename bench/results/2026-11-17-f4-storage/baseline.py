"""F4 baseline: five task starts of open-margincode on the dogfood daemon.

Per run: wall time of `av task start` (client), the start's `workspace:ready` event (ms, overlay_ms,
files, bytes, count_ms), and the wall time of `av checkpoint` after changing three files. Each task
is abandoned afterwards. Raw lines go to stdout as JSON, one per run.

Usage: python baseline.py [runs] [dir to start from]; AV_URL names the daemon.
"""
import json
import os
import re
import statistics
import subprocess
import sys
import time
import urllib.request

sys.stdout.reconfigure(encoding="utf-8")
AV = "av.exe"
URL = os.environ.get("AV_URL", "http://127.0.0.1:7420")
REPO = "open-margincode"
RUNS = int(sys.argv[1]) if len(sys.argv) > 1 else 5
CWD = sys.argv[2] if len(sys.argv) > 2 else "."
env = dict(os.environ, AV_AGENT="claude-opus")


def av(args, cwd=None):
    return subprocess.run([AV, *args], cwd=cwd, env=env, capture_output=True, text=True,
                          encoding="utf-8", errors="replace")


def get(path):
    with urllib.request.urlopen(f"{URL}{path}") as r:
        return json.load(r)


def head():
    return get(f"/v1/events?repo={REPO}")["next"]


def events_for(task_id, since):
    body = get(f"/v1/events?repo={REPO}&since={since}&limit=500")
    return [e for e in body["events"] if e.get("task_id") == task_id]


rows = []
for i in range(1, RUNS + 1):
    since = head()
    t0 = time.monotonic()
    out = av(["task", "start", "--repo", REPO, "--title", f"F4 baseline run {i}",
              "--goal", "F4 baseline: measure one start, then abandon (no change kept)"],
             cwd=CWD)
    start_s = time.monotonic() - t0
    if out.returncode != 0:
        print(json.dumps({"run": i, "error": out.stderr.strip()[-400:]}), flush=True)
        continue
    ws = re.search(r"^workspace: (.+)$", out.stdout, re.M).group(1).strip()
    task = re.search(r"^task: ([0-9a-f-]{36})", out.stdout, re.M).group(1)
    ready = [e for e in events_for(task, since) if e["kind"] == "workspace:ready"]
    p = ready[0]["payload"] if ready else {}
    for name in ("CHANGELOG.md", "README.md", "CONTRIBUTING.md"):
        with open(os.path.join(ws, name), "a", encoding="utf-8", newline="") as f:
            f.write("\nF4 baseline edit\n")
    t1 = time.monotonic()
    ck = av(["checkpoint"], cwd=ws)
    checkpoint_s = time.monotonic() - t1
    ab = av(["abandon"], cwd=ws)
    row = {"run": i, "task": task, "start_s": round(start_s, 2), "ms": p.get("ms"),
           "overlay_ms": p.get("overlay_ms"), "count_ms": p.get("count_ms"),
           "files": p.get("files"), "bytes": p.get("bytes"),
           "checkpoint_s": round(checkpoint_s, 2), "checkpoint_ok": ck.returncode == 0,
           "abandoned": ab.returncode == 0}
    if ck.returncode != 0:
        row["checkpoint_err"] = ck.stderr.strip()[-300:]
    if ab.returncode != 0:
        row["abandon_err"] = ab.stderr.strip()[-300:]
    rows.append(row)
    print(json.dumps(row), flush=True)

ok = [r for r in rows if r.get("ms") is not None]
if ok:
    summary = {}
    for k in ("start_s", "ms", "overlay_ms", "count_ms", "files", "bytes", "checkpoint_s"):
        vals = [r[k] for r in ok]
        summary[k] = {"median": statistics.median(vals), "worst": max(vals)}
    print(json.dumps({"summary": summary, "runs": len(ok)}), flush=True)
