# Handoff — 2026-10-29

State of Scholion after the handoff session, how to pick it up on your own machine, and what to
work on next. Delete this file once you've acted on it.

## 1. Where things stand

- **Default branch is `main`.** Plans 1–5 are built and merged. The handoff session's single PR added:
  - `scripts/install.sh` (`curl | sh`), and the README status with the E1/E3 numbers;
  - **A1**: BM25 (not `ts_rank`) and reciprocal rank fusion for code hits, the leftover budget
    spent on earlier handles, test code ranked down. E1 at 4k: `symbols_sig` 0.73 against
    `bm25_symbols`' 0.64 (before: 0.61), 8k 0.82 against 0.76, 2k level. `files_named` 0.96 vs
    0.98. Details and the variants that did not help: `bench/retrieval/README.md`;
  - **B6** `PreToolUse` hook (`av hook pre-read`, `GET /v1/tasks/{id}/records/at`) and **B7**
    `Stop` hook (`av hook stop`, `GET /v1/tasks/{id}/unrecorded`), installed by
    `integrations/claude-code/install.sh`; the `/ui` task drawer lists the unrecorded changes;
  - **E4** knowledge relay (`bench/tasks-relay`, arms `baseline`/`notes`/`av`, run on DeepSeek;
    result in 1a below) and the harness changes it needed.
- **Releases**: the release workflow builds on `v*` and `release/*` tags (first release from
  `release/0.0.1`). Tag later releases `v0.1.0` and so on.
- **Evidence** (all in `bench/`): E1 above; E3 Haiku on `tasks-tallyho` (160 sessions): av saves
  ~12 % turns and ~14 % wall time at equal cost; E4 below. Not yet measured: `bench/tasks-scholion`
  (20k lines Rust, ~1 GB disk/session), E2 merge fuzzing, E5 worktree baseline, E6, E7. Plan 6
  (`docs/superpowers/plans/2026-10-25-scholion-plan-6-evaluation.md`) is the roadmap.

### 1a. E4 knowledge relay

`bench/tasks-relay`: agent A's ten lessons (five dead ends, five team decisions) reach agent B, who
starts with no memory of A. Arms: `cold` (no lessons, the control), `baseline` (lessons as commit
messages), `notes` (plus `NOTES.md`), `av` (plus anchored records, delivered by the pack and the
`PreToolUse` hook). DeepSeek flash, headless Claude Code; costs at assumed peak prices ($0.30 in /
$0.006 cache hit / $1.20 out per Mtok).

1. **First run** (2026-10-28, `bench/results/2026-10-28-relay/`): every arm 30/30, av +22 % cost per
   accepted task. Not a test of memory: `baseline` agents ran `git log` in 18 of 30 sessions and
   read the lessons there, four goals gave the trap away, and no agent could run pytest (the venv
   was not on their PATH). Fixed with the `cold` arm, plain goals and `path_prepend`.
2. **Lean av** (2026-10-29, `bench/results/2026-10-29-relay-lean/`, 90 sessions, $1.17 at those
   prices, the balance fell $3.40 → $2.83). av on hooks + CLI (no MCP), no manual checkpoints,
   records and submit in one call, 800-token default context
   (`docs/superpowers/specs/2026-10-29-lean-av-design.md`).

| arm | accepted | cost / accepted | wall / accepted |
|---|---|---|---|
| cold | 18/30 | $0.022 | 63 s |
| notes | 30/30 | $0.013 | 36 s |
| av | 30/30 | $0.013 | 38 s |

- Recorded knowledge pays: with the lessons, cost per accepted task falls about 42 % and every task
  is solved. Without them `customer-region`, `product-slug` and `refund-event` fail every time (team
  decisions the code does not show).
- av now costs the same as a `NOTES.md` (-1.8 %, within noise), where the first run's av cost 22 %
  more. The token analysis behind the fix: 85 % of that overhead was close-out calls
  (`checkpoint`, `record`, `submit`, `status`, MCP schema loading), not the pack.
- av does not beat `notes` here. With ten notes a plain file is cheap to read whole. What av is
  built for, hundreds of records where the agent can read only the relevant ones, is untested.
- The trap tasks (coupon, money, quote-split, store-count) pass even `cold`, so only about half the
  set tests memory.

