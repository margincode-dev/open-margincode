# M0, amendment 2: the primary pair on a second model, Claude Sonnet 5.5

**Committed 2026-11-19, before the first Sonnet session.** `PREREG.md`, `AMENDMENT-1-haiku.md`
and `haiku-tree.sh` are not edited. The Haiku result stands as `README.md` reports it. This file
registers a second, smaller run: M0's primary pair, again, on another model. Where this file and
the two before it disagree, this file holds for the Sonnet run only. Everything it does not name
stays as `PREREG.md` and amendment 1 registered it.

After its push, this file is changed once: a commit adds the section "The smoke's numbers" at the
end, before the first arm session. That commit only adds lines.

## Why

The owner approved this run on 2026-11-19 in the words "Run M0": M0's primary pair on a second
model, Claude Sonnet 5.5, at about $5 to $7, with a hard ceiling of $7.00 for everything the run
spends. `HANDOFF.md`, "Next" 3: "One model has shown it."

## Question and claim rule

**Q. On M0's three tasks, whose wrong note nothing in the repository contradicts, is `av` with
Phase 0 (build B) accepted more often than `av` before Phase 0 (build A), on
`claude-sonnet-5-5`?**

- The primary is `PREREG.md`'s: accepted / valid sessions, `av-phase0` against `av-before`,
  pooled over the three tasks. A session is accepted when its row has `success` true and
  `status == "ok"`. Each arm gets its 95 % Wilson score interval, by
  `margincode_bench::stats::wilson_interval` (`sonnet-wilson.rs` compiles that function of the
  run tree; `sonnet_analyze.py` uses `analyze.py`'s copy of the same formula).
- **Phase 0 helps** if `av-phase0`'s lower end is above `av-before`'s upper end. **Phase 0
  hurts** if `av-phase0`'s upper end is below `av-before`'s lower end. Otherwise it is **not
  shown at this size**, both intervals are given, and the words "equal" and "no effect" are not
  used.
- **No claim** unless both arms have 30 valid sessions, 10 per task. An arm cut short by its
  budget is reported as partial, never extrapolated, and the primary is then not assessed.
- The per-task table is printed beside the pooled decision. No claim is made beyond these three
  tasks, this model and this repository (tallyho).

## A registered prediction

A stronger model may see through the wrong note unaided. Then `av-before` is high, as every arm
already was on `relay-tax-with-rate` on Haiku (10 of 10), and the outcome is "not shown at this
size". That is a valid result. If it comes, no arm, repetition, task or rerun is added after it is
seen, and nothing in this file is changed to reach a claim.

## Not pooled with Haiku

The Sonnet rows are never pooled with the Haiku rows, and no test sets one model's rows against
the other's. The README may set the Haiku numbers beside the Sonnet ones as a description only.

## What is run, and what that leaves out

Run: the two smokes (one session on each build, not a result), then `av-before` on build A and
`av-phase0` on build B, 3 tasks × 10 repetitions each, 60 sessions, both in this run and back to
back.

Not run: `cold`, `notes`, the guard, and therefore the gate. What that leaves uninterpretable:

- **No `cold` and no gate.** `PREREG.md` starts the `av` arms only after `cold` is accepted at
  least 8 of 10 times on each task. Here the gate is not run. It is not known whether Sonnet
  solves each task unaided. A task where both arms are low cannot be told from a task Sonnet
  does not solve. A high `av-before` cannot be told from a task where the note simply does not
  matter to this model.
- **No `notes` arm.** Secondary 1 (the gap to the notes file) is not assessed, and nothing here
  says how `av` compares with a plain `NOTES.md` on Sonnet.
- **No guard.** Nothing here shows that Phase 0's line costs nothing on true lessons on Sonnet.
  The Haiku guard (29 of 30) does not carry over to another model.
- Secondary measures 3 to 6 are reported for the two `av` arms. Secondary 7 (cost) is reported
  from the harness and Claude Code figures. Secondary 8 (`content.py`'s last edits) is not run.

## The model, pinned everywhere

`claude-sonnet-5-5` on Anthropic's first-party API. `ANTHROPIC_BASE_URL` is not set. The key
comes from the host's `CLAUDE_API_KEY`, is passed into the container by name at `docker exec`,
and is given to Claude Code as `ANTHROPIC_API_KEY`. It is never written down. The main model is
each task file's `model` line, which `sonnet-tree.sh` sets to `claude-sonnet-5-5`. `m0scmd.sh`
also sets `ANTHROPIC_MODEL`, `ANTHROPIC_DEFAULT_HAIKU_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`,
`ANTHROPIC_SMALL_FAST_MODEL` and `CLAUDE_CODE_SUBAGENT_MODEL` to `claude-sonnet-5-5`, so every
background call Claude Code makes uses the same model. With one model, the single price vector
prices every request. The smoke checks that every assistant message names `claude-sonnet-5-5`.
Claude Code's own defaults for thinking and effort apply. The harness sets neither, as before.

## Copied from amendment 1

Claude Code 2.1.318 (with `DISABLE_AUTOUPDATER=1`, as amendment 3 of the M1/M2 run did),
`--jit-hooks`, no MCP, no noise, run seed 1, `--agent-user benchagent`, no `--keep-work`,
`max_turns` from the task files (30). There is no `--wrong-reverted`: amendment 1 does not use
it, and neither build's harness has the flag. Each build gets a new daemon with a new, empty
database and store. Each command runs with its own build's harness binary, as `PREREG.md`'s
commands and `m0hcmd.sh` did: build A's `scholion-bench` runs `av-before`, and build B's runs
`av-phase0`. Free check 5's rule holds: only the command's own build's `bin` is on `PATH` and open,
and the other build's is mode 700 for the whole command.

## Prices

`--price-per-mtok 2.00,2.50,0.10,10.00`, in amendment 1's order: input, 5-minute cache write,
cache read, output. These are Claude Sonnet 5.5's first-party list prices as the `claude-api`
skill gives them (its model table and its "Migrating to Claude Sonnet 5.5" section, cached
2026-11-03): $2 input and $10 output per MTok, 5-minute cache writes $2.50, 1-hour cache writes
$4, cache reads $0.10. The skill lists a second rate card by prompt length only for Haiku 5.5
(split at 100K). None is listed for Sonnet 5.5. The smoke still reports the largest single
request's prompt. The harness prices every cache write at the 5-minute rate, so a 1-hour write
would be under-counted. The money rule below therefore counts each session at the larger of the
harness's figure and Claude Code's own `total_cost_usd`.

## Money, the session cap and the budgets (registered before the smoke)

- **Ceiling: $7.00** for everything this run spends: smokes, any smoke rerun, and both arms.
- **What counts as spent**: `sonnet_spend.py` sums, over every session so far, the larger of the
  row's `cost_usd` and Claude Code's `total_cost_usd` from that session's `events.jsonl`. It
  reads no accepted count.
- **The smoke**: `m0schain.sh smoke A 0.50 0.50`, then `m0schain.sh smoke B 0.50 0.50`. Each is
  one `av` session on `bench/tasks-relay/order-ids.toml` with `--session-cap-usd 0.50` and
  `--budget-usd 0.50`. The $0.50 cap is set high so that it does not bind.
- **From the smoke's two sessions**, with each session counted as above:
  - `m`: the cost of the most expensive smoke session.
  - `C`, the session cap for both arms: 3 × `m`, rounded up to the cent.
  - `a`: the mean cost of the two smoke sessions. `S`: what the smoke spent in all.
  - `r`: Claude Code's total over the two sessions divided by the harness's total, or 1 if that is
    smaller. The harness stops launching on its own figure, so each budget is divided by `r`.
- **Go or stop.** The projection is `P = S + 60 × a + C`: the smoke, sixty sessions at the smoke's
  mean, and one cap of room for the last launch. If `P` is above $7.00, the run stops after the
  smoke and the owner is told. Repetitions are never cut and no arm is dropped to fit.
- **Each arm's budget** is `(7.00 − spent so far) / r`, rounded down to the cent. `m0schain.sh
  arms C 7.00 r` works it out just before each of the two commands. `av-before` gets what the
  ceiling leaves after the smoke, and `av-phase0` gets what it leaves after `av-before`.
- **An arm that runs out** keeps its `skipped_budget` rows. It is reported as partial, and the
  primary is not assessed. Nothing is topped up, extrapolated or rerun.
- **A session stopped by its cap** keeps its row as the harness wrote it (valid or invalid), and
  it is not rerun.

## The smoke's pass conditions

`PREREG.md`'s, with amendment 1's (a): every row is `status=ok` and valid; the model answered and
made at least one edit; verify ran; `av`'s `hook-count` is not empty; and `meta.json` names
`benchagent`, Claude Code 2.1.318 and `models: ["claude-sonnet-5-5"]`. Every assistant message in
the event stream is from `claude-sonnet-5-5`. Amendment 1's (b) (no prompt above 100K) is not a
stop condition here, since no long-prompt rate is listed for Sonnet 5.5. The largest prompt is
reported. Amendment 1's (c) is replaced by the cap rule above. If a smoke fails, the cause is
fixed and it runs again in a new directory (`smoke-s-a-2`, …), and its cost counts toward the
ceiling. The arms start only after both smokes pass and the smoke's numbers are pushed.

## Order and what is read when

1. Free checks (below, all done before this commit).
2. This commit is pushed. Every paid session comes after the push.
3. The two smokes. Then a second commit adds the section "The smoke's numbers" (`m`, `C`, `a`,
   `S`, `r`, `P`, the largest prompt, both figures per session, and the go or stop) and is
   pushed before the first arm session.
4. `m0schain.sh arms C 7.00 r`: `av-before` (build A), then `av-phase0` (build B), back to back,
   in the background.
5. While the arms run, only `chain.log`, exit codes, row counts and costs (`sonnet_spend.py`) are
   read. The harness logs and `report.md` print accepted counts, and they are not read.
6. After both arms end, and not before: the store export (`sonnet-export.sh m0sa` into
   `m0s-av-before/`, `sonnet-export.sh m0sb` into `m0s-phase0/`), the leak scan (`sonnet_leak.py`,
   written to `checks-sonnet/`), then the analysis (`sonnet_analyze.py`, `sonnet-wilson.rs`).
   The primary is computed on all valid sessions. Exposed sessions are listed, and the tables are
   given again without them. An exposed session is a deviation.

## Results directories

`smoke-s-a/`, `smoke-s-b/`, `m0s-av-before/`, `m0s-phase0/` (rows, `meta.json` and `report.md` as
the harness wrote them, plus the store exports in the two arm directories), `first-packs-s-a/`,
`first-packs-s-b/` (the free replay) and `checks-sonnet/` (the free checks, the harness and chain
logs, the leak scan and the analysis). The harness's run ids are those names without hyphens
(`smokesa`, `smokesb`, `m0savbefore`, `m0sphase0`). Transcripts stay in the container, closed to
the agent user.

## Build (filled in before the first paid command)

- **Builds A and B were not found** in `m2b-harness`. A search of `/m2b`, `/m2home` and `/target`
  for `av` and `scholion*` binaries matched none of the recorded sha256s. Both were rebuilt in the
  harness with Rust 1.97.1, using `cargo build --release --locked -j 8 -p scholion-cli -p
  scholion-daemon -p scholion-bench`. Each was built from a `git archive` at the source path its
  first build used, so that embedded paths match. The tree id of each extracted archive was
  checked with `git write-tree`.
  - **Build A**, commit `a67f130`, tree `e197872d30634a90c5731f952a6f24fd1f8c47c9`, in `/m0/srcA`
    with `CARGO_TARGET_DIR=/target/m0a`. All four binaries are byte for byte M0's:
    `av` `29b70d0c07b747f5919f36d0a0373136cf6fca3de90c50a31e172f125e599f17`,
    `scholion` `4058eb1e676a96f22bdb2de522108b605c5367a2c739f85d43570128de4cd811`,
    `scholion-daemon` `659fb7fae363e0fbd98edddd051e39378cef8e610732ab690dd53f48aadadb7a`,
    `scholion-bench` `22bc607f4a08b8efb41b1f2fb95ce4bb05f58242fdc7befc76bdcf984d2783d0`.
  - **Build B**, commit `90ba62e`, tree `391e83cd3c74ff4da19423f96eb666195c5d0bb6`, in `/b2/tree`
    with `CARGO_TARGET_DIR=/target/b2`. All four binaries are byte for byte stage 2's and M0's:
    `av` `021ad167c462d568d5c0d0c4e0fe8842ceb80fddb1f543dc56d179f3166ac956`,
    `scholion` `f52b8b0182e11a94cd4f62b80b1e6ac5e30120d69264cf544757f7cf0f68f90b`,
    `scholion-daemon` `461f0e6632b91af3a20b20e9a023256c0152ee8205089207d49f9056b9bf2d47`,
    `scholion-bench` `ead477b874ec48f3c3c794cb61bdf4189ad148de2fcd98501c419c6ebf32a891`.
  - Both were copied to `/m2b/m0s/A/bin` and `/m2b/m0s/B/bin`, with the same sha256 there.
    `/m0`, `/b2`, `/target/m0a` and `/target/b2` are 700.
- **Run tree**: `/m2b/m0s/tree`, a `git archive` of `main` at `7dd5208`, tree
  `012def994ea9a8d2f0a6e7b553fd138e7434be62` (M0's, checked with `git write-tree`).
  `/m2b/m0s/tree-s` was made from it by `sonnet-tree.sh`: exactly the 13 `model` lines of
  `bench/tasks-m0` and `bench/tasks-relay` differ. Both are 700. The run's scripts are copies of
  this commit's files.
- **Container**: `m2b-harness` (`rust:1.97.1`, `sha256:b1b3c9c0…dec813f`, the same image as
  M0's `lp-harness`; node 22.23.5, git 2.47.3, Python 3.13.5), started with `docker start`, with
  volumes `m2b-root` → `/m2b`, `m2-home` → `/m2home` (read-only) and `m2b-target` → `/target`. The
  run's root is `/m2b/m0s` (711), and `HOME=/m2b/m0s` for every command. Claude Code 2.1.318 is
  `/usr/local/bin/claude`.
- **Services**, on network `lp-bench` and started again after a stop: Postgres 16 (`lp-pg`,
  `sha256:ca0bd484…dbecb641d`), Qdrant 1.13.6 (`lp-qdrant`) and TEI cpu-1.7.2 (`lp-tei`) serving
  `BAAI/bge-small-en-v1.5`. The Qdrant collections (`code_chunks`, `conventions`, `records`) are
  shared, as in every earlier run on this stack.
- **Daemons** (`m0sdaemon.sh`, started once, not restarted during the run):
  - build A on `127.0.0.1:7461`, new database `m0sa`, store `/m2b/m0s/A/store`, workspace root
    `/m2b/m0s/bench-work/ws-a`;
  - build B on `127.0.0.1:7462`, new database `m0sb`, store `/m2b/m0s/B/store`, workspace root
    `/m2b/m0s/bench-work/ws-b`.

  Both use `AV_QDRANT_URL=http://lp-qdrant:6334` and `AV_EMBED_URL=http://lp-tei:80`. Neither
  database name was in use on `lp-pg` (it held `bench`, `bench2`, `bench3`, `m0a`, `m0b`,
  `m0ha`, `m0hb`, `m1p0`, `m1p1`, `m2b`, `m2h`, `m2p`, `m2r`). No other bench daemon runs.
- **Options** (`m0scmd.sh`): `--work-root /m2b/m0s/bench-work --import-dir
  /m2b/m0s/bench-work/src --import-mount /m2b/m0s/bench-work/src --av-url
  http://127.0.0.1:<port> --av-bin /m2b/m0s/<build>/bin/av`, and for sessions also
  `--agent-user benchagent --av-ws-root` with the command's workspace root.
- **Task repository**: `bench/tallyho/make-repo.sh` into `/m2b/m0s/bench-repos/tallyho`. Its
  `main` is `a032981`, with tree `c0ea5267c8a8000fd499a9301e7d8d37ac7cc7f9`, M0's tree (the
  commit id differs by its date only).
- **Venv**: `/m2b/m0s/bench-work/venv`, Python 3.13.5, with M0's package versions: pytest 9.2.0,
  pluggy 1.7.0, iniconfig 2.3.1, packaging 26.3, Pygments 2.21.0.
- **Isolation**: `/m2b/m0s` 711; `tree`, `tree-s`, `bench-repos`, `results`, `checks`, both
  stores and `bench-work/src` 700; `bench-work`, `runs`, `ws-a` and `ws-b` 711; each command's
  run, results and workspace directories are closed (700) when it ends, and `/tmp` is emptied
  before each command. The agent user is `benchagent` (uid 1000, home `/nonexistent`). Each
  session gets its own home and Claude Code config from the harness. Other runs under `/m2b` keep
  their own modes: their trees, results, stores and checks are 700. `/m2home` is 700. `/m2b/bin`
  (755) holds a newer `av`. It is on no command's `PATH`, and since `/m2b` is 711 it can be
  reached only by its full path. The leak scan counts it as `source tree`.
- **Scripts** of this commit: `sonnet-tree.sh`, `m0sdaemon.sh`, `m0scmd.sh`, `m0schain.sh`,
  `sonnet_spend.py`, `sonnet-export.sh`, `sonnet_leak.py`, `sonnet_analyze.py` and
  `sonnet-wilson.rs`. `sonnet_analyze.py` was tested on the Haiku rows (`--dirs
  m0-av-before=av-before,m0-wrong=av-phase0`, `checks-sonnet/analyze-test-haiku.txt`). It
  reproduces `README.md`'s `av-before` 11/30 (0.219 to 0.545), `av-phase0` 30/30 (0.886 to
  1.000), "Phase 0 helps", the per-task table and the store counts (15 own, 1 true). The
  transcript columns are 0 there because the Haiku transcripts are not in this container.

