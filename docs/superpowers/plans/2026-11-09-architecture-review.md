# Architecture review: what to improve (2026-11-09)

Scope and limits: this review reads the architecture as `CLAUDE.md` and the plans describe it,
plus a structural scan of the tree (file sizes, shared state, auth, notification paths). It is
not a line-by-line audit. Items marked **verify** are concerns the description raises that were
not confirmed in the code; check them before acting. Nothing here was changed in code.

Order is by what I would do first.

## P0: correctness and safety

1. **Fail closed on the network.** `AV_LISTEN` defaults to `0.0.0.0:7420` and `AV_TOKEN` is
   optional; safety rests on compose binding to `127.0.0.1`. Refuse to start on a non-loopback
   address without a token (or default to `127.0.0.1` and make the wide bind explicit).
   Test: start-up with `0.0.0.0` and no token exits with a message naming both variables.

2. **Agent identity is a claim, not a credential.** Every caller holds the same token and names
   itself in `X-Av-Agent`/`by`/`from_agent`. The rules built on it (a reviewer is never the
   owner, the handing-off agent never takes the task back, `agent_posts = propose`) stop an
   honest agent, not a confused or misdirected one. Add per-agent tokens (a `agent_tokens`
   table, hash stored, `av agent token NAME`); a request with one has its agent fixed by the
   daemon and a conflicting header is refused. Keep the shared token as "operator". This also
   lets the Commercial edition drop four hand-checked headers for one upstream mechanism.

3. **Attestations are not bound to what was checked.** A validation is a client's word. Store
   with each one the tree id and the command, and have `approve`/`merge` compare the tree, so a
   pass at another head is visibly stale. The Commercial Stemma layer already computes a scope
   oid; upstream the simple (whole-tree) form.

4. **State that dies with the process.** `AppState.reviews` (600 s reservations), MCP sessions
   (the tasks a session started), `footprints`, the task locks and `MERGE_LOCK` are in memory.
   A restart mid-work double-offers reviews and makes MCP calls without `task_id` refuse.
   Persist reservations and MCP session→task rows in Postgres (two small tables; expiry by
   timestamp). State the single-daemon rule in code: take a Postgres advisory lock at start-up
   and refuse a second daemon on the same database, since two would break every in-process
   lock silently.

## P1: structure

5. **The extension surface grows one method per need.** `ext.rs` is now ~12 points, and the
   Commercial notes list three things blocked on "another `ext.rs` addition": a candidate
   source for the pack's live-work section, a way to tell the core trunk moved (a held import
   landing skips `trunk:advanced` and the re-index of main, so the index goes stale), and
   `av validate`'s pre-check. Add them as a batch: `PackSource` trait, `AppState::trunk_moved(repo)`
   (event + re-index + build-cache config, the tail of `advance_trunk` made public), and a
   `ValidatePrecheck` answer on `GET /v1/tasks/{id}/checks`. Then add a small crate in this
   repo that compiles against every point (an `ext` contract test), so a signature change
   fails here and not at the next vendor sync.

6. **Replace polling with notification.** The outbox worker polls every second and `next`
   waits re-run on a 10 s tick; nothing uses `LISTEN/NOTIFY`. `NOTIFY` in the outbox insert's
   transaction and a listener in the worker gives lower latency and an idle daemon that does
   no queries. Keep a slow tick as the safety net.

7. **Reference resolution by bare name.** A ref links to every same-named symbol at weight
   1/k, and the fan-out cap is off by default. Structural candidates for common names
   (`new`, `get`, `run`) are noise. Steps: turn the cap on by default at a measured value;
   then resolve through imports/`use` paths per language where tree-sitter gives them, falling
   back to bare name. Gate each step on `scholion-bench retrieval` not regressing.

8. **Split the largest files by domain.** `scholion-cli/src/main.rs` (2,817 lines),
   `scholion-store/src/pg.rs` (2,395), `scholion-index/src/store.rs` (1,637),
   `scholion-pack/src/assemble.rs` (1,569), and the test modules `cli.rs` (5,359) and `api.rs`
   (5,118). These are the files every agent task collides on and the ones a pack can only
   point at whole. One module per aggregate in `pg.rs` (tasks, records, backlog, targets,
   repos); one file per command group in the CLI. Mechanical, no behaviour change, one PR each.

9. **One typed configuration.** Behaviour is spread over dozens of `AV_*` variables read in
   several crates (daemon, CLI, hooks). Collect them in one table (name, type, default, who
   reads it), generate the reference section from it, add `av config` to print the effective
   values, and warn on an unknown `AV_*` variable (typos are silent today).

## P2: evidence and upkeep

10. **Run the evaluations before the next feature plan.** Plan 6 is in progress and Plan 8's
    evaluation is written but not run, while Plans 9–15 built on their assumptions. The
    product's claim (packs and records save tokens and rework) should have a current number.

11. **A skipped test should not read as a pass.** Qdrant tests pass in ~0 s without
    `AV_TEST_QDRANT_URL`. Mark them `#[ignore]` with a reason, or print a counted skip, and
    add a CI job with a Qdrant service so the semantic path is exercised on every PR.

12. **`CLAUDE.md` is 35 KB**, loaded into every agent session (roughly 9k tokens), mostly one
    paragraph per module. That is the cost this product exists to remove. Cut it to commands,
    invariants and a module map (~150 lines); move each module's detail into a doc comment at
    the top of that module or into records anchored to it, which a pack then delivers only
    when relevant. A good dogfooding measurement for item 10.

13. **jj-lib is pinned to one exact version behind one crate** (good). Write the upgrade
    procedure down (which suites prove equivalence: `vcs.rs`, `merge.rs`, the E2 fuzz, the
    patch interop tests) and try one bump, so the pin does not become permanent by default.

14. **verify:** whether a slow `subscribe_events` subscriber can miss events (a lagging
    broadcast receiver); extensions that keep books from events (alerts, gates) need a
    replay-from-`events`-table path by id if so.
