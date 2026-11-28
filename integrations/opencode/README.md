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

`av init --for opencode` also writes a plugin, `plugins/av.js`, into the same config directory
(`~/.config/opencode/plugins/av.js`), with the path of the `av` that wrote it. OpenCode loads
every file in that folder and reloads a changed one by itself. The plugin runs the same
`av hook …` commands as Claude Code's hooks:

| OpenCode hook | `av hook` | What the session gets |
|---|---|---|
| `prompt`: a session's first prompt, and the first after a compaction | `session-start` | the task's pack, as the session's instruction entry `av-pack`. The entry stays across a restart or a resumed session, so it is put once; a compaction puts a fresh one |
| `tool` `execute.after` on `read` | `pre-read` | other tasks' records about the file, added to what the tool returns |
| `tool` `execute.after` on `edit`, `write` and `patch` | `post-edit` | a checkpoint (at most every 2 s) |
| `experimental.idle`, when the session has no background work left | `stop` | once, a message asking for the records behind unrecorded changes; else queued work (`call start_task with no title`). The message continues the session |

**Where to start it.** The session's directory decides the task: start `opencode` (or
`opencode serve`) in the task's workspace, the directory `av task start` printed. Outside any
task the plugin puts no pack, adds nothing to tool results and does not hold the first prompt.
It still runs `av hook session-start` and `av hook stop` there for the disk footprint check (see
"Disk" in `docs/reference.md`): a session that left a large build or cache directory outside any
workspace gets one `av:` message about it. `AV_FOOTPRINT=off` in the server's environment turns
that off.

**The daemon.** The plugin reads the `av` entry at each call, from `opencode.json` and then
`opencode.jsonc` in that config directory (comments and trailing commas allowed; the later file
wins), and passes its URL and its token to `av hook` in the environment, so the server needs no
`AV_URL` or `AV_TOKEN` of its own. A value written as `{env:…}` or `{file:…}` is left to
OpenCode, and `av hook` then uses the server's own environment. The hooks act as the task's owner,
whoever started the task, unless the server's environment sets `AV_AGENT` (as `av watch` does for
the command it runs); the entry's `X-Av-Agent` names the MCP session only.

**The wait for work.** Claude Code's `Stop` hook waits up to `AV_HOOK_WAIT_SECS` (600 s) for
queued work. OpenCode gives the idle hook no time limit and holds the next prompt you type until
it returns, so the plugin waits 1 s unless the server's environment sets `AV_HOOK_WAIT_SECS`.
Set it only for a headless server: `AV_HOOK_WAIT_SECS=600 opencode serve`. The background
service reads its environment when it starts, not from the shell you run `opencode` in. Under
`av watch`, run `opencode run --standalone …` so that the server is the command's own and gets
the `AV_HOOK_WAIT_SECS=0` `av watch` sets. Interrupting the session while the hook waits stops
the wait, and nothing is sent.

**Subagents.** A subagent's session (one with a parent) gets the `read` and edit hooks, not the
pack or the stop message; its parent gets those.

**What OpenCode does not get.** The session's shell commands do not see the task: Claude Code's
`SessionStart` exports `AV_SCRATCH` and `AV_DOCKER_LABELS` to its Bash commands through
`$CLAUDE_ENV_FILE`, and OpenCode has no such file. The pack names the scratch directory in its
last line. The hook messages say `av record`; in the model's shell that needs `av` on `PATH`
and, with a token, `AV_URL` and `AV_TOKEN` in the server's environment. Otherwise the model
records through the `av` MCP tools.

**Failures.** The plugin never fails a prompt, a tool call or a turn: an `av` that cannot start,
fails, or runs past 60 s (`AV_OPENCODE_HOOK_TIMEOUT_MS`; the stop hook gets its wait on top), or
a daemon that does not answer,
leaves the session as it was. The 60 s is a backstop: `av hook` itself gives up on a daemon that
does not answer within its budget (3 s for `read`, 5 s for an edit, 15 s for the first prompt,
10 s for idle; "When the daemon does not answer" in `docs/reference.md`), and for 30 s after that
the `read` and edit hooks skip the daemon at once. The first problem is written once to the server's standard error,
as `av plugin: …`; `opencode serve --print-logs` shows it among its log lines.

