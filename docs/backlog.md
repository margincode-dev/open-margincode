# Backlog of observations

Errors and anything else that did not fit, written down when they were seen so they can be taken
up later (`constitution.md`, rule 5). Newest first. An entry stays when it is fixed, marked with
the pull request that fixed it. No secrets, keys or local paths: this repository is public.

Each entry: the date; where (command, test, commit); what was seen, exactly; what was ruled out;
the suspected cause, marked as a guess; the status.

## 2026-11-28: `av-wrap.sh` waits up to 300 s twice on a daemon that does not answer

- **Where.** `integrations/generic/av-wrap.sh`, which runs `av context` before the agent command and
  `av checkpoint` after it; in a container, against stand-in listeners.
- **Seen.** With a listener that accepts and never answers, `av-wrap.sh true` took 623.9 s: the
  agent command started only after `av context` gave up at 300 s (the CLI's overall timeout), and
  `av checkpoint` waited as long again. With packets dropped it took 10.0 s (5 s connect timeout,
  twice); with nothing listening, or with HTTP 500, 0.0 s. Exit status was the command's in all
  four.
- **Ruled out.** The hook budgets (`AV_HOOK_*_SECS`): the wrapper runs ordinary commands, which keep
  their timeouts.
- **Guess.** The wrapper could run `av context` under a limit of its own (a budget like the
  session-start hook's), or the CLI could take an opt-in overall timeout from the environment.
- **Status.** Open.

## 2026-11-25: the daemon's git ran a remote's hooks with the daemon's environment

- **Where.** A security review of an embedding product, which runs the daemon with secrets in its
  environment next to agents that hold `AV_TOKEN`. `Vcs::push_backup`, `Vcs::advance_trunk` and
  `init_repo` (`margincode-vcs`, `jj.rs`), `POST /v1/repos/{name}/backup`, `.../backup/push` and
  `.../trunk/advance` (`backup.rs`), and `margincode-agentrun`'s `process::run_until`.
- **Seen.** A push to a path remote ran its `pre-receive`, `update`, `reference-transaction`,
  `post-receive` and `post-update` hooks and its `core.alternateRefsCommand`, as the daemon, with
  its whole environment (checked by hand with git 2.43, then by
  `merge::a_backup_push_to_a_path_runs_none_of_the_remotes_hooks`); a `-c core.hooksPath` of the
  local git does not reach the remote side, since git clears `GIT_CONFIG_PARAMETERS` for it. An
  advance fetch ran the store git dir's `reference-transaction` hook. A git child's environment
  held every variable of the daemon's (`merge::git_gets_none_of_the_daemons_environment_...`
  read `CARGO_MANIFEST_DIR` in it). `EXT::sh`, `fd::`, other `helper::` and unknown schemes were
  passed to git. The three routes moved the backup and trunk with no guard. `run_until` started
  the `kill` program with the caller's environment.
- **Ruled out.** `uploadpack.packObjectsHook` in a remote's own config does not fire (git reads it
  from protected config only; the fetch test sets it). `core.sshCommand` is read from the store's
  own git dir and the global config only, never from a remote, and is kept: it is how an operator
  picks an ssh key.
- **Fixed.** Commits on branch `sec-open`: `git_command` (cut-down environment, hooks and
  fsmonitor off, `GIT_ALLOW_PROTOCOL`, the same settings for a path remote's receive-pack and
  upload-pack), `check_backup_remote`, `Guarded::Backup`, `kill(2)` in `margincode-agentrun`.
- **Not covered.** (a) `embed_choice.rs` starts `ollama` with the daemon's whole environment;
  not changed. (b) The CLI's own `git` and `kill` children (`local.rs`, `remote.rs`) run on the
  user's machine and were left as they were. (c) `GIT_CONFIG_COUNT`/`GIT_CONFIG_KEY_n` settings
  in the daemon's environment no longer reach its git; an install that relied on them (an
  `insteadOf`, say) must move them to the global config. (d) Whether jj-lib 0.46 starts a git
  child in any path the daemon uses was not checked line by line (it imports and exports through
  gix). (e) On Windows, `core.hooksPath=/dev/null` was not tried. (f) `Guarded::Backup` carries
  no actor: the routes have none.
- **Status.** Fixed on `sec-open`, not yet merged.

## 2026-11-24: most merges of the day conflicted in this file, because every entry goes at its top

- **Where.** `av merge` in this repository's dogfood, on tasks that each added an entry here.
- **Seen.** By the owner's count, four of the day's six merges conflicted, each in
  `docs/backlog.md`. The file says "newest first", so every task inserts its entries right under
  the header; any two tasks open at once then change the same lines, and the second to merge
  conflicts there and needs `av sync`, a hand resolution keeping both sides, a new bar run and
  a resubmit.
