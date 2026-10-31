# Plan 9: Close the Loop — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Agents that loop on "take my next job" carry every task from backlog to merged. They also
pick up handoffs and abandoned work, wait for new work instead of quitting, and need no task ids
and no human prompting between steps.

**Architecture:**
- `POST /v1/backlog/next` (MCP `start_task` with no title/goal) offers every kind of waiting work:
  1. first, finishing work: reviews and merges of other agents' tasks;
  2. then rescue work: any handoff, tagged or not, and open tasks whose owner went quiet (lease
     expired);
  3. then the backlog.
- An optional `wait` holds the call open until work appears.
- `next` with `dry_run` peeks without taking anything. `av watch` uses it to start an agent CLI
  only when work fits that agent.
- Task ids may be given as slugs.
- Review and handoff bundles explain that workspace edits are snapshotted into commits.

No migration and no new MCP tool.

**As implemented (2026-10-31; where the code differs from the tasks below, the code wins):**
- `abandoned_tasks(repo, lease_secs)` takes the lease length and returns `routing::Stale { task,
  since }` (the time the lease ran out), not `now`.
- An owner that asks for its own abandoned task resumes it (`flow::resume_as`: lease renewed, same
  generation) instead of claiming it from itself.
- Two fallbacks the plan lacked, found by the tests: a review whose tags only its owner has goes to
  any other agent, and so does an untagged handoff whose inherited tags only the agent that handed
  it off has. Otherwise such tasks stay in review or handed off for good. Explicit handoff `needs`
  stay strict.
- `MAX_WAIT_SECS` is 50, a guess meant to stay under a 60 s client timeout. Task 7's measurement
  of the real CLIs' MCP timeouts was not run, so no client's timeout is known; one that times out
  sooner sees a failed call.
- The test that a stale generation is refused after a rescue lives in `tests/routing.rs`, beside
  the other rescue tests.
- `tools/list` grew by 112 bytes (10,200 to 10,312).

**Tech Stack:** Rust 1.97.1 (pinned), axum daemon, Postgres (`PgStore`), hand-rolled MCP in
`mcp.rs`, clap CLI.

**Spec:** this plan. The design is the Architecture paragraph above plus the "Rules" section
below. It comes from the pantry smoke test of 2026-10-31 (`examples/routing-smoke/`), where four
problems showed up:
- review and merge stalled until a human named task ids;
- Claude read `git status` in Grok's workspaces and concluded they were untouched;
- untagged handoffs and abandoned open tasks are never offered;
- an agent whose queue is empty stops, so later work waits for a human to restart it.

**Base branch:** `plan-8-routing`. This plan needs its routing code. First commit the work that is
still uncommitted there:
- `examples/routing-smoke/`;
- the `repos` table (migration 0023);
- the dashboard repo-list fix.

Then branch `plan-9-review-loop` from it.

## Rules (the design decisions)

1. **The order of `next`.** For the calling agent A (its name resolved through profile aliases,
   as in Plan 8), `next` returns the first of these that exists:
   1. **review/merge**: the oldest (by task `created_at`) `in_review` task that A does not own,
      whose tags A can do, and that no live reservation gives to another agent;
   2. **rescue**: a task in either of these states:
      - `handed_off` and not owned by A (any handoff, tagged or not);
      - `open` with `lease_expires < now`. Its owner may be A: an agent that restarted gets its
        own abandoned task back.

      Rescue tasks are ranked together with the backlog by Plan 8's `routing::order_for`. That
      means reserve windows, `prefer` and tags. The window starts at the handoff time for a
      handoff, and at `lease_expires` for an abandoned task;
   3. **backlog**: queued items, as in Plan 8.

   Rescue and backlog are one `order_for` call, and Plan 8's rule that the agent's own top-ranked
   items come first still decides between them.
2. **Tags of an existing task.**
   - A task made from a backlog item carries that item's `needs`.
   - A handoff that set `needs` uses those instead.
   - Otherwise the task has no tags, and every agent fits it.
   - "Can do" is `Profile::can_do`. An agent with no profile fits only untagged work.
3. **Review reservation.** A review/merge offer reserves the task for A for
   `REVIEW_RESERVE_SECS = 600`.
   - It lives in a `Mutex<HashMap<TaskId, (String, Instant)>>` in `AppState`. A restart forgets it,
     which only means a review may be offered twice.
   - A asking again gets the same task back while the reservation lasts.
   - A `dry_run` peek never reserves.