Next for E4: a scale test (hundreds of notes, most irrelevant) is the one that can show av beating
`NOTES.md`; and tasks where the lesson matters, not traps the model avoids alone.

### Not verified by hand

The hooks were tested through the CLI and daemon test suites and `bench/tasks-relay` (headless
Claude Code on DeepSeek), not in an interactive Claude Code session. `integrations/claude-code/install.sh`
was not run end to end. `scripts/install.sh` was run against `release/0.0.1`.

## 2. Kick-start on your machine

Prereqs: `rustup` (picks up 1.97.1 from `rust-toolchain.toml`; jj-lib 0.46 needs ≥ 1.97.1),
Docker + Compose, `git`, `jq` (Claude Code hook installer). Optional: `ollama` with
`qwen3-embedding` if you run the daemon on the host and want semantic search.

```bash
git clone git@github.com:vinhqbui/scholion.git && cd scholion

# 1. Prove the toolchain (first run downloads an embedded Postgres; no Docker needed)
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

# 2. Full stack (Postgres 16, Qdrant, TEI, daemon on 127.0.0.1:7420)
export UID GID=$(id -g) AV_IMPORT_ROOT=~/projects AV_WS_ROOT=~/ws
docker compose -f deploy/docker-compose.yml up -d --build
cargo build --release -p scholion-cli        # target/release/{scholion,av}; put on PATH
scripts/smoke.sh                             # end-to-end check against the stack

# 3. Hook up an agent
claude mcp add --transport http av http://localhost:7420/mcp
integrations/claude-code/install.sh          # SessionStart pack + PostToolUse checkpoint hooks
av repo add myrepo /import/myrepo            # path *inside* the container
open http://localhost:7420/ui                # dashboard
```

Run the daemon on the host instead (faster edit loop), against the compose Postgres/Qdrant:

```bash
docker compose -f deploy/docker-compose.yml up -d postgres qdrant
docker compose -f deploy/docker-compose.yml exec postgres createdb -U av av_host   # once
docker compose -f deploy/docker-compose.yml stop daemon   # frees :7420 if the full stack ran
AV_DATABASE_URL=postgres://av:av-dev-only@127.0.0.1:5432/av_host \
AV_QDRANT_URL=http://127.0.0.1:6334 AV_STORE_ROOT=~/.scholion/store AV_WS_ROOT=~/ws \
AV_LISTEN=127.0.0.1:7420 cargo run -p scholion-daemon     # AV_LISTEN defaults to 0.0.0.0
```

(The host daemon's jj store is not the container's `av-store` volume, so repos added through the
container won't be found from the host. Hence the separate `av_host` database. Run only one daemon
per database — see E18.)

Gotchas:
- Qdrant tests pass in ~0 s unless `AV_TEST_QDRANT_URL=http://127.0.0.1:6334` is set.
- TEI downloads its model on first start; packs say `degraded: semantic` until it's up.
- Docker Desktop on Windows without WSL integration: see the README section of that name.
- New migration? Next number is **`0021`**, and add its hash to `tests/migrations_frozen.rs`.

## 4. What to improve next — ranked

Ranked by (expected impact on the core claims) / effort. Ideas borrowed from other open-source
projects are named.

### A. Retrieval (cheap, measurable offline; A1, A4 and A5 are done)

