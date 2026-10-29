# Retrieval task set (Plan 6, E1)

`public.jsonl` holds 42 retrieval tasks mined from six public repositories by
`scholion-bench mine`:
- one to three non-test source files changed;
- at most 150 changed lines;
- committed since 2025-01-01.

Each task's goal is the commit message, with trailers and links removed. Every repo is in a
language the index parses.

| repo | language | clone |
|---|---|---|
| click | Python | `https://github.com/pallets/click` |
| httpx | Python | `https://github.com/encode/httpx` |
| ripgrep | Rust | `https://github.com/BurntSushi/ripgrep` |
| fd | Rust | `https://github.com/sharkdp/fd` |
| ky | TypeScript | `https://github.com/sindresorhus/ky` |
| hono | TypeScript | `https://github.com/honojs/hono` |

## Running it

```bash
mkdir -p ~/eval-repos && cd ~/eval-repos
for r in pallets/click encode/httpx BurntSushi/ripgrep sharkdp/fd sindresorhus/ky honojs/hono; do
  git clone https://github.com/$r.git
done
cd -   # back to this repo

# baselines only, no daemon (seconds)
scholion-bench retrieval bench/retrieval/public.jsonl --no-pack --out /tmp/e1

# with packs, from a bench daemon (see bench/README.md for the bench stack)
scholion-bench retrieval bench/retrieval/public.jsonl --url http://127.0.0.1:7430 \
  --import-dir ~/bench-work/src --out bench/results/retrieval-$(date +%F)
```

`--repos DIR` points the tasks at clones somewhere other than `~/eval-repos`.

## How to read it

- **Gold set:** the files the fix modified (not added), and the innermost symbols whose lines it
  changed.
- **Scoring:** each method's text is cut to the same cl100k budget and scored by line matching
  against the gold set. The metrics are explained in the report.
- **Two packs:** `pack` asks the daemon for `code=full` (comparable with runs before
  2026-10-29); `pack_sig` asks for `code=signatures`, the default agents get. Budgets default to
  800 (the agents' default), 2000, 4000 and 8000.
- **Leaky goals:** commit messages often name the code they change (a `leak`). The report shows
  those tasks separately, since they are easier for every method.
- **Limits:**
  - A commit message is a thinner goal than a real issue.
  - The gold set is what one fix touched, not everything relevant.
  - The text measures what an agent is shown, not whether it would succeed. E3 measures that.

`--reuse-run ID` measures a new packer on the repos an earlier run (its `meta.json` names the id)
already imported into the same daemon, so a rerun takes minutes instead of an hour of embedding.
Reruns differ by about ±0.02 between themselves (semantic search sometimes times out).

## CI regression gate

`cargo test -p scholion-bench --test retrieval_gate` (part of `cargo test --workspace`, about 5 s)
runs the same pipeline on a fixture in `bench/retrieval/ci/`: a 12-file repo in Python, Rust and
TypeScript (`repo/`), nine tasks (`tasks.jsonl`: a goal each) and one patch per task (`fixes/`) that
is the fix. It starts a real daemon with no semantic layer, so it guards the lexical, structural
and budgeting parts of the packer, not the embedding model, and it needs no network, Ollama or
clones. The pack's mean `files_named`, `files_code` and `symbols_sig` at 300, 450 and 600 tokens
may not fall below `FLOORS` in `crates/scholion-bench/tests/retrieval_gate.rs`. The budgets are
small because the fixture is: past about 1000 tokens everything fits and any ranking scores 1.0.

To change a floor on purpose, run the test with `-- --nocapture`, which prints the measured mean for
every metric and budget, set each floor to that value minus 0.03 (rounded down), record the
measured values in the comment above `FLOORS`, and say in the commit message why retrieval
dropped. Do not lower a floor to make an unexplained failure pass. To add a task, add a line to
`tasks.jsonl` and a `fixes/NAME.patch` (the diff of the fix against `repo/`), then re-measure.

## Results: BM25 and rank fusion (2026-10-28, handoff A1)

`results/2026-10-28-packer-v3` re-baselined the pack at `main` (fresh imports). Then every variant
ran on those imports with `--reuse-run`. `symbols_sig` is the share of gold symbols whose
signature the text shows:

| pack variant (all tasks) | 2k | 4k | 8k | files_named 4k |
|---|---|---|---|---|
| `main` (`results/2026-10-28-control`) | 0.41 | 0.61 | 0.74 | 0.98 |
| + Rust/Python test code ranked down | 0.34 | 0.57 | 0.68 | 0.98 |
| + true BM25 instead of `ts_rank` | 0.29 | 0.55 | 0.71 | 0.96 |
| + budget the last section leaves upgrades earlier handles | 0.37 | 0.58 | 0.74 | 0.96 |
| same, signatures before any body | 0.33 | 0.43 | 0.67 | 0.96 |
| + rank fusion of lexical and semantic hits, k = 60 (`results/2026-10-28-packer-rrf`) | **0.54** | **0.73** | **0.82** | 0.96 |
| same, k = 10 | 0.34 | 0.58 | 0.74 | 0.96 |
| `bm25_symbols` baseline | 0.55 | 0.64 | 0.76 | 0.92 |

Only the `main`, k = 60 and `bm25_symbols` rows were scored by the current scorer. The other rows
used the old one, which missed a signature shown without its opener (` {`, `:`), the form the
pack's summary tier prints. That understated the pack, by 0.03 at 4k for `main` alone, more once
more items are summaries. Compare the intermediate rows with each other, not with the first and
last.

The kept change is the k = 60 row (it includes the rows above it except "signatures before any
body"). It passes the A1 gate: `symbols_sig` at 4k is 0.73 against `bm25_symbols`' 0.64, and 0.82
against 0.76 at 8k; 2k is level (0.54 vs 0.55). `files_named` is 0.96 instead of 0.98 at 4k:
one task (ripgrep-020687a) names one of its two gold files instead of both; 2k and 8k are unchanged.
