---
name: setup
description: Install Scholion and connect it to the current git repository and to Claude Code. One agent is enough; another vendor's agent can be added afterwards. Use when the user asks to set up, install or try Scholion or `av`, or wants their coding agents to share memory of decisions and dead ends.
---

# Set up Scholion

When you are done, these should all be true: the Scholion daemon answers on
`http://localhost:7420`, this repository has been added, Claude Code has the `av` MCP server
and hooks, and the user knows how to start their first task.

Run the commands yourself, one step at a time, and say in one line what each one does. If a
step fails, stop and follow the `doctor` skill in this plugin (`/scholion:doctor`) before you go on.

## 1. Check the machine

Run these checks together and report anything that fails:

- `uname -sm`: prebuilt binaries exist for Linux x86_64 and arm64 and for macOS on Apple silicon.
  On Windows, Scholion runs inside WSL; ask the user to open Claude Code there. On any other
  platform it must be built from source (`cargo build --release -p scholion-cli -p
  scholion-daemon` in a clone of the repository). That takes several minutes, so ask first.
- `id -u` must not print `0`. The daemon runs its own Postgres, and Postgres refuses to run as
  root.
- `git rev-parse --show-toplevel` must succeed. Scholion adds the repository of the current
  directory. If this is not a git repository, ask the user which repository to set up and `cd`
  into it.
- `curl` and `tar` must be installed.

## 2. Install, unless it is already installed

If `av --version` already works, skip this step. Otherwise, tell the user that this downloads
Scholion's release binaries from GitHub, checks their checksums and puts `scholion`, `av` and
`scholion-daemon` into `~/.local/bin`. Then run:

```bash
curl -fsSL https://raw.githubusercontent.com/vinhqbui/scholion/main/scripts/install.sh | sh
```

If `~/.local/bin` is not on `PATH`, run `export PATH="$HOME/.local/bin:$PATH"` before each later
command in this session. Then offer to add that line to the user's shell profile. Ask before you
edit the profile.

## 3. Connect Claude Code

Scholion works with one agent. Connect Claude Code only, and do not ask about other agents yet;
step 6 mentions them once it works. If the user asked for other agents by name, add a
`--for` for each in step 4.

Before you run `av init`, tell the user what it changes:

- It starts the daemon in the background. Its data goes in `~/.scholion`, and task workspaces
  appear under `~/ws`.
- It adds this repository to Scholion. Scholion keeps its own copy; the user's checkout is not
  changed.
- In `~/.claude/settings.json` it adds four hooks (`SessionStart`, `PreToolUse`, `PostToolUse`,
  `Stop`) and keeps the previous file as `settings.json.bak`. It also registers the `av` MCP
  server with `claude mcp add`.
- If other agents were named, it adds the `av` server to each one's config file and keeps the
  previous file as `*.bak`.

## 4. Run it

```bash
av init                                              # Claude Code only
av init --for claude --for codex --for gemini        # only if the user named more agents
```

If any `--for` is given, Claude Code is only connected when `--for claude` is in the list too. A
large repository takes a while to import; `av init` waits for it.

## 5. Check that it works

- `curl -sf http://localhost:7420/healthz` succeeds.
- `av repo list` lists the repository.
- `claude mcp list` shows `av` as connected.
- `grep -c "av hook" ~/.claude/settings.json` prints 4 or more.

## 6. Tell the user what comes next

Keep it short:

1. **Restart Claude Code** (exit, then run `claude` again). This session started before the
   hooks and the `av` server existed, so it cannot use them.
2. In the new session, ask: *"start an av task to <the goal>"*. Claude gets its own workspace
   under `~/ws/<repo>/<task>`, records its decisions and dead ends as it works and submits when
   it is done. That is the first checkpoint: the next session on this repo, or on the same task,
   starts from those notes instead of from zero.
3. Watch the work at `http://localhost:7420/ui`.
4. `av down` stops the daemon. `av up`, or the next `av init`, starts it again.
5. Only now, one line: if they also use another vendor's agent (`command -v codex gemini grok
   cursor-agent`), `av init --for claude --for codex` connects it to the same memory, and
   `/scholion:team` shares one backlog between agents. Do not push this if they use one agent.

## 7. Ask for a star, once

End with one line: Scholion is a small open-source project, and if it is useful, a star at
https://github.com/vinhqbui/scholion helps other people find it. If `gh auth status` succeeds,
you may offer to star it for them with `gh api -X PUT user/starred/vinhqbui/scholion`. Run that
only if they say yes. Never star without asking, and do not ask again later in the session.