4. **What a review offer says.** The response has these fields:
   - `"from": "review"`;
   - the `task`;
   - `"approved": bool` (a verdict `approve` at the current head);
   - `"next"`: a one-line instruction (exact strings in Task 2).

   It does not include the bundle, so an offer costs a few tokens.

   Rescue offers return what `claim_task` returns, with `"from": "handoff"` (any handed-off task)
   or `"from": "abandoned"` (an expired-lease open task).
5. **Reviewers may validate.** `validate` is fenced only by claim generation, not by owner, so a
   reviewer can run the tests in the owner's workspace and attest them. No code change; only
   wording (Task 5).
6. **Slugs.** A `task_id` argument that is not a UUID is resolved:
   - as `repo/slug` when it contains `/`;
   - otherwise as a slug across all repos.

   It is an error when none matches. When several match, the error lists them as `repo/slug`.
7. **Waiting.** `next` takes `wait_secs` (MCP `start_task` takes `wait`), clamped to
   `MAX_WAIT_SECS`, whose value is set by Task 7's measurement. With nothing to offer, it rechecks
   every 2 s until something fits or the time runs out, then returns `{"task": null, …}` as today.
   It stops early when the client disconnects.
8. **Peek.** `next` with `"dry_run": true` returns either of these, and takes, claims and reserves
   nothing:
   - `{"would": {"from", "task_id" | "backlog_id", "repo", "title"}}`;
   - `{"would": null, "queued": N}`.

   It is not exposed over MCP.
9. **`av watch`.** A client-side loop on the user's machine; the daemon still launches nothing.
   - It peeks as `--agent`.
   - When something fits, it runs the given command with `AV_AGENT` set and waits for it to exit.
   - It never runs the command while the peek is empty.
10. **Token rule.**
    - The tool list grows by at most 220 bytes over today's 10,200 (about 55 tokens), measured by
      the guard in `tests/mcp.rs`. Raise its cap only to the measured size.
    - The only tool-list changes are `start_task`'s description and its `wait` property.
    - Slugs, peek and the workspace note add nothing to the tool list.

## Global Constraints

- TDD per task: the failing test first. Before each commit:
  `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`.
- Agent-facing text says `av`, never `scholion`.
- Do not change `ext.rs` signatures (the commercial edition builds on them).
- Do not edit merged migrations. This plan adds none.
- The daemon never starts processes. Only `av watch`, a CLI command, does.

## Review Focus

1. **The owner of every in-review task asks `next`.** It must never get its own review. Test in
   Task 2: `next_never_offers_an_agent_its_own_review`.
2. **Two agents ask at the same moment.** Exactly one gets the review. Test in Task 2:
   `two_agents_are_not_offered_the_same_review`.
3. **A slow but live owner.** An agent thinking for a long time without writing loses its lease
   after `AV_LEASE_SECS` (1800 s by default), and another agent rescues the task. The old owner's
   next write must then be refused with "claim lost", and must not corrupt the workspace. Test in
   Task 3: `a_rescued_task_refuses_the_old_owners_writes`.
4. **`wait` above the maximum, or zero.** It is clamped, and zero means no wait. It never hangs
   past `MAX_WAIT_SECS`. Test in Task 7: `wait_is_clamped_and_zero_returns_at_once`.
5. **`av watch` when the command fails or the daemon is down.** It must not spin: back off,
   print why, and stop after `--max-runs`. Test in Task 8:
   `watch_stops_after_max_runs_and_backs_off_on_failure`.

---

### Task 1: Store lookups

**Files:**
- Modify: `crates/scholion-core/src/traits.rs` (next to `task_by_slug` and
  `handed_off_with_needs`)
- Modify: `crates/scholion-store/src/pg.rs`
- Test: `crates/scholion-store/tests/store.rs`, `crates/scholion-store/tests/routing.rs`

**Interfaces:**
- Produces:
  - `IntentStore::tasks_with_slug(&self, slug: &str) -> Result<Vec<Task>>`, any status, ordered
    by repo;
  - `IntentStore::handed_off_with_needs` becomes `handed_off_tasks(&self, repo: Option<&str>)
    -> Result<Vec<HandedOff>>`. It is the same row type, but it has no `cardinality(needs) > 0 OR
    prefer IS NOT NULL` filter, so it covers every handoff. Rename the callers;
  - `IntentStore::abandoned_tasks(&self, repo: Option<&str>, now: DateTime<Utc>) ->
    Result<Vec<Task>>`: `status = 'open' AND lease_expires < now`, oldest `lease_expires` first.

