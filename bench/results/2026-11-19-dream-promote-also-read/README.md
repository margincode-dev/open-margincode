# A real model and the `--promote` and `--also-read` sections of `av dream`, 2026-11-19

**An observation, not a claim.** One review run with each flag, one model, one store each. Not
pre-registered, no arms, no comparison. It answers `HANDOFF.md` "Next" 2: the text `av dream`
appends to `DREAM.md` for `--promote` and for `--also-read` had been tested only with agent
scripts that run no model. Here a model read it once each, and this file writes down what it did.

## What ran

- **Build**: `main` at `21f30dc` (tree `b2b72bf4130fbcfdfaa5aede78026661df9212ce`), `av`, `margincode-daemon` and
  `margincode-bench` built from a `git archive` in the stage B container `m2b-harness`
  (`rust:1.97.1`), `cargo build --release --locked` from an empty target directory. sha256:
  - `av` `ff48898161a2d38b88f52535170e545e796b53784ec4254ff56e72e89adc27b9`
  - `margincode-daemon` `a5702a789ea90ae797cd76963d5d031ae9eb931f012a60b1a4dbe711f2302b1c`
  - `margincode-bench` `a1f7f1e207ef04036e1d45dbab157f5c74c9d5136e46e420357a1376614ec160`
- **Daemon**: a new one, `127.0.0.1:7491`, a new database on stage B's Postgres (`lp-pg`,
  migrations to 0032), new store and workspace root, `AV_PLAIN_RECORDS=on` (stage B's setting),
  stage B's Qdrant and TEI. `scripts/daemon.sh` starts it (`daemon.sh 7491 <db> /m2b/r1`).
- **Stores**: two, each seeded exactly as stage B's `dream-freeze` seeds a task store:
  `scripts/r1cmd.sh seed relay-bus-resubscribe …` runs `margincode-bench dream-freeze bench/tasks-m0
  --only relay-bus-resubscribe --limit 50` with agent commands that exit 1 at once (no model,
  $0), as the `DREAM.md` pilots did, so the seeded repository stays and nothing is judged
  (`checks/seed-a.log`, `checks/seed-b.log`). One store per dream, so each dream is handed all
  13 records and neither sees the other's verdicts.
- **Which task, and why**: the three M0 stores hold the same 13 records (`notes-silent.toml`)
  on the same code (`seed_relay_records` seeds the pool, not the task), so the choice does not
  change what a dream is handed beyond the ids. `relay-bus-resubscribe` was taken because that
  store holds the records a promote is for in the same form as the others (decisions that state a
  rule for a path: `customer-region` for `tallyho/validate.py`, `refund-event`, `ledger-refund`,
  `order-ids`, `product-slug`), and it is the store in which stage B's freeze judged one of those
  (`customer-region`, `disputed`), so it is where such a record had last been looked at.
- **The agent**: stage B's `dreamer.sh` unchanged (Claude Code 2.1.318, `claude -p "Read DREAM.md
  and do what it says."`, `--allowedTools "Read,Write,Bash"`, `--permission-mode acceptEdits`),
  `M2_MODEL=claude-sonnet-5-5`, `M2_RUN_CAP=0.25` (its own `--max-budget-usd`). `av dream` was
  given no `--budget-usd`.
- **Commands** (`scripts/r1cmd.sh dream REPO HOME FLAGS`, which runs `av dream --repo REPO --limit
  50 --timeout 1800 FLAGS -- sh -c 'sh /m2b/dreamer.sh'`):
  - `--promote` run: `--fix-rounds 1 --promote` (`checks/dream-promote.log`).
  - `--also-read` run: `--fix-rounds 0 --also-read /m2b/r1/also-read`
    (`checks/dream-also-read.log`). Fix rounds 0 so that the run's worst case ($0.25) stayed
    inside the $0.60 ceiling after the first run's spend; nothing was refused, so no round was
    lost.

## What the model was handed

- `promote/DREAM.md` and `also-read/DREAM.md` are the files the model read, as `av dream` wrote
  them. Each is `integrations/dream/DREAM.md` at `21f30dc` (sha256
  `f66dc2ec655cf8747df585d35c4dc8d0e7205f14ebd3220da50fc220aa6973aa`) followed by
  one appended section: "`promote`: a decision as a convention" or "More to read". Their sha256
  (`18f3b6ca8259cb4433760132a00ef3d0da0726f4d3fb03c55a1e4b013b0e3583` and
  `cf24eedd4ecff2ba0cb8d0e23ad4d80388df865c78609c066c10852e27e25ca7`) equal the `instructions_sha` the daemon stored on each run
  (`dream.json`).
- `bundle.json`: the 13 records each run was handed (all decisions), with their anchored code.
- `also-read/also-read.txt`: one line, `/m2b/r1/also-read`. That directory is `also-read-notes/`
  here: three plain notes written for this run, made up for the purpose and labelled as session
  notes:
  - `2026-11-08-worker-double-send.md` bears on bus-resubscribe's wrong note ("dedupes by name
    ... comparing the function objects misses re-imports"). It says the double send came from
    one function object subscribed twice, and that nothing was imported twice: the reason the note
    gives is a guess that turned out wrong.
  - `2026-11-06-support-log.md` bears on `customer-region` (a true lesson): why the rule exists
    and that it covers every check in `validate.py`, with the old messages still to be fixed.
  - `2026-11-07-standup.md` bears on nothing.

## What the model did

Both runs took 7 turns and about 35 s, and the daemon refused nothing.

| | `--promote` run | `--also-read` run |
|---|---|---|
| verdicts | 5 `disputed` (proposed) | 5 `disputed` (proposed) |
| `promote` verdicts | **0** | (not offered) |
| left out | 8 of 13 | 8 of 13 |
| cost (`total_cost_usd`) | $0.1219 | $0.1196 |

The same five wrong notes were disputed in both runs, with code quotes (`coupon-remove`,
`store-count`, `money-plain`, `tax-with-rate`, `line-taxes`). Both left out the same eight:
`bus-resubscribe` and `quote-split` (wrong notes), `order-merge` (wrong), and the five true
lessons. No verify run was made, so no verdict is effective, and none was accepted by a person.

**With `--promote`.** It proposed no convention, and nothing in its tool calls or its final
message mentions `promote` or conventions (`tool-calls.jsonl`, `agent-final-message.md`; the
thinking blocks are empty in the transcript, so what it weighed is not visible). The records the
section is for were all left out: `customer-region` and `refund-event` as "each states a rule,
and code that breaks a rule doesn't dispute it" (the `customer-region` check on `main` still
echoes the value), and `ledger-refund`, `order-ids` and `product-slug` because it "found no code"
to check them against. Read against the two texts it had: the base text says a rule is not
disputed by code that breaks it, and the promote section says "promote only a decision you checked
holds"; a rule that the code breaks, or that no code exercises, can then be neither confirmed nor
promoted. That is a reading of one run, not a finding about the text.

