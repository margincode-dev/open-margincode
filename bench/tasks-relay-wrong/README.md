# E4, wrong lessons: what a confident wrong note does

The five relay tasks whose lesson is a dead end, taken unchanged from
[`../tasks-relay`](../tasks-relay), with one difference: the pool is
[`../tasks-relay/pool/notes-wrong.toml`](../tasks-relay/pool/notes-wrong.toml). In it each of
these five tasks' own lesson is replaced by its opposite, a note that recommends, as a decision,
the approach the real lesson warns about. `reference/check.py` proves that approach fails the
task's hidden test (`dead` in `reference/edits.py`).

| arm | what B gets |
|---|---|
| `cold` | the base tree only: the control |
| `notes` | the five true decision lessons as commits, plus a `NOTES.md` of all ten notes, five of them wrong |
| `av` | the same commits, plus all ten notes as anchored records of an earlier task |

The question, the design and the reason these five tasks were chosen are in
[`../results/2026-11-wrong-lesson/DESIGN.md`](../results/2026-11-wrong-lesson/DESIGN.md).

- **Only the memory channel carries the wrong note.** The five wrong notes set
  `no_commit = true`: they are in `NOTES.md` and in the av records, and in no commit message. A
  later record can correct a record; nothing corrects a commit message, so a wrong note in the
  log would be outside anything a memory layer can do about it.
- **Delivery is held constant.** A wrong note keeps its lesson's place in the pool, its path and
  its symbol, so the pack and the pre-read hook find it exactly where they found the lesson.
- **The other five notes are the true decision lessons**, unchanged. Their tasks are not in this
  directory: a decision task that loses its lesson only falls back to what `cold` does.

The free replay comes before any paid session:

```bash
margincode-bench first-packs bench/tasks-relay-wrong --out <dir> <the stack's options>
```

It reports, per task, whether the wrong note is in the session-start pack and in the pre-read
answers. A cell is only worth paying for if the note reaches the agent.