**Removing it.** Delete `plugins/av.js`; `av init --for opencode` writes it again.

`experimental.idle` is marked experimental in OpenCode and may change in a later release.

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
service. Not checked: `CLAUDE.md` as a fallback (from OpenCode's docs) and OpenCode on macOS.

The hooks, the same day, with OpenCode 2.3.2 on Linux, the free model `opencode/big-pickle` and an
av 0.0.5 daemon with `AV_TOKEN` set, against a private `opencode serve` with no av variables in
its environment (the plugin read the daemon's address and token from `opencode.json`):

- **Pack.** Asked to name a word from its task's goal without calling a tool, the model named it.
  The session had one instruction entry, `av-pack`. A session started outside any task had none.
- **Read.** After `read` of a file another task had a record about, the model quoted a word that
  was only in that record.
- **Edit.** While the turn was still running (a shell `sleep` after the edit), `av status` already
  showed the edited file as changed, and `.av-task` had a new `last_checkpoint_ms`.
- **Stop.** With a change no record explained, the session got one `av:` message asking for the
  record and answered it, then stopped. With work posted to the backlog, it got
  `av: work is waiting: … - call start_task with no title`, called `start_task`, did the work and
  submitted it. The model's shell had no `av` on `PATH` and no token, so its `av record` failed and
  it recorded through the MCP `record` tool, passing the task's slug as `task_id`.
- **Interrupt.** With `AV_HOOK_WAIT_SECS=120` in the server's environment and nothing to offer,
  the session was interrupted while `av hook stop` waited: that child was gone at once, and after
  the wait would have ended the session had no new message and had not run again.
- **Subagent.** A subagent's `read` came back with another task's record and its `write` moved
  `last_checkpoint_ms`; the subagent's session had no instruction entry and no `av:` message,
  and the parent got the stop message.
- **Daemon stopped.** The first prompt of a session in a task went through without a pack, after
  `av hook session-start` had given up; the server's output (`--print-logs`) had one
  `av plugin: …` line.
- **Config edited by hand.** With comments and trailing commas in `opencode.json`, and then with
  the entry only in `opencode.jsonc`, a new session got its pack.
- **Compaction.** After `opencode api session.compact`, the session's next prompt got a fresh pack
  (a new pack session in `.av-task`). Automatic compaction was not checked.
- **`--standalone`.** `opencode run --standalone` in the workspace loaded the plugin, and
  `AV_HOOK_WAIT_SECS` in its environment set the wait.
- **Reload.** Running `av init --for opencode` again while the server ran made it load the plugin
  once more, with no restart.

Not checked with the plugin: OpenCode on Windows or macOS, and OpenCode's background service
(every session above ran against a private `opencode serve` or `--standalone`).

The file names and their order are from OpenCode's source at tag `v2.3.2`
(`packages/core/src/config/discovery.ts`, `config.ts`, `config/plugin/mcp.ts`). The writer and
its tests are in `crates/margincode-cli/src/agents.rs`. The CLI test
`opencode_is_wired_through_its_global_config_and_named_by_its_header`
(`crates/margincode-cli/tests/cli.rs`) runs `av init --for opencode`, reads the URL and headers
from the file, and starts a task over MCP under a stand-in client name; the task's owner is
`opencode`, from the header. `agents_wired_by_init_can_start_record_checkpoint_and_submit` does the
same against a daemon with a token.

The plugin's contract tests (`opencode_plugin_*` in `crates/margincode-cli/tests/cli.rs`, Unix
only) load `av.js` under `node` with a stand-in for OpenCode's plugin API
(`crates/margincode-cli/tests/opencode/harness.mjs`, the shapes OpenCode 2.3.2 hands a plugin),
play OpenCode's calls to it against a real daemon and `av`, and check what it puts, adds and
sends. Without `node` on `PATH` they print a `SKIPPED` line and pass.
