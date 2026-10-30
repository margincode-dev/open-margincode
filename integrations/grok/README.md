# xAI Grok Build

`av init --for grok` (in the git repository to add) writes the `av` MCP server into `~/.grok/config.toml`,
keeping the rest of the file and the previous version as `*.bak`:

```toml
[mcp_servers.av]
url = "http://localhost:7420/mcp"
headers = { Authorization = "Bearer ${AV_TOKEN}" }   # only when the daemon has a token
```

MCP over HTTP (`url`); Grok expands `${VAR}` in headers when it starts. `grok mcp doctor av` checks the connection. Without a token the header line is left out.

There are no hooks: the agent calls the tools itself. Put the instructions from
[`../agents-instructions.md`](../agents-instructions.md) into `AGENTS.md`. To get the start pack as a file
before the agent starts and a checkpoint after it exits, run it under
`integrations/generic/av-wrap.sh grok`.

The config format was taken from the vendor's documentation in October 2026 and is checked by unit
tests against fixtures (`crates/scholion-cli/src/agents.rs`), not yet against the real CLI.
