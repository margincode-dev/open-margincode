#!/usr/bin/env python3
"""Records the website's replay demo (site/demo.html) from a real daemon.

Drives a fresh scholion daemon through a small multi-agent story on a toy `shop` repo, then writes
every command, caption and (trimmed) daemon response to site/demo-replay.json. The page only plays
that file back, so what it shows is what the daemon returned.

    # a daemon on an empty database, with its workspaces visible to this script:
    AV_DATABASE_URL=... AV_WS_ROOT=/tmp/avdemo/ws AV_STORE_ROOT=/tmp/avdemo/store scholion-daemon &
    scripts/demo-replay.py                    # AV_URL defaults to http://127.0.0.1:7420

Standard library only. Re-run it after a change to packs or the API, and check the captions still
describe what the new output shows.
"""
import json
import os
import pathlib
import re
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.request

BASE = os.environ.get("AV_URL", "http://127.0.0.1:7420").rstrip("/")
OUT = pathlib.Path(__file__).resolve().parent.parent / "site" / "demo-replay.json"
MAX_OUT = 3200  # characters of daemon output kept per step

FILES = {
    "shop/pricing.py": '''"""Prices, discounts and totals for a cart."""
from decimal import Decimal, ROUND_HALF_UP

CENT = Decimal("0.01")


def line_total(price: Decimal, qty: int) -> Decimal:
    return price * qty


def total(lines: list[tuple[Decimal, int]]) -> Decimal:
    """Sum of all lines, rounded once to the cent."""
    subtotal = sum((line_total(p, q) for p, q in lines), Decimal(0))
    return subtotal.quantize(CENT, rounding=ROUND_HALF_UP)
''',
    "shop/cart.py": '''from decimal import Decimal

from pricing import total


class Cart:
    def __init__(self):
        self.lines: list[tuple[Decimal, int]] = []

    def add(self, price: Decimal, qty: int = 1):
        self.lines.append((price, qty))

    def checkout_total(self) -> Decimal:
        return total(self.lines)
''',
    "shop/receipt.py": '''from cart import Cart


def render(cart: Cart) -> str:
    rows = [f"{q} x {p}" for p, q in cart.lines]
    rows.append(f"TOTAL {cart.checkout_total()}")
    return "\\n".join(rows)
''',
    "shop/tests/test_pricing.py": '''from decimal import Decimal

from pricing import total


def test_total_rounds_once():
    assert total([(Decimal("0.333"), 3)]) == Decimal("1.00")
''',
}


def fixture_repo() -> str:
    d = tempfile.mkdtemp(prefix="shop-")
    for rel, text in FILES.items():
        p = pathlib.Path(d, rel)
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(text)
    git = ["git", "-c", "user.name=demo", "-c", "user.email=demo@example.com", "-c", "commit.gpgsign=false"]
    subprocess.run(["git", "init", "-q", "-b", "main", d], check=True)
    subprocess.run([*git, "-C", d, "add", "-A"], check=True)
    subprocess.run([*git, "-C", d, "commit", "-qm", "shop: carts and totals"], check=True)
    return d


def call(method, path, body=None, gen=None):
    data = json.dumps(body).encode() if body is not None else None
    req = urllib.request.Request(BASE + path, data=data, method=method)
    if data:
        req.add_header("content-type", "application/json")
    if gen is not None:
        req.add_header("x-av-generation", str(gen))
    if os.environ.get("AV_TOKEN"):
        req.add_header("authorization", "Bearer " + os.environ["AV_TOKEN"])
    try:
        with urllib.request.urlopen(req) as r:
            return r.status, json.loads(r.read() or b"null")
    except urllib.error.HTTPError as e:
        return e.code, json.loads(e.read() or b"null")


# ---- what the page shows: a board of tasks and the records ("margin") of each ----

tasks: dict[str, dict] = {}  # full id -> {id, title, status, owner, records: [...]}
steps: list[dict] = []
ws_root = ""


def tidy(text: str) -> str:
    """Host paths and long ids are noise on a web page."""
    if ws_root:
        text = text.replace(ws_root, "~/ws")
    return re.sub(r"\b([0-9a-f]{8})[0-9a-f]{32}\b", r"\1", text)


def clip(text: str) -> str:
    text = tidy(text)
    return text if len(text) <= MAX_OUT else text[:MAX_OUT].rstrip() + "\n… (trimmed for the page)"


def add_record(task_id, kind, summary, agent, anchors=(), rid=None):
    tasks[task_id]["records"].append({
        "id": (rid or "")[:8], "kind": kind, "summary": summary, "agent": agent,
        "anchors": [a["path"] + (":" + a["symbol"] if a.get("symbol") else "") for a in anchors],
    })


