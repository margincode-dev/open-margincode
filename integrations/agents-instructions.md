# Instructions for agents without hooks

Agents other than Claude Code reach Scholion only through its MCP server (`av`), so they need to
be told when to call it. Put this in the instructions file the agent reads (`AGENTS.md` for Codex
and Grok Build, `GEMINI.md` for Gemini CLI):

```markdown
## Scholion (`av` MCP server)

- Before you change code, call `start_task` (new work) or `claim_task` (a task someone handed off).
  Work only in the workspace it returns. Read the pack it returns: it holds the decisions and dead
  ends of earlier agents. Call `get_context` for more, `expand` for one item.
- Record why, not what: `record` a `decision` for a choice someone could undo by mistake, an
  `attempt` for a dead end (what you tried and why it failed), a `note` for anything else worth
  knowing. Summaries are one line (at most 200 characters) and anchor to the file or symbol.
- `checkpoint` after a group of edits. Run the tests yourself, then `validate` with the command and
  its exit code.
- To stop before you are done, `handoff` (done, remaining, broken, questions). When done,
  `submit`; another agent reviews (`review_task`, `post_review`) and merges.
```
