# Changelog

## Unreleased

## 0.0.4 (2026-11-21)

The project is now Margincode (it was Scholion), and this is its first release from
`margincode-dev/open-margincode`; releases 0.0.1 to 0.0.3 stay in `vinhqbui/scholion`. Agents can
correct a stored record (`av record --supersedes`), and packs say when a record comes from
reverted, abandoned or open work. `av dream` and `av verdict` give records a second look: a run
proposes verdicts that take effect only once a verify run or a person agrees, and with `--promote`
a decision can become a convention. `av record` cuts an overlong summary to fit instead of
refusing it, and agents are asked for their records before their final message. `av init --for
antigravity` connects Google's Antigravity CLI, `av diff --check` attests a docs-only change,
`.av/sparse` keeps paths out of task workspaces (`av widen` brings them back), and `av backlog
requeue` puts an abandoned task's item back. Migrations 0031 and 0032 run on the first start.
From 0.0.3: the binaries are `margincode`, `margincode-daemon` and `av` (`scholion` and
`scholion-daemon` are no longer shipped); an existing `~/.scholion` is still used while
`~/.margincode` does not exist, and lite mode's database and the Compose project keep their
names, so an install keeps its data. The Claude Code plugin is `margincode@margincode`.
`AV_LISTEN` defaults to `127.0.0.1:7420`, and a daemon on a non-loopback address needs
`AV_TOKEN` or `AV_LISTEN_OPEN=1`.

- **Install paths point at this repository.** `scripts/install.sh`, the cargo-binstall metadata,
  the Homebrew tap, the Claude Code plugin command (`claude plugin marketplace add
  margincode-dev/open-margincode`) and the docs and site links name `margincode-dev/open-margincode`,
  and the Docker image is `ghcr.io/margincode-dev/open-margincode`. The installer reads only this
  repository, so `MARGINCODE_VERSION` must name a release made here (2026-11-21).
