# Google Gemini CLI

`av init --for gemini` (in the git repository to add) writes the `av` MCP server into `~/.gemini/settings.json`,
keeping the rest of the file and the previous version as `*.bak`:

```json
{
  "mcpServers": {
    "av": {
      "httpUrl": "http://localhost:7420/mcp",
      "headers": { "Authorization": "Bearer ${AV_TOKEN}" }
    }
  }
}
```

MCP over streamable HTTP (`httpUrl`); Gemini expands environment variables in settings. Without a token the header line is left out.

There are no hooks: the agent calls the tools itself. Put the instructions from
[`../agents-instructions.md`](../agents-instructions.md) into `GEMINI.md`. To get the start pack as a file
before the agent starts and a checkpoint after it exits, run it under
`integrations/generic/av-wrap.sh gemini`.

The config format was taken from the vendor's documentation in October 2026 and is checked by unit
tests against fixtures (`crates/scholion-cli/src/agents.rs`), not yet against the real CLI.
