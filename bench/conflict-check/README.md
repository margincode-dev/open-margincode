# The conflict check: a planted set for `av dream --conflicts`

`av dream --conflicts` lets a review run say that a record contradicts a rule in force (an
article of the constitution or a convention on the record's paths). Before it is switched on
anywhere, it has to pass the check its design fixes: a planted set, two review runs, four
conditions. This directory is the set; `margincode-bench conflict-check` is the harness. No
result is kept here.

## The set

- `repo/`: **tidepool**, a small invoicing service in Python (handlers, billing, two jobs,
  storage). It is the only thing imported as the repository, so neither this file nor the key
  is in it.
- `set.toml`: what a person and four agents put in the store. The person (`dana`) adds three
  articles (logs carry no customer data or secrets; money is integer cents; times are UTC) and
  three conventions (`api/**`: errors are JSON; `jobs/**`: a job is safe to run twice;
  `storage/**`: every value is a query parameter). Four agents, one task each, write 24 records
  anchored to `repo/`'s code. One record is written before the rules are added; the others after.
- `key.toml`: which record is which, by its id in `set.toml`:

  | category | records | what it is |
  |---|---|---|
  | `planted` | 8 | its own words tell agents to do what a rule forbids (4 against an article, 4 against a convention; one older than its rule, one claiming an exception) |
  | `overlap` | 4 | the same subject as a rule, no contradiction |
  | `silent` | 4 | the anchored code breaks a rule; the record says nothing about it |
  | `attempt` | 2 | an `attempt` describing a try that broke a rule |
  | `untouched` | 3 | no rule bears on it |
  | `code-disputes` | 3 | the anchored code contradicts it or has moved on |

- `dreamer.sh`: the agent of the last freeze (Plan 16 M2 stage B's, byte for byte): Claude Code
  with `--model "$M2_MODEL" --max-budget-usd "$M2_RUN_CAP"`, tools `Read,Write,Bash`, writing
  `claude-result-<round>.json` in the run's directory.

`margincode-bench conflict-check` refuses a set that does not have these numbers, or that the
daemon could not take as written (a planted convention conflict whose glob covers none of its
anchors, an attempt control that is not an `attempt`, an anchor that is not a file of `repo/`,
a summary over 200 characters, an article over its limits). The set is frozen once committed:
a revised `CONFLICTS` text needs a new set, since this one will have been read.

## What the harness does

1. Seeds the set twice, alike, as two repositories of one bench daemon (`<run>-conflicts` and
   `<run>-plain`): the fixture as one commit, the agents' tasks, the older record, the rules
   (articles through the constitution route, by the person; conventions not required), then
   the other records. A daemon has no copy of a repository, so "copies of one store" are two
   stores seeded from the same file; it checks that exactly the set's six rules are in force in
   each (an organisation article on the daemon stops it).
2. Runs `av dream --repo R --limit 50 --timeout 1800 --fix-rounds 1` on each, with
   `--conflicts` on the first, the agent command in the run's directory. What the agent wrote
   in each round is kept outside that directory (`av dream` removes it before a fix round), and
   `av dream`'s output is kept as `dream.log`.
3. Checks each run was handed all 24 records with their code, rules only with `--conflicts`, and
   `DREAM.md` as pinned (sha256 `f66dc2ec…`) only without it.
4. Scores, from the key:
   - **(a)** at least 6 of the 8 planted records have a `conflict` the daemon accepted, against
     the keyed rule, in the run with `--conflicts`;
   - **(b)** at most 1 of the 16 controls was sent as a `conflict` in that run, accepted or
     refused (the daemon refuses one on an `attempt`; the agent still sent it);
   - **(c)** at least 14 of the 16 controls have the same verdict kind in both runs, the kind
     being what the daemon accepted, or `none` for a record with no verdict;
   - **(d)** at most 2 records were refused, in any round of that run, because a rule quote is
     not in the rule (`the quote is not in art:N|org:N|conv:N word for word`).

It prints each condition with its count and pass or FAIL, then the overall result and the
spend, and writes `result.json` (every record's verdict in both runs, its refusals, the score)
and `report.txt` to `--out`. It exits 1 when a condition fails. A `--out` that holds a finished
check is refused.

## Spend

Only `--confirm` calls a model. Every call is capped at $0.45 (`M2_RUN_CAP`), and a run is at
most two calls (one fix round), so the check is at most four calls, $1.80. Before each run the
harness adds up `total_cost_usd` of every `claude-result-*.json` under `--out` (a call that left
no cost counts at its cap) and does not start a run that could take the total past $2.00. A
check that stops before `result.json` is not a result: it may be started again with the same
`--out`, nothing changed in between, and what it spent counts toward the $2.00 (each start
works in its own `<run>/` under `--out`).

## Running it

A dry run (no model, no network) runs the whole path with a scripted agent,
`margincode-bench conflict-fake`: `keyed` proposes what the key says and passes all four;
`break-a` to `break-d` each break one condition (the wrong article for three planted records;
two controls as conflicts; three controls left out of one run; three rule quotes not in the
rule, corrected in the fix round) and fail it alone. It needs `av` and a bench daemon built
from this tree, on Unix (`av dream` starts its agent only there):

```bash
margincode-bench conflict-check bench/conflict-check --dry keyed \
  --av-url http://127.0.0.1:7430 --av-bin target/release/av \
  --import-dir ~/bench-work/src --import-mount /import --out ~/bench-work/conflict-dry
```

The test `conflict::a_dry_check_passes_with_the_keyed_agent_and_each_break_fails_its_condition`
runs all five against a test daemon; it skips, saying so, unless `av` was built beside it
(`cargo build -p margincode-cli`).

The real run is the same without `--dry`, which prints the plan, and then with `--confirm`. It
also needs Claude Code on the `PATH` and the key in `CLAUDE_API_KEY`, which the harness hands to
the agent as `ANTHROPIC_API_KEY` and never prints. Give it a `--out` outside the repository.

## What stays open

- The agent has `Read` and `Bash`. Nothing but file permissions keeps it from this directory
  (`key.toml`) or from the other run's directory under `--out`: keep both out of its reach (a
  bench tree closed to the agent's user, as in `bench/README.md`, "Agent isolation").
- The bundle gives the record's `created_at` and no date for a rule, so the record older than
  its rule is older in the store, not in anything the agent is shown.
- One run per mode on one model: a pass is that run's, not the text's in general.
