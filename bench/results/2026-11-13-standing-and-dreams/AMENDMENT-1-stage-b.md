# M2 stage B, amendment 1: raw against dreamed on an arm with room to move

**To be committed before the first stage B session.** `PREREG.md` is not edited. Where this
file and `PREREG.md` ("M2 stage B: raw memory against dreamed memory") disagree, this file holds
for every stage B session. Everything it does not name stays as registered: the question, one
frozen dream replayed into all 30 sessions of the dreamed arm, 30 sessions per arm, the primary
metric (the accepted rate, by the hidden pytest), the reported verdicts and dream cost, and the
Build rule (filled and committed before the first session).

## What had been seen when this was written

Written before any stage B session (no stage B `dream-freeze`, no `run --dreamed`), and **after**
these results, which shape it:

- Stage A's six passes and its Decision (`README.md`, "M2 stage A"): the bars were met;
  quote-split's wrong note ended `confirmed` in all six passes; the true-lesson bar was exercised
  once.
- "2026-11-15: the review worklist after stage A" (declined records set aside; the Build names
  the daemon commit).
- M1 Part 2 (`README.md`, "M1 Part 2, paid: results, 2026-11-16"): `av-p01` (Phases 0 and 1,
  build P1 `84bb472`, `--wrong-reverted`, the M0 tasks, Haiku 5.5) **30 of 30**; `av-p0` 30 of
  30.
- M0 (`../2026-11-14-m0/README.md`): `av-phase0` 30/30, `av-before` **11/30** (Wilson 0.219 to
  0.545).

The base arm is chosen *because* `av-before` scored 11/30: a choice made on seen data, said here.
No stage B outcome has been seen.

## Why

As registered, arm raw is `av` with Phases 0 and 1 on the M0 tasks with `--wrong-reverted`. M1
Part 2 ran exactly that on Haiku 5.5 and got 30 of 30 (Wilson 0.886 to 1.000). The dreamed arm
cannot be accepted more often than 30 of 30, so by the claim rule stage B as registered could
only show that dreaming hurts. To ask the registered question ("is an agent accepted more often
when the store has had one verified dream pass?") the base arm needs room to move. M0's
`av-before` (no correction line, no `--supersedes`, no standing) had it.

## What changes

1. **Both arms run with `AV_PLAIN_RECORDS=on`.** `av-before` ran on build A (`a67f130`), which
   has no dream routes, so it cannot be dreamed; no build had both. `open-margincode` #51
   adds `AV_PLAIN_RECORDS`: the daemon prints no line inviting `av record --supersedes` (packs,
   `records/at`), refuses to supersede another task's record (422), and leaves what became of a
   record's task out of its rank and its line (no `reverted`/`abandoned`/`open` label, no
   ×0.4/×0.6, no `standing` in record lists). Phase 2's verdicts still act: a disputed record is
   ×0.3 and marked, a duplicate or outdated one hidden. So raw is `av-before`'s memory on today's
   build, and dreamed is the same plus one frozen dream. `meta.json` says `plain_records: true`
   for both, from the daemon itself.
2. **No `--wrong-reverted`** (owner, 2026-11-18), as M0's `av-before` ran: each task's wrong note
   is a record of an open earlier task. With standing off the flag would only change how the note
   is seeded.
3. **The dreamer and the verifier on Sonnet 5.5** (owner, 2026-11-18): stage A's bars were met
   on Sonnet only. The sessions stay on Haiku 5.5, as M0's amendment 1 has them.
4. **`--limit 50`** (owner, 2026-11-18), as stage A ran, and **no `--until-empty`**: the
   registered stage B command is one review run and one verify run. An M0 task store holds the
   13 notes of `notes-silent.toml`, fewer than 50, so one review run is handed every record. If
   `dream-freeze` reports a record of any task not handed, the freeze is not used, and the owner
   is asked.
5. **A $5 ceiling** for all stage B work (owner, 2026-11-18): the freeze, the smokes and both
   arms. Before each command the money spent plus the command's budget must stay under it; at the
   ceiling the run stops and is reported as it stands. It is inside the key's $100 a month.

