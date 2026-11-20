# M3 gate, step 1: areas with 8 or more live records, 2026-11-20

**Counts, not a claim.** Plan 16 (hivemind), Phase 4 (pages), "Measurement, M3", step 1: "Count,
on the foundry's own database, how many areas of our three repositories have 8 or more live
records. If fewer than five areas qualify, pages have no user yet and the phase waits." Free and
read-only: SELECTs only, every session with `default_transaction_read_only=on`.

**Verdict, on the rule below: 3 areas qualify on the foundry, fewer than five. Phase 4 waits.**
It depends on the reading of "area" (see "Depth-one directories"): counting the depth-one
directories as areas too gives exactly 5. Which reading the gate means is the owner's call.

## Definitions

- **Area**: the rule of the Cloud's `cost_by_area` report, applied to each anchor path: the first
  two segments (`crates/margincode-daemon`), or the only segment of a path at depth one
  (`README.md`). A path of exactly two segments is its own area, so `docs/reference.md` is an
  "area" that is one file; the tables mark those.
- **Single file**: an anchor path itself.
- **Live agent-written record** (`PAGE_MIN_RECORDS` is 8):
  - kind `decision`, `attempt` or `note`, the kinds a dream judges (`dream_candidates`); this
    leaves out `intent`, `handoff`, `review` and `validation` ("Kinds" below shows the verdict
    does not depend on it);
  - not mined from git (`author_agent <> 'git'`, `margincode_core::history::MINED_AUTHOR`);
  - not superseded (no record has `supersedes` = its id);
  - no effective `disputed`, `duplicate` or `outdated` verdict: not retracted, and its newest
    review agrees (`VERDICT_STATE` in `margincode-store`'s `dreams.rs`). The plan's staleness
    rule counts `disputed` too; `dream_candidates` hides only `duplicate` and `outdated`.
- A record belongs to every area one of its anchors falls in, and is counted once per area.

On both databases there are 0 verdicts and 0 mined records, so those two clauses filter nothing
today. On the foundry no record of the three kinds is superseded either (all 57 are live); on the
dogfood 6 of 207 are.

## Data

- **The foundry**: its own Postgres, database `av`, migrations to 0032. Repositories as imported:
  `scholion`, `agentvcs`, `openevaluations`. The newest record is from 2026-11-11 03:29 UTC; every
  anchor path is from before the rename (`ee/scholion-*`, `crates/scholion-*`).
- **The dogfood daemon** (a second data point, not the gate): its embedded Postgres, database
  `scholion`, migrations to 0032, one repository `open-margincode`. It is live: the numbers are
  the run of 2026-11-20 04:42 UTC, and they include this task's own records (its intent, a
  validation, and its decision anchored to this file, which puts `bench/results` at 6, not 5).
  The foundry's numbers were the same on both runs.

## Results

Totals (all kinds; live = the predicate above):

| database | repository | tasks | records | live | live and anchored |
|---|---|---:|---:|---:|---:|
| foundry | agentvcs | 3 | 48 | 37 | 33 |
| foundry | scholion | 8 | 44 | 20 | 13 |
| foundry | openevaluations | 0 | 0 | 0 | 0 |
| dogfood | open-margincode | 97 | 469 | 201 | 167 |

Areas (the rule above) by threshold; "of which files" counts areas that are a file:

| database | repository | areas | ≥ 8 | ≥ 5 | ≥ 3 | of which files at ≥ 8 / 5 / 3 |
|---|---|---:|---:|---:|---:|---|
| foundry | agentvcs | 5 | 2 | 2 | 2 | 0 / 0 / 0 |
| foundry | scholion | 4 | 1 | 1 | 1 | 0 / 0 / 0 |
| foundry | openevaluations | 0 | 0 | 0 | 0 | 0 / 0 / 0 |
| foundry | **all three** | 9 | **3** | 3 | 3 | 0 / 0 / 0 |
| dogfood | open-margincode | 25 | 6 | 7 | 11 | 1 / 1 / 2 |

