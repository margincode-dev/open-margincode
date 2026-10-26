use std::collections::HashMap;
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use globset::Glob;
use scholion_core::*;
use sqlx::Row;
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use uuid::Uuid;

#[derive(Clone)]
pub struct PgStore {
    pool: PgPool,
}

pub(crate) fn serr(e: sqlx::Error) -> AvError {
    AvError::Store(e.to_string())
}

fn slugify(title: &str) -> String {
    let mut slug = String::new();
    for c in title.chars() {
        if c.is_alphanumeric() {
            slug.extend(c.to_lowercase());
        } else if !slug.ends_with('-') {
            slug.push('-');
        }
    }
    let slug: String = slug.trim_matches('-').chars().take(40).collect();
    let slug = slug.trim_end_matches('-').to_owned();
    if slug.is_empty() { "task".into() } else { slug }
}

fn task_from_row(r: &PgRow) -> Result<Task> {
    let status: String = r.try_get("status").map_err(serr)?;
    Ok(Task {
        id: TaskId(r.try_get("id").map_err(serr)?),
        repo: r.try_get("repo").map_err(serr)?,
        slug: r.try_get("slug").map_err(serr)?,
        title: r.try_get("title").map_err(serr)?,
        goal: r.try_get("goal").map_err(serr)?,
        status: status.parse()?,
        owner_agent: r.try_get("owner_agent").map_err(serr)?,
        base_change: ChangeId(r.try_get("base_change").map_err(serr)?),
        head_change: ChangeId(r.try_get("head_change").map_err(serr)?),
        workspace_path: PathBuf::from(r.try_get::<String, _>("workspace_path").map_err(serr)?),
        generation: r.try_get("generation").map_err(serr)?,
        lease_expires: r.try_get("lease_expires").map_err(serr)?,
        pack_mode: match r.try_get::<String, _>("pack_mode").map_err(serr)?.as_str() {
            "task_only" => PackMode::TaskOnly,
            _ => PackMode::Full,
        },
    })
}

const TASK_COLS: &str = "id, repo, slug, title, goal, status, owner_agent, base_change, head_change, \
                         workspace_path, generation, lease_expires, pack_mode";

impl PgStore {
    /// Deletes the ledgers and expand totals of sessions that wrote nothing for `idle`; returns
    /// how many sessions were removed.
    pub async fn prune_sessions(&self, idle: std::time::Duration) -> Result<u64> {
        let secs = idle.as_secs_f64();
        let mut tx = self.pool.begin().await.map_err(serr)?;
        let gone: Vec<String> = sqlx::query_scalar(
            "WITH idle AS (
                 SELECT session FROM ledger GROUP BY session
                 HAVING max(last_seen) < now() - make_interval(secs => $1)
               UNION
                 SELECT session FROM ledger_sessions s
                 WHERE last_seen < now() - make_interval(secs => $1)
                   AND NOT EXISTS (SELECT 1 FROM ledger l WHERE l.session = s.session
                                   AND l.last_seen >= now() - make_interval(secs => $1))
             )
             SELECT session FROM idle",
        )
        .bind(secs)
        .fetch_all(&mut *tx)
        .await
        .map_err(serr)?;
        for table in ["ledger", "ledger_sessions"] {
            sqlx::query(&format!("DELETE FROM {table} WHERE session = ANY($1)"))
                .bind(&gone)
                .execute(&mut *tx)
                .await
                .map_err(serr)?;
        }
        tx.commit().await.map_err(serr)?;
        Ok(gone.len() as u64)
    }

    /// Connects (pool of 8) and runs migrations.
    pub async fn connect(url: &str) -> Result<Self> {
        Self::connect_with(url, 8).await
    }

    /// Connects with a pool of at most `max_connections` and runs migrations.
    pub async fn connect_with(url: &str, max_connections: u32) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections.max(1))
            .connect(url)
            .await
            .map_err(serr)?;
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(|e| AvError::Store(e.to_string()))?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub(crate) async fn hydrate_records(&self, rows: Vec<PgRow>) -> Result<Vec<Record>> {
        let mut out: Vec<Record> = Vec::with_capacity(rows.len());
        for r in &rows {
            let kind: String = r.try_get("kind").map_err(serr)?;
            out.push(Record {
                id: RecordId(r.try_get("id").map_err(serr)?),
                created_at: r.try_get("created_at").map_err(serr)?,
                task_id: TaskId(r.try_get("task_id").map_err(serr)?),
                kind: kind.parse()?,
                summary: r.try_get("summary").map_err(serr)?,
                body: r.try_get("body").map_err(serr)?,
                change_id: r
                    .try_get::<Option<String>, _>("change_id")
                    .map_err(serr)?
                    .map(ChangeId),
                supersedes: r
                    .try_get::<Option<Uuid>, _>("supersedes")
                    .map_err(serr)?
                    .map(RecordId),
                superseded_by: r
                    .try_get::<Option<Uuid>, _>("superseded_by")
                    .map_err(serr)?
                    .map(RecordId),
                author_agent: r.try_get("author_agent").map_err(serr)?,
                anchors: vec![],
                anchor_hashes: vec![],
            });
        }
        let ids: Vec<Uuid> = out.iter().map(|r| r.id.0).collect();
        let anchor_rows = sqlx::query("SELECT record_id, path, symbol, content_hash FROM anchors WHERE record_id = ANY($1) ORDER BY path COLLATE \"C\", symbol COLLATE \"C\"")
            .bind(&ids)
            .fetch_all(&self.pool)
            .await
            .map_err(serr)?;
        let mut by_record: HashMap<Uuid, Vec<(Anchor, Option<String>)>> = HashMap::new();
        for a in anchor_rows {
            by_record
                .entry(a.try_get("record_id").map_err(serr)?)
                .or_default()
                .push((
                    Anchor {
                        path: a.try_get("path").map_err(serr)?,
                        symbol: a.try_get("symbol").map_err(serr)?,
                    },
                    a.try_get("content_hash").map_err(serr)?,
                ));
        }
        for r in &mut out {
            (r.anchors, r.anchor_hashes) = by_record
                .remove(&r.id.0)
                .unwrap_or_default()
                .into_iter()
                .unzip();
        }
        Ok(out)
    }
}

