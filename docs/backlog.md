# Backlog of observations

Errors and anything else that did not fit, written down when they were seen so they can be taken
up later (`constitution.md`, rule 5). Newest first. An entry stays when it is fixed, marked with
the pull request that fixed it. No secrets, keys or local paths: this repository is public.

Each entry: the date; where (command, test, commit); what was seen, exactly; what was ruled out;
the suspected cause, marked as a guess; the status.

## 2026-11-23: the retrieval benchmark is not repeatable run to run

- **Where.** `margincode-bench retrieval` (`crates/margincode-bench/src/retrieval.rs`).
- **Seen.** Two runs of the same build over the same tasks give different rows. A small
  difference between two ranking configurations therefore cannot be read from one run of each.
- **Ruled out.** Nothing yet.
- **Guess.** None yet: the cause is not found.
- **Status.** Open; being worked on.

## 2026-11-23: a pack rebuilt because main moved refers back to the pack it threw away

- **Where.** `pack_for_task` (`crates/margincode-daemon/src/api.rs`) and `Packer::build`
  (`crates/margincode-pack/src/packer.rs`), read while building the measures of packs.
- **Seen (read, not reproduced).** When main moves during a build, `pack_for_task` builds the
  same request again and answers the second pack. The first build already wrote its items to the
  session's ledger (`ledger_put`), so the second sends those items as `↩ … (sent earlier)`
  although the agent never received the first pack. Both receipts are stored.
- **Ruled out.** Nothing: no test builds a pack while main moves.
- **Guess.** The first build's ledger write should not happen (or be undone) when its pack is
  discarded. A change to the packer or the daemon's pack path, not made here.
- **Status.** Open.

## 2026-11-23: two test-path rules, the packer's and the benchmark's

- **Where.** `margincode_pack::score::is_test_path` (ranks test code at 0.6) and
  `margincode_core::outcome::is_test_path` (the benchmark's gold files and the live measure of
  packs).
- **Seen.** They differ: the benchmark's also treats `spec`, `specs` and `testing` directories,
  `conftest.py` and `*_tests.py` as tests, and ignores case; the packer's does not.
- **Ruled out.** Nothing.
- **Guess.** One rule for both would change ranking, so it needs a `PACKER_VERSION` bump and new
  golden files.
- **Status.** Open.

## 2026-11-23: the CLI test binary crashed with SIGSEGV once

- **Where.** `cargo test --workspace` at `7e3bfea` (unchanged `main`), in a `rust:1.97.1`
  container as root, Postgres from `AV_TEST_DATABASE_URL`, another build running on the host.
- **Seen.** `tests/cli.rs` stopped with `signal: 11, SIGSEGV: invalid memory reference` after
  about 50 of its tests had passed; no test was named as failing. The same binary run alone right
  after passed: 171 passed, 0 failed.
- **Ruled out.** A change of ours: the tree was `main` as merged.
- **Guess.** A crash in a dependency under load, or a stack overflow in one test thread. Not
  reproduced.
- **Status.** Open.

## 2026-11-22: OpenCode sessions do not get `AV_SCRATCH` or `AV_DOCKER_LABELS`

- **Where.** The OpenCode plugin (`integrations/opencode/av.js`), live with OpenCode 2.3.2.
- **Seen.** `av hook session-start` exports `AV_SCRATCH` and `AV_DOCKER_LABELS` by appending to
  `$CLAUDE_ENV_FILE`, which only Claude Code sets. Under OpenCode the variable is unset, so the
  model's shell commands have neither; the pack still names the scratch directory in its last
  line, and nothing tells the model to label its containers.
- **Ruled out.** Nothing: the plugin does not try to pass them on.
- **Guess.** OpenCode 2.3.2 has a plugin hook `shell` `create.before` whose `env` the plugin may
  change before a shell command starts (`packages/core/src/shell.ts` at `v2.3.2`; read, not run).
  The plugin could point `CLAUDE_ENV_FILE` at a file of its own for `av hook session-start`, read
  the `export` lines back, and add them to each command of that session. Not done in the change
  that added the plugin.
- **Status.** Open.

## 2026-11-22: under OpenCode, the hook messages' `av record` fails and MCP `record` needs a `task_id`

- **Where.** Live OpenCode 2.3.2 sessions in a task, free model, a private server with no av
  variables in its environment.
- **Seen.** The pack's last line and the stop's message tell the model to run `av record --json -`.
  In the model's shell that failed (`av: command not found` there; with `av` on `PATH` it would
  still lack `AV_URL` and `AV_TOKEN`). The model then called the MCP tool `tools.av.record`
  without `task_id` and got `no task: pass task_id, or call start_task or claim_task first`; it
  found the task by trying `get_context` and `status` with the slug as `task_id`, and recorded
  with that. Each session spent several tool calls on this.
- **Ruled out.** The plugin: the hooks themselves ran and their messages arrived.
- **Guess.** The MCP session is not bound to the workspace's task, since OpenCode's MCP client
  starts outside it. The pack or the messages could name the `task_id` for MCP callers (a change
  to the hook text), and the `shell` `create.before` hook (entry above) could give the model's
  shell `AV_URL`, `AV_AGENT` and `AV_TOKEN`.