- [ ] **Step 1: Write the failing tests:**
  - `tasks_with_slug_finds_it_in_every_repo`: "Fix login" in repos `a` and `b` gives two tasks
    with repos `["a", "b"]`; `"nope"` gives none.
  - `handed_off_tasks_include_untagged_handoffs`: one handoff with `needs [image]` and one with
    none; both are returned.
  - `abandoned_tasks_are_open_tasks_past_their_lease`: one open task with an expired lease (set
    with a short `claim_task` lease, or by updating `lease_expires`), one with a live lease, and
    one in review. Only the first is returned.
- [ ] **Step 2: Run** `cargo test -p scholion-store`. Expected: these fail (missing methods).
- [ ] **Step 3: Implement** the three queries, reusing the existing row-to-`Task` and `HandedOff`
  mappings.
- [ ] **Step 4: Run** `cargo test -p scholion-store` and `cargo test -p scholion-daemon --test routing`.
  Expected: PASS. The Plan 8 routing tests still pass after the rename.
- [ ] **Step 5: Commit**: `store: slug lookup across repos, all handoffs, abandoned open tasks`.

### Task 2: `next` offers reviews and merges first

**Files:**
- Modify: `crates/scholion-daemon/src/routing_api.rs` (`next`, plus a new `review_offer`)
- Modify: `crates/scholion-daemon/src/lib.rs` (`AppState` field `reviews`)
- Test: `crates/scholion-daemon/tests/routing.rs`

**Interfaces:**
- Consumes:
  - `st.pg.list_tasks(repo, Some(TaskStatus::InReview), MAX_TASKS)`;
  - `st.store.list_backlog(repo, Some(BacklogState::Taken))`, giving a task's `needs` through
    `task_id`;
  - `st.store.latest_verdict(task, &head)`;
  - `routing::resolve` and `Profile::can_do`.
- Produces:
  - `pub const REVIEW_RESERVE_SECS: u64 = 600`;
  - `AppState.reviews: Arc<Mutex<HashMap<TaskId, (String, std::time::Instant)>>>`;
  - `async fn task_needs(st: &AppState, task: &Task) -> Result<Vec<String>>` (Rule 2; Task 3
    reuses it);
  - `async fn review_offer(st: &AppState, agent: &str, repo: Option<&str>, profiles: &[Profile], reserve: bool) -> Result<Option<Value>>`.

The `next` strings, exact:
- not approved: `"Review it: review_task, run its tests in its workspace and validate, then post_review. If you approve, merge it."`
- approved: `"It is approved: merge it."`

The tests use the profiles of `tests/routing.rs` (claude best code; grok best image, can code;
codex can code):

- [ ] **Step 1: Write the failing tests:**
  - `next_offers_another_agents_task_in_review_before_backlog_work`: claude submits a `code`
    task, and an `image` item is queued. Grok's `next` returns `from == "review"` with claude's
    task and `approved == false`.
  - `next_never_offers_an_agent_its_own_review`.
  - `a_review_needs_the_tags_of_the_item`: grok submits an `image` task. Claude (no `image`) is
    not offered it. After `PUT /v1/agents/claude` adds `image` to `can`, claude is offered it.
  - `two_agents_are_not_offered_the_same_review`: codex then grok ask; grok does not get the task,
    and codex's second `next` gets it again.
  - `an_approved_task_is_offered_as_merge`.
  - `a_resubmitted_task_is_offered_for_review_again_not_as_approved`.
- [ ] **Step 2: Run** `cargo test -p scholion-daemon --test routing`. Expected: the new tests fail.
- [ ] **Step 3: Implement** `review_offer` per Rules 1.1, 3 and 4, and call it first in `next`.
  Prune expired reservations and insert the caller's under one lock.
- [ ] **Step 4: Run** `cargo test -p scholion-daemon --test routing --test swarm`. Expected:
  PASS. If a Plan 8 swarm test now receives reviews, its pullers must handle `from == "review"`.
- [ ] **Step 5: Commit**: `next offers reviews and merges before new work, reserved per agent`.

### Task 3: `next` rescues handoffs and abandoned tasks

**Files:**
- Modify: `crates/scholion-daemon/src/routing_api.rs` (`next`: build rescue items next to the
  handoff items)
- Test: `crates/scholion-daemon/tests/routing.rs`

