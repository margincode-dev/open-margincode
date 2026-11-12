# Pre-registration: stage 2, the same cell on the build with correctable records, 2026-11-12

Written and committed after the free checks and before the first paid session of this run. The
git commit time of this file, and of the push of the branch `wrong-lesson-stage2`, is the
evidence. Nothing below is changed after the first paid session; anything that turns out
differently is written in `README.md` as a deviation.

## Why this run

The second run (`../2026-11-wrong-lesson-2/README.md`) found, by its pre-registered rule, that on
`relay-quote-split` an agent that gets a confidently wrong note as a Scholion record is accepted
less often than one that gets the same text in a notes file: `av` 2 of 30 (0.018 to 0.213),
`notes` 27 of 30 (0.744 to 0.965), `cold` 12 of 12. Its pre-registration said what would follow:
the change then being built, "measured on this same cell with the same rule". That change is now
merged into `ai-work` (#148). This run measures it.

What #148 changes for the `av` agent on this task:

1. The pack prints one line under "Decisions & dead ends" when it shows records of earlier
   agents: ``Claims by earlier agents: check against the code; correct wrong ones with `av record
   --supersedes rec:…` ``. The pre-read hook says the same once per session.
2. `av record --supersedes rec:…` exists: a decision, attempt or note can be replaced by a newer
   one. In the second run's build the agent had no command for that.

## Questions and hypotheses

- **Q1. Does the change help the `av` arm on this cell?** Compared with the second run's `av`
  arm, which is another run on another build (see "Confounds").
  - **R1 (it helps):** `av` is accepted more often than in the second run.
  - Otherwise: not shown at this size.
- **Q2. Does the gap to the notes file remain?** The second run's question, asked again inside
  this run, where both arms ran side by side.
  - **The gap remains:** `av` is accepted less often than `notes`.
  - Otherwise: `av` is not shown worse than `notes` at this size. That is not a finding that
    they are equal.

No hypothesis is stated about which of the two changes does what; they are not separated here.
A prediction, for the record: nothing the agent can read on this task contradicts the note, so
"check against the code" may find nothing to object to, and the result may well be that the
change does not help here.

## Design

The second run's primary cell, unchanged: the task `bench/tasks-relay-wrong/quote-split.toml` on
the pool `bench/tasks-relay/pool/notes-wrong.toml` (texts fixed in commit `a1ad6c2`), no noise,
run seed 1, model `deepseek-flash` through `https://api.deepseek.com/anthropic`, Claude Code
2.1.318, no other model and no other key, `--agent-user benchagent`, no `--keep-work`.

| arm | sessions | what the agent gets |
|---|---|---|
| `cold` | 12 | the base tree only |
| `notes` | 30 | `NOTES.md` with all ten notes, and a `CLAUDE.md` that says to read it and follow its decisions |
| `av` | 30 | the same ten notes as anchored records of an earlier task; hooks + CLI, `--jit-hooks`, no MCP; the build's new line and command |

`cold` and `notes` get nothing #148 touches. They are here as drift controls: if they behave as
they did in the second run, the model, the task and the harness are taken to be the same, and the
two runs' `av` arms may be compared.

No secondary cell. The second run's secondary cell (the silent tasks) is still owed on the second
run's build and is not part of this run.

**Build.** Harness, `av` and daemon are release builds of `ai-work` at the commit named under
"Free checks": the second run's build plus #149 (the second run's results, no code), #148 (above;
`PACKER_VERSION` 8) and #150 (a refused `--agent-user` preflight cleans up after itself, a run
closes its directory when it ends, results files are created private). #150 changes the harness's
housekeeping, not what a session is given.

**Where.** The same container as the second run, in a new root `/b2` with its own daemon (port
7451), database, store, task-repository clone and venv. The second run's set-up under `/bench`
stays as it was, closed to the agent user.

## Free checks, done before this file was committed

