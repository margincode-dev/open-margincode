# M1 Part 1, the free replay: results, 2026-11-13

No agent ran, no model was called, nothing was spent. The registration is `PREREG.md` in this
directory; its "Build" section (Part 1) was committed in `597be88` before the first `first-packs`
command. Binaries built from `850359b` (tree `e7fcaa3`).

## Commands as run

Inside the harness container `m1-harness` (`rust:1.97.1`, this checkout at `/src`,
`PATH=/target/release:$PATH`), against the release daemon at `127.0.0.1:7450` described under
"Build":

```bash
OUT=bench/results/2026-11-13-standing-and-dreams
COMMON="--av-url http://127.0.0.1:7450 --work-root /root/bench-work --import-dir /root/bench-work/src --import-mount /root/bench-work/src"
margincode-bench check bench/tasks-relay-wrong bench/tasks-relay        # all 15: ok
margincode-bench first-packs bench/tasks-relay-wrong --wrong-reverted --out $OUT/m1-reverted $COMMON
margincode-bench first-packs bench/tasks-relay-wrong                  --out $OUT/m1-open     $COMMON
for n in 0 50 500; do
  margincode-bench first-packs bench/tasks-relay --noise $n --out $OUT/m1-true-$n $COMMON
done
```

All five exited 0, one after the other on one daemon and one database, in the order above
(noise seed 1). Each wrote `first-packs.jsonl`, `first-packs.md` and `packs/`.

## B1: the wrong record is not at `summary` or `full` in the hook pack, at least 4 of 5

| task | `m1-reverted` hook pack | `m1-open` hook pack | `m1-reverted` pre-read has it | `m1-open` pre-read has it |
|---|---|---|---|---|
| relay-coupon-remove | handle | summary | yes | no |
| relay-line-taxes | handle | summary | yes | no |
| relay-money-plain | handle | summary | yes | no |
| relay-quote-split | handle | summary | no | no |
| relay-store-count | handle | summary | yes (`store/base.py`) | no |

**B1: 5 of 5, pass.** In `m1-reverted` the wrong record is at the `handle` tier in every hook
pack, listed as `decision, reverted: …` and cut to 70 characters. In `m1-open`, the same build
without the flag, it is at `summary` in 5 of 5, so B1 passing is the flag's doing, not the
build's: without it 0 of 5 tasks meet the bar.

Not a bar, and reported because it bears on Part 2: with the flag, the pre-read answer for a
relevant file carries the wrong record in 4 of 5 tasks (a pre-read answer carries the whole
summary), against 0 of 5 without it. "Lesson shown in full" in `first-packs.md` is therefore 4
of 5 in `m1-reverted` and 5 of 5 in `m1-open`. The demotion moves the wrong note from the hook
pack into the pre-read answer more than it takes it away from the agent. (Why the pre-read
answer is empty of it in `m1-open` is inferred, not checked: the hook pack already sent it in
full under the same session, and the pre-read answer leaves out what the session was given.)

## B2: on the true-lesson set, every lesson shown in full, 10 of 10 at each N

| N | shown in full | hook pack: summary / handle / absent | in full only through pre-read |
|---|---|---|---|
| 0 | **10 of 10** | 10 / 0 / 0 | none |
| 50 | **10 of 10** | 7 / 2 / 1 | coupon-remove, money-plain (handle); order-ids (absent) |
| 500 | **10 of 10** | 8 / 0 / 2 | order-ids, refund-event (absent) |

**B2: pass at N = 0, 50 and 500.** Relevant files named in the hook pack: 14 of 14 at each N; no
pack marked degraded.

## Decision

B1 and B2 both pass, so the registration's decision rule allows Part 2. Part 2 was not run.

## Deviations and notes

- **The `check` command** is not among the registration's commands; it was run on both task
  sets before the first replay, as `bench/README.md` asks, and changed nothing.
- **Docker engine restart before the build.** During the first build attempt (it had not got past
  updating the crates.io index), every container on the machine exited at once with code 255
  (16:27 UTC). The `m1-` containers were started again and the build was rerun with `-j 6`;
  nothing of the measurement ran before that. The cause was not determined.
- **The tallyho repo** was first built from the bind-mounted checkout, where Docker Desktop shows
  every file as mode 755; it was removed and rebuilt from `git archive HEAD bench/tallyho`, so the
  files keep their committed modes. Only the second one was used.
- Run ids are `fp<pid>` per command (the harness's own); the workspaces were removed after each
  task (no `--keep-work`).
