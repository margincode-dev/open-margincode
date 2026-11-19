# M2 stage B, amendment 2: raw against dreamed again, on the revised `DREAM.md`, at 90 a side

**To be committed before its first paid command.** `PREREG.md` and `AMENDMENT-1-stage-b.md` are
not edited. This file registers a second stage B run. Where it does not name a change, the run is
stage B as `AMENDMENT-1-stage-b.md` has it: the question, one frozen dream replayed into every
session of the dreamed arm, the two arms, the primary metric (the accepted rate, by the hidden
pytest), the models, the daemon setting, the free checks, the smoke and the claim rule. Its
sessions do not pool with stage B's 30 a side, and stage B's result stands as it was reported.

## What had been seen when this was written

Written before any session or freeze of this run, and **after** these results, which shape it:

- **Stage B** (`README.md`, "M2 stage B: raw against dreamed memory, results, 2026-11-18"): raw
  **14 of 30** (0.467, Wilson 0.302 to 0.639), dreamed **21 of 30** (0.700, Wilson 0.521 to
  0.833); the intervals overlap, so **not shown at this size**. Per task: quote-split raw 0/10,
  dreamed 1/10; bus-resubscribe raw 4/10, dreamed 10/10; **tax-with-rate 10/10 in both arms**.
  Its freeze: quote-split's wrong note ended `confirmed` in all three stores; 17 of 24
  wrong-note verdicts `disputed`; one true lesson (`customer-region`) `disputed`; $0.5677.
