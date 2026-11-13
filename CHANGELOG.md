# Changelog

## Unreleased

- **`margincode-bench run|first-packs --wrong-reverted`: a wrong note as the record of reverted work.** The pool's `no_commit` notes (the wrong-lesson pool's wrong notes) are seeded as the records of a second earlier task. Before the agent's task starts, that task is merged and its merge reverted, so their standing is `reverted`. Its one file (`AV_BENCH_REVERTED.md`) is gone again, so the agent's workspace is still the base tree. Without the flag every note is an open earlier task's record, as before. This is the harness side of Plan 16's M1, pre-registered and not run in `bench/results/2026-11-13-standing-and-dreams/PREREG.md`, together with M2's raw-against-dreamed comparison.
- **No open address without a token.** `AV_LISTEN` now defaults to `127.0.0.1:7420` (was
  `0.0.0.0:7420`), and the daemon refuses to start on a non-loopback address unless `AV_TOKEN` is
  set or `AV_LISTEN_OPEN=1` says something else limits who can reach it. The compose file sets
  `AV_LISTEN_OPEN=1`, since it publishes the port on the host's `127.0.0.1` only. A deployment that
  sets `AV_LISTEN=0.0.0.0:...` with no token must add one of the two. The Docker image sets `AV_LISTEN=0.0.0.0:7420` (a published port reaches nothing else), so a plain `docker run` needs `-e AV_LISTEN_OPEN=1` beside `-p 127.0.0.1:7420:7420`, or `-e AV_TOKEN=...`.
- **`av dream` and `av verdict`: the agent side of a second look at records.** `av dream --repo R -- <agent command>` asks the daemon for a run, writes what the run is handed (`bundle.json`) and the instructions (`integrations/dream/DREAM.md`, built into the CLI, its SHA-256 stored on the run) into `$AV_HOME/dreams/<run id>/`, runs the agent there and posts the `verdicts.json` it wrote; `--verify` does the same for the second look (`reviews.json`). What the daemon refuses goes back to the agent once (`--fix-rounds`). A command that fails, runs past `--timeout` or writes no file ends the run as failed and posts nothing. `--max-agents` shares the host's agent slots with `av watch`. `av dream undo|show <run id>`, and `av verdict list|show|accept|reject|retract` for a person's second look. New route `GET /v1/verdicts/{id}`: one verdict with the record it judges. Tested with agent scripts that run no model; no real model has dreamed yet.
- **Extensions are asked before shared memory changes.** `MergeGuard` gains `Guarded::MemoryWrite { repo, action, by, dream }`, asked once the daemon's own rules allow the write and before anything is written: before the review of a proposed verdict (`MemoryAction::VerdictReview`) and before a record supersedes another (`MemoryAction::Supersede`). A refusal is `422`, and the verdict stays proposed or the record live. `AllowAll` allows both, so Community behaves as before. A guard with an exhaustive `match` on `Guarded` needs the new arm. A review of a verdict that is no longer proposed is now refused (`409`) before its evidence is stored, as before, and before the guard is asked.
- **The project is now Margincode** (it was Scholion; that name is in use elsewhere). The CLI is `margincode`, the daemon `margincode-daemon`, the crates `margincode-*`, the site `open.margincode.dev`. **`av`, the `AV_` variables, the HTTP API, the MCP server and the hooks do not change**, so scripts and agent configurations keep working. What an upgrade keeps: a data directory `~/.scholion` is still used when it exists and `~/.margincode` does not (new installs get `~/.margincode`); lite mode's database and the Compose project keep their names, since a new one would start on empty data; `scripts/install.sh` still installs a release made under the old name and still reads `SCHOLION_VERSION` and `SCHOLION_INSTALL_DIR`. What changes for an existing install: the binaries `scholion` and `scholion-daemon` are no longer shipped (use `av`, or `margincode`); the Docker image's user is `margincode` (uid 1000 as before); the Claude Code plugin is `margincode@margincode`. Earlier entries below, the plans under `docs/superpowers/` and the benchmark results keep the old name: they are history.
- **Dreams and verdicts: a second look at records** (the daemon's side). A run started with `POST /v1/repos/{repo}/dreams` is handed the records where a second look is most likely to pay (reverted or abandoned work, anchored code that changed, records that share an anchor, records packs show most) with their task and the code on `main`, and proposes verdicts: `confirmed`, `disputed`, `duplicate`, `outdated`. The daemon runs no model: it refuses a verdict whose record is not on the run's list or whose evidence is not found word for word in the file, record or event it cites. A verdict has no effect until a `verify` run with a quote of its own, or a person, agrees (`POST /v1/verdicts/{id}/review`); then a disputed record ranks at 0.3 and says why, a duplicate or outdated one leaves packs and the pre-read hook's answer, and a confirmed one loses its "changed since recorded" mark until the code changes again. Records are never edited; `retract` and a run's `undo` put packs back byte for byte. Migration 0031 (`dreams`, `record_verdicts`, `verdict_reviews`). `PACKER_VERSION` 10.
- **A record's standing: where it comes from.** A pack served a decision of a task whose merge was reverted, or that was abandoned, like one of merged work. Now another task's record is printed with its origin unless that is merged work (`decision, reverted:`, `abandoned`, `open`), in packs, the pre-read hook's answer and `av expand`, and a claim of reverted work ranks at 0.4 and of abandoned work at 0.6 (an `attempt` keeps its rank; merged and open work rank as before; nothing ranks above merged work). No table and no migration: the origin is computed from the task's status and its last `revert` event, in one query per pack. `GET /v1/tasks/{id}/records` and search add `standing: {origin, factor}`. `PACKER_VERSION` 9.
- **The summary limit is said before an agent writes.** In a benchmark run 53 `av record` calls in 28 of 30 sessions were refused for a summary over 200 characters and retried. `av record --help` now states the limit for `--summary` and for the entries of `--json`, and the Stop hook's request for records names it.
- **`margincode-bench run --agent-user`: a refused preflight names a path it could not look at.** A directory the preflight may have made, and then could not look at, was dropped from the clean-up without a word; the refusal now names it with the `chmod 700` that closes it. The run report's method paragraph no longer says that every relay note is a commit message (a note marked `no_commit` is in `NOTES.md` and the records only).
- **Tests run offline on a machine with PostgreSQL installed.** With no `AV_TEST_DATABASE_URL`, `margincode-testutil` downloaded an embedded Postgres on first run, which fails without network access or behind a proxy that refuses the download. It now first looks for an installed PostgreSQL (`AV_TEST_PG_INSTALL_DIR`, `pg_config`, the usual install directories) and starts a private server per test from its binaries in a temporary directory, as `postgres` or `nobody` when the tests run as root; the server and its files are gone when the test ends, and an installed service is never touched. `AV_TEST_PG_INSTALL_DIR=none` keeps the download.
- **`margincode-bench run --agent-user` leaves no run directory open.** A finished run sets `<work-root>/runs/<run id>` to 700 (a resume opens it again while it runs), a refused preflight removes the empty directories it made and puts back the modes it changed, the refusal on another run's open directory names it with the `chmod 700` that closes it, and `rows.jsonl`, `meta.json` and `report.md` are mode 600 from their first byte and after a resume that adds no row. Without `--agent-user` nothing changes.
- **An agent can correct a stored record.** `supersedes` was in the store and the HTTP API, but
  no agent could reach it, and nothing in a pack said that a record is an earlier agent's claim:
  in a benchmark an agent handed a confidently wrong record followed it, where the same text in a
  notes file was simply edited. Now `av record … --supersedes rec:…` (also per entry of `--json`),
  the MCP `record` tool's `supersedes` and the API's `supersedes` all take the `rec:` id a pack
  prints (a prefix of 6 or more hex characters or a full id too, with or without hyphens), and the
  answer names what was superseded. The old record is kept as it was and marked superseded; a
  correction without anchors takes the old record's, so it is shown where the wrong one was. What
  corrects a decision, attempt or note is a decision, attempt or note (another kind would not be
  shown in its place: `422`). Refused: a record of another repo (`422`; it was accepted before,
  which let one repo's task hide another repo's record), a `review` or `validation` record
  (`422`), another task's `intent` or `handoff` (`422`; a task restates its own with the same
  kind), an already superseded record (`409`, naming the live one when the chain of corrections
  is at most 64 long; it was accepted before, which left a record with two successors), an unknown id (`404`, was `500`). The store enforces these
  inside the insert with the old record's row locked, so two tasks superseding one record at once
  leave it one successor. A `submit` whose inline records are refused writes none of them, two
  that supersede the same record included (`409`, naming both entries). The feed gets `record:superseded` (`{old, new, by}`). A pack that shows
  another task's decision, attempt or note says in one line under *Decisions & dead ends* that
  these are claims to check against the code and how a wrong one is corrected (25 tokens, inside
  the budget, and no room kept for it by a pack that only refers back to such a record;
  `PACKER_VERSION` 8), and the pre-read hook says it once per session. The MCP tool list stays
  within its size guard: `start_task` lost its empty `required` list, `submit`'s inline records
  the receipt description that `record` already carries (they list `supersedes` too) and
  `records` a description that repeated the tool's.