The arms run back to back on the same binaries and stack, on the M0 tasks (`bench/tasks-m0`:
quote-split, bus-resubscribe, tax-with-rate; 3 × 10), Claude Haiku 5.5 as in
`../2026-11-14-m0/AMENDMENT-1-haiku.md`, with its prices, session cap ($0.08) and smoke
conditions.

## Known differences from `av-before`

- The CLI still offers `av record --supersedes` (`av record --help`), where build A had none. An
  agent that tries it on another task's record is answered 422. Reported: how many sessions tried.
- The build is today's `main`, not `a67f130`: every pack change between them (PACKER_VERSION 7 to
  11: the goal always printed, the per-directory map, and others in `CHANGELOG.md`) is in both
  arms. So the raw arm is not a re-run of `av-before`, and 11/30 is never used as its value: raw is
  measured here.

## The free checks, before the first paid session

`first-packs` on the plain daemon for each M0 task, raw and with the frozen dream replayed:

- raw: no line containing `--supersedes`, no origin label, each task's wrong note shown.
- dreamed against raw: the packs differ, and only where the frozen verdicts say (a `disputed`
  mark, a record hidden).

If either fails, no paid session runs and the cause is reported.

## Claim rule (M0's)

M0 `PREREG.md`: "A difference is claimed only if the Wilson intervals are disjoint", by
`margincode_bench::stats::wilson_interval`. **Dreaming helps** if dreamed's lower end is above
raw's upper end; **hurts** if dreamed's upper end is below raw's lower end; otherwise **not
shown at this size**. Pooled over the three tasks, valid sessions only. No claim below n = 30 per
arm.

## Reported beside the primary

- The dream runs' own cost (both passes, Claude Code's `total_cost_usd`).
- The verdicts the passes ended with: which wrong notes ended `disputed` and effective, whether
  any true lesson ended `disputed` or `outdated`, and the undecided share.
- **A wrong note that ended `confirmed`.** If the frozen dream confirms a task's wrong note, the
  README reports, per task, the note's ending state and each arm's accepted count on that task.
  The primary is not changed and the task is not dropped; a per-task drop there is described as
  possible harm from a confirmed wrong note, not claimed (10 sessions a task is below the claim
  size).
- Sessions that tried `--supersedes` on another task's record.

## Cost estimate (an estimate)

- Sessions: M1 Part 2's 60 cost $0.3605 on Haiku 5.5; M0's `av-before` cost $0.1726 for 30.
  About **$0.35 to $0.45** for 60, plus two smokes (about $0.02).
- Dream: stage A's passes cost $0.567 to $0.874 on Sonnet 5.5 with about 61 records; an M0 store
  holds 13. Scaled by records, about $0.12 to $0.19 a task, **$0.4 to $0.6** for three; a review
  run has fixed overhead, so this is a guess.
- Total: **about $0.8 to $1.1**, under the $5 ceiling.

## Build (filled and committed before the first stage B session)

Written in two commits, both before the first stage B session: the first, before the freeze (the
only paid command before the sessions), names the build, the stack, the commands and the rules;
the second adds the frozen `dreamed.json`'s sha256 and the free checks' results. The scripts are
in `m2-stage-b/`.

- **Commit**: the binaries and the harness are `main` at `aa2547b` (the merge of #51, tree
  `d7e0ac96b2ecefb13bb334347b72a28ef7a11408`), from a `git archive` in `/m2b/src`; the daemon is
  the same build. `cargo build --release --locked -j 8 -p margincode-daemon -p margincode-cli -p
  margincode-bench` in `rust:1.97.1` (`sha256:b1b3c9c0…dec813f`) from an empty
  `CARGO_TARGET_DIR=/target/m2b`, copied to `/m2b/bin` (mode 755) and the harness to `/m2b/H/bin`
  (mode 700, root only):
  - `av` `7a4204d227f48cb3f65e20a30526d74abff46b46818920f10b29fb7e92f2a170`
  - `margincode` `93d4e50bb17fcfd2f5c5f66a0828e970191f2e3ed324de14b08b741396a17078`
  - `margincode-daemon` `276ab590215e6cf8ab01fd6d338c6a339053b0114f2cc8ca2cb1d0904e778024`
  - `margincode-bench` `1d1c8ce4854d3d089817af44023ba21ed29cc1cba7b1967e7027edbd4a138097`
