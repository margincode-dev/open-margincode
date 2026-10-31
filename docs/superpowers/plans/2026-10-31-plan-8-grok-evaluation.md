# Plan 8 evaluation: is routing work to Grok worth it?

Companion to `2026-10-31-scholion-plan-8-routing.md`. Plan 6 sets the rules (paired designs,
pre-registration, strong baselines, honest overhead); this plan applies them to one question:
**when one person has Claude Code, Codex, Grok Build and Muse Code, does sending each task to the
agent that fits it beat the simple alternatives, and is Grok the right home for image work?**

"Grok is the best at images" is a belief, not a measurement. Nothing in this repository measures
Grok today: the `av init --for grok` config format is checked against fixtures only, never the real
CLI (`integrations/grok/README.md`). So the profile in the routing design (`grok --best image`) is a
placeholder until E-G1 sets it from data.

## Claims under test

| # | Claim | Evidence today |
|---|---|---|
| G0 | Grok Build works as an `av` agent: connects, starts and claims tasks, writes records, respects fencing | fixtures only |
| G1 | Per tag (image, ui, code, tests, docs, math), the agents differ enough to justify `best`/`can` | none |
| G2 | Tag routing beats one agent for everything, and beats random routing, on cost per accepted task | none |
| G3 | The reserve window keeps work moving when one agent's quota runs out | none (unit tests of the rules only) |
| G4 | A lesson recorded by one vendor's agent is honored by another's, with Grok in the chain | Claude to Codex demo only (`scripts/relay-demo.sh`) |

## Constraints

- **The API budget is a few dollars** (project memory). Claude Code, Codex, Grok Build and Muse Code
  run on the user's subscriptions, so a run costs quota and wall time, not API dollars. The scarce
  things are therefore **quota and the user's time**. Cheap sweeps use DeepSeek through the
  existing harness, as in E4.
- Each vendor reports usage differently. Report **tokens** per vendor from its own output
  (`grok usage <session>`, Claude's `result` event, and so on). Dollar figures are estimates from
  list prices, labelled as such. Never compare quota between vendors directly.
- A cell of the study is small (see E-G1). Claims are limited to large effects, with Wilson
  intervals shown. "No detectable difference" is a result, not a failure.

## Principles (from Plan 6)

- **Pre-register** in `bench/results/<date>-routing-grok/PREREG.md` before any run that costs quota:
  hypotheses, primary metric (tokens per accepted task), decision rules, stop rules.
- **Paired**: every task runs with every agent. McNemar on accepted, Wilcoxon on tokens, bootstrap
  over tasks for intervals.
- **Hidden tests decide acceptance**, never the agent's own report and never a judge model. Image
  tasks need deterministic checks too (below).
- **Count the routing overhead.** `start_task` without arguments and the `needs` parameter of
  `handoff` add tool-description text; `av next` adds a request. Report the tokens added to
  `tools/list` and the latency of a pick.
- **Strong baselines.** One agent for everything is the baseline users actually have. Random among
  capable agents is the control that tells whether tags matter at all.

## E-G0. Grok Build as an `av` agent (free, minutes)

Closes the "fixtures only" caveat and fixes the facts every later step relies on.

1. `av init --for grok`, then `grok mcp doctor av`. Pass: connects and lists the tools.
2. A scripted headless session: `grok -p "<prompt>" --output-format json --always-approve
   --max-turns 12 --cwd <task workspace>` that calls `start_task`, edits a file, `record`s a
   decision, `checkpoint`s, `validate`s and `submit`s. The daemon event feed is the oracle.
3. Record, without assuming, what the harness will need:
   - the agent name the daemon sees (`clientInfo.name` or an `X-Av-Agent` header): decides whether
     `aliases` in the profile are required;
   - whether Grok's MCP config supports a static header at all (the routing design writes
     `X-Av-Agent` where it does);
   - how Grok reports usage: `--output-format json` fields, `grok usage <session>`;
   - how an image reaches Grok headless (`--prompt-json` content blocks, or a path in the prompt).
     E-G1's image tasks depend on this;
   - what `--always-approve` lets it run, so the task workspace is the only writable place.
4. Fencing: start a task as Claude, hand it off, claim it as Grok, then make Claude write with the
   old generation. Pass: refused.

Output: `bench/results/<date>-routing-grok/conformance.md` with the observed values, and fixes to
`integrations/grok/README.md` where reality differs.

### What E-G0 found (Grok Build 1.0.70, 2026-10-31; `bench/results/2026-10-31-routing-grok/conformance.md`)