- **The README and site date the fewer-turns claim and add the tracker's later run.** The claim of fewer turns at the same cost is from the October 2026 benchmark (`bench/results/2026-10-27-haiku/`) and now says so in `README.md`, `docs/reference.md`, `site/index.html`, `site/llms.txt` and the agent-memory guide. Each also has a short note on the openevaluations tracker's run of 2026-11-20 (same four tasks and model), in which release 0.0.2 cost more than plain Claude Code ($0.0679 against $0.0540 for a leg of 12 sessions, too few for a claim), with a link to its report. `site/README.md` records the root domain's new state and lists the tracker report as a source (2026-11-21).
- **A file whose line endings changed no longer counts every symbol in it as changed.** A changed symbol is one that overlaps a line the task inserted, replaced or deleted, and lines were compared with their endings, so rewriting a file from CRLF to LF (or back) replaced every line. Adding `DEFAULT_PAGE_SIZE = 100` to a CRLF `scan.py` and saving it with LF endings listed `DEFAULT_PAGE_SIZE, count_records, iter_records`, and a session that edited only `apply_discounts`, twice, in a CRLF `discounts.py` was stopped over all 8 symbols of the file (its second edit rewriting the file's endings is the likely cause; the session's files were not kept). Now `\r\n` and `\n` end a line alike when lines are compared; trailing spaces or tabs, a lone `\r` and a missing final newline still count, and line numbers are unchanged. This applies everywhere the changed symbols are used: `GET /v1/tasks/{id}/unrecorded` (the Stop hook's block), `av submit`'s advisory and the submit and handoff refusals, the pack's seeds from the task's own changes, and the handoff and review bundles, where a file whose endings alone changed is now listed as `(changed)` with no symbol under it (2026-11-21).
- **`av record` fits an overlong summary, and agents are asked for records before their final message.** In Claude Code sessions with the av hooks (release 0.0.2), the Stop hook's request for records cost extra model calls: it came after the agent's final answer, the first `av record --json -` was refused for a summary over 200 characters, and the agent then resent the record and wrote its answer again. `av record` (`--summary` and `--json`), `av submit --record` and `av review --finding` now cut a summary over 200 characters to a headline (at the last sentence end in its second half, else at the last word with `…`) and put the whole text at the start of the body, before the body given; one line on stderr says so, and the record is stored. The daemon's API and the MCP `record` tool still refuse such a summary. `av hook session-start` prints one more line after the pack, asking for the records in one `av record --json -` call before the final message, anchoring each changed symbol and its file's bare path (the symbol ranks the record for a later task on that code, the path covers the rest of the file for the Stop hook's check), and the Stop hook's request, no longer than before, asks the agent to end its turn in a line without a recap (2026-11-21).
- **Another agent may accept or reject a `promote`.** `av verdict accept|reject vd:…` under a registered agent profile's name or alias is no longer refused: a promote is now refused only to the agent that proposed it (the verdict's author or the proposing run's agent, in any letter case) and, as before, to a verify run. `av verdict list` and `show` print a promote as `(waits for a review)`, `(accepted)` or `(rejected)` instead of naming a person. The `promote` section `av dream --promote` appends to `DREAM.md` now says another agent or a person decides, so a `--promote` run's `instructions_sha` differs from one made before; a run without `--promote` is handed `DREAM.md` and its hash as before. Stored runs keep the hash of what they were handed (2026-11-20).
- **A judged record stops leading the dream worklist for its failed origin.** The review worklist gave a record 3 for a reverted merge or a decision of abandoned work whatever its verdicts, so a decision already disputed and effective scored 3.8 in a live run and was handed again ahead of a never-judged note at 1.0, taking half of a `--limit 2` run to dispute it a second time. The 3 now counts only while no review has agreed on a `confirmed` or `disputed` verdict for the record; a rejected or retracted verdict leaves it. Changed anchored code, a shared anchor and the packs that showed it still count, so a judged record comes back when its code changes or packs keep showing it. Declined records wait as before (2026-11-20).
- **`av init --for antigravity`, and an empty config file is no longer an error.** Google's Antigravity CLI (`agy`) reads MCP servers from `~/.gemini/config/mcp_config.json`, which it ships as a 0-byte file, and `av init --for mcp-json --path` on it failed with "not valid JSON". `av init` now reads an empty or blank JSON file as `{}`, for every agent's config and hooks file, and `--for antigravity` merges `mcpServers.av` with the `/mcp` `url` and `"headers": {"X-Av-Agent": "antigravity"}` into that file, keeping the rest and a `.bak`; the header names the session `antigravity` for routing. With `AV_TOKEN` set it also writes `"Authorization": "Bearer <the token>"`: Antigravity's docs give no environment-variable expansion there, so the token itself is written, as `claude mcp add --header` does for Claude Code; on Unix the file and its `.bak` are then made readable by their owner alone, and `av init` says to run it again when the token changes. The instructions go into the repository's `AGENTS.md` or `GEMINI.md`. No hooks: each of Antigravity's hook events reads stdout in a form `av hook` does not print (`integrations/antigravity/README.md`, `docs/backlog.md`). From `agy mcp add --help` (1.5.13, which has `--header "Key: Value"`) and the docs shipped with the CLI; not yet run against a real `agy` (2026-11-20).
- **A daemon restart pushes a backup in backoff at once.** Start-up made a backup at rest due but left one waiting out a failure's backoff (up to 300 s) to wait on, though the docs said every configured backup is due at start-up. Now every backup with a remote is pushed right after a restart, as a restart is usually someone fixing the remote. Its failure count and last error stay until a push succeeds, so `av repo backup` still shows them, and a remote still down backs off from where it was (2026-11-19).
- **The reclaim tick stops retrying a workspace it cannot delete every minute.** When the daemon could not remove a finished task's workspace (on the dogfood host, a root-owned directory inside it), the 60 s tick tried again and logged `reclaiming a workspace failed` on every pass: 2,316 lines in a day for three workspaces. The tick now leaves such a workspace for an hour after a failure, then tries again, so it warns once an hour and walks it no more often. `av gc` still tries it at once and lists the error under `errors`. The record of failures is in memory, so a restart tries every workspace again (2026-11-19).
- **`av watch` ends its command when it is interrupted.** A SIGINT or SIGTERM to `av watch` alone (`kill`, a supervisor stopping it) ended `av` and left the command it ran going on (and spending). Now the command runs in a process group of its own, as `av dream`'s agent does: SIGINT, SIGTERM, SIGHUP or SIGQUIT while it runs kills that whole group and ends `av watch`, `--once` or not, and `av` then dies of that signal; a signal that lands as the command ends cleanly starts no further run. A signal `av` was started ignoring stays ignored. Since the command is no longer in the terminal's foreground group, a terminal's Ctrl-C reaches only `av`, which ends the command with SIGKILL, and a command that reads the terminal is stopped by it; `av watch` is for headless agents. Windows is unchanged (2026-11-19).
- **The `promote` verdict: a dream can propose a decision as a repository convention, and another agent or a person decides.** Plan 16 lists five verdicts and four were built. A review run may now answer a `decision` that states a rule for a set of paths with `{"verdict": "promote", "text", "scope_glob", evidence}`, quoting the decision; it is refused on another kind of record, on a superseded one, on one that already has a promote waiting or accepted, with an empty text or one over 400 characters, or with a glob `av convention add` would refuse. The agent is told about it only in a run asked for with `av dream --promote` (review runs only), which appends a section to that run's `DREAM.md`; a run without it gets `DREAM.md` and its hash as before. Conventions are never dropped from a pack, so only an acceptance gives it effect: one `conventions` row with `source = vd:<id>`. A verify run is never handed a promote and its review of one is refused, and so is a review under, in any letter case, the verdict's author or the proposing run's agent (the defaults do not collide: `av dream` runs as `agent`, `av verdict accept` reviews as `user`); any other name may accept or reject it. Community has one shared token, so that is a rule about names, not authentication; a deployment that must know who decided enforces it in its `MergeGuard` (the Commercial edition's guard). Two acceptances at once leave one review and one convention, a retraction racing an acceptance leaves none live, and two proposals of one record at once leave one verdict. A decision superseded after its promote was accepted keeps its convention until `av verdict retract` or `av convention rm` takes it out. Retracting the verdict or undoing its run removes that convention and no hand-written one. Store migration 0032 adds the verdict's text and glob (2026-11-18).
- **Ctrl-C ends a dream's agent too.** `av dream` starts the agent command in a process group of its own, so a Ctrl-C reached only `av`: it exited and the agent went on running (and spending), and with `--every` nothing was left to stop it. Now SIGINT, SIGTERM, SIGHUP (a closed terminal) or SIGQUIT while the agent runs kills its whole process group, ends the run as failed and ends `av dream`, the `--every` loop included, and `av` then dies of that signal, so its parent (a shell loop, a supervisor) sees it; a signal that lands as the agent ends cleanly starts no further fix round or run. A signal `av` was started ignoring stays ignored, and each run puts back the dispositions it found. `--budget-usd` is refused beside a command that carries its own `--max-budget-usd`, and `--also-read` refuses a directory `also-read.txt` or `AV_DREAM_ALSO_READ` cannot name (not UTF-8, or holding `:`) (2026-11-18).
- **`av dream --budget-usd`, `--also-read DIR` and `--every D`.** Plan 16 promised the three and `av dream` had none: a cap had to be written into the agent command by hand, a dreaming agent had no way to be told where the user's own agent transcripts are, and a schedule needed cron. `--budget-usd X` appends `--max-budget-usd X` to the agent command in every round (for a CLI that takes it, as Claude Code does; Community still stores no dollars). `--also-read DIR`, repeatable, checks each directory before a run is asked for and hands the absolute paths to the agent as `also-read.txt` in the run's directory, `AV_DREAM_ALSO_READ`, and a short section at the end of that run's `DREAM.md` saying they are context, not evidence; `av` reads nothing under them and sends the daemon nothing of them, and a run without the flag gets `DREAM.md` and its `instructions_sha` byte for byte as before. `--every D` (`30s`, `90m`, `24h`, `1d`) runs one mode again every `D` from each run's start, never two at once; a failed run (the agent, a held area, a daemon out of reach) is printed and the loop goes on until Ctrl-C (2026-11-18).
- **The README and site carry the E4-at-scale result.** With 500 unrelated notes next to the ten that matter (DeepSeek Flash, 10 tasks × 3 runs), cost per accepted task was $0.0143 with av against $0.0237 with a flat `NOTES.md` (39.9 % less). Both solved 30/30, so it is a cost result (`bench/results/2026-11-e4-scale/`). The site's figure for the first, MCP-based relay run is corrected from 22 % to 24.5 % more than notes, and `docs/reference.md` no longer says E3 took 14 % less wall time: the shipped run measured 12.9 % fewer turns and 2.3 % less wall time, the second inside its interval (`bench/results/2026-10-27-haiku/`). Docs only.
- **The Docker image can push a backup over SSH.** It had no `ssh` (only a Recommends of `git`), and compose runs the daemon as the host's UID, for which OpenSSH refuses to run ("No user exists for uid ..."). The image now installs `openssh-client` and `libnss-wrapper`, and its entrypoint gives such a UID a passwd entry through nss_wrapper (the image's `/etc/passwd` is unchanged), so `AV_BACKUP_REMOTES` with an SSH URL and `AV_BACKUP_SSH_COMMAND` work as `deploy/docker-compose.yml` describes (2026-11-18).
- **`av diff --check` reports a CRLF line ending.** It read the diff by lines, which dropped the CR before each newline, so a file written on Windows passed. A jj commit applies no `.gitattributes`, so two result files reached `main` with CRLF endings (#64). An added line that ends in a CR is now reported as `path:line: CRLF line ending` (2026-11-18).
- **`DREAM.md`: a record's advice is judged with its fact, and code that breaks a rule does not dispute it.** In Plan 16's M2 stages A and B, one wrong note was `confirmed` in all nine runs it was handed in (six stage A passes, three stage B stores), reviewer and verifier agreeing: its fact was true (a money helper's parts can differ by a cent) and its advice (do not use the helper for splits) was the dead end. Once, a true lesson was `disputed` because the code on `main` still had the bug the lesson was written against. The judging rules now sit in one section that both `review` and `verify` read. A record that recommends one way over another is `confirmed` only when the advice holds too, and is disputed when the rejected way's name, docstring or tests show it is there for that job. A record that says what the code does is still disputed by code that does not do it; one that states a rule is disputed only by a docstring, a test, a later outcome or code that works only by the opposite, not by code that breaks it. The instructions' hash (`instructions_sha`) changes, so runs before and after can be told apart (2026-11-18).
- **`av init --source PATH`, and a hint when Git Bash rewrote a path.** `av init` imported the local git root, a path a daemon in Docker cannot read, so a Docker install had to run `av repo add` first. `--source PATH` gives the path as the daemon sees it (`/import/NAME`). Git Bash rewrites an argument such as `/import/NAME` into a path under its install directory before `av` sees it; when the daemon refuses such a source, `av repo add` and `av init` now add a line naming the path that was meant and the way around it (`//import/NAME` or `MSYS_NO_PATHCONV=1`). From the first dogfood fix plan (F3, F2, F13; 2026-11-18).
- **`av diff --check`.** Approval needs a passing `av validate` at the head, and a docs-only task had nothing to run: a task workspace is jj, with no `.git` for `git diff --check` to read. `av diff --check` reads the task's change since its base from the daemon (as of its last checkpoint; `av validate` checkpoints first) and reports, on the lines the task added, trailing whitespace, leftover conflict markers (labelled ones, as the submit check reads them) and a file left without a final newline, as `path:line: what`, exiting 2 when it finds any. So `av validate -- av diff --check` attests a docs change. From the first dogfood fix plan (F9; 2026-11-18).
- **`av validate` warns about an unguarded pipeline.** A pipeline's exit status is its last command's, so `av validate -- sh -c 'cargo test | tail'` attested a pass when the tests failed, and a convention's `--check` written with a pipe had the same hole. `av validate` (for a script argument, one holding whitespace, and for each check it runs) and `av convention add` (for the check) now warn when the script pipes outside quotes and no argument says `pipefail`. It is a warning, not a refusal: `set -o pipefail; cargo test | tee log` is fine, and the parse is a heuristic. From the second dogfood fix plan (F24; 2026-11-18).
- **`av notes` no longer publishes home directories, or failed checks the task moved past.** A check's line is the validated command as it ran, so `av notes --pr` posted paths such as `/home/NAME/...` or `C:\Users\NAME\...` on the pull request, and a failed run stayed listed after a later fix passed. The block now writes a home directory in its common forms as `~` (`/home/NAME`, `/var/home/NAME`, `/Users/NAME`, `C:\Users\NAME` in any case, `C:/Users/NAME`, `/c/Users/NAME`, `/mnt/c/Users/NAME`, `\\wsl$\DISTRO\home\NAME`), by pattern, so it works whichever machine validated; it is not a guarantee against every private path. It also leaves out a check that failed at an older head once the same command has passed at the task's head; every check at the head stays, failed ones too, and so does a failure whose command has not passed there. `--format json` lists the same records the block shows. The stored records are unchanged. From the second dogfood fix plan (F29; 2026-11-18).
- **How a self-hosted install gets main to GitHub, and a merge gate without Actions.** Nothing in `docs/reference.md` told a self-hosted user how merged work reaches a GitHub pull request; the rolling-PR workflow existed only in this repository's `.github/`. `docs/reference.md` gains "Getting main to GitHub (self-hosted)": a backup branch `av-main` (pushed by the daemon itself in lite mode; with the Docker stack, over SSH with a mounted key or to a bare repository on a mount and on from the host), one pull request per task or one rolling pull request, merge commits only, `av repo advance` right after each merge on GitHub (and why it can answer `diverged` for a while), and a required convention with a check as the gate when GitHub Actions is off. `integrations/github/av-main-pr.yml` is the rolling-PR workflow to copy. Docs and a workflow file only. From the dogfood fix plan (F7, F8, F19, F20; 2026-11-18).
- **Merge commits name who did the work, and `AV_COMMIT_AUTHOR` sets their author.** Every commit on `main` was authored `Margincode <margincode@localhost>`, written into the daemon's jj settings, and the merge message did not say which agent wrote, reviewed or merged the task. A merge message now ends with git trailers after the `av task <id>` line: `Av-Owner` (the task's owner), `Av-Reviewer` (the approval at the merged head; for an approval `av` carried across a rebase, the person or agent who gave it) and `Av-Merged-By`. A control character in a name becomes a space, so a name cannot add a line such as a second `av task` marker. Notes mined from git history drop these trailers. The daemon's new `AV_COMMIT_AUTHOR` (`Name <email>`) is the author and committer of every commit it writes; unset, it stays `Margincode <margincode@localhost>`. It is one value per daemon: a per-task author would change the `Vcs` trait. There is no `Co-authored-by`, because profiles have no e-mail, and no signing, because the daemon holds no key. `margincode_daemon::Config` gains `commit_author`, and `JjVcs` gains `with_author`; `JjVcs::new` and the `Vcs` trait are unchanged. From the dogfood fix plan (F6; 2026-11-18).
- **`AV_PLAIN_RECORDS`: a daemon can serve records without correction and origin standing.** Plan 16's M2 stage B compares raw and dreamed stores on an arm with room to move, and the arm that had it in M0 (`av-before`) was built before dream verdicts existed. With `AV_PLAIN_RECORDS=on` the daemon turns off what Phases 0 and 1 give an agent: packs and `records/at` print no line inviting `av record --supersedes`, superseding another task's record is refused (422; a task still supersedes its own), and a record's rank and line ignore what became of its task (no `reverted`/`abandoned`/`open` label, no ×0.4/×0.6), as do record lists (no `standing`). Phase 2's verdicts still act: a disputed record is ×0.3 and marked, and a duplicate or outdated one is hidden. `GET /v1/version` says `plain_records`; `margincode-bench run` asks the daemon, writes it into `meta.json`, prints it in the report, and refuses to resume results of the other setting. Off by default: nothing changes. `margincode_daemon::Config` gains `plain_records`, so code that builds it field by field adds one line. PACKER_VERSION stays 11, so a receipt does not tell the two apart; `meta.json` does (2026-11-18).
- **The merge commit follows a restated intent.** A merge wrote the task's goal as first given to `av task start` into the commit message, so after a redesign the public commit described the first design (on the dogfood host, a task three designs on still said "the socket stays open"). Packs already showed the newest intent. Now, once the intent made from the goal is superseded (`av record --kind intent --supersedes rec:…`), the merge writes the live intent, its summary and then its body, in the goal's place, and `av notes` quotes the same text. The title and the `av task <id>` line are unchanged; a task that never restated its intent gets the goal as before. If the design changed, restate the intent before you submit. From the dogfood fix plan (F36; 2026-11-18).
- **The first pack always prints the goal, and the repo map lists every top-level directory.** The Task section gets 10 % of the usable budget, 72 tokens at the default 800, so a goal longer than a line did not fit: the task item stayed at its title, and the goal came in only when the other sections left room. On the dogfood host a task's first pack had its title and no goal. The task item is now kept like a required convention, and its shortest form carries the goal, clipped at 1,200 characters with a pointer to `av expand task:SLUG`, which prints the rest. The full repo map listed children under one cap of 80 lines in alphabetical order, so a large `bench/` used it up and `crates/` and `docs/` were never listed; it now lists at most 8 children per top-level directory, then `… +N more`. Pack text changes for every agent. From the dogfood fix plan (F17; 2026-11-18).
- **The Docker stack caps build output, and builds in review go first.** `deploy/docker-compose.yml` now sets `AV_BUILD_CACHE_MAX_GB` to 40 (GiB per repo, about nine full builds of this repository at 4.3 GB each); it was unset, so the budget was off and task builds were only removed at merge or abandon. Set it to `0` to turn the budget off again; the daemon's own default (lite mode) stays `0`. Over the budget, an idle `target/` of a task in review is now removed before an idle open task's, since its owner waits on the reviewer; a `target/` touched within `AV_BUILD_IDLE_SECS` is still never removed. From the dogfood fix plan (F4; 2026-11-17).
- **A start no longer holds up other tasks of the repo.** A task start wrote its workspace's files while holding the repo's lock, so every other task's checkpoint, sync or merge in that repo waited for the copy: on the dogfood host a checkpoint took 4.8 to 5.3 s during another start, against 0.35 s alone. The lock now covers only what changes the repo; the files of a new workspace, a restored one (`av undo`, `av revert`) and an `av widen` are written after it is released. From the dogfood fix plan (F4; 2026-11-17).
- **`av widen PATH`, and packs say what is not on disk.** A sparse task workspace (`.av/sparse`) left its listed paths unreachable: an agent could only read them with `av show`. `av widen PATH...` (`POST /v1/tasks/{id}/widen`) writes listed files or directories to the workspace, keeps any the agent already wrote, and they stay widened: edits and deletions there are recorded and what `main` adds under them is written on `av sync`. The pack's Task section now says `Not on disk: ...` with both commands, read from the working copy itself, so a full workspace shows nothing (a task worked in a checkout still shows its daemon workspace's line, though the checkout holds every file). `Vcs` gains `widen_workspace` and `off_disk`, with defaults for other backends. From the dogfood fix plan (F4; 2026-11-17).
- **Every `av` command was 2 s slower on Windows.** The CLI's default daemon address was `http://localhost:7420`. On Windows `localhost` resolves to `::1` first; the daemon listens on `127.0.0.1` (its default `AV_LISTEN`, and the port Docker publishes), and a refused connection on Windows takes about 2 s before the CLI tried IPv4. So every command, hook and checkpoint waited 2 s: on the dogfood host `av show` took 2,139 ms, and 91 ms with `AV_URL=http://127.0.0.1:7420`. The default is now `http://127.0.0.1:7420`. An `AV_URL` you set is used as before. From the dogfood fix plan (F4; 2026-11-17).
- **This repository's task workspaces leave out its result archives.** `.av/sparse` now lists `bench/results/` and `bench/retrieval/results/` (4,670 of 5,173 files). On Docker Desktop with the workspaces on a Windows bind mount, a start went from 89.95 s to 6.54 s and a checkpoint from 1.27 s to 0.23 s (median of five, `bench/results/2026-11-17-f4-sparse/`), faster than the volume layout's 15.9 s; `docs/reference.md` says when each layout pays. An agent reads an archived file with `av show PATH`. From the dogfood fix plan (F4; 2026-11-17).
- **A repo can keep paths out of its task workspaces (`.av/sparse`).** Every task workspace was a full checkout, so a start wrote every file of the repo, most of them in paths few tasks touch (in this repository, 4,670 of 5,171 files and 91 % of the bytes are result archives). A repo that commits `.av/sparse`, one path prefix per line, now gets task workspaces without those paths on disk. They stay in every commit; only the files on disk change. Nothing an agent writes is lost: a new file anywhere, or one written under a listed path, is snapshotted; `av sync` writes what `main` added and every conflicted file; a remote checkpoint may change a listed file. Without the file nothing changes. From the dogfood fix plan (F4; 2026-11-17).
- **`av next --checkout DIR`, and a Windows recipe that makes a start fast.** On Docker Desktop a start of this repository took 93 s, 87 s of it jj writing the workspace into the bind mount. `docs/reference.md` ("Workspaces on a volume (Windows)") now puts the workspaces on a volume and the agent in its own checkout: the same start took 15.9 s and a checkpoint 0.6 s instead of 3.1 s (`bench/results/2026-11-17-f4-storage/`). `av next` takes `--checkout DIR` as `av task start` does, for whatever work it hands out (a start, a claim, a handoff, a task sent back), under that work's session and generation. Both refuse a DIR that is not empty before they take any work (`task start --checkout` used to start the task first). From the dogfood fix plan (F4, F11; 2026-11-17).
- **A start says what its workspace cost.** A task start took about 90 s on the dogfood host and nothing said where the time went. Every start (`av task start`, `next`, MCP `start_task`) now adds a `workspace:ready` event after `started`: `ms`, the workspace copy (jj's checkout of `main`); `overlay_ms`, the task's index overlay; `files` and `bytes`, what the new workspace holds (`.jj/` included, `target/` left out); and `count_ms`, what counting them took. The count runs inline, so the event's place in the feed is fixed. A consumer of the event feed (`GET /v1/events`, `subscribe_events`) sees one more kind. From the dogfood fix plan (F4, F21; 2026-11-17).
- **Lite mode's start error mentions root only to root.** When lite mode's Postgres could not be created, set up or queried, the error always added "Postgres refuses to run as root", which sent anyone not running as root after the wrong cause. It now says so only when the daemon runs as root, and still names `AV_DATABASE_URL` as the way to another server. The two `embedded_pg` tests skip with a `SKIPPED embedded_pg` line on stderr when `AV_TEST_PG_INSTALL_DIR` is unset, empty or `none` (`none` was taken as a directory) and when they run as root (they failed there); `CONTRIBUTING.md` has a recipe for running the bar in a container. From the dogfood fix plan (F22, F25, F27; 2026-11-17).
- **A backlog item whose task was abandoned can be requeued or dropped.** `av abandon` ends a task but left its backlog item `taken` for good: `av backlog drop` accepted only queued and proposed items, nothing put it back, and an item listed in another's `after` kept that one waiting for good. The workaround was to post the work again, which kept a "(retry)" in the title of the commit that finally merged it. Now `av backlog requeue ITEM` (`POST /v1/backlog/{id}/requeue`) puts such an item back to `queued` with no task, so the next `next` starts a new task from it under the same title, and `av backlog drop` drops it. Both are refused (422) while the item's task is not abandoned. Only a requeue lets the items that list it in `after` start, once its new task merges; a dropped item keeps them waiting, as before. The requeue's `backlog:requeued` event names the abandoned task, which the item no longer does. `GET /v1/backlog` adds each item's `task_status`, and `av backlog` prints such an item as `taken (task abandoned)`. Abandoning still leaves the item alone: an agent that abandons because it cannot do the work would take it again. From the dogfood fix plan (F12, F18; 2026-11-17).
- **`AV_EXEC_BIT=ignore` keeps file modes on a Windows bind mount.** In Docker Desktop, a file Windows creates on a bind-mounted workspace reads as executable (`777`) in the daemon's container, while `chmod` there works, so jj's own check finds the executable bit supported and every file a task wrote from Windows was committed as `100755` (this repository's own dogfood merges did that to 35 files, now reset). The daemon's new `AV_EXEC_BIT` (`auto`, the default and the old behaviour; `respect`; `ignore`) sets jj's `working-copy.exec-bit-change` for every workspace. With `ignore` a snapshot keeps each file's mode from the repo and a new file is not executable, so a task cannot change a mode, and a checkout writes files without the executable bit. The compose file passes it through; set it for workspaces edited from Windows. From the dogfood notes (F51; 2026-11-17).
- **A refused review writes nothing.** `av review` (`POST /v1/tasks/{id}/review`, which the MCP tool also calls) wrote the verdict first, then each finding, and the 200-character summary limit was checked only as each finding was written: one long third finding left a `request_changes` verdict and the first two findings behind a call that answered 400. Every finding is now checked before anything is written, a blank summary is refused too, the error names the finding (`findings[2]: the summary has 263 characters; at most 200`), and the verdict is written after the findings. From the dogfood fix plan (F15; 2026-11-16).
- **`av watch` turns the Stop hook's wait off for its command.** Since every workspace got its `.av-task` (F14), the Stop hook of a headless agent that `av watch` started (`claude -p`) waited up to `AV_HOOK_WAIT_SECS` (600 s) for more work after its task was done, holding its `--max-agents` slot, and could carry that session into the next task. `av watch` is already the loop that waits, so it now sets `AV_HOOK_WAIT_SECS=0` for the command it runs, unless `av watch`'s own environment sets the variable (then it is passed on as it is). Interactive sessions keep the hook's default wait. From the second dogfood fix plan (T2; owner decision 5; 2026-11-16).
- **Names fence only when both sides are known.** After a claim, a previous owner still running in the workspace could keep writing: the daemon's `.av-task` names no agent (F14), so every reader wrote under the new generation, while a file the CLI wrote refused any other name, the owner's own unnamed hooks too (they act as `agent`), as "claim lost". Now one rule (`margincode_core::taskfile::claim_of`) decides for the CLI and the hooks: a reader with a name (`--agent`, `AV_AGENT`, a Grok hook as `grok`) in a workspace whose file names another owner (not the default `agent`, nor one of its aliases) lost its claim. The CLI refuses before sending anything, naming both; the `post-edit` hook checkpoints nothing and tells the agent on every edit (`additionalContext`), also when the daemon answers "claim lost"; the Stop hook blocks once per session with the same sentence, in place of its record nudge. The daemon names the owner, with its profile's aliases, in the file of a workspace it hands an MCP session named by `X-Av-Agent` (a start, a start from the backlog, a claim, a resume), so a named previous owner is fenced after a claim over MCP too, which nothing did before. Given up: an unnamed command in a named owner's workspace is now let through, with a warning (`set AV_AGENT`), since it cannot be told from the owner's own hooks; a previous owner with no name is not fenced through its hooks or `av` (its MCP tools still are). `av validate` by a named agent that does not own the task (a reviewer in the owner's workspace) checkpoints nothing, says so, and sends no generation, as an attestation; the daemon records it at the task's head as it is (`at_head` in the validate body) and refuses it when the workspace holds files that head does not, so a previous owner cannot move the head through a validation. The CLI's own files name the owner too, with its aliases: a file lists as `aliases` the other names of the profile the owner names or is an alias of (a start or claim under an alias stores the alias; `next` stores the profile's name), so an owner under any of them is not fenced, and names nobody when the profiles cannot be read; `av remote clone` names the owner rather than whoever cloned. The daemon names the owner in the same write that hands out a claim's or a resume's generation, so no named previous owner can read the new generation from an unnamed file and land a write under it; `av claim` and `av next` ask for that with `X-Av-Name-Owner`, and their own later write leaves a file a newer claim wrote, or one the daemon already named for the same generation. A profile's own name wins over another profile's alias when the other names are looked up. In a remote checkout, a non-owner's `av validate` is refused (the command would run on files the recorded head may not hold). `integrations/claude-code/hooks/post-edit.sh` no longer discards `av hook post-edit`'s answer, so its notices reach Claude Code on that install path too. The Stop hook keeps the session it told apart from its record nudge's (`told_session`) and tells also when the stop continues an earlier block; a fenced stop then ends there, without waiting for new work. `--generation` (`AV_GENERATION`) is still sent as given, over the rule. A Grok whose MCP session is named as a persona needs `AV_AGENT` set to the same name (the Cloud's runner sets it), or its hooks act as `grok` and are fenced. From the second dogfood fix plan (T1a, F5; 2026-11-16).
- **A workspace that undo or revert re-creates gets its `.av-task` back.** Once the reclaim removed a merged task's workspace (`AV_WS_RETAIN_SECS`, `av gc`), an undo or a revert re-created it at the task's head, but snapshots leave `.av-task` out, so it came back without one: the owner's hooks did nothing there and `av` commands found no task without `--task`. Now the daemon writes it, as on its other hand-outs: the task, a new session and the task's generation, naming no agent (the caller is the merger, not the owner). A workspace that was still there keeps its file. Found reviewing the `.av-task` change (2026-11-16).
- **A hook's write of `.av-task` no longer undoes a claim.** The hooks read `.av-task` when they start and wrote the whole file back later: session start (a new session), `post-edit` (its debounce stamp) and the Stop hook (the record nudge, and its keep-busy count after a wait of up to `AV_HOOK_WAIT_SECS`, 600 s by default). When the task was claimed in between, that write put the previous owner's generation back, and the new owner's hooks and commands in the workspace were refused as "claim lost". Now each hook re-reads the file and changes only its own field, and only while the file still holds the same task and generation; a file that changed hands is left alone. A claim that lands between that read and the write is still lost. The Stop hook now keeps a session busy only on a continue it counted in the file: it checks `AV_HOOK_MAX_CONTINUES` again on the file after its wait, and does not block when the file changed hands during the wait or cannot be written (before, a continue it could not write still blocked, and was never counted toward the cap). Found reviewing the `.av-task` change (2026-11-16).
- **The Stop hook no longer offers a session its own task as a review.** In a default Claude Code install the hooks act as `agent` (no `AV_AGENT`) while the MCP session acts as `claude-code`. Since `.av-task` is written for tasks started over MCP (below), the Stop hook's keep-busy check runs there too, and it asked `next` as `agent`. Once the session submitted its task, `next` offered that task to `agent` as a review waiting (a review is kept only from its owner), the session's `start_task` found nothing, and the hook blocked the stop again, up to `AV_HOOK_MAX_CONTINUES` (20) turns. Now a hook given no name asks as its task's owner, which is the session's own name when it owns the task. An unnamed session in a task it does not own (a previous owner still running after a claim over MCP) is asked for as the owner too. A hook with `--agent` or `AV_AGENT` asks as that name, and one Grok runs as `grok`, as before. Found by the second dogfood fix plan (2026-11-16).
- **The daemon writes `.av-task` into every workspace it hands out.** Only the CLI wrote it, so a task started over MCP (`start_task`, `claim_task`) had none: the hooks did nothing in its workspace and `av validate` and the other commands found no task without `--task`. Now a start, a start from the backlog, a claim and a resume (`next` handing an owner back its task) write it with the task, a session (a start's caller's, over MCP the MCP session; a claim's or a resume's new one) and the generation. It names no agent: the daemon does not know the name `av` and the hooks act as in that directory (Claude Code's hooks act as `agent`, its MCP session as `claude-code`), and a wrong one would refuse the owner's own writes as "claim lost". The cost: after a claim over MCP, a previous owner's hooks and `av` commands still running in that directory write under the new generation and are accepted (before, its old file fenced them; the MCP tools of its session are still fenced). The CLI's own `av claim`, `av next` and `av task start` still write their name, which fences as before. A resume keeps what the file already held for that generation (the Stop hook's count of continues among it). A review offered by `next` does not touch the owner's file. A failed write is logged and never fails the start or the claim. `TaskFile` moved to `margincode_core::taskfile` (the CLI re-exports it). Found by dogfooding (2026-11-16).
- **A `start_task` the MCP client gave up on is handed to its retry.** A first pack can take longer than the client's tool timeout. Claude Code (2.1.325) resets a tool call's socket 60 s after it sent the call; it sends `notifications/cancelled` only when its `MCP_TOOL_TIMEOUT` is shorter than that (none at the default 60 s), and calls again about 1 s after it gave up, in the same MCP session. So the start went on unseen, its task stayed claimed by nobody who knew of it, and the retry was told no task was waiting (or was handed another one). Now the start goes on and its answer goes to the retry, which waits for it when it still runs, and no second task is made. Only a start still waiting for work stops, and takes nothing, when its socket closed with no retry holding it or its session ended. It holds within one MCP session and while the task is still open under the same claim. Found by dogfooding (2026-11-16).
- **`next` takes `skip_returned`.** With `skip_returned: true`, `POST /v1/backlog/next` does not offer the caller its own tasks that review sent back (in a `dry_run`, a wait and at the `max_open` limit too), for a caller that takes those up itself: the Commercial runner counts review rounds and escalates, and after the change below it was handed back the task it had just escalated, ahead of its other work, in a loop. Without it nothing changes. Found syncing the Commercial edition (2026-11-16).
- **A task sent back with requested changes goes back to its owner at once.** `next`, `start_task` with no arguments and `av watch` used to offer it to nobody until the owner's lease ran out (the change request renews it, 30 minutes by default), so a headless owner never came back to its findings, and after the lease any fitting agent could take it. Now the owner gets it first, after reviews of others' work: `from: changes_requested`, resumed under the same generation, with the handoff bundle (the findings) and a `next` line. It is not offered again once the owner moved its head, and it is kept at a team's `max_open` limit. Found by dogfooding (2026-11-16).
- **deploy/Dockerfile builds from main again.** It copies `integrations/dream`, which `av dream`'s instructions are compiled from, so the image build needs that directory.

- **A review run is no longer handed the same declined record every time.** A record that a finished review run was handed and wrote no verdict on waits behind the other records until it scores higher than it did in that run (for instance when its task's merge is reverted afterwards); among waiting records, and among the others, the score decides, ties by id. Runs back to back with a small `--limit` (`margincode-bench dream-freeze --until-empty`) used to be handed the declined record again whenever it outranked or tied the others (ties go by record id, which is random), and stopped with records never handed; now they reach every record. A review run's `worklist` now carries `scores`, each handed record's score by its id, beside `records`.
- **`margincode-bench dream-freeze` and `run|first-packs --dreamed FILE`: one dreamed memory for every session of an arm.** `dream-freeze` seeds each relay task's store as a session would, runs `av dream` and then `av dream --verify` as a second agent, and writes the verdicts and reviews they ended with to a file. A record is named there by its place in the pool (or its earlier task, kind and place), an event by its kind and task, so that another seed finds them. `--dreamed` replays that file into each av session's freshly seeded store through the daemon's dream routes, before the agent's task starts; every quote is checked again, and every verdict must end in the state it was frozen in. `meta.json` gains `memory` (`raw`, `wrong-reverted`, `dreamed sha256:…`), and a results directory holds one. `--rounds N` repeats the review and verify runs; `--until-empty` (stage A) makes review runs back to back until the worklist is empty (or holds only what the agent left undecided), then verify runs until none waits, and exits 1 when a wrong note or true lesson was never handed to a run. The tally reports the records the review runs were handed and how many were left undecided, and counts each note once, by its newest effective verdict, beside how many notes any verdict judged. This is the harness side of Plan 16's M2 (`bench/results/2026-11-13-standing-and-dreams/PREREG.md`).
- **`GET /v1/verdicts/{id}` answers the verdict's `reviews`** (oldest first: `reviewer`, `dream`, null for a person, `agree`, `evidence`), so whoever decides on a verdict can read who already looked at it and what they quoted.
- **`margincode-bench run|first-packs --wrong-reverted`: a wrong note as the record of reverted work.** The pool's `no_commit` notes (the wrong-lesson pool's wrong notes) are seeded as the records of a second earlier task. Before the agent's task starts, that task is merged and its merge reverted, so their standing is `reverted`. Its one file (`AV_BENCH_REVERTED.md`) is gone again, so the agent's workspace is still the base tree. Without the flag every note is an open earlier task's record, as before. This is the harness side of Plan 16's M1, pre-registered and not run in `bench/results/2026-11-13-standing-and-dreams/PREREG.md`, together with M2's raw-against-dreamed comparison.
- **No open address without a token.** `AV_LISTEN` now defaults to `127.0.0.1:7420` (was
  `0.0.0.0:7420`), and the daemon refuses to start on a non-loopback address unless `AV_TOKEN` is
  set or `AV_LISTEN_OPEN=1` says something else limits who can reach it. The compose file sets
  `AV_LISTEN_OPEN=1`, since it publishes the port on the host's `127.0.0.1` only. A deployment that
  sets `AV_LISTEN=0.0.0.0:...` with no token must add one of the two. The Docker image sets `AV_LISTEN=0.0.0.0:7420` (a published port reaches nothing else), so a plain `docker run` needs `-e AV_LISTEN_OPEN=1` beside `-p 127.0.0.1:7420:7420`, or `-e AV_TOKEN=...`.
- **`av dream` and `av verdict`: the agent side of a second look at records.** `av dream --repo R -- <agent command>` asks the daemon for a run, writes what the run is handed (`bundle.json`) and the instructions (`integrations/dream/DREAM.md`, built into the CLI, its SHA-256 stored on the run) into `$AV_HOME/dreams/<run id>/`, runs the agent there and posts the `verdicts.json` it wrote; `--verify` does the same for the second look (`reviews.json`). What the daemon refuses goes back to the agent once (`--fix-rounds`). A command that fails, runs past `--timeout` or writes no file ends the run as failed and posts nothing. `--max-agents` shares the host's agent slots with `av watch`. `av dream undo|show <run id>`, and `av verdict list|show|accept|reject|retract` for a person's second look. New route `GET /v1/verdicts/{id}`: one verdict with the record it judges. Tested with agent scripts that run no model; no real model has dreamed yet.
- **Extensions are asked before shared memory changes.** `MergeGuard` gains `Guarded::MemoryWrite { repo, action, by, dream }`, asked once the daemon's own rules allow the write and before anything is written: before the review of a proposed verdict (`MemoryAction::VerdictReview`) and before a record supersedes another (`MemoryAction::Supersede`). A refusal is `422`, and the verdict stays proposed or the record live. `AllowAll` allows both, so Community behaves as before. A guard with an exhaustive `match` on `Guarded` needs the new arm. A review of a verdict that is no longer proposed is now refused (`409`) before its evidence is stored, as before, and before the guard is asked.
- **The project is now Margincode** (it was Scholion; that name is in use elsewhere). The CLI is `margincode`, the daemon `margincode-daemon`, the crates `margincode-*`, the site `open.margincode.dev`. **`av`, the `AV_` variables, the HTTP API, the MCP server and the hooks do not change**, so scripts and agent configurations keep working. What an upgrade keeps: a data directory `~/.scholion` is still used when it exists and `~/.margincode` does not (new installs get `~/.margincode`); lite mode's database and the Compose project keep their names, since a new one would start on empty data; `scripts/install.sh` still reads `SCHOLION_VERSION` and `SCHOLION_INSTALL_DIR`. What changes for an existing install: the binaries `scholion` and `scholion-daemon` are no longer shipped (use `av`, or `margincode`); the Docker image's user is `margincode` (uid 1000 as before); the Claude Code plugin is `margincode@margincode`. Earlier entries below, the plans under `docs/superpowers/` and the benchmark results keep the old name: they are history.
- **Dreams and verdicts: a second look at records** (the daemon's side). A run started with `POST /v1/repos/{repo}/dreams` is handed the records where a second look is most likely to pay (reverted or abandoned work, anchored code that changed, records that share an anchor, records packs show most) with their task and the code on `main`, and proposes verdicts: `confirmed`, `disputed`, `duplicate`, `outdated`. The daemon runs no model: it refuses a verdict whose record is not on the run's list or whose evidence is not found word for word in the file, record or event it cites. A verdict has no effect until a `verify` run with a quote of its own, or a person, agrees (`POST /v1/verdicts/{id}/review`); then a disputed record ranks at 0.3 and says why, a duplicate or outdated one leaves packs and the pre-read hook's answer, and a confirmed one loses its "changed since recorded" mark until the code changes again. Records are never edited; `retract` and a run's `undo` put packs back byte for byte. Migration 0031 (`dreams`, `record_verdicts`, `verdict_reviews`). `PACKER_VERSION` 10.
- **A record's standing: where it comes from.** A pack served a decision of a task whose merge was reverted, or that was abandoned, like one of merged work. Now another task's record is printed with its origin unless that is merged work (`decision, reverted:`, `abandoned`, `open`), in packs, the pre-read hook's answer and `av expand`, and a claim of reverted work ranks at 0.4 and of abandoned work at 0.6 (an `attempt` keeps its rank; merged and open work rank as before; nothing ranks above merged work). No table and no migration: the origin is computed from the task's status and its last `revert` event, in one query per pack. `GET /v1/tasks/{id}/records` and search add `standing: {origin, factor}`. `PACKER_VERSION` 9.
- **The summary limit is said before an agent writes.** In a benchmark run 53 `av record` calls in 28 of 30 sessions were refused for a summary over 200 characters and retried. `av record --help` now states the limit for `--summary` and for the entries of `--json`, and the Stop hook's request for records names it.
- **`margincode-bench run --agent-user`: a refused preflight names a path it could not look at.** A directory the preflight may have made, and then could not look at, was dropped from the clean-up without a word; the refusal now names it with the `chmod 700` that closes it. The run report's method paragraph no longer says that every relay note is a commit message (a note marked `no_commit` is in `NOTES.md` and the records only).
- **Tests run offline on a machine with PostgreSQL installed.** With no `AV_TEST_DATABASE_URL`, `margincode-testutil` downloaded an embedded Postgres on first run, which fails without network access or behind a proxy that refuses the download. It now first looks for an installed PostgreSQL (`AV_TEST_PG_INSTALL_DIR`, `pg_config`, the usual install directories) and starts a private server per test from its binaries in a temporary directory, as `postgres` or `nobody` when the tests run as root; the server and its files are gone when the test ends, and an installed service is never touched. `AV_TEST_PG_INSTALL_DIR=none` keeps the download.
- **`margincode-bench run --agent-user` leaves no run directory open.** A finished run sets `<work-root>/runs/<run id>` to 700 (a resume opens it again while it runs), a refused preflight removes the empty directories it made and puts back the modes it changed, the refusal on another run's open directory names it with the `chmod 700` that closes it, and `rows.jsonl`, `meta.json` and `report.md` are mode 600 from their first byte and after a resume that adds no row. Without `--agent-user` nothing changes.
- **An agent can correct a stored record.** `supersedes` was in the store and the HTTP API, but
  no agent could reach it, and nothing in a pack said that a record is an earlier agent's claim:
  in a benchmark an agent handed a confidently wrong record followed it, where the same text in a
  notes file was simply edited. Now `av record … --supersedes rec:…` (also per entry of `--json`),
  the MCP `record` tool's `supersedes` and the API's `supersedes` all take the `rec:` id a pack
  prints (a prefix of 6 or more hex characters or a full id too, with or without hyphens), and the
  answer names what was superseded. The old record is kept as it was and marked superseded; a
  correction without anchors takes the old record's, so it is shown where the wrong one was. What
  corrects a decision, attempt or note is a decision, attempt or note (another kind would not be
  shown in its place: `422`). Refused: a record of another repo (`422`; it was accepted before,
  which let one repo's task hide another repo's record), a `review` or `validation` record
  (`422`), another task's `intent` or `handoff` (`422`; a task restates its own with the same
  kind), an already superseded record (`409`, naming the live one when the chain of corrections
  is at most 64 long; it was accepted before, which left a record with two successors), an unknown id (`404`, was `500`). The store enforces these
  inside the insert with the old record's row locked, so two tasks superseding one record at once
  leave it one successor. A `submit` whose inline records are refused writes none of them, two
  that supersede the same record included (`409`, naming both entries). The feed gets `record:superseded` (`{old, new, by}`). A pack that shows
  another task's decision, attempt or note says in one line under *Decisions & dead ends* that
  these are claims to check against the code and how a wrong one is corrected (25 tokens, inside
  the budget, and no room kept for it by a pack that only refers back to such a record;
  `PACKER_VERSION` 8), and the pre-read hook says it once per session. The MCP tool list stays
  within its size guard: `start_task` lost its empty `required` list, `submit`'s inline records
  the receipt description that `record` already carries (they list `supersedes` too) and
  `records` a description that repeated the tool's.
- **`margincode-bench` can run Grok.** `--vendor grok` launches `grok -p … --output-format json --always-approve --max-turns N --cwd` and reads `usage.input_tokens`, `usage.cache_read_input_tokens`, `usage.output_tokens`, `total_cost_usd` and `num_turns` into the session usage. It does not pass `--max-budget-usd`; the turn cap stands in for it. Reasoning tokens are not kept: the usage total has no field for them.
- **`margincode-bench routing-profile`** reads an E-G1 results file and prints accepted rates, Wilson intervals, and the `av agent set` lines the decision rule implies. A tie, or a cell with fewer than 10 trials, is inconclusive and does not replace that agent's profile.
- **Containers started without the labels are footprint too.** With `docker` on `PATH`, the
  session-start snapshot also keeps the id of every container then (`docker ps -aq --no-trunc`),
  and in a task the Stop hook reports the containers that are new since and carry no `av.task`
  label as `containers.unlabelled` (`{id, name, state}`) with the footprint; `av disk` shows them
  under the session. Running ones count toward `AV_FOOTPRINT_CONTAINERS` with the task's labelled
  ones, and the block names them and says to add `$AV_DOCKER_LABELS` next time. `av gc
  --containers` still never touches an unlabelled container. Without docker, or without the ids
  from session start, nothing changes.
- **`GET /v1/disk?fs=1`: the filesystem numbers without the walk.** With `fs=1` (or `fs=true`) the
  answer carries only `fs_free_bytes`, `fs_total_bytes`, `min_free_bytes`, `below_floor` and
  `build_budget_bytes`, measured as the free-space floor measures them, and reads neither the store
  nor any workspace, `target/` or footprint; without the flag the answer is unchanged. A monitor
  that polls free space (Margincode Cloud's `disk_low` alert, every 60 s) no longer pays for a walk
  of every workspace. `av disk --fs` prints the line, `--json` the object.
- **Containers are footprint.** Everything a session starts through docker is to carry the
  labels `av.task=<task id>` and `av.agent=<agent>`: in a task the session-start hook exports
  them as `$AV_DOCKER_LABELS` for `docker run`/`docker build` and, with `docker` on `PATH`, says so
  in one line. The Stop hook then lists the task's labelled containers, images and volumes with
  the footprint (`av disk` shows them under the session) and blocks once per session while more
  than `AV_FOOTPRINT_CONTAINERS` (default 0) of its containers still run, naming them; it never
  stops one. `av gc --containers` removes the stopped containers, dangling images and volumes of
  merged and abandoned tasks (running ones are listed, open and unknown tasks' things left alone;
  `--dry-run` plans, `--json` carries it as `containers`). The daemon runs no docker: the CLI does.
- **`AppState::with_vcs_layer`: an extension can wrap the version-control engine.** Called from
  `run_with`'s `customize` step, it hands the extension the jj engine and uses what it returns
  everywhere from then on: the API, the packer, the backup loop and the disk sweep. A layer may
  forward every call and observe it, or replace the engine.
- **`AppState::with_checkpoint_hook`: an extension can add to a checkpoint's answer.** Asked
  after `POST /v1/tasks/{id}/checkpoint` (a remote checkout's patch too) with the task, the new
  head and the changed paths; the object it answers is added beside `head_change`, `changed` and
  `warnings` (the daemon's fields win), and its `notices`, lines for the agent, reach the agent
  through the MCP `checkpoint` tool's answer and the Claude Code `post-edit` hook, which prints
  them as `additionalContext`. A hook that fails adds nothing and never fails the checkpoint.

## 0.0.3 (2026-11-06)

Agents can now propose backlog items that wait for a person's accept (`av post --propose`,
`av backlog accept`), and `av attention` (`GET /v1/attention`) lists what needs a person or an
agent. `av repo advance` brings work merged on the remote back to main, and `av notes` puts a
task's decisions and dead ends on its pull request. A new repo can start with records mined from
its git history (`--mine-history N`). `embeddinggemma-2` is the default Ollama embedder, and
Ollama may run in another container or host. A head takes one approval per review round, and the
dashboard is redesigned. Lean disk and lean builds reclaim finished tasks' workspaces and their
build output. Migrations 0027–0030 run on the first start; a 0.0.2 `AV_HOME` upgrades in place.

- **Ollama on another container or host.** `AV_EMBED_PROVIDER=ollama` no longer needs an `ollama`
  CLI on the daemon's `PATH`: with none, the daemon sends `AV_OLLAMA_EMBED_MODEL` (now required in
  that case, for example `embeddinggemma-2`) as written to the Ollama server at `AV_OLLAMA_URL`'s
  `/api/embed`, as a daemon in one container and Ollama in another need. Nothing connects at start:
  the daemon boots while that server is down, and a missing model or a server that is down is
  logged at the first embedding (the outbox keeps the work and retries). With the CLI, selection is
  unchanged (`ollama list` resolves the name to a pulled tag), and so is automatic selection with
  no CLI (`AV_EMBED_URL`, else off). The `api` path (`AV_EMBED_URL`) speaks text-embeddings-
  inference's `/embed` and cannot talk to Ollama, which serves `/api/embed` and `/v1/embeddings`.
- **The CLI says why a request was malformed.** An error the daemon answers in plain text rather
  than its JSON `error` (a path it cannot parse, such as `av backlog accept SLUG` or `av review`
  with a short id, which need a full id) was printed as `request failed (HTTP 400)`; the CLI now
  prints the text (for a slug: that `id` is not a UUID; at most 300 characters). Found by the
  0.0.3 release check.
- **Proposals: agents queue what they are sure of and propose the rest.** A backlog item can be
  `proposed` (`POST /v1/backlog` with `propose: true` and an optional `why` of at most 300
  characters; `av post --propose --why`): `next` never offers it until a human accepts it with
  `POST /v1/backlog/{id}/accept` (`{by}`; `av backlog accept ID`), which queues it, adds a
  `backlog:accepted` event and wakes waiting `next` calls; `av backlog drop` drops one. Accepting
  an item that is not proposed is a 422. The repo setting `agent_posts` (`queue`, the default, or
  `propose`; `av repo settings NAME --agent-posts`) makes every agent's post a proposal: a post is
  an agent's when it comes through the new MCP tool `post_backlog` (repo, title, goal, propose,
  why, target, after) or its `posted_by` is an agent profile's name or alias. A proposed item is
  not counted in its target's `items`/`queued` (`proposed` counts it apart); an item `--after` it
  waits until it is accepted and merged; an accepted item's reserve window starts at the accept.
  `MergeGuard` is asked `Guarded::AcceptBacklog { item, by }` before an accept (`AllowAll`
  allows it), so an extension can keep agents from accepting their own proposals. `GET
  /v1/attention` adds the low-severity kind `proposed_backlog`; `av backlog` and the dashboard
  show proposals with who proposed them and why. Migration 0030 (`backlog.state` adds
  `proposed`; `backlog.why`, `accepted_by`, `accepted_at`; `repo_settings.agent_posts`). The MCP
  tool list grows by 477 bytes (its guard in `tests/mcp.rs` now allows 10,800).
- **One approval per head** (Plan 15 Phase 0, "Loop noise"). In the first dogfood run two reviewers
  approved the same head seconds apart, the second leaning on the first one's validation. `POST
  /v1/tasks/{id}/review` with `approve` on a head whose newest verdict in the current review round
  (since the task's latest submit) is already an approval is now refused (422) with who approved it
  and when; `request_changes` on an approved head, an approval after a resubmit (a new head, or the
  same head after a request for changes or a revert), and the board's merge of an approved task work
  as before. A review takes the task's lock, so two concurrent approvals cannot both pass. A finding
  the reviewer already stored at that head (the same summary, body and anchors, the anchors in any
  order, as a retried `av review` sends) is not stored again. `next` already kept a task offered
  for review from a second reviewer (the 600 s review reservation). No migration: the verdict's
  time is `review_verdicts.created_at`, the round starts at the newest `submit` event
  (`IntentStore::round_verdict_at`).
- **Scheduled regressions** (Plan 15 Phase 3). `.github/workflows/nightly-scale.yml` runs every
  night against a release-built daemon with no semantic layer: `scholion-bench scale` on tokio
  (`tokio-1.54.1`), failing when one-agent p95 latency rises above 600 ms (start), 300 ms (context)
  or 100 ms (checkpoint), when merges/s falls below 1.5 at 32 agents or 1.2 at 100, or when any
  level does not merge every agent; and `scholion-bench retrieval` on the E1 CI fixture with the
  E1 gate's floors (`bench/retrieval/ci/make-repo.sh` builds the fixture repo and its task lines).
  `.github/workflows/e4-smoke.yml` runs the E4-at-scale smoke every Monday (relay-customer-region,
  `av` arm, `--noise 50`, $0.02) on DeepSeek with the repository secret `LLM_API_KEY`, reports
  whether the start pack shows the lesson (`pack-hit.json`), and skips with a notice when the
  secret is not set. Neither runs on pull requests; the E1 gate in `ci.yml` is unchanged.
- **Needs attention, for machines: `GET /v1/attention` and `av attention`** (Plan 15 Phase 2).
  What the dashboard's strip computed in the page is now the daemon's: red main, conflicted tasks,
  reviews nobody gave, approvals nobody merged, handoffs nobody claimed, open tasks nobody wrote to
  for a lease, backlog items no agent fits, overdue targets and big footprints, each
  `{kind, severity, repo, task_id?, slug?, agent?, since, title, detail}`, worst first. The page reads
  it (the strip and the cards' stuck marks look the same; conflicted cards are marked at once). The
  page's thresholds are `AV_ATTN_REVIEW_SECS`, `AV_ATTN_APPROVED_SECS`, `AV_ATTN_HANDOFF_SECS`,
  `AV_ATTN_FOOTPRINT_BYTES` and `AV_ATTN_FOOTPRINT_SECS` with the same defaults. `av attention
  [--repo R] [--json]` prints one line per item and exits 0. `GET /v1/repos/{name}/health` adds
  `since_at` to a red answer. No new table.
- **Main comes back from the remote: `av repo advance`** (Plan 15 Phase 0). The backup can push
  main to a branch for a pull request, but nothing brought the remote's `main` back, so work merged
  there (that pull request, and others) never reached main and the next task started on a stale
  base. `POST /v1/repos/{name}/trunk/advance` (`av repo advance NAME [--remote URL] [--branch B]`;
  defaults: the backup remote and `main`) fetches the branch and fast-forwards main to it when it
  descends from main, then re-indexes main and emits `trunk:advanced`; `409` with both heads when
  they diverged, main unchanged. `av repo backup NAME --remote URL --advance` makes every backup
  push advance first (migration 0029 adds `backup_state.advance`). New `Vcs::advance_trunk` with
  the `Advance` outcome.
- **A guard can refuse a review verdict: `Guarded::Review`.** `MergeGuard` is now also asked
  `Guarded::Review { task, reviewer, verdict }` for every verdict, `approve` and
  `request_changes`, once the task is known to be in review and the reviewer is not its owner,
  before anything is written. A refusal is HTTP 422 (`cannot review: <reason>`) and leaves the task
  in review with no verdict and no `review` records. An approval is still asked as
  `Guarded::Approve` afterwards, so a guard that knows only that question keeps working.
  Extensions with an exhaustive `match` on `Guarded` need a `Review` arm.
- **A change request keeps the task with its owner.** `request_changes` renews the owner's lease
  before it reopens the task, and `next` no longer gives the reviewer who requested the changes
  that task as an abandoned one once the lease runs out (another agent that fits still may; the
  owner still resumes it). Found by the 2026-11-03 dogfood: the owner's lease had run out while the
  task waited in review, so the reviewer's very next `next` took the task it had just sent back.
- **`embeddinggemma-2` is the default Ollama embedder.** On the E1 retrieval set (42 tasks, six
  repos) it scored the same as `qwen3-embedding` (symbols shown 0.59 vs 0.57 at 2000 tokens, 0.81 vs
  0.83 at 8000; files with code shown equal) at 271M parameters and 512 dimensions instead of 7.6B
  and 4096. The daemon now takes `embeddinggemma-2` when `ollama list` has it, else
  `qwen3-embedding`, so a host with only the old model keeps it and its vectors. After pulling
  `embeddinggemma-2` on such a host, run `av reindex` (no `--repo`) once: the collections are
  rebuilt at the new dimension, and until then the outbox worker logs a dimension mismatch that now
  says so and packs fall back to full-text. `AV_OLLAMA_EMBED_MODEL` (now in `docs/reference.md`)
  still names a single model.
- **Site refresh.** The Mission section, the problem intro and the many-agents heading follow the
  solo-first hero; a screenshot of the recorded run sits under the hero; the Claude Code plugin
  install (`/scholion:setup`) is on the page and in `site/llms.txt`; the guides show and declare
  their dates and author, `/demo` has structured data, the sitemap has `lastmod`, the 404 page is
  `noindex`; `og.png` names the new domain and is rendered from `scripts/og.html`. The page is wider
  (1320 px), the How-it-works steps sit two by two, and no command block scrolls sideways.
- **Claude Code under its own name, and a full first pack.** `av --agent NAME init --for claude`
  adds the MCP server with `X-Av-Agent: NAME`, so Claude Code's MCP calls are made under the name
  `av next` and the hooks use rather than its client name (without `--agent`, or with the default
  `agent`, nothing changes; a server already configured is left alone and the output says how to
  re-add it). The `SessionStart` hook starts a new pack session at `startup` too, not only after
  compaction or `/clear`: the pack `av next` or `av task start` printed went to whoever ran
  them, so a Claude Code process started in the workspace got only "sent earlier" handles. Both
  found by the first Teams run (Scholion Cloud's Plan 13 Phase 4).
- **Solo-first pitch.** The README, the site hero, `site/llms.txt` and the `/scholion:setup` skill lead
  with one agent on one repo ("Your next Claude Code session starts knowing last session's dead
  ends"); the several-vendor relay comes second. `av init --for codex` now also writes the
  `X-Av-Agent` header, like Grok, Gemini and Cursor (docs updated).
- **Records on the pull request: `av notes`** (Plan 13 Phase 2, the PR bot). Renders a task's
  decisions, dead ends, notes, review findings, handoffs and checks, plus what other tasks recorded
  about the files it changes, as one Markdown block between `<!-- scholion:notes v1 task=<id> -->`
  markers; reads `/v1` only. `--pr [N]` posts it through `gh`, or edits the comment that already
  holds the task's block (other text and other tasks' blocks stay; an unchanged block is not
  sent). The task is an id, a slug or `repo/slug`; `--format json`. Record text cannot close the
  markers and its `@names` notify no one. The CLI posts because the daemon runs on the
  developer's machine, out of a GitHub runner's reach; `integrations/github/` has the format and a
  sample workflow for a self-hosted runner that can reach it. `integrations/README.md` lists every
  integration.
- **Cold start from git history** (Plan 13 Phase 2). `av repo add NAME PATH --mine-history N`,
  `av init --mine-history N` and `av repo mine NAME [--limit N]` (`POST /v1/repos/{name}/history`)
  turn at most N commits of the repo's history (every ancestor of main, newest first) into
  records: one `note` by the agent `git` per commit with one parent, a subject that is not
  housekeeping, a message that says something (trailers and links dropped) and at most 25 files,
  anchored to the files it touched, with the commit as its `change_id`. They live on one merged
  task per repo, `<repo>/git-history`; no migration. Mining again skips what was mined. Packs rank
  them at half weight (`MINED_FACTOR`, packer version 5), `records/at` lists them after agents'
  records, every rendering names the commit (`note (git 1a2b3c4)`), and `av why` on imported
  history prints the blamed commit's mined record. The commit rules (`housekeeping`, the message
  cleaning) moved from `scholion-bench mine` to `scholion_core::history`, which both use; `Vcs`
  gains `history` (all ancestors, default `trunk_log`).
- **Bench: E4 at scale, `run --noise N`** (Plan 13 Phase 1, harness only; nothing run yet). Adds N
  deterministic irrelevant notes to every relay task, the same ones in the `notes` arm's
  `NOTES.md` and as the `av` arm's anchored records: a seeded generator over the base tree's own
  files and symbols, anchored everywhere (the lessons' files too), spread among the lessons, each
  level the first N of the next. `scholion-bench noise` prints them. Rows gain `input_tokens`
  (cache reads included) and `memory_tokens` (`NOTES.md` or the start pack); a noise run's report
  names its level and adds a "Memory at scale" table; a results directory holds one level. Design,
  levels, metrics and a per-cell cost estimate: `bench/results/2026-11-e4-scale/DESIGN.md`.
- **Remote workspaces by checkpoint-by-patch** (Plan 14). An agent on another host works in a git
  checkout of its task (`av remote clone TASK DIR`, `av task start --checkout DIR`) and each
  checkpoint goes to the daemon as a patch (`POST /v1/tasks/{id}/checkpoint?base=`), which the
  daemon applies in process to the workspace it still owns and snapshots as a local checkpoint;
  a stale base is `409` with the head and `av checkpoint` pulls (`GET /v1/tasks/{id}/patch?from=`,
  `av remote pull`) and resends. Hooks, `submit`, `record`, `handoff`, `validate` and `sync` in
  a checkout send its edits first. The daemon runs no git for it. Reference: "An agent on another
  host"; plan and measurement: `docs/superpowers/plans/2026-11-04-scholion-plan-14-remote-workspaces.md`.
- **Name fan-out cap** (`AV_INDEX_MAX_FANOUT=N`, off by default). The code graph links a call by its
  bare name to every symbol of that name, so `new` alone gave tokio 500k of its 896k edges. With a
  cap, a name more than N other symbols define links to none of them; overlay views apply it to
  the names as the task sees them, and the overlay equivalence proptest runs under a cap of 1 too.
  E1 on the public task set: pack scores unchanged at caps 64 and 16 but for one task at one
  budget, edges −40% / −54%; on tokio a cap of 64 drops 71% of the edges
  (`bench/results/2026-11-03-e1-fanout-cap/`). It applies to repos imported after it is set.
- **Lean builds, one target dir per task** (Plan 12). Build output was 99.99% of the disk the
  first dogfood run took: about 20 GB per Rust task (14 GB of test binaries that were 73% debug
  info, 4 GB of incremental cache) against 3 MB of Scholion's own state.
  - The per-repo cargo config the daemon writes now sets `incremental = false`,
    `debug = "line-tables-only"` (no debug info at all for dependencies) and **no `target-dir`**:
    every task workspace builds into its own `target/`, removed with the workspace. Measured on
    this repo, a full `cargo test` build is 4.3 GB instead of 21 GB.
  - **Fixes Plan 11's shared build cache**, which pointed every workspace of a repo at one target
    dir: cargo names a path package's artifacts by its path relative to the workspace root, so
    `cargo test` in one workspace ran another task's test binary (a false green or red
    validation; hit twice in the run). An av-written config from before is rewritten at start-up.
    A test runs `cargo metadata` in two workspaces of one repo and checks their target dirs differ.
  - **Build output goes with the merge.** A merge or abandon removes the workspace's `target/` at
    once (the answer says `build: removed|none|kept`; the feed gets `build:reclaimed` with the
    bytes) while the workspace itself waits for `AV_WS_RETAIN_SECS` as before, for an undo or a
    reviewer's look; `never` keeps both. A finished task no longer holds gigabytes for an hour.
  - `GET /v1/disk` and `av disk` show each workspace's build output apart (`build_bytes`, a `BUILD`
    column, a total); `av gc` says how much of a reclaimed workspace was build output. The
    dashboard's Disk card shows the same.
  - The snapshot never takes a workspace's top-level `target/`, even in a repo whose `.gitignore`
    forgot it (`/.av-task` was already ignored this way).
  - **One integration-test binary per crate** (this repo, not the product). Its `tests/*.rs` files
    are now modules of one test binary per crate, `it` (`tests/main.rs`; `cargo test -p
    scholion-daemon --test it api::NAME`), instead of 45 binaries that each linked every
    dependency: a full `cargo test` build of this workspace is 2.4 GB instead of 4.3 GB.
  - **Dependencies are built once per repo** (Task 5). When a task's build output is reclaimed,
    the units built from registry and git crates go into `<repo>/.cache/deps-target/`, and a new
    task's `target/` starts with them: compiled libraries hard-linked (one copy on disk for every
    task; rustc replaces them, never writes into them), dep-info, fingerprints and build-script
    output copied (those are rewritten in place). Measured on this repo: a seeded full `cargo
    test` build adds 1.2 GB instead of 2.4 GB and takes 15 s instead of 38 s, no dependency
    recompiled. The repo's own crates never enter the cache. `av disk` counts a linked file
    once; both steps run in the background (`build:reclaimed` with `cached_bytes`, `build:seeded`); `av gc
    --build-cache` empties the cache; `AV_BUILD_CACHE=off` keeps and seeds nothing.
  - **`av convention add` warns when a required check is scoped to every path** (`**`): such a
    check runs for every task, so a docs-only change pays the full build (the run's docs-only task
    paid 20 GB and 10 min). The hint says to scope it (`--scope 'crates/**'`); a change outside
    the scope then skips it, as `av validate` already does.
  - **`{crates}` in a check runs only the crates a change can affect** (Task 2). `av validate`
    replaces it with `-p a -p b`: the workspace members holding a changed path plus every member
    that depends on one of them, by any kind of path dependency (dev and build too), from
    `cargo metadata --no-deps` in the workspace. A changed path outside every member (root
    `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/`, `bench/`) or a failed
    `cargo metadata` gives `--workspace`. The line prints the command that ran, and the check's
    result keeps it (`command`); `GET /v1/tasks/{id}/checks` adds `changed`, the task's changed
    paths. Approval counts a result for a `{crates}` check only with an expanded `command`: run
    as written, `cargo test {crates}` filters every test out and passes, so an older `av` (or an
    MCP agent that leaves `command` out) cannot pass it.
  - **A task's builds stay in its own `target/`** (Task 3). A `CARGO_TARGET_DIR` in the
    user's environment (or `build.target-dir` in `~/.cargo/config.toml`) put every task workspace
    of a repo into one target dir again, where `cargo test` runs another task's test binary. In a
    cargo workspace, `av validate` (the command and every check) and `av-wrap.sh` set
    `CARGO_TARGET_DIR` to the workspace's `target/` and unset `CARGO_BUILD_TARGET_DIR`; when the
    user's environment names a target dir, the session-start hook adds shell code to
    `$CLAUDE_ENV_FILE` that does the same for every Bash command, for the task workspace it runs in
    (a session the Stop hook moves to another task follows it). That is the dir the dependency cache seeds and the merge
    reclaims. An explicit `--target-dir` still wins.
- **`av agent rm NAME`** (`DELETE /v1/agents/{name}`). Removes a profile. An unknown name is
  404. An agent that still owns an open, handed-off, in-review or conflicted task is 409 unless `--force`
  (`?force=true`); the tasks stay. A profile an extension manages is refused, the same way
  `av agent set` is.
- **Lean disk** (Plan 11). Agents' files no longer pile up:
  - **Finished tasks give their disk back.** A task merged or abandoned for `AV_WS_RETAIN_SECS`
    (default 3600; `0` at once, `never` to keep) loses its workspace and scratch dir; a sweep looks
    every 60 s. The task's commits stay in the store. An undo or revert that puts a task back to
    work restores its workspace at its head. Open, handed-off, in-review and conflicted tasks are
    never touched, and nothing outside `ws_root` is deleted. No migration.
  - **`av disk`** (`GET /v1/disk`): per repo, every workspace with its task's status, size, age and
    whether a sweep would remove it, plus scratch, build cache and store, and what sessions left
    outside any workspace. **`av gc [--dry-run] [--retain SECS] [--build-cache]`** (`POST /v1/gc`)
    reclaims on demand. Both take `--json`.
  - **A cargo config per Rust repo.** For a repo with a root `Cargo.toml` the daemon writes
    `<ws_root>/<repo>/.cargo/config.toml` (at import and start-up), which every task workspace
    inherits (what it holds: see Plan 12 above; the shared target dir it first set was withdrawn
    before release). The repo's own cargo config and `CARGO_*` still win; `AV_BUILD_CACHE=off`
    writes nothing.
  - **The hooks watch what a session leaves behind.** `SessionStart` gives a task a scratch dir,
    exported as `$AV_SCRATCH` through `$CLAUDE_ENV_FILE`, and takes a snapshot of the build,
    dependency and cache dirs under `$HOME` and the temp dir. `Stop` compares, blocks once when the
    session added `AV_FOOTPRINT_WARN_MB` (default 512) or more outside any workspace, and reports
    it to the daemon (`POST /v1/footprint`), which shows it in `av disk`. `AV_FOOTPRINT=off`,
    `AV_FOOTPRINT_ROOTS`, `AV_FOOTPRINT_SECS`.
  - `Vcs::remove_workspace` and `Vcs::restore_workspace` (jj: forget the workspace, re-create it
    at a commit).
  - `av up` moves a `daemon.log` over 50 MB to `daemon.log.1`.
  - **`scholion-bench run` removes each session's copy after verify**: the git arms' clone, and
    the av arms' import copy (their bench task is abandoned, so the bench daemon reclaims the
    workspace). Invalid sessions keep everything; `--keep-work` keeps every copy.
- **A redesigned dashboard** (`/ui`): a neutral, professional look in place of the pixel-slime
  theme. It adds a row of summary figures (in progress, in review, conflicted, merged, backlog,
  agents busy), agent cards with initials avatars, a state badge, skill tags and a load bar against
  `max_open`, a kanban board whose columns never overflow, side panels for targets, backlog and
  activity, and a slide-in drawer for tasks and agents. Light and dark follow the system, or pick
  one with the theme button (kept in this browser). The page reads the same `/v1` routes as before.
- **Main, the daemon and the disk on the dashboard.** Under the board, a Main panel lists main's
  last commits with their post-merge check (passed, failed with the exit code, or unchecked) and
  the task that landed each; its header says whether semantic search is on and how many rows wait
  to be embedded, the backup's state, how many conventions the repo has, and an import still
  running. A Disk panel (`GET /v1/disk`, read at start, on its button and every five minutes)
  shows what workspaces, build caches, the store and scratch dirs take, the repo's workspaces
  largest first with the ones a sweep would reclaim, and what sessions left outside any
  workspace; a session of this repo's tasks (or of no task) that left 512 MiB or more (the hooks'
  default) in the last day gets a "Needs attention" row. The page never deletes. When the page cannot load (the daemon
  is away at start, or the history cannot be read) it asks `/healthz` every five seconds and
  starts over once it answers; a feed that drops mid-session keeps reconnecting as before.
- **What needs attention, and how the work flows.** Every task card says how long it has been in
  its status ("in review 3h", "approved 20m" once its review is given), and a card goes amber once
  it is stuck (a submit with no verdict for 2 h, an approval not merged for an hour, a handoff
  nobody took for an hour, a conflict unresolved for an hour, an open task with no write for a
  whole lease). A "Needs attention" strip above the board lists, worst first, a red main,
  conflicted tasks, those stuck tasks, backlog items no agent can do and overdue targets (from
  the health, backlog and targets the page already reads); it is hidden when there is nothing
  and shows at most eight rows. A Flow panel shows, over 7, 14 or 30 local days, merges per day
  as a sparkline, the median cycle time (start to merge), the median review wait (submit to
  verdict, carried approvals left out), rework (changes requested, conflicts, undos, reverts),
  and per agent what it started, merged and reviewed, from each agent's timeline (read at most
  once a minute). No cost or model usage is shown.
- **The task drawer shows the work itself.** The diff from base to head, one file at a time with
  its added and removed lines; the task's own timeline (started, every checkpoint with how many
  paths it touched, claims, submits, verdicts, merges, conflicts); each review round (the verdict
  with the reviewer's findings at that head); a validation record as passed or failed, with its
  command and output; and, after "Build context pack", the pack's receipt: every item with its
  tier, why it is there, where it comes from and its tokens. The page reads the repo's event
  history (`GET /v1/events` in pages of 500, the newest 10 000 ids of a long one) and derives
  these from it; the daemon records nothing new.
- **The teams stand left of the board.** From 1580 px wide the dashboard is three columns: the
  agents' desks on the left, tasks by status in the middle, targets, backlog and activity on the
  right. Narrower, the desks are a row above the board, as before; below 1180 px everything stacks.
- **A `checkpoint` event**: a snapshot that moves a task's head (`POST /v1/tasks/{id}/checkpoint`,
  a submit, a sync) now lands in the event feed as kind `checkpoint` with the new head, the old one
  (`payload.from`) and how many paths the task changes (`payload.touched`), so `GET /v1/events`,
  `scholion events --follow` and in-process subscribers see work as it happens instead of polling
  `head_change`. A snapshot that moves nothing is no event.
- **Targets** (Plan 10 A3): `av target add --repo R --title T [--due YYYY-MM-DD]` names something
  the team works toward; `av post --target SLUG` (or `av backlog target ITEM SLUG`) counts a
  backlog item toward it, and `av target list` shows how many of its items merged, are being
  worked on or wait. `av target done|drop|reopen`. The dashboard shows each active target's
  progress. Migration `0027_targets`. Targets do not change routing.
- **Office and timelines on the dashboard** (Plan 10 A3): each Teams desk now says what the agent is
  on, what it did last, and whether it is working, reviewing, idle or quiet (open tasks with no write
  for a whole lease). Clicking a desk, or a task's owner, opens the agent's timeline: its records,
  starts, claims, review verdicts, submits and merges. New read-only routes `GET /v1/office` and
  `GET /v1/agents/{name}/timeline`, built from data Scholion already keeps (no migration).
- **`scholion-agentrun`**: `AgentCommand::env_clear()` starts the agent from an empty environment
  (later `env` calls still apply; `path_prepend` adds nothing after the prepended dirs when there
  is no `PATH`), and `stop_flag(flag)` kills the run's process group as soon as another thread
  raises the flag (`RunResult::stopped`; `process::run_until`). A runner uses them to keep host
  credentials away from an agent and to stop it at a budget.

## 0.0.2 (2026-11-01)

Several coding agents (Claude Code, Codex, Cursor, Grok Build, Gemini CLI) now share one backlog:
each takes the work that fits it, reviews the others' work, and merges what it approves.

### Install and run

- **The release now includes `scholion-daemon`, with its own Postgres (lite mode).** `av init`
  in a git repository starts it and adds the repo; no Docker needed. 0.0.1 shipped only the CLI,
  so the installer in the README failed on it.
- `av init --for codex|grok|gemini|cursor` wires those CLIs to the MCP server. Grok, Gemini and
  Cursor also get an `X-Av-Agent` header, so the daemon knows which agent is calling.
  `av --agent NAME init --for mcp-json --path FILE` does the same for any client with an
  `mcpServers` JSON file.
- **Ask Claude to set it up**: this repository is a Claude Code plugin marketplace. Run
  `claude plugin marketplace add vinhqbui/scholion`, then `claude plugin install
  scholion@scholion`. It adds three skills: `/scholion:setup` (install, `av init`, checks),
  `/scholion:team` (several agents on one backlog: profiles, first items, `av watch`) and
  `/scholion:doctor` (finds what is broken).
- **A short README**: what Scholion is for, told as a story, how to try it and what it measured.
  The full reference moved, unchanged, to `docs/reference.md`.
- `av repo backup NAME --set URL --branch BR` pushes main to a named branch of a backup remote.

### Route work to the agent that fits (Plan 8)

- **Agent profiles**: `av agent set NAME --best TAGS --can TAGS --cost N`. 1 is the cheapest.
- **A backlog**: `av post --repo R --title T --goal G [--needs TAGS] [--prefer NAME]`, then
  `av backlog`. Tags are set by people; the routing rules are deterministic, with no LLM call.
- **`av next`, or `start_task` with no title and goal**, gives an agent the next task that fits
  it. A task is reserved for its best agent for 10 minutes, then any agent that can do it may
  take it.
- `av handoff --needs TAGS` hands a task to an agent with those skills.
- **Order the backlog**: `av post --after ITEM[,ITEM]` (an item id or the slug of its title).
  `av next` does not offer the item, and `start_task` without a title does not take it, until
  every item it lists has merged; `av backlog` shows it as `waiting on X`.

### Close the review loop (Plan 9)

- `next` offers, in order: reviews of other agents' work, handoffs, abandoned tasks, then new
  work. An owner never reviews its own task. The reviewer merges what it approves.
- `av next --wait N` (MCP: `wait`) waits up to 50 s for work instead of returning empty.
- `av watch -- CMD` starts an agent CLI only when there is work for it. It runs on your machine;
  the daemon never starts processes.
- **A team's limit**: `av agent set NAME ... --max-open N`. At the limit, an agent is offered only
  reviews and its own unfinished tasks until it submits or hands one off.
- **Take only some kinds of work**: `av next --take review,merge` (or `--take work`). A
  review-only agent is never given backlog work, and a worker is never offered (or reserved)
  other agents' reviews.
- **A conflicted task goes back to its owner**: an approved task whose merge conflicted is
  offered to its owner with what to do (sync onto main, resolve, re-check, submit again), so
  `av watch` picks it up. If the owner does not act within the lease, any agent that fits may
  claim it.
- **Keep agents busy**: after its record check, the Claude Code `Stop` hook asks for this agent's
  next work. If something fits, the agent carries on instead of stopping. It waits for work up to
  `AV_HOOK_WAIT_SECS` (default 600, `0` turns it off), and continues a session at most
  `AV_HOOK_MAX_CONTINUES` times (default 20). Grok Build gets the same hook.
- MCP tools take a task's slug (or `repo/slug`) as well as its id. The CLI still takes the id.
- One MCP session can hold several tasks: Claude Code's subagents share their parent's
  session. Without `task_id`, a call uses the session's only open task, or asks which one.
  Before, a subagent's `start_task` sent the parent's later calls to the wrong task without an
  error.

### Packs, index and scale

- Work packs are 800 tokens of signatures by default. Lexical (BM25) and semantic hits are
  combined by rank fusion.
- Overlay index (Plan 7): a task reads main's index plus its own changes and no longer copies the
  index at start. A start that waits for its turn creates nothing until it gets one.
- Hooks show other tasks' records when an agent opens a file.

### For code built on the daemon

- Extension points in `scholion_daemon`: `router_with`, `with_extension`, `subscribe_events`,
  `MergeGuard` (asked before submit, approve and merge), `with_mcp_tools`, `UsageSink`
  (`POST /v1/agents/{name}/usage`), `put_profile` and `AppState::migrate_extension`. An extension
  runs its own migrations in its own Postgres schema, so its migration numbers never clash with
  the daemon's. `run_with` is the daemon's whole start-up as a library call, so an extension
  binary adds its routes and services without copying `main.rs`.
- `scholion-agentrun`: a library for running an agent CLI (argv, environment, deadline that kills
  the process group, parsed usage). The benchmark uses it.

### Upgrading from 0.0.1

- Stop the daemon and install the new binaries (rerun the installer, or `git pull` and
  `docker compose -f deploy/docker-compose.yml up -d --build`). The daemon adds its new tables
  (migrations 0021-0026) at start-up; existing tasks and records stay as they were.
- Restart your agent CLIs so they load the new tool parameters.

### Evidence

- A real run: two Claude Code agents on DeepSeek Flash took tasks from the backlog, reviewed each
  other's work and merged it. Reproduce it with `examples/routing-smoke/deepseek-demo.sh`; that
  run cost about $0.03. It shows that the loop works. It is not a measurement of token savings.
- Benchmarks and their limits: `bench/results/`. Plain notes files delivered recorded lessons as
  well as av did at 10 notes (E4). The task set with 300 notes (`bench/tasks-relay-scale`) is
  ready but has not been run, so av's advantage at scale is not measured yet.

## 0.0.1 (2026-10-28)

First release: the `scholion` CLI (alias `av`). The daemon ran from Docker
(`deploy/docker-compose.yml`).