def step(actor, say, cli, method, path, body=None, gen=None, task=None, note=None, show=None, expect=()):
    """`expect`: text the output must contain, so a caption never claims what the daemon did not do."""
    status, resp = call(method, path, body, gen)
    if status != 200:
        sys.exit(f"{method} {path} -> {status}: {json.dumps(resp)[:600]}")
    out = show(resp) if show else json.dumps(resp, indent=1)
    for want in expect:
        if want not in out:
            sys.exit(f"{cli}: the caption relies on {want!r}, which is not in the output:\n{out}")
    steps.append({
        "actor": actor, "say": say, "cli": cli, "note": note, "task": task and task[:8],
        "out": clip(out),
        "board": json.loads(json.dumps(list(tasks.values()))),
    })
    print(f"ok {actor:7} {cli}")
    time.sleep(0.3)
    return resp


def started(resp, actor):
    global ws_root
    t = resp["task"]
    ws_root = ws_root or str(pathlib.Path(t["workspace_path"]).parent.parent)
    tasks[t["id"]] = {"id": t["id"][:8], "title": t["title"], "status": t["status"], "owner": actor,
                      "records": []}
    add_record(t["id"], "intent", t["goal"], actor)
    return t


def start(actor, say, title, goal, session, budget, note=None, expect=()):
    def show(r):
        started(r, actor)
        return f"task: {r['task']['slug']}\nworkspace: {r['task']['workspace_path']}\n\n{r['pack']['markdown']}"

    r = step(actor, say, f'av task start --title "{title}"', "POST", "/v1/tasks",
             {"repo": "shop", "title": title, "goal": goal, "agent": actor, "session": session, "budget": budget},
             note=note, show=show, expect=expect)
    steps[-1]["task"] = r["task"]["id"][:8]
    return r["task"]


def record(actor, say, task, gen, kind, summary, body="", anchors=(), note=None):
    anchor_flags = "".join(f" --anchor {a['path']}" + (f":{a['symbol']}" if a.get("symbol") else "") for a in anchors)
    short = summary if len(summary) <= 48 else summary[:45] + "…"

    def show(r):
        add_record(task["id"], kind, summary, actor, anchors, r["id"])
        where = f" → {', '.join(a['path'] + (':' + a['symbol'] if a.get('symbol') else '') for a in anchors)}" if anchors else ""
        return f"rec:{r['id'][:8]} {kind}{where}\n{summary}" + (f"\n\n{body}" if body else "")

    return step(actor, say, f'av record --kind {kind}{anchor_flags} "{short}"', "POST",
                f"/v1/tasks/{task['id']}/records",
                {"kind": kind, "summary": summary, "body": body, "agent": actor, "anchors": list(anchors)},
                gen=gen, task=task["id"], note=note, show=show)


def checkpoint(actor, say, task, gen, files):
    for rel, text in files.items():
        pathlib.Path(task["workspace_path"], rel).write_text(text)
    return step(actor, say, "av checkpoint", "POST", f"/v1/tasks/{task['id']}/checkpoint", {}, gen=gen,
                task=task["id"],
                show=lambda r: f"head {r['head_change'][:8]}\nchanged: {', '.join(r['changed'])}")


def set_status(task, status, owner=None):
    tasks[task["id"]]["status"] = status
    if owner:
        tasks[task["id"]]["owner"] = owner


