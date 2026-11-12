"""Where does each task's lesson rank among its repo's records by embedding similarity to the goal?

Usage: semrank.py FIRST_PACKS_JSONL TASKS_DIR RUN_ID
Reads the bench Qdrant and TEI directly; changes nothing.
"""
import json
import sys
import tomllib
import urllib.request

QDRANT = "http://lp-qdrant:6333"
TEI = "http://lp-tei:80"


def post(url, body):
    req = urllib.request.Request(url, json.dumps(body).encode(), {"content-type": "application/json"})
    with urllib.request.urlopen(req, timeout=60) as r:
        return json.load(r)


rows = [json.loads(line) for line in open(sys.argv[1], encoding="utf-8")]
tasks_dir, run_id = sys.argv[2], sys.argv[3]
first = True
for row in rows:
    name = row["task"]
    spec = tomllib.load(open(f"{tasks_dir}/{name.removeprefix('relay-')}.toml", "rb"))
    repo = f"b{run_id}-{name}-av-1"
    vec = post(f"{TEI}/embed", {"inputs": spec["goal"]})[0]
    hits = post(
        f"{QDRANT}/collections/records/points/search",
        {
            "vector": vec,
            "limit": 600,
            "with_payload": True,
            "filter": {"must": [{"key": "repo", "match": {"value": repo}}]},
        },
    )["result"]
    if first and hits:
        print("payload keys:", sorted(hits[0]["payload"].keys()), file=sys.stderr)
        first = False
    want = row["lesson"].removeprefix("rec:")
    rank = next(
        (i + 1 for i, h in enumerate(hits) if str(h["payload"].get("record_id", h["id"])).replace("-", "").startswith(want)),
        None,
    )
    score = hits[rank - 1]["score"] if rank else None
    top = hits[0]["score"] if hits else None
    eighth = hits[7]["score"] if len(hits) > 7 else None
    print(f"{name}: lesson rank {rank} of {len(hits)}, sim {score}, top {top}, 8th {eighth}")
