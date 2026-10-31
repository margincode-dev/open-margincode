# xAI Grok Build

`av init --for grok` (in the git repository to add) writes the `av` MCP server into `~/.grok/config.toml`,
keeping the rest of the file and the previous version as `*.bak`:

```toml
[mcp_servers.av]
url = "http://localhost:7420/mcp"
headers = { Authorization = "Bearer ${AV_TOKEN}" }   # only when the daemon has a token
```

MCP over HTTP (`url`); Grok expands `${VAR}` in headers when it starts. `grok mcp doctor av` checks the connection. Without a token the header line is left out.

There are no hooks: the agent calls the tools itself. The daemon's MCP server instructions (how
to work a task, take the next job and review) reach the model with no setup: Grok Build 1.0.70
puts them in the system prompt verbatim under the server's name (checked 2026-10-31, headless
`grok -p`). It reaches MCP tools through `search_tool`/`use_tool` rather than listing them, so asked
"which av tools do you have" it may say none; a prompt that names `start_task` works. For the
loop (keep taking jobs until there are none), put the text from
[`../agents-instructions.md`](../agents-instructions.md) into `AGENTS.md`, or say it in the prompt. To get the start pack as a file
before the agent starts and a checkpoint after it exits, run it under
`integrations/generic/av-wrap.sh grok`.

The config format was checked against Grok Build 1.0.70 on 2026-10-31 (`grok mcp doctor av`:
handshake `2025-06-18`, 24 tools). A static `headers` table is sent on every request, and
`${AV_TOKEN}` is expanded at start. `av init --for grok` writes `X-Av-Agent = "grok"` (and
`Authorization` when the daemon has a token). Without that header the daemon sees the MCP
`clientInfo.name` `grok-shell-av` (user agent `grok-cli/1.0.70`); a profile can list it as an alias
(`av agent set grok ... --alias grok-shell-av`).

Headless notes from that run, for the benchmark adapter: tools are registered as `av__<name>`;
`--output-format json` reports `usage` and `total_cost_usd` (an estimate; `input_tokens` there is
the uncached portion, while `grok usage <session>` `inputTokens` includes the cache);
`--prompt-json` takes ACP blocks `{"type":"image","mimeType":"image/png","data":"<base64>"}`
(an 8×8 image was dropped; 256×256 was read); a path in the prompt also works; `--add-dir` is
rejected (`a session works only in its cwd`); the MCP session forgets the task when the process
exits, so later calls must pass `task_id`. `--sandbox workspace` did not stop a write outside
that directory. Details: `bench/results/2026-10-31-routing-grok/conformance.md`.