pub(crate) const RECORD_COLS: &str = "r.id, r.task_id, r.kind, r.summary, r.body, r.change_id, r.supersedes, r.author_agent, r.created_at,
    (SELECT n.id FROM records n WHERE n.supersedes = r.id ORDER BY n.created_at LIMIT 1) AS superseded_by";

#[async_trait]
impl IntentStore for PgStore {
    async fn add_verdict(
        &self,
        task: TaskId,
        reviewer: &str,
        verdict: Verdict,
        head: &ChangeId,
        base: Option<&ChangeId>,
        carried_from: Option<&ChangeId>,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO review_verdicts (task_id, reviewer, verdict, head_change, base_change, carried_from)
             VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(task.0)
        .bind(reviewer)
        .bind(verdict.as_str())
        .bind(&head.0)
        .bind(base.map(|b| &b.0))
        .bind(carried_from.map(|c| &c.0))
        .execute(&self.pool)
        .await
        .map_err(serr)?;
        Ok(())
    }

    async fn latest_verdict_any(&self, task: TaskId) -> Result<Option<VerdictRow>> {
        let row = sqlx::query(
            "SELECT reviewer, verdict, head_change, base_change, carried_from FROM review_verdicts
             WHERE task_id = $1 ORDER BY id DESC LIMIT 1",
        )
        .bind(task.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(serr)?;
        row.map(|r| verdict_row(&r)).transpose()
    }

    async fn human_verdict_at(&self, task: TaskId, head: &ChangeId) -> Result<Option<VerdictRow>> {
        let row = sqlx::query(
            "SELECT reviewer, verdict, head_change, base_change, carried_from FROM review_verdicts
             WHERE task_id = $1 AND head_change = $2 AND reviewer <> 'av' ORDER BY id DESC LIMIT 1",
        )
        .bind(task.0)
        .bind(&head.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(serr)?;
        row.map(|r| verdict_row(&r)).transpose()
    }

    async fn add_check(&self, repo: &str, c: &CheckRow) -> Result<()> {
        sqlx::query(
            "INSERT INTO checks (repo, change, command, exit_code, output_tail, runner)
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (repo, change, command) DO UPDATE
             SET exit_code = $4, output_tail = $5, runner = $6, at = now()",
        )
        .bind(repo)
        .bind(&c.change)
        .bind(&c.command)
        .bind(c.exit_code)
        .bind(&c.output_tail)
        .bind(&c.runner)
        .execute(&self.pool)
        .await
        .map_err(serr)?;
        Ok(())
    }

    async fn checks_for(&self, repo: &str, changes: &[String]) -> Result<Vec<CheckRow>> {
        let rows = sqlx::query(
            "SELECT change, command, exit_code, output_tail, runner, at FROM checks
             WHERE repo = $1 AND change = ANY($2) ORDER BY at, id",
        )
        .bind(repo)
        .bind(changes)
        .fetch_all(&self.pool)
        .await
        .map_err(serr)?;
        rows.iter()
            .map(|r| {
                Ok(CheckRow {
                    change: r.try_get("change").map_err(serr)?,
                    command: r.try_get("command").map_err(serr)?,
                    exit_code: r.try_get("exit_code").map_err(serr)?,
                    output_tail: r.try_get("output_tail").map_err(serr)?,
                    runner: r.try_get("runner").map_err(serr)?,
                    at: r.try_get("at").map_err(serr)?,
                })
            })
            .collect()
    }

    async fn derived_get(
        &self,
        repo: &str,
        kind: &str,
        change: &ChangeId,
        key: &str,
    ) -> Result<Option<serde_json::Value>> {
        sqlx::query_scalar(
            "SELECT value FROM derived WHERE repo = $1 AND kind = $2 AND change = $3 AND key = $4",
        )
        .bind(repo)
        .bind(kind)
        .bind(&change.0)
        .bind(key)
        .fetch_optional(&self.pool)
        .await
        .map_err(serr)
    }

    async fn derived_put(
        &self,
        repo: &str,
        kind: &str,
        change: &ChangeId,
        key: &str,
        value: &serde_json::Value,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO derived (repo, kind, change, key, value) VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT (repo, kind, change, key) DO UPDATE SET value = $5, computed_at = now()",
        )
        .bind(repo)
        .bind(kind)
        .bind(&change.0)
        .bind(key)
        .bind(value)
        .execute(&self.pool)
        .await
        .map_err(serr)?;
        Ok(())
    }

    async fn repo_settings(&self, repo: &str) -> Result<RepoSettings> {
        let row =
            sqlx::query("SELECT carry_globs, check_command FROM repo_settings WHERE repo = $1")
                .bind(repo)
                .fetch_optional(&self.pool)
                .await
                .map_err(serr)?;
        Ok(match row {
            Some(r) => RepoSettings {
                carry_globs: r.try_get("carry_globs").map_err(serr)?,
                check_command: r.try_get("check_command").map_err(serr)?,
            },
            None => RepoSettings::default(),
        })
    }

    async fn set_repo_settings(&self, repo: &str, settings: &RepoSettings) -> Result<()> {
        for g in &settings.carry_globs {
            Glob::new(g).map_err(|e| AvError::Invalid(format!("bad glob {g:?}: {e}")))?;
        }
        sqlx::query(
            "INSERT INTO repo_settings (repo, carry_globs, check_command) VALUES ($1, $2, $3)
             ON CONFLICT (repo) DO UPDATE SET carry_globs = $2, check_command = $3",
        )
        .bind(repo)
        .bind(&settings.carry_globs)
        .bind(&settings.check_command)
        .execute(&self.pool)
        .await
        .map_err(serr)?;
        Ok(())
    }

    async fn latest_verdict(
        &self,
        task: TaskId,
        head: &ChangeId,
    ) -> Result<Option<(String, Verdict)>> {
        let row = sqlx::query(
            "SELECT reviewer, verdict FROM review_verdicts
             WHERE task_id = $1 AND head_change = $2 ORDER BY id DESC LIMIT 1",
        )
        .bind(task.0)
        .bind(&head.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(serr)?;
        row.map(|r| {
            let verdict: String = r.try_get("verdict").map_err(serr)?;
            Ok((r.try_get("reviewer").map_err(serr)?, verdict.parse()?))
        })
        .transpose()
    }

    async fn merge_state(&self, task: TaskId) -> Result<MergeState> {
        let row = sqlx::query(
            "SELECT pre_merge_commit, merge_commit, merged_by, conflicts FROM tasks WHERE id = $1",
        )
        .bind(task.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(serr)?
        .ok_or_else(|| AvError::NotFound(format!("task {task}")))?;
        let change = |col: &str| -> Result<Option<ChangeId>> {
            Ok(row
                .try_get::<Option<String>, _>(col)
                .map_err(serr)?
                .map(ChangeId))
        };
        Ok(MergeState {
            pre_merge: change("pre_merge_commit")?,
            merge_commit: change("merge_commit")?,
            merged_by: row.try_get("merged_by").map_err(serr)?,
            conflicts: row.try_get("conflicts").map_err(serr)?,
        })
    }

    async fn set_merge_state(&self, task: TaskId, state: &MergeState) -> Result<()> {
        let n = sqlx::query(
            "UPDATE tasks SET pre_merge_commit = $2, merge_commit = $3, merged_by = $4, conflicts = $5,
                    merged_at = CASE WHEN $3::text IS NULL THEN NULL ELSE now() END
             WHERE id = $1",
        )
        .bind(task.0)
        .bind(state.pre_merge.as_ref().map(|c| c.0.as_str()))
        .bind(state.merge_commit.as_ref().map(|c| c.0.as_str()))
        .bind(state.merged_by.as_deref())
        .bind(&state.conflicts)
        .execute(&self.pool)
        .await
        .map_err(serr)?
        .rows_affected();
        if n == 0 {
            return Err(AvError::NotFound(format!("task {task}")));
        }
        Ok(())
    }

    async fn create_task(&self, t: NewTask) -> Result<Task> {
        let base = slugify(&t.title);
        let id = Uuid::new_v4();
        for n in 1u32.. {
            let slug = if n == 1 {
                base.clone()
            } else {
                format!("{base}-{n}")
            };
            let inserted = sqlx::query(
                "INSERT INTO tasks (id, repo, slug, title, goal, owner_agent) VALUES ($1,$2,$3,$4,$5,$6)
                 ON CONFLICT (repo, slug) DO NOTHING",
            )
            .bind(id)
            .bind(&t.repo)
            .bind(&slug)
            .bind(&t.title)
            .bind(&t.goal)
            .bind(&t.owner_agent)
            .execute(&self.pool)
            .await
            .map_err(serr)?
            .rows_affected();
            if inserted == 1 {
                break;
            }
        }
        let task = self.get_task(TaskId(id)).await?;
        let mut tx = self.pool.begin().await.map_err(serr)?;
        insert_event(
            &mut tx,
            &task.repo,
            "started",
            Some(task.id),
            None,
            serde_json::json!({"agent": task.owner_agent, "slug": task.slug}),
        )
        .await?;
        tx.commit().await.map_err(serr)?;
        Ok(task)
    }

    async fn get_task(&self, id: TaskId) -> Result<Task> {
        let row = sqlx::query(&format!("SELECT {TASK_COLS} FROM tasks WHERE id = $1"))
            .bind(id.0)
            .fetch_optional(&self.pool)
            .await
            .map_err(serr)?
            .ok_or_else(|| AvError::NotFound(format!("task {id}")))?;
        task_from_row(&row)
    }

    async fn task_by_slug(&self, repo: &str, slug: &str) -> Result<Task> {
        let row = sqlx::query(&format!(
            "SELECT {TASK_COLS} FROM tasks WHERE repo = $1 AND slug = $2"
        ))
        .bind(repo)
        .bind(slug)
        .fetch_optional(&self.pool)
        .await
        .map_err(serr)?
        .ok_or_else(|| AvError::NotFound(format!("task {slug} in repo {repo}")))?;
        task_from_row(&row)
    }

    async fn set_status(&self, id: TaskId, status: TaskStatus) -> Result<()> {
        let mut tx = self.pool.begin().await.map_err(serr)?;
        let row = sqlx::query("UPDATE tasks SET status = $2 WHERE id = $1 RETURNING repo")
            .bind(id.0)
            .bind(status.as_str())
            .fetch_optional(&mut *tx)
            .await
            .map_err(serr)?
            .ok_or_else(|| AvError::NotFound(format!("task {id}")))?;
        let repo: String = row.try_get("repo").map_err(serr)?;
        insert_event(
            &mut tx,
            &repo,
            &format!("status:{}", status.as_str()),
            Some(id),
            None,
            serde_json::json!({}),
        )
        .await?;
        tx.commit().await.map_err(serr)
    }

    async fn transition(
        &self,
        id: TaskId,
        event: scholion_core::lifecycle::Event,
    ) -> Result<TaskStatus> {
        let from = self.get_task(id).await?.status;
        let to = scholion_core::lifecycle::next(from, event)?;
        let mut tx = self.pool.begin().await.map_err(serr)?;
        set_status_reporting(&mut tx, id, from, to, event).await?;
        tx.commit().await.map_err(serr)?;
        Ok(to)
    }

    async fn transition_with_record(
        &self,
        id: TaskId,
        event: scholion_core::lifecycle::Event,
        r: NewRecord,
    ) -> Result<(TaskStatus, Record)> {
        r.validate()?;
        let from = self.get_task(id).await?.status;
        let to = scholion_core::lifecycle::next(from, event)?;
        let mut tx = self.pool.begin().await.map_err(serr)?;
        set_status_reporting(&mut tx, id, from, to, event).await?;
        let record_id = insert_record(&mut tx, &r, &[], None).await?;
        tx.commit().await.map_err(serr)?;
        Ok((to, self.get_record(record_id).await?))
    }

    async fn claim_task(&self, id: TaskId, agent: &str, lease_secs: u64) -> Result<Task> {
        let row = sqlx::query(&format!(
            "UPDATE tasks
             SET status = 'open', owner_agent = $2, generation = generation + 1,
                 lease_expires = now() + make_interval(secs => $3)
             WHERE id = $1 AND owner_agent <> $2
               AND (status = 'handed_off'
                    OR (status = 'open' AND (lease_expires IS NULL OR lease_expires < now())))
             RETURNING {TASK_COLS}"
        ))
        .bind(id.0)
        .bind(agent)
        .bind(lease_secs as f64)
        .fetch_optional(&self.pool)
        .await
        .map_err(serr)?;
        if let Some(row) = row {
            let task = task_from_row(&row)?;
            let mut tx = self.pool.begin().await.map_err(serr)?;
            insert_event(
                &mut tx,
                &task.repo,
                scholion_core::lifecycle::Event::Claim.feed_kind(),
                Some(task.id),
                Some(&task.head_change.0),
                serde_json::json!({"agent": agent, "generation": task.generation}),
            )
            .await?;
            tx.commit().await.map_err(serr)?;
            return Ok(task);
        }
        // explain the refusal from the current row
        let t = self.get_task(id).await?;
        Err(AvError::IllegalState(if t.owner_agent == agent {
            format!("task {} is already yours ({agent})", t.slug)
        } else if t.status == TaskStatus::Open {
            format!(
                "{} holds task {} with a lease until {}; it can be claimed once the lease \
                 expires or after a handoff",
                t.owner_agent,
                t.slug,
                t.lease_expires
                    .map(|l| l.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))
                    .unwrap_or_default()
            )
        } else {
            format!(
                "only a handed-off task, or an open one whose lease expired, can be claimed; \
                 task {} is {}",
                t.slug,
                t.status.as_str()
            )
        }))
    }

    async fn renew_lease(&self, id: TaskId, generation: i64, lease_secs: u64) -> Result<bool> {
        let n = sqlx::query(
            "UPDATE tasks SET lease_expires = now() + make_interval(secs => $3)
             WHERE id = $1 AND generation = $2",
        )
        .bind(id.0)
        .bind(generation)
        .bind(lease_secs as f64)
        .execute(&self.pool)
        .await
        .map_err(serr)?
        .rows_affected();
        Ok(n > 0)
    }

    async fn set_pack_mode(&self, id: TaskId, mode: PackMode) -> Result<()> {
        let n = sqlx::query("UPDATE tasks SET pack_mode = $2 WHERE id = $1")
            .bind(id.0)
            .bind(mode.as_str())
            .execute(&self.pool)
            .await
            .map_err(serr)?
            .rows_affected();
        if n == 0 {
            return Err(AvError::NotFound(format!("task {id}")));
        }
        Ok(())
    }

    async fn set_workspace(&self, id: TaskId, path: &Path, base: &ChangeId) -> Result<()> {
        let n = sqlx::query("UPDATE tasks SET workspace_path = $2, base_change = $3, head_change = $3 WHERE id = $1")
            .bind(id.0)
            .bind(path.to_string_lossy().as_ref())
            .bind(&base.0)
            .execute(&self.pool)
            .await
            .map_err(serr)?
            .rows_affected();
        if n == 0 {
            return Err(AvError::NotFound(format!("task {id}")));
        }
        Ok(())
    }

    async fn set_head(&self, id: TaskId, head: &ChangeId, touched: &[String]) -> Result<()> {
        let mut tx = self.pool.begin().await.map_err(serr)?;
        let n = sqlx::query("UPDATE tasks SET head_change = $2 WHERE id = $1")
            .bind(id.0)
            .bind(&head.0)
            .execute(&mut *tx)
            .await
            .map_err(serr)?
            .rows_affected();
        if n == 0 {
            return Err(AvError::NotFound(format!("task {id}")));
        }
        sqlx::query("DELETE FROM task_touched WHERE task_id = $1")
            .bind(id.0)
            .execute(&mut *tx)
            .await
            .map_err(serr)?;
        sqlx::query("INSERT INTO task_touched (task_id, path) SELECT $1, unnest($2::text[]) ON CONFLICT DO NOTHING")
            .bind(id.0)
            .bind(touched)
            .execute(&mut *tx)
            .await
            .map_err(serr)?;
        tx.commit().await.map_err(serr)
    }

    async fn touched_paths(&self, id: TaskId) -> Result<Vec<String>> {
        let rows: Vec<(String,)> = sqlx::query_as(
            "SELECT path FROM task_touched WHERE task_id = $1 ORDER BY path COLLATE \"C\"",
        )
        .bind(id.0)
        .fetch_all(&self.pool)
        .await
        .map_err(serr)?;
        Ok(rows.into_iter().map(|(p,)| p).collect())
    }

    async fn live_tasks_touching(
        &self,
        repo: &str,
        paths: &[String],
        exclude: TaskId,
    ) -> Result<Vec<Task>> {
        let rows = sqlx::query(&format!(
            "SELECT {TASK_COLS} FROM tasks t
             WHERE t.repo = $1 AND t.status NOT IN ('merged', 'abandoned') AND t.id <> $2
               AND EXISTS (SELECT 1 FROM task_touched x WHERE x.task_id = t.id AND x.path = ANY($3))
             ORDER BY t.created_at, t.slug COLLATE \"C\""
        ))
        .bind(repo)
        .bind(exclude.0)
        .bind(paths)
        .fetch_all(&self.pool)
        .await
        .map_err(serr)?;
        rows.iter().map(task_from_row).collect()
    }

    async fn add_record_with(
        &self,
        r: NewRecord,
        anchor_hashes: &[Option<String>],
        receipt: Option<&str>,
    ) -> Result<Record> {
        r.validate()?;
        if !anchor_hashes.is_empty() && anchor_hashes.len() != r.anchors.len() {
            return Err(AvError::Invalid(format!(
                "{} anchor hashes for {} anchors",
                anchor_hashes.len(),
                r.anchors.len()
            )));
        }
        // Receipts are immutable once written, so resolving one ahead of the transaction is safe;
        // this also rejects a malformed, unknown or ambiguous id before anything is written.
        let receipt_hex = match receipt {
            Some(rc) => {
                let shown = self.get_receipt(rc).await?;
                let shown = shown["id"].as_str().unwrap_or_default();
                Some(format!("{}%", shown.trim_start_matches("rcp:")))
            }
            None => None,
        };
        let mut tx = self.pool.begin().await.map_err(serr)?;
        let record_id = insert_record(&mut tx, &r, anchor_hashes, receipt_hex).await?;
        tx.commit().await.map_err(serr)?;
        self.get_record(record_id).await
    }

    async fn task_records(&self, id: TaskId) -> Result<Vec<Record>> {
        let rows = sqlx::query(&format!(
            "SELECT {RECORD_COLS} FROM records r WHERE r.task_id = $1 ORDER BY r.created_at, r.id"
        ))
        .bind(id.0)
        .fetch_all(&self.pool)
        .await
        .map_err(serr)?;
        self.hydrate_records(rows).await
    }

    async fn records_anchored_to(
        &self,
        repo: &str,
        paths: &[String],
        symbols: &[String],
    ) -> Result<Vec<Record>> {
        let rows = sqlx::query(&format!(
            "SELECT {RECORD_COLS} FROM records r JOIN tasks t ON t.id = r.task_id
             WHERE t.repo = $1
               AND NOT EXISTS (SELECT 1 FROM records n WHERE n.supersedes = r.id)
               AND EXISTS (SELECT 1 FROM anchors a WHERE a.record_id = r.id
                           AND (a.path = ANY($2) OR a.symbol = ANY($3)))
             ORDER BY r.created_at, r.id"
        ))
        .bind(repo)
        .bind(paths)
        .bind(symbols)
        .fetch_all(&self.pool)
        .await
        .map_err(serr)?;
        self.hydrate_records(rows).await
    }

    async fn get_record(&self, id: RecordId) -> Result<Record> {
        let rows = sqlx::query(&format!(
            "SELECT {RECORD_COLS} FROM records r WHERE r.id = $1"
        ))
        .bind(id.0)
        .fetch_all(&self.pool)
        .await
        .map_err(serr)?;
        self.hydrate_records(rows)
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| AvError::NotFound(format!("record {id}")))
    }

    async fn find_record(&self, id_or_prefix: &str) -> Result<Record> {
        let p = id_or_prefix.trim().to_lowercase();
        if p.len() < 6 || !p.chars().all(|c| c.is_ascii_hexdigit() || c == '-') {
            return Err(AvError::Invalid(format!(
                "record id prefix must be >= 6 hex chars: {id_or_prefix:?}"
            )));
        }
        let ids: Vec<(Uuid,)> =
            sqlx::query_as("SELECT id FROM records WHERE id::text LIKE $1 LIMIT 2")
                .bind(format!("{p}%"))
                .fetch_all(&self.pool)
                .await
                .map_err(serr)?;
        match ids.as_slice() {
            [] => Err(AvError::NotFound(format!("record {p}"))),
            [(id,)] => self.get_record(RecordId(*id)).await,
            _ => Err(AvError::Invalid(format!("record prefix {p} is ambiguous"))),
        }
    }

    async fn add_convention(
        &self,
        repo: &str,
        scope_glob: &str,
        text: &str,
    ) -> Result<ConventionId> {
        Glob::new(scope_glob)
            .map_err(|e| AvError::Invalid(format!("bad glob {scope_glob}: {e}")))?;
        let mut tx = self.pool.begin().await.map_err(serr)?;
        let (id,): (i64,) = sqlx::query_as(
            "INSERT INTO conventions (repo, scope_glob, text) VALUES ($1,$2,$3) RETURNING id",
        )
        .bind(repo)
        .bind(scope_glob)
        .bind(text)
        .fetch_one(&mut *tx)
        .await
        .map_err(serr)?;
        PgStore::enqueue(
            &mut *tx,
            &NewOutbox {
                kind: SemanticKind::Convention,
                op: OutboxOp::Upsert,
                entity_id: id.to_string(),
                repo: repo.to_owned(),
                task_id: None,
                record_id: None,
                symbol_id: None,
                convention_id: Some(ConventionId(id)),
                path: None,
                change_id: None,
                superseded: false,
                text: Some(text.to_owned()),
            },
        )
        .await?;
        tx.commit().await.map_err(serr)?;
        Ok(ConventionId(id))
    }

    async fn set_convention_required(&self, id: ConventionId, required: bool) -> Result<()> {
        let done = sqlx::query("UPDATE conventions SET required = $2 WHERE id = $1")
            .bind(id.0)
            .bind(required)
            .execute(&self.pool)
            .await
            .map_err(serr)?;
        if done.rows_affected() == 0 {
            return Err(AvError::NotFound(format!("convention {}", id.0)));
        }
        Ok(())
    }

    async fn events_since(
        &self,
        repo: Option<&str>,
        since: i64,
        limit: usize,
    ) -> Result<Vec<AvEvent>> {
        let rows = sqlx::query(
            "SELECT id, repo, kind, task_id, change, payload, at FROM events
             WHERE id > $1 AND ($2::text IS NULL OR repo = $2) ORDER BY id LIMIT $3",
        )
        .bind(since)
        .bind(repo)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(serr)?;
        rows.iter()
            .map(|r| {
                Ok(AvEvent {
                    id: r.try_get("id").map_err(serr)?,
                    repo: r.try_get("repo").map_err(serr)?,
                    kind: r.try_get("kind").map_err(serr)?,
                    task_id: r
                        .try_get::<Option<Uuid>, _>("task_id")
                        .map_err(serr)?
                        .map(TaskId),
                    change: r.try_get("change").map_err(serr)?,
                    payload: r.try_get("payload").map_err(serr)?,
                    at: r.try_get("at").map_err(serr)?,
                })
            })
            .collect()
    }

    async fn last_event_id(&self, repo: Option<&str>) -> Result<i64> {
        sqlx::query_scalar(
            "SELECT COALESCE(max(id), 0) FROM events WHERE $1::text IS NULL OR repo = $1",
        )
        .bind(repo)
        .fetch_one(&self.pool)
        .await
        .map_err(serr)
    }

    async fn conventions(&self, repo: &str) -> Result<Vec<Convention>> {
        let rows = sqlx::query(
            "SELECT id, repo, scope_glob, text, required, check_command FROM conventions
             WHERE repo = $1 AND removed_at IS NULL ORDER BY id",
        )
        .bind(repo)
        .fetch_all(&self.pool)
        .await
        .map_err(serr)?;
        rows.iter()
            .map(|r| {
                Ok(Convention {
                    id: ConventionId(r.try_get("id").map_err(serr)?),
                    repo: r.try_get("repo").map_err(serr)?,
                    scope_glob: r.try_get("scope_glob").map_err(serr)?,
                    text: r.try_get("text").map_err(serr)?,
                    required: r.try_get("required").map_err(serr)?,
                    check_command: r.try_get("check_command").map_err(serr)?,
                })
            })
            .collect()
    }

    async fn set_convention_check(&self, id: ConventionId, command: Option<&str>) -> Result<()> {
        let n = sqlx::query("UPDATE conventions SET check_command = $2 WHERE id = $1")
            .bind(id.0)
            .bind(command.filter(|c| !c.trim().is_empty()))
            .execute(&self.pool)
            .await
            .map_err(serr)?
            .rows_affected();
        if n == 0 {
            return Err(AvError::NotFound(format!("convention {}", id.0)));
        }
        Ok(())
    }

    async fn remove_convention(&self, repo: &str, id: ConventionId) -> Result<()> {
        let n = sqlx::query(
            "UPDATE conventions SET removed_at = now()
             WHERE id = $1 AND repo = $2 AND removed_at IS NULL",
        )
        .bind(id.0)
        .bind(repo)
        .execute(&self.pool)
        .await
        .map_err(serr)?
        .rows_affected();
        if n == 0 {
            return Err(AvError::NotFound(format!(
                "convention {} in repo {repo}",
                id.0
            )));
        }
        Ok(())
    }

    async fn conventions_for(&self, repo: &str, paths: &[String]) -> Result<Vec<Convention>> {
        let mut out = vec![];
        for c in self.conventions(repo).await? {
            let matcher = Glob::new(&c.scope_glob)
                .map_err(|e| AvError::Store(e.to_string()))?
                .compile_matcher();
            if paths.iter().any(|p| matcher.is_match(p)) {
                out.push(c);
            }
        }
        Ok(out)
    }

    async fn ledger_get(&self, session: &str) -> Result<HashMap<String, Tier>> {
        let rows = sqlx::query("SELECT item_id, tier FROM ledger WHERE session = $1")
            .bind(session)
            .fetch_all(&self.pool)
            .await
            .map_err(serr)?;
        let mut out = HashMap::new();
        for r in rows {
            let tier: String = r.try_get("tier").map_err(serr)?;
            out.insert(r.try_get("item_id").map_err(serr)?, tier.parse()?);
        }
        Ok(out)
    }

    async fn add_receipt(&self, r: NewReceipt) -> Result<String> {
        let id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO context_receipts
               (id, task_id, session, head_change, packer_version, budget, tokens, items, omitted)
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)",
        )
        .bind(id)
        .bind(r.task_id.0)
        .bind(&r.session)
        .bind(&r.head_change.0)
        .bind(r.packer_version as i32)
        .bind(r.budget as i32)
        .bind(r.tokens as i32)
        .bind(serde_json::to_value(&r.items).map_err(|e| AvError::Store(e.to_string()))?)
        .bind(serde_json::to_value(&r.omitted).map_err(|e| AvError::Store(e.to_string()))?)
        .execute(&self.pool)
        .await
        .map_err(serr)?;
        Ok(format!("rcp:{}", &id.simple().to_string()[..8]))
    }

    async fn get_receipt(&self, id_or_prefix: &str) -> Result<serde_json::Value> {
        let p = id_or_prefix
            .trim()
            .trim_start_matches("rcp:")
            .to_lowercase();
        if p.len() < 6 || !p.chars().all(|c| c.is_ascii_hexdigit() || c == '-') {
            return Err(AvError::Invalid(format!(
                "receipt id prefix must be >= 6 hex chars: {id_or_prefix:?}"
            )));
        }
        let rows: Vec<(serde_json::Value,)> = sqlx::query_as(
            "SELECT json_build_object(
                'id', 'rcp:' || left(replace(id::text, '-', ''), 8),
                'task_id', task_id, 'session', session, 'head_change', head_change,
                'packer_version', packer_version, 'budget', budget, 'tokens', tokens,
                'items', items, 'omitted', omitted, 'created_at', created_at)::jsonb
             FROM context_receipts WHERE replace(id::text, '-', '') LIKE $1 LIMIT 2",
        )
        .bind(format!("{}%", p.replace('-', "")))
        .fetch_all(&self.pool)
        .await
        .map_err(serr)?;
        match rows.as_slice() {
            [] => Err(AvError::NotFound(format!("receipt {p}"))),
            [(v,)] => Ok(v.clone()),
            _ => Err(AvError::Invalid(format!("receipt prefix {p} is ambiguous"))),
        }
    }

    async fn charge_expand(
        &self,
        session: &str,
        tokens: u64,
        cap: Option<u64>,
    ) -> Result<Option<u64>> {
        // Racing charges on one session queue on the row lock and each re-checks the cap against
        // the committed total, so the total never passes it. No row back means refused.
        let total: Option<(i64,)> = sqlx::query_as(
            "INSERT INTO ledger_sessions (session, expand_tokens)
             SELECT $1, $2 WHERE $3::bigint IS NULL OR $2 <= $3
             ON CONFLICT (session) DO UPDATE
               SET expand_tokens = ledger_sessions.expand_tokens + EXCLUDED.expand_tokens,
                   last_seen = now()
               WHERE $3::bigint IS NULL
                  OR ledger_sessions.expand_tokens + EXCLUDED.expand_tokens <= $3
             RETURNING expand_tokens",
        )
        .bind(session)
        .bind(tokens as i64)
        .bind(cap.map(|c| c.min(i64::MAX as u64) as i64))
        .fetch_optional(&self.pool)
        .await
        .map_err(serr)?;
        Ok(total.map(|(t,)| t as u64))
    }

    async fn expand_tokens(&self, session: &str) -> Result<u64> {
        let row: Option<(i64,)> =
            sqlx::query_as("SELECT expand_tokens FROM ledger_sessions WHERE session = $1")
                .bind(session)
                .fetch_optional(&self.pool)
                .await
                .map_err(serr)?;
        Ok(row.map_or(0, |(t,)| t as u64))
    }

    async fn set_anchor_hashes(&self, record: RecordId, hashes: &[Option<String>]) -> Result<()> {
        let rec = self.get_record(record).await?;
        let mut tx = self.pool.begin().await.map_err(serr)?;
        for (a, h) in rec.anchors.iter().zip(hashes) {
            sqlx::query(
                "UPDATE anchors SET content_hash = $4
                 WHERE record_id = $1 AND path = $2 AND symbol IS NOT DISTINCT FROM $3",
            )
            .bind(record.0)
            .bind(&a.path)
            .bind(&a.symbol)
            .bind(h)
            .execute(&mut *tx)
            .await
            .map_err(serr)?;
        }
        tx.commit().await.map_err(serr)
    }

    async fn cite_receipt(&self, record: RecordId, receipt: &str) -> Result<()> {
        let full: String = self.get_receipt(receipt).await?["id"]
            .as_str()
            .unwrap_or_default()
            .to_owned();
        let done = sqlx::query(
            "UPDATE records SET receipt_id =
               (SELECT id FROM context_receipts WHERE replace(id::text, '-', '') LIKE $2 LIMIT 1)
             WHERE id = $1",
        )
        .bind(record.0)
        .bind(format!("{}%", full.trim_start_matches("rcp:")))
        .execute(&self.pool)
        .await
        .map_err(serr)?;
        if done.rows_affected() == 0 {
            return Err(AvError::NotFound(format!("record {}", record.0)));
        }
        Ok(())
    }

    async fn ledger_put(&self, session: &str, sent: &[(String, Tier)]) -> Result<()> {
        let mut tx = self.pool.begin().await.map_err(serr)?;
        for (item, tier) in sent {
            sqlx::query(
                "INSERT INTO ledger (session, item_id, tier) VALUES ($1,$2,$3)
                 ON CONFLICT (session, item_id) DO UPDATE SET last_seen = now(), tier = CASE
                   WHEN array_position(ARRAY['handle','summary','full'], EXCLUDED.tier)
                      > array_position(ARRAY['handle','summary','full'], ledger.tier)
                   THEN EXCLUDED.tier ELSE ledger.tier END",
            )
            .bind(session)
            .bind(item)
            .bind(tier.as_str())
            .execute(&mut *tx)
            .await
            .map_err(serr)?;
        }
        tx.commit().await.map_err(serr)
    }

    async fn search_records_fts(
        &self,
        repo: &str,
        query: &str,
        limit: usize,
    ) -> Result<Vec<Record>> {
        if query.trim().is_empty() {
            return Ok(vec![]);
        }
        let rows = sqlx::query(&format!(
            "SELECT {RECORD_COLS} FROM records r
             JOIN tasks t ON t.id = r.task_id
             WHERE t.repo = $1
               AND r.tsv @@ plainto_tsquery('english', $2)
               AND NOT EXISTS (SELECT 1 FROM records n WHERE n.supersedes = r.id)
             ORDER BY ts_rank(r.tsv, plainto_tsquery('english', $2)) DESC, r.id
             LIMIT $3"
        ))
        .bind(repo)
        .bind(query)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(serr)?;
        self.hydrate_records(rows).await
    }
}

