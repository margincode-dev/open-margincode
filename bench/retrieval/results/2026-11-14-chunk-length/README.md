# Embedding chunk length and chunking strategy (2026-11-14)

Question: the index cuts every symbol's chunk to `MAX_EMBED_CHARS = 2000` characters before it is
embedded (`margincode_index::cap_chars`), a cap sized for `BAAI/bge-small-en-v1.5` (512 tokens).
The host default embedder is `embeddinggemma-2` via Ollama (model context 262,144) and its fallback
`qwen3-embedding` is a 32K model. Which chunk length, and which way of cutting a long symbol, works
best for each?

## Answer

- **Keep `MAX_EMBED_CHARS = 2000` for now; the next candidate is 4000.**
  - 4,000 chars is the one length that beat 2,000 for both Ollama embedders. It roughly doubles
    what they see of a long symbol and stays well inside their default limits.
  - It was not changed here because two things are unmeasured at 4,000: the lexical side
    (`code_chunks.text` is also BM25's corpus) and an end-to-end semantic pack run. Gains at the
    pack's default 8 code hits are inside the noise.
  - An existing index also would not pick a new cap up (below). Do both runs in the change
    that raises the cap.
- **Shorter is clearly worse.** For `embeddinggemma-2`, 500 and 1,000 lose at every metric.
- **`embeddinggemma-2`:**
  - Longer helps deep recall but not the top hit. At 4,000 to 8,000 chars r@20 and r@40 rise
    (public set: r@20 +0.14 to +0.15, 11 to 13 tasks better and 1 to 2 worse); MRR and r@8
    move about +0.02 to +0.05 with intervals that include 0.
  - Past about 7,000 chars there is nothing more except a few very large click functions, and
    reaching past that needs `num_ctx`.
- **`qwen3-embedding:8b`** (public set, truncation arms only):
  - Best at 4,000: r@20 +0.06 (7 better / 1 worse), r@8 +0.04.
  - 8,000 and longer are no better than 2,000, and r@8 is worse (−0.04, 1 better / 5 worse).
  - So "as long as the 32K context allows" is wrong for this model; it is not the window that
    limits it.
- **`bge-small-en-v1.5` (TEI) is unaffected by any cap above about 1,550 chars.**
  - It embeds its first 512 tokens: 2,000 chars of code are 550–650 tokens.
  - 2,000 and 8,000 chars give the same vector for Rust and TypeScript (cosine 1.000000), and
    nearly the same for prose-heavy Python (0.993, where 2,000 chars are under 512 tokens).
  - Raising the cap costs it nothing. The full TEI arm run was stopped at 41 of 121 tasks and
    is not reported.
- **`num_ctx` does not need to be passed for 4,000.**
  - Ollama embeds at most min(`num_ctx`, `num_batch`) tokens of one input: 2,048 for gemma
    with its defaults, 4,096 for qwen3. `num_ctx` alone changes nothing; both must be raised.
  - 4,000 chars is about 1,000 to 1,200 tokens for both, and even 8,000 chars (about 2,050
    gemma tokens) is at gemma's limit.
  - Passing options would also have to be done identically on every query and index request,
    or Ollama reloads the model and blows the pack's 150 ms search budget.
- **Strategy: truncation is enough.**
  - Windows (best window or mean of windows) and members-only containers are no better than
    plain truncation at the same length.
  - Max-over-windows would also need several Qdrant points per symbol (today one per symbol
    id). Do not build it.
  - Containers barely matter: Rust has no impl-block symbol, and gold symbols are innermost.

## Results

Data:
- **Public set:** `bench/retrieval/public.jsonl`, six public repos, 40 of 42 tasks scored (2
  had no gold symbol). Lead with this.
- **Local set:** 112 tasks mined from three local repos.
  - `oss-rust` is this repository and `priv-rust` is a private one that vendors it, so their
    tasks are not independent; `priv-py` is a private Python repo (14 tasks).
  - Only aggregates are committed. A fourth repo had 6 indexable files and was left out.

Measured token counts (one input at a time):

| chars | gemma tokens (Rust, local) | gemma tokens (public) | qwen3 tokens (public) | bge-small tokens |
|---:|---:|---:|---:|---:|
| 2,000 | 613 | 532 | 452 | 550–650 (cut at 512) |
| 4,000 | 1,187 | 1,050 | 914 | cut at 512 |
| 7,000 | 2,047 | 1,788 | | |
| 8,000 | 2,316 | 2,050 | 1,805 | |
| 16,000 | 4,076 | 4,090 | 3,513 | |

How many chunks are long (all symbol versions):
- **Public set:** 4.7% are over 2,000 chars (types 8.4%) and 0.8% over 7,000.
- **Gold symbols:** 42.7% are over 2,000 chars. Changed code sits in long functions more than
  in short ones.
- **Tasks:** 28 of 40 public tasks and 70 of 112 local ones have a gold symbol over 2,000 chars.

Means, all tasks (`mrr`, `r@8`, `r@20`, `r@40`; file@8 is in the reports):

| arm | gemma public | gemma local | qwen3 public |
|---|---|---|---|
| trunc:1000 | .398 .391 .581 .647 | .353 .340 .433 .550 | .379 .442 .594 .738 |
| **trunc:2000** (today) | .388 .383 .498 .681 | .407 .375 .534 .625 | .395 .465 .615 .770 |
| trunc:4000 | .403 .440 .651 .804 | .419 .392 .569 .667 | .405 .506 .675 .817 |
| trunc:8000 | .439 .415 .640 .807 | .417 .393 .584 .682 | .383 .425 .626 .763 |
| trunc:inf | .448 .428 .674 .829 | .418 .399 .591 .694 | .381 .430 .618 .755 |
| win:7000:500 | .403 .409 .671 .812 | .419 .401 .587 .694 | |
| winmean:7000:500 | .420 .425 .690 .800 | .418 .390 .587 .702 | |
| members:7000 | .411 .424 .637 .800 | .419 .394 .583 .681 | |

Paired against `trunc:2000` at 4,000 chars (mean difference, 95% bootstrap interval, tasks better / worse):

| | r@8 | r@20 | r@40 | mrr |
|---|---|---|---|---|
| gemma public | +.057 [−.006, +.141] 5/2 | +.153 [+.057, +.258] 13/2 | +.123 [+.034, +.226] 9/2 | +.014 [−.021, +.049] 11/7 |
| gemma local | +.017 [−.018, +.055] 9/5 | +.034 [+.004, +.068] 9/4 | +.042 [+.009, +.077] 13/3 | +.012 [−.019, +.044] 23/29 |
| qwen3 public | +.042 [−.008, +.109] 5/2 | +.061 [+.012, +.124] 7/1 | +.047 [−.007, +.113] 6/1 | +.010 [−.024, +.043] 12/5 |

Lexical only (`margincode-bench retrieval`, no semantic layer, public set, cap 2,000 vs 8,000
in the daemon):
- Cap 8,000 was equal or better at every budget.
  - `pack` at 800 tokens: `files_code` 0.69 → 0.74 (`pack_sig` 0.58 → 0.65).
  - `symbols_body` at 4,000 and 8,000 tokens: +0.05 and +0.04.
- Nothing got worse, so the longer text did not hurt BM25 here; 4,000 was not run.
- Reports: `lexical-cap2000.md`, `lexical-cap8000.md`.

Per-embedder reports with every arm, subset (long gold, no long gold, no leaky goal), per-repo
MRR and index cost: `gemma-public.md`, `gemma-local.md`, `qwen3-public.md`. Per-task rows
(public tasks only): `gemma-public-rows.jsonl`, `qwen3-public-rows.jsonl`.

Index cost (chars embedded, all symbol versions): 4,000 is 1.10× (public) and 1.14× (local)
today's, 8,000 is 1.18× and 1.22×, uncapped 1.41× and 1.34×. Window arms embed 1.32–1.34× the
chars and up to 16% more vectors.

## When the cap changes

`code_chunks.text` is written when a file is parsed. Unchanged files are not re-parsed, and
`av reindex` re-embeds the stored text. So an existing index keeps 2,000-char chunks until each
file changes. A cap change needs a CHANGELOG line saying how to rebuild the index (re-import the
repo), or a migration that re-parses.

## Not measured

- An end-to-end semantic pack run (`margincode-bench retrieval` with the semantic layer) at
  any cap, and lexical at 4,000.
- `embeddinggemma-2` with Ollama's default options. All gemma arms ran with `num_ctx` =
  `num_batch` = 32768. The defaults read 2,048 tokens, which falls between the measured 7,000
  and 8,000 arms; 4,000 is well under it.
- `qwen3-embedding` windows and members arms, and qwen3 on the local set.
- The full TEI arm table: stopped at 41 of 121 tasks on CPU. The cap question for TEI is
  answered by the identical vectors above.
- Other Ollama versions (this is 0.40.14), and the 0.6b and 4b qwen3 sizes.

## Method

`margincode-bench chunk-length` (new, `crates/margincode-bench/src/chunklen.rs`). Offline,
dense-only, no daemon:

1. For each retrieval task, the gold set as `margincode-bench retrieval` defines it (`gold`):
   the non-test files the fix modified and the innermost symbols whose lines it changed.
2. Every symbol at `base_rev` the index would embed: the files `margincode_index` parses (`.py`,
   `.ts`, `.tsx`, `.rs`, at most `MAX_FILE_BYTES`, not under `node_modules`/`.venv`/`target`/`.git`),
   one chunk per `(path, qname)` (the first, as the index keeps), text
   `margincode_index::chunk_source` = `{path}\n{qname}\n{signature}\n{body}`. The arm
   `trunc:2000` is byte for byte the chunk the index stores today (unit test
   `trunc_2000_is_the_index_chunk`).
3. The query is the goal, trimmed, no prefix, as `margincode-pack` sends it.
4. Every symbol is ranked by cosine similarity of its vector(s) to the goal's. Metrics per task:
   `mrr` (1 / rank of the best gold symbol), `r@k` (share of gold symbols in the first k; 8 is
   the pack's semantic code-hit limit at the agents' default budget, 40 its largest), `file@8`
   (one of the first 8 symbols is in a gold file).
5. Paired per-task differences against `trunc:2000`, with a 95% bootstrap interval and
   better / worse / tied counts. A task is "long gold" when one of its gold symbols' chunks is
   over 2,000 characters; only those tasks can change through the gold symbol's own vector (the
   others change only through the other symbols they compete with).