- **`margincode-bench` can run Grok.** `--vendor grok` launches `grok -p … --output-format json --always-approve --max-turns N --cwd` and reads `usage.input_tokens`, `usage.cache_read_input_tokens`, `usage.output_tokens`, `total_cost_usd` and `num_turns` into the session usage. It does not pass `--max-budget-usd`; the turn cap stands in for it. Reasoning tokens are not kept: the usage total has no field for them.
- **`margincode-bench routing-profile`** reads an E-G1 results file and prints accepted rates, Wilson intervals, and the `av agent set` lines the decision rule implies. A tie, or a cell with fewer than 10 trials, is inconclusive and does not replace that agent's profile.
- **Containers started without the labels are footprint too.** With `docker` on `PATH`, the
  session-start snapshot also keeps the id of every container then (`docker ps -aq --no-trunc`),
  and in a task the Stop hook reports the containers that are new since and carry no `av.task`
  label as `containers.unlabelled` (`{id, name, state}`) with the footprint; `av disk` shows them
  under the session. Running ones count toward `AV_FOOTPRINT_CONTAINERS` with the task's labelled
  ones, and the block names them and says to add `$AV_DOCKER_LABELS` next time. `av gc
  --containers` still never touches an unlabelled container. Without docker, or without the ids
  from session start, nothing changes.