**Interfaces:**
- Consumes: `handed_off_tasks` and `abandoned_tasks` (Task 1), `task_needs` (Task 2), and
  `flow::claim_as`.
- Produces: rescue offers that return the `claim_as` value with `"from": "handoff"` or
  `"from": "abandoned"`.

Each rescue task becomes a `BacklogItem` for `order_for`:
- `needs` by Rule 2;
- `prefer` from the handoff;
- `created_at` = the handoff time, or `lease_expires`.

Handed-off tasks owned by the caller are left out before ranking. Abandoned tasks are not.

- [ ] **Step 1: Write the failing tests:**
  - `an_untagged_handoff_is_offered_to_another_agent`: claude hands off with no `needs`. Grok's
    `next` claims it (`from == "handoff"`, generation 2). Claude's own `next` does not get it.
  - `an_abandoned_task_is_offered_after_its_lease`: use `env_with(|s| s.with_lease_secs(1))`.
    Claude starts a task, then 2 s pass. Grok's `next` returns `from == "abandoned"`, with grok
    now the owner.
  - `an_abandoned_task_goes_back_to_its_restarted_owner`: the same setup, but claude asks first.
    Claude gets it back.
  - `a_rescued_task_refuses_the_old_owners_writes`: after grok rescues it, claude's `checkpoint`
    with its old generation header returns 422 with "claim lost".
- [ ] **Step 2: Run** `cargo test -p scholion-daemon --test routing`. Expected: the new tests fail.
- [ ] **Step 3: Implement.** Map `IllegalState` from `claim_as` (someone else got it) to "try the
  next item", as Plan 8 already does.
- [ ] **Step 4: Run** `cargo test -p scholion-daemon --test routing --test swarm`. Expected: PASS.
- [ ] **Step 5: Commit**: `next offers every handoff and tasks whose owner went quiet`.

### Task 4: Slugs wherever MCP takes a task id

**Files:**
- Modify: `crates/scholion-daemon/src/mcp.rs`:
  - `target()`;
  - `claim_task`, `review_task` and `post_review`;
  - each place that formats `str_arg(args, "task_id")` into a URI.
- Test: `crates/scholion-daemon/tests/mcp.rs`

**Interfaces:**
- Consumes: `tasks_with_slug` (Task 1) and `task_by_slug`.
- Produces: `async fn task_ref(st: &AppState, s: &str) -> Result<String, RpcError>`, returning the
  UUID string by Rule 6. A UUID passes through without a query.

- [ ] **Step 1: Write the failing tests:**
  - `status_takes_a_slug`;
  - `review_task_takes_repo_slash_slug`;
  - `an_ambiguous_slug_names_every_match`: the error contains both `demo/<slug>` and
    `other/<slug>`;
  - `a_merged_task_resolves_by_slug`;
  - `an_unknown_slug_says_so`: the error contains `no task` and the slug.
- [ ] **Step 2: Run** `cargo test -p scholion-daemon --test mcp slug`. Expected: FAIL.
- [ ] **Step 3: Implement** `task_ref`, and route every `task_id` through it.
- [ ] **Step 4: Run** `cargo test -p scholion-daemon --test mcp`. Expected: PASS, with the size
  guard unchanged.
- [ ] **Step 5: Commit**: `mcp: a task_id may be a slug or repo/slug`.

### Task 5: Bundles explain workspaces and say reviewers validate

**Files:**
- Modify: `crates/scholion-pack/src/bundle.rs` (the `Workspace:` line near 129, and the
  "Tests: not run" line near 330)
- Modify: the golden files under `crates/scholion-pack/tests/` that contain them
- Test: `crates/scholion-pack/tests/bundle.rs`

Exact new text:
- after `Workspace: {path}`, a new line:
  `(edits there are snapshotted into commits, so git status shows nothing; the changes are listed below)`
- the tests line becomes:
  `Tests: not run at this head ({head}); run them in the workspace, then av validate -- <cmd>`

- [ ] **Step 1: Update the assertions** in `tests/bundle.rs` for both the handoff and the PR
  bundle.
- [ ] **Step 2: Run** `cargo test -p scholion-pack --test bundle`. Expected: FAIL.
- [ ] **Step 3: Change the two format strings** and regenerate the golden output. The diff must
  contain only these lines.
- [ ] **Step 4: Run** `cargo test -p scholion-pack` and
  `cargo test -p scholion-daemon --test api bundle`. Expected: PASS.
- [ ] **Step 5: Commit**: `bundles: explain snapshotted workspaces; reviewers validate`.

