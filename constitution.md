# Constitution

The rules every change to this repository follows, whoever makes it: a person or an agent. The
mechanics (commands, test layout, migrations) are in `CLAUDE.md` and `CONTRIBUTING.md`; this file
says what those serve. When another document here disagrees with this one, this one wins. The
owner's own word wins over both.

## 1. Simple code

- Write the plainest code that passes the test. A reader should understand a change without a
  paragraph of explanation; if they need one, simplify the code, not the explanation.
- No abstraction before its second use. A new trait, generic parameter, macro or module needs two
  callers that exist today, not one that might come.
- Prefer a function to a framework, a loop to a combinator chain that needs a comment, and a
  concrete type to a `dyn` unless something outside the crate must swap it (`ext.rs`).
- Small diffs. Deleting code is a good change. Don't reshape code a change does not need.

## 2. Library docs from Context7

- Before using or changing a library's API (jj-lib, axum, sqlx, tokio, tree-sitter, clap, the
  Qdrant client, any crate), look it up in Context7 (`resolve-library-id`, then `query-docs`),
  for the version pinned in `Cargo.lock`. Don't write an API from memory.
- Where Context7 does not have that version (jj-lib is pinned at `=0.46.0`), read the crate's own
  source in the cargo registry. Never guess a signature.
- Say in the pull request which source you used when a change depends on library behaviour.

## 3. Comments only when they are needed

- Names, types and tests carry the meaning. A comment says a non-obvious *why*: a constraint, a
  measured number with its source, a workaround and the upstream bug it works around.
- Never a comment that says what the next line does, the history of a change (that is the commit
  message), or a TODO (that is the backlog, rule 5).
- Doc comments on public items stay short: what it is for and what a caller must know.

## 4. The board is updated before every pull request

- The owner keeps a progress board. Before a pull request is opened, update it: what changes, its
  status, and the pull request once it exists. A pull request whose board entry is stale is not
  ready.
- The local notes the owner keeps beside the repositories are updated in the same step.
- The board is private: its link is never written into this repository, a commit or a pull
  request.

## 5. Errors go to the backlog, with what was observed

- An error that is not fixed on the spot gets an entry in `docs/backlog.md` before the work moves
  on: a test that failed, a command that broke, a number that does not fit, a flaky run.
- Each entry has the date; where it happened (command, test, commit); what was seen, exactly
  (the message, the numbers); what was ruled out; the suspected cause, marked as a guess; and its
  status.
- Observations, not conclusions dressed as facts. "Failed once in eight runs, not reproduced" is
  an entry; "flaky, ignore" is not.
- A fixed entry stays, marked with the pull request that fixed it.
- No secrets, keys, tokens or local paths in an entry: this repository is public.

## 6. Tests first, and they bite

- Write the failing test, then the code (`CONTRIBUTING.md`). Run the new test against the code
  before the change: if it passes there, it does not test the change. Say so in the pull request.

## 7. Measure, don't assert

- A claim about speed, size or retrieval quality carries its numbers, how they were measured and
  where the results live (`bench/results/`). A run that does not fit (an outlier) is reported, not
  dropped.
- A floor or a threshold is changed only on purpose, with the old and new values and the reason.

## 8. The bar runs locally

- fmt, clippy with `-D warnings` and `cargo test --workspace`, green on the head being proposed,
  before a pull request. GitHub Actions is on a free tier: start a workflow only when the work
  cannot land without it, and then once.

## 9. Public text is clean

- Commits, pull requests, docs, the backlog and the changelog carry no secrets, keys, tokens,
  personal data or local paths. Pass a key by its variable's name, never its value.

## 10. Leave things as you found them

- Remove only what your own work created: containers, volumes, directories, branches. Deletes on
  disk go through the daemon's guarded paths (`under_ws_root`). Keep a way back (the previous
  image, a backup) until the new one is proven.

## 11. Plain words

- Docs, pull requests and messages say what happens in short sentences, with the numbers. Text an
  agent reads (packs, MCP tool descriptions, hook messages) says `av`, not `margincode`.
