# Google Gemini CLI

`av init --for gemini` (in the git repository to add) writes the `av` MCP server into `~/.gemini/settings.json`,
keeping the rest of the file and the previous version as `*.bak`:

```json
{
  "mcpServers": {
    "av": {
      "httpUrl": "http://localhost:7420/mcp",
      "headers": { "X-Av-Agent": "gemini", "Authorization": "Bearer ${AV_TOKEN}" }
    }
  }
}
```

MCP over streamable HTTP (`httpUrl`); Gemini expands `${VAR}` in settings, so the file never
holds the token. Without a token the `Authorization` header is left out. The `X-Av-Agent` header
names it `gemini` for routing, whatever name the client gives itself.

**Trust the workspaces folder.** Gemini CLI has folder trust on by default
(`security.folderTrust.enabled`), and in a folder it does not trust it connects to no MCP server,
not even one from your user settings (`gemini mcp list` says "disabled because this folder is
untrusted"). Task workspaces are new folders under `AV_WS_ROOT` (default `~/ws`), so trust that
folder once with "Trust parent folder" in the trust dialog (it is saved in
`~/.gemini/trustedFolders.json`). A headless run in an untrusted folder exits with an error
instead; Gemini's own way past that for one session is `--skip-trust` or
`GEMINI_CLI_TRUST_WORKSPACE=true`.

Put the instructions from [`../agents-instructions.md`](../agents-instructions.md) into
`GEMINI.md` (the default `context.fileName`).

## Hooks

Gemini CLI has hooks in `settings.json` (`hooks.SessionStart`, `BeforeTool`, `AfterTool`,
`BeforeAgent`, `AfterAgent`, `SessionEnd`, `PreCompress` and others; `hooksConfig.enabled`,
default on) and `gemini hooks migrate` to convert Claude Code's. `av init --for gemini` installs
none yet: they have not been run against av's hook commands, which needs a Gemini key. The agent
calls the tools itself; to get the start pack as a file before the agent starts and a checkpoint
after it exits, run it under `integrations/generic/av-wrap.sh gemini`.

## Verified

Against Gemini CLI 0.66.0 (`npm install -g @google/gemini-cli@0.66.0`) on 2026-11-04, in a
container, without a Gemini key, with a logging stub MCP server at the URL above:

- `gemini mcp list` read the file above and connected (`✓ av ... (http) - Connected`), sending
  `X-Av-Agent: gemini` and `Authorization: Bearer <AV_TOKEN>` (the variable expanded) with
  `initialize`, once the folder was trusted; before that it listed the server as disabled.
- Not checked: a model session (tool calls, `GEMINI.md`, hooks), which needs a key. The rest is
  from the CLI's `docs/reference/configuration.md` and `docs/cli/trusted-folders.md` at v0.66.0.

The writer and its tests are in `crates/scholion-cli/src/agents.rs`; the contract test
`agents_wired_by_init_can_start_record_checkpoint_and_submit` (`crates/scholion-cli/tests/cli.rs`)
reads the file as Gemini does and works a task through the real daemon.