E-G0 was run; these facts replace the "record, without assuming" list above and shape the runner:

- **Name:** Grok sends `clientInfo.name` `grok-shell-av`. A static `X-Av-Agent` header works, and
  `av init --for grok` now writes it. Tools appear to the model as `av__<name>`.
- **Usage:** `--output-format json` gives `usage.input_tokens` (uncached only),
  `usage.cache_read_input_tokens`, `usage.output_tokens`, `usage.reasoning_tokens` and
  `total_cost_usd` (an estimate). `grok usage <session>` counts the cache in its input. There is no
  budget flag: cap with `--max-turns`.
- **Images:** `--prompt-json` takes ACP blocks `{"type":"image","mimeType":"image/png","data":"<base64>"}`;
  a path in the prompt also works. An 8x8 image was dropped, a 256x256 one read. **Image task assets
  must be at least 256x256.**
- **Workspace:** `--add-dir` is rejected headless, so the task workspace must sit under `--cwd`
  (`AV_WS_ROOT` inside it). `--sandbox workspace` did **not** stop a write outside the directory, so
  a headless Grok run can write anywhere this user can: run the benchmark as a throwaway user or in a
  disposable checkout, never next to work that matters.
- **Task memory:** a resumed process forgets its task; calls after a restart must pass `task_id`
  (now in `integrations/agents-instructions.md`).
- **Noise:** python writes `__pycache__` into `changed`; set `PYTHONDONTWRITEBYTECODE=1` for every
  agent in the benchmark.
- **Baseline cost:** one Grok session carried about 413k input tokens (387k cached) from this
  machine's already-enabled MCP servers and skills, for a two-line repo. **Run every agent in the
  study with a clean config** (an empty home for each CLI, only the `av` entry), or the per-task
  token comparison measures the user's setup and not the agent. The E-G0 conformance session cost an
  estimated $0.26; the whole of E-G0, about $0.46 (subscription estimate, no API money).
- **Still open from E-G0:** the claim step was checked with a direct MCP client, not with a second
  `grok -p` process.

## E-G1. Capability profile (the data behind `best` and `can`)

**Design.** Six tags: `image`, `ui`, `code`, `tests`, `docs`, `math`. Six tasks per tag (36), each a
small self-contained repo (the tallyho style: `bench/tasks-routing/`, built by a script, hidden
tests). Four agents: Grok, Claude Code, Codex, Muse Code. Three repetitions per task and agent to
see variance. That is 36 x 4 x 3 = 432 sessions, which is too much for a first run, so:

- **Pilot first**: one task per tag, one repetition, all four agents (24 sessions). It measures
  quota use per session and checks that the harness and the acceptance checks work. Stop and fix
  if more than 10% of sessions fail for harness reasons.
- **Main run** only if the pilot's quota use fits the budget set in PREREG (sessions per agent per
  day). Run in batches of the cheapest tags first; stop a batch when an agent's quota is gone and
  resume the next day. A cell is "agent x tag" with 18 trials at most.

**Image tasks** (the tag Grok is claimed to lead). Acceptance must not depend on a human or a
judge model:
- *Read a chart/table image into JSON*: `assets/q3.png` is generated by the task script from known
  numbers; hidden test compares the JSON to the gold values.
- *Reproduce a mockup*: given `assets/mockup.png`, build `index.html`/CSS. Hidden test renders with
  headless Chromium at a fixed size and passes when SSIM against the reference render is at least
  a threshold chosen in the pilot (so a blank page fails and a faithful copy passes), plus DOM
  assertions for required text and elements.
- *Fix a visible bug from a screenshot*: a screenshot shows a layout bug in a given page; hidden
  test asserts the computed layout (bounding boxes) after the fix.
- *Controls*: the same tasks with the image replaced by a text description. If Grok's lead is
  about vision and not general skill, it shows only on the image versions.

Non-image tags use the existing task format; `math` tasks are small algorithm/number-theory
functions with hidden property tests.

**Metrics per cell**: accepted rate (Wilson interval), tokens per accepted task, turns, wall time,
tool errors.

**Decision rule** (written in PREREG, applied mechanically by a new `scholion-bench routing-profile`
report): for a tag, an agent is `best` if its accepted rate is within 10 points of the highest
and no other such agent is cheaper in tokens per accepted task; it is `can` if its accepted rate is
at least half the highest; otherwise it is left out of that tag. The report prints the `av agent
set ...` commands it implies, which replaces the placeholder profiles. Ties and cells with fewer
than 10 trials are marked "inconclusive" and keep the user's own setting.

## E-G2. Does routing pay? (paired, over a mixed backlog)

