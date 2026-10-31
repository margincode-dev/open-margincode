# Instructions for agents without hooks

The `av` MCP server already sends short instructions when an agent connects (how to work a task,
take the next job with `start_task`, and review). Claude Code, Grok Build and Cursor Agent show them to
the model, so the basics need no setup. What the server deliberately does not say is to keep taking
jobs: an agent you open for your own work should not start pulling the backlog. To make an agent
loop, put this in the instructions file it reads (`AGENTS.md` for Codex, Grok Build and Cursor
Agent, `GEMINI.md` for Gemini CLI), or say it in the prompt:

```markdown
## Scholion (`av` MCP server)

- Call start_task with no title or goal and do what it returns (review, merge, handoff or new task).
  When it returns no task, call it again with wait set; stop after three empty waits.
- Work only in the workspace it returns. Your edits there are snapshotted into commits, so
  `git status` shows nothing; the pack lists the changes. Read the pack: it holds the decisions and
  dead ends of earlier agents. Call `get_context` for more, `expand` for one item.
- Record why, not what: `record` a `decision` for a choice someone could undo by mistake, an
  `attempt` for a dead end (what you tried and why it failed), a `note` for anything else worth
  knowing. Summaries are one line (at most 200 characters) and anchor to the file or symbol.
- `checkpoint` after a group of edits. Run the tests yourself, then `validate` with the command and
  its exit code.
- If your process restarts, the task is forgotten: pass `task_id` on later calls.
- To stop before you are done, `handoff` (done, remaining, broken, questions). If the rest needs
  something you lack (images, say), pass `needs: ["image"]` so only a fitting agent is offered it. When done,
  `submit`; another agent reviews it. When `start_task` gives you a review: `review_task`, run its
  tests in its workspace and `validate`, then `post_review`; if you approve, `merge` it. You never
  review your own work.
```