- **`DREAM.md` was revised after those numbers were seen, and fitted on these same three tasks**
  (#63, merged as `96286c3`). Three in-sample pilots reran stage B's `dream-freeze` on the same M0
  stores with an `av` built from the revised text (Sonnet 5.5, `--limit 50`; only `av` was
  rebuilt, the daemon and the harness were stage B's): pilot 1 **$0.6519**, pilot 2 **$0.6504**,
  pilot 3 **$0.6030** ($1.905 in all). Each text was changed after the pilot before it was seen.
  **Pilot 3** (the text that was merged) met all four conditions declared before it ran:
  quote-split's wrong note `confirmed` in 0 of 3 stores; 21 of 24 wrong-note verdicts `disputed`
  (7 of 8 in each store); bus-resubscribe's wrong note `disputed` in 3 of 3; no true lesson
  `disputed` or `outdated`, but **no true lesson got any verdict** (judged 0 of 5 in each store),
  so that last condition held because nothing was judged. 6 of 13 handed records were left
  undecided in each store.
- **HANDOFF "Next" 2, run just before this was written** (`../2026-11-19-dream-promote-also-read/`,
  $0.2415): two review runs on this run's build (`21f30dc`, the revised text plus the
  `--promote` section in one and the `--also-read` section in the other), each on a freshly
  seeded relay-bus-resubscribe store of 13 records, Sonnet 5.5, `dreamer.sh` as here. Both
  disputed the same 5 of 8 wrong notes (coupon-remove, store-count, money-plain, tax-with-rate,
  line-taxes), left out bus-resubscribe's and quote-split's wrong notes (bus-resubscribe's read
  as "a rule"), and judged no true lesson. Pilot 3 had bus-resubscribe's note `disputed` in 3 of
  3 stores; this is the note behind stage B's only per-task gain. One run of each, each with an
  appended section the freeze does not have, and no verify run: not a prediction of the freeze.
  Nothing in this design was changed after it.
- **tax-with-rate was 10 of 10 in both arms** of stage B: on that task the dreamed arm has no
  room to move, and a third of each arm's sessions are on it.
- **More repetitions over the same three tasks are not more tasks.** M0's pre-registration,
  which stage B copies (`../2026-11-14-m0/PREREG.md`): "Thirty sessions over three tasks are
  not thirty tasks." Ninety over three are not ninety either: a larger n narrows the intervals
  for these three tasks, with this one frozen dream, and says nothing beyond them.

So the dreamed arm here uses an instruction text tuned on the stores it is measured on. A
`helps` result would show that this frozen dream, from text fitted to these three tasks, helps on
these three tasks; it would not show that the revised text helps elsewhere. No session of this
run has been seen.

## Why

Stage B leaned towards dreaming (21 against 14) and did not show it. The open question in
`HANDOFF.md` ("Next" 1) is whether the lean survives at a larger size, and no result under
`bench/results/` uses the revised `DREAM.md` yet. Thirty a side cannot separate two rates that
far apart; ninety a side can, if the rates hold. The freeze is new because the text the dreamer
is handed changed; the old `dreamed.json` was made from the old text and is not reused.

## What changes

1. **The build.** `main` at `21f30dc` for `av`, the daemon and the harness, so the dreamer is
   handed the revised `DREAM.md` (embedded in `av`). Every other change between `aa2547b` and
   `21f30dc` is in both arms; see "Known differences".
2. **A new freeze** with that build: one review run and one verify run per task, as stage B's.
3. **`--reps 30`**: 3 tasks × 30 = **90 sessions per arm**, 180 in all.
4. **`--budget-usd 2.10` per arm** (stage B's 0.70, × 3), under the ceiling below.
5. **New directories** for everything: a new database, daemon port, store, work and workspace
   roots, results and checks, under `/m2b/r2/` (stage B's stay as they are).
6. **A $3.00 ceiling** for all of this run's paid work: the freeze, the smoke and both arms.

Unchanged from `AMENDMENT-1-stage-b.md`: the M0 tasks (`bench/tasks-m0`), the arms (`av` raw
against `av --dreamed`), `AV_PLAIN_RECORDS=on`, no `--wrong-reverted`, the dreamer and verifier on
Sonnet 5.5 with `M2_RUN_CAP=0.25`, `--limit 50` and no `--until-empty`, the sessions on Claude
Haiku 5.5 with M0 amendment 1's prices, `--session-cap-usd 0.08`, `--jit-hooks`, run seed 1, the
free checks and their pass conditions, the smoke and its conditions, and the claim rule.

## Money

The ceiling is $3.00 of money spent: the freeze's Claude Code `total_cost_usd` summed over every
`claude-result-*.json`, and the harness's session costs (`rows.jsonl`). After each command the
spend is summed and logged. AMENDMENT-1's rule ("what has been spent plus the command's budget
stays under the ceiling") cannot be applied as written with these numbers: the freeze's own cap
is 12 calls × $0.25 = $3.00, and after the raw arm the spend plus the dreamed arm's $2.10 would
pass $3.00 even though about $0.5 is expected. So:

- Before the freeze: nothing spent. The freeze runs with its fixed per-call cap.
- Before the smoke and before each arm: if what has been spent is $3.00 or more, nothing more
  runs. Otherwise the command runs with its budget as written (smoke $0.10, raw $2.10), except
  that **an arm's `--budget-usd` is the smaller of $2.10 and $3.00 less what has been spent**,
  so the harness (which stops launching once spent + one session's $0.08 cap would pass its
  budget) cannot carry the run past $3.00.
- An arm that stops part-way for money is reported as it stands and not rerun; the claim rule
  below then reads what is valid.

At the expected costs neither arm's budget binds (each arm about $0.5 of $2.10).

## Known differences from stage B

- **Code between `aa2547b` and `21f30dc`** (merges #52 to #68): the pack source is unchanged
  (`crates/margincode-pack/src` has no diff; `PACKER_VERSION` is 11 in both). The daemon changed
  in `dream.rs` (`3283966`, `74cd619`, `a93d3bd`: the `promote` verdict, migration 0032) and in
  `flow.rs` and `wiring.rs` (`d94f70c`: merge commits name owner, reviewer and merger;
  `AV_COMMIT_AUTHOR`); this is the first freeze with the changed daemon `dream.rs`, since the
  pilots rebuilt only `av`. The CLI changed in `dream.rs` (`--budget-usd`, `--also-read`,
  `--every`, `--promote`), `main.rs`, `notes.rs`, `local.rs` and a new `diff_check.rs`. The
  harness's source (`crates/margincode-bench/src`) is unchanged. The sessions' hook (`hook.rs`) is
  unchanged. These are in both arms; the arms differ only by `--dreamed`.
- **The dreamer is handed the revised `DREAM.md`** without `--promote` or `--also-read`: the
  freeze's `av dream` passes neither, so the text is `integrations/dream/DREAM.md` at `21f30dc`
  as it is, and its `instructions_sha` differs from stage B's.
- `git diff aa2547b 21f30dc -- bench/tasks-m0 bench/tasks-relay bench/tallyho` is empty: the
  tasks, the pool and the task repository are stage B's.
- **The two arms are not one unbroken chain.** Before the smoke and before each arm the host's
  build lock is checked and, if held, waited out, so the arms run on a quiet machine; the time
  between the arms is reported.

## The free checks, before the first paid session

As `AMENDMENT-1-stage-b.md` has them, on this run's daemon: `first-packs` for each M0 task, raw
and with the new frozen dream replayed. Raw: no line containing `--supersedes`, no origin label,
each task's wrong note shown. Dreamed against raw: the packs differ, and only where the frozen
verdicts say (a `disputed` mark, a record hidden). If either fails, no paid session runs and the
cause is reported.

**One freeze.** It is made once and used as it comes out, whatever its verdicts; there is no
second freeze. **The freeze is used** only if, for each task, every record of its store was handed
to the review run and `dream-freeze` exits 0. Otherwise no session runs and the cause is reported.

## Claim rule (M0's, as stage B)

"A difference is claimed only if the Wilson intervals are disjoint", by
`margincode_bench::stats::wilson_interval`. **Dreaming helps** if dreamed's lower end is above
raw's upper end; **hurts** if dreamed's upper end is below raw's lower end; otherwise **not
shown at this size**. Pooled over the three tasks, valid sessions only.

- The registered size here is 90 valid sessions per arm, 30 per task. If an arm ends with fewer
  (invalid sessions, or a stop for money), the rule is read on the valid sessions it has, and no
  claim is made unless each arm has at least 30 valid sessions with at least 10 on each task
  (stage B's size). Sessions are not rerun.
- For scale at 90: 42 of 90 (0.467) has the Wilson interval 0.367 to 0.569, and 63 of 90 (0.700)
  has 0.599 to 0.785. Stage B's two rates at this size would be disjoint; that is arithmetic, not
  a prediction.
- Whatever the outcome, the README says that the text was fitted on these three tasks.

## Reported beside the primary

As `AMENDMENT-1-stage-b.md`: the freeze's cost; the verdicts it ended with (each task's own
wrong note, any true lesson `disputed` or `outdated`, how many true lessons were judged at all,
the undecided share); a wrong note that ended `confirmed`, with each arm's count on its task;
sessions that tried `--supersedes` on another task's record. Also, per task, each arm's accepted
count (30 sessions a task, still described and not claimed), and stage B's numbers beside this
run's, not pooled.

## Cost estimate (an estimate)

- Freeze: pilots 1 to 3 on the revised text cost $0.652, $0.650 and $0.603: about **$0.65**.
- Sessions: stage B's arms cost $0.1615 and $0.1500 for 30 each; × 3, about **$0.94** for 180.
  Smoke about $0.01.
- Total: about **$1.6**, under the $3.00 ceiling.

## Build (filled and committed before the first session)

Written in two commits, both before the first session: the first, before the freeze (the first
paid command of this run), names the build, the stack, the commands and the rules; the second
adds the frozen `dreamed.json`'s sha256 and the free checks' results. The scripts are in
`m2-stage-b-revised/`. Both commits are pushed to the public repository, on the branch
`m2-stage-b-revised`, before the first session, and are merged later together with the results.
That is a difference from stage B, whose amendment was merged (#52) before its first session:
this one is only pushed. The branch starts at `main`'s `989a9ae`, one docs-only commit past the
build commit (#69, an entry in `docs/backlog.md`); the build is `21f30dc`.

- **Commit**: the binaries and the harness are `main` at `21f30dc` (the merge of #68, tree
  `b2b72bf4130fbcfdfaa5aede78026661df9212ce`), from a `git archive` (no line-ending conversion) in
  `/m2b/main-21f30dc/src`, whose tree was checked to hash to that id. Dependencies fetched first
  (`cargo fetch --locked`), then `cargo build --release --locked --offline -j 8 -p
  margincode-daemon -p margincode-cli -p margincode-bench` in `rust:1.97.1` (the stage B
  container) from an empty `CARGO_TARGET_DIR=/target/main-21f30dc`, copied to
  `/m2b/main-21f30dc/bin` (mode 755) and the harness to `/m2b/main-21f30dc/H/bin` (mode 700, root
  only):
  - `av` `ff48898161a2d38b88f52535170e545e796b53784ec4254ff56e72e89adc27b9`
  - `margincode` `c24399064efa56c1974204a7646fb626fca3af8736cb62597d6238368c44ba9a`
  - `margincode-daemon` `a5702a789ea90ae797cd76963d5d031ae9eb931f012a60b1a4dbe711f2302b1c`
  - `margincode-bench` `a1f7f1e207ef04036e1d45dbab157f5c74c9d5136e46e420357a1376614ec160`
- **The run tree**: `/m2b/main-21f30dc/tree-h`, a `git archive` of `21f30dc` made into a Haiku
  tree by M0's `haiku-tree.sh` (only the `model` line of the 13 task files differs).
  `margincode-bench check bench/tasks-m0`: 3 of 3 `ok`.
- **Container and services**: stage B's `m2b-harness` (node, Claude Code 2.1.318 with
  `DISABLE_AUTOUPDATER=1`, the agent user `benchagent`), started again after a host restart,
  on the network `lp-bench` with stage B's `lp-pg`, `lp-qdrant` and `lp-tei`.
- **Daemon**: `127.0.0.1:7492`, new database `m2r`, store `/m2b/r2/store`, workspace root
  `/m2b/r2/bench-work/ws`, `AV_QDRANT_URL=http://lp-qdrant:6334`, `AV_EMBED_URL=http://lp-tei:80`
  and **`AV_PLAIN_RECORDS=on`**, started by `m2-stage-b-revised/daemon.sh 7492 m2r /m2b/r2` (the
  database password is read from a root-only file and never written down). `GET /v1/version` must
  answer `"plain_records":true`. One daemon, not restarted, serves the freeze, the free checks,
  the smoke and both arms.
- **Options**: `HOME=/m2b`; `--av-url http://127.0.0.1:7492 --work-root /m2b/r2/bench-work
  --import-dir /m2b/r2/bench-work/src --import-mount /m2b/r2/bench-work/src --av-bin
  /m2b/main-21f30dc/bin/av`; for sessions also `--agent-user benchagent --av-ws-root
  /m2b/r2/bench-work/ws`. Task repository and venv are stage B's (`/m2b/bench-repos/tallyho`,
  `/m2b/bench-work/venv`). `m2-stage-b-revised/m2rcmd.sh` runs one command with these options
  (stage B's `m2bcmd.sh` with the new paths); `m2-stage-b-revised/m2rchain.sh` runs an arm.
- **Isolation**: as stage B's: `/m2b/r2` 711 (as `/m2b`); its store, results, checks and
  `bench-work/src` 700; `bench-work` and `ws` 711; the freeze's dream directories and
  `dreamed.json` closed (700) after the freeze, before any session; each command's run and
  workspace directories closed when it ends; `/tmp` emptied before each command.
- **Dreamer and verifier**: stage B's `dreamer.sh` (sha256
  `e2452e7f04cce6ce1e548caf388fec010961b9a4d11d3d6f6497daca6539eea9`), unchanged, with
  `M2_MODEL=claude-sonnet-5-5` and `M2_RUN_CAP=0.25`.
- **The sessions**: as stage B's (Claude Haiku 5.5, first-party API, the key passed in by name
  and never written down, `--price-per-mtok 0.10,0.125,0.01,0.50`, `--jit-hooks`, no MCP, no
  `--keep-work`, no noise, run seed 1, `--session-cap-usd 0.08`, no `--wrong-reverted`).

**Commands** (`m2rcmd.sh <subcommand> <args>`, which adds the options above; results to
`/m2b/r2/results/<out>`, copied to `m2-stage-b-revised/<out>` as the harness wrote them):

```bash
# the freeze (paid, Sonnet 5.5): one review run and one verify run per task
m2rcmd.sh dream-freeze bench/tasks-m0 --limit 50 \
  --dream-cmd 'sh /m2b/dreamer.sh' --verify-cmd 'sh /m2b/verifier.sh' \
  --out /m2b/r2/results/dreamed.json
# the free checks
m2rcmd.sh first-packs bench/tasks-m0 --out /m2b/r2/results/first-packs-raw
m2rcmd.sh first-packs bench/tasks-m0 --dreamed /m2b/r2/results/dreamed.json \
  --out /m2b/r2/results/first-packs-dreamed
# the smoke (not a result)
m2rcmd.sh run bench/tasks-relay/order-ids.toml --arms av --jit-hooks --reps 1 \
  --session-cap-usd 0.08 --budget-usd 0.10 --out /m2b/r2/results/smoke-r2
# raw, then dreamed (budget: the smaller of 2.10 and 3.00 less what has been spent)
m2rcmd.sh run bench/tasks-m0 --arms av --jit-hooks --reps 30 --session-cap-usd 0.08 \
  --budget-usd 2.10 --out /m2b/r2/results/m2-raw-r2
m2rcmd.sh run bench/tasks-m0 --arms av --jit-hooks --reps 30 --session-cap-usd 0.08 \
  --budget-usd 2.10 --dreamed /m2b/r2/results/dreamed.json --out /m2b/r2/results/m2-dreamed-r2
```

- **Output names.** A session's repository is named after its `--out` directory
  (`bm2rawr2-…`), and the semantic index (`lp-qdrant`) is shared with every earlier bench run on
  this stack, so the names `smoke-r2`, `m2-raw-r2` and `m2-dreamed-r2` were chosen to match no
  repository of any earlier run's database (stage B's are `bsmoke`, `bm2raw`, `bm2dreamed`).
- **One smoke**, under `AMENDMENT-1-stage-b.md`'s conditions: every row `status=ok` and valid;
  the model answered and made at least one edit; verify ran; `hook-count` not empty; `meta.json`
  names `benchagent`, Claude Code 2.1.318, `models: ["claude-haiku-5-5"]` and
  `plain_records: true`; no single request with a prompt above 100K tokens; no session above
  $0.04. If it fails, no arm runs and the cause is reported.
- A halt: `/m2b/checks/STOP` refuses every command of `m2rcmd.sh`, as stage B's.

**Results of the freeze and the free checks** (second commit, before the first session):

- To be filled.
