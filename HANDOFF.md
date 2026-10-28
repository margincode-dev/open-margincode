# Handoff — 2026-10-28

State of Scholion at the end of the cloud session, how to pick it up on your own machine, and
what to work on next. Delete this file once you've acted on it.

## 1. Where things stand

- **Default branch is `main`.** Plans 1–5 are built and merged (PRs #1–#11, including the
  discoverability work: README quickstart, site guides, release workflow, CI on `main`). No open
  PRs, no open issues.
- **Releases**: the release workflow builds on `v*` and `release/*` tags. The first release is
  built from the existing `release/0.0.1` tag (archives are named `scholion-v0.0.1-<target>`).
  Tag later releases `v0.1.0` and so on.
- **Evidence so far** (all in `bench/`):
  - E1 offline retrieval (42 public tasks, `bench/retrieval/`): the pack names gold files best
    (0.93–0.98) but **BM25 over symbols beats it on symbol recall and precision at every budget**
    (e.g. 4k: symbols_sig 0.62 vs 0.60, symbols_body 0.58 vs 0.56, gold_share 0.33 vs 0.25).
  - E3 Haiku on `tasks-tallyho` (25-file Python, 160 sessions): av saves **~12 % turns and ~14 %
    wall time at equal cost** (cost interval contains zero). Pack bodies were paid twice because
    Claude Code re-reads files before editing, hence the signatures-only default.
  - Not yet measured: `bench/tasks-scholion` (20k lines Rust, ~1 GB disk/session — did not fit
    the sandbox), E2 merge fuzzing, E4 knowledge relay (the core thesis), E5 git-worktree
    baseline, E6, E7. Plan 6 (`docs/superpowers/plans/2026-10-25-scholion-plan-6-evaluation.md`)
    is the roadmap for those.

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

## 3. Quick fixes (an hour or less each)

1. README "Status" still says the paid benchmark "has not been done"; the Haiku runs exist now.
   Replace with the one-line E1/E3 summary above.
2. Add a `curl | sh` installer (or use `cargo-dist`, which generates installer scripts, a
   Homebrew formula and the release workflow in one go).

## 4. What to improve next — ranked

Ranked by (expected impact on the core claims) / effort. Ideas borrowed from other open-source
projects are named.

### A. Retrieval: close the gap E1 already shows (cheap, measurable offline)

1. **Fuse BM25-over-symbols into the candidate set** with reciprocal rank fusion (RRF, as in
   Elastic/Vespa hybrid search) alongside structural, record and semantic candidates. E1 says it
   is the strongest single signal you don't use. Postgres FTS can do it (`ts_rank_cd` over a
   per-symbol tsvector of name + signature + docstring + split identifiers); or embed
   [Tantivy](https://github.com/quickwit-oss/tantivy) for real BM25. Gate: E1 symbols_sig at 4k
   ≥ 0.62 without losing files_named.
2. **PageRank over the reference graph** as a prior (the
   [Aider repo map](https://aider.chat/docs/repomap.html) idea, personalised to the seeds).
   repo_map wins symbols_sig at 8k (0.81); the pack's Map section could use the same ranking.
3. **Rerank the final candidates** with a cross-encoder — TEI already serves
   `BAAI/bge-reranker-*` on a `/rerank` endpoint, so it's one more container, no new code path
   in the hot loop beyond one HTTP call. Keep it optional so packs stay deterministic without it.
4. **Identifier splitting** (`parseHTTPResponse` → parse http response) for FTS and seeds; a
   common cheap win in code search (Sourcegraph, Zoekt).
5. Make **E1 a CI regression gate** on a fixed subset (Plan 6 already proposes this).

### B. Deliver context just in time instead of up front

The Haiku runs show an up-front pack is cache-written once and re-read every turn, so it costs
about what it saves. Move the history part to the moment it matters:

6. **Claude Code `PreToolUse`/`PostToolUse` hook on `Read`/`Edit`**: when the agent opens a file,
   inject only the decisions and dead ends anchored to that file/symbol (`additionalContext`).
   Small, targeted, and it never duplicates file bodies. This is the cleanest test of "why next
   to the code".
7. **Make records get written.** E4's failure mode "never written" is the likelier one. Add a
   `Stop`/`PreCompact` hook that, when a task has changed symbols with no covering record, asks the
   agent to record decisions/dead ends before the session ends (the submit gate already computes
   the uncovered symbols). Inspired by memory layers like [Mem0](https://github.com/mem0ai/mem0)
   and [Letta](https://github.com/letta-ai/letta), which extract memories at session boundaries.
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
23. E4 knowledge relay (20 pairs, ~$3 on a cheap model) is the single most important unrun
    experiment: it tests the project's thesis. Do it before polishing anything in C–E.
24. E5 with a `git worktree` baseline, so the coordination claim has a comparison.

## 5. Suggested order

1. Quick fixes 1–2.
2. A1 (BM25 symbols + RRF) with E1 as the gate — a day, fully offline.
3. B6 + B7 (just-in-time records, record-writing nudge), then E4 to measure them.
4. Run `tasks-scholion` (F22) to see whether the effect grows with repo size.
5. Then adoption (C9, C11, C12) or merge quality (D14) depending on what E4/E5 say.
