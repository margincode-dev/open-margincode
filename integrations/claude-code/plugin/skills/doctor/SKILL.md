---
name: doctor
description: Find and fix what is wrong with a Scholion setup, such as `av` not found, the daemon not answering, the `av` MCP server not connected in Claude Code, hooks that seem silent, 401 errors or a repository import that hangs. Use when Scholion or `av` commands fail, or when the user says Scholion isn't working.
---

# Fix a Scholion setup

Go through the checks in order and stop at the first one that fails. Fix it, explain the cause
in one sentence, then run the remaining checks. Never delete `~/.scholion` (the daemon's data:
its Postgres database and its copies of the repositories) or anything under `~/ws` (task
workspaces that may hold unsubmitted work) unless the user asks you to.

## 1. Is `av` installed and on PATH?

`av --version`. If it is not found, look for `~/.local/bin/av`. If that file exists, the
directory is missing from `PATH`: run `export PATH="$HOME/.local/bin:$PATH"`, and offer to add the
line to the user's shell profile (ask first). If the file is missing, use the `setup` skill
(`/scholion:setup`).

## 2. Does the daemon answer?

`curl -sf http://localhost:7420/healthz` (or the address in `AV_URL`, if it is set).

If it does not answer, run `av up` and read the end of the log, `tail -50 ~/.scholion/daemon.log`
(or `$AV_HOME/daemon.log`, if `AV_HOME` is set):

- **Running as root** (`id -u` prints `0`): the daemon's Postgres refuses to start as root. Run
  Scholion as a normal user.
- **The port is already in use**: find out what holds it (`ss -ltnp | grep 7420` or
  `lsof -i :7420`). If it is an old daemon, `av down`, then `av up`. To use another port, set
  `AV_URL=http://localhost:7421` for the daemon and for every client, and run `av init` again so
  the agents' configs point at the new address.
- **A stale pid file**: `av down` removes it, then run `av up`.
- **Postgres fails to start or to download**: source builds download Postgres on their first
  start, so they need the network. `AV_PG_INSTALL_DIR` can point at an existing PostgreSQL 16
  installation (the directory that holds `bin/`), and `AV_DATABASE_URL` at an existing database.
- Anything else: show the user the relevant log lines and the error. Don't guess.

## 3. Is the repository there?

`av repo list`. If the repository is missing, run `av init` from inside it. If an import seems to
hang, it is probably still running: large repositories take minutes. Run `av repo list` again
before you do anything else.

## 4. Is Claude Code connected?

- `claude mcp list` should show `av` as connected. If `av` is missing, run `av init` again, or
  `claude mcp add --transport http av http://localhost:7420/mcp`. If it is listed but failed,
  the daemon is down (go back to check 2).
- A session started before setup cannot see the `av` server or the hooks. Restart Claude Code.

## 5. Are the hooks doing nothing?

`grep -c "av hook" ~/.claude/settings.json` should print 4 or more. If it prints less, run
`av init` again (it replaces earlier Scholion hooks and keeps `settings.json.bak`).

The hooks are **silent outside a task**, by design. They only act when Claude Code runs inside a
task workspace (`~/ws/<repo>/<task>`, the directory with an `.av-task` file). Start a task first
(ask the agent to *"start an av task to …"*, or run `av task start --repo REPO --title … --goal …`),
then open Claude Code in the workspace it prints.

## 6. Are requests refused with 401?

The daemon was started with `AV_TOKEN`, and the client does not send it. Export the same
`AV_TOKEN` in the shell that runs `av` and the agents. The agents' configs name the variable,
never the token itself. For `claude mcp add`, add
`--header "Authorization: Bearer $AV_TOKEN"`.

## 7. Still stuck?

Collect `av --version`, `uname -sm`, the failing command with its full output, and the end of
`daemon.log`. Remove any token or key before you show them. Suggest opening an issue at
https://github.com/vinhqbui/scholion/issues with that information.

The full reference is at
https://github.com/vinhqbui/scholion/blob/main/docs/reference.md.