Top areas (the foundry has 9 in all, the dogfood's top 15):

| foundry | live | | dogfood | live |
|---|---:|---|---|---:|
| agentvcs `ee/scholion-stemma` | 17 | | `crates/margincode-daemon` | 66 |
| agentvcs `ee/scholion-stemma-jj` | 12 | | `crates/margincode-cli` | 41 |
| scholion `crates/scholion-daemon` | 11 | | `crates/margincode-vcs` | 20 |
| agentvcs `ee/scholion-ee-daemon` | 2 | | `crates/margincode-core` | 9 |
| agentvcs `vendor/scholion` | 2 | | `crates/margincode-pack` | 9 |
| scholion `crates/scholion-store` | 2 | | `docs/reference.md` (file) | 9 |
| agentvcs `ee/scholion-runner` | 1 | | `bench/results` | 6 |
| scholion `crates/scholion-cli` | 1 | | `crates/margincode-store` | 4 |
| scholion `crates/scholion-vcs` | 1 | | `docs/backlog.md` (file) | 4 |
| | | | `integrations/dream` | 4 |
| | | | `crates/margincode-bench` | 3 |
| | | | `.av/sparse` (file) | 2 |
| | | | `constitution.md` (file) | 2 |
| | | | `crates/margincode-agentrun` | 2 |
| | | | `deploy/Dockerfile` (file) | 2 |

Single files by threshold:

| database | repository | files | ≥ 8 | ≥ 5 | ≥ 3 |
|---|---|---:|---:|---:|---:|
| foundry | agentvcs | 16 | 0 | 2 | 5 |
| foundry | scholion | 7 | 0 | 1 | 1 |
| foundry | openevaluations | 0 | 0 | 0 | 0 |
| dogfood | open-margincode | 67 | 9 | 13 | 25 |

The foundry's most-anchored files: agentvcs `ee/scholion-stemma/src/import.rs` 7, scholion
`crates/scholion-daemon/src/flow.rs` 7, agentvcs `ee/scholion-stemma/src/patch.rs` 6. The
dogfood's nine at ≥ 8: `crates/margincode-cli/src/main.rs`, `crates/margincode-daemon/src/flow.rs`,
`crates/margincode-daemon/src/mcp.rs`, `crates/margincode-daemon/tests/routing.rs`,
`crates/margincode-vcs/src/jj.rs` (11 each), `crates/margincode-daemon/src/routing_api.rs`,
`docs/reference.md` (9 each), `crates/margincode-cli/src/hook.rs`, `crates/margincode-cli/src/notes.rs`
(8 each).

### Depth-one directories

The plan says "at most two segments deep", which also admits a one-segment directory; the
`cost_by_area` rule never yields one. Live records under each first segment (paths of two or
more segments):

| database | repository | directory | live |
|---|---|---|---:|
| foundry | agentvcs | `ee` | 31 |
| foundry | agentvcs | `vendor` | 2 |
| foundry | scholion | `crates` | 13 |
| dogfood | open-margincode | `crates` | 140 |
| dogfood | open-margincode | `docs` | 13 |
| dogfood | open-margincode | `integrations` | 6 |

On this reading the foundry has 3 + 2 = 5 areas at ≥ 8, exactly the gate. Each record under
`ee` or `crates` is also in one of their two-segment areas: the two added areas are the union of
areas already in the table above.

### Kinds

Including every anchored kind (not superseded) instead of the three above adds no foundry area at
≥ 8: the three stay the only ones (18, 15, 12), and no other area reaches 5. The excluded
anchored records on the foundry are 1 `intent` and 4 `review` findings; on the dogfood 9 `intent`
and 23 `review`.

## SQL

Run as given with `psql -At`, the session read-only (`PGOPTIONS='-c default_transaction_read_only=on'`).
Each query starts with the same two CTEs:

```sql
WITH live AS (
  SELECT r.id, t.repo
  FROM records r JOIN tasks t ON t.id = r.task_id
  WHERE r.kind IN ('decision', 'attempt', 'note')
    AND r.author_agent <> 'git'
    AND NOT EXISTS (SELECT 1 FROM records n WHERE n.supersedes = r.id)
    AND NOT EXISTS (
      SELECT 1 FROM record_verdicts v
      WHERE v.record_id = r.id
        AND v.verdict IN ('disputed', 'duplicate', 'outdated')
        AND NOT EXISTS (SELECT 1 FROM record_verdicts x WHERE x.retracts = v.id)
        AND COALESCE((SELECT w.agree FROM verdict_reviews w WHERE w.verdict_id = v.id
                      ORDER BY w.id DESC LIMIT 1), false))
), la AS (
  SELECT DISTINCT l.repo, l.id, a.path,
         CASE WHEN split_part(a.path, '/', 2) = '' THEN split_part(a.path, '/', 1)
              ELSE split_part(a.path, '/', 1) || '/' || split_part(a.path, '/', 2)
         END AS area
  FROM live l JOIN anchors a ON a.record_id = l.id
)
```

followed by one of:

```sql
-- totals
SELECT t.repo, count(*), count(*) FILTER (WHERE r.id IN (SELECT id FROM live)),
       count(*) FILTER (WHERE r.id IN (SELECT id FROM la))
FROM records r JOIN tasks t ON t.id = r.task_id GROUP BY 1 ORDER BY 1;

-- areas by threshold
SELECT repo, count(*) FILTER (WHERE n >= 8), count(*) FILTER (WHERE n >= 5),
       count(*) FILTER (WHERE n >= 3), count(*)
FROM (SELECT repo, area, count(DISTINCT id) n FROM la GROUP BY 1, 2) s GROUP BY 1 ORDER BY 1;

-- the same, split into areas that are a file (file = t) and directories
SELECT repo, file, count(*) FILTER (WHERE n >= 8), count(*) FILTER (WHERE n >= 5),
       count(*) FILTER (WHERE n >= 3), count(*)
FROM (SELECT repo, area, bool_and(path = area) file, count(DISTINCT id) n FROM la GROUP BY 1, 2) s
GROUP BY 1, 2 ORDER BY 1, 2;

-- top areas
SELECT repo, area, count(DISTINCT id) n FROM la GROUP BY 1, 2 ORDER BY n DESC, repo, area LIMIT 15;

-- single files by threshold, and the top ones
SELECT repo, count(*) FILTER (WHERE n >= 8), count(*) FILTER (WHERE n >= 5),
       count(*) FILTER (WHERE n >= 3), count(*)
FROM (SELECT repo, path, count(DISTINCT id) n FROM la GROUP BY 1, 2) s GROUP BY 1 ORDER BY 1;
SELECT repo, path, count(DISTINCT id) n FROM la GROUP BY 1, 2 ORDER BY n DESC, repo, path LIMIT 10;

-- depth-one directories
SELECT repo, split_part(path, '/', 1) d, count(DISTINCT id) n FROM la WHERE path LIKE '%/%'
GROUP BY 1, 2 ORDER BY n DESC LIMIT 10;
```

Without the CTEs:

```sql
-- every anchored kind, not superseded, per area (the kinds check)
SELECT t.repo, CASE WHEN split_part(a.path, '/', 2) = '' THEN split_part(a.path, '/', 1)
  ELSE split_part(a.path, '/', 1) || '/' || split_part(a.path, '/', 2) END ar, count(DISTINCT r.id) n
FROM anchors a JOIN records r ON r.id = a.record_id JOIN tasks t ON t.id = r.task_id
WHERE NOT EXISTS (SELECT 1 FROM records x WHERE x.supersedes = r.id)
GROUP BY 1, 2 HAVING count(DISTINCT r.id) >= 5 ORDER BY n DESC;

-- anchored records by kind
SELECT t.repo, r.kind, count(DISTINCT r.id) FROM anchors a JOIN records r ON r.id = a.record_id
JOIN tasks t ON t.id = r.task_id GROUP BY 1, 2 ORDER BY 1, 2;

-- tasks per repository, verdicts, newest record
SELECT r.name, (SELECT count(*) FROM tasks t WHERE t.repo = r.name) FROM repos r ORDER BY 1;
SELECT count(*) FROM record_verdicts;
SELECT max(created_at) FROM records;
```
