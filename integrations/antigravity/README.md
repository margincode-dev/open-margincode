# Google Antigravity CLI (`agy`)

`av init --for antigravity` (in the git repository to add) writes the `av` MCP server into
`~/.gemini/config/mcp_config.json`, Antigravity's global MCP config, keeping the rest of the file
and the previous version as `*.bak`:

```json
{
  "mcpServers": {
    "av": {
      "url": "http://127.0.0.1:7420/mcp"
    }
  }
}
```

MCP over streamable HTTP (`url`). Antigravity ships that file empty (0 bytes); `av init` reads an
empty or blank file as `{}`, as it does for every agent's config.

**No headers.** Antigravity's docs (`mcp_servers.md`) give an HTTP server `url` (or the legacy
`serverUrl`) and nothing else: no `headers` and no `${VAR}` expansion. So the file cannot do two
things the other agents' files do:

- **Name it.** Without an `X-Av-Agent` header the daemon names the MCP session by the
  `clientInfo.name` Antigravity sends in `initialize`, which has not been seen here. To route work
  to it, read that name once from its first task (its owner on the dashboard, or `owner_agent` in
  the answer of the `status` tool), then give it a profile under the name you want:
  `av agent set antigravity --best … --can … --cost N --alias <that name>`.
- **Carry the token.** With `AV_TOKEN` set, the daemon refuses every request without
  `Authorization: Bearer …`, and Antigravity cannot send one; `av init` says so. Use it with a
  daemon without a token (the default, listening on `127.0.0.1`).

The daemon reads no query parameter on `/mcp` either, so `url` cannot name it today.

Put the instructions from [`../agents-instructions.md`](../agents-instructions.md) into the
repository's `AGENTS.md` (or `GEMINI.md`). Antigravity loads those files from the working
directory up to the repository root (`rules.md`, at most 24,000 bytes each); committed, they are in
every task workspace. Not checked: where that walk stops in a task workspace, which is a jj
workspace without `.git`. The file in the workspace's own root is found either way.

## Hooks

Antigravity has hooks in `.agents/hooks.json` (`SessionStart`, `PreToolUse`, `PostToolUse`,
`PreInvocation`, `PostInvocation`, `Stop`), with JSON on stdin and JSON on stdout. Its events are
close to Claude Code's, but every output differs from what `av hook …` prints, so
`av init --for antigravity` installs none. Do not copy Claude Code's hooks into `hooks.json`:
they would run, and Antigravity would not read what they print.

| Antigravity event | av command | What differs |
|---|---|---|
| `SessionStart` | `av hook session-start` | av prints the pack as plain text, then lines about `$AV_SCRATCH` and containers; Antigravity reads stdout as `{"injectSteps": [{"systemMessage": {"systemMessage": "…"}}]}`. Its input has `conversationId`, not `session_id`, and no `source` (av starts a fresh pack session only on `startup`, `compact` or `clear`). av exports `$AV_SCRATCH` through `$CLAUDE_ENV_FILE`, which Antigravity has no counterpart for. |
| `PreToolUse` | `av hook pre-read` | The output needs a permission `decision` (`allow`, `deny`, `ask`, `force_ask`); its only text is the `reason` that goes with it. To hand over the records about a file, av would have to allow every read itself (over the user's permission settings) or ask on each one. The file is in `toolCall.args`, under a key the docs do not give for its file tools (they show only `run_command`'s `CommandLine`); av reads `tool_input.file_path`. |
| `PostToolUse` | `av hook post-edit` | Antigravity expects `{}`; av prints nothing, or `hookSpecificOutput.additionalContext` (another task's notices, a claim taken over). The docs do not name its edit tools, so the matcher would have to be `*`. The checkpoint itself would work: av finds the task from `cwd`, which Antigravity sends. |
| `Stop` | `av hook stop` | av blocks a stop with `{"decision": "block"}`; Antigravity carries on only on `"continue"` and stops on any other value. The record nudge would mark the session as asked and the wait for work would count a continue while the agent stops anyway. Its `continuationActive` is av's `stop_hook_active`, its `conversationId` av's `session_id`. |
| `PreInvocation`, `PostInvocation` | none | av has no hook for these. |

For every event:

- Antigravity runs a hook with `cmd /c` on Windows, where the `command -v av …; exit 0` guard
  av writes for the other agents is not valid.
- A hook's default timeout is 30 s; the Stop hook waits for work for up to `AV_HOOK_WAIT_SECS`
  (600 s). `timeout` can be set per hook.
- The docs name `.agents/hooks.json` in the project. A file `av init` wrote into the repository
  would reach task workspaces only once committed; whether `~/.gemini/config/hooks.json` is read
  too, they do not say.

What it would take is an Antigravity mode in `av hook` (`docs/backlog.md`, "Antigravity's hooks
need their own output"). Until then the agent calls `start_task` and the other tools itself; to
get the start pack as a file before the agent starts and a checkpoint after it exits, run it under
`integrations/generic/av-wrap.sh`.

## Verified

Not run: `agy` was never started here (it spends the owner's Google quota). Everything above is
from the docs shipped with the Antigravity CLI (`builtin/skills/agy-customizations/docs/`:
`mcp_servers.md`, `hooks.md`, `rules.md`, `json_configs.md`) in the install of 2026-11-19, whose
language server logged version 1.5.13; the 0-byte `mcp_config.json` is that install's.

The writer and its tests are in `crates/margincode-cli/src/agents.rs`. The CLI test
`antigravity_is_wired_into_its_empty_config_and_named_by_its_client`
(`crates/margincode-cli/tests/cli.rs`) runs `av init --for antigravity` on an empty file, reads
the URL from it as Antigravity does, and starts a task over MCP under a stand-in client name,
which becomes the task's owner.
