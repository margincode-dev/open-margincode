# Meta Muse Code

Install (from Meta): `irm https://dev.meta.ai/install.ps1 | iex` on Windows. The `muse` command was
also found at `~/.local/bin/muse` inside WSL on 2026-10-31 (Muse Code, checked with `muse --help`).
Log in once with `muse login`; `muse usage` shows your subscription allowance.

**There is no `--for muse`.** Muse's settings path and key names are still unverified, and this
repository does not invent a dedicated writer for a file it has not seen work (Plan 8, Task 8).
Once you know the path:

```bash
av --agent muse init --for mcp-json --path PATH/TO/settings.json
```

That merges `mcpServers.av` as `{ "type": "http", "url": "…/mcp", "headers": { "X-Av-Agent": "muse" } }`
(and `Authorization: Bearer ${AV_TOKEN}` when the daemon has a token). If Muse wants different
keys, edit the entry. The example further down is still unchecked.

## What was checked (2026-10-31, no login)

- `muse mcp --help`: servers are "streamable-HTTP entries under `mcpServers` in `settings.json`",
  and `muse mcp login <server>` does OAuth for them. That is the shape the `av` daemon speaks
  (`/mcp`), so an `av` entry should work.
- `muse exec [PROMPT]` runs one prompt headless. `--json` prints JSONL events, `--workspace PATH`
  roots its tools there, `--image PATH` attaches an image (repeatable), `--model`,
  `--reasoning-effort`, `-c KEY=VALUE` sets a setting for one run.
- `muse init` scaffolds an `AGENTS.md` in the workspace ("Muse Code reads this file as project
  rules"). `muse init --dry-run` prints it without writing. So the text in
  [`../agents-instructions.md`](../agents-instructions.md) goes into `AGENTS.md`, as for Codex.

## What was not checked

- Where the user-level `settings.json` is. `muse config status` lists only the enterprise planes,
  and there is no `~/.muse` before a login. Find it after `muse login`, then add the entry below.
- Whether an entry uses `url` or `httpUrl`, and whether it accepts a static `headers` table. Take
  the key names from Muse's own documentation, not from this file.
- How `muse exec --json` reports token usage (the benchmark adapter needs it).
- What name Muse sends in `clientInfo.name`. Run one session, then look at the task owner
  (`av status`) and add that name as an alias: `av agent set muse --can code,docs --cost 1 --alias
  <name>`.

The entry should end up like this (key names unverified):

```json
{ "mcpServers": { "av": { "url": "http://localhost:7420/mcp",
                          "headers": { "X-Av-Agent": "muse", "Authorization": "Bearer ${AV_TOKEN}" } } } }
```

## Without MCP: pull with the CLI

This works today and needs none of the above. `av next` takes the next task that fits `muse`,
creates its workspace and prints the pack; the wrapper runs Muse in it and checkpoints afterwards:

```bash
av agent set muse --can code,docs --cost 1          # the cheapest agent: it gets untagged work
cd "$(AV_AGENT=muse av next | sed -n 's/^workspace: //p')"   # empty output: nothing is waiting
AV_AGENT=muse integrations/generic/av-wrap.sh muse exec --workspace "$PWD" \
    "Do the task in AGENTS.md; run the tests; then stop."
AV_AGENT=muse av submit --record "decision: …"      # or hand off with `av handoff --needs image`
```

Muse also takes images (`--image`), so do not assume it cannot do `image` work: set its profile from
the measurements in `docs/superpowers/plans/2026-10-31-plan-8-grok-evaluation.md` (E-G1).