- **Build.** Commit `90ba62e` of `ai-work` (`main` + #146, #145, #149, #148, #150; CI on that
  commit, through #147, is green: `check` and `windows-cli`), copied into `/b2/tree` with `git
  archive`; the tree id is the commit's (`391e83c`). `cargo build --release --locked -p
  scholion-cli -p scholion-daemon -p scholion-bench` in a target directory of its own. sha256 of
  the binaries the run uses (`/b2/bin`):
  `av` `021ad167c462d568d5c0d0c4e0fe8842ceb80fddb1f543dc56d179f3166ac956`,
  `scholion` `f52b8b0182e11a94cd4f62b80b1e6ac5e30120d69264cf544757f7cf0f68f90b`,
  `scholion-daemon` `461f0e6632b91af3a20b20e9a023256c0152ee8205089207d49f9056b9bf2d47`,
  `scholion-bench` `ead477b874ec48f3c3c794cb61bdf4189ad148de2fcd98501c419c6ebf32a891`.
  `av record --help` lists `--supersedes`. Claude Code 2.1.318.
- **Specs.** `scholion-bench check` on the build: `relay-quote-split`, `relay-order-ids`: ok.
- **The wrong note and the new line reach the av agent** (`scholion-bench first-packs`, no agent,
  kept as `first-packs-qs/`): 10 records seeded; in the session-start hook's pack (699 of 800
  tokens, 4 records, not degraded) the line "Claims by earlier agents: check against the code;
  correct wrong ones with `av record --supersedes rec:…`" stands directly under "Decisions & dead
  ends" and the task's wrong note is the first record after it, summary whole, with its `rec:`
  handle. The same holds for the pack at task start (725 of 800 tokens).
- **Isolation.** As `benchagent` (uid 999, no home): it can run `claude`, `av` and the venv's
  `pytest`; `/b2/tree`, `/b2/results`, `/b2/store`, `/b2/bench-repos` and `/b2/bench-work/src`
  are mode 700, `/b2/bench-work`, its `runs` and `ws` 711, the earlier run directory under it
  700; the second run's `/bench/bench-work/runs/*` are 700, as are `/root` and the source mount
  `/src`. `/tmp` was emptied. The harness's own preflight runs at the smoke.
- **Fresh state.** A new database and a new store for this run's daemon. The task repository is
  a clone of the second run's with identical refs (`main` = `11c17fd`); the venv has the same
  package versions (pytest 9.2.0). The second run's daemon was stopped; only this
  run's daemon answers in the container. The Qdrant and embedding services are the ones the
  second run used (collections are per repository and store).
- **Balance** read at 06:55 UTC on 2026-11-12: $2.19.

## Commands (in this order)

In `lp-harness`, as root, `HOME=/b2`, from `/b2/tree`, each under the subreaper, the key passed
in with `docker exec -e` and never written down; the model environment is the second run's:

```bash
P="--price-per-mtok 0.30,0.30,0.006,1.20"
ISO="--agent-user benchagent --av-ws-root /b2/bench-work/ws"
COMMON="--av-url http://127.0.0.1:7451 --work-root /b2/bench-work --import-dir /b2/bench-work/src --import-mount /b2/bench-work/src $ISO"
OUT=/b2/results
# 0. smoke: does an isolated session run at all on this build? (not a result)
scholion-bench run bench/tasks-relay/order-ids.toml --arms cold,notes,av --jit-hooks --reps 1 $P \
  --session-cap-usd 0.08 --budget-usd 0.30 --out $OUT/smoke $COMMON --confirm
# 1. the control
scholion-bench run bench/tasks-relay-wrong/quote-split.toml --arms cold --jit-hooks --reps 12 $P \
  --session-cap-usd 0.08 --budget-usd 0.40 --out $OUT/s2-cold $COMMON --confirm
# 2. the two arms that carry the wrong note
scholion-bench run bench/tasks-relay-wrong/quote-split.toml --arms notes,av --jit-hooks --reps 30 $P \
  --session-cap-usd 0.08 --budget-usd 1.40 --out $OUT/s2-wrong $COMMON --confirm
```

A rerun of the same command resumes it. The results directories are copied into this directory
afterwards, as the harness wrote them.

**The smoke** is the second run's: three isolated sessions on `relay-order-ids` with its true
lesson. It passes if each row is `status=ok` and valid, the event stream shows the model answered
and made at least one edit, the verify command ran, `av`'s `hook-count` is not empty, and
`meta.json` names `benchagent` under `agent_isolation` and Claude Code 2.1.318. If it fails, the
cause is fixed and it is run again in a new directory (`smoke-2`, …); no other command starts
before one smoke passes. Smoke rows are not part of any arm. Whether the new line is in the
smoke's `av` session is looked at and reported, and decides nothing.

Commands 0 to 2 run now, at DeepSeek's peak prices, as the second run's did.

## Metrics and decision rule (fixed now)

**Primary:** accepted / valid sessions on `relay-quote-split`, per arm, with the 95 % Wilson
score interval, computed by `analyze.py` of this directory (committed with this file; it imports
the second run's `analyze.py`, so the tables are the same code).

Reference values, from the second run: `av` 2 of 30 (0.018 to 0.213), `notes` 27 of 30 (0.744 to
0.965), `cold` 12 of 12.

- No claim is made unless `av` and `notes` each have 30 valid sessions.
- **Drift controls.** `cold` is accepted at least 10 of 12 times (or the same share of fewer
  valid sessions), and this run's `notes` interval overlaps the second run's (0.744 to 0.965; at
  30 sessions, 18 accepted or more).
- **R1 is reported** only if both drift controls hold and the lower end of this run's `av`
  interval is above 0.213, the upper end of the second run's. At 30 sessions that is 11 accepted
  or more (10 of 30 has the lower end 0.192).
- **R1 is not assessed** if a drift control fails: this run is then not compared with the second
  run, whatever the `av` arm shows, and the README says which control failed.
- **Otherwise R1 is not reported:** the change is not shown to help at this size.
- **Q2, inside this run and whatever the controls say:** if the upper end of `av`'s interval is
  below the lower end of `notes`'s, **the gap remains**. Otherwise `av` is **not shown worse than
  `notes` at this size**, reported with both intervals and without the word "equal" or "closed".
- If `cold` is below 10 of 12, the README also says the task is not one the model solves unaided
  here, as the second run's rule did.

The four outcomes and what the README may say:

| R1 | Q2 | the README says |
|---|---|---|
| reported | gap remains | the change helps on this cell and does not close the gap to the notes file |
| reported | not shown worse | the change helps on this cell; `av` is no longer shown worse than the notes file at this size |
| not reported | gap remains | the change does not fix this cell; the second run's finding stands on the new build |
| not reported | not shown worse | no clear answer at this size; both intervals given, nothing claimed |

**Secondary, reported and not used for the decision:**

1. **Delivery of the new line:** `av` sessions whose session-start hook output holds it
   (`analyze.py`, `hint`).
2. **Use of the command**, read from each `av` session's transcript: sessions that ran `av
   record` naming `supersedes`; sessions where the CLI answered `supersedes rec:…` (accepted by
   the daemon); sessions where the superseded handle is the task's wrong note; and the two-by-two
   table of "superseded the wrong note" against "accepted". The table describes; it is not a test,
   and a session can supersede the note after writing the note's code or before.
3. Sessions that wrote the wrong edit (`repeated_dead_ends` ≥ 1), accepted and not, per arm.
4. What the agent did with the note, by `behaviour.py` (the first run's counts: edited
   `NOTES.md`; `av` sessions that recorded the wrong approach again; `av` sessions that recorded
   an `attempt`).
5. Delivery of the note per session, as in the second run.
6. Cost per accepted task and median session cost, as the harness reports them.
7. `content.py`, which in the second run was written after the result and is registered here in
   advance: a search of every transcript for hidden-test names, and for every session that was
   not accepted the last text it wrote to the task's file and whether that text calls `.times(`
   (the note's approach) or `.allocate(`.

**Leak check, before the result is read.** `leak.py` of this directory is the second run's scan
with this run's directories; everything under `/bench` (the second run's tree, results and
transcripts, which hold accepted solutions of this same task) counts as exposing. It and
`content.py`'s first table are run and committed before `rows.jsonl` is read for accepted counts.
The primary is computed on all valid sessions, as registered. Exposed sessions are listed, and
the same table is given without them. An exposed session is a deviation and is written up as one.

## What follows, decided now

- R1 reported: the README says so with the limits below. A second task and a second wrong note
  are the next evidence needed before anything general is said.
- R1 not reported, or not assessed: the README says that the line and the command, as built, are
  not shown to help on this cell. The command stays as a capability (an agent that finds a record
  wrong can now correct it); no claim of benefit is made for it anywhere. What to try next is not
  decided here.
- Either way the numbers of both runs are reported together, the unfavourable ones as plainly as
  the favourable.

## Confounds named in advance

- **R1 compares two runs.** The second run's `av` arm ran about two hours earlier, on another
  build, against another daemon and store. The drift controls guard against a change of model
  behaviour, task or harness between the runs; they cannot exclude one that touches only the `av`
  arm (#150's housekeeping, the fresh store, the shared Qdrant). An old-build `av` arm run
  alongside would be the stronger control and was left out for cost.
- **Two changes at once.** The line and the command arrived together; a difference cannot be
  assigned to either. The line also changes the pack's wording, and through `PACKER_VERSION` 8
  what the pack reserves room for (the pack here still shows 4 records with the wrong note first).
- **The line names the remedy.** An agent told that claims may be wrong may distrust true records
  too. This cell has one wrong note and nine true ones that do not bear on the task, so it cannot
  show that cost. The smoke's single session on a true lesson is not a measure of it.
- The second run's confounds stand: the arms differ in framing (`CLAUDE.md` says to follow
  `NOTES.md`'s decisions; the pack now says to check claims), the pack shows a record's summary
  and the notes file summary and body, and it is one task, one wrong note, one wording, one model.
  Thirty repetitions of one task are not thirty tasks.

## Validity and missing sessions

As in the second run: a session the harness marks `invalid` stays in `rows.jsonl` and is not
retried; a session that stops the harness with an error (no row written) is an infrastructure
failure, the cause is fixed and the same command is rerun; no row is edited, dropped or re-run by
hand. A host crash in the middle of a command is such a failure, and the rerun resumes it.
Delivery is checked on the first session of each note arm of command 2 before the rest is read as
valid: `notes`, a read of `NOTES.md`; `av`, the wrong note whole and the new line in the
session-start hook's output. If it fails, the run stops and nothing of that arm is reported as a
result.

## Stopping rules

- `--session-cap-usd 0.08` per session; `max_turns` 30 is the real per-session limit.
- `--budget-usd` per command as written above: $0.30, $0.40, $1.40.
- **Balance floor $0.50.** The balance is read before command 0 and after command 2. Before each
  command its budget is lowered to what the balance leaves above the floor, if that is less. An
  arm that runs out of budget is reported as partial, never extrapolated.

## Limits known in advance

- Everything under "Confounds".
- The isolation closes file-system routes only (`bench/README.md`, "Agent isolation"): the daemon
  answers any session about any repository, and the container's network is open.
- The use of the command is read from transcripts: a correction made some other way (the HTTP API
  directly) is counted by `leak.py` as a daemon call, not by measure 2.
- This run's rows are not pooled with the second run's.
