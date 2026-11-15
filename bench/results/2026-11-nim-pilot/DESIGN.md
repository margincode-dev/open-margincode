# E4 on NVIDIA-hosted DeepSeek: pilot design

Status: **draft, not run.** Nothing has been spent. The decision rules below are written before
any session so the pilot cannot be read to suit its result; they are frozen when this file is
committed, and any change after that is a dated amendment.

## Why

Every relay result so far (lean E4, E4 at scale, the wrong-lesson runs) used one model on one
host: `deepseek-flash` on `api.deepseek.com`. The provider can change what that alias means without
notice, and `meta.json` records only the alias, so the claims rest on one unpinned endpoint.

NVIDIA's API catalog (`integrate.api.nvidia.com`, "NIM") serves DeepSeek V4 Flash as a **dated
snapshot**, `deepseek-ai/deepseek-v4-flash-0731`, on NVIDIA's own infrastructure. If the E4
finding reproduces there, the relay suite gets a second, independently hosted, pinned provider,
and the finding stops depending on one API.

## Questions

1. **Plumbing.** Can Claude Code drive a NIM-hosted model through a translating proxy and run E4
   sessions as cleanly as it does against `api.deepseek.com`? The proxy is the risk: a tool call
   lost in translation looks exactly like an agent that ignored its memory.
2. **Replication.** Does E4 at scale's main result hold on the NIM host? At N = 500 irrelevant
   notes, `notes` sessions read a 27k-token `NOTES.md` and av's did not: median input tokens
   (cache reads included) 439,830 for `notes` against 228,843 for `av`, a 48 % gap, and cost per
   accepted task 39.9 % lower for av (`../2026-11-e4-scale/README.md`). That gap comes from how much
   text the agent reads before editing, so it should not depend much on which host serves the model.

Not asked: whether absolute accept rates or token counts match E4. The snapshot on NIM (0731) may
not be what `deepseek-flash` pointed to on 2026-11-10 (DeepSeek released V4.1 Flash on
2026-10-17), and the harness has moved on since E4 (Claude Code 2.1.318 and av `fbc5bb0` then; both
are recorded for this run). The av-against-`notes` comparison inside the pilot is self-contained;
its comparison with E4's magnitudes is not, and is reported as context only.

## Setup

```
harness container ── claude -p --model deepseek-flash ──► LiteLLM proxy (:4000, /v1/messages)
                                                             │  Anthropic Messages → OpenAI chat
                                                             ▼
                                         integrate.api.nvidia.com/v1  (deepseek-v4-flash-0731)
```

- **Proxy: LiteLLM**, its Anthropic-format `/v1/messages` endpoint with the `nvidia_nim/` provider.
  It runs as one more container on the `e4-bench` network, image pinned to an exact version
  (recorded in the README). The model name it serves is `deepseek-flash`, so **the task files and
  the harness do not change**: the same `ANTHROPIC_DEFAULT_HAIKU_MODEL` / `ANTHROPIC_SMALL_FAST_MODEL`
  / `CLAUDE_CODE_SUBAGENT_MODEL` exports as in `../../tasks-relay/README.md`, only
  `ANTHROPIC_BASE_URL` points at the proxy.

  ```yaml
  # litellm-config.yaml (field names checked against the pinned version's docs before S0)
  model_list:
    - model_name: deepseek-flash
      litellm_params:
        model: nvidia_nim/deepseek-ai/deepseek-v4-flash-0731
        api_key: os.environ/NVIDIA_NIM_API_KEY
        rpm: 35                # NVIDIA's free tier is about 40 requests/min per account
  litellm_settings:
    drop_params: true          # params NIM rejects are dropped, not sent
  general_settings:
    master_key: os.environ/LITELLM_MASTER_KEY
  ```

- **Keys.** The NVIDIA key lives in `~/.config/nvidia/key` (chmod 600) and reaches only the proxy
  container, by environment variable name. The harness authenticates to the proxy with
  `ANTHROPIC_AUTH_TOKEN=$LITELLM_MASTER_KEY`, a random local value. Neither key is written to a
  task file, a command line or the results.
- **Rate limit.** `margincode-bench run` runs sessions one at a time, but one session can exceed
  40 requests a minute at its peak. Whether LiteLLM's `rpm` queues those requests or answers 429
  (leaving Claude Code to retry) is checked in S0. A 429 that Claude Code retried successfully is
  not a proxy error under the rules below; one that ends a session is.
- **Cache accounting.** E4's token measures exclude cache reads, and its prices charge them at
  $0.006/M. NIM's OpenAI-format usage, passed through LiteLLM, probably reports no cache reads, so
  every re-sent prompt token may count as fresh input. S0 reads the `result` event's usage and
  records whether `cache_read_input_tokens` is ever non-zero. Either way, the replication rule uses
  **input tokens with cache reads included**, which means the same thing on both hosts.
