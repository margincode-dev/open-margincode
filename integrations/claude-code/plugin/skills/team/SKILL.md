---
name: team
description: Set up several coding agents (Claude Code, Codex, Gemini CLI, Grok Build, Cursor Agent or any MCP client) to share one Scholion backlog. Connects each agent, records what each is good at and what it costs, posts the first work and starts each agent when work fits it. Use when the user wants agents to split work, review each other or work through a backlog without supervision.
---

# Set up a team of agents on Scholion

Scholion treats each agent CLI as a team. You post work to one backlog. Each agent takes the
items that fit it best, another agent reviews the result, and the reviewer merges it. All of them
share the same notes, so a dead end one agent found is in the next agent's brief.

## 1. Make sure Scholion is running

Run `curl -sf http://localhost:7420/healthz` and `av repo list`. If the first fails or the
repository is not listed, follow the `setup` skill (`/scholion:setup`) first.

## 2. Connect the agents

Check which agent CLIs are installed: `command -v claude codex gemini grok cursor-agent`. Connect
each one the user wants, from inside the repository:

```bash
av init --for claude --for codex --for gemini --for grok --for cursor   # only the ones installed
av --agent NAME init --for mcp-json --path FILE   # any other client with an mcpServers JSON file
```

Each command keeps a `*.bak` copy of every config file it changes. Tell the user that before you
run it.

## 3. Describe each agent

Ask the user once, for all of the agents together:

- What is each agent **best** at, and what else **can** it do? Use short tags such as `code`,
  `tests`, `docs`, `ui`, `review` or `image`.
- What does each one **cost**, from 1 (cheapest) to 10?
- Optionally, the most tasks each one may hold open at once.

Suggest a starting point that matches the agents found, and let the user change it. Then run one
command per agent:

```bash
av agent set claude --best code,tests --can docs --cost 4
av agent set codex --best code --can tests --cost 3 --max-open 2
av agent list
```

Routing needs no model to decide. An item goes to an agent that has all of its tags among its
`best` and `can` tags. The best fit (most `best` tags, then the cheapest) has the item to itself
for 10 minutes; after that, any agent that fits may take it. Untagged work goes to the cheapest
agent.

## 4. Agents without hooks need instructions

Claude Code and Grok Build get hooks, so they know to keep taking jobs. Codex, Gemini CLI and
Cursor Agent do not have hooks. For those, offer to add the block under "Instructions for agents
without hooks" from
https://raw.githubusercontent.com/vinhqbui/scholion/main/integrations/agents-instructions.md
to the repository's `AGENTS.md` (Codex, Cursor) or `GEMINI.md` (Gemini). Ask before you
edit those files.

## 5. Post the first work

Size each item as a whole job with a definition of done, not as a single step. Ask the user what
they want done, then post it:

```bash
av post --repo REPO --title "Add retry to the HTTP client" --goal "retry 5xx and timeouts with backoff; tests pass" --needs code
av post --repo REPO --title "Document retries" --goal "..." --needs docs --after add-retry-to-the-http-client
av backlog
```

`--after` holds an item back until the item it names has merged.

## 6. Start the agents

`av watch` starts an agent only when work fits it. Scholion never launches agents by itself.
Each agent needs its own long-running terminal, so give the user these commands to run rather
than starting them in the background yourself:

```bash
av --agent claude watch -- claude
av --agent grok watch --every 30 -- grok -p "take your next av job"
av --agent cursor watch -- cursor-agent -p --approve-mcps --trust "take your next av job"
```

For Codex and Gemini, use the same pattern with that CLI's own non-interactive command.
`--agent` must match the profile name from step 3.

Tell the user to watch the work at `http://localhost:7420/ui`. The Teams panel shows each agent's
strengths, cost and open tasks, and which items are reserved for which agent.