- **`GET /v1/disk?fs=1`: the filesystem numbers without the walk.** With `fs=1` (or `fs=true`) the
  answer carries only `fs_free_bytes`, `fs_total_bytes`, `min_free_bytes`, `below_floor` and
  `build_budget_bytes`, measured as the free-space floor measures them, and reads neither the store
  nor any workspace, `target/` or footprint; without the flag the answer is unchanged. A monitor
  that polls free space (Margincode Cloud's `disk_low` alert, every 60 s) no longer pays for a walk
  of every workspace. `av disk --fs` prints the line, `--json` the object.
- **Containers are footprint.** Everything a session starts through docker is to carry the
  labels `av.task=<task id>` and `av.agent=<agent>`: in a task the session-start hook exports
  them as `$AV_DOCKER_LABELS` for `docker run`/`docker build` and, with `docker` on `PATH`, says so
  in one line. The Stop hook then lists the task's labelled containers, images and volumes with
  the footprint (`av disk` shows them under the session) and blocks once per session while more
  than `AV_FOOTPRINT_CONTAINERS` (default 0) of its containers still run, naming them; it never
  stops one. `av gc --containers` removes the stopped containers, dangling images and volumes of
  merged and abandoned tasks (running ones are listed, open and unknown tasks' things left alone;
  `--dry-run` plans, `--json` carries it as `containers`). The daemon runs no docker: the CLI does.
- **`AppState::with_vcs_layer`: an extension can wrap the version-control engine.** Called from
  `run_with`'s `customize` step, it hands the extension the jj engine and uses what it returns
  everywhere from then on: the API, the packer, the backup loop and the disk sweep. A layer may
  forward every call and observe it, or replace the engine.
- **`AppState::with_checkpoint_hook`: an extension can add to a checkpoint's answer.** Asked
  after `POST /v1/tasks/{id}/checkpoint` (a remote checkout's patch too) with the task, the new
  head and the changed paths; the object it answers is added beside `head_change`, `changed` and
  `warnings` (the daemon's fields win), and its `notices`, lines for the agent, reach the agent
  through the MCP `checkpoint` tool's answer and the Claude Code `post-edit` hook, which prints
  them as `additionalContext`. A hook that fails adds nothing and never fails the checkpoint.

## 0.0.3 (2026-11-06)

Agents can now propose backlog items that wait for a person's accept (`av post --propose`,
`av backlog accept`), and `av attention` (`GET /v1/attention`) lists what needs a person or an
agent. `av repo advance` brings work merged on the remote back to main, and `av notes` puts a
task's decisions and dead ends on its pull request. A new repo can start with records mined from
its git history (`--mine-history N`). `embeddinggemma-2` is the default Ollama embedder, and
Ollama may run in another container or host. A head takes one approval per review round, and the
dashboard is redesigned. Lean disk and lean builds reclaim finished tasks' workspaces and their
build output. Migrations 0027–0030 run on the first start; a 0.0.2 `AV_HOME` upgrades in place.

- **Ollama on another container or host.** `AV_EMBED_PROVIDER=ollama` no longer needs an `ollama`
  CLI on the daemon's `PATH`: with none, the daemon sends `AV_OLLAMA_EMBED_MODEL` (now required in
  that case, for example `embeddinggemma-2`) as written to the Ollama server at `AV_OLLAMA_URL`'s
  `/api/embed`, as a daemon in one container and Ollama in another need. Nothing connects at start:
  the daemon boots while that server is down, and a missing model or a server that is down is
  logged at the first embedding (the outbox keeps the work and retries). With the CLI, selection is
  unchanged (`ollama list` resolves the name to a pulled tag), and so is automatic selection with
  no CLI (`AV_EMBED_URL`, else off). The `api` path (`AV_EMBED_URL`) speaks text-embeddings-
  inference's `/embed` and cannot talk to Ollama, which serves `/api/embed` and `/v1/embeddings`.
- **The CLI says why a request was malformed.** An error the daemon answers in plain text rather
  than its JSON `error` (a path it cannot parse, such as `av backlog accept SLUG` or `av review`
  with a short id, which need a full id) was printed as `request failed (HTTP 400)`; the CLI now
  prints the text (for a slug: that `id` is not a UUID; at most 300 characters). Found by the
  0.0.3 release check.
- **Proposals: agents queue what they are sure of and propose the rest.** A backlog item can be
  `proposed` (`POST /v1/backlog` with `propose: true` and an optional `why` of at most 300
  characters; `av post --propose --why`): `next` never offers it until a human accepts it with
  `POST /v1/backlog/{id}/accept` (`{by}`; `av backlog accept ID`), which queues it, adds a
  `backlog:accepted` event and wakes waiting `next` calls; `av backlog drop` drops one. Accepting
  an item that is not proposed is a 422. The repo setting `agent_posts` (`queue`, the default, or
  `propose`; `av repo settings NAME --agent-posts`) makes every agent's post a proposal: a post is
  an agent's when it comes through the new MCP tool `post_backlog` (repo, title, goal, propose,
  why, target, after) or its `posted_by` is an agent profile's name or alias. A proposed item is
  not counted in its target's `items`/`queued` (`proposed` counts it apart); an item `--after` it
  waits until it is accepted and merged; an accepted item's reserve window starts at the accept.
  `MergeGuard` is asked `Guarded::AcceptBacklog { item, by }` before an accept (`AllowAll`
  allows it), so an extension can keep agents from accepting their own proposals. `GET
  /v1/attention` adds the low-severity kind `proposed_backlog`; `av backlog` and the dashboard
  show proposals with who proposed them and why. Migration 0030 (`backlog.state` adds
  `proposed`; `backlog.why`, `accepted_by`, `accepted_at`; `repo_settings.agent_posts`). The MCP
  tool list grows by 477 bytes (its guard in `tests/mcp.rs` now allows 10,800).
