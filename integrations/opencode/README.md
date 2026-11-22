# OpenCode

`av init --for opencode` (in the git repository to add) writes the `av` MCP server into
OpenCode's global config, `$XDG_CONFIG_HOME/opencode/opencode.json` (default
`~/.config/opencode/opencode.json`, on Windows too), keeping the rest of the file and the previous
version as `*.bak`:

```json
{
  "mcp": {
    "servers": {
      "av": {
        "type": "remote",
        "url": "http://127.0.0.1:7420/mcp",
        "headers": {
          "X-Av-Agent": "opencode"
        }
      }
    }
  }
}
```

This is the shape `opencode mcp add av --global --url … --header …` writes in OpenCode 2.x. It is
for 2.x only: 1.x read `mcp.av`, which `av init` leaves alone if it finds one.

**Headers.** `X-Av-Agent: opencode` names the MCP session, so work routes to the agent profile
`opencode`. With `AV_TOKEN` set, `av init` also writes `"Authorization": "Bearer <the token>"`:
the token itself, not `{env:AV_TOKEN}`. OpenCode expands `{env:…}` in its server process, and by
default that is its background service, not the shell you start `opencode` in; a header
`Bearer {env:AV_TOKEN}` reached the daemon as an empty `Bearer`. On Linux and macOS `av init` then
makes the file and its `*.bak` readable by you alone. When the token changes, run
`av init --for opencode` again.

**Files `av init` does not edit.** OpenCode reads `opencode.json` and then `opencode.jsonc` from
the same directory, and for each MCP server the later file wins. When an `opencode.jsonc` is
there, `av init` says so: an `mcp.servers.av` entry in it overrides the one `av init` wrote.
OpenCode also allows comments and trailing commas in `opencode.json`; `av init` cannot keep those,
so it changes nothing in such a file and says why. In either case, add the entry above by hand.

A running OpenCode server notices the changed file by itself within a few seconds; no
`opencode reload` is needed.

**How the model reaches the tools.** The `av` server's instructions reach the model in its system
prompt. OpenCode offers MCP tools through its Code Mode: the model calls the built-in `execute`
tool with code such as `await tools.av.start_task({repo: "demo", title: "…"})`, and can look a
tool up first with `search`.

Put the instructions from [`../agents-instructions.md`](../agents-instructions.md) into the
repository's `AGENTS.md` (OpenCode reads `CLAUDE.md` where a directory has no `AGENTS.md`), or
into `AGENTS.md` in OpenCode's config directory for every project.

## Hooks

None yet: the agent calls `start_task` and the other tools itself. A plugin that runs
`av hook …` from OpenCode's plugin API is the next change. Until then, to get the start pack as a
file before the agent starts and a checkpoint after it exits, run it under
`integrations/generic/av-wrap.sh`.

## Verified

With OpenCode 2.3.2 (`@opencode/cli` from npm) on Linux, on 2026-11-22, against an av 0.0.5
daemon with `AV_TOKEN` set, all in scratch config, data and state directories:

- `av init --for opencode` wrote the entry above with the token in `Authorization`, mode `0600`.
- A private `opencode serve`, started before `av init` and without `AV_TOKEN` in its environment,
  connected: `opencode mcp list` showed `av connected` (on its second call; the first call to a
  fresh server prints no servers). With the token in the file changed to a wrong one, it showed
  `av needs authentication` a few seconds later, and `av connected` again once the file was put
  back, with no restart or reload.
- One `opencode run` against that server, on the free model `opencode/big-pickle`, asked to call
  `start_task`, `record` and `checkpoint`: the model called `execute` twice (a `search`, then
  `tools.av.start_task`, `tools.av.record` and `tools.av.checkpoint`). The daemon then showed the
  task with owner `opencode`, and its intent and note with author `opencode`.

Seen earlier the same day with OpenCode 2.3.2 on Windows, against a stand-in MCP server: the
server's instructions in the system prompt, the `clientInfo` name `opencode`, the global and
project `AGENTS.md` loaded, and the empty `Bearer` from `{env:AV_TOKEN}` under the background
service. Not checked: `CLAUDE.md` as a fallback (from OpenCode's docs), OpenCode on macOS, and a
session that edits files in the task's workspace.

The file names and their order are from OpenCode's source at tag `v2.3.2`
(`packages/core/src/config/discovery.ts`, `config.ts`, `config/plugin/mcp.ts`). The writer and
its tests are in `crates/margincode-cli/src/agents.rs`. The CLI test
`opencode_is_wired_through_its_global_config_and_named_by_its_header`
(`crates/margincode-cli/tests/cli.rs`) runs `av init --for opencode`, reads the URL and headers
from the file, and starts a task over MCP under a stand-in client name; the task's owner is
`opencode`, from the header. `agents_wired_by_init_can_start_record_checkpoint_and_submit` does the
same against a daemon with a token.