Arms:

| arm | what is embedded for a symbol |
|---|---|
| `trunc:N` | the chunk cut to N characters, ending with `…` (`inf`: no cap) |
| `win:N:O` | a chunk over N chars split at line boundaries into windows of at most N chars, each starting with the header (path, qname, signature) and repeating about O chars of the window before; the symbol scores its best window |
| `winmean:N:O` | the same windows, one vector per symbol: the normalized mean of the window vectors |
| `members:N` | a symbol that contains others (a class, a trait) keeps its own lines and only the first line of each contained symbol; then cut to N |

Vectors are cached by (embedder, model digest, options, text), so arms that share a text embed
it once. All `embeddinggemma-2` arms were embedded with `options: {num_ctx: 32768, num_batch:
32768}`, so the text is the only variable; below the default limit the vectors are the same as
without options (cosine 1.000000 at 300, 2,000 and 6,000 chars, single and batched).

Reproduce (tasks mined with `margincode-bench mine REPO --limit 400`; the public set is
`bench/retrieval/public.jsonl`, clones as in `bench/retrieval/README.md`):

```bash
margincode-bench chunk-length bench/retrieval/public.jsonl --repos ~/eval-repos \
  --ollama http://127.0.0.1:11434 --model embeddinggemma-2 --num-ctx 32768 \
  --arms trunc:500,trunc:1000,trunc:2000,trunc:4000,trunc:7000,trunc:16000,trunc:inf,win:2000:200 \
  --cache /tmp/gemma.bin --out /tmp/chunk-gemma
margincode-bench chunk-length bench/retrieval/public.jsonl --repos ~/eval-repos \
  --tei http://127.0.0.1:8080 --arms trunc:500,trunc:1000,trunc:1500,trunc:2000,win:1500:150 \
  --cache /tmp/tei.bin --out /tmp/chunk-tei
```
