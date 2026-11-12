# Integrations

How each agent CLI, and GitHub, connects to a Margincode daemon, and against which release of each
agent CLI the wiring was last checked. `av init --for <agent>` writes an agent's config
(`crates/margincode-cli/src/agents.rs`); the details of every command are in
[docs/reference.md](../docs/reference.md#agent-integrations), and what was and was not checked is in
each folder's README.

## Agent CLIs

| Agent | `av init` | Config file | MCP entry | Hooks av installs | Instructions file | Verified |
|---|---|---|---|---|---|---|
| [Claude Code](claude-code/) | `--for claude` (or `claude-code/install.sh`, or the plugin in `claude-code/plugin/`) | `~/.claude/settings.json` | `claude mcp add` | `SessionStart` (a pack), `PreToolUse` (other tasks' records when a file is opened), `PostToolUse` (checkpoints), `Stop` (asks for records) (`claude-code/install.sh`) | `CLAUDE.md` | the CLI the benchmark runs (`bench/`) |
| [OpenAI Codex CLI](codex/README.md) | `--for codex` | `$CODEX_HOME/config.toml` | `[mcp_servers.av]` `url`, `http_headers`, `bearer_token_env_var` | none (Codex has hooks; untrusted until reviewed) | `AGENTS.md` | against CLI 0.166.0 on 2026-11-04 ([codex](codex/README.md)) |
| [Google Gemini CLI](gemini/README.md) | `--for gemini` | `~/.gemini/settings.json` | `mcpServers.av` `httpUrl`, `headers` | none (Gemini has hooks; not run without a key) | `GEMINI.md` | against CLI 0.66.0 on 2026-11-04, config and MCP connection only, no key ([gemini](gemini/README.md)) |
| [xAI Grok Build](grok/README.md) | `--for grok` | `~/.grok/config.toml` | `[mcp_servers.av]` `url`, `headers` | `SessionStart`, `Stop` (`~/.grok/hooks/av.json`) | `AGENTS.md` | against CLI 1.0.70 on 2026-10-31 ([grok](grok/README.md)) |
| [Cursor Agent and editor](cursor/README.md) | `--for cursor` | `~/.cursor/mcp.json` | `mcpServers.av` `url`, `headers` | none | `AGENTS.md` | against `cursor-agent` 2026.10.22 on 2026-10-31 ([cursor](cursor/README.md)) |
| [Meta Muse Code](muse/README.md) | `av --agent muse init --for mcp-json --path FILE` | unverified | `mcpServers.av` (key names unverified) | none; `av next` with `av-wrap.sh` | unverified | `muse --help` only, 2026-10-31 ([muse](muse/README.md)) |
| Any other CLI | `--for mcp-json --path FILE` | yours | `mcpServers.av` `type`, `url`, `headers` | none; wrap it with [`generic/av-wrap.sh`](generic/av-wrap.sh) | yours | n/a |

Agents without av hooks call `start_task` and the other tools themselves; give them the text in
[`agents-instructions.md`](agents-instructions.md), to put in their instructions file (`AGENTS.md`,
`GEMINI.md`) so they keep taking work from the backlog. `generic/av-wrap.sh <command…>` writes the
start pack to `$AV_CONTEXT_FILE` before the agent starts and checkpoints after it exits.

## Dreaming

[`dream/DREAM.md`](dream/DREAM.md) is what an agent started by `av dream` reads: how to check
the records it was handed against the code, and the two files it may write. It is built into the
CLI, which writes it beside the run's `bundle.json`; a change to it changes the
`instructions_sha` stored on every later run ([reference](../docs/reference.md#a-second-look-at-records-dreams-and-verdicts)).

## GitHub

| Folder | For | Set up with | What it gives |
|---|---|---|---|
| [`github/`](github/README.md) | GitHub pull requests | `av notes --pr`, or the sample workflow on a self-hosted runner | the task's decisions, dead ends and review findings as one comment on the pull request, updated in place |