### Task 6: `dry_run` peek

**Files:**
- Modify: `crates/scholion-daemon/src/routing_api.rs` (`NextBody.dry_run: bool`, default false)
- Test: `crates/scholion-daemon/tests/routing.rs`

**Interfaces:**
- Produces: the Rule 8 response. It is computed by the same selection as `next`; factor the
  selection into `async fn choose(st, agent, repo, profiles) -> Result<Option<Choice>>`, which both
  the peek and the real call use, so they cannot disagree. Add
  `enum Choice { Review(Task, bool), Rescue(Task, &'static str), Backlog(BacklogItem) }`.

- [ ] **Step 1: Write the failing tests:**
  - `a_peek_takes_nothing`: peek, then check that `GET /v1/backlog` still shows the item
    `queued`, and that a second agent can still take it.
  - `a_peek_names_what_next_would_return`: for one review case, one handoff case and one backlog
    case, the peek's `from` and id equal those of the following real `next`.
  - `a_peek_does_not_reserve_a_review`.
- [ ] **Step 2: Run** `cargo test -p scholion-daemon --test routing peek`. Expected: FAIL.
- [ ] **Step 3: Implement** `choose`. `next` becomes `choose`, then act on the result. Keep the
  try-the-next-candidate behavior of Plan 8 when the action fails with `IllegalState`.
- [ ] **Step 4: Run** `cargo test -p scholion-daemon --test routing --test swarm`. Expected: PASS.
- [ ] **Step 5: Commit**: `next: dry_run peek shares the selection with next`.

### Task 7: `wait`

**Files:**
- Modify: `crates/scholion-daemon/src/routing_api.rs` (`NextBody.wait_secs: Option<u64>`)
- Modify: `crates/scholion-daemon/src/mcp.rs` (`start_task` gets a `wait` property; pass it as
  `wait_secs`)
- Modify: `crates/scholion-cli/src/main.rs` (`av next --wait SECS`)
- Test: `crates/scholion-daemon/tests/routing.rs`, `crates/scholion-daemon/tests/mcp.rs`

- [ ] **Step 1: Measure the clients' MCP call timeouts.** Use a throwaway daemon route or a test
  build where a tool sleeps for 30, 60, 120 and 300 s. Call it from real `claude -p` and
  `grok -p` sessions; this spends subscription quota, not API money.
  - Record the longest call each client completes, and the setting that changes it (Claude Code:
    `MCP_TOOL_TIMEOUT`).
  - Write the findings in `integrations/agents-instructions.md` under a "Waiting" note.
  - Set `pub const MAX_WAIT_SECS: u64` to 80% of the smaller default, never above 300.
- [ ] **Step 2: Write the failing tests:**
  - `next_waits_for_work_that_arrives`: grok calls `next` with `wait_secs: 10` in a spawned task.
    After 1 s an image item is posted. The call returns that item in under 5 s.
  - `wait_is_clamped_and_zero_returns_at_once`: `wait_secs: 0` returns `task: null` at once (under
    0.5 s). `wait_secs: 100000` returns within `MAX_WAIT_SECS + 3` s. To keep the test fast, run
    it with `MAX_WAIT_SECS` overridden by a test-only `AppState::with_max_wait(Duration)`.
  - `start_task_passes_wait`, in `tests/mcp.rs`.
- [ ] **Step 3: Run** them. Expected: FAIL.
- [ ] **Step 4: Implement** a loop of `choose`, then `tokio::time::sleep(2 s)`, until the deadline.
  A `dry_run` call never waits.
- [ ] **Step 5: Run** `cargo test -p scholion-daemon`. Expected: PASS, and the size guard fails
  only by the new `wait` property and description. Leave it failing for Task 9, which sets the cap
  once.
- [ ] **Step 6: Commit** (with the guard cap raised temporarily by the measured amount, noted in
  the commit message): `next and start_task: wait for work`.

### Task 8: `av watch`

**Files:**
- Create: `crates/scholion-cli/src/watch.rs`
- Modify: `crates/scholion-cli/src/main.rs` (the `Watch` subcommand)
- Test: `crates/scholion-cli/tests/cli.rs`

