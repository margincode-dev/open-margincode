#!/usr/bin/env bash
# Posts the first backlog for the Margincode foundry (a standing Margincode Cloud cell that develops
# Margincode): the loop's own noise, taken from the first dogfood run (2026-11-03). Each item is one
# `av post` with a goal written for an agent that has not seen that run.
#
#   AV_URL=http://127.0.0.1:7420 AV_TOKEN=... scripts/foundry-backlog.sh
#
# Environment:
#   AV_URL, AV_TOKEN   the daemon and its token, read by `av` itself (no flags needed)
#   AV_AGENT           who the items are posted by (default: av's own default)
#   AV                 the av binary (default: av)
#   REPO               the repository name in the daemon (default: margincode)
#   DRY_RUN=1          print what would be posted and post nothing
#
# Idempotent-ish: it prints each item it posts, and skips an item whose title is already on the
# repo's backlog (queued, taken or dropped) and a target that already exists, so a rerun posts
# only what is missing. It does not edit an item that exists with different text.
#
# Items count toward the target "Loop noise" (slug loop-noise). `av post --target` takes the slug,
# so no separate `PUT /v1/backlog/{id}/target` call is needed.
#
# Not included: restarting the owner's lease on request_changes (dogfood sev-3 #3). That fix is in
# flight as its own change.
set -euo pipefail

AV="${AV:-av}"
REPO="${REPO:-margincode}"
TARGET_TITLE="Loop noise"
TARGET_SLUG="loop-noise"
DRY_RUN="${DRY_RUN:-0}"

command -v "$AV" >/dev/null 2>&1 || {
  echo "foundry-backlog: $AV not found (set AV=/path/to/av)" >&2
  exit 1
}

run() {
  if [[ "$DRY_RUN" == 1 ]]; then
    echo "DRY RUN: would run: $*"
  else
    "$@"
  fi
}

if [[ "$DRY_RUN" == 1 ]]; then
  existing_backlog=""
  existing_targets=""
else
  existing_backlog="$("$AV" backlog --repo "$REPO")"
  existing_targets="$("$AV" target list --repo "$REPO" --all)"
fi

if grep -q "^${TARGET_SLUG} " <<<"$existing_targets"; then
  echo "target ${TARGET_SLUG}: exists"
else
  echo "adding target: ${TARGET_TITLE}"
  run "$AV" target add --repo "$REPO" --title "$TARGET_TITLE" \
    --detail "The foundry's own friction from the first dogfood run (2026-11-03): fixes that make the agent loop cheaper and its verdicts trustworthy."
fi

# post PRIORITY TITLE GOAL
post() {
  local priority="$1" title="$2" goal="$3"
  if grep -Fq -- "$title" <<<"$existing_backlog"; then
    echo "skip (already on the backlog): $title"
    return
  fi
  echo "posting p${priority}: $title"
  run "$AV" post --repo "$REPO" --priority "$priority" --target "$TARGET_SLUG" \
    --title "$title" --goal "$goal"
}

post 50 "Validation summaries show the worst check and name failing tests" \
  "A validation record's summary names only the main command, so a validation whose required convention check (the full bar inside it) failed can read 'cargo fmt --all --check exit 0'. In the first dogfood run 12 of 25 validations failed but only 4 said so in their summary; a lead scanning records or the PR bundle saw green. Make the summary reflect the worst check that ran (any failed check makes the headline failed and names that check), and when a validation fails make the output name the failing tests, not only 'exit 101'. A 'check passed' line and a 'validation FAILED' line must not print together without saying which check failed. Add tests with a validation whose first check passes and second fails. (Dogfood 2026-11-03, severity 3 item 5.)"

post 40 "Reviewer attestations outside the claim fence, no clobbering, clean child env" \
  "Three faults in how a validation is attested. (1) A reviewer cannot record a validation on someone else's claimed task: the claim-generation fence treats it as a stale owner write ('claim lost'), so the reviewer's attestation needs its own path, outside the owner's fence, and still counts toward approval. (2) Approval follows the latest attestation, so concurrent runs in one directory clobber each other and a stray red run (a killed clippy, a timing flake) blocks or flips an approval; make the newest complete run count, so an interrupted run never replaces a finished one. (3) av validate leaks AV_* variables (AV_GENERATION made 8 CLI tests fail) into the tests it runs: strip every AV_* variable from the child environment of av validate, and make the CLI test helper strip AV_GENERATION too. Add tests for each. (Dogfood 2026-11-03, severity 2 items 6 and 7.)"

post 30 "Review diff from the merge base, not two-dot against main" \
  "av diff --from main --to task:SLUG is a two-dot diff against the current main, so changes merged since the task branched show up as deletions: a reviewer of one task saw another task's merged work as removed. Reviewers need the diff from the merge base to the task head (three-dot). Make the review diff, the bundle's Changes section and the CLI diff used for review compute from the merge base; keep a way to ask for the two-dot form explicitly. The bundle's Tests line should say which head the shown validation was run at, so a stale result is visible. Add a test where main moves after the task branched. (Dogfood 2026-11-03, severity 2 item 11.)"

post 20 "next explains an empty answer; the backlog pack shows the goal" \
  "Two gaps that cost agents time. First, av next with nothing to take gives no reason: an agent was told nothing while 3 items were queued but reserved for better-fitting agents. Make the answer say why it is empty, for example 'queued 3, reserved for sonnet until 03:10, waiting on 1 item' and what it would take (a different agent, waiting, a repo). Second, the context pack for a backlog item shows its title but not its goal (three agents reported it and ran av bundle or av expand to learn what to build). Put the goal in the pack's Task section within the token budget. Add tests for both. (Dogfood 2026-11-03, severity 2 items 9 and 12.)"

post 10 "Refuse a second approve on an already-approved head" \
  "Two reviewers approved the same head 7 seconds apart, and the second leaned on the first one's validation: a wasted review unit and a misleading audit trail. A review verdict of approve on a head that is already approved should be refused with an answer that names who approved it and when, and next must not offer a task someone is mid-review on. While there, identical review findings stored twice (a retried av review) should be stored once. Keep request_changes and a re-review after a new head working as today. Add tests for the duplicate approve and for approve after a new submit. (Dogfood 2026-11-03, 'double review' at 03:50.)"

echo "done."