Free checks, all done before this commit (outputs in `checks-sonnet/`):

1. **Task set** (`check-set.txt`). `reference/check.py`: "all 13 pairs ok (3 of them silent)".
   `tasks-relay-silent/check_set.py`: "silent set ok". `tasks-m0/check_set.py` on the run tree
   `7dd5208`: "M0 set ok". On the Sonnet tree, the same script passes every check except its three
   byte-for-byte comparisons of the M0 specs with their sources. Those differ by exactly the
   `model` line that `sonnet-tree.sh` changes, as the file shows.
2. **Specs** (`spec-check.txt`): `scholion-bench check` of `bench/tasks-m0` and `bench/tasks-relay`
   on the Sonnet tree, with build A's harness and with build B's: 26 of 26 `ok`.
3. **Replay** (`first-packs`, one per build, on the run's own daemons; `first-packs-s-a/`,
   `first-packs-s-b/`, `replay.txt`): passes, as in M0. On both builds, each task's wrong note is
   in the session-start hook's pack, summary whole, with its `rec:` handle. On build B, the line
   stands directly under "Decisions & dead ends", and on build A it is absent. As in M0,
   `relay-tax-with-rate`'s note is missing from the pack at task start on both builds. Against
   M0's `first-packs-a/` and `-b/`, with record ids masked, the packs differ only in the workspace
   path, symbol ids, token counts within 4 of 800, and one related symbol in `relay-quote-split`'s
   hook pack on build B (`Order.add` in place of `NotFound`). `first-packs` takes no
   `--agent-user`, so it ran without it, as in M0.
4. **One `av` per command** (`one-av.txt`), checked as `benchagent` with each command's `PATH` and
   the other build's `bin` at 700: `which -a av` finds only the command's own build. Build A's
   `av record --help` does not name `--supersedes`, and build B's does. The other build's `av` is
   refused. The trees, the task repository, the results and the build sources are closed. The
   venv's pytest and Claude Code 2.1.318 run.
5. **Dry runs** (`dry-run.txt`): `scholion-bench run` without `--confirm`, with each build's
   harness, for the arm and for the smoke. Each prints its plan (30 sessions, or 1; `model:
   claude-sonnet-5-5`; agent isolation `benchagent`) and launches nothing. The cap and budget in
   these dry runs are placeholders.
