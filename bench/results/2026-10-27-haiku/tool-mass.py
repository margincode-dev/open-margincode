"""Where do tool-result chars go? Per arm: tool kind -> total chars, count; plus output/thinking."""
import json, sys, glob, collections, re
def kind(name, inp):
    if name == "Bash":
        c = inp.get("command", "")
        if "pytest" in c or "python -m" in c: return "Bash:test"
        if re.search(r"\b(grep|rg|find|ls)\b", c): return "Bash:search"
        if re.search(r"\b(cat|head|sed|tail)\b", c): return "Bash:cat"
        return "Bash:other"
    return name
for d in sys.argv[1:]:
    tot = collections.Counter(); cnt = collections.Counter(); n = 0
    think = text = 0
    for p in glob.glob(f"{d}/*/events.jsonl"):
        n += 1; uses = {}
        for l in open(p):
            if not l.startswith("{"): continue
            e = json.loads(l)
            if e.get("type") == "assistant":
                for b in e["message"]["content"]:
                    if b.get("type") == "tool_use": uses[b["id"]] = kind(b["name"], b["input"]); tot["(call input) " + uses[b["id"]]] += len(json.dumps(b["input"]))
                    if b.get("type") == "thinking": think += len(b.get("thinking", ""))
                    if b.get("type") == "text": text += len(b.get("text", ""))
            if e.get("type") == "user" and isinstance(e["message"].get("content"), list):
                for b in e["message"]["content"]:
                    if b.get("type") == "tool_result":
                        k = uses.get(b["tool_use_id"], "?"); c = b.get("content")
                        s = c if isinstance(c, str) else "".join(x.get("text", "") for x in c if isinstance(x, dict))
                        tot[k] += len(s); cnt[k] += 1
    print(f"== {d} ({n} sessions) chars/session")
    for k, v in tot.most_common(): print(f"  {k:28s} {v/n:8.0f}  calls/session {cnt[k]/n:.2f}")
    print(f"  thinking {think/n:.0f}  text {text/n:.0f}")