**With `--also-read`.** It printed `also-read.txt` (`cat also-read.txt`, in the same command that
dumped the bundle) and did not list or open `/m2b/r1/also-read`, or use `$AV_DREAM_ALSO_READ`,
in any later call (`also-read/tool-calls.jsonl`). No reason or quote draws on the notes. The note
that bears on bus-resubscribe's wrong note was not read; that record was left out as "also a rule.
`subscribe` doesn't dedupe, but that doesn't show the rule is wrong." It did not try to quote from
the directory and changed nothing in it.

Beside the two flags, seen in both runs and not asked about: bus-resubscribe's wrong note (a claim
about what re-imports do, plus a rule) was read as a rule and left out, where `DREAM.md`'s third
pilot (#63) had it `disputed` in 3 of 3 stores. One run each; it is not a measure of the text.

## Cost

$0.1219 (`--promote`) + $0.1196 (`--also-read`) = **$0.2415**, 2 Claude Code calls on Sonnet 5.5,
`claude-sonnet-5-5` only (`claude-result-1.cost.json`). The seeding was $0. Ceiling $0.60.

## Not run

- No verify run of either dream, and no `av verdict accept`: nothing here took effect.
- No run with both flags, no second run of either flag, no other model, no other store.
- No run with an `--also-read` directory of real agent transcripts; the notes are made up.
- Nothing ran against the owner's daemon.

## Files

- `promote/`, `also-read/`: `DREAM.md` (as handed), `bundle.json`, `verdicts.json` (as the
  agent wrote it), `verdicts-daemon.json` (the repository's verdicts as the daemon lists them),
  `dream.json` (the run, with `instructions_sha`), `claude-result-1.cost.json` (the cost and
  usage fields of Claude Code's result), `agent-final-message.md`, `tool-calls.jsonl` (each tool
  call's name and input, from the session transcript); `also-read/also-read.txt`.
- `also-read-notes/`: the directory named to the `--also-read` run.
- `scripts/`: `r1cmd.sh` (seed and dream), `daemon.sh` (starts the daemon; reads the database
  password from a root-only file in the container).
- `checks/`: what each command printed.