- **The run tree**: `/m2b/tree-h`, a `git archive` of `aa2547b` made by M0's `haiku-tree.sh`, so
  it differs from the archive only in the `model` line of the 13 task files. `git diff 7dd5208
  aa2547b -- bench/tasks-m0 bench/tasks-relay bench/tallyho` is empty: the tasks, the pool and the
  task repository are M0's and M1 Part 2's. `margincode-bench check bench/tasks-m0`: 3 of 3 `ok`.
- **Container and services**: a new harness container `m2b-harness` (`rust:1.97.1`, root `/m2b`,
  Debian 13.6) with node 22.23.5 (nodejs.org tarball, checksum checked), Claude Code 2.1.318
  (`npm install -g @anthropic-ai/claude-code@2.1.318`, run with `DISABLE_AUTOUPDATER=1`), jq 1.7,
  git 2.47.3, Python 3.13.5 and the agent user `benchagent` (`useradd -m`), on the network
  `lp-bench` with M0's Postgres 16 (`lp-pg`), Qdrant 1.13.6 (`lp-qdrant`) and TEI cpu-1.7.2
  (`lp-tei`) serving `BAAI/bge-small-en-v1.5`.
- **Daemon**: `127.0.0.1:7490`, new database `m2b`, store `/m2b/store`, workspace root
  `/m2b/bench-work/ws`, `AV_QDRANT_URL=http://lp-qdrant:6334`, `AV_EMBED_URL=http://lp-tei:80`
  and **`AV_PLAIN_RECORDS=on`**. `GET /v1/version` answers `"plain_records":true`. One daemon,
  not restarted, serves the freeze, the free checks, the smoke and both arms.
- **Options**: `HOME=/m2b`; `--av-url http://127.0.0.1:7490 --work-root /m2b/bench-work
  --import-dir /m2b/bench-work/src --import-mount /m2b/bench-work/src --av-bin /m2b/bin/av`; for
  sessions also `--agent-user benchagent --av-ws-root /m2b/bench-work/ws`. Task repository
  `bench/tallyho/make-repo.sh` into `/m2b/bench-repos/tallyho` (`main` tree `c0ea5267…`, M0's);
  venv `/m2b/bench-work/venv`, Python 3.13.5 with pytest 9.2.0, pluggy 1.7.0, iniconfig 2.3.1,
  packaging 26.3, Pygments 2.21.0. `m2-stage-b/m2bcmd.sh` runs one command with these options;
  `m2-stage-b/m2bchain.sh` runs the two arms back to back.
- **Isolation**: `/m2b` 711; the trees, the source, the task repository, the store, the results,
  the checks, `/m2b/H` and `bench-work/src` 700; `bench-work` and `ws` 711; stage A's home is
  mounted read-only at a 700 path. The dream's directories (`bundle.json`, `verdicts.json`,
  `reviews.json`) and `dreamed.json` say which notes are wrong: they are closed (700) after the
  freeze, before any session, and `/tmp` is emptied before each command.
- **Dreamer and verifier**: stage A's script, `m2-stage-b/dreamer.sh` (sha256
  `e2452e7f04cce6ce1e548caf388fec010961b9a4d11d3d6f6497daca6539eea9`, the same file as stage A's
  dreamer and verifier), with `M2_MODEL=claude-sonnet-5-5` and **`M2_RUN_CAP=0.25`** ($0.25 per
  agent call where stage A had $1: three tasks, one review and one verify run each, at most one
  fix round each, so at most 12 calls and $3.00, which keeps the freeze's budget plus the
  sessions' under the $5 ceiling; an M0 store holds 13 records, a fifth of stage A's). Sonnet 5.5
  list prices: $2 input / $10 output per million tokens, cache read $0.10. The dream's cost is
  Claude Code's own `total_cost_usd` summed over every `claude-result-*.json` of the freeze.