- **Status.** Open.

## 2026-11-22: OpenCode's automatic compaction is untested with the plugin

- **Where.** The OpenCode plugin's `compaction` hook.
- **Seen.** A compaction started through `opencode api session.compact` fired the hook, and the
  next prompt got a fresh pack. A compaction OpenCode starts by itself when the context fills
  was not reached on the free model.
- **Guess.** The same hook fires for both: a manual, an automatic and an overflow compaction all
  go through `SessionCompaction.compact` (`packages/core/src/session/compaction.ts` at `v2.3.2`).
  That was read, not run.
- **Status.** Open.

## 2026-11-22: the OpenCode plugin can still send a stop message the interrupt meant to stop

- **Where.** `integrations/opencode/av.js`, the `experimental.idle` hook.
- **Seen.** The plugin learns of an interrupt from the server-wide event
  `session.execution.interrupted`, which reaches a plugin's subscription after the interrupt has
  happened. Live, an interrupt while `av hook stop` waited killed that child and nothing was sent.
  But when `av hook stop` has already answered `block` and the plugin has called `synthetic`
  before the event arrives, the message goes out and starts a new turn (a late `synthetic` after
  an interrupt does, as the gates saw).
- **Guess.** The window is the event's delivery delay after `av hook stop` answers. Closing it
  needs a session state OpenCode does not hand the idle hook (whether it was interrupted).
- **Status.** Open.

## 2026-11-22: with the daemon unreachable, every hooked OpenCode tool call waits

- **Where.** `integrations/opencode/av.js`, the `read` and edit hooks (`execute.after`), with
  the daemon stopped, live with OpenCode 2.3.2.
- **Seen.** The first prompt of a session in a task went through without a pack once
  `av hook session-start` had given up on the connection. By the code, every later `read`,
  `edit`, `write` and `patch` of the session runs `av hook pre-read` or `post-edit` and waits the
  same way before the tool's result reaches the model; only the first prompt was run.
- **Guess.** A short cool-down after a hook that could not reach the daemon (skip the `av hook`
  calls of that server for a while) would bound the wait to one per cool-down.
- **Status.** Open.

## 2026-11-22: one `av plugin:` line in OpenCode's log, then silence

- **Where.** `integrations/opencode/av.js`, `warn`.
- **Seen.** By design the plugin logs only its first problem per server. A benign first line on
  `av hook`'s stderr (for example a footprint scan marked `partial`) uses it up, and a later real
  failure is then not logged.