/// Inserts `r` (anchors, outbox rows) inside `tx`. `receipt_hex` is a `LIKE` pattern for the cited
/// receipt's id, already resolved.
fn verdict_row(r: &PgRow) -> Result<VerdictRow> {
    let verdict: String = r.try_get("verdict").map_err(serr)?;
    let opt = |col: &str| -> Result<Option<ChangeId>> {
        Ok(r.try_get::<Option<String>, _>(col)
            .map_err(serr)?
            .map(ChangeId))
    };
    Ok(VerdictRow {
        reviewer: r.try_get("reviewer").map_err(serr)?,
        verdict: verdict.parse()?,
        head: ChangeId(r.try_get("head_change").map_err(serr)?),
        base: opt("base_change")?,
        carried_from: opt("carried_from")?,
    })
}

/// Advisory lock taken by every event insert: event ids then commit in the order they were
/// drawn, so a reader paging by `id > since` never skips one that committed late.
const EVENTS_LOCK: i64 = 0x6176_6576_656e_7473; // "avevents"

/// Appends one feed event inside `tx` (see `EVENTS_LOCK`).
async fn insert_event(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    repo: &str,
    kind: &str,
    task: Option<TaskId>,
    change: Option<&str>,
    payload: serde_json::Value,
) -> Result<()> {
    sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(EVENTS_LOCK)
        .execute(&mut **tx)
        .await
        .map_err(serr)?;
    sqlx::query(
        "INSERT INTO events (repo, kind, task_id, change, payload) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(repo)
    .bind(kind)
    .bind(task.map(|t| t.0))
    .bind(change.filter(|c| !c.is_empty()))
    .bind(payload)
    .execute(&mut **tx)
    .await
    .map_err(serr)?;
    Ok(())
}

/// The conditional status update of a lifecycle transition, and its feed event, inside `tx`.
/// `merged` reports the merge commit (stored by `set_merge_state` just before), any other event
/// the task's head.
async fn set_status_reporting(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    id: TaskId,
    from: TaskStatus,
    to: TaskStatus,
    event: scholion_core::lifecycle::Event,
) -> Result<()> {
    let row = sqlx::query(
        "UPDATE tasks SET status = $3 WHERE id = $1 AND status = $2
         RETURNING repo, head_change, merge_commit",
    )
    .bind(id.0)
    .bind(from.as_str())
    .bind(to.as_str())
    .fetch_optional(&mut **tx)
    .await
    .map_err(serr)?
    .ok_or_else(|| {
        AvError::IllegalState(format!(
            "task {id} changed status while {event:?} was being applied; reload and retry"
        ))
    })?;
    let repo: String = row.try_get("repo").map_err(serr)?;
    let change: Option<String> = if event == scholion_core::lifecycle::Event::MergeOk {
        row.try_get("merge_commit").map_err(serr)?
    } else {
        row.try_get("head_change").map_err(serr)?
    };
    insert_event(
        tx,
        &repo,
        event.feed_kind(),
        Some(id),
        change.as_deref(),
        serde_json::json!({"from": from.as_str(), "to": to.as_str()}),
    )
    .await
}

async fn insert_record(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    r: &NewRecord,
    anchor_hashes: &[Option<String>],
    receipt_hex: Option<String>,
) -> Result<RecordId> {
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO records (id, task_id, kind, summary, body, change_id, supersedes, author_agent,
                              receipt_id)
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8,
                 (SELECT c.id FROM context_receipts c
                  WHERE replace(c.id::text, '-', '') LIKE $9 LIMIT 1))",
    )
    .bind(id)
    .bind(r.task_id.0)
    .bind(r.kind.as_str())
    .bind(&r.summary)
    .bind(&r.body)
    .bind(r.change_id.as_ref().map(|c| c.0.clone()))
    .bind(r.supersedes.map(|s| s.0))
    .bind(&r.author_agent)
    .bind(receipt_hex)
    .execute(&mut **tx)
    .await
    .map_err(serr)?;
    for (i, a) in r.anchors.iter().enumerate() {
        sqlx::query(
            "INSERT INTO anchors (record_id, path, symbol, content_hash) VALUES ($1,$2,$3,$4)",
        )
        .bind(id)
        .bind(&a.path)
        .bind(&a.symbol)
        .bind(anchor_hashes.get(i).cloned().flatten())
        .execute(&mut **tx)
        .await
        .map_err(serr)?;
    }
    let repo: String = sqlx::query_scalar("SELECT repo FROM tasks WHERE id = $1")
        .bind(r.task_id.0)
        .fetch_one(&mut **tx)
        .await
        .map_err(serr)?;
    let text = if r.body.is_empty() {
        r.summary.clone()
    } else {
        format!("{}\n{}", r.summary, r.body)
    };
    let record_id = RecordId(id);
    PgStore::enqueue(
        &mut **tx,
        &NewOutbox {
            kind: SemanticKind::Record,
            op: OutboxOp::Upsert,
            entity_id: record_id.to_string(),
            repo: repo.clone(),
            task_id: Some(r.task_id),
            record_id: Some(record_id),
            symbol_id: None,
            convention_id: None,
            path: None,
            change_id: r.change_id.clone(),
            superseded: false,
            text: Some(text),
        },
    )
    .await?;
    if let Some(old) = r.supersedes {
        let (old_repo, old_task): (String, Uuid) = sqlx::query_as(
            "SELECT t.repo, r.task_id FROM records r
             JOIN tasks t ON t.id = r.task_id WHERE r.id = $1",
        )
        .bind(old.0)
        .fetch_one(&mut **tx)
        .await
        .map_err(serr)?;
        PgStore::enqueue(
            &mut **tx,
            &NewOutbox {
                kind: SemanticKind::Record,
                op: OutboxOp::Supersede,
                entity_id: old.to_string(),
                repo: old_repo,
                task_id: Some(TaskId(old_task)),
                record_id: Some(old),
                symbol_id: None,
                convention_id: None,
                path: None,
                change_id: None,
                superseded: true,
                text: None,
            },
        )
        .await?;
    }
    Ok(record_id)
}