- **The sessions**: Claude Haiku 5.5 as M0's amendment 1 has it (`claude-haiku-5-5`, first-party
  API, `ANTHROPIC_BASE_URL` unset, the key passed in by name at `docker exec` and given to Claude
  Code as `ANTHROPIC_API_KEY`, never written down; `ANTHROPIC_DEFAULT_HAIKU_MODEL`,
  `ANTHROPIC_SMALL_FAST_MODEL`, `CLAUDE_CODE_SUBAGENT_MODEL` = `claude-haiku-5-5`), prices
  `--price-per-mtok 0.10,0.125,0.01,0.50`, `--jit-hooks`, no MCP, no `--keep-work`, no noise, run
  seed 1, `--session-cap-usd 0.08`, no `--wrong-reverted`.

**Commands** (`m2bcmd.sh <subcommand> <args>`, which adds the options above; results to
`/m2b/results/<out>`, copied to `m2-stage-b/<out>` as the harness wrote them):

```bash
# the freeze (paid, Sonnet 5.5): one review run and one verify run per task
m2bcmd.sh dream-freeze bench/tasks-m0 --limit 50 \
  --dream-cmd 'sh /m2b/dreamer.sh' --verify-cmd 'sh /m2b/verifier.sh' \
  --out /m2b/results/m2-stage-b/dreamed.json
# the free checks
m2bcmd.sh first-packs bench/tasks-m0 --out /m2b/results/first-packs-raw
m2bcmd.sh first-packs bench/tasks-m0 --dreamed /m2b/results/m2-stage-b/dreamed.json \
  --out /m2b/results/first-packs-dreamed
# the smoke (not a result)
m2bcmd.sh run bench/tasks-relay/order-ids.toml --arms av --jit-hooks --reps 1 \
  --session-cap-usd 0.08 --budget-usd 0.10 --out /m2b/results/smoke
# raw, then dreamed, back to back (m2bchain.sh)
m2bcmd.sh run bench/tasks-m0 --arms av --jit-hooks --reps 10 --session-cap-usd 0.08 \
  --budget-usd 0.70 --out /m2b/results/m2-raw
m2bcmd.sh run bench/tasks-m0 --arms av --jit-hooks --reps 10 --session-cap-usd 0.08 \
  --budget-usd 0.70 --dreamed /m2b/results/m2-stage-b/dreamed.json --out /m2b/results/m2-dreamed
```

(`/m2b/verifier.sh` is a copy of `/m2b/dreamer.sh`.)

- **The freeze is used** only if, for each task, every record of its store was handed to the
  review run (the tally's `handed` equals the store's records) and `dream-freeze` exits 0.
  Otherwise no session runs and the owner is asked (change 4).
- **One smoke.** M0's smokes are one per build, and there is one build. `relay-order-ids` is not
  in the freeze, so `--dreamed` cannot run on it (the harness refuses a task the file does not
  name). The replay into a session's store is checked by the free check with `--dreamed`, and in
  each dreamed session by the harness, which stops a session whose replayed verdict does not end in
  its frozen state. The smoke passes under M0's conditions and its amendment's: every row
  `status=ok` and valid; the model answered and made at least one edit; verify ran; `hook-count`
  not empty; `meta.json` names `benchagent`, Claude Code 2.1.318, `models: ["claude-haiku-5-5"]`
  and `plain_records: true`; no single request with a prompt above 100K tokens; no session above
  $0.04. If the prompt or cost condition fails, the run stops and the owner is asked.
- **Money**: before each command, what stage B has spent plus that command's budget (freeze
  $3.00, smoke $0.10, each arm $0.70) stays under $5. An arm that stops part-way is reported as
  it stands and not rerun.
