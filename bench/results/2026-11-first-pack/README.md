# Does a task's own lesson reach its agent? (E4 at scale, follow-up)

2026-11-10. No agent ran and nothing was spent. This is the diagnosis that
`2026-11-e4-scale/README.md` left open ("The lesson is often not in av's first pack").

## Question

E4 at scale reported the task's own lesson in `pack.md` in 12/30 sessions at N = 0, 12/30 at
N = 50 and 6/30 at N = 500 noise notes. Is that what the agents were given, and if not, what were
they given?

## Method

`scholion-bench first-packs` sets each of the 10 relay tasks up exactly as `run --arms av` does
(base tree, relay commits, the pool and the seeded noise as an earlier task's records), waits for
the repo's outbox to drain, and then asks the daemon for what the two hooks print:

- the session-start pack (`GET /v1/tasks/{id}/context`, budget 800, `code=signatures`, with map);
- the pre-read answer for each of the task's `relevant_files`, in order, under the same session.

The lesson is looked up by record id in the pack's receipt, so a shortened summary still counts.
A pack is deterministic for a given state, so one pass per level stands for the 3 reps of E4.

- A record is **shown in full** at the `summary` tier, the `full` tier, or in a pre-read answer:
  all three carry its whole summary (200 characters at most).
- At the **`handle`** tier the summary is cut to 70 characters.
- **Listed** means it is only in the pack's list of omitted items (also cut to 70 characters).

Setup: daemon and `av` built from this branch at `19879a8`, `scholion-bench` at the commit that
adds this file's baseline (the packer is `main`'s, `d4f43c5`), Postgres 16, Qdrant v1.13.6, TEI cpu-1.7.2 with `BAAI/bge-small-en-v1.5`,
noise seed 1: the same stack and embedder as the sweep.

The daemon is the release build, run inside the `lp-harness` container (rust:1.97.1, this
worktree at `/src`) on the Docker network that also holds Postgres (`lp-pg`), Qdrant (`lp-qdrant`)
and TEI (`lp-tei`). This environment was not written down for the baseline and change A; it was
rebuilt for change B from the daemon's start-up log, and checked by replaying the after-A state at
N = 500, which gave the same tier for every task as `after-a/n500`:

```bash
# AV_DATABASE_URL=postgres://av:<password>@lp-pg:5432/bench, the password from lp-pg's environment
cd /root
AV_LISTEN=127.0.0.1:7450 AV_STORE_ROOT=/root/store AV_WS_ROOT=/root/bench-work/ws \
AV_QDRANT_URL=http://lp-qdrant:6334 AV_EMBED_URL=http://lp-tei:80 \
nohup /target/release/scholion-daemon >> /root/daemon.log 2>&1 &
```

```bash
COMMON="--av-url http://127.0.0.1:7450 --work-root /root/bench-work --import-dir /root/bench-work/src --import-mount /root/bench-work/src"
for N in 0 50 500; do
  scholion-bench first-packs bench/tasks-relay --noise $N --out bench/results/2026-11-first-pack/before/n$N $COMMON
done
python3 bench/results/2026-11-first-pack/semrank.py bench/results/2026-11-first-pack/before/n500/first-packs.jsonl \n  bench/tasks-relay <run id> > bench/results/2026-11-first-pack/before/n500/semantic-rank.txt
```

## Result

Tasks out of 10 whose own lesson is in each place:

| noise notes | pack at task start: shown | at task start: shown or listed | hook pack: summary | hook pack: handle | hook pack: listed only | pre-read | shown in full by either hook | shown at all |
|---|---|---|---|---|---|---|---|---|
| 0 | 4 | 4 | 3 | 7 | 0 | 6 | 9 | 10 |
| 50 | 2 | 4 | 3 | 6 | 1 | 6 | 9 | 10 |
| 500 | 1 | 2 | 1 | 5 | 2 | 0 | 1 | 6 |

Per task: `before/n*/first-packs.md`. The packs and their receipts: `before/n*/packs/`.

