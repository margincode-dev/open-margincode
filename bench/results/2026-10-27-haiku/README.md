# Haiku runs, 2026-10-27: why av did not beat git, and what changed

Headless Claude Code (`claude-haiku-5-5`, Claude Code 2.1.311) on `bench/tasks-tallyho`: four one-file
bugs in a 25-file Python library, hidden pytest files decide acceptance. A local daemon without the
semantic layer (`--allow-no-semantic`; packs use Postgres full-text search instead). The tables use
`paired.py` in this directory: per task, arm mean / baseline mean − 1, averaged over the tasks, with a
task-stratified bootstrap 95% interval. Truncated sessions (a full disk once) are left out. Every
av session was accepted; one baseline session in `02-av-with-mcp` was not.

## Before: av cost more than plain git

`01-pilot-before-fixes` (8 sessions per arm):

| vs baseline | av_no_packs | av |
|---|---|---|
| cost | +12.5% [+3.3, +23.2] | +8.2% [−4.3, +23.4] |
| tokens (no cache reads) | +11.6% [+5.3, +18.7] | +20.8% [+3.1, +40.5] |
| turns | +11.9% [−10.8, +37.4] | −18.1% [−25.0, −9.8] |
| wall time | +6.3% [−5.2, +19.7] | +1.5% [−9.7, +15.0] |

## Why

1. **The MCP server is not the cost.** No agent called an av MCP tool in any session. Claude Code
   defers MCP tool schemas, so the 24 tools add about 440 tokens. Hooks only
   (`02-av-hooks-only`) against hooks + MCP (`02-av-with-mcp`): cost +3.6% vs +3.2%,
   indistinguishable.
2. **The pack said the goal three times.** The auto-recorded intent clips its summary to 200
   characters, so it never equalled a long goal's first line and was shown with the goal as its
   body, next to the `Goal:` line. The agent's prompt is the goal already.
3. **Bodies in the pack are paid for twice.** Claude Code refuses to edit a file it has not read,
   so every av agent read the file whose code the pack had just pasted. The pack's bodies were pure
   duplicates, cache-written at the 1-hour price (2× input) and re-read on every turn.
4. **Retrieval on a cold start.** With no edits yet and no identifier in the goal, the pack has
   no seeds, so nothing comes in through the reference graph: agents went looking for the types the
   target code uses (`cat money.py`). Full-text search also takes the first 16 goal words (the
   comment said "rarest first"), so a goal that describes the symptom first can lose its telling
   words; on paginated-scan the pack missed `iter_records`. Searching every goal word fixed that
   task but cost the E1 retrieval benchmark 0.02 to 0.05 on several metrics, and keeping only the
   16 rarest words cost more, so both were dropped and only the comment was corrected.
5. **Where the money goes.** A Haiku session here is about 41% cache writes, 25% cache reads and
   34% output. av saves about one turn (the first `grep`) when the pack names the target, which is
   worth roughly one turn of cache reads. The pack is written to the cache once and read on every
   turn, which cost about as much as it saved. av agents also think about 35% more: once over the
   pack, then again over the files they read. Baseline agents think only once they have the code.
6. **A 25-file repo is the hard case for av.** Baseline needs one cheap `grep` to find the target,
   so a pack can save at most that turn. The medium task set (`bench/tasks-scholion`, 20k lines of
   Rust) is where navigation costs more; it needs about 1 GB of disk per session for cargo and did
   not fit this sandbox, so it is unmeasured.

## The fixes

- The intent that repeats a (clipped) goal is not shown (`repeats_goal`).
- With no seeds, the best functions and methods among the text hits stand in for seeds: what they
  reference joins the pack, as signatures only and at most half the candidate limit. Types do not
  stand in: a type's neighbours are everything that uses it.
- `GET /v1/tasks/{id}/context?code=signatures` (and `CodeDetail::Signatures`): code items show path,
  lines and signature only. `av hook session-start` (the Claude Code hook) now defaults to it;
  `--code full` brings bodies back.
- The session ledger records the tier whose text was actually sent, so a body left out of a
  signatures pack is still sent by a later full pack.
- `PACKER_VERSION` 3.
- `scholion-bench run --no-av-mcp` (the shipped integration is hooks only) and `--hook-code`.

## After

`04-final-shipped-*`, the code in this change with its defaults (40 sessions per arm, all accepted):

| av vs baseline | change | 95% CI |
|---|---|---|
| cost | ±0.0% | [−3.9, +4.2] |
| tokens (no cache reads) | +5.9% | [+2.5, +9.4] |
| turns | −12.9% | [−18.3, −7.0] |
| wall time | −2.3% | [−7.4, +3.3] |

`03-*` compared full bodies with signatures on an intermediate packer that searched every goal
word (40 sessions per arm, `-a` and `-b` are two runs): signatures beat full bodies on every
metric (cost −3.2% vs +1.8%, tokens +2.8% vs +8.4%, wall −9.1% vs −0.8%), which is why the hook
now defaults to them. Wall time depends on API latency at the time of a run, so it is the
noisiest metric; it is no worse with av, and faster in two of the three signature runs.

On this small repo av now matches plain git on cost and takes 13% fewer turns, where it used to
cost 8% more and use 21% more tokens. It does not save tokens yet. The remaining gap is the pack
itself (about 1k tokens cache-written once and re-read every turn) and the extra thinking it
prompts. Both shrink relative to what a pack saves as the repo grows. The next measurement is
`bench/tasks-scholion` on a machine with the disk for it.

## Retrieval (E1, 42 public tasks, no semantic layer)

`e1-retrieval-main` is `main` before this change, `e1-retrieval-final` this change. Columns as in
`bench/retrieval/README.md`: tokens, files_code, symbols_sig, symbols_body, files_named, gold_share.

| budget | main | this change |
|---|---|---|
| 2000 | 1517 · 0.80 · 0.39 · 0.35 · 0.92 · 0.25 | 1531 · 0.80 · 0.40 · 0.34 · 0.94 · 0.24 |
| 4000 | 3163 · 0.88 · 0.56 · 0.49 · 0.96 · 0.25 | 3190 · 0.88 · 0.56 · 0.48 · 0.98 · 0.23 |
| 8000 | 6394 · 0.92 · 0.68 · 0.64 · 0.96 · 0.24 | 6442 · 0.92 · 0.66 · 0.63 · 0.98 · 0.24 |

Within 0.02 everywhere; more gold files are named. E1 scores full packs. A signatures pack shows
no bodies, so for it only files_code, symbols_sig and files_named apply.

## Follow-up: the pack makes agents open more files

`tool-mass.py` over the `04-final-shipped-*` transcripts: av agents read 2.88 files per session
(4.9k characters), baseline agents 1.45 (1.9k, plus 1.8k through `cat`). Signatures packs list a
median of 12 code items over several files, and agents open the files of the weak ones (`Order`,
`Product.__post_init__` on csv-escaping). Full-body packs, which answer more in place, led to about
1.8 reads. Half of an 800-token pack was the repo layout.

On E1 (2000-token packs), the first gold file is the top code item in 64% of tasks, within the top
3 in 83% and the top 5 in 88%. So the hook can name far fewer items and lose little:
`av hook session-start --no-map --max-code N` (and `map=false&max_code=N` on the context route)
leave out the Map section and keep the N best code items. The defaults are unchanged until a run
measures them: `scholion-bench run --no-av-mcp --hook-arg=--no-map --hook-arg=--max-code=3`.