- **Guess.** A per-kind once (a can't-start, a time-out, a failed exit) would keep the log short
  and still show a later failure.
- **Status.** Open.

## 2026-11-22: the embedded test Postgres refuses to start as root

- **Where.** `margincode-cli` `tests/cli.rs` (any test that calls `start_daemon`), in a
  `rust:1.97.1` container run as root, with no PostgreSQL installed in it and
  `AV_TEST_DATABASE_URL` unset.
- **Seen.** `TestDb::new` panicked with `postgres setup (needs network on first run):
  DatabaseInitializationError("Command error: stdout=; stderr=initdb: error: cannot be run as
  root ...")`, and the test then failed with `daemon failed to start: RecvError`.
- **Ruled out.** The change under test (OpenCode's `av init` writer): the same tests passed once
  `AV_TEST_DATABASE_URL` pointed at a throwaway `postgres:16` container.
- **Guess.** As root, only the installed-binaries path (`installed.rs`) drops to `postgres` or
  `nobody`; the embedded fallback runs `initdb` as the current user. `CLAUDE.md` ("Test
  environment") reads as if a root container works without an installed PostgreSQL. Either the
  embedded path could drop privileges too, or that text could name the requirement.
- **Status.** Open.

## 2026-11-22: the build-slot test missed its "waiting" line once under load

- **Where.** `margincode-cli` `tests/cli.rs` `validate_waits_for_a_free_build_slot_on_the_host`,
  in `cargo test --workspace` on the release 0.0.5 branch (version bump only), in a
  `rust:1.97.1` container while two other workspaces' test runs used the same host.
- **Seen.** The assertion at line 5158 failed: stderr did not contain "all 1 build slots on this
  host are in use; waiting". The checks before it passed (nothing ran during the 1.5 s, the
  command was still running, it then succeeded and wrote its file). The other 158 tests of the
  binary passed. Rerun alone in the same container: passed in 5.9 s.
- **Ruled out.** The change under test: it touches versions and text only.
- **Guess.** `av validate` checkpoints the task before it asks for a build slot. When that took
  longer than the test's 1.5 s, the test released the slot before validate reached
  `slots::wait_for`, which then found it free and printed nothing. The 1.5 s sleep is then an
  upper bound on the checkpoint's time, which `CONTRIBUTING.md` ("Timing") forbids. The test
  could wait for the "waiting" line on stderr instead of sleeping.
- **Status.** Open.

## 2026-11-21: the installer and the formula install on a glibc that is too old

- **Where.** `brew install margincode` from the 0.0.4 formula in the `homebrew/brew` image
  (Ubuntu 22.04, glibc 2.35).
- **Seen.** The archive's sum matched and the three binaries were linked; `margincode --version`
  then stopped with ``version `GLIBC_2.39' not found``. `scripts/install.sh` makes no check
  either. On Ubuntu 24.04 (glibc 2.39) both work.
- **Status.** Open; `docs/install.md` now states the requirement. The installer could read
  `ldd --version` and say so before it downloads.

## 2026-11-21: a local build of the lite image fails with 401 when no token is given

- **Where.** `deploy/Dockerfile`, built by hand with
  `--build-arg FEATURES=margincode-daemon/bundled-postgres` and no `github_token` secret
  (Docker 29.9.3, buildx 0.38.1), at the 0.0.4 release commit.
- **Seen.** `postgresql_embedded`'s build script stops with "401 Unauthorized" from
  `api.github.com/repos/theseus-rs/postgresql-binaries/releases`. The same `cargo build` in a
  `rust:1.97.1` container started with `docker run`, with no token at all, passes.
- **Cause.** `--mount=type=secret,id=github_token,env=GITHUB_TOKEN,required=false` sets
  `GITHUB_TOKEN` to an empty string when the secret is absent (a three-line Dockerfile shows
  "set, length 0"), and the build script sends the empty token.
- **Not affected.** `image.yml` passes the secret, so the published image builds; Compose
  builds without the feature.
- **Status.** Open. Unsetting an empty `GITHUB_TOKEN` before `cargo build` would do.

## 2026-11-21: `margincode-daemon --version` starts a daemon

- **Where.** The 0.0.4 archive, in the release's install check.
- **Seen.** `margincode-daemon --version` printed "lite mode: embedded postgres started" and
  "margincode-daemon ready", and was still running when `timeout 25` ended it: the daemon reads
  no arguments. `margincode --version` and `av --version` print the version.
- **Status.** Open.

## 2026-11-21: a file whose line endings changed lists every symbol of the file as changed

- **Where.** `GET /v1/tasks/{id}/unrecorded` (so the Stop hook's block and `av submit`'s
  advisory), which takes its symbols from `margincode_pack::assemble::changed_symbols`. Seen on a
  build of `f2ebcde` and in Claude Code sessions on `7cc7d1f`.
- **Seen.** Adding `DEFAULT_PAGE_SIZE = 100` to `tallyho/store/scan.py` listed
  `DEFAULT_PAGE_SIZE, count_records, iter_records`. A session that changed only
  `apply_discounts` was stopped over all 8 symbols of `discounts.py`, three of them above the
  edited function. Those files are stored with CRLF endings. That session made two edits, both inside
  `apply_discounts`; every session that made one edit to its file was stopped over the edited
  symbols only. That the second edit rewrote the file's endings is inferred from this, not
  seen: the session's files were not kept. Reproduced through the route: the same insertion
  with the endings kept lists only `DEFAULT_PAGE_SIZE`, and with the file saved as CRLF lists all
  three.
- **Ruled out.** A symbol that only moved down: an insertion above two functions lists only the
  inserted name. The whole-file fallback of `changed_ranges_of` (both reads succeed) and stale
  index spans (the new name is listed, so the index is at head, and a shift could not list
  symbols above the edit).
- **Cause.** `line_ranges` compared lines with their endings (`similar::TextDiff::from_lines`),
  so a file whose endings changed was one replacement over every line, and every symbol
  overlapped it. Whether, and why, an agent's edit tool rewrites a file's endings on a second
  edit is not verified.
- **Status.** Fixed in this change: lines are compared with `\r\n` read as `\n`, split as
  before so the line numbers stay those of head; other differences at a line end still count.

## 2026-11-21: a file-anchored record ranks lower and goes stale on any edit in the file

- **Where.** The packer's record scoring and its `(changed since recorded)` check; a reader-side
  check of deterministic packs, no model.
- **Seen.** For a later task working on the same function, a record anchored only by the file's
  bare path scored 0.485 (path match), and one anchored on that function, the seed symbol, 1.66.
  After an edit anywhere in the file the file-anchored record was flagged
  `(changed since recorded)` and dropped from 0.485 to 0.3395; a symbol-anchored one is flagged
  only when its symbol changes. A symbol-only record does not satisfy the Stop and submit check
  when something else in the file changed too. So the session-start line now asks for both: the
  changed symbol, for ranking, and the file's bare path, for the check.
- **Ruled out.** Reach: a file-anchored record reaches a later task's pack in the same cases as a
  symbol-anchored one.
- **Guess.** None. Whether the packer should treat a file-anchored record as anchored on the
  symbols its own task changed is open.
- **Status.** Open.

## 2026-11-21: the pre-read hook shows a record without its changed or stale flag

- **Where.** `margincode_pack::Packer::records_at` (`GET /v1/tasks/{id}/records/at`, the
  pre-read hook's answer), `crates/margincode-pack/src/packer.rs`.
- **Seen.** It renders each record with
  `let (id, texts) = record_texts(rec, false, false, origins.whence(rec, task.id));`, so the
  `stale` and `changed` arguments are always false: a record the pack prints with
  `(changed since recorded)` or `(anchor_stale)` is shown in the pre-read answer without either
  flag. The pack's own calls pass the computed values (`record_texts(rec, stale, changed, …)` in
  `assemble.rs`).
- **Ruled out.** Nothing yet.
- **Guess.** The flags were not wired into `records_at` when it was written; not checked in its
  history.
- **Status.** Open.

## 2026-11-20: the MCP `record` tool still refuses a summary over 200 characters

- **Where.** `margincode-daemon` `mcp.rs`, `check_sizes`, for the `record` tool (and `submit`'s
  inline records over MCP).
- **Seen.** In Claude Code sessions with the av hooks (release 0.0.2), a first
  `av record --json -` was refused for a summary over 200 characters and resent. `av record`, `av submit --record` and `av review --finding` now fit such a summary into
  a headline and keep the whole text in the body. The MCP tool was left alone: its description
  states the limit up front, and its refusal (`-32602`) names the length.
- **Ruled out.** Nothing: whether MCP agents send overlong summaries and retry on the refusal
  has not been measured.
- **Guess.** If they do, the same `margincode_core::fit_summary` could serve the tool.
- **Status.** Open.

## 2026-11-20: the dreamed-arm bench test passes in 0 s without `av` or `jq`

- **Where.** `margincode-bench` `tests/runner.rs`,
  `runner::a_frozen_dream_is_replayed_into_a_session_of_the_dreamed_arm`, run 40 times in a loop
  to check #88 (the loop #13 used to show its fix), in a `rust:1.97.1` container.
- **Seen.** The first loop of 40 passed 40 of 40, each run in 0.00 s. The test prints
  `skip: no av binary beside the test binary (cargo build -p margincode-cli)` or `skip: jq not
  installed` and returns, so cargo reports it as passed. Neither line shows without
  `--nocapture`, and the container had neither. With `cargo build -p margincode-cli` first and
  jq 1.7 on `PATH`, each run took about 19 s and 40 of 40 passed.
- **Ruled out.** Not a slow or flaky test: it never ran.
- **Guess.** None needed. A loop that means to exercise it needs both, and nothing in the bar's
  output says it skipped. Marking it `#[ignore]` with its needs in the reason, or failing when an
  environment variable asks for it, would make a skip visible.
- **Status.** Open.

## 2026-11-20: a session-start pack showed a disputed record without its reason, and another not at all

- **Where.** A live run of `av dream` with real models on a small Python repository (`tallyho`,
  one commit), av 0.0.3, no semantic layer, first packs at the default 800-token budget. Probe tasks were started
  after records were seeded and after verdicts on them took effect.
- **Seen.**
  - Goal "Fix Money.format in tallyho/money.py for JPY": `tokens: 710/800`, 20 items omitted.
    The disputed decision was printed as
    `- rec:… decision, abandoned: Every currency tallyho supports has two decimals. Plain amounts are mi… (disputed)`,
    without the reason or the `vd:` id. The pre-read answer for the same file (`records_at`)
    printed `(disputed: …, vd:…)`.
  - Goal "Fix slugify in tallyho/text.py for accented product names": `tokens: 488/800`, a Code
    section led by `slugify`, and no "Decisions & dead ends" section. It was the same before any
    verdict existed and after a disputed verdict on the decision anchored to `slugify` took effect.
    The pre-read answer for `text.py` listed that decision.
- **Ruled out.** Both are the packer working as written, not a rendering gap, and neither comes
  from dreaming.
  - The handle tier of every record line is its summary clipped to 70 characters plus a short
    flag. A disputed record's flag is `(disputed)`, and its summary and full tiers add the reason
    and the verdict (`record_texts`). A disputed record also ranks ×0.3 (`standing`). With 20
    items left out, it got the handle tier.
  - `av expand rec:…` prints the full tier: the reason and the `vd:` id. The pack's footer says to
    expand an id whenever it leaves items out.
  - `slugify` in plain words is not a named identifier (`lexical::named_identifiers`: backticks,
    `name(`, snake_case, camelCase or a dotted path), and a file name is left to full-text search.
    So the goal seeded nothing. Without seeds, `records_anchored_to` gets no paths or names, and
    the best text hits stand in for seeds only to bring in their neighbours (`expand_best_hits`),
    not their records. Without a semantic layer no record arrives as a hit either.
    `Money.format` is a dotted name, so that goal seeded it and got its records.
- **Pinned.** The pack test
  `a_function_the_goal_names_only_as_a_plain_word_brings_in_none_of_its_records` (and the same
  goal in backticks brings them in). The unit test
  `a_disputed_records_handle_says_disputed_and_its_longer_tiers_say_why`. The daemon test
  `expanding_a_disputed_record_gives_the_reason_and_the_verdict`.
- **Guess.** An open question, not a defect: whether the records anchored on the best text hits
  of a cold start should join as the hits' neighbours do. That would change first packs for
  existing inputs, and the measured results on how a dreamed pack reads rest on them, so it
  needs its own measurement first.
- **Status.** Works as designed; pinned by the tests named above.

## 2026-11-19: Antigravity's hooks need their own output

- **Where.** `av init --for antigravity` (`integrations/antigravity/README.md`, "Hooks"), against
  the hook contract in the docs shipped with the Antigravity CLI (`hooks.md`).
- **Seen.** Antigravity runs hooks from `.agents/hooks.json` with JSON on stdin and reads JSON on
  stdout, and none of av's four hook commands prints what it reads. `SessionStart` wants
  `{"injectSteps": [{"systemMessage": {"systemMessage": "…"}}]}` and `av hook session-start` prints
  plain text. `PreToolUse` needs a permission `decision`, with a `reason` as its only text, where
  `av hook pre-read` prints `additionalContext` and decides nothing. `PostToolUse` expects `{}`. `Stop` carries on only on
  `"decision": "continue"`, and `av hook stop` prints `"block"`, so its record nudge and its wait
  for work would be counted and then ignored. The input names differ too: `conversationId` for
  `session_id`, `continuationActive` for `stop_hook_active`, `toolCall.args` for `tool_input`
  (the docs do not give its file tools' names or argument keys), and no `source`. On Windows a
  hook runs under `cmd /c`, where av's `command -v av …; exit 0` guard is not valid; the default
  timeout is 30 s against the Stop hook's 600 s wait.
- **Ruled out.** A field mapping alone: every event's output differs, so each command would need
  a second output form.
- **Guess.** An Antigravity mode for `av hook` (detected from `hookEventName` in the input, or a
  flag) that renames the input fields and prints `injectSteps`, `{}` or `continue` would cover
  `SessionStart`, `PostToolUse` and `Stop`; `pre-read` could hand over its records only in a
  `reason` beside a decision it would have to make for the user. Not built
  because nothing here can run `agy` to check it.
- **Status.** Open. `av init --for antigravity` installs no hooks.

## 2026-11-19: a Workers Builds preview failed with "preview already exists"

- **Where.** The Workers Builds check on #79 (branch `av/gate-per-task`), Cloudflare's preview
  build for that branch.
- **Seen.** The build failed at `npx wrangler preview` with an error that the preview already
  exists. From the builds API, the Git integration had created that branch's preview 0.16 s after
  the branch's preview trigger was made, before the build's own `wrangler preview` ran. A new build
  started on the same commit through the trigger (`POST .../builds/triggers/{trigger}/builds`)
  passed; the retry route for the failed build answered 404.
- **Ruled out.** The change: #79 touched only a test, a README and this file, and the new build of
  the same commit passed.
- **Guess.** A race in Cloudflare's own setup between the integration's preview and the build
  step's.
- **Seen again.** #84 (branch `av/antigravity-init`), 2026-11-20: the same error, code 100338,
  ("A preview with name 'av/antigravity-init' already exists"), 40 s after the build started; a
  new build of the same commit from the branch's trigger passed and the PR was merged after it.
  The previews of #78 and #80 to #83 passed on their first build.
- **Seen again.** #93 and #94, 2026-11-21: the Workers Builds check failed on both branches. Both
  were merged before it had passed, against this entry's own advice: #93 while no check had
  reported, #94 while the state was unstable. The production builds of `main` after each merge
  (`5b9bb65`, `6945ee0`) passed. The branch builds' logs were not read, so it is not confirmed that
  the cause was the same "already exists" race.
- **Status.** Open, seen twice, and two more failed checks whose cause is not confirmed. When it
  shows, start a new build from the trigger rather than merging on a red check.

## 2026-11-19: reclaim retried every tick on a workspace it could not delete

- **Where.** The dogfood daemon (Docker image, user 1000, workspaces on a bind mount), in
  `disk.rs`'s reclaim tick.
- **Seen.** `WARN reclaiming a workspace failed task=... error=vcs error: Permission denied (os
  error 13)` for three ended tasks, once a minute each (`RECLAIM_TICK`), 2,316 lines in a day.
  By the code a gc's answer lists such a task under `errors` (`av gc` was not run); the log was
  the only place it was noticed. Each workspace held `bench/tasks-routing/repo` owned by root,
  mode 755, with a `.git` inside. Removing that one directory from the host let the next tick
  reclaim all three (15 MB).
- **Ruled out.** The files the agent edited from the host: they also show as root-owned through
  the bind mount, but with mode 777 inside directories the daemon owns, and they delete fine.
- **Guess.** `margincode-bench`'s `tests/routing_tasks.rs` runs `make-repo.py`, which builds that
  repository inside the source tree when it is missing. A `cargo test` run as root in a container
  with the workspace mounted (how the bar ran before rule 12 moved it to a volume) left it
  root-owned, and the daemon's user cannot delete inside it. Two things could change: the test
  could build into a temporary directory instead of the checkout, and the tick could say once,
  not every minute, that a workspace cannot be reclaimed and why.
- **Status.** The tick's half is fixed in #74: a workspace the tick failed to remove is left for
  an hour (`RECLAIM_RETRY`) before it is tried again, so it warns once an hour, and `av gc` still
  tries it at once and lists the error. The test's half is fixed in this change:
  `routing_tasks.rs` builds the pilot repository in a temporary directory, so `cargo test` writes
  nothing into the checkout. `margincode-bench check` still builds `bench/tasks-routing/repo`
  when it is missing, because the task files point there.

## 2026-11-18: the deadline-kill test needs the shell to print inside 400 ms

- **Where.** `margincode-agentrun` `tests/command.rs`
  `a_session_past_its_deadline_is_killed_and_reads_as_truncated`, in a full workspace test run
  under load (first written down in the Commercial edition's backlog).
- **Seen.** Failed once: `assertion left == right failed: no result event: the message sum is
  kept`. Passed when run alone. The same run's output had `kill: failed to parse argument: ''`.
  Not reproduced in 30 runs (each test) with every core busy. Reproduced with `sh` on `PATH`
  replaced by a shim that waits a random 0 to 600 ms before starting the shell: in 30 runs the
  test failed 15 times with that message (`left: 0`), and its sibling below 7 times (no pid
  file). The empty-pid-file case behind the `kill` line was not reproduced: its window is the
  time between the redirect and the `echo`.
- **Ruled out.** The production kill: `run_until` kills `-<child pid>`, never an empty argument,
  and `av down` (`local.rs`) passes an empty pid only to `alive`, whose stderr is discarded. The
  shell's builtin `kill` says `Illegal number` instead; the `failed to parse argument` wording is
  the `kill` binary, started from Rust with an empty argument.
- **Guess.** The script echoes two lines and sleeps, with a 400 ms deadline; on a loaded host the
  shell had not printed both when the deadline killed it. The `kill` line is its sibling
  `tests/process.rs` `a_child_past_its_deadline_is_killed_with_everything_it_started` (500 ms
  deadline): a kill between `> grandchild.pid` and the `echo` leaves the file empty, so
  `kill -0 ''` fails at once and the test passes without checking the grandchild; a kill before
  the redirect leaves no file and fails the test. Both upper bounds on time break
  `CONTRIBUTING.md` ("Timing").
- **Status.** Fixed in this change: both tests run their script with a deadline from 400 ms,
  doubling, until the script has printed what the test needs (bounded within `HANG`), and assert
  killed on every try. With the same shim, both passed 30 of 30.

## 2026-11-18: the post-edit debounce test needs three CLI calls inside 2 s

- **Where.** `margincode-cli` `tests/cli.rs` `hook_post_edit_debounces_within_two_seconds`, found
  reading every timing bound in `crates/*/tests` for the timing-flake change.
- **Seen.** Not a failure yet. The check "second edit within 2 s is debounced" holds only if the
  first `av hook post-edit`, an `av status` and the second `av hook post-edit` (three
  subprocesses, each talking to the daemon) finish inside `DEBOUNCE_MS` (2,000 ms). On a loaded
  machine the second hook can run after the window and checkpoint, and the test then sees
  `changed (2)` where it expects `changed (1)`. It is an upper bound on time, which the rule in
  `CONTRIBUTING.md` ("Timing") forbids; the 2.1 s sleep after it is a sleep past the window and
  is fine.
- **Ruled out.** The other sleeps in that file: the 1.5 s one in
  `validate_waits_for_a_free_build_slot_on_the_host` checks that nothing ran, which a slow
  machine can only make pass.
- **Guess.** The stamp lives in `.av-task` (`last_checkpoint_ms`). The test could write a fresh
  stamp into the task file before the second edit instead of relying on the first hook's, and an
  old one (more than 2 s back) instead of sleeping 2.1 s.
- **Status.** Fixed in this change: the test writes `last_checkpoint_ms` an hour ahead before the
  in-window edit and 2.1 s back before the last one, so neither half depends on how long a call
  takes, and the 2.1 s sleep is gone. It still checks that the first hook writes the stamp.

## 2026-11-18: a backup in backoff keeps its backoff across a daemon restart

- **Where.** `mark_backups_due` (`crates/margincode-store/src/backup.rs`), called by
  `start_backups` (`crates/margincode-daemon/src/backup.rs`) at start-up; seen checking SSH
  backups on the compose stack from `main` (105f99f).
- **Seen.** About 13 s after a daemon restart, the backup status still showed the failure from
  before the restart, with the next retry about 100 s away. Both doc comments say start-up makes
  every configured backup due, but the update is `SET next_retry = now() WHERE remote IS NOT NULL
  AND next_retry IS NULL`: a repo at rest (`next_retry` NULL after a success) is made due, and a
  repo in backoff (`next_retry` set) keeps its wait.
- **Ruled out.** A lost retry: the row stays due later, and backoff is capped at 300 s
  (`MAX_BACKOFF`), so the push comes at most 5 min late. It looks harmless.
- **Guess.** Not known which is meant: the comments, or the code (keep a backoff across
  restarts). Either the comments or the `WHERE` should change.
- **Status.** Fixed in this change. The owner chose the comments' behaviour (a restart is usually
  someone fixing the remote): start-up now makes every configured backup due, one in backoff
  too. Only the next retry moves; the failure count and last error stay, so the status still
  shows them and a remote that is still down goes on backing off from where it was.

## 2026-11-18: two result files were stored with CRLF line endings

- **Where.** `bench/results/2026-11-17-f4-storage/baseline.jsonl` and `volume-checkout.jsonl`
  (#46), found by the Cloud's vendor sync to `d0102c0`.
- **Seen.** Both blobs on `main` had CRLF endings (6 lines each). `.gitattributes` asks for LF,
  so the Cloud's `git add` stored them LF, and `scripts/sync-margincode.sh --check` then found the
  vendored tree's sha256 different from the one it recorded.
- **Ruled out.** Not the sync script: `git archive` of `main` gives the CRLF bytes, and they are
  the only two blobs on `main` with a CR.
- **Guess.** They were written on Windows and committed through `av` (jj), which does not apply
  `.gitattributes`, so nothing normalized them.
- **Status.** Fixed in #64 (both stored LF). `av diff --check` now reports an added CRLF line
  (branch `claude/cloud-capabilities-offline-22d48v`); a commit that runs no check can still
  carry one.

## 2026-11-18: the patch route answered with the previous head

- **Where.** `GET /v1/tasks/{id}/patch?from=main`, called by hand right after `av validate` or
  `av checkpoint` returned, while preparing fix-plan PR 22 (F29) for review.
- **Seen.** On two occasions the patch did not hold the edit the checkpoint had just taken: it was
  the previous head's. The same request a few seconds later held it. `av validate` had already
  printed the new head.
- **Ruled out.** Nothing yet: not reproduced on purpose, and no test reads the patch right after a
  checkpoint.
- **Guess.** The route reads the task's head, or the workspace's files, before the checkpoint's
  write is visible to it.
- **Status.** Open. A wait of a few seconds before reading the patch has avoided it since.
  Not reproduced in the daemon itself (2026-11-18): a throwaway test in `tests/remote.rs` that
  edited a file, then called `checkpoint` or `validate` and at once `GET .../patch?from=main`,
  60 rounds, three runs, always got the new head (`X-Av-Head`) and the edit. Both routes write
  the head in one transaction before they answer. So the cause is more likely outside one
  daemon process's ordering: the client (which head it read the patch for), or the Windows bind
  mount the edits were made on (a write the snapshot did not see yet).

## 2026-11-18: the Docker image cannot push a backup over SSH

- **Where.** `deploy/Dockerfile` (the apt line), `deploy/docker-compose.yml` (the SSH key and
  `AV_BACKUP_SSH_COMMAND` comments), found reviewing fix-plan PR 24 (F7, F8, F19, F20).
- **Seen.** The image installs `git ca-certificates curl` with `--no-install-recommends`, and
  `ssh-client` is only a Recommends of Debian's `git`: `which ssh` finds nothing in the image. With
  `openssh-client` added, ssh still exits "No user exists for uid 4242" for a UID with no passwd
  entry, and compose runs the daemon as the host `${UID}` (its own comment says that UID has no
  passwd entry). So the compose file's SSH backup comments describe a setup that cannot work.
- **Ruled out.** Lite mode: the daemon runs on the host and uses the host's ssh.
- **Done for now.** `docs/reference.md` ("Getting main to GitHub (self-hosted)") sends the Docker
  stack to a bare repository on a mount and a push from the host.
- **Guess at a fix.** Add `openssh-client`, and give ssh a user it accepts (for example a passwd
  entry written at start, or `nss_wrapper`), then a test that pushes a backup over SSH from the
  image. Or drop the compose comments.
- **Status.** Fixed on `claude/cloud-capabilities-offline-22d48v`: the image installs
  `openssh-client` and `libnss-wrapper`, and its entrypoint (`deploy/entrypoint.sh`) gives a UID
  without a passwd entry one through nss_wrapper before it starts the daemon. Checked on the
  image's runtime layer (same base and packages, a stand-in daemon): as UID 4242, `ssh` without
  the entrypoint exits "No user exists for uid 4242"; with it, a `git push` over SSH to an sshd
  container succeeded. Checked 2026-11-18 on the real image from `main` (105f99f) and the compose
  stack as UID 4242: the smoke steps pass, the daemon pushes main to an sshd container over SSH
  after a merge, and without the entrypoint the push fails with "No user exists for uid 4242".
  Not checked: `scripts/smoke.sh` run as is on a Linux host.

## 2026-11-18: M2 stage B, three stops before the first session

- **Where.** M2 stage B (`bench/results/2026-11-13-standing-and-dreams/m2-stage-b/checks/`).
- **Seen.** (1) The first `dream-freeze` stopped at its first agent call: Claude Code answered
  "Invalid API key" and a direct models request returned 401; the owner replaced the key and the
  freeze ran again from a fresh seed ($0 spent). (2) The smoke's preflight refused twice:
  `first-packs` and the freeze leave run and workspace directories open to the agent user, and
  `useradd -m` gives `benchagent` a home it can write. Closed with `chmod 700` and
  `usermod -d /nonexistent benchagent`; `m2bchain.sh` now closes `ws/*` too between the arms.
- **Ruled out.** No session ran in either state; the harness refused before launching.
- **Guess.** (2) is setup for a new container: M0's container had been set up that way already.
- **Status.** Worked around. A harness recipe for a new container could name both steps.

## 2026-11-18: a two-second lease ran out inside a routing test under load

- **Where.** `margincode-daemon` `tests/routing.rs`
  `the_reviewer_who_requested_changes_does_not_rescue_the_task` (line 590), in the full
  `cargo test --workspace` run for fix-plan PR 21 (F6).
- **Seen.** Right after grok's `request_changes`, which renews the owner's lease, `next` for
  `codex` returned the task `from: abandoned` where the test expects nothing offered. Alone the
  test passed five runs out of five (5.9 s each).
- **Ruled out.** Not F6's change: F6 touches the merge message and the commit author, not
  leases, reviews or `next`.
- **Guess.** The test's lease is 2 s; with the whole workspace's tests running, the time from the
  renewal to the two `next` calls passed 2 s. A longer lease with the sleeps scaled to it would
  hold.
- **Status.** Fixed on `claude/cloud-capabilities-offline-22d48v`: the test's lease is 5 s and
  its sleeps 5.5 s, so a loaded run has 5 s from the renewal to the two `next` calls. Then fixed
  for the whole family by the pull request "Tests set state instead of waiting": the test runs on
  the default lease and `expire_lease` writes the lease lapsed where it slept, so no lease can run
  out by itself inside a test (the rule is in `CONTRIBUTING.md`, "Timing").

## 2026-11-18: the Workers Builds preview of #49 failed

- **Where.** The Cloudflare Workers Builds preview check on #49 (`constitution.md` and this file;
  build `63832ff8`).
- **Seen.** The preview build failed. `main`'s production build after #49 merged passed.
- **Ruled out.** Not the change's content, as far as can be told without the log: #49 touched only
  Markdown, and the same tree built on `main`.
- **Guess.** Something in the preview environment, not the repository. The build log was not read.
- **Status.** Answered (2026-11-19, the builds API): the build was cancelled, not failed. Each
  branch gets its own preview trigger; the merge deleted the branch, and Cloudflare stopped the
  build while `npx wrangler preview` was still installing wrangler, then deleted the trigger
  (about 35 s after it was made). #75's red check is the same. A red preview on a merged PR whose
  build reads "cancelled" says nothing about the change; wait for the preview before a merge to
  have it say something.

## 2026-11-18: which retrieval task lost a signature at 600 tokens

- **Where.** `margincode-bench` `tests/retrieval_gate.rs`, on #48 (F17).
- **Seen.** The pack's `symbols_sig` mean at budget 600 fell from 0.833 to 0.778 (half of one
  task's gold signatures, of 9), while at 300 it rose from 0.333 to 0.444 and `files_code` from
  0.444 to 0.556. The floors were re-set from the new means.
- **Ruled out.** Not the goals' length: the fixture's goals are one line each.
- **Guess.** The task handle is now reserved before the section shares, which moves a few tokens
  between sections at tight budgets.
- **Status.** Answered (2026-11-18, the gate with a per-task print at #48's parent `6721a56` and
  at `e515440`): at 600, `cache-key-order` lost its one signature of two (0.5 to 0; every other
  task unchanged, `token-skew` was already 0); at 300, `aging-due-today` gained its file and its
  signature (0 to 1). Why `cache-key-order`'s signature no longer fits at 600 is not looked at
  yet. A per-task line in the gate's report would answer the next such question at once.
  The gate prints that line (each task's scores under each budget's means) since this change.
  With it: at 600 on `main` the pack shows both of `cache-key-order`'s gold symbols, as signature
  lines without `export` (`function cacheKey(...)`, 501 of 600 tokens used), and `score` does not
  count them because it looks for the source line `export function ...`; so no section takes the
  tokens. `token-skew` (an exported TypeScript function, 0 at every budget) may be the same miss
  (guess, its pack not looked at).

## 2026-11-18: `av` on Windows does not read a Git Bash path

- **Where.** `av record --json FILE` in Git Bash with `av.exe`.
- **Seen.** `av: cannot read /c/Users/.../records.json: The system cannot find the path
  specified. (os error 3)`. The same file passed as a Windows path (`cygpath -w`) was read.
- **Guess.** `av.exe` is a Windows program and Git Bash converts paths only for arguments it
  recognises as paths; with `MSYS_NO_PATHCONV=1` (which `docs/reference.md` recommends for `/import/...`)
  it converts none.
- **Status.** Documented in #60 (F2, F13): `docs/reference.md` says Git Bash converts only
  arguments and that a Windows program needs `C:/...` (`cygpath -m`). `av` still reads such a path
  as given.

## 2026-11-17: a checkpoint during another task's start took 2.2 s once

- **Where.** The measurement for #46 (`bench/results/2026-11-17-f4-lock/`), on a throwaway daemon
  on the Windows bind mount.
- **Seen.** With the write outside the repo lock, seven of eight checkpoints during another start
  took 370-378 ms; one took 2,236 ms. On `main` before the change they took 4.7-9.7 s.
- **Ruled out.** Not reproduced in five more runs, or on the dogfood daemon (369-381 ms).
- **Guess.** The host's disk, or the snapshot meeting the start's index overlay.
- **Status.** Open.

## 2026-11-17: checkpoints after widening every archive take 4.2 s

- **Where.** `av widen` on a sparse workspace of this repository (#45).
- **Seen.** Widening one result directory took 14.9 s and checkpoints after it stayed under a
  second. Widening all of `bench/results/` made each checkpoint 4.2 s.
- **Guess.** The snapshot walks every widened file on disk; a path that a sparse pattern already
  covers could skip the walk.
- **Status.** Open.