- **Repeatability.** `before/n500-again` is a second pass at N = 500 on a fresh setup: every
  task's tier, pre-read answers and named files are the same, and one lesson moved from place 3
  to place 4 in an omitted list. An earlier pass at N = 0 (not kept: it had no receipts) had two
  lessons one step lower (summary 2, handle 6, listed 1); shown in full was 9 of 10 both times.
- **Code.** Every hook pack names all of its task's `relevant_files` (14 of 14 at each level),
  and none is marked degraded.
- `before/n500/semantic-rank.txt` is from the first pass at N = 500 (same inputs, other record
  ids).

## Findings

1. **The sweep's count measured a pack no agent read.** "Shown or listed at task start" is 4, 4
   and 2 of 10, which is the sweep's 12/30, 12/30 and 6/30 exactly. That pack is the answer to
   `POST /v1/tasks`. The harness asks for it right after it posts the records and before they are
   embedded (`start_av_task`, then `wait_for_index`), so semantic search cannot find them yet.
   The agent reads the session-start hook's pack, which is built after the index is ready.
2. **Without noise, delivery is good.** At N = 0 and N = 50 the lesson is shown in full to 9 of
   10 tasks and in some form to all 10. The tenth, `relay-quote-split`, gets a 70-character
   handle that stops before the part that matters ("`Money.allocate(n)` splits exactly"), and
   the pre-read hook does not help: the lesson is anchored in `money.py`, which is not among the
   task's `relevant_files`.
