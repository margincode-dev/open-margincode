# E-G0. Grok Build as an `av` agent

Date: 2026-10-31. Grok Build 1.0.70 (`grok-cli/1.0.70`), model `grok-4.7` / billed as
`grok-4.7-build`. Daemon: this branch's `target/debug/scholion-daemon`, isolated
`AV_HOME=/tmp/eg0/home`, `http://127.0.0.1:7421`, repo `eg0` (one `hello.py`).
`~/.grok/config.toml` was restored to its pre-run contents afterward, and the daemon was stopped.
No API key was spent. The dollar figures below are the CLI's own `total_cost_usd` estimates
(`costUsdTicks / 1e10`).

## Result

G0 holds for the lifecycle this daemon already has, with the harness facts below. Claim G0's
"claims tasks" part was checked with a direct MCP client, not with a second `grok -p` process.

| Step | Result |
|---|---|
| `av init --for grok` | Wrote `[mcp_servers.av] url = "http://127.0.0.1:7421/mcp"` and left `config.toml.bak`. No header, because this daemon had no token. |
| `grok mcp doctor av` | Healthy. Protocol `2025-06-18`. 24 tools. |
| Static headers | `headers = { Authorization = "Bearer ${AV_TOKEN}", X-Av-Agent = "grok" }` is accepted. A captured request carried `authorization: Bearer probe-token-not-a-secret` and `x-av-agent: grok`. |
| Headless task | `grok -p … --output-format json --always-approve --max-turns 12 --sandbox workspace --cwd /tmp/eg0`. Started `grok-conformance`, edited `hello.py`, recorded the decision, checkpointed. Validate and submit needed two resumes that passed `task_id`. Daemon then showed `in_review`, event `submit`. |
| Fencing | Claude Code (no header) started and handed off generation 1. A client with `X-Av-Agent: grok` and `clientInfo.name` `should-not-win` claimed generation 2; owner became `grok`. Claude's next checkpoint was refused: `claim lost … generation 2, this write was made under 1` (MCP `isError`, and HTTP 422). |

## What the harness has to assume

- **Agent name.** `initialize` sends `clientInfo.name` `grok-shell-av`, version `1.0.70`, user agent `grok-cli/1.0.70`. The daemon prefers `X-Av-Agent`. `av init` does not write that header, so a profile named `grok` does not match unless the header is added (or the profile aliases `grok-shell-av`).
- **Tool names.** The session registers `av__start_task`, `av__record`, and so on. The daemon's tool names are unprefixed.
- **Usage.** `--output-format json` fields: `sessionId`, `num_turns`, `usage.input_tokens` (uncached), `usage.cache_read_input_tokens`, `usage.output_tokens`, `usage.reasoning_tokens`, `usage.total_tokens`, `total_cost_usd`, `modelUsage`. On the image probe, `grok usage` `inputTokens` 21687 = json `input_tokens` 12855 + `cache_read_input_tokens` 8832, and `totalTokens` matched. `grok usage` `turnCount` counts user prompts; `modelCalls` counts model turns. There is no `--max-budget-usd`. Use `--max-turns`.
- **Images.** `--prompt-json` is an array of ACP blocks. An image block is `{"type":"image","mimeType":"image/png","data":"<base64>"}`. `media_type` is rejected (`missing field mimeType`). A nested Anthropic `source` object is rejected (`missing field data`). An 8×8 PNG was dropped ("too small"); a 256×256 solid red PNG was answered "Red" in one turn. A filesystem path in the prompt also worked (two turns, the model read the file). `agentCapabilities.promptCapabilities.image` is true.
- **cwd.** `--add-dir` fails headless: `additionalDirectories is not supported: a session works only in its cwd`. The task workspace has to sit under `--cwd`, or the process has to be allowed to write outside it. `AV_WS_ROOT` for a run should be under that cwd.
- **Task memory.** The first process called `record` and `checkpoint` with no `task_id`. After `grok -r <session>`, the new process's MCP session said `no task: pass task_id, or call start_task or claim_task first`. A resumed process must pass `task_id`.
- **`--always-approve` and the sandbox.** The flag auto-approves every tool. `--sandbox workspace` did not confine writes: with cwd `/tmp/eg0/repo` the model created both `/tmp/eg0/repo/inside.txt` and `/tmp/eg0-outside-write.txt`. Those probe files were deleted. A headless run can write anywhere this user can write.
- **Events.** `GET /v1/events` with no `since` starts at the current head and returns nothing. The oracle is `since=0`.
- **Side effect.** `python3 -c "import hello"` wrote `__pycache__/hello.cpython-314.pyc`, and submit listed it in `changed` next to `hello.py`.

## Tokens and estimated cost

The conformance session `01a1926b-879f-79c3-98fd-e719a81536aa` (`grok usage`): 3 prompts, 17 model calls, input 413259 of which 387200 were cached, output 1975, reasoning 1115, total 415234, estimate **$0.258**. Most of the input is this machine's already-enabled MCP servers and skills, not the two-line repo. Other probes, same estimator: color image $0.031, path image $0.043, 8×8 image (dropped) $0.032, sandbox write $0.059, plus one 1-turn ping in the scholion repo $0.035. About **$0.46** estimated for E-G0.

## Not run

E-G1, E-G2, and E-G4 were not started. The plan spends quota only after `PREREG.md`, and only after the runner adapter, `bench/tasks-routing/`, and `scholion-bench routing-profile` / `routing` exist. None of that is on this branch.

E-G3's scripted reserve-window run was not started either. `scripts/route-demo.sh` is not in the tree. Plan 8's daemon routes and CLI (`av next`, backlog) are still unbuilt; the branch has the routing core and the store (through migration `0022`), not the HTTP API. There is nothing for the scripted agents to pull.
