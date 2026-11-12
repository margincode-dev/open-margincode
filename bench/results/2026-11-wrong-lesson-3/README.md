# Stage 2: the wrong-lesson cell on the build with correctable records, 2026-11-12

The second run ([`../2026-11-wrong-lesson-2/README.md`](../2026-11-wrong-lesson-2/README.md))
found that on one task a confidently wrong note costs far more as a Scholion record than in a
notes file. This run repeats that cell on the build that carries the change the second run named
as its follow-up (#148): the pack tells the agent that records are earlier agents' claims to check
against the code, and `av record --supersedes` lets it replace a wrong one.

The rule, the commands and the analysis scripts were fixed in [`PREREG.md`](PREREG.md) and pushed
before the first paid session (commit `f51bff7`).

## Result, by the pre-registered rule

`relay-quote-split`, model `deepseek-flash`, 72 isolated sessions ([`analysis.md`](analysis.md)):

| arm | valid | accepted | 95 % Wilson | second run, build without the change |
|---|---|---|---|---|
| `cold` | 12 | 12 | 0.758 to 1.000 | 12 of 12 |
| `notes` | 30 | 28 | 0.787 to 0.982 | 27 of 30 (0.744 to 0.965) |
| `av` | 30 | 12 | 0.246 to 0.577 | 2 of 30 (0.018 to 0.213) |

- Both drift controls hold: `cold` 12 of 12, and the `notes` interval overlaps the second run's.
- **R1 is reported: the change helps on this cell.** The lower end of `av`'s interval (0.246) is
  above the upper end of the second run's (0.213).
- **The gap remains.** The upper end of `av`'s interval (0.577) is below the lower end of
  `notes`'s (0.787).

In the words the pre-registration allowed for this outcome: the change helps on this cell and
does not close the gap to the notes file. With the wrong note as a record, an agent on this build
still fails this task 18 times in 30; with the same text in a notes file, 2 times in 30; with no
note at all, never.

R1 compares two runs made about two hours apart on two builds. The controls did not move, but
that comparison is weaker than the one inside this run. See "Limits".

## Leak check (run before any row was read)

[`leak.md`](leak.md) and [`content-scan.md`](content-scan.md) were committed (`7305264`) before
`rows.jsonl` was read for accepted counts.

- No session is exposed: none named a hidden test, another session's directory or workspace, the
  bench tree, the results, the store, the second run's directories under `/bench`, the web, or
  the daemon over HTTP.
- No transcript holds the file name of a hidden test or the name of any of the 30 test functions
  the hidden tests define (72 sessions searched, inputs and outputs).
- The "looked around" column is inflated, as in the second run: the scan counts a bare `/` inside
  other text. It is kept as written.
- What the two scans can miss is unchanged from the second run's README ("Leak check"): a path
  put together in the shell or read through a variable is not classed by `leak.py`; a partial
  read of a hidden test that showed none of the searched names would pass `content.py`; neither
  sees what the daemon or the network would answer.

## Secondary measures as registered

**1. Delivery of the new line.** The line "Claims by earlier agents: check against the code;
correct wrong ones with `av record --supersedes rec:…`" is in the session-start pack of 30 of 30
`av` sessions, with the task's wrong note and its handle.

**2. Use of the command** (`analysis.md`, read from transcripts):

| `av` sessions | ran `av record` naming `supersedes` | accepted by the daemon | superseded the task's wrong note |
|---|---|---|---|
| 30 | 16 | 16 | 12 |

| | accepted | not accepted |
|---|---|---|
| superseded the task's wrong note | 12 | 0 |
| did not | 0 | 18 |

The table describes and is not a test. It does not show that the command causes an accepted
task: see the post hoc timing below.

**3. The wrong edit.** `av`: 18 sessions wrote the note's approach and none of them was accepted;
the 12 accepted sessions never wrote it. `notes`: 2 wrote it, neither accepted. `cold`: 0.

**4. What the agent did with the note** ([`behaviour.md`](behaviour.md)). `notes`: 28 of 30
sessions edited `NOTES.md`, and those 28 are the 28 that were accepted (checked per session after
the result; the two that did not edit it are the two that failed). `av`: 30 of 30 ran `av record`, none recorded an
`attempt`. The column "restates the wrong edit" (29 of 30) matches any record text that names the
note's call, so it also counts the corrections that say not to use it; it does not separate the
two and is not used here.

**5. Delivery of the note.** 30 of 30 in both note arms (`av`: in the session-start hook's
output; `notes`: a read of `NOTES.md`). `cold`: 0 of 12.

**6. Cost**, as the harness counted it at the registered prices: `cold` $0.1112 (median session
$0.0093), `notes` $0.4372 (median $0.0145, $0.0156 per accepted task), `av` $0.4393 (median
$0.0134, $0.0366 per accepted task).

**7. Content of the last edits** ([`content.md`](content.md), registered in advance this time).
All 20 sessions that were not accepted (18 `av`, 2 `notes`) last wrote a `split` that calls
`.times(` and not `.allocate(`.

## Post hoc: what was superseded, and when

Written after the result was known ([`supersedes.py`](supersedes.py),
[`supersedes.md`](supersedes.md)). Not part of the rule.

- **The correction follows the decision.** In all 12 sessions that superseded the task's wrong
  note, the superseding record was written after the first edit of the task's file, and that edit
  already used `Money.allocate`. So in these sessions the command is how the agent wrote down
  that it had rejected the note; it is not what made it reject the note. The new line may be; this
  run cannot tell the line from the command, or either from chance differences between two runs.
- **Other tasks' wrong notes were corrected too.** The pack shows four records: this task's wrong
  note, the wrong notes of two other tasks (`money-plain`: every currency has two decimals;
  `line-taxes`: tax each line on its own) and one true note. The code contradicts those two.
  Sessions superseded the `money-plain` note 9 times and the `line-taxes` note 6 times. Four
  sessions corrected one or both of those and still followed this task's wrong note, which no
  code contradicts.
- **No true note was superseded** in any session. Only one true note is in the pack, so this is a
  small sample of the cost the pre-registration named (an agent told to distrust records may
  distrust true ones).
- **The summary limit costs turns.** 53 `av record` calls in 28 of 30 sessions were refused
  because the summary was over 200 characters, and the session tried again. Nothing told the
  agent the limit before it wrote.

## Deviations and corrections

- **An error in the pre-registration.** Under "Confounds" it says the cell has "one wrong note
  and nine true ones". The pool `notes-wrong.toml` has five wrong notes, one per task of
  `bench/tasks-relay-wrong`, and five true ones; only one of the five is wrong about this task.
  The design and the rule are not affected (the pool is the second run's, unchanged), but the
  sentence is wrong, and the pack an `av` agent saw held three wrong notes and one true one. The
  post hoc section reports what sessions did with the other two.
- **The first start of the smoke was refused** by the harness's own preflight: the workspace the
  free `first-packs` check had left was open to the agent user. No session started. It was closed
  (`chmod 700`) and the smoke ran in the same directory. The smoke passed: three rows `ok`, the
  model answered and edited in each, verify ran, `av`'s hook count is not empty, `meta.json` names
  `benchagent` and Claude Code 2.1.318. The new line is in the smoke's `av` session.
- **A second file for the content scan.** `content.py` prints both of its tables at once, and the
  second reads which sessions were accepted. Only the first table was kept before the rows were
  read (`content-scan.md`); the whole output is `content.md`.
- **`supersedes.py`** is post hoc, as said above.
- **Spend.** The harness counted $0.9877 for the 72 sessions and $0.0532 for the smoke. The
  account balance went from $2.19 to $1.21. The second run's 72 sessions were counted at $0.9469
  and the balance then fell by $0.48; why the two runs were billed differently is not known.

## Limits

- **R1 is a comparison between runs.** The second run's `av` arm ran earlier, on another build,
  daemon and store. `cold` and `notes` did not move between the runs, which argues against a
  change in the model, the task or the harness. They cannot exclude a difference that touches
  only the `av` arm. An `av` arm of the old build run alongside would have been the stronger
  control.
- **Two changes at once.** The line and the command came together. The post hoc timing suggests
  the command did not cause the rejections; it does not show that the line did.
- **One task, one wrong note, one wording, one model.** Thirty repetitions of one task are not
  thirty tasks. On this task nothing the agent can read contradicts the note, which is the hard
  case; the result says nothing yet about notes the code contradicts, beyond the post hoc counts.
- **The cost of distrust is not measured.** No task here depends on a true record that the agent
  might now doubt.
- The arms still differ in framing and in what they show (the pack prints a record's summary,
  the notes file summary and body), as the second run's README says.
- The isolation closes file-system routes only (`bench/README.md`, "Agent isolation").
- These rows are not pooled with the second run's.

## What follows

As decided in the pre-registration: a second task and a second wrong note are the next evidence
needed before anything general is said about the change. Two things this run shows that are worth
acting on whatever that evidence says:

- An agent that follows a wrong record goes on to record what it built: all 18 `av` sessions that
  were not accepted stored a record of their own that names `total.times(` (counted after the
  result, by that text in an `av record` call the daemon accepted). The wrong claim then stands in
  a second, newer record.
- The 200-character limit should be told to the agent before it writes a summary, not after:
  `av record --help` and the hooks do not state it.

## Files

- `PREREG.md`: the pre-registration. `analyze.py`, `leak.py`, `content.py`, `behaviour.py`:
  committed with it. `supersedes.py`: post hoc.
- `s2-cold/`, `s2-wrong/`, `smoke/`: what the harness wrote (`rows.jsonl`, `report.md`,
  `meta.json`). `first-packs-qs/`: the free check of the first pack, with the packs.
- `leak.md`, `exposed.json`, `content-scan.md`: the scans made before the rows were read.
  `analysis.md`, `behaviour.md`, `content.md`, `supersedes.md`: the scripts' output.
- Transcripts are not committed; they stay in the container under `/b2/bench-work/runs`, closed
  to the agent user.