- **Prices.** NIM bills in credits, not dollars. `--price-per-mtok 0.30,0.30,0.006,1.20` (DeepSeek's
  peak prices, as in E4) stays, so dollar columns are a **token-weighted cost**, not money spent.
  Without cache reads a `notes` session at N = 500 prices at about $0.13 (440k × $0.30/M), so the
  caps below assume nothing is cached. The real spend is counted as proxy requests per session, from
  the proxy log.
- **Record what `meta.json` cannot.** `meta.json` will say `deepseek-flash`. The README records the
  base URL, the NIM model id, the LiteLLM version, whether cache reads were reported, and the proxy
  log's error lines.

## Budget

NVIDIA's developer tier grants about 1,000 credits, which appear to be one per request
(community-reported, to be checked on the account page). The Inception portal may grant more:
check it before S1.

| stage | what | sessions | requests (at ~25 each) |
|---|---|---|---|
| S0 | plumbing at S1's conditions: `relay-customer-region`, arms `notes,av`, `--noise 500`, 1 rep | 2 | ~50 |
| S1 | replication: all 10 tasks, arms `notes,av`, `--noise 500`, 1 rep | 20 | ~500 |
| S2 | control: all 10 tasks, arm `cold`, 1 rep | 10 | ~250 |

About 800 requests in all. S0 measures the real requests per session. If it is above 30, S2 is
dropped (it is a control for task difficulty, not part of the replication question), or more
credits are requested first.

```bash
P="--price-per-mtok 0.30,0.30,0.006,1.20"; OUT=bench/results/2026-11-nim-pilot
margincode-bench run bench/tasks-relay --only relay-customer-region --arms notes,av --noise 500 \
  --jit-hooks --reps 1 $P --session-cap-usd 0.20 --budget-usd 0.50 --out $OUT/s0 $COMMON --confirm
margincode-bench run bench/tasks-relay --arms notes,av --noise 500 \
  --jit-hooks --reps 1 $P --session-cap-usd 0.20 --budget-usd 4.00 --out $OUT/s1 $COMMON --confirm
margincode-bench run bench/tasks-relay --arms cold \
  --jit-hooks --reps 1 $P --session-cap-usd 0.20 --budget-usd 2.00 --out $OUT/s2 $COMMON --confirm
```

The dollar caps are nominal (see Prices); what limits the run is the credit balance.

## Decision rules

**S0 passes** when both sessions end `status=ok` (not truncated, not an error) and each made at
least one edit; the `notes` session's `NOTES.md` has 510 sections; the `av` session has a non-empty
`hook-count` and a `pack.md`; and the proxy log has no 4xx/5xx other than retried 429s. If S0
fails, the pilot stops there and reports what broke. A proxy fault is a plumbing result, not a
finding about av.

**Plumbing holds** when at least 19 of S1's 20 sessions end `status=ok` and no session's failure
traces to the proxy (dropped tool call, rejected parameter, malformed tool input).

**Replication** compares `av` with `notes` in S1, task by task (each task ran once in each arm).
A task whose `notes` or `av` session is not `status=ok` or made no edit counts as a task where av
is **not** lower. The primary measure is input tokens with cache reads included:

- **holds** if `av`'s input tokens are lower than `notes`' on at least 8 of the 10 tasks (one-sided
  sign test p = 0.055) **and** the median of the per-task relative gaps is at least 30 % (E4 at
  scale: 48 %);
- **fails** if `av` is lower on 5 or fewer tasks, or `av`'s cost per accepted task is not below
  `notes`';
- is **inconclusive** otherwise, and is reported as such.

Tokens to first edit and cost per accepted task are reported alongside. If S0 shows cache reads
are not reported, tokens to first edit counts every re-sent prompt, which E4's did not, and the
README says so next to it.

Accepted tasks are reported per arm with their Wilson intervals and no claim, because E4 hit a
ceiling (30/30 in both arms). S2 is reported only: if `cold` solves fewer than `notes` and `av`,
the tasks still separate memory from no memory on this host (E4: 18/30).

## Stopping rules

- Proxy errors in more than 2 sessions of a stage: stop the stage, report it as partial.
- An authorization error mid-run (credits exhausted): stop, report what ran, never extrapolate.
- No stage starts until the previous one's report has been read.

## What goes out

Only the tallyho repository (public, written for the benchmark), the relay notes and the
generated noise go to NVIDIA. No private repository, Cloud code or user data is sent. The pilot
runs under NVIDIA's API catalog terms.

## Before S0

- [ ] An NVIDIA developer account and API key (build.nvidia.com; it asks for phone verification),
      created by a maintainer.
- [ ] The credit balance and any Inception credits checked.
- [ ] `deepseek-ai/deepseek-v4-flash-0731` still listed in the catalog; if it was replaced, the
      replacement's id goes into an amendment here before S0.
- [ ] The LiteLLM image pulled at a pinned version and its config checked against that version's
      docs.
- [ ] The harness's `rows.jsonl` carries per-session input tokens with cache reads (the replication
      rule needs them per task, not only the report's median).
- [ ] Each command above run without `--confirm`, its plan read.