2. **PageRank over the reference graph** as a prior (the
   [Aider repo map](https://aider.chat/docs/repomap.html) idea, personalised to the seeds).
   repo_map wins symbols_sig at 8k (0.81); the pack's Map section could use the same ranking.
3. **Rerank the final candidates** with a cross-encoder — TEI already serves
   `BAAI/bge-reranker-*` on a `/rerank` endpoint, so it's one more container, no new code path
   in the hot loop beyond one HTTP call. Keep it optional so packs stay deterministic without it.
5. ~~Make **E1 a CI regression gate**~~ done: `crates/scholion-bench/tests/retrieval_gate.rs` on the
   fixture in `bench/retrieval/ci/` (lexical, structural and budget parts; no semantic layer).

### B. Deliver context just in time (B6 and B7 are done)

The Haiku runs show an up-front pack is cache-written once and re-read every turn, so it costs
about what it saves. Move the history part to the moment it matters:

8. **Stable pack prefix for prompt caching**: order sections so the part that changes least
   (conventions, map) comes first and is byte-identical across packs in a session.

### C. Reduce lock-in, increase adoption

9. **Export records into git on merge**: write each merged task's intent/decisions/dead ends as
   `git notes` (or commit trailers) on the merge commit, and `av import-notes` to read them back.
   Teams can adopt Scholion without betting their history on a Postgres DB, and `git log` users
   see the "why". (Gerrit's `Change-Id` trailers and [git-bug](https://github.com/git-bug/git-bug)
   are the precedents for storing structured data in git.)
10. **Generate `AGENTS.md`/`CLAUDE.md` sections from conventions** (`av conventions export`), so
    agents without the MCP server still get the repo rules.
11. **A no-Docker local mode**: the daemon already has an embedded Postgres in `scholion-testutil`
    (`postgresql_embedded`); a `scholion-daemon --local` that starts it and runs without Qdrant
    (FTS-only) would make "try it" a single binary.
12. **Package the Claude Code integration as a plugin** (hooks + MCP server config in one
    installable unit) instead of `install.sh` editing `settings.json`. Add the same hooks for
    Codex CLI and Gemini CLI, which now have hook/MCP config of their own.
13. **GitHub bridge**: mirror a task in review to a draft PR with the review bundle as its body,
    and map PR approvals back to `review` records, so humans can review where they already work.

### D. Merge and index quality

14. **Syntax-aware merges with [Mergiraf](https://mergiraf.org)** (Rust, tree-sitter-based, works
    as a git/jj merge driver). Many "conflicts" between agents editing the same file are
    structurally independent (two new functions, two imports); resolving those automatically
    attacks C3 directly. jj supports external merge tools, so this can be tried without touching
    jj-lib internals.
15. **More languages**: Go, Java, C# and JS (separate from TS) via their tree-sitter grammars, or
    fall back to [universal-ctags](https://github.com/universal-ctags/ctags) for symbols-only
    coverage of everything else. For precise cross-file references,
    [SCIP](https://github.com/sourcegraph/scip) indexers or an LSP bridge (as
    [Serena](https://github.com/oraios/serena) does over MCP) beat name matching.
16. **Structural diffs in review bundles** with
    [difftastic](https://github.com/Wilfred/difftastic)-style hunks, so reviewers (human or agent)
    see moved code as moved.
17. **E2 merge fuzzing with `proptest`** (Plan 6): lost-work and false-clean merges are release
    blockers, and nothing currently looks for them.

### E. Operations and hardening

18. **Multi-instance safety**: `AppState::lock_task` is an in-process mutex, so two daemons on one
    database would race. Replace with Postgres advisory locks (`pg_advisory_xact_lock(task_id)`)
    — the outbox worker already uses one — or document "exactly one daemon per DB" loudly.
19. **Per-agent tokens** instead of one shared `AV_TOKEN`, so agent names stop being
    self-asserted (README "Security" already calls this out).
20. **Observability**: `tracing` → OpenTelemetry export plus a `/metrics` endpoint (pack latency,
    outbox lag, merge outcomes). The dashboard could read the same numbers.
21. **CI speed and supply chain**: `cargo-nextest` (test binaries are ~450 MB each; nextest
    parallelises across them) and `cargo-deny` for licences/advisories.

### F. Evaluation (Plan 6 phases still open)

22. Run `bench/tasks-scholion` on your machine (it needs the ~1 GB/session disk the sandbox
    lacked) — it is the repo size where navigation savings should show up.
24. E5 with a `git worktree` baseline, so the coordination claim has a comparison.

## 5. Suggested order

The goal is a product people adopt, judged by tokens per solved task. E4 shows recorded knowledge
pays and lean av delivers it at `NOTES.md` cost; it does not yet show av is worth installing over a
`NOTES.md`.

1. E4 at scale: hundreds of notes, most irrelevant, so reading them all costs more than av's
   targeted delivery. This is the test that decides whether av's infrastructure earns its place.
2. Adoption (option B in the 2026-10-29 discussion): records as files in git and a no-daemon solo
   mode (C9, C11, C12), so the install cost matches the measured gain.
3. Run `tasks-scholion` (F22) when there is budget, to see whether the effect grows with repo size.