- **Ruled out.** Nothing checked: the conflicts were not looked at one by one, and whether other
  files (`CHANGELOG.md`'s "Unreleased" list, which is also prepended to) conflicted the same way
  was not counted.
- **Guess.** Options, none decided: (a) new entries at the end of the file, which likely moves
  the collision rather than ending it: two tasks would still insert at one point, after the same
  last line, and a three-way merge conflicts on two insertions at one point; (b)
  one file per entry (`docs/backlog/2026-11-24-<slug>.md`), which two tasks never both create,
  and an index written by hand or not at all; (c) a union merge for files marked append-only
  (git's `merge=union` in `.gitattributes`), which needs the daemon's merge, done by jj-lib and
  not git, to honour it: whether jj-lib 0.46 reads merge attributes was not checked, and a union
  can keep both halves of an edited entry; (d) a repository setting beside `carry_globs` (which
  already lets an approval survive a resolution in such files) naming append-only files that
  the daemon's merge resolves itself by keeping both sides, with the same risk as (c).
- **Status.** Open.

## 2026-11-24: a title word that names a common symbol filled a first pack with `main` functions

- **Where.** `av task start` in this repository (packer version 13, budget 800), on the task that
  gave "Main" and "Default" titles slugs of their own. Its title held the word "Main"; its goal
  named, in backticks, `slugify`, `create_task`, `valid_component` and `task_workspace`, the
  file `pg.rs` by its crate and name alone, and the other four files the change touched
  (`CHANGELOG.md`, `docs/backlog.md` and the two test files) with their paths.
- **Seen.** The Map listed two files under `bench/results/`, and the Code section held three
  functions named `main` from `bench/results/` and a `load` of `bench/tasks-m0/`. `bench/results/`
  is in `.av/sparse`, so none of those is in the task's workspace. The pack carried none of the
  five changed files as code: it named `crates/margincode-store/src/pg.rs` only in its list of
  omitted items, and the other four were not in the shown part of that list. A second pack with
  `--focus crates/margincode-store/src/pg.rs` carried `slugify` and `IntentStore.create_task`,
  but also a `slugify` of `bench/tallyho/` and another `main` of `bench/results/`.
- **Reproduced.** On dogfood at `5f8c193` (packer version 14, no semantic layer: the repo's
  `outbox` route answers `semantic: false`), a new task with the same title and goal, budget 800:
  the Map listed two files under `bench/results/`, and Code held three `main` functions of
  `bench/results/` (score 0.6, reason `seed`) and the `slugify` of `bench/tallyho/` (0.6). A pack
  of the same task at 30,000 tokens had `PgStore.create_task`, `slugify` and `task_slug` of
  `pg.rs` among its seeds, all at 0.6 as well. The same goal under another title gave the same
  candidates and scores (they differed only in records' recency, in the fifth decimal).
- **Ruled out.** The title. The packer's query is the goal, or the focus when one is given
  (`assemble::semantic_query`, `lexical::named_identifiers(&task.goal)`); the title is only
  printed in the Task line.
- **Cause (found, three rules together).**
  - *The names the goal gives.* `lexical::named_identifiers` takes every backticked word of four
    characters or more as an identifier and keeps the first eight (`MAX_NAMED`). This goal's
    first backticks are `main` and `default` ("gives the slug `main` or `default`"), and a
    backticked name with a `-` gives each of its words (`margincode-store` gives `margincode` and
    `store`, `main-task` gives `task`). The eight were `main`, `default`, `margincode`, `store`,
    `create_task`, `task_slug`, `slugify` and `task`; `valid_component` and `task_workspace`,
    named last, were cut.
  - *Which symbols a name finds.* `PgIndex::search_symbols` (`margincode-index` `store.rs`)
    matches a substring of the qname or the path, puts exact qnames first, then orders by path
    byte by byte, and stops at 10 rows (`FOCUS_LIMIT`) before `names_symbol_at` filters them and
    `NAMED_PER_IDENT` keeps four. The repository has more than ten functions named `main`, and the
    first ten by path are all under `bench/results/` (a pack with `focus=main` seeds exactly
    those ten), so the four `main` seeds were the four first by path. Names compare without case,
    so `store` found `Store` and `task` found `Task`.
  - *Which seeds fit.* A seed scores `score::code_score(0, 0.0)`, 0.6, unless a lexical hit adds
    to it; none did here (the eight lexical hits at this budget were other symbols, at most 0.56).
    Equal scores go by `Item::tie_key`, the source path, so `bench/` seeds took the Code section's
    room before any `crates/` seed. The fill (`budget::fill`) also passes over an item whose handle
    does not fit what is left and takes a shorter one after it: the fourth `main` was left out and
    the `slugify` of `bench/tallyho/` taken. Neighbours are cut the same way by age: the packer
    keeps 40 (`MAX_NEIGHBORS`) ordered by hops, then symbol id, and a file untouched since the
    import (the result archives) holds the lowest ids, since a re-indexed file's symbols get new
    ones; at 30,000 tokens every hop-1 neighbour of this goal was under `bench/`. `expand_best_hits`
    cuts its neighbours by id too: this backlog task's own first pack had `JobQueue.push` and
    `JobQueue.len` of `bench/retrieval/ci/` beside its one hit, `PgStore.task_packs`, whose body
    uses `push` and `len`.
  - Not reproduced: the `load` of `bench/tasks-m0/`. No `load` reached the reproduced pack at 800
    tokens; the one `load` among the candidates at 30,000 tokens is of a result archive, at hop 1
    (0.425), below every seed.
- **Fixed in part (packer version 15).** The `bench/results/` items such a pack shows now say
  they are not on disk, and the Task line naming `bench/results/` is in every first pack (see
  the entry on a left-out file in the Map). Which names and seeds the goal gives is unchanged.
- **Status.** Open.

## 2026-11-24: `av record --anchor path:create_task` was taken, but did not cover `PgStore.create_task`

- **Where.** `av record --kind decision --anchor crates/margincode-store/src/pg.rs:create_task`,
  then `av submit`, on the same task as the entry above.
- **Seen.** The record answered `recorded` with no warning. `av submit` then advised that two
  changed symbols had no record anchored, and `GET /v1/tasks/{id}/unrecorded` named
  `MAX_TASK_SLUG_BYTES` and `PgStore.create_task`. A new record anchored to
  `pg.rs:PgStore.create_task` cleared it.
- **Ruled out.** Nothing; where the first anchor landed was not looked at.
- **Guess.** An anchor names a symbol by its qualified name, and one that matches no symbol is
  stored without a word to the agent.
- **Status.** Open.

## 2026-11-24: a first pack for a new bench subcommand named the files beside it, not its entry point

- **Where.** `av task start` in this repository, on a task whose goal asked for a planted set
  under `bench/` and a `margincode-bench` command to run `av dream` on it.
- **Seen.** The pack named `crates/margincode-bench/src/dreamed.rs`, `isolate.rs`, the CLI's
  `dream.rs` and a `check_set.py`; the change went into `crates/margincode-bench/src/main.rs`
  (the subcommand), `lib.rs` (the module line) and `runner.rs` (one helper made visible), none of
  which it named.
- **Reproduced.** On dogfood at `5f8c193` (packer version 14, no semantic layer), a new task with
  the same title and goal, budget 800: the Map listed three `check_set.py` under `bench/`,
  `crates/margincode-bench/src/conflict.rs` (merged since), `dreamed.rs` and the CLI's
  `dream.rs`; Code held `CONFLICTS`, three `check` functions and a `BENCH` of the `check_set.py`
  files, and the `check` of `conflict.rs`, all seeds.
- **Ruled out.** A gap in the index for `main.rs`: its clap enum `Cmd` is a symbol (a pack with
  `focus=Cmd` names `Cmd (crates/margincode-bench/src/main.rs:129-439)`), and `main.rs` at the
  task's base already held `enum Cmd` with `DreamFreeze`.
- **Cause (found).** `Cmd` had no score: it was never a candidate. The entry point was one hop
  from a seed and was cut for its age.
  - *Not named.* The goal's eight names (`named_identifiers`) were `bench`, `conflict`, `check`,
    `margincode`, `dream`, `freeze`, `crates` and `conflicts`: the words of its backticked paths
    and commands (`bench/conflict-check/`, `margincode-bench`, `dream-freeze`,
    `crates/margincode-bench/src/dreamed.rs`), while a name ending in `.rs` is dropped as a file.
    These seeded the `check` functions of `bench/tasks-*/check_set.py` (first by path, as in the
    entry on `main` functions above), `BENCH` (names compare without case), `freeze` of
    `dreamed.rs`, two `Dream` structs, `CRATES` and `CONFLICTS`. Nothing in the goal names `Cmd`
    or one of its variants.
  - *Not searched.* Full-text search takes the goal's first 16 words (`lexical::MAX_TERMS`),
    which end at "records"; "harness", "command", "dreamed" and "main" come later in the goal, so
    none of them was searched. `Cmd` was not among the 8 lexical hits a pack of 800 tokens takes,
    nor among the 40 of a pack of 30,000 tokens on today's main.
  - *Cut from the neighbours.* `fn main` of `crates/margincode-bench/src/main.rs` calls
    `dreamed::freeze`, at the task's base as today, so it is one hop from the seed `freeze`, and
    `Cmd` two. The packer keeps 40 neighbours ordered by hops, then symbol id (`MAX_NEIGHBORS`,
    `gather_structural`). At 30,000 tokens the 40 kept were all at hop 1, with ids up to 474, all
    under `bench/`; a file that is often changed is re-indexed with new ids (`Cmd` is `sym:79785`
    today), so `main.rs` lost the cut to symbols that only had older ids. That is what outranked
    it.
  - *`runner.rs`* had no item in the pack at 30,000 tokens either; why was not looked at.
  - *The module list is no symbol.* `extract::rs_block` walks a `mod` only when it has a body, so
    `pub mod x;` lines make nothing: the crate's `lib.rs` has one symbol, `pub type Result` (a
    pack with its path as focus finds only that). Its module list can reach a pack only as a
    changed file or through that `Result`.
  - Not checked: a pack against the task's own base. Today's index has `conflict.rs`, which the
    first pack could not have had; it named `isolate.rs` instead.
- **Status.** Open.

## 2026-11-24: four places print a record with no verdict mark at all

- **Where.** `av why` (`derived::why` and its CLI print), `av search` (the CLI print of
  `GET /v1/repos/{repo}/search`), the MCP tool `search_history`, and `av expand task:<slug>`
  (`Packer::expand` prints the task's records with an empty `Whence`).
- **Seen.** Each prints a record's id, kind and summary and nothing of its verdicts: a disputed
  record is not marked, and since the conflict verdict an overruled one is not either. Packs, the
  pre-read hook's answer and `av expand rec:…` mark both, and search's JSON carries `verdict` and
  `overruled`.
- **Ruled out.** A conflict-only gap: the same holds for `disputed`, so these places were left as
  they are to keep the two consistent.
- **Guess.** These views predate verdicts and were never passed `Origins`.
- **Status.** Open.

## 2026-11-24: a first pack for a change to packs named no file of the packer

- **Where.** `av task start` in this repository (packer version 13, budget 800), on a task whose
  goal named the rank factor, the pack line, the packer version, `records_at`, `av expand`,
  `av why` and the MCP record text.
- **Seen.** The Map listed `crates/margincode-daemon/src/api.rs` and its tests, then
  `bench/results/2026-11-14-m0/sonnet_spend.py` (left out of the workspace by `.av/sparse`, as in
  the entry below), `bench/tallyho/src/tallyho/money.py` and `scripts/demo-replay.py`. Code named
  `records_at` and the `CONFLICTS` text of the previous task. Nothing named
  `crates/margincode-core/src/standing.rs`, `crates/margincode-pack/src/assemble.rs` or
  `crates/margincode-pack/src/packer.rs`, where the change went.
- **Ruled out.** Nothing yet.
- **Guess.** Words such as "rank", "line" and "version" match many files weakly, and the goal's
  names (`Whence`, `DISPUTED_FACTOR`) were not in it, so no symbol seeded the packer's files.
- **Status.** Open.

## 2026-11-24: a first pack's Map listed a file the task workspace leaves out

- **Where.** `av task start` in this repository (packer version 13, budget 800), twice on
  2026-11-24, on tasks about dreams and verdicts.
- **Seen.** Both Maps listed `bench/results/2026-10-27-haiku/tool-mass.py`, and the second one
  had it among four entries. `bench/results/` is in `.av/sparse`, so the file is not in the task's
  workspace; it can be read only with `av show`. The second pack also named `mcp.rs`, which the
  goal mentioned, but not `crates/margincode-cli/src/main.rs`, where the new flag went, nor
  `crates/margincode-daemon/tests/dream.rs`, which held the tests the goal asked for.
- **Reproduced.** On dogfood at `5f8c193` (packer version 14, no semantic layer), new tasks with
  each goal, budget 800. The second task's Map listed the file as its fourth entry, score 0.425
  (reason `map`, the score of hop 1); in the first task's pack it was a candidate at the same
  score and the first of the omitted items.
- **Ruled out.** That the file won on its text, and that the Map ranks the repository's files.
  The Map takes only the files of seeds and of their neighbours (`gather_structural`:
  `map_paths` is `seed_paths` with `neighbor_paths`); a lexical or semantic hit adds a Code item,
  never a Map entry.
- **Cause (found).**
  - *How it came in.* The second goal names `propose_one`, whose body uses the identifier `kind`,
    and a ref links by bare name to every symbol of that name (`rebuild_edges`). The Python
    function `kind` at lines 3 to 10 of `tool-mass.py` is one hop from it. The packer keeps 40
    neighbours ordered by hops, then symbol id, and this one has one of the lowest ids in the
    index (`sym:4`), so it is always kept. Its file scores as hop 1; equal Map scores go by the
    item id `file:<path>`, so `bench/` comes before `crates/`. In the first task the seed is
    `PgStore.record_origins`, which has a local `kind` too.
  - *Left-out files are in the index.* Nothing reads `.av/sparse` but the Task line. The index
    skips only `node_modules`, `.venv`, `target` and `.git` (`margincode-index` `is_ignored`); a
    task's scope reads main's rows for every path its overlay does not hold, and a snapshot of a
    sparse workspace keeps the left-out files in the tree, so they have symbols, refs and BM25
    chunks in every task's view. `sym:4` is the first lexical hit of a focus made of its own
    words, and a pack with `focus=main` seeds ten `main` functions of `bench/results/`.
    `Vcs::off_disk` is read in one place, `gather_structural`, for the Task line.
  - *What the pack says about such a file.* Nothing on its Map line or Code item. The line
    `Not on disk: …` is in the Task item's summary and full tiers, not its handle; with a long
    goal at 800 tokens the Task goes at its handle, and none of the four packs reproduced for
    these entries (both tasks here, and those of the entries on `main` functions and on a new
    bench subcommand) had the line. `docs/reference.md` says the first lines of a pack say what
    the workspace leaves out, and the `CHANGELOG.md` entry for `av widen` says the pack's Task
    section says it; `the_task_section_names_what_the_workspace_leaves_off_disk` checks it only
    with a short goal. Whether a left-out file may be a Map or Code candidate is written nowhere.
  - Not looked at: why the second pack named `mcp.rs` but not the CLI's `main.rs` or
    `tests/dream.rs`.
- **Fixed in part (packer version 15).** The `Not on disk: …` line is a required Task item
  of its own (`off-disk`), scored below the task's, so a long goal at a small budget keeps it
  and a budget that holds only the goal still sends the goal; a Map entry or Code item of a file
  under a left-out path ends its first line with `` (not on disk: `av show`) ``
  (`assemble::mark_off_disk`). The pack tests
  `a_task_at_its_handle_tier_still_names_what_the_workspace_leaves_off_disk`,
  `a_budget_too_small_for_the_off_disk_line_still_sends_the_goal` and
  `a_symbol_in_a_file_left_off_disk_is_marked_in_the_map_and_the_code` check them. Later packs
  of a session print the line as sent earlier, as they do the task.
- **Status.** Open: left-out files are still candidates, ranked as before; whether they should be
  is a ranking question that waits for a measured comparison.

## 2026-11-24: a first pack named a same-named struct of another crate and no migration

- **Where.** `av task start` on a task whose goal named `VerdictRow`, `NewVerdict`,
  `VERDICT_STATE`, `record_origins` and "migration 0035" (packer version 13, budget 800).
- **Seen.** The Code section listed `VerdictRow` at `crates/margincode-core/src/types.rs`, the
  task-review struct, not the `VerdictRow` of `crates/margincode-store/src/dreams.rs` the goal
  meant (that file was in the Map through another symbol). Neither
  `crates/margincode-store/migrations/` nor `tests/migrations_frozen.rs`, which a new migration
  must touch, was named. The other files the task changed (`standing.rs`, `dreams.rs`, `pg.rs`)
  were in the Map.
- **Ruled out.** A stale index: the task started on a fresh `main`.
- **Guess.** A bare name in the goal seeds every symbol of that name, and the first one found
  wins the slot; a `.sql` file has no symbols for "migration" to match.
- **Status.** Open; seen once.

## 2026-11-24: `swarm_loop_runs_backlog_to_merged` failed once in a full test run

- **Where.** `crates/margincode-daemon/tests/swarm.rs`, `cargo test --workspace` in the local
  Docker bar, on a branch that changes the index, the pack version and the benchmark only.
- **Seen.** One failure in one full run: task `code-item-1` was still `in_review` where the test
  expects `merged`, and the loop took 13 s. The test then passed three times on its own (about
  5.5 s each), and the next full run passed.
- **Ruled out.** The branch's own changes: none touches the swarm loop, the merge queue or a
  task's states.
- **Guess.** Timing under load: the full run has many test binaries on one Postgres, and the test
  stops waiting before the landing is done.
- **Status.** Open; seen once.

## 2026-11-23: the retrieval benchmark is not repeatable run to run

- **Where.** `margincode-bench retrieval` (`crates/margincode-bench/src/retrieval.rs`).
- **Seen.** Two runs of the same build over the same tasks give different rows. A small
  difference between two ranking configurations therefore cannot be read from one run of each.
  Reproduced with a handful of the public tasks, each run on a fresh daemon database with the
  same paths, no semantic layer: the packs differed in their `Workspace:` line in every task, and
  on one repo (hono, which has the same class in two files) also in the order of two code items,
  at every budget.
- **Ruled out.** Symbol ids (the same in both runs on a fresh database), the order the indexer
  stores files and symbols, the packer's own tie-breaks (`budget.rs`: total orders), the scorer
  and the `bm25_files` and `bm25_symbols` baselines (their texts were the same in every run). The
  semantic layer was off, so its share (embedder, Qdrant, timeouts marked `degraded`) is not
  checked.
- **Cause (found, three).**
  - *The harness.* Each run named its imports from the clock (`r<hex seconds>-<task>`). The name
    is in every pack, in the task's workspace path, so its token count took budget from the
    items: which item fit at the edge changed with the id, and so did the rows' `tokens` and
    `first_gold`. A different workspace root does the same.
  - *The product.* `search_code_lexical` (`crates/margincode-index/src/store.rs`) summed a chunk's
    BM25 terms in the order Postgres handed the rows to the aggregate. On a large repo the
    planner sorts them by symbol before grouping, and that sort does not keep each chunk's terms
    in order, so two copies of one function got sums that differ in the last bit. A lexical hit
    weighs by its rank, so the copies swapped places and every item after them moved. The plan
    depends on the table statistics, which a fresh database gathers at its own time. Seen in
    psql on the public repo by changing the term order and plan settings; real packs handed to
    agents had the same instability wherever identical code sits in two files.
  - *The `repo_map` baseline.* Its PageRank summed each file's edge weights over a `HashMap`,
    whose order differs from map to map, so files of two copies of one package swapped places in
    its text. Seen on the full public set once the first two were fixed; its rows did not change.
- **Status.** Fixed in this change. `retrieval --run-id ID` (default `e1`) replaces the clock;
  `bench/retrieval/README.md` ("Repeatable runs") names the conditions: same id, same workspace
  root, a fresh database. The lexical sum runs `ORDER BY` the term, so copies tie and go by path.
  `repo_map` keeps its edges in a `BTreeMap`. The test
  `retrieval_gate::two_runs_on_fresh_daemons_give_the_same_rows_and_packs` fails when the two
  runs' ids differ, and `retrieval::repo_map_is_the_same_every_time` failed before the change.
  The index test for the tie (`lexical::copies_of_one_function_tie_*`) passes on the code before
  the fix too: on a fixture that small the planner keeps each chunk's terms in order. Exact ties
  can now order differently from before, so `PACKER_VERSION` went to 13. Not checked: the
  semantic path.

## 2026-11-23: records found by full-text search tie on a random id

- **Where.** `search_records_fts` in `crates/margincode-store/src/pg.rs` (`ORDER BY
  ts_rank(...) DESC, r.id`).
- **Seen (read, not reproduced).** Records with the same rank are ordered by `r.id`, a random
  UUID, so two stores holding the same records can list equal-rank records in different orders.
- **Ruled out.** Nothing; the retrieval benchmark has no records, so it does not reach this.
- **Guess.** A tie-break on something both stores share (created time, then the summary) would
  make it repeatable; a ranking change, not made here.
- **Status.** Open.

## 2026-11-23: a pack rebuilt because main moved refers back to the pack it threw away

- **Where.** `pack_for_task` (`crates/margincode-daemon/src/api.rs`) and `Packer::build`
  (`crates/margincode-pack/src/packer.rs`), read while building the measures of packs.
- **Seen (read, not reproduced).** When main moves during a build, `pack_for_task` builds the
  same request again and answers the second pack. The first build already wrote its items to the
  session's ledger (`ledger_put`), so the second sends those items as `↩ … (sent earlier)`
  although the agent never received the first pack. Both receipts are stored.
- **Ruled out.** Nothing: no test builds a pack while main moves.
- **Guess.** The first build's ledger write should not happen (or be undone) when its pack is
  discarded. A change to the packer or the daemon's pack path, not made here.
- **Status.** Open.

## 2026-11-23: two test-path rules, the packer's and the benchmark's

- **Where.** `margincode_pack::score::is_test_path` (ranks test code at 0.6) and
  `margincode_core::outcome::is_test_path` (the benchmark's gold files and the live measure of
  packs).
- **Seen.** They differ: the benchmark's also treats `spec`, `specs` and `testing` directories,
  `conftest.py` and `*_tests.py` as tests, and ignores case; the packer's does not.
- **Ruled out.** Nothing.
- **Guess.** One rule for both would change ranking, so it needs a `PACKER_VERSION` bump and new
  golden files.
- **Status.** Open.

## 2026-11-23: the CLI test binary crashed with SIGSEGV once

- **Where.** `cargo test --workspace` at `7e3bfea` (unchanged `main`), in a `rust:1.97.1`
  container as root, Postgres from `AV_TEST_DATABASE_URL`, another build running on the host.
- **Seen.** `tests/cli.rs` stopped with `signal: 11, SIGSEGV: invalid memory reference` after
  about 50 of its tests had passed; no test was named as failing. The same binary run alone right
  after passed: 171 passed, 0 failed.
- **Ruled out.** A change of ours: the tree was `main` as merged.
- **Guess.** A crash in a dependency under load, or a stack overflow in one test thread. Not
  reproduced.
- **Status.** Open.

## 2026-11-22: `JjVcs::create_workspace` takes the names `main` and `default`

- **Where.** `margincode-vcs` `jj.rs`, `create_workspace`, reached through `POST /v1/tasks` on
  `main` 7e3bfea with a task titled "Main" or "Default" (a daemon API test). On 2026-11-24 the
  test `a_reserved_or_long_multibyte_title_starts_and_its_workspace_is_reclaimed`, run before the
  fix, got the same two answers to the start; the directory and `av gc` were not looked at again.
- **Seen.** `create_workspace` checks only `valid_component`, not `task_workspace`, so it makes a
  workspace for either slug. "Main": the workspace was made, then `init_task_overlay` refused
  with `400 invalid input: main never has an overlay`, the task was set abandoned, its directory
  stayed, and `POST /v1/gc` with `retain_secs: 0` answered the error `workspace main belongs to
  the repo, not a task`. "Default": jj-lib panicked with `BUG: Descendants have not been rebased
  after the last rewrites.` and the start answered 500.
- **Ruled out.** Whether the "Default" start changed the repo's own workspace was not checked.
- **Guess.** The store now never gives a task such a slug (it becomes `main-task` or
  `default-task`), so the daemon no longer reaches this. Refusing them in `create_workspace`
  would change `JjVcs`'s behaviour, which the Commercial edition's engine mirrors, so it was left.
  A task made before the change with the slug `main` or `default` keeps it, and its workspace
  still cannot be reclaimed.
- **Status.** Open.

## 2026-11-22: OpenCode sessions do not get `AV_SCRATCH` or `AV_DOCKER_LABELS`

- **Where.** The OpenCode plugin (`integrations/opencode/av.js`), live with OpenCode 2.3.2.
- **Seen.** `av hook session-start` exports `AV_SCRATCH` and `AV_DOCKER_LABELS` by appending to
  `$CLAUDE_ENV_FILE`, which only Claude Code sets. Under OpenCode the variable is unset, so the
  model's shell commands have neither; the pack still names the scratch directory in its last
  line, and nothing tells the model to label its containers.
- **Ruled out.** Nothing: the plugin does not try to pass them on.
- **Guess.** OpenCode 2.3.2 has a plugin hook `shell` `create.before` whose `env` the plugin may
  change before a shell command starts (`packages/core/src/shell.ts` at `v2.3.2`; read, not run).
  The plugin could point `CLAUDE_ENV_FILE` at a file of its own for `av hook session-start`, read
  the `export` lines back, and add them to each command of that session. Not done in the change
  that added the plugin.
- **Status.** Open.

## 2026-11-22: under OpenCode, the hook messages' `av record` fails and MCP `record` needs a `task_id`

- **Where.** Live OpenCode 2.3.2 sessions in a task, free model, a private server with no av
  variables in its environment.
- **Seen.** The pack's last line and the stop's message tell the model to run `av record --json -`.
  In the model's shell that failed (`av: command not found` there; with `av` on `PATH` it would
  still lack `AV_URL` and `AV_TOKEN`). The model then called the MCP tool `tools.av.record`
  without `task_id` and got `no task: pass task_id, or call start_task or claim_task first`; it
  found the task by trying `get_context` and `status` with the slug as `task_id`, and recorded
  with that. Each session spent several tool calls on this.
- **Ruled out.** The plugin: the hooks themselves ran and their messages arrived.
- **Guess.** The MCP session is not bound to the workspace's task, since OpenCode's MCP client
  starts outside it. The pack or the messages could name the `task_id` for MCP callers (a change
  to the hook text), and the `shell` `create.before` hook (entry above) could give the model's
  shell `AV_URL`, `AV_AGENT` and `AV_TOKEN`.
- **Status.** Open.

## 2026-11-22: OpenCode's automatic compaction is untested with the plugin

- **Where.** The OpenCode plugin's `compaction` hook.
- **Seen.** A compaction started through `opencode api session.compact` fired the hook, and the
  next prompt got a fresh pack. A compaction OpenCode starts by itself when the context fills
  was not reached on the free model.
- **Guess.** The same hook fires for both: a manual, an automatic and an overflow compaction all
  go through `SessionCompaction.compact` (`packages/core/src/session/compaction.ts` at `v2.3.2`).
  That was read, not run.
- **Status.** Open.

## 2026-11-22: the OpenCode plugin can still send a stop message the interrupt meant to stop

- **Where.** `integrations/opencode/av.js`, the `experimental.idle` hook.
- **Seen.** The plugin learns of an interrupt from the server-wide event
  `session.execution.interrupted`, which reaches a plugin's subscription after the interrupt has
  happened. Live, an interrupt while `av hook stop` waited killed that child and nothing was sent.
  But when `av hook stop` has already answered `block` and the plugin has called `synthetic`
  before the event arrives, the message goes out and starts a new turn (a late `synthetic` after
  an interrupt does, as the gates saw).
- **Guess.** The window is the event's delivery delay after `av hook stop` answers. Closing it
  needs a session state OpenCode does not hand the idle hook (whether it was interrupted).
- **Status.** Open.

## 2026-11-22: with the daemon unreachable, every hooked OpenCode tool call waits

- **Where.** `integrations/opencode/av.js`, the `read` and edit hooks (`execute.after`), with
  the daemon stopped, live with OpenCode 2.3.2.
- **Seen.** The first prompt of a session in a task went through without a pack once
  `av hook session-start` had given up on the connection. By the code, every later `read`,
  `edit`, `write` and `patch` of the session runs `av hook pre-read` or `post-edit` and waits the
  same way before the tool's result reaches the model; only the first prompt was run.
- **Guess.** A short cool-down after a hook that could not reach the daemon (skip the `av hook`
  calls of that server for a while) would bound the wait to one per cool-down.
- **Measured (2026-11-28).** In a container, through `av.js` and the plugin harness: with nothing
  listening (a stopped daemon) each hook took 0.3 s, so there was no real wait. With a listener
  that accepts and never answers, every hook took 60 s (the plugin's limit; `av hook` itself would
  have waited 300 s for a read or an edit, 614 s for a session start); with packets dropped, 5 s
  (the connect timeout) per request.
- **Status.** Fixed on branch `hooks-fail-open`: `av hook` has a budget per hook (3 s for a read,
  5 s for an edit, 15 s for the first prompt, 10 s for idle) and the hooks skip a daemon that made
  one of them wait in vain for 30 s.

## 2026-11-22: one `av plugin:` line in OpenCode's log, then silence

- **Where.** `integrations/opencode/av.js`, `warn`.
- **Seen.** By design the plugin logs only its first problem per server. A benign first line on
  `av hook`'s stderr (for example a footprint scan marked `partial`) uses it up, and a later real
  failure is then not logged.
- **Guess.** A per-kind once (a can't-start, a time-out, a failed exit) would keep the log short
  and still show a later failure.
- **Status.** Open.

## 2026-11-22: the embedded test Postgres refuses to start as root

- **Where.** `margincode-cli` `tests/cli.rs` (any test that calls `start_daemon`), in a
  `rust:1.97.1` container run as root, with no PostgreSQL installed in it and
  `AV_TEST_DATABASE_URL` unset.
- **Seen.** `TestDb::new` panicked with `postgres setup (needs network on first run):
  DatabaseInitializationError("Command error: stdout=; stderr=initdb: error: cannot be run as
  root ...")`, and the test then failed with `daemon failed to start: RecvError`.
- **Ruled out.** The change under test (OpenCode's `av init` writer): the same tests passed once
  `AV_TEST_DATABASE_URL` pointed at a throwaway `postgres:16` container.
- **Guess.** As root, only the installed-binaries path (`installed.rs`) drops to `postgres` or
  `nobody`; the embedded fallback runs `initdb` as the current user. `CLAUDE.md` ("Test
  environment") reads as if a root container works without an installed PostgreSQL. Either the
  embedded path could drop privileges too, or that text could name the requirement.
- **Status.** Open.

## 2026-11-22: the build-slot test missed its "waiting" line once under load

- **Where.** `margincode-cli` `tests/cli.rs` `validate_waits_for_a_free_build_slot_on_the_host`,
  in `cargo test --workspace` on the release 0.0.5 branch (version bump only), in a
  `rust:1.97.1` container while two other workspaces' test runs used the same host.
- **Seen.** The assertion at line 5158 failed: stderr did not contain "all 1 build slots on this
  host are in use; waiting". The checks before it passed (nothing ran during the 1.5 s, the
  command was still running, it then succeeded and wrote its file). The other 158 tests of the
  binary passed. Rerun alone in the same container: passed in 5.9 s.
- **Ruled out.** The change under test: it touches versions and text only.
- **Guess.** `av validate` checkpoints the task before it asks for a build slot. When that took
  longer than the test's 1.5 s, the test released the slot before validate reached
  `slots::wait_for`, which then found it free and printed nothing. The 1.5 s sleep is then an
  upper bound on the checkpoint's time, which `CONTRIBUTING.md` ("Timing") forbids. The test
  could wait for the "waiting" line on stderr instead of sleeping.
- **Seen again, 2026-11-27.** Same assertion (now line 6247), in `cargo test --workspace` on the
  questions change (PR #114, nothing under `validate` or `slots` touched), in a `rust:1.97.1`
  container (the host's load was not recorded). The other 176 tests of the binary
  passed. The same build's run before it passed, and five runs of the test alone passed (each
  about 5.8 s).
- **Status.** Open.

## 2026-11-21: the installer and the formula install on a glibc that is too old

- **Where.** `brew install margincode` from the 0.0.4 formula in the `homebrew/brew` image
  (Ubuntu 22.04, glibc 2.35).
- **Seen.** The archive's sum matched and the three binaries were linked; `margincode --version`
  then stopped with ``version `GLIBC_2.39' not found``. `scripts/install.sh` makes no check
  either. On Ubuntu 24.04 (glibc 2.39) both work.
- **Status.** Open; `docs/install.md` now states the requirement. The installer could read
  `ldd --version` and say so before it downloads.

## 2026-11-21: a local build of the lite image fails with 401 when no token is given

- **Where.** `deploy/Dockerfile`, built by hand with
  `--build-arg FEATURES=margincode-daemon/bundled-postgres` and no `github_token` secret
  (Docker 29.9.3, buildx 0.38.1), at the 0.0.4 release commit.
- **Seen.** `postgresql_embedded`'s build script stops with "401 Unauthorized" from
  `api.github.com/repos/theseus-rs/postgresql-binaries/releases`. The same `cargo build` in a
  `rust:1.97.1` container started with `docker run`, with no token at all, passes.
- **Cause.** `--mount=type=secret,id=github_token,env=GITHUB_TOKEN,required=false` sets
  `GITHUB_TOKEN` to an empty string when the secret is absent (a three-line Dockerfile shows
  "set, length 0"), and the build script sends the empty token.
- **Not affected.** `image.yml` passes the secret, so the published image builds; Compose
  builds without the feature.
- **Status.** Open. Unsetting an empty `GITHUB_TOKEN` before `cargo build` would do.

## 2026-11-21: `margincode-daemon --version` starts a daemon

- **Where.** The 0.0.4 archive, in the release's install check.
- **Seen.** `margincode-daemon --version` printed "lite mode: embedded postgres started" and
  "margincode-daemon ready", and was still running when `timeout 25` ended it: the daemon reads
  no arguments. `margincode --version` and `av --version` print the version.
- **Status.** Open.

## 2026-11-21: a file whose line endings changed lists every symbol of the file as changed

- **Where.** `GET /v1/tasks/{id}/unrecorded` (so the Stop hook's block and `av submit`'s
  advisory), which takes its symbols from `margincode_pack::assemble::changed_symbols`. Seen on a
  build of `f2ebcde` and in Claude Code sessions on `7cc7d1f`.
- **Seen.** Adding `DEFAULT_PAGE_SIZE = 100` to `tallyho/store/scan.py` listed
  `DEFAULT_PAGE_SIZE, count_records, iter_records`. A session that changed only
  `apply_discounts` was stopped over all 8 symbols of `discounts.py`, three of them above the
  edited function. Those files are stored with CRLF endings. That session made two edits, both inside
  `apply_discounts`; every session that made one edit to its file was stopped over the edited
  symbols only. That the second edit rewrote the file's endings is inferred from this, not
  seen: the session's files were not kept. Reproduced through the route: the same insertion
  with the endings kept lists only `DEFAULT_PAGE_SIZE`, and with the file saved as CRLF lists all
  three.
- **Ruled out.** A symbol that only moved down: an insertion above two functions lists only the
  inserted name. The whole-file fallback of `changed_ranges_of` (both reads succeed) and stale
  index spans (the new name is listed, so the index is at head, and a shift could not list
  symbols above the edit).
- **Cause.** `line_ranges` compared lines with their endings (`similar::TextDiff::from_lines`),
  so a file whose endings changed was one replacement over every line, and every symbol
  overlapped it. Whether, and why, an agent's edit tool rewrites a file's endings on a second
  edit is not verified.
- **Status.** Fixed in this change: lines are compared with `\r\n` read as `\n`, split as
  before so the line numbers stay those of head; other differences at a line end still count.

## 2026-11-21: a file-anchored record ranks lower and goes stale on any edit in the file

- **Where.** The packer's record scoring and its `(changed since recorded)` check; a reader-side
  check of deterministic packs, no model.
- **Seen.** For a later task working on the same function, a record anchored only by the file's
  bare path scored 0.485 (path match), and one anchored on that function, the seed symbol, 1.66.
  After an edit anywhere in the file the file-anchored record was flagged
  `(changed since recorded)` and dropped from 0.485 to 0.3395; a symbol-anchored one is flagged
  only when its symbol changes. A symbol-only record does not satisfy the Stop and submit check
  when something else in the file changed too. So the session-start line now asks for both: the
  changed symbol, for ranking, and the file's bare path, for the check.
- **Ruled out.** Reach: a file-anchored record reaches a later task's pack in the same cases as a
  symbol-anchored one.
- **Guess.** None. Whether the packer should treat a file-anchored record as anchored on the
  symbols its own task changed is open.
- **Status.** Open.

## 2026-11-21: the pre-read hook shows a record without its changed or stale flag

- **Where.** `margincode_pack::Packer::records_at` (`GET /v1/tasks/{id}/records/at`, the
  pre-read hook's answer), `crates/margincode-pack/src/packer.rs`.
- **Seen.** It renders each record with
  `let (id, texts) = record_texts(rec, false, false, origins.whence(rec, task.id));`, so the
  `stale` and `changed` arguments are always false: a record the pack prints with
  `(changed since recorded)` or `(anchor_stale)` is shown in the pre-read answer without either
  flag. The pack's own calls pass the computed values (`record_texts(rec, stale, changed, …)` in
  `assemble.rs`).
- **Ruled out.** Nothing yet.
- **Guess.** The flags were not wired into `records_at` when it was written; not checked in its
  history.
- **Status.** Open.

## 2026-11-20: the MCP `record` tool still refuses a summary over 200 characters

- **Where.** `margincode-daemon` `mcp.rs`, `check_sizes`, for the `record` tool (and `submit`'s
  inline records over MCP).
- **Seen.** In Claude Code sessions with the av hooks (release 0.0.2), a first
  `av record --json -` was refused for a summary over 200 characters and resent. `av record`, `av submit --record` and `av review --finding` now fit such a summary into
  a headline and keep the whole text in the body. The MCP tool was left alone: its description
  states the limit up front, and its refusal (`-32602`) names the length.
- **Ruled out.** Nothing: whether MCP agents send overlong summaries and retry on the refusal
  has not been measured.
- **Guess.** If they do, the same `margincode_core::fit_summary` could serve the tool.
- **Status.** Open.

## 2026-11-20: the dreamed-arm bench test passes in 0 s without `av` or `jq`

- **Where.** `margincode-bench` `tests/runner.rs`,
  `runner::a_frozen_dream_is_replayed_into_a_session_of_the_dreamed_arm`, run 40 times in a loop
  to check #88 (the loop #13 used to show its fix), in a `rust:1.97.1` container.
- **Seen.** The first loop of 40 passed 40 of 40, each run in 0.00 s. The test prints
  `skip: no av binary beside the test binary (cargo build -p margincode-cli)` or `skip: jq not
  installed` and returns, so cargo reports it as passed. Neither line shows without
  `--nocapture`, and the container had neither. With `cargo build -p margincode-cli` first and
  jq 1.7 on `PATH`, each run took about 19 s and 40 of 40 passed.
- **Ruled out.** Not a slow or flaky test: it never ran.
- **Guess.** None needed. A loop that means to exercise it needs both, and nothing in the bar's
  output says it skipped. Marking it `#[ignore]` with its needs in the reason, or failing when an
  environment variable asks for it, would make a skip visible.
- **Status.** Open.

## 2026-11-20: a session-start pack showed a disputed record without its reason, and another not at all

- **Where.** A live run of `av dream` with real models on a small Python repository (`tallyho`,
  one commit), av 0.0.3, no semantic layer, first packs at the default 800-token budget. Probe tasks were started
  after records were seeded and after verdicts on them took effect.
- **Seen.**
  - Goal "Fix Money.format in tallyho/money.py for JPY": `tokens: 710/800`, 20 items omitted.
    The disputed decision was printed as
    `- rec:… decision, abandoned: Every currency tallyho supports has two decimals. Plain amounts are mi… (disputed)`,
    without the reason or the `vd:` id. The pre-read answer for the same file (`records_at`)
    printed `(disputed: …, vd:…)`.
  - Goal "Fix slugify in tallyho/text.py for accented product names": `tokens: 488/800`, a Code
    section led by `slugify`, and no "Decisions & dead ends" section. It was the same before any
    verdict existed and after a disputed verdict on the decision anchored to `slugify` took effect.
    The pre-read answer for `text.py` listed that decision.
- **Ruled out.** Both are the packer working as written, not a rendering gap, and neither comes
  from dreaming.
  - The handle tier of every record line is its summary clipped to 70 characters plus a short
    flag. A disputed record's flag is `(disputed)`, and its summary and full tiers add the reason
    and the verdict (`record_texts`). A disputed record also ranks ×0.3 (`standing`). With 20
    items left out, it got the handle tier.
  - `av expand rec:…` prints the full tier: the reason and the `vd:` id. The pack's footer says to
    expand an id whenever it leaves items out.
  - `slugify` in plain words is not a named identifier (`lexical::named_identifiers`: backticks,
    `name(`, snake_case, camelCase or a dotted path), and a file name is left to full-text search.
    So the goal seeded nothing. Without seeds, `records_anchored_to` gets no paths or names, and
    the best text hits stand in for seeds only to bring in their neighbours (`expand_best_hits`),
    not their records. Without a semantic layer no record arrives as a hit either.
    `Money.format` is a dotted name, so that goal seeded it and got its records.
- **Pinned.** The pack test
  `a_function_the_goal_names_only_as_a_plain_word_brings_in_none_of_its_records` (and the same
  goal in backticks brings them in). The unit test
  `a_disputed_records_handle_says_disputed_and_its_longer_tiers_say_why`. The daemon test
  `expanding_a_disputed_record_gives_the_reason_and_the_verdict`.
- **Guess.** An open question, not a defect: whether the records anchored on the best text hits
  of a cold start should join as the hits' neighbours do. That would change first packs for
  existing inputs, and the measured results on how a dreamed pack reads rest on them, so it
  needs its own measurement first.
- **Status.** Works as designed; pinned by the tests named above.

## 2026-11-19: Antigravity's hooks need their own output

- **Where.** `av init --for antigravity` (`integrations/antigravity/README.md`, "Hooks"), against
  the hook contract in the docs shipped with the Antigravity CLI (`hooks.md`).
- **Seen.** Antigravity runs hooks from `.agents/hooks.json` with JSON on stdin and reads JSON on
  stdout, and none of av's four hook commands prints what it reads. `SessionStart` wants
  `{"injectSteps": [{"systemMessage": {"systemMessage": "…"}}]}` and `av hook session-start` prints
  plain text. `PreToolUse` needs a permission `decision`, with a `reason` as its only text, where
  `av hook pre-read` prints `additionalContext` and decides nothing. `PostToolUse` expects `{}`. `Stop` carries on only on
  `"decision": "continue"`, and `av hook stop` prints `"block"`, so its record nudge and its wait
  for work would be counted and then ignored. The input names differ too: `conversationId` for
  `session_id`, `continuationActive` for `stop_hook_active`, `toolCall.args` for `tool_input`
  (the docs do not give its file tools' names or argument keys), and no `source`. On Windows a
  hook runs under `cmd /c`, where av's `command -v av …; exit 0` guard is not valid; the default
  timeout is 30 s against the Stop hook's 600 s wait.
- **Ruled out.** A field mapping alone: every event's output differs, so each command would need
  a second output form.
- **Guess.** An Antigravity mode for `av hook` (detected from `hookEventName` in the input, or a
  flag) that renames the input fields and prints `injectSteps`, `{}` or `continue` would cover
  `SessionStart`, `PostToolUse` and `Stop`; `pre-read` could hand over its records only in a
  `reason` beside a decision it would have to make for the user. Not built
  because nothing here can run `agy` to check it.
- **Status.** Open. `av init --for antigravity` installs no hooks.

## 2026-11-19: a Workers Builds preview failed with "preview already exists"

- **Where.** The Workers Builds check on #79 (branch `av/gate-per-task`), Cloudflare's preview
  build for that branch.
- **Seen.** The build failed at `npx wrangler preview` with an error that the preview already
  exists. From the builds API, the Git integration had created that branch's preview 0.16 s after
  the branch's preview trigger was made, before the build's own `wrangler preview` ran. A new build
  started on the same commit through the trigger (`POST .../builds/triggers/{trigger}/builds`)
  passed; the retry route for the failed build answered 404.
- **Ruled out.** The change: #79 touched only a test, a README and this file, and the new build of
  the same commit passed.
- **Guess.** A race in Cloudflare's own setup between the integration's preview and the build
  step's.
- **Seen again.** #84 (branch `av/antigravity-init`), 2026-11-20: the same error, code 100338,
  ("A preview with name 'av/antigravity-init' already exists"), 40 s after the build started; a
  new build of the same commit from the branch's trigger passed and the PR was merged after it.
  The previews of #78 and #80 to #83 passed on their first build.
- **Seen again.** #93 and #94, 2026-11-21: the Workers Builds check failed on both branches. Both
  were merged before it had passed, against this entry's own advice: #93 while no check had
  reported, #94 while the state was unstable. The production builds of `main` after each merge
  (`5b9bb65`, `6945ee0`) passed. The branch builds' logs were not read, so it is not confirmed that
  the cause was the same "already exists" race.
- **Status.** Open, seen twice, and two more failed checks whose cause is not confirmed. When it
  shows, start a new build from the trigger rather than merging on a red check.

## 2026-11-19: reclaim retried every tick on a workspace it could not delete

- **Where.** The dogfood daemon (Docker image, user 1000, workspaces on a bind mount), in
  `disk.rs`'s reclaim tick.
- **Seen.** `WARN reclaiming a workspace failed task=... error=vcs error: Permission denied (os
  error 13)` for three ended tasks, once a minute each (`RECLAIM_TICK`), 2,316 lines in a day.
  By the code a gc's answer lists such a task under `errors` (`av gc` was not run); the log was
  the only place it was noticed. Each workspace held `bench/tasks-routing/repo` owned by root,
  mode 755, with a `.git` inside. Removing that one directory from the host let the next tick
  reclaim all three (15 MB).
- **Ruled out.** The files the agent edited from the host: they also show as root-owned through
  the bind mount, but with mode 777 inside directories the daemon owns, and they delete fine.
- **Guess.** `margincode-bench`'s `tests/routing_tasks.rs` runs `make-repo.py`, which builds that
  repository inside the source tree when it is missing. A `cargo test` run as root in a container
  with the workspace mounted (how the bar ran before rule 12 moved it to a volume) left it
  root-owned, and the daemon's user cannot delete inside it. Two things could change: the test
  could build into a temporary directory instead of the checkout, and the tick could say once,
  not every minute, that a workspace cannot be reclaimed and why.
- **Status.** The tick's half is fixed in #74: a workspace the tick failed to remove is left for
  an hour (`RECLAIM_RETRY`) before it is tried again, so it warns once an hour, and `av gc` still
  tries it at once and lists the error. The test's half is fixed in this change:
  `routing_tasks.rs` builds the pilot repository in a temporary directory, so `cargo test` writes
  nothing into the checkout. `margincode-bench check` still builds `bench/tasks-routing/repo`
  when it is missing, because the task files point there.

## 2026-11-18: the deadline-kill test needs the shell to print inside 400 ms

- **Where.** `margincode-agentrun` `tests/command.rs`
  `a_session_past_its_deadline_is_killed_and_reads_as_truncated`, in a full workspace test run
  under load (first written down in the Commercial edition's backlog).
- **Seen.** Failed once: `assertion left == right failed: no result event: the message sum is
  kept`. Passed when run alone. The same run's output had `kill: failed to parse argument: ''`.
  Not reproduced in 30 runs (each test) with every core busy. Reproduced with `sh` on `PATH`
  replaced by a shim that waits a random 0 to 600 ms before starting the shell: in 30 runs the
  test failed 15 times with that message (`left: 0`), and its sibling below 7 times (no pid
  file). The empty-pid-file case behind the `kill` line was not reproduced: its window is the
  time between the redirect and the `echo`.
- **Ruled out.** The production kill: `run_until` kills `-<child pid>`, never an empty argument,
  and `av down` (`local.rs`) passes an empty pid only to `alive`, whose stderr is discarded. The
  shell's builtin `kill` says `Illegal number` instead; the `failed to parse argument` wording is
  the `kill` binary, started from Rust with an empty argument.
- **Guess.** The script echoes two lines and sleeps, with a 400 ms deadline; on a loaded host the
  shell had not printed both when the deadline killed it. The `kill` line is its sibling
  `tests/process.rs` `a_child_past_its_deadline_is_killed_with_everything_it_started` (500 ms
  deadline): a kill between `> grandchild.pid` and the `echo` leaves the file empty, so
  `kill -0 ''` fails at once and the test passes without checking the grandchild; a kill before
  the redirect leaves no file and fails the test. Both upper bounds on time break
  `CONTRIBUTING.md` ("Timing").
- **Status.** Fixed in this change: both tests run their script with a deadline from 400 ms,
  doubling, until the script has printed what the test needs (bounded within `HANG`), and assert
  killed on every try. With the same shim, both passed 30 of 30.

## 2026-11-18: the post-edit debounce test needs three CLI calls inside 2 s

- **Where.** `margincode-cli` `tests/cli.rs` `hook_post_edit_debounces_within_two_seconds`, found
  reading every timing bound in `crates/*/tests` for the timing-flake change.
- **Seen.** Not a failure yet. The check "second edit within 2 s is debounced" holds only if the
  first `av hook post-edit`, an `av status` and the second `av hook post-edit` (three
  subprocesses, each talking to the daemon) finish inside `DEBOUNCE_MS` (2,000 ms). On a loaded
  machine the second hook can run after the window and checkpoint, and the test then sees
  `changed (2)` where it expects `changed (1)`. It is an upper bound on time, which the rule in
  `CONTRIBUTING.md` ("Timing") forbids; the 2.1 s sleep after it is a sleep past the window and
  is fine.
- **Ruled out.** The other sleeps in that file: the 1.5 s one in
  `validate_waits_for_a_free_build_slot_on_the_host` checks that nothing ran, which a slow
  machine can only make pass.
- **Guess.** The stamp lives in `.av-task` (`last_checkpoint_ms`). The test could write a fresh
  stamp into the task file before the second edit instead of relying on the first hook's, and an
  old one (more than 2 s back) instead of sleeping 2.1 s.
- **Status.** Fixed in this change: the test writes `last_checkpoint_ms` an hour ahead before the
  in-window edit and 2.1 s back before the last one, so neither half depends on how long a call
  takes, and the 2.1 s sleep is gone. It still checks that the first hook writes the stamp.

## 2026-11-18: a backup in backoff keeps its backoff across a daemon restart

- **Where.** `mark_backups_due` (`crates/margincode-store/src/backup.rs`), called by
  `start_backups` (`crates/margincode-daemon/src/backup.rs`) at start-up; seen checking SSH
  backups on the compose stack from `main` (105f99f).
- **Seen.** About 13 s after a daemon restart, the backup status still showed the failure from
  before the restart, with the next retry about 100 s away. Both doc comments say start-up makes
  every configured backup due, but the update is `SET next_retry = now() WHERE remote IS NOT NULL
  AND next_retry IS NULL`: a repo at rest (`next_retry` NULL after a success) is made due, and a
  repo in backoff (`next_retry` set) keeps its wait.
- **Ruled out.** A lost retry: the row stays due later, and backoff is capped at 300 s
  (`MAX_BACKOFF`), so the push comes at most 5 min late. It looks harmless.
- **Guess.** Not known which is meant: the comments, or the code (keep a backoff across
  restarts). Either the comments or the `WHERE` should change.
- **Status.** Fixed in this change. The owner chose the comments' behaviour (a restart is usually
  someone fixing the remote): start-up now makes every configured backup due, one in backoff
  too. Only the next retry moves; the failure count and last error stay, so the status still
  shows them and a remote that is still down goes on backing off from where it was.

## 2026-11-18: two result files were stored with CRLF line endings

- **Where.** `bench/results/2026-11-17-f4-storage/baseline.jsonl` and `volume-checkout.jsonl`
  (#46), found by the Cloud's vendor sync to `d0102c0`.
- **Seen.** Both blobs on `main` had CRLF endings (6 lines each). `.gitattributes` asks for LF,
  so the Cloud's `git add` stored them LF, and `scripts/sync-margincode.sh --check` then found the
  vendored tree's sha256 different from the one it recorded.
- **Ruled out.** Not the sync script: `git archive` of `main` gives the CRLF bytes, and they are
  the only two blobs on `main` with a CR.
- **Guess.** They were written on Windows and committed through `av` (jj), which does not apply
  `.gitattributes`, so nothing normalized them.
- **Status.** Fixed in #64 (both stored LF). `av diff --check` now reports an added CRLF line
  (branch `claude/cloud-capabilities-offline-22d48v`); a commit that runs no check can still
  carry one.

## 2026-11-18: the patch route answered with the previous head

- **Where.** `GET /v1/tasks/{id}/patch?from=main`, called by hand right after `av validate` or
  `av checkpoint` returned, while preparing fix-plan PR 22 (F29) for review.
- **Seen.** On two occasions the patch did not hold the edit the checkpoint had just taken: it was
  the previous head's. The same request a few seconds later held it. `av validate` had already
  printed the new head.
- **Ruled out.** Nothing yet: not reproduced on purpose, and no test reads the patch right after a
  checkpoint.
- **Guess.** The route reads the task's head, or the workspace's files, before the checkpoint's
  write is visible to it.
- **Status.** Open. A wait of a few seconds before reading the patch has avoided it since.
  Not reproduced in the daemon itself (2026-11-18): a throwaway test in `tests/remote.rs` that
  edited a file, then called `checkpoint` or `validate` and at once `GET .../patch?from=main`,
  60 rounds, three runs, always got the new head (`X-Av-Head`) and the edit. Both routes write
  the head in one transaction before they answer. So the cause is more likely outside one
  daemon process's ordering: the client (which head it read the patch for), or the Windows bind
  mount the edits were made on (a write the snapshot did not see yet).

## 2026-11-18: the Docker image cannot push a backup over SSH

- **Where.** `deploy/Dockerfile` (the apt line), `deploy/docker-compose.yml` (the SSH key and
  `AV_BACKUP_SSH_COMMAND` comments), found reviewing fix-plan PR 24 (F7, F8, F19, F20).
- **Seen.** The image installs `git ca-certificates curl` with `--no-install-recommends`, and
  `ssh-client` is only a Recommends of Debian's `git`: `which ssh` finds nothing in the image. With
  `openssh-client` added, ssh still exits "No user exists for uid 4242" for a UID with no passwd
  entry, and compose runs the daemon as the host `${UID}` (its own comment says that UID has no
  passwd entry). So the compose file's SSH backup comments describe a setup that cannot work.
- **Ruled out.** Lite mode: the daemon runs on the host and uses the host's ssh.
- **Done for now.** `docs/reference.md` ("Getting main to GitHub (self-hosted)") sends the Docker
  stack to a bare repository on a mount and a push from the host.
- **Guess at a fix.** Add `openssh-client`, and give ssh a user it accepts (for example a passwd
  entry written at start, or `nss_wrapper`), then a test that pushes a backup over SSH from the
  image. Or drop the compose comments.
- **Status.** Fixed on `claude/cloud-capabilities-offline-22d48v`: the image installs
  `openssh-client` and `libnss-wrapper`, and its entrypoint (`deploy/entrypoint.sh`) gives a UID
  without a passwd entry one through nss_wrapper before it starts the daemon. Checked on the
  image's runtime layer (same base and packages, a stand-in daemon): as UID 4242, `ssh` without
  the entrypoint exits "No user exists for uid 4242"; with it, a `git push` over SSH to an sshd
  container succeeded. Checked 2026-11-18 on the real image from `main` (105f99f) and the compose
  stack as UID 4242: the smoke steps pass, the daemon pushes main to an sshd container over SSH
  after a merge, and without the entrypoint the push fails with "No user exists for uid 4242".
  Not checked: `scripts/smoke.sh` run as is on a Linux host.

## 2026-11-18: M2 stage B, three stops before the first session

- **Where.** M2 stage B (`bench/results/2026-11-13-standing-and-dreams/m2-stage-b/checks/`).
- **Seen.** (1) The first `dream-freeze` stopped at its first agent call: Claude Code answered
  "Invalid API key" and a direct models request returned 401; the owner replaced the key and the
  freeze ran again from a fresh seed ($0 spent). (2) The smoke's preflight refused twice:
  `first-packs` and the freeze leave run and workspace directories open to the agent user, and
  `useradd -m` gives `benchagent` a home it can write. Closed with `chmod 700` and
  `usermod -d /nonexistent benchagent`; `m2bchain.sh` now closes `ws/*` too between the arms.
- **Ruled out.** No session ran in either state; the harness refused before launching.
- **Guess.** (2) is setup for a new container: M0's container had been set up that way already.
- **Status.** Worked around. A harness recipe for a new container could name both steps.

## 2026-11-18: a two-second lease ran out inside a routing test under load

- **Where.** `margincode-daemon` `tests/routing.rs`
  `the_reviewer_who_requested_changes_does_not_rescue_the_task` (line 590), in the full
  `cargo test --workspace` run for fix-plan PR 21 (F6).
- **Seen.** Right after grok's `request_changes`, which renews the owner's lease, `next` for
  `codex` returned the task `from: abandoned` where the test expects nothing offered. Alone the
  test passed five runs out of five (5.9 s each).
- **Ruled out.** Not F6's change: F6 touches the merge message and the commit author, not
  leases, reviews or `next`.
- **Guess.** The test's lease is 2 s; with the whole workspace's tests running, the time from the
  renewal to the two `next` calls passed 2 s. A longer lease with the sleeps scaled to it would
  hold.
- **Status.** Fixed on `claude/cloud-capabilities-offline-22d48v`: the test's lease is 5 s and
  its sleeps 5.5 s, so a loaded run has 5 s from the renewal to the two `next` calls. Then fixed
  for the whole family by the pull request "Tests set state instead of waiting": the test runs on
  the default lease and `expire_lease` writes the lease lapsed where it slept, so no lease can run
  out by itself inside a test (the rule is in `CONTRIBUTING.md`, "Timing").

## 2026-11-18: the Workers Builds preview of #49 failed

- **Where.** The Cloudflare Workers Builds preview check on #49 (`constitution.md` and this file;
  build `63832ff8`).
- **Seen.** The preview build failed. `main`'s production build after #49 merged passed.
- **Ruled out.** Not the change's content, as far as can be told without the log: #49 touched only
  Markdown, and the same tree built on `main`.
- **Guess.** Something in the preview environment, not the repository. The build log was not read.
- **Status.** Answered (2026-11-19, the builds API): the build was cancelled, not failed. Each
  branch gets its own preview trigger; the merge deleted the branch, and Cloudflare stopped the
  build while `npx wrangler preview` was still installing wrangler, then deleted the trigger
  (about 35 s after it was made). #75's red check is the same. A red preview on a merged PR whose
  build reads "cancelled" says nothing about the change; wait for the preview before a merge to
  have it say something.

## 2026-11-18: which retrieval task lost a signature at 600 tokens

- **Where.** `margincode-bench` `tests/retrieval_gate.rs`, on #48 (F17).
- **Seen.** The pack's `symbols_sig` mean at budget 600 fell from 0.833 to 0.778 (half of one
  task's gold signatures, of 9), while at 300 it rose from 0.333 to 0.444 and `files_code` from
  0.444 to 0.556. The floors were re-set from the new means.
- **Ruled out.** Not the goals' length: the fixture's goals are one line each.
- **Guess.** The task handle is now reserved before the section shares, which moves a few tokens
  between sections at tight budgets.
- **Status.** Answered (2026-11-18, the gate with a per-task print at #48's parent `6721a56` and
  at `e515440`): at 600, `cache-key-order` lost its one signature of two (0.5 to 0; every other
  task unchanged, `token-skew` was already 0); at 300, `aging-due-today` gained its file and its
  signature (0 to 1). Why `cache-key-order`'s signature no longer fits at 600 is not looked at
  yet. A per-task line in the gate's report would answer the next such question at once.
  The gate prints that line (each task's scores under each budget's means) since this change.
  With it: at 600 on `main` the pack shows both of `cache-key-order`'s gold symbols, as signature
  lines without `export` (`function cacheKey(...)`, 501 of 600 tokens used), and `score` does not
  count them because it looks for the source line `export function ...`; so no section takes the
  tokens. `token-skew` (an exported TypeScript function, 0 at every budget) may be the same miss
  (guess, its pack not looked at).

## 2026-11-18: `av` on Windows does not read a Git Bash path

- **Where.** `av record --json FILE` in Git Bash with `av.exe`.
- **Seen.** `av: cannot read /c/Users/.../records.json: The system cannot find the path
  specified. (os error 3)`. The same file passed as a Windows path (`cygpath -w`) was read.
- **Guess.** `av.exe` is a Windows program and Git Bash converts paths only for arguments it
  recognises as paths; with `MSYS_NO_PATHCONV=1` (which `docs/reference.md` recommends for `/import/...`)
  it converts none.
- **Status.** Documented in #60 (F2, F13): `docs/reference.md` says Git Bash converts only
  arguments and that a Windows program needs `C:/...` (`cygpath -m`). `av` still reads such a path
  as given.

## 2026-11-17: a checkpoint during another task's start took 2.2 s once

- **Where.** The measurement for #46 (`bench/results/2026-11-17-f4-lock/`), on a throwaway daemon
  on the Windows bind mount.
- **Seen.** With the write outside the repo lock, seven of eight checkpoints during another start
  took 370-378 ms; one took 2,236 ms. On `main` before the change they took 4.7-9.7 s.
- **Ruled out.** Not reproduced in five more runs, or on the dogfood daemon (369-381 ms).
- **Guess.** The host's disk, or the snapshot meeting the start's index overlay.
- **Status.** Open.

## 2026-11-17: checkpoints after widening every archive take 4.2 s

- **Where.** `av widen` on a sparse workspace of this repository (#45).
- **Seen.** Widening one result directory took 14.9 s and checkpoints after it stayed under a
  second. Widening all of `bench/results/` made each checkpoint 4.2 s.
- **Guess.** The snapshot walks every widened file on disk; a path that a sparse pattern already
  covers could skip the walk.
- **Status.** Open.
