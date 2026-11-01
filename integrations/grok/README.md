# xAI Grok Build

`av init --for grok` (in the git repository to add) writes the `av` MCP server into `~/.grok/config.toml`,
keeping the rest of the file and the previous version as `*.bak`:

```toml
[mcp_servers.av]
url = "http://localhost:7420/mcp"
headers = { Authorization = "Bearer ${AV_TOKEN}" }   # only when the daemon has a token
```

MCP over HTTP (`url`); Grok expands `${VAR}` in headers when it starts. `grok mcp doctor av` checks the connection. Without a token the header line is left out.

The daemon's MCP server instructions (how to work a task, take the next job and review) reach the
model with no setup: Grok Build 1.0.70 puts them in the system prompt verbatim under the server's
name (checked 2026-10-31, headless `grok -p`). It reaches MCP tools through `search_tool`/`use_tool`
rather than listing them, so asked "which av tools do you have" it may say none; a prompt that names
`start_task` works. To get the start pack as a file before the agent starts and a checkpoint after
it exits, run it under `integrations/generic/av-wrap.sh grok`.

## Keep agents busy

`av init --for grok` also installs two hooks in `~/.grok/hooks/av.json` (Grok loads every
`~/.grok/hooks/*.json`). `SessionStart` prints the task pack. `Stop` is the same hook Claude Code
runs: once per session it asks for records of changes no record explains, then, when av has work
that fits this agent, it blocks the stop with `av: <what is waiting> - call start_task with no title`.

One installed command serves both CLIs. The hook acts as `--agent` or `AV_AGENT` when that is set.
Otherwise it acts as `grok` when Grok runs it (`GROK_HOOK_EVENT` or `GROK_SESSION_ID` is in the
environment) and as the Claude default (`agent`, when neither the flag nor `AV_AGENT` is set) when
Claude Code runs it.

Grok also loads `~/.claude/settings.json`. If that file already contains the av hooks, or this
`av init` is installing them too, `av init --for grok` does not write `~/.grok/hooks/av.json`, so
the hook does not run twice. Installing Claude's hooks later deletes that file when it is the one
`av init` wrote.

Grok caps a turn at **8 continuations**. Each Stop block counts, and after 8 the turn ends even
if av still has work. The next prompt starts that counter over, so a long backlog spans turns.
The hook's own cap is `AV_HOOK_MAX_CONTINUES` (default 20 continues per session, counted in
`.av-task`). `Stop` waits up to `AV_HOOK_WAIT_SECS` (default 600; `0` turns the wait off) for work
to appear. That wait sends no model tokens. The hook's timeout is the wait plus 60 seconds: a hook
Grok cancels at its timeout has its output discarded, and Grok's own default for `Stop` is 600
seconds, the same as the default wait, not above it.

### Folder trust

Hooks in `~/.grok/hooks/` and `~/.claude/settings.json` are your own hooks. In an interactive
session they run only after you trust the folder (`/hooks-trust` or `grok --trust`), except in
your home directory and at a filesystem root, which count as trusted. A headless `grok -p` counts
the folder as trusted for those hooks, so this Stop hook runs there without a prompt. Hooks that
live in the project (`.grok/hooks` or `.claude/settings.json` inside the repo) still need the
folder trusted. `av init` installs the user-level file, not a project hook.

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
