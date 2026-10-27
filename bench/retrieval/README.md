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
- **Leaky goals:** commit messages often name the code they change (a `leak`). The report shows
  those tasks separately, since they are easier for every method.
- **Limits:**
  - A commit message is a thinner goal than a real issue.
  - The gold set is what one fix touched, not everything relevant.
  - The text measures what an agent is shown, not whether it would succeed. E3 measures that.
