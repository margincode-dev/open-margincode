# OpenAI Codex CLI

`av init --for codex` (in the git repository to add) writes the `av` MCP server into `$CODEX_HOME/config.toml` (default `~/.codex/config.toml`),
keeping the rest of the file and the previous version as `*.bak`:

```toml
[mcp_servers.av]
url = "http://localhost:7420/mcp"
http_headers = { X-Av-Agent = "codex" }
bearer_token_env_var = "AV_TOKEN"   # only when the daemon has a token
```

MCP over streamable HTTP; the token comes from the environment variable `bearer_token_env_var`
names, so the file never holds it. Codex introduces itself as `codex-mcp-client`, so the
`X-Av-Agent` header is what makes it route, own tasks and review as `codex`.

Put the instructions from [`../agents-instructions.md`](../agents-instructions.md) into
`AGENTS.md`. Codex reads `AGENTS.md`; in one `codex exec` check the model had not seen the MCP
server's own instructions, so for Codex the file is not optional.

## Hooks

Codex has lifecycle hooks (`SessionStart`, `PreToolUse`, `PostToolUse`, `Stop`,
`UserPromptSubmit`, `SessionEnd`, `PreCompact` and others), in `$CODEX_HOME/hooks.json` or a
`[hooks]` table in `config.toml`. A hook you add runs only after you trust it (the `/hooks` review
in the TUI); an untrusted `hooks.json` did not fire in `codex exec`. `av init --for codex`
installs none yet: av's hook commands read Claude Code's hook payloads, and Codex's have not been
checked against them. Codex also runs the program in `notify = [...]` after each turn, with a JSON
argument (`{"type": "agent-turn-complete", "cwd": ..., "last-assistant-message": ...}`).

Until then the agent calls the tools itself. To get the start pack as a file before the agent
starts and a checkpoint after it exits, run it under `integrations/generic/av-wrap.sh codex`.

## Verified

Against Codex CLI 0.166.0 (`npm install -g @openai/codex@0.166.0`, release `rust-v0.166.0`) on
2026-11-04, in a container, with a logging stub MCP server at the URL above:

- `codex mcp get av --json` parsed the file as `streamable_http` with the URL, the header and
  `bearer_token_env_var = "AV_TOKEN"`.
- In a `codex exec` session (on DeepSeek through a custom model provider) Codex sent `initialize`
  with `Authorization: Bearer <AV_TOKEN>` and `X-Av-Agent: codex`, clientInfo
  `codex-mcp-client`, then `tools/list`.
- It followed an instruction in `AGENTS.md`; asked for a word given only in the server's
  `instructions`, it had none.
- `notify` ran after the turn; the untrusted `hooks.json` did not.

The writer and its tests are in `crates/scholion-cli/src/agents.rs`; the contract test
`agents_wired_by_init_can_start_record_checkpoint_and_submit` (`crates/scholion-cli/tests/cli.rs`)
reads the file as Codex does and works a task through the real daemon.