def main():
    src = fixture_repo()
    step("human", "Import a repository. Scholion indexes it with tree-sitter: symbols, references, skeletons.",
         "av repo add shop ./shop", "POST", "/v1/repos", {"name": "shop", "git_source": src},
         show=lambda r: f"repo: {r['name']}\nindexed files: {r['indexed_files']}\nmain: {r['main_change'][:8]}")

    # ---- Task 1: claude decides, hits a dead end, hands off ----
    t1 = start("claude", "Claude starts a task. It gets its own jj workspace and a first context pack.",
               "Bulk discount on cart totals",
               "Orders of 10+ units of one item get 5% off that line. Totals must still match invoices to the cent.",
               "claude-1", 2000)
    g1 = t1["generation"]
    record("claude", "A decision, anchored to the function it is about.", t1, g1, "decision",
           "Apply the bulk discount in pricing.line_total, not in Cart: invoices call pricing directly",
           "billing/invoice.py imports pricing.total and never builds a Cart, so a Cart-level discount would "
           "make invoices and checkout disagree.", [{"path": "shop/pricing.py", "symbol": "line_total"}])
    checkpoint("claude", "Claude edits with its usual tools. A checkpoint snapshots the workspace and re-indexes.",
               t1, g1, {"shop/pricing.py": FILES["shop/pricing.py"].replace(
                   'CENT = Decimal("0.01")\n',
                   'CENT = Decimal("0.01")\nBULK_QTY = 10\nBULK_RATE = Decimal("0.05")\n').replace(
                   "    return price * qty\n",
                   "    gross = price * qty\n    if qty >= BULK_QTY:\n"
                   "        gross -= (gross * BULK_RATE).quantize(CENT, rounding=ROUND_HALF_UP)\n"
                   "    return gross\n")})
    record("claude", "A dead end goes on the record too, so no agent tries it again.", t1, g1, "attempt",
           "Tried rounding every line to the cent before summing: totals drift 1-2 cents from invoices",
           "test_total_rounds_once fails: 3 x 0.333 gives 0.99 instead of 1.00. Round only the discount "
           "amount, and the total once.", [{"path": "shop/pricing.py", "symbol": "total"}],
           note="Git would keep the code and lose this. Scholion keeps both, anchored to pricing.py:total.")
    remaining = "Tests for the 10-unit threshold; show the discount on receipts (separate task)."

    def handed(r):
        set_status(t1, r["status"])
        add_record(t1["id"], "handoff", remaining, "claude", rid=r["record"])
        return f"status: {r['status']}\n\n{r['bundle']['markdown']}"

    step("claude", "Claude's context window is nearly full. It hands off: what is done, what remains, what is open.",
         'av handoff --done "…" --remaining "…"', "POST", f"/v1/tasks/{t1['id']}/handoff",
         {"agent": "claude", "done": "Discount in line_total, rounded on the discount amount only.",
          "remaining": remaining, "broken": "",
          "questions": "Should the threshold be per line or per order? Spec says per line."},
         gen=g1, task=t1["id"], show=handed)

    # ---- codex takes over ----
    def claimed(r):
        set_status(t1, r["task"]["status"], "codex")
        return f"owner: codex · generation {r['generation']} (claude's writes are now fenced off)\n\n{r['bundle']['markdown']}"

    c = step("codex", "Codex claims the task. The claim bumps the generation, so a late write from Claude is refused.",
             "AV_AGENT=codex av claim bulk-discount-on-cart-totals", "POST", f"/v1/tasks/{t1['id']}/claim",
             {"agent": "codex", "budget": 2000}, task=t1["id"], show=claimed)
    g2 = c["generation"]
    step("codex", "Codex asks for context. No LLM picks it: seeds, scoring and a token budget do.",
         "av context --budget 2000", "GET", f"/v1/tasks/{t1['id']}/context?budget=2000&session={c['session']}",
         task=t1["id"], show=lambda r: r["markdown"],
         expect=("decision: Apply the bulk discount", "attempt: Tried rounding", "changed since recorded"),
         note="Codex never re-read the repo. Claude's decision and dead end come first, then only the code "
              "they touch, well inside the 2,000-token budget. The decision is flagged \"changed since "
              "recorded\" because Claude edited line_total after writing it.")
    checkpoint("codex", "Codex adds the missing tests.", t1, g2, {"shop/tests/test_pricing.py": FILES[
        "shop/tests/test_pricing.py"].replace("import total", "import line_total, total") +
        '\n\ndef test_bulk_discount_starts_at_ten():\n    assert line_total(Decimal("2.00"), 9) == Decimal("18.00")\n'
        '    assert line_total(Decimal("2.00"), 10) == Decimal("19.00")\n'})
    record("codex", "Codex answers Claude's open question on the record.", t1, g2, "note",
           "Threshold is per line (10 of one item), as the goal says; per-order would need Cart",
           anchors=[{"path": "shop/pricing.py", "symbol": "line_total"}])

    def validated(r):
        add_record(t1["id"], "validation", "`pytest -q` exit 0", "codex", rid=r["record"])
        return f"pytest -q → exit 0 ({'passed' if r['passed'] else 'failed'})\nbound to head {r['head'][:8]}"

    step("codex", "The test run is recorded against this exact head. An approval needs a passing one.",
         "av validate -- pytest -q", "POST", f"/v1/tasks/{t1['id']}/validate",
         {"agent": "codex", "command": "pytest -q", "exit_code": 0, "output_tail": "2 passed in 0.02s"},
         gen=g2, task=t1["id"], show=validated)

    def submitted(task):
        def show(r):
            set_status(task, r["status"])
            return f"status: {r['status']}\nhead: {r['head'][:8]}\nchanged: {', '.join(r['changed'])}"
        return show

    step("codex", "Submit for review. Scholion refuses a task that has no recorded why.", "av submit", "POST",
         f"/v1/tasks/{t1['id']}/submit", {"agent": "codex"}, gen=g2, task=t1["id"], show=submitted(t1),
         expect=("status: in_review",))

    def reviewed(r):
        set_status(t1, r["status"])
        add_record(t1["id"], "review", "approve", "gemini")
        return f"verdict: {r['verdict']} at head {r['head'][:8]}\nstatus: {r['status']}"

    step("gemini", "A different agent reviews. An owner cannot approve its own task.",
         "AV_AGENT=gemini av review --verdict approve", "POST", f"/v1/tasks/{t1['id']}/review",
         {"reviewer": "gemini", "verdict": "approve", "findings": []}, task=t1["id"], show=reviewed)

    def merged(r):
        set_status(t1, r["status"])
        return f"status: {r['status']}\nmain → {r['main'][:8]}\nchanged: {', '.join(r['changed'])}"

    step("codex", "Merge. Main only ever advances this way, and the records stay attached to the code.",
         "av merge", "POST", f"/v1/tasks/{t1['id']}/merge", {"agent": "codex"}, gen=g2, task=t1["id"],
         show=merged)

    # ---- more agents, more tasks ----
    t2 = start("gemini", "Meanwhile Gemini starts a follow-up in its own workspace.",
               "Show the bulk discount on receipts", "Receipts list a 'bulk -5%' line under each discounted item.",
               "gemini-1", 2000)
    checkpoint("gemini", "Work in progress; nothing it does touches Codex's workspace.", t2, t2["generation"],
               {"shop/receipt.py": FILES["shop/receipt.py"].replace(
                   "from cart import Cart\n", "from cart import Cart\nfrom pricing import BULK_QTY\n").replace(
                   '    rows = [f"{q} x {p}" for p, q in cart.lines]\n',
                   '    rows = []\n    for p, q in cart.lines:\n        rows.append(f"{q} x {p}")\n'
                   '        if q >= BULK_QTY:\n            rows.append("  bulk -5%")\n')})
    record("gemini", "Gemini records why it reads the constant instead of hard-coding 10.", t2, t2["generation"],
           "decision", "Receipt reads pricing.BULK_QTY so the threshold has one source of truth",
           anchors=[{"path": "shop/receipt.py", "symbol": "render"}])
    step("gemini", "Gemini asks for context. It only touched receipt.py, but it now uses pricing's constant.",
         "av context --budget 2000", "GET", f"/v1/tasks/{t2['id']}/context?budget=2000&session=gemini-1",
         task=t2["id"], show=lambda r: r["markdown"],
         expect=("decision: Apply the bulk discount", "attempt: Tried rounding"),
         note="render uses BULK_QTY, and BULK_QTY is used by line_total: two hops through the reference "
              "graph, so the decision and dead end recorded on pricing.py reach a task in another file.")

    t3 = start("claude", "Claude, in a new session, has another idea about line_total.", "Cache price lookups",
               "Memoize pricing.line_total for repeated carts.", "claude-2", 1000,
               note="Its very first pack already lists what earlier agents decided and ruled out about "
                    "line_total, including the dead end from a task that is now merged.",
               expect=("## Decisions & dead ends", "attempt: Tried rounding"))
    record("claude", "It profiles first and finds nothing to gain. That result becomes a dead end too.", t3,
           t3["generation"], "attempt",
           "Memoizing line_total saves nothing: carts are tiny and Decimal math is ~1us",
           anchors=[{"path": "shop/pricing.py", "symbol": "line_total"}])

    def abandoned(r):
        set_status(t3, r["status"])
        return f"status: {r['status']}"

    step("claude", "Abandoned. The task is gone; the reason stays on line_total for the next agent.", "av abandon",
         "POST", f"/v1/tasks/{t3['id']}/abandon", {"agent": "claude"}, task=t3["id"], show=abandoned)

    t4 = start("codex", "Codex picks up a small fix.", "Reject negative quantities",
               "Cart.add raises ValueError for qty < 1.", "codex-2", 1000)
    checkpoint("codex", "Edit and checkpoint.", t4, t4["generation"], {"shop/cart.py": FILES["shop/cart.py"].replace(
        "        self.lines.append", '        if qty < 1:\n            raise ValueError("quantity must be at least 1")\n'
        "        self.lines.append")})
    record("codex", "Why it raises instead of clamping.", t4, t4["generation"], "decision",
           "Raise on qty < 1 instead of clamping: a silent clamp hid a bad import in billing",
           anchors=[{"path": "shop/cart.py", "symbol": "Cart.add"}])
    step("codex", "Submitted, waiting for a reviewer. Four tasks, three agents, one repository, no shared checkout.",
         "av submit", "POST", f"/v1/tasks/{t4['id']}/submit", {"agent": "codex"}, gen=t4["generation"],
         task=t4["id"], show=submitted(t4))

    OUT.write_text(json.dumps({"repo": "shop", "recorded": time.strftime("%Y-%m-%d"), "steps": steps},
                              indent=1, ensure_ascii=False) + "\n")
    print(f"wrote {OUT} ({len(steps)} steps)")


if __name__ == "__main__":
    main()