**Interfaces:**
- Consumes: the Rule 8 peek, `POST /v1/backlog/next` with `dry_run: true`.
- Produces:
  `av watch --agent NAME [--repo R] [--every SECS (default 15)] [--max-runs N] [--once] -- CMD [ARGS…]`.
  - It peeks every `--every` seconds.
  - When the peek is non-null, it prints `watch: <from> <repo>/<title> -> running CMD`, runs
    CMD with `AV_AGENT=NAME` (and `AV_URL`) and inherited stdio, and waits for it.
  - After CMD exits non-zero, or when the daemon cannot be reached, it backs off: double the
    interval, at most 300 s, reset after a success.
  - It stops after `--max-runs` runs. `--once` means one peek and at most one run.

- [ ] **Step 1: Write the failing tests:**
  - `watch_runs_the_command_only_when_work_fits`: post an item tagged `image`. Watching as
    claude (`--once`, CMD = `sh -c 'touch ran'`) does not create `ran`. Watching as grok does.
  - `watch_passes_the_agent_name`: CMD = `sh -c 'echo $AV_AGENT > who'`, so `who` contains
    `grok`.
  - `watch_stops_after_max_runs_and_backs_off_on_failure`: CMD = `false`, `--every 1
    --max-runs 2`. It exits after two runs, and its output says the command failed.
- [ ] **Step 2: Run** `cargo test -p scholion-cli --test cli watch`. Expected: FAIL.
- [ ] **Step 3: Implement** `watch.rs`. Use `std::process::Command`; no shell unless the user
  passes one.
- [ ] **Step 4: Run** `cargo test -p scholion-cli`. Expected: PASS.
- [ ] **Step 5: Commit**: `av watch: start an agent CLI only when work fits it`.

### Task 9: Tell agents, and prove the loop end to end

**Files:**
- Modify: `crates/scholion-daemon/src/mcp.rs` (the `start_task` description)
- Modify: `crates/scholion-daemon/tests/mcp.rs` (the size-guard cap, final value)
- Modify:
  - `integrations/agents-instructions.md`;
  - `examples/routing-smoke/README.md` (the agent prompt, and an `av watch` example per agent);
  - `README.md` (the "Several agents, several subscriptions" section);
  - `CLAUDE.md` (the `routing_api.rs` bullet: the order of `next`, reservations, wait, peek,
    slugs; the `scholion-cli` bullet: `av watch`).
- Test: `crates/scholion-daemon/tests/swarm.rs`

`start_task` description, exact replacement of its last sentence:
`Omit title and goal for your next job: a review or merge of others' work, a handoff, else a backlog task that fits you. wait: seconds to wait for one.`

Loop instruction (agents-instructions.md and the smoke prompt), exact:
`Call start_task with no title or goal and do what it returns (review, merge, handoff or new task). When it returns no task, call it again with wait set; stop after three empty waits.`

- [ ] **Step 1: Write the failing test** `swarm_loop_runs_backlog_to_merged`.
  - Claude and grok profiles; four items (two `code`, one `image`, one untagged).
  - One claude task hands off halfway with no `needs`.
  - Each agent loops on `next` with `wait_secs: 1` until three empty answers. For each answer:
    - a backlog task, or a claimed handoff: edit with `edit_greeter`, then `submit`;
    - a review offer that is not approved: validate, approve, merge;
    - a review offer that is approved: merge.
  - Assert that every task is `merged`, that no reviewer is its owner, and that there are at most
    25 `next` calls per agent.
- [ ] **Step 2: Run** `cargo test -p scholion-daemon --test swarm swarm_loop`. Expected: PASS if
  Tasks 2–7 are right. Otherwise fix `routing_api.rs`, not the test.
- [ ] **Step 3: Change the description.** Set the size-guard cap to the measured size, and assert
  that the growth over 10,200 bytes is at most 220.
- [ ] **Step 4: Update the docs** listed above.
- [ ] **Step 5: Run the full gate, then commit**: `Plan 9: agents loop from backlog to merged; docs`.

## Verification

- Full gate green: fmt, clippy, `cargo test --workspace`.
- `scripts/route-demo.sh --scripted` still prints the Plan 8 assignments.
- Real run (subscriptions, no API spend):
  - reset the smoke daemon, then run `examples/routing-smoke/setup.sh`;
  - start Claude and Grok with only the loop instruction;
  - halfway through, stop Grok, and restart it later with `av watch --agent grok -- grok -p "…"`.
- The real run passes when:
  - every pantry task ends `merged` with no human naming a task id;
  - no agent reviews its own task;
  - the task Grok was holding when stopped is either resumed by Grok or rescued once its lease
    expires;
  - `~/scholion-runs/` keeps the event log.
