# Cursor Agent (CLI)

`av init --for cursor` (in the git repository to add) writes the `av` MCP server into
`~/.cursor/mcp.json`, keeping the rest of the file and the previous version as `*.bak`. The Cursor
editor reads the same file, so it gets the server too.

```json
{
  "mcpServers": {
    "av": {
      "url": "http://localhost:7420/mcp",
      "headers": {
        "X-Av-Agent": "cursor",
        "Authorization": "Bearer ${env:AV_TOKEN}"
      }
    }
  }
}
```

MCP over streamable HTTP (`url`). Cursor expands `${env:NAME}` in headers, so the file never holds
the token; without a token that line is left out. `X-Av-Agent` names it `cursor` for routing
(`av agent set cursor --best … --can … --cost N`).

Checked against `cursor-agent` 2026.10.22 on 2026-10-31:
- `cursor-agent mcp list` shows `av: ready`; `cursor-agent mcp list-tools av` lists the 24 tools.
- The server's instructions reach the model verbatim (as the `av` namespace's instructions), so it
  knows how to work a task, take its next job (`start_task` with no title or goal) and review,
  with no setup.
- A headless run (`-p`) needs the server approved: `cursor-agent mcp enable av` once, or
  `--approve-mcps` per run.

There are no hooks: the agent calls the tools itself. The CLI reads `AGENTS.md` and `CLAUDE.md` at
the project root as rules; to make it keep taking jobs, put the loop from
[`../agents-instructions.md`](../agents-instructions.md) there or in the prompt. To start it only
when work fits it:

```bash
av --agent cursor watch --repo pantry -- cursor-agent -p --approve-mcps --trust "take your next av job"
```

`-p` gives the agent every tool, shell and writes included (see `cursor-agent --help`).