3. **At N = 500 it breaks: 1 of 10 in full, 4 of 10 not at all.** Two separate causes:
   - **The pre-read hook ignores the task.** `records_at` orders a file's records by kind, then
     newest first, and stops at 400 tokens (7 or 8 records). With 500 notes spread over the repo's
     files, the lesson was in none of the 14 answers. At N = 0 the same hook delivered 7 lessons.
   - **In the pack, being anchored on a named symbol outranks being about the task.**
     `semantic-rank.txt`: by embedding similarity to the goal, the lesson is the best-matching
     record of 512 for 6 tasks (rank 2, behind the task's own intent), and rank 5, 6,
     14 and 17 for the rest. But a record anchored on a symbol the goal names gets
     `SEED_BONUS` 1.0, and similarity is worth at most 0.30. So noise such as "New optional
     parameters of `ValidationError` are keyword-only" takes the summary slots and the lesson gets
     a handle (`relay-customer-region`) or is cut (`relay-quote-split`, `relay-store-count`).
     The receipts show it: in those three packs the records ahead of the lesson are all
     `anchored` with scores of 1.58 to 1.70, and the best `semantic` record scores 0.64 to 0.73.
   - The two lessons at rank 14 and 17 (`relay-refund-event`, `relay-order-ids`) are outside the
     8 record hits a pack of 800 tokens asks for, so they are not candidates at all.
4. **The History section shows 4 records** in most 800-token packs, whatever the number of
   candidates (up to 203 omitted).

## What this does not show

- Whether showing the lesson changes outcomes: every `notes` and `av` session of the sweep was
  accepted at every level, lesson or not.
- Anything about other repositories, embedders or real records. The noise is 15 templates over
  one repo, and every noise note names its symbol, which the embedder rewards.
- What the agents of the sweep were given by the pre-read hook: that output was not kept, and the
  run's volumes are gone. This replays the same inputs on the same stack, with new record ids and
  timestamps.

## Results of the two changes (2026-11-11)

`PREREG.md` states both changes, the measures, the guards and the decision rule. Nothing was paid
and no model was called: every number below is `scholion-bench first-packs` or `scholion-bench
retrieval` on the stack of "Setup" above.

- **A** (`b35dad0`): the pre-read hook orders a file's records by how well they match the reading
  task's goal. Replay: `after-a/` (`a5325d5`).
- **B** (`bd9cef5`): a pack asks for 32 record hits (`score::record_hit_limit`: up to 4,000
  tokens; from 4,250 tokens, twice the code cap, so 48 at 6,000, 64 at 8,000 and 80 from 10,000)
  plus one for the task's own intent, which no longer takes the place of a
  record, while code hits stay as they were. The 2 History records that match the pack's goal best
  (`score::GOAL_LEADS`: BM25 over the goal's words fused by rank with embedding similarity, as in
  A) lead "Decisions & dead ends" with a score above any anchored record's, as long as the lead's
  own anchored code is unchanged since it was recorded (qualified 2026-11-11, see "Found in
  review"; true of every record of this benchmark). Mined records and other tasks' intents never
  lead. Replay: `after-b/` (`4bffcf0`), held-out set `after-b/held-out/`.

### Design set (noise seed 1)

Tasks out of 10. "Summary/full" is the hook pack at the `summary` or `full` tier. "Pre-read"
counts a lesson in a pre-read answer; after B it falls because a record the pack already showed at
the summary tier is not repeated there.

| noise | state | hook pack: summary/full | hook pack: handle | hook pack: listed only | pre-read | shown in full by either hook | shown at all | relevant files named | degraded hook packs |
|---|---|---|---|---|---|---|---|---|---|
| 0 | before | 3 | 7 | 0 | 6 | 9 | 10 | 14 of 14 | 0 |
| 0 | after A | 3 | 6 | 1 | 6 | 9 | 10 | 14 of 14 | 0 |
| 0 | after B | 9 | 1 | 0 | 1 | 10 | 10 | 14 of 14 | 0 |
| 50 | before | 3 | 6 | 1 | 6 | 9 | 10 | 14 of 14 | 0 |
| 50 | after A | 3 | 6 | 1 | 6 | 9 | 10 | 14 of 14 | 0 |
| 50 | after B | 6 | 3 | 0 | 4 | 10 | 10 | 14 of 14 | 0 |
| 500 | before | 1 | 5 | 2 | 0 | 1 | 6 | 14 of 14 | 0 |
| 500 | after A | 1 | 5 | 2 | 8 | 9 | 9 | 14 of 14 | 0 |
| 500 | after B | 8 | 0 | 0 | 2 | 10 | 10 | 14 of 14 | 0 |

Per task: `after-b/n*/first-packs.md`; packs and receipts in `after-b/n*/packs/` (a lead's receipt
reason is `goal-lead`). At N = 500 after B the two tasks whose lesson is not in the hook pack,
`relay-order-ids` and `relay-refund-event`, are the two whose lesson ranked 17th and 14th by
similarity in `before/n500/semantic-rank.txt`; both get it from the pre-read hook.

**For agents without a pre-read hook** (MCP-only CLIs, which read only the pack), the column that
counts is the first one: at N = 500 the lesson reaches them whole in 8 of 10 tasks instead of 1.

### Tuning on the design set

One constant set was tried: `GOAL_LEADS` = 2 and `RECORD_HITS` = 32 unchanged from A. An
800-token pack upgrades about two History records to the summary tier, so a third lead would only
push an anchored record down to a handle. On the working tree before the commit, this set gave
summary/full 10, 9 and 8 of 10 and shown in full 10 of 10 at each level, with 14 of 14 files
named. That pass had one hook pack marked degraded (`semantic`, `relay-line-taxes` at N = 50); the
same task run alone three times, and a second whole N = 50 pass, had none, and so has the
committed replay above. The summary column varies by pass: at N = 50 it was 9, then 7, then the 6
above. The receipts of the three passes show why. In the tasks that flipped (`relay-money-plain`,
`relay-coupon-remove`, `relay-refund-event`), the lesson is one of the two goal leads in every
pass, with the same score as the other lead: one is first by its words and second by similarity,
the other the reverse, so both fuse to 1/61 + 1/62. Which of the two gets the one summary upgrade
is then decided by record id, and ids are new on every setup. Shown in full was 10 of 10 in every
pass. The replays are all 800-token packs, so the larger record cap above 4,000 tokens (up to 81
record fetches a pack) is not measured here.

### Guards

1. **N = 0 and N = 50: shown in full stays 9 of 10 or better, shown at all stays 10 of 10.**
   Holds: 10 and 10 of 10 in full, 10 and 10 of 10 at all.
2. **Relevant files named is not lower than the baseline at any level.** Holds: 14 of 14 at
   N = 0, 50 and 500, as before; on the held-out set too (below).
3. **No hook pack is marked degraded.** Holds in the committed replays (0 of 30 on the design
   set, 0 of 30 on the held-out set). One tuning pass had one (see above).
4. **`cargo test --workspace` passes, with the retrieval gate's floors unchanged.** Holds. The
   floors are unchanged and `retrieval_gate` passes. At `bd9cef5`'s code (unchanged since) the
   whole workspace passes in `lp-harness`: 1,062 passed, 0 failed, 2 ignored, run under a child
   subreaper that reaps orphans (a few lines of Python with `prctl(PR_SET_CHILD_SUBREAPER)`). Run
   plainly in that container, two tests fail (`scholion-agentrun`
   `process::a_child_past_its_deadline_is_killed_with_everything_it_started` and `scholion-cli`
   `up_starts_a_background_daemon_and_down_stops_it`; 1,060 passed, 2 failed): its PID 1 is
   `sleep infinity`, which never reaps, so a killed process stays a zombie and `kill -0` still
   finds it. `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings`
   pass. Qdrant tests were skipped (no `AV_TEST_QDRANT_URL`).
5. **`scholion-bench retrieval` on `bench/retrieval/ci`, semantic layer on: no metric at 300, 450
   or 600 tokens falls by more than 0.03.** Holds: no metric moved. `before/` had no retrieval
   number, so "before" was measured with the packer of `cc8e8e3` (built in a temporary
   `git worktree`, since removed) and "after" with `bd9cef5`'s, each on a fresh import of the
   fixture (`make-repo.sh`), budgets 300, 450, 600, 0 of 27 packs degraded both times:

   | budget | method | before: files_code / symbols_sig / symbols_body / files_named / gold_share | after |
   |---|---|---|---|
   | 300 | pack | 0.22 / 0.22 / 0.00 / 1.00 / 0.02 | the same |
   | 450 | pack | 0.61 / 0.44 / 0.11 / 1.00 / 0.10 | the same |
   | 600 | pack | 1.00 / 0.89 / 0.56 / 1.00 / 0.21 | the same |
   | 300 | pack_sig | 0.22 / 0.22 / 0.00 / 1.00 / 0.02 | the same |
   | 450 | pack_sig | 0.44 / 0.44 / 0.00 / 1.00 / 0.03 | the same |
   | 600 | pack_sig | 0.67 / 0.67 / 0.00 / 1.00 / 0.04 | the same |

   The subset whose goals name no gold file or symbol (n = 7) is the same before and after too.
   Only mean tokens at 600 moved, by one (`pack_sig` 453 to 454 over all tasks; on the subset
   `pack` 503 to 504 and `pack_sig` 462 to 463).

   **`--reuse-run` is not a fair "after" here.** A first "after" measured with `--reuse-run` on the
   before run's repos fell to 0.00 files_code at 300 tokens. Re-posting the same nine goals makes
   each earlier task's intent, word for word the new goal, a foreign record the pack shows. The
   `cc8e8e3` packer does the same: run with `--reuse-run` on the fresh "after" import, it also
   gives 0.00 at 300 tokens (0.56 / 0.56 at 450). So the drop comes from the procedure, not from
   change B, and before and after are compared on fresh imports. `bench/retrieval/README.md`
   recommends `--reuse-run` for re-measuring a packer without saying this.

### Held-out set

Run after `bd9cef5` was committed, with no change to the packer since then. The baselines in
`before/held-out/` were run before either change.

| set | state | hook pack: summary/full | hook pack: handle | hook pack: listed only | pre-read | shown in full by either hook | shown at all | relevant files named | degraded hook packs |
|---|---|---|---|---|---|---|---|---|---|
| N = 200, seed 1 | before | 2 | 7 | 1 | 7 | 9 | 10 | 14 of 14 | 0 |
| N = 200, seed 1 | after B | 7 | 2 | 0 | 3 | 10 | 10 | 14 of 14 | 0 |
| N = 500, seed 2 | before | 2 | 3 | 3 | 3 | 5 | 6 | 14 of 14 | 0 |
| N = 500, seed 2 | after B | 8 | 1 | 0 | 2 | 10 | 10 | 14 of 14 | 0 |
| N = 500, seed 3 | before | 2 | 3 | 2 | 2 | 4 | 5 | 14 of 14 | 0 |
| N = 500, seed 3 | after B | 8 | 1 | 0 | 2 | 10 | 10 | 14 of 14 | 0 |

Per task: `after-b/held-out/*/first-packs.md`. The held-out set shows what the design set showed.
At N = 500 the hook pack shows the lesson at the summary tier for 8 of 10 tasks on both seeds. The
lesson of `relay-order-ids` is not in the hook pack at any noise level from 50 up, and that of
`relay-refund-event` is a handle there or, at N = 500 with seed 1, absent (both reach the summary
tier at N = 0). The pre-read hook gives both of them the lesson.

### Decision rule

**Fixed, with a semantic layer.** The primary measure (shown in full by either hook at N = 500,
seed 1) is 10 of 10 (baseline 1, after A 9). Each held-out N = 500 seed is 10 of 10 (baselines 5
and 4). Every guard holds.

What it does not show is unchanged from "What this does not show" and the PREREG's limits: one
repository, one embedder, template noise, and no agent runs. Without a semantic layer, change B
works from the goal's words alone (the unit and pack tests), and a record still becomes a
candidate only through an anchor.

### Protocol notes

- The held-out baselines were not opened before `bd9cef5` was committed, but one recursive search
  (`grep -rl retrieval bench/results/2026-11-first-pack/`, looking for a guard 5 number) walked
  `before/held-out/`. Its output was filtered to exclude that directory, so no held-out file name
  or content was shown.
- Building `cc8e8e3` with the same `CARGO_TARGET_DIR` overwrote this branch's release artifacts
  (cargo hashes path crates by workspace-relative path), and the next build of this branch
  thought them fresh. This was caught before any replay, by checking the binary, and the branch's
  crates were cleaned and rebuilt. Every after-B replay and the after-B retrieval run used a
  binary checked to contain change B.
- A gap in the tests of change B as committed in `bd9cef5`: the test that record hits past the
  code cap are candidates runs at 6,000 tokens, where the cap is 48, so its 32 records and the
  task's own intent fit whether or not the intent counts toward the cap. `bd9cef5`'s message says
  the test covers it, and that commit and the test's comment also say the record cap grows "past
  8,000 tokens", where it is from 4,250. The packer was frozen on this branch by then, so a later
  commit that touches tests and this file only closes both: the comment is corrected, and
  `the_tasks_own_intent_does_not_count_toward_the_record_hit_cap` runs at 4,000 tokens, where the
  cap is 32, with the task's own intent and 33 other records as hits, and expects the 32nd shown
  and the 33rd not. It was checked to fail when the intent is made to count toward the cap in
  `merge_hits`, and when the search stops asking for one record more; both edits were reverted,
  and `crates/*/src` is unchanged since `46c221f`.

### Found in review (2026-11-11, after the held-out set; the packer was not changed)

A second reviewer (Grok, read-only, on the pull request) read the diff of both changes. It found
no panic, no nondeterminism reaching a pack and no disagreement between the two `search_with`
implementations, and its spot-checks of two rows of the tables above matched the files. It found
these limits, which the replays could not show because no relay record has a changed anchor:

- **A goal lead whose anchored code has changed since it was recorded may not come first.** It
  keeps the reason `goal-lead`, but a lead scores `(GOAL_LEAD_SCORE + match) * record_factor`.
  For a record whose anchor changed, `record_factor` is `CHANGED_FACTOR` (0.7), so the lead scores about 1.42, below an unchanged
  record anchored on a symbol the goal names (up to 2.0; 1.58 to 1.70 in the N = 500 receipts).
  "Lead with a score above any anchored record's" is therefore true only for records whose
  anchors are unchanged, which is every record of this benchmark. Whether a possibly stale record
  should lead is a decision for a follow-up, not a slip to patch here: ranking a record down when
  its code has moved is the one defence the packer has against a note that went out of date.
- `score::tests::a_goal_lead_outranks_every_record_score` compares `GOAL_LEAD_SCORE` with the
  best record score before `record_factor`, so it does not pin the case above either way. No test
  covers a lead with a changed anchor, or that exactly two records are promoted.
- In `merge_hits` a semantic hit is counted toward the record cap before `include_record` drops a
  record the store says is superseded, so a stale vector can use up one of the 32 places.
- Without an embedder, one goal word that is not in every candidate is enough for a record to
  lead. Candidates then come only through anchors, so this reorders records already anchored near
  the task. It is covered by unit tests and was not measured on a benchmark.

**Follow-up (2026-11-11, pull request #145, `PACKER_VERSION` 7).** Tests and one fix; nothing was
replayed and no number above is new. No record of this benchmark is superseded (the harness posts
none with `supersedes`, and neither the `av` CLI nor the MCP `record` tool can) and, as said
above, none has a changed anchor, so what follows does not run differently on it.

- **Pinned, not changed: a goal lead whose anchored code changed stays damped and may not lead.**
  That is the decision for now; the owner may overrule it. The reason is the one given above: a
  changed anchor is the only sign the packer has that a note may be out of date, and matching the
  goal well does not make a note current. Since then the E4 wrong-lesson cell (pull request #143,
  a draft when this was written) has an observation of what a wrong record costs when nothing
  marks it: on one task, all six `av` sessions took the approach a confidently wrong record
  recommended (post hoc, six sessions, not a claim). Pinned by
  `pack::a_goal_lead_whose_anchor_changed_ranks_below_an_unchanged_record_on_a_named_symbol` (the
  same record leads while its anchor is unchanged, and once it changed sorts below an unchanged
  attempt on the symbol the goal names, still with the receipt reason `goal-lead`) and
  `score::tests::a_goal_lead_whose_anchor_changed_scores_below_a_new_record_on_a_seed`.
  `score::tests::a_goal_lead_outranks_every_record_score` is renamed to what it checks,
  `before_its_record_factor_a_goal_lead_scores_at_least_the_best_record_score`, and the
  description of change B above and the doc comments now carry the condition.
- **Pinned: exactly two records lead.** Of three records that match the goal, the two best lead
  and the third keeps its own rank; a tie the cut would split promotes neither of the tied
  records, and a tie above the cut both
  (`pack::exactly_two_records_lead_and_the_third_best_match_keeps_its_own_rank`,
  `pack::a_tie_that_the_cut_would_split_promotes_neither_record_and_a_tie_above_it_both`).
- **Fixed: a superseded hit takes no place under the record cap.** `merge_hits` now drops a hit
  whose record the store says is superseded before it counts
  (`pack::a_superseded_record_hit_takes_no_place_under_the_record_hit_cap`, which fails without
  the fix). This is half of the finding. The search itself answers at most the cap and one more
  record, and a stale vector still takes one of those. When the task's own intent is among the
  hits, which is the usual case, there is no spare hit behind the cap, and the pack is one record
  short exactly as before. Closing that means asking the search for more than the cap, or having
  the semantic layer check the store; neither is done here.
- **Still open:** the last finding above (without an embedder one goal word is enough to lead,
  not measured on a benchmark), the rest of the superseded-hit finding, and whether a record whose
  code has moved should ever lead.