- **One approval per head** (Plan 15 Phase 0, "Loop noise"). In the first dogfood run two reviewers
  approved the same head seconds apart, the second leaning on the first one's validation. `POST
  /v1/tasks/{id}/review` with `approve` on a head whose newest verdict in the current review round
  (since the task's latest submit) is already an approval is now refused (422) with who approved it
  and when; `request_changes` on an approved head, an approval after a resubmit (a new head, or the
  same head after a request for changes or a revert), and the board's merge of an approved task work
  as before. A review takes the task's lock, so two concurrent approvals cannot both pass. A finding
  the reviewer already stored at that head (the same summary, body and anchors, the anchors in any
  order, as a retried `av review` sends) is not stored again. `next` already kept a task offered
  for review from a second reviewer (the 600 s review reservation). No migration: the verdict's
  time is `review_verdicts.created_at`, the round starts at the newest `submit` event
  (`IntentStore::round_verdict_at`).
- **Scheduled regressions** (Plan 15 Phase 3). `.github/workflows/nightly-scale.yml` runs every
  night against a release-built daemon with no semantic layer: `scholion-bench scale` on tokio
  (`tokio-1.54.1`), failing when one-agent p95 latency rises above 600 ms (start), 300 ms (context)
  or 100 ms (checkpoint), when merges/s falls below 1.5 at 32 agents or 1.2 at 100, or when any
  level does not merge every agent; and `scholion-bench retrieval` on the E1 CI fixture with the
  E1 gate's floors (`bench/retrieval/ci/make-repo.sh` builds the fixture repo and its task lines).
  `.github/workflows/e4-smoke.yml` runs the E4-at-scale smoke every Monday (relay-customer-region,
  `av` arm, `--noise 50`, $0.02) on DeepSeek with the repository secret `LLM_API_KEY`, reports
  whether the start pack shows the lesson (`pack-hit.json`), and skips with a notice when the
  secret is not set. Neither runs on pull requests; the E1 gate in `ci.yml` is unchanged.
- **Needs attention, for machines: `GET /v1/attention` and `av attention`** (Plan 15 Phase 2).
  What the dashboard's strip computed in the page is now the daemon's: red main, conflicted tasks,
  reviews nobody gave, approvals nobody merged, handoffs nobody claimed, open tasks nobody wrote to
  for a lease, backlog items no agent fits, overdue targets and big footprints, each
  `{kind, severity, repo, task_id?, slug?, agent?, since, title, detail}`, worst first. The page reads
  it (the strip and the cards' stuck marks look the same; conflicted cards are marked at once). The
  page's thresholds are `AV_ATTN_REVIEW_SECS`, `AV_ATTN_APPROVED_SECS`, `AV_ATTN_HANDOFF_SECS`,
  `AV_ATTN_FOOTPRINT_BYTES` and `AV_ATTN_FOOTPRINT_SECS` with the same defaults. `av attention
  [--repo R] [--json]` prints one line per item and exits 0. `GET /v1/repos/{name}/health` adds
  `since_at` to a red answer. No new table.
- **Main comes back from the remote: `av repo advance`** (Plan 15 Phase 0). The backup can push
  main to a branch for a pull request, but nothing brought the remote's `main` back, so work merged
  there (that pull request, and others) never reached main and the next task started on a stale
  base. `POST /v1/repos/{name}/trunk/advance` (`av repo advance NAME [--remote URL] [--branch B]`;
  defaults: the backup remote and `main`) fetches the branch and fast-forwards main to it when it
  descends from main, then re-indexes main and emits `trunk:advanced`; `409` with both heads when
  they diverged, main unchanged. `av repo backup NAME --remote URL --advance` makes every backup
  push advance first (migration 0029 adds `backup_state.advance`). New `Vcs::advance_trunk` with
  the `Advance` outcome.
- **A guard can refuse a review verdict: `Guarded::Review`.** `MergeGuard` is now also asked
  `Guarded::Review { task, reviewer, verdict }` for every verdict, `approve` and
  `request_changes`, once the task is known to be in review and the reviewer is not its owner,
  before anything is written. A refusal is HTTP 422 (`cannot review: <reason>`) and leaves the task
  in review with no verdict and no `review` records. An approval is still asked as
  `Guarded::Approve` afterwards, so a guard that knows only that question keeps working.
  Extensions with an exhaustive `match` on `Guarded` need a `Review` arm.
- **A change request keeps the task with its owner.** `request_changes` renews the owner's lease
  before it reopens the task, and `next` no longer gives the reviewer who requested the changes
  that task as an abandoned one once the lease runs out (another agent that fits still may; the
  owner still resumes it). Found by the 2026-11-03 dogfood: the owner's lease had run out while the
  task waited in review, so the reviewer's very next `next` took the task it had just sent back.
- **`embeddinggemma-2` is the default Ollama embedder.** On the E1 retrieval set (42 tasks, six
  repos) it scored the same as `qwen3-embedding` (symbols shown 0.59 vs 0.57 at 2000 tokens, 0.81 vs
  0.83 at 8000; files with code shown equal) at 271M parameters and 512 dimensions instead of 7.6B
  and 4096. The daemon now takes `embeddinggemma-2` when `ollama list` has it, else
  `qwen3-embedding`, so a host with only the old model keeps it and its vectors. After pulling
  `embeddinggemma-2` on such a host, run `av reindex` (no `--repo`) once: the collections are
  rebuilt at the new dimension, and until then the outbox worker logs a dimension mismatch that now
  says so and packs fall back to full-text. `AV_OLLAMA_EMBED_MODEL` (now in `docs/reference.md`)
  still names a single model.
- **Site refresh.** The Mission section, the problem intro and the many-agents heading follow the
  solo-first hero; a screenshot of the recorded run sits under the hero; the Claude Code plugin
  install (`/scholion:setup`) is on the page and in `site/llms.txt`; the guides show and declare
  their dates and author, `/demo` has structured data, the sitemap has `lastmod`, the 404 page is
  `noindex`; `og.png` names the new domain and is rendered from `scripts/og.html`. The page is wider
  (1320 px), the How-it-works steps sit two by two, and no command block scrolls sideways.
- **Claude Code under its own name, and a full first pack.** `av --agent NAME init --for claude`
  adds the MCP server with `X-Av-Agent: NAME`, so Claude Code's MCP calls are made under the name
  `av next` and the hooks use rather than its client name (without `--agent`, or with the default
  `agent`, nothing changes; a server already configured is left alone and the output says how to
  re-add it). The `SessionStart` hook starts a new pack session at `startup` too, not only after
  compaction or `/clear`: the pack `av next` or `av task start` printed went to whoever ran
  them, so a Claude Code process started in the workspace got only "sent earlier" handles. Both
  found by the first Teams run (Scholion Cloud's Plan 13 Phase 4).
- **Solo-first pitch.** The README, the site hero, `site/llms.txt` and the `/scholion:setup` skill lead
  with one agent on one repo ("Your next Claude Code session starts knowing last session's dead
  ends"); the several-vendor relay comes second. `av init --for codex` now also writes the
  `X-Av-Agent` header, like Grok, Gemini and Cursor (docs updated).
- **Records on the pull request: `av notes`** (Plan 13 Phase 2, the PR bot). Renders a task's
  decisions, dead ends, notes, review findings, handoffs and checks, plus what other tasks recorded
  about the files it changes, as one Markdown block between `<!-- scholion:notes v1 task=<id> -->`
  markers; reads `/v1` only. `--pr [N]` posts it through `gh`, or edits the comment that already
  holds the task's block (other text and other tasks' blocks stay; an unchanged block is not
  sent). The task is an id, a slug or `repo/slug`; `--format json`. Record text cannot close the
  markers and its `@names` notify no one. The CLI posts because the daemon runs on the
  developer's machine, out of a GitHub runner's reach; `integrations/github/` has the format and a
  sample workflow for a self-hosted runner that can reach it. `integrations/README.md` lists every
  integration.
- **Cold start from git history** (Plan 13 Phase 2). `av repo add NAME PATH --mine-history N`,
  `av init --mine-history N` and `av repo mine NAME [--limit N]` (`POST /v1/repos/{name}/history`)
  turn at most N commits of the repo's history (every ancestor of main, newest first) into
  records: one `note` by the agent `git` per commit with one parent, a subject that is not
  housekeeping, a message that says something (trailers and links dropped) and at most 25 files,
  anchored to the files it touched, with the commit as its `change_id`. They live on one merged
  task per repo, `<repo>/git-history`; no migration. Mining again skips what was mined. Packs rank
  them at half weight (`MINED_FACTOR`, packer version 5), `records/at` lists them after agents'
  records, every rendering names the commit (`note (git 1a2b3c4)`), and `av why` on imported
  history prints the blamed commit's mined record. The commit rules (`housekeeping`, the message
  cleaning) moved from `scholion-bench mine` to `scholion_core::history`, which both use; `Vcs`
  gains `history` (all ancestors, default `trunk_log`).
- **Bench: E4 at scale, `run --noise N`** (Plan 13 Phase 1, harness only; nothing run yet). Adds N
  deterministic irrelevant notes to every relay task, the same ones in the `notes` arm's
  `NOTES.md` and as the `av` arm's anchored records: a seeded generator over the base tree's own
  files and symbols, anchored everywhere (the lessons' files too), spread among the lessons, each
  level the first N of the next. `scholion-bench noise` prints them. Rows gain `input_tokens`
  (cache reads included) and `memory_tokens` (`NOTES.md` or the start pack); a noise run's report
  names its level and adds a "Memory at scale" table; a results directory holds one level. Design,
  levels, metrics and a per-cell cost estimate: `bench/results/2026-11-e4-scale/DESIGN.md`.
- **Remote workspaces by checkpoint-by-patch** (Plan 14). An agent on another host works in a git
  checkout of its task (`av remote clone TASK DIR`, `av task start --checkout DIR`) and each
  checkpoint goes to the daemon as a patch (`POST /v1/tasks/{id}/checkpoint?base=`), which the
  daemon applies in process to the workspace it still owns and snapshots as a local checkpoint;
  a stale base is `409` with the head and `av checkpoint` pulls (`GET /v1/tasks/{id}/patch?from=`,
  `av remote pull`) and resends. Hooks, `submit`, `record`, `handoff`, `validate` and `sync` in
  a checkout send its edits first. The daemon runs no git for it. Reference: "An agent on another
  host"; plan and measurement: `docs/superpowers/plans/2026-11-04-scholion-plan-14-remote-workspaces.md`.
- **Name fan-out cap** (`AV_INDEX_MAX_FANOUT=N`, off by default). The code graph links a call by its
  bare name to every symbol of that name, so `new` alone gave tokio 500k of its 896k edges. With a
  cap, a name more than N other symbols define links to none of them; overlay views apply it to
  the names as the task sees them, and the overlay equivalence proptest runs under a cap of 1 too.
  E1 on the public task set: pack scores unchanged at caps 64 and 16 but for one task at one
  budget, edges −40% / −54%; on tokio a cap of 64 drops 71% of the edges
  (`bench/results/2026-11-03-e1-fanout-cap/`). It applies to repos imported after it is set.
- **Lean builds, one target dir per task** (Plan 12). Build output was 99.99% of the disk the
  first dogfood run took: about 20 GB per Rust task (14 GB of test binaries that were 73% debug
  info, 4 GB of incremental cache) against 3 MB of Scholion's own state.
  - The per-repo cargo config the daemon writes now sets `incremental = false`,
    `debug = "line-tables-only"` (no debug info at all for dependencies) and **no `target-dir`**:
    every task workspace builds into its own `target/`, removed with the workspace. Measured on
    this repo, a full `cargo test` build is 4.3 GB instead of 21 GB.
  - **Fixes Plan 11's shared build cache**, which pointed every workspace of a repo at one target
    dir: cargo names a path package's artifacts by its path relative to the workspace root, so
    `cargo test` in one workspace ran another task's test binary (a false green or red
    validation; hit twice in the run). An av-written config from before is rewritten at start-up.
    A test runs `cargo metadata` in two workspaces of one repo and checks their target dirs differ.
  - **Build output goes with the merge.** A merge or abandon removes the workspace's `target/` at
    once (the answer says `build: removed|none|kept`; the feed gets `build:reclaimed` with the
    bytes) while the workspace itself waits for `AV_WS_RETAIN_SECS` as before, for an undo or a
    reviewer's look; `never` keeps both. A finished task no longer holds gigabytes for an hour.
  - `GET /v1/disk` and `av disk` show each workspace's build output apart (`build_bytes`, a `BUILD`
    column, a total); `av gc` says how much of a reclaimed workspace was build output. The
    dashboard's Disk card shows the same.
  - The snapshot never takes a workspace's top-level `target/`, even in a repo whose `.gitignore`
    forgot it (`/.av-task` was already ignored this way).
  - **One integration-test binary per crate** (this repo, not the product). Its `tests/*.rs` files
    are now modules of one test binary per crate, `it` (`tests/main.rs`; `cargo test -p
    scholion-daemon --test it api::NAME`), instead of 45 binaries that each linked every
    dependency: a full `cargo test` build of this workspace is 2.4 GB instead of 4.3 GB.
  - **Dependencies are built once per repo** (Task 5). When a task's build output is reclaimed,
    the units built from registry and git crates go into `<repo>/.cache/deps-target/`, and a new
    task's `target/` starts with them: compiled libraries hard-linked (one copy on disk for every
    task; rustc replaces them, never writes into them), dep-info, fingerprints and build-script
    output copied (those are rewritten in place). Measured on this repo: a seeded full `cargo
    test` build adds 1.2 GB instead of 2.4 GB and takes 15 s instead of 38 s, no dependency
    recompiled. The repo's own crates never enter the cache. `av disk` counts a linked file
    once; both steps run in the background (`build:reclaimed` with `cached_bytes`, `build:seeded`); `av gc
    --build-cache` empties the cache; `AV_BUILD_CACHE=off` keeps and seeds nothing.
  - **`av convention add` warns when a required check is scoped to every path** (`**`): such a
    check runs for every task, so a docs-only change pays the full build (the run's docs-only task
    paid 20 GB and 10 min). The hint says to scope it (`--scope 'crates/**'`); a change outside
    the scope then skips it, as `av validate` already does.
  - **`{crates}` in a check runs only the crates a change can affect** (Task 2). `av validate`
    replaces it with `-p a -p b`: the workspace members holding a changed path plus every member
    that depends on one of them, by any kind of path dependency (dev and build too), from
    `cargo metadata --no-deps` in the workspace. A changed path outside every member (root
    `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/`, `bench/`) or a failed
    `cargo metadata` gives `--workspace`. The line prints the command that ran, and the check's
    result keeps it (`command`); `GET /v1/tasks/{id}/checks` adds `changed`, the task's changed
    paths. Approval counts a result for a `{crates}` check only with an expanded `command`: run
    as written, `cargo test {crates}` filters every test out and passes, so an older `av` (or an
    MCP agent that leaves `command` out) cannot pass it.
  - **A task's builds stay in its own `target/`** (Task 3). A `CARGO_TARGET_DIR` in the
    user's environment (or `build.target-dir` in `~/.cargo/config.toml`) put every task workspace
    of a repo into one target dir again, where `cargo test` runs another task's test binary. In a
    cargo workspace, `av validate` (the command and every check) and `av-wrap.sh` set
    `CARGO_TARGET_DIR` to the workspace's `target/` and unset `CARGO_BUILD_TARGET_DIR`; when the
    user's environment names a target dir, the session-start hook adds shell code to
    `$CLAUDE_ENV_FILE` that does the same for every Bash command, for the task workspace it runs in
    (a session the Stop hook moves to another task follows it). That is the dir the dependency cache seeds and the merge
    reclaims. An explicit `--target-dir` still wins.
- **`av agent rm NAME`** (`DELETE /v1/agents/{name}`). Removes a profile. An unknown name is
  404. An agent that still owns an open, handed-off, in-review or conflicted task is 409 unless `--force`
  (`?force=true`); the tasks stay. A profile an extension manages is refused, the same way
  `av agent set` is.
- **Lean disk** (Plan 11). Agents' files no longer pile up:
  - **Finished tasks give their disk back.** A task merged or abandoned for `AV_WS_RETAIN_SECS`
    (default 3600; `0` at once, `never` to keep) loses its workspace and scratch dir; a sweep looks
    every 60 s. The task's commits stay in the store. An undo or revert that puts a task back to
    work restores its workspace at its head. Open, handed-off, in-review and conflicted tasks are
    never touched, and nothing outside `ws_root` is deleted. No migration.
  - **`av disk`** (`GET /v1/disk`): per repo, every workspace with its task's status, size, age and
    whether a sweep would remove it, plus scratch, build cache and store, and what sessions left
    outside any workspace. **`av gc [--dry-run] [--retain SECS] [--build-cache]`** (`POST /v1/gc`)
    reclaims on demand. Both take `--json`.
  - **A cargo config per Rust repo.** For a repo with a root `Cargo.toml` the daemon writes
    `<ws_root>/<repo>/.cargo/config.toml` (at import and start-up), which every task workspace
    inherits (what it holds: see Plan 12 above; the shared target dir it first set was withdrawn
    before release). The repo's own cargo config and `CARGO_*` still win; `AV_BUILD_CACHE=off`
    writes nothing.
  - **The hooks watch what a session leaves behind.** `SessionStart` gives a task a scratch dir,
    exported as `$AV_SCRATCH` through `$CLAUDE_ENV_FILE`, and takes a snapshot of the build,
    dependency and cache dirs under `$HOME` and the temp dir. `Stop` compares, blocks once when the
    session added `AV_FOOTPRINT_WARN_MB` (default 512) or more outside any workspace, and reports
    it to the daemon (`POST /v1/footprint`), which shows it in `av disk`. `AV_FOOTPRINT=off`,
    `AV_FOOTPRINT_ROOTS`, `AV_FOOTPRINT_SECS`.
  - `Vcs::remove_workspace` and `Vcs::restore_workspace` (jj: forget the workspace, re-create it
    at a commit).
  - `av up` moves a `daemon.log` over 50 MB to `daemon.log.1`.
  - **`scholion-bench run` removes each session's copy after verify**: the git arms' clone, and
    the av arms' import copy (their bench task is abandoned, so the bench daemon reclaims the
    workspace). Invalid sessions keep everything; `--keep-work` keeps every copy.
- **A redesigned dashboard** (`/ui`): a neutral, professional look in place of the pixel-slime
  theme. It adds a row of summary figures (in progress, in review, conflicted, merged, backlog,
  agents busy), agent cards with initials avatars, a state badge, skill tags and a load bar against
  `max_open`, a kanban board whose columns never overflow, side panels for targets, backlog and
  activity, and a slide-in drawer for tasks and agents. Light and dark follow the system, or pick
  one with the theme button (kept in this browser). The page reads the same `/v1` routes as before.
- **Main, the daemon and the disk on the dashboard.** Under the board, a Main panel lists main's
  last commits with their post-merge check (passed, failed with the exit code, or unchecked) and
  the task that landed each; its header says whether semantic search is on and how many rows wait
  to be embedded, the backup's state, how many conventions the repo has, and an import still
  running. A Disk panel (`GET /v1/disk`, read at start, on its button and every five minutes)
  shows what workspaces, build caches, the store and scratch dirs take, the repo's workspaces
  largest first with the ones a sweep would reclaim, and what sessions left outside any
  workspace; a session of this repo's tasks (or of no task) that left 512 MiB or more (the hooks'
  default) in the last day gets a "Needs attention" row. The page never deletes. When the page cannot load (the daemon
  is away at start, or the history cannot be read) it asks `/healthz` every five seconds and
  starts over once it answers; a feed that drops mid-session keeps reconnecting as before.
- **What needs attention, and how the work flows.** Every task card says how long it has been in
  its status ("in review 3h", "approved 20m" once its review is given), and a card goes amber once
  it is stuck (a submit with no verdict for 2 h, an approval not merged for an hour, a handoff
  nobody took for an hour, a conflict unresolved for an hour, an open task with no write for a
  whole lease). A "Needs attention" strip above the board lists, worst first, a red main,
  conflicted tasks, those stuck tasks, backlog items no agent can do and overdue targets (from
  the health, backlog and targets the page already reads); it is hidden when there is nothing
  and shows at most eight rows. A Flow panel shows, over 7, 14 or 30 local days, merges per day
  as a sparkline, the median cycle time (start to merge), the median review wait (submit to
  verdict, carried approvals left out), rework (changes requested, conflicts, undos, reverts),
  and per agent what it started, merged and reviewed, from each agent's timeline (read at most
  once a minute). No cost or model usage is shown.
- **The task drawer shows the work itself.** The diff from base to head, one file at a time with
  its added and removed lines; the task's own timeline (started, every checkpoint with how many
  paths it touched, claims, submits, verdicts, merges, conflicts); each review round (the verdict
  with the reviewer's findings at that head); a validation record as passed or failed, with its
  command and output; and, after "Build context pack", the pack's receipt: every item with its
  tier, why it is there, where it comes from and its tokens. The page reads the repo's event
  history (`GET /v1/events` in pages of 500, the newest 10 000 ids of a long one) and derives
  these from it; the daemon records nothing new.
- **The teams stand left of the board.** From 1580 px wide the dashboard is three columns: the
  agents' desks on the left, tasks by status in the middle, targets, backlog and activity on the
  right. Narrower, the desks are a row above the board, as before; below 1180 px everything stacks.
- **A `checkpoint` event**: a snapshot that moves a task's head (`POST /v1/tasks/{id}/checkpoint`,
  a submit, a sync) now lands in the event feed as kind `checkpoint` with the new head, the old one
  (`payload.from`) and how many paths the task changes (`payload.touched`), so `GET /v1/events`,
  `scholion events --follow` and in-process subscribers see work as it happens instead of polling
  `head_change`. A snapshot that moves nothing is no event.
- **Targets** (Plan 10 A3): `av target add --repo R --title T [--due YYYY-MM-DD]` names something
  the team works toward; `av post --target SLUG` (or `av backlog target ITEM SLUG`) counts a
  backlog item toward it, and `av target list` shows how many of its items merged, are being
  worked on or wait. `av target done|drop|reopen`. The dashboard shows each active target's
  progress. Migration `0027_targets`. Targets do not change routing.
- **Office and timelines on the dashboard** (Plan 10 A3): each Teams desk now says what the agent is
  on, what it did last, and whether it is working, reviewing, idle or quiet (open tasks with no write
  for a whole lease). Clicking a desk, or a task's owner, opens the agent's timeline: its records,
  starts, claims, review verdicts, submits and merges. New read-only routes `GET /v1/office` and
  `GET /v1/agents/{name}/timeline`, built from data Scholion already keeps (no migration).
- **`scholion-agentrun`**: `AgentCommand::env_clear()` starts the agent from an empty environment
  (later `env` calls still apply; `path_prepend` adds nothing after the prepended dirs when there
  is no `PATH`), and `stop_flag(flag)` kills the run's process group as soon as another thread
  raises the flag (`RunResult::stopped`; `process::run_until`). A runner uses them to keep host
  credentials away from an agent and to stop it at a budget.

## 0.0.2 (2026-11-01)

Several coding agents (Claude Code, Codex, Cursor, Grok Build, Gemini CLI) now share one backlog:
each takes the work that fits it, reviews the others' work, and merges what it approves.

### Install and run

- **The release now includes `scholion-daemon`, with its own Postgres (lite mode).** `av init`
  in a git repository starts it and adds the repo; no Docker needed. 0.0.1 shipped only the CLI,
  so the installer in the README failed on it.
- `av init --for codex|grok|gemini|cursor` wires those CLIs to the MCP server. Grok, Gemini and
  Cursor also get an `X-Av-Agent` header, so the daemon knows which agent is calling.
  `av --agent NAME init --for mcp-json --path FILE` does the same for any client with an
  `mcpServers` JSON file.
- **Ask Claude to set it up**: this repository is a Claude Code plugin marketplace. Run
  `claude plugin marketplace add vinhqbui/scholion`, then `claude plugin install
  scholion@scholion`. It adds three skills: `/scholion:setup` (install, `av init`, checks),
  `/scholion:team` (several agents on one backlog: profiles, first items, `av watch`) and
  `/scholion:doctor` (finds what is broken).
- **A short README**: what Scholion is for, told as a story, how to try it and what it measured.
  The full reference moved, unchanged, to `docs/reference.md`.
- `av repo backup NAME --set URL --branch BR` pushes main to a named branch of a backup remote.

### Route work to the agent that fits (Plan 8)

- **Agent profiles**: `av agent set NAME --best TAGS --can TAGS --cost N`. 1 is the cheapest.
- **A backlog**: `av post --repo R --title T --goal G [--needs TAGS] [--prefer NAME]`, then
  `av backlog`. Tags are set by people; the routing rules are deterministic, with no LLM call.
- **`av next`, or `start_task` with no title and goal**, gives an agent the next task that fits
  it. A task is reserved for its best agent for 10 minutes, then any agent that can do it may
  take it.
- `av handoff --needs TAGS` hands a task to an agent with those skills.
- **Order the backlog**: `av post --after ITEM[,ITEM]` (an item id or the slug of its title).
  `av next` does not offer the item, and `start_task` without a title does not take it, until
  every item it lists has merged; `av backlog` shows it as `waiting on X`.

### Close the review loop (Plan 9)

- `next` offers, in order: reviews of other agents' work, handoffs, abandoned tasks, then new
  work. An owner never reviews its own task. The reviewer merges what it approves.
- `av next --wait N` (MCP: `wait`) waits up to 50 s for work instead of returning empty.
- `av watch -- CMD` starts an agent CLI only when there is work for it. It runs on your machine;
  the daemon never starts processes.
- **A team's limit**: `av agent set NAME ... --max-open N`. At the limit, an agent is offered only
  reviews and its own unfinished tasks until it submits or hands one off.
- **Take only some kinds of work**: `av next --take review,merge` (or `--take work`). A
  review-only agent is never given backlog work, and a worker is never offered (or reserved)
  other agents' reviews.
- **A conflicted task goes back to its owner**: an approved task whose merge conflicted is
  offered to its owner with what to do (sync onto main, resolve, re-check, submit again), so
  `av watch` picks it up. If the owner does not act within the lease, any agent that fits may
  claim it.
- **Keep agents busy**: after its record check, the Claude Code `Stop` hook asks for this agent's
  next work. If something fits, the agent carries on instead of stopping. It waits for work up to
  `AV_HOOK_WAIT_SECS` (default 600, `0` turns it off), and continues a session at most
  `AV_HOOK_MAX_CONTINUES` times (default 20). Grok Build gets the same hook.
- MCP tools take a task's slug (or `repo/slug`) as well as its id. The CLI still takes the id.
- One MCP session can hold several tasks: Claude Code's subagents share their parent's
  session. Without `task_id`, a call uses the session's only open task, or asks which one.
  Before, a subagent's `start_task` sent the parent's later calls to the wrong task without an
  error.

### Packs, index and scale

- Work packs are 800 tokens of signatures by default. Lexical (BM25) and semantic hits are
  combined by rank fusion.
- Overlay index (Plan 7): a task reads main's index plus its own changes and no longer copies the
  index at start. A start that waits for its turn creates nothing until it gets one.
- Hooks show other tasks' records when an agent opens a file.

### For code built on the daemon

- Extension points in `scholion_daemon`: `router_with`, `with_extension`, `subscribe_events`,
  `MergeGuard` (asked before submit, approve and merge), `with_mcp_tools`, `UsageSink`
  (`POST /v1/agents/{name}/usage`), `put_profile` and `AppState::migrate_extension`. An extension
  runs its own migrations in its own Postgres schema, so its migration numbers never clash with
  the daemon's. `run_with` is the daemon's whole start-up as a library call, so an extension
  binary adds its routes and services without copying `main.rs`.
- `scholion-agentrun`: a library for running an agent CLI (argv, environment, deadline that kills
  the process group, parsed usage). The benchmark uses it.

### Upgrading from 0.0.1

- Stop the daemon and install the new binaries (rerun the installer, or `git pull` and
  `docker compose -f deploy/docker-compose.yml up -d --build`). The daemon adds its new tables
  (migrations 0021-0026) at start-up; existing tasks and records stay as they were.
- Restart your agent CLIs so they load the new tool parameters.

### Evidence

- A real run: two Claude Code agents on DeepSeek Flash took tasks from the backlog, reviewed each
  other's work and merged it. Reproduce it with `examples/routing-smoke/deepseek-demo.sh`; that
  run cost about $0.03. It shows that the loop works. It is not a measurement of token savings.
- Benchmarks and their limits: `bench/results/`. Plain notes files delivered recorded lessons as
  well as av did at 10 notes (E4). The task set with 300 notes (`bench/tasks-relay-scale`) is
  ready but has not been run, so av's advantage at scale is not measured yet.

## 0.0.1 (2026-10-28)

First release: the `scholion` CLI (alias `av`). The daemon ran from Docker
(`deploy/docker-compose.yml`).