**Design.** One backlog of 24 tasks (4 per tag), posted with their tags. Arms, each a full pass
over the same backlog from the same base commit:

| arm | how tasks are assigned |
|---|---|
| `solo-claude` | every task to Claude Code |
| `solo-grok` | every task to Grok Build |
| `random` | each task to a random agent that `can` do it |
| `routed` | `av next` with the profiles from E-G1 |

Each arm runs the agents the way the user would: one session per task, to completion or its turn
cap. Run with 3 parallel puller processes for the routed arm and sequentially for the solo arms, and
also report the routed arm run sequentially, so the benefit of parallelism is not mistaken for the
benefit of routing.

**Metrics**: accepted tasks out of 24; tokens per accepted task, in total and for the premium agent
only (tokens on the most expensive subscription are what the user wants to save); wall time to
finish the backlog; merge conflicts; `av next` latency and the tokens it adds.

**Hypotheses and rules**:
- H1: `routed` accepts at least as many tasks as `solo-claude` (within 2 tasks) while using at most
  70% of its premium-agent tokens. If not, say routing does not pay on this task mix.
- H2: `routed` beats `random` on accepted tasks (McNemar). If not, tags do not matter for this
  mix and the profiles add configuration for no gain.
- H3: routing overhead is under 60 tokens in `tools/list` and `av next` picks in under 200 ms.

## E-G3. Quota exhaustion (the reserve window)

**Design.** Same backlog as E-G2, `routed` arm. Mid-run, one agent stops pulling (a scripted
"quota out" after its k-th task, applied to Grok in one variant and Claude in another).

- With the reserve window (the default 600 s) versus with `reserve_secs = 0` and versus a design
  that pins items to their best agent forever (`--prefer` with no expiry, simulated).
- **Metrics**: items left unfinished at the end, and the delay of the items whose best agent
  disappeared.
- **H4**: with the reserve window every item that some remaining agent `can` do is finished;
  pinning forever strands items. This is mostly a correctness check of the reserve rules under
  real timing, so it can run on the scripted agents of `scripts/route-demo.sh` and costs nothing;
  one real-agent confirmation at the end.

## E-G4. Relay with Grok in the chain

Reuse E4 (`bench/tasks-relay`, the knowledge relay): ten tasks each needing a lesson an earlier
agent left. Chains: Claude to Grok, Grok to Codex, Claude to Grok to Codex. Arms `cold` (no
lessons, the control) and `av`. Metric as in E4: tasks where the lesson was honored. Expected from
E4: `cold` fails often, `av` mostly honors. The question here is only whether a vendor change in the
chain loses lessons. Use `--price-per-mtok` for models the harness cannot price.

## Work to build (in this order)

1. **Runner adapter.** `crates/scholion-bench/src/runner.rs` launches Claude Code only (`claude_bin`,
   Claude's stream-json parsing, `--max-budget-usd`). Add a small vendor adapter: command line,
   how the prompt and the MCP/hook wiring are given, how usage is read back into `events::Usage`.
   Start with Grok (`grok -p ... --output-format json --always-approve --max-turns N --cwd`),
   then Codex (`codex exec`) and Muse Code. A fake CLI script per vendor tests the parsing, as the
   existing fake `claude` does. Session turn caps replace `--max-budget-usd` for vendors without a
   budget flag.
2. **`bench/tasks-routing/`** and its build script: the 36 tasks of E-G1 (pilot subset first), image
   assets generated by the script from known data so the repo needs no binary files in git, and the
   Chromium render check for the SSIM tasks.
3. **`scholion-bench routing-profile`**: reads E-G1 results, prints the table, intervals and the
   `av agent set` commands per the decision rule above.
4. **`scholion-bench routing`**: runs the E-G2 arms over a backlog through the daemon's `av next`
   and reports the paired statistics (reuse `stats.rs`).
5. **PREREG.md** written and committed before the pilot.

Order of spending: E-G0 and E-G3's scripted run (free), then the E-G1 pilot, then decide on the
main run, E-G2, and last E-G4.

## What changes depending on the result

- Grok leads `image` clearly: keep `grok --best image` and mention it in the README with the
  measured numbers and the task mix.
- Grok shows no image lead: say so, set the profile from the data, and keep the routing feature
  (cost-based routing and the quota fallback may still pay). Do not keep a marketing claim the data
  does not support.
- H1 or H2 fails: routing is configuration without a measured gain on this mix. Report it in the
  README next to the E4 result, and weigh whether to keep the feature on by default (it adds no
  tokens unless used).
