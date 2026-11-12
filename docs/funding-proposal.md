# Margincode: Funding Proposal

2026-11-09 · Vinh Bui, Tri V Holdings Texas

Working copy: [Margincode — Funding Proposal](https://claude.ai/artifact/6j9zZLsGc67gx1bcyEePAC)
(the editable doc this file was exported from).

## The ask

Tri V Holdings Texas asks for compute and cloud credits, plus a benchmark grant, to move Margincode from
one founder's machine to a hosted rollout over 12 months. Every result it pays for is published on
an open schema.

Model API calls are cheap. The rollout is blocked by hosting and GPU time. A full four-CLI tracker
run cost $0.57 in API spend. Running the product's own hosted cell, its embedder and open-weight
models as tracker arms has no budget today.

| What the funding pays for | Known cost | Basis |
| --- | --- | --- |
| Weekly tracker, 4 agent CLIs on Harbor | $0.57 per run (measured 2026-11-04); $10 per run cap | openevaluations #8 |
| Model tiers at n = 30 per cell (DeepSeek Flash vs Pro, easy and hard) | about $5.90 at peak prices | openevaluations #13 (pilot $1.45) |
| Cross-vendor handoff (Claude Sonnet ↔ DeepSeek) on a harder task set | under $20 Claude + $1 DeepSeek per run | openevaluations #14 (first attempt $1.15) |
| E5-lite: paired eval against plain git + NOTES.md | about $25 (budget) | Plan 15 |
| Handoff benchmark at n ≥ 30 per cell, several models | not yet costed | |
| Hosted Margincode Cloud cell (daemon, runner, 2× Postgres, Qdrant, embedder) | not yet costed | runs locally today |
| GPU for the embedder and for open-weight tracker arms | not yet costed | |
| CI minutes for the private repo | not yet costed | Actions off since 2026-11-06 |

Open: the total amount to request per program. It needs the three uncosted hosting lines above.

## Why this is worth funding

Every dollar here buys a public, reproducible number, not a promise. The product already works, is
measured, and costs almost nothing per unit of work. What it lacks is hosting.

- **Measured, not claimed.** In a pre-registered run of 210 sessions (30 per cell), handing one
  agent's lessons to the next took acceptance from 18/30 to 30/30 and cut cost per accepted task
  from $0.0216 to $0.0125. With 500 unrelated records in memory, Margincode's cost per accepted task
  rose 14 % while a plain notes file's rose 88 %: Margincode was 40 % cheaper.
- **The neutral layer under every agent.** Margincode sits beneath Claude Code, Codex, Gemini and
  Grok at once, through MCP and hooks, without forking any of them. No single vendor will build a
  memory its competitors' agents can read.
- **Capital efficient by construction.** One founder shipped three releases in under two weeks.
  The product now builds itself: agents work the backlog through Margincode at $0.0122 per merged
  feature, with a human approving each merge.
- **Open by rule.** The evaluation code is Apache-2.0 and the data CC BY 4.0, with a
  pre-registration before every paid comparison. Funders can check every number in this proposal
  against the raw rows.

## The problem

Coding agents start every session from zero. Each one re-reads the repository and retries what the
last session already ruled out. A handoff between two agents carries the diff and none of the
reasoning.

- **Git records what changed, not why.** Nothing records the dead ends, so the next agent pays for
  them again.
- **Teams run several agents at once.** Developers pay for Claude Code, Codex, Gemini and others
  because each is good at something different. The agents cannot share what they learned about a
  codebase.
- **Nobody can see the cost.** A team running agents cannot tell what each merged change cost, who
  approved it, or why it was made.
- **Nobody measures agent CLIs in public.** Vendor numbers are self-reported. The one published
  handoff study (Handoff Debt, arXiv 2606.02875) released no code.

## The product

Margincode is version control for coding agents: one memory per repository that every agent reads
and writes. It ships as a free open-source core, a paid hosted edition, and an open evaluation that
measures both against other tools.

| | Margincode (Community) | Margincode Cloud (Commercial) | openevaluations |
| --- | --- | --- | --- |
| What it is | Version control for coding agents, local daemon + CLI | The same daemon, hosted for a team, plus controls | Open, vendor-neutral evaluation of agent CLIs |
| Licence | MIT, [public repo](https://github.com/vinhqbui/scholion) | Proprietary | Apache-2.0 code, CC BY 4.0 data, [public repo](https://github.com/vinhqbui/openevaluations) |
| State | Release 0.0.3 (2026-11-06) | Teams v0 run end to end once; no customer yet | Tracker built, run locally; handoff benchmark built |

**How Margincode works.**

1. Each task runs in its own isolated workspace.
2. Agents leave short typed records (intent, decision, dead end and why, note, handoff, review),
   anchored to files and symbols.
3. Every session starts from a context pack: the task, a code map, the decisions and dead ends
   touching those files, and the code to read. It is built inside a token budget,
   deterministically, with no LLM call, and stored as a receipt.
4. One backlog is shared across vendors. Claude Code starts a job, Codex picks it up from the
   brief, another agent reviews it.

It runs on the developer's machine, needs no account and never sees model keys. Install channels:
Claude Code plugin, shell script, cargo, Homebrew and a container image.

**What Margincode Cloud adds.** Personas (model, CLI, instructions, path scope, budget, review
policy), provider keys sealed with AES-GCM, budgets with alerts at 50/80/100 % and a hard hold, a
sandboxed runner, human gates with an audit trail, and reports such as cost per merged feature.
Stemma, an agent-native storage engine, is in build inside Cloud.

**Rules that keep the core honest.** Code flows from Community to Cloud only. Community is never
made worse to sell Cloud: no agent caps, seat checks or cost tracking in the free edition.

## Evidence so far

The strongest result: when one agent's lessons are handed to the next, every task is accepted, and
as memory grows Margincode's cost stays nearly flat while a notes file's almost doubles. Each number
below carries its sample size; nothing below 30 sessions per cell is called significant.

| Run (date) | Sample | Result | Source |
| --- | --- | --- | --- |
| E4 at scale (2026-11-10) | 210 sessions, DeepSeek Flash, 30 per cell, pre-registered | No handoff 18/30 at $0.0216 per accepted task; Margincode 30/30 at $0.0125. From 0 to 500 unrelated records, a notes file's cost per accepted task rose 87.7 % (the one significant difference), Margincode's 14.0 %; at 500 records Margincode is 39.9 % cheaper. $3.09 | margincode #140 |
| Model tiers, pilot (2026-11-10) | 40 trials, Claude Code on DeepSeek Flash vs Pro | Hard tier: Flash 12/12 at $0.017 per solved task, Pro 8/8 at $0.100; easy: Flash 13/14, Pro 4/6. Pro solved no more at 6–9× the cost; the hard tier needs harder tasks | openevaluations #13 |
| E4 relay, lessons handed on (2026-10-31) | 90 sessions, DeepSeek Flash, pre-registered | Cost per accepted task $0.012 cold → $0.006 with Margincode; accepted 16/30 → 30/30. Ties a hand-written NOTES.md (−2.5 %). Descriptive at this n. | [report](../bench/results/2026-10-31-relay-image) |
| E3, solo agent (2026-10-27) | 160 sessions, Claude Haiku | 12.9 % fewer turns, 95 % CI [−18.3, −7.0]; about 14 % less wall time | [report](../bench/results/2026-10-27-haiku) |
| E1, retrieval (2026-11-04/05) | 42 tasks on 6 repos | A 271M embedder (EmbeddingGemma 2) scores level with a 7.6B one, so it became the default | [results](../bench/retrieval/results) |
| E2, scale (2026-10-30) | tokio (814 files), django (2,939 files), 4-core machine | Context p50 94 ms and 268 ms. 8/8 agents merged on one repo; 32 agents 16/32; 100 agents 17/100 | [report](../bench/results/2026-10-30-e2-scale) |
| Tracker, 4 agent CLIs (2026-11-04) | 12 trials per CLI | 12/12 solved on every CLI, $0.57 total. The task set is too easy to separate them; a harder set is next | openevaluations #8 |
| Remote workspaces (2026-11-04) | measured median | Checkpoint from another host 31.5 ms vs 21 ms local (1.5×, under the 2× target) | margincode #101 |
| Dogfood: the product builds itself (2026-11-03) | 5 backlog items | 5/5 merged in 2 h, 0 reverts, 0 merge conflicts. Cross-vendor review caught 2 real defects | internal report |
| Teams v0, first hosted-style run (2026-11-04) | 2 features | $0.0122 per merged feature, backlog-to-merge median 496 s, human gate on each | internal note |

**What the numbers do not show yet.** In E4 at scale both the notes file and Margincode solved every
task, so it separates them on cost, not on solving; the task's own lesson reached Margincode's first
pack in only 6–12 of 30 sessions, a retrieval gap to close. The first cross-vendor handoff run
(Sonnet ↔ DeepSeek, openevaluations #14) produced no valid mid-task handoff: the tasks were solved
inside the cut, so it needs the harder task set. E2 shows the system breaks beyond 8 agents per
repo. Stemma's metrics are fixture tests, not production data. There are no external users yet.

## Business model

Open core: the Community edition is free and stays complete; teams pay for Margincode Cloud. No
prices are set yet, and the company is pre-revenue.

- **Free: Margincode Community.** The full local product under MIT. It drives adoption and is the
  layer every agent CLI plugs into.
- **First paid product: Margincode Cloud Teams.** A hosted board, roles, audit, reports and usage for
  teams that drive their own agents. Customers bring their own model keys, so Teams is priced per
  seat, not on tokens.
- **Later add-on: Autopilot.** The runner that launches agents for a customer, on the customer's
  own keys. It waits on a legal read of the model providers' terms and on the E5/E6 evaluations.
- **Buyer.** The engineering lead who runs coding agents on a team's repositories and needs to see
  what they cost and why they changed things.

Open: seat price and the first three design partners.

## Rollout plan and use of funds

The rollout follows six phases, and each one exits on a measured number. Two are done. The two
that turn Margincode into a hosted product with public proof wait on compute.

| Phase | What | Exit | Status |
| --- | --- | --- | --- |
| 0 · Name and ground truth | One brand, clean IP split, provider terms read | both PRs green | **Done** |
| 1 · The public number | E4 at scale, n ≥ 30 per arm; tracker weekly, 3+ CLIs | cost per solved task published | **First n = 30 result (E4)**; tracker at n = 30 waits on compute |
| 2 · Pull | Homebrew, cargo, image; PR bot; cold start from git | installs, stars per week | Shipped, measuring |
| 3 · Remote workspaces | Agent on a laptop, daemon on another host | 1.5× vs a 2× target, met | **Done** |
| 4 · Teams, hosted | Hosted cell, 3 design partners; one run done at $0.0122/feature | cost per merged feature from a real run | **Waits on compute** |
| 5 · Autopilot | Runner on customers' own keys | supervised design-partner run | Gated: needs a legal read and E5/E6 |

Phases 1 and 2 run side by side; phase 4 needs phase 3's remote workspaces for a hosted cell.

**What the funding unlocks.**

1. **Phase 1, first.** E4 at scale is done (margincode #140, $3.09). Next: a harder task set, then the
   model tiers at n = 30 per cell (about $5.90), the cross-vendor handoff on it, and the weekly
   tracker in CI. That gives the public cost-per-solved-task number across vendors and open models.
2. **Phase 4, next.** Move the hosted cell (now on one local machine, with Stemma about to start a
   two-week shadow run) to cloud hosting with a GPU embedder. Then onboard three design partners
   on Teams.
3. **Alongside.** Run the handoff benchmark at n ≥ 30 per cell and E5-lite (about $25), and
   publish every run.

Open: start dates for each phase.

## Team and company

One founder, Vinh Bui ([vinhqbui](https://github.com/vinhqbui)), builds and runs all three products
under Tri V Holdings Texas. The company is bootstrapped, with no outside investment.

Most of the code is written by coding agents (Claude Code, Grok and others) with the founder
steering, reviewing and approving every merge. That work runs through Margincode itself, so the
product's own history is its longest test. The first commit was 2026-10-25; three releases
followed in under two weeks.

Open before any form is filed:

- [ ] Country of incorporation and registration date of Tri V Holdings Texas
- [ ] Registered activity code (it must read as software, not consulting)
- [ ] Copyright of the repositories assigned or licensed to Tri V Holdings Texas

## Where to apply, in order

Apply to NVIDIA Inception first: it is free, takes bootstrapped companies, and opens the larger AWS
and Nebius credit routes. Terms below were read on the providers' pages on 2026-11-06; re-read them
on the day you apply.

| Order | Program | What a bootstrapped company can get | What it would pay for | Watch out |
| --- | --- | --- | --- | --- |
| 1 | [NVIDIA Inception](https://www.nvidia.com/en-us/startups/) | Free, no equity, rolling. Partner credits after acceptance, e.g. Nebius AI Lift up to $150,000 | GPU for the embedder and open-weight tracker arms | Excludes consulting firms; asks for a pitch deck and incorporation date |
| 2 | [AWS Activate](https://aws.amazon.com/activate/credits/) | Founders: $1,000, up to $5,000. Portfolio: up to $200,000 with an Org ID (Inception can supply one) | Hosted cell on EC2 + RDS Postgres, tracker and benchmark runners | Paid-tier AWS account; new to Activate |
| 3 | [Microsoft for Startups](https://learn.microsoft.com/en-us/startups/) | $200 at once, up to $5,000 after business checks, more with usage, cap $150,000 | GitHub Actions minutes, first-party Foundry models as tracker arms | Personal Microsoft account; no prior Azure; excludes consultancies |
| 4 | [Snorkel Open Benchmarks Grants](https://benchmarks.snorkel.ai/) | Compute credits, hosting, expert review; rolling | Handoff benchmark at n ≥ 30 per cell; weekly tracker in CI | Outputs must be open (they already are) |

Not open now: Claude for Startups (cohort full). Google's credit tiers above the starter tier need
venture funding.

Application texts for each program, a 12-slide deck outline and the Snorkel proposal are drafted in
a separate funding dossier kept outside this repository.
