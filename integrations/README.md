# Integrations

How each agent CLI, and GitHub, connects to a Scholion daemon. The details of every command are in
[docs/reference.md](../docs/reference.md#agent-integrations).

| Folder | For | Set up with | What it gives |
|---|---|---|---|
| [`claude-code/`](claude-code/) | Claude Code | `av init` (or `install.sh`, or the plugin in `claude-code/plugin/`) | MCP server and hooks: a pack at session start, other tasks' records when a file is opened, checkpoints, a stop that asks for records |
| [`codex/`](codex/README.md) | OpenAI Codex CLI | `av init --for codex` | MCP server |
| [`gemini/`](gemini/README.md) | Google Gemini CLI | `av init --for gemini` | MCP server |
| [`grok/`](grok/README.md) | xAI Grok Build | `av init --for grok` | MCP server, `SessionStart` and `Stop` hooks |
| [`cursor/`](cursor/README.md) | Cursor Agent and editor | `av init --for cursor` | MCP server |
| [`muse/`](muse/README.md) | Meta Muse Code | `av --agent muse init --for mcp-json --path FILE` | MCP server (settings path unverified); `av next` with `av-wrap.sh` |
| [`generic/`](generic/av-wrap.sh) | any other agent | `generic/av-wrap.sh <command…>` | the pack in `$AV_CONTEXT_FILE`, a checkpoint after the command |
| [`github/`](github/README.md) | GitHub pull requests | `av notes --pr`, or the sample workflow on a self-hosted runner | the task's decisions, dead ends and review findings as one comment on the pull request, updated in place |

[`agents-instructions.md`](agents-instructions.md) is the text to put in an agent's instructions
file (`AGENTS.md`, `GEMINI.md`) so it keeps taking work from the backlog.
