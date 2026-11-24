//! The constitution's rows: articles are `conventions` rows with `article` set (`repo = ''` for
//! the shared level), and every change is a `constitution_amendments` row. Who may amend is the
//! daemon's rule; here are the limits, the numbering and the one lock.

use margincode_core::*;
use sqlx::Row;
use sqlx::postgres::PgRow;

use crate::pg::{PgStore, serr};

/// Every amendment, at either level, takes this lock: the cap of ten counts both levels.
const LOCK: &str = "SELECT pg_advisory_xact_lock(hashtext('constitution'))";

const AMENDMENT_COLS: &str = "a.repo, a.version, a.at, a.amended_by, a.action, a.article, \
                              a.convention, a.reason";

fn key<'a>(level: ConstitutionLevel<'a>) -> Result<&'a str> {
    match level {
        ConstitutionLevel::Shared => Ok(""),
        ConstitutionLevel::Repo(r) if r.trim().is_empty() => {
            Err(AvError::Invalid("repo must not be blank".into()))
        }
        ConstitutionLevel::Repo(r) => Ok(r),
    }
}

/// The keys a read covers: the shared level, and the repository's when there is one.
fn keys(level: ConstitutionLevel<'_>) -> Result<Vec<String>> {
    let k = key(level)?;
    let mut out = vec![String::new()];
    if !k.is_empty() {
        out.push(k.to_owned());
    }
    Ok(out)
}

fn amendment_from_row(r: &PgRow) -> Result<Amendment> {
    let repo: String = r.try_get("repo").map_err(serr)?;
    let article: i32 = r.try_get("article").map_err(serr)?;
    let action: String = r.try_get("action").map_err(serr)?;
    Ok(Amendment {
        handle: article_handle(repo.is_empty(), article),
        repo,
        version: r.try_get("version").map_err(serr)?,
        at: r.try_get("at").map_err(serr)?,
        by: r.try_get("amended_by").map_err(serr)?,
        action: action.parse()?,
        article,
        convention_id: r
            .try_get::<Option<i64>, _>("convention")
            .map_err(serr)?
            .map(ConventionId),
        reason: r.try_get("reason").map_err(serr)?,
    })
}

fn check_of(check: Option<&str>) -> Option<String> {
    check
        .map(str::trim)
        .filter(|c| !c.is_empty())
        .map(str::to_owned)
}

impl PgStore {
    /// Whether `name` (any case) is a registered agent profile's name or alias, or the agent of
    /// a task of `repo`.
    pub async fn names_an_agent(&self, repo: &str, name: &str) -> Result<bool> {
        sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM agent_profiles p
                            WHERE lower(p.name) = lower($2)
                               OR lower($2) IN (SELECT lower(x) FROM unnest(p.aliases) x))
                 OR EXISTS (SELECT 1 FROM tasks WHERE repo = $1 AND lower(owner_agent) = lower($2))",
        )
        .bind(repo)
        .bind(name.trim())
        .fetch_one(self.pool())
        .await
        .map_err(serr)
    }

    pub(crate) async fn read_constitution(
        &self,
        level: ConstitutionLevel<'_>,
    ) -> Result<Constitution> {
        let repo = key(level)?;
        let keys = keys(level)?;
        let versions: Vec<(String, i32)> = sqlx::query_as(
            "SELECT repo, max(version) FROM constitution_amendments WHERE repo = ANY($1)
             GROUP BY repo",
        )
        .bind(&keys)
        .fetch_all(self.pool())
        .await
        .map_err(serr)?;
        let version_of = |k: &str| versions.iter().find(|(r, _)| r == k).map_or(0, |(_, v)| *v);
        let rows = sqlx::query(&format!(
            "SELECT c.id, c.title, c.text, c.check_command, {AMENDMENT_COLS}
             FROM conventions c
             JOIN LATERAL (SELECT * FROM constitution_amendments x
                           WHERE x.repo = c.repo AND x.article = c.article
                           ORDER BY x.version DESC LIMIT 1) a ON true
             WHERE c.repo = ANY($1) AND c.article IS NOT NULL AND c.removed_at IS NULL
             ORDER BY c.repo <> '', c.article"
        ))
        .bind(&keys)
        .fetch_all(self.pool())
        .await
        .map_err(serr)?;
        let mut articles = Vec::with_capacity(rows.len());
        for r in &rows {
            let last = amendment_from_row(r)?;
            articles.push(Article {
                handle: last.handle.clone(),
                number: last.article,
                shared: last.repo.is_empty(),
                title: r
                    .try_get::<Option<String>, _>("title")
                    .map_err(serr)?
                    .unwrap_or_default(),
                text: r.try_get("text").map_err(serr)?,
                check: r.try_get("check_command").map_err(serr)?,
                convention_id: ConventionId(r.try_get("id").map_err(serr)?),
                last,
            });
        }
        Ok(Constitution {
            repo: repo.to_owned(),
            version: version_of(repo),
            shared_version: version_of(""),
            articles,
        })
    }

    pub(crate) async fn read_amendments(
        &self,
        level: ConstitutionLevel<'_>,
    ) -> Result<Vec<Amendment>> {
        let rows = sqlx::query(&format!(
            "SELECT {AMENDMENT_COLS} FROM constitution_amendments a WHERE a.repo = ANY($1)
             ORDER BY a.at, a.repo <> '', a.version"
        ))
        .bind(keys(level)?)
        .fetch_all(self.pool())
        .await
        .map_err(serr)?;
        rows.iter().map(amendment_from_row).collect()
    }

    pub(crate) async fn write_amendment(
        &self,
        level: ConstitutionLevel<'_>,
        change: &ArticleChange,
        by: &str,
        reason: &str,
    ) -> Result<Amendment> {
        let repo = key(level)?;
        let by = by.trim();
        if by.is_empty() {
            return Err(AvError::Invalid("by must not be blank".into()));
        }
        let reason = within_limit("the reason", reason, MAX_AMENDMENT_REASON)?;
        let new_text = |title: &str, text: &str| -> Result<(String, String)> {
            Ok((
                within_limit("the title", title, MAX_ARTICLE_TITLE)?,
                within_limit("the text", text, MAX_ARTICLE_TEXT)?,
            ))
        };
        let mut tx = self.pool().begin().await.map_err(serr)?;
        sqlx::query(LOCK).execute(&mut *tx).await.map_err(serr)?;
        let (version, last_number): (i32, i32) = sqlx::query_as(
            "SELECT COALESCE(max(version), 0) + 1, COALESCE(max(article), 0)
             FROM constitution_amendments WHERE repo = $1",
        )
        .bind(repo)
        .fetch_one(&mut *tx)
        .await
        .map_err(serr)?;
        let in_force = |n: i32| {
            AvError::NotFound(format!(
                "{} is not in force",
                article_handle(repo.is_empty(), n)
            ))
        };
        let (action, number, convention) = match change {
            ArticleChange::Add { title, text, check } => {
                let (title, text) = new_text(title, text)?;
                check_cap(&mut tx, repo).await?;
                let number = last_number + 1;
                let id = insert_article(
                    &mut tx,
                    repo,
                    number,
                    &title,
                    &text,
                    check_of(check.as_deref()),
                )
                .await?;
                (AmendAction::Add, number, Some(id))
            }
            ArticleChange::Amend {
                number,
                title,
                text,
                check,
            } => {
                let old = retire(&mut tx, repo, *number)
                    .await?
                    .ok_or_else(|| in_force(*number))?;
                let (new_title, new_body) = new_text(
                    title.as_deref().unwrap_or(&old.title),
                    text.as_deref().unwrap_or(&old.text),
                )?;
                let new_check = match check {
                    Some(c) => check_of(Some(c)),
                    None => old.check.clone(),
                };
                if new_title == old.title && new_body == old.text && new_check == old.check {
                    return Err(AvError::IllegalState(format!(
                        "this amendment changes nothing in {}",
                        article_handle(repo.is_empty(), *number)
                    )));
                }
                let id = insert_article(&mut tx, repo, *number, &new_title, &new_body, new_check)
                    .await?;
                (AmendAction::Amend, *number, Some(id))
            }
            ArticleChange::Repeal { number } => {
                retire(&mut tx, repo, *number)
                    .await?
                    .ok_or_else(|| in_force(*number))?;
                (AmendAction::Repeal, *number, None)
            }
        };
        let row = sqlx::query(&format!(
            "INSERT INTO constitution_amendments AS a
                 (repo, version, amended_by, action, article, convention, reason)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING {AMENDMENT_COLS}"
        ))
        .bind(repo)
        .bind(version)
        .bind(by)
        .bind(action.as_str())
        .bind(number)
        .bind(convention.map(|c| c.0))
        .bind(&reason)
        .fetch_one(&mut *tx)
        .await
        .map_err(serr)?;
        let amendment = amendment_from_row(&row)?;
        tx.commit().await.map_err(serr)?;
        Ok(amendment)
    }
}

struct InForce {
    title: String,
    text: String,
    check: Option<String>,
}

/// Retires article `number` of `repo` (`removed_at`) and returns what it said; `None` when it is
/// not in force.
async fn retire(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    repo: &str,
    number: i32,
) -> Result<Option<InForce>> {
    let row = sqlx::query(
        "UPDATE conventions SET removed_at = now()
         WHERE repo = $1 AND article = $2 AND removed_at IS NULL
         RETURNING title, text, check_command",
    )
    .bind(repo)
    .bind(number)
    .fetch_optional(&mut **tx)
    .await
    .map_err(serr)?;
    row.map(|r| {
        Ok(InForce {
            title: r
                .try_get::<Option<String>, _>("title")
                .map_err(serr)?
                .unwrap_or_default(),
            text: r.try_get("text").map_err(serr)?,
            check: r.try_get("check_command").map_err(serr)?,
        })
    })
    .transpose()
}

/// Not put on the semantic index: a pack carries every article whatever the query.
async fn insert_article(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    repo: &str,
    number: i32,
    title: &str,
    text: &str,
    check: Option<String>,
) -> Result<ConventionId> {
    let (id,): (i64,) = sqlx::query_as(
        "INSERT INTO conventions (repo, scope_glob, text, required, check_command, article, title)
         VALUES ($1, '**', $2, true, $3, $4, $5) RETURNING id",
    )
    .bind(repo)
    .bind(text)
    .bind(check)
    .bind(number)
    .bind(title)
    .fetch_one(&mut **tx)
    .await
    .map_err(serr)?;
    Ok(ConventionId(id))
}

/// Refuses an add that would put more than `MAX_ARTICLES` on a repository, the shared ones
/// counted on every repository.
async fn check_cap(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, repo: &str) -> Result<()> {
    let count = |r: &'static str| {
        format!(
            "SELECT count(*) FROM conventions
             WHERE {r} AND article IS NOT NULL AND removed_at IS NULL"
        )
    };
    let shared: i64 = sqlx::query_scalar(&count("repo = ''"))
        .fetch_one(&mut **tx)
        .await
        .map_err(serr)?;
    let max = MAX_ARTICLES as i64;
    if !repo.is_empty() {
        let own: i64 = sqlx::query_scalar(&count("repo = $1"))
            .bind(repo)
            .fetch_one(&mut **tx)
            .await
            .map_err(serr)?;
        if shared + own >= max {
            return Err(AvError::IllegalState(format!(
                "{repo} has {} articles in force ({shared} from the organisation, {own} its own); \
                 the limit is {max}: repeal one first",
                shared + own
            )));
        }
        return Ok(());
    }
    if shared >= max {
        return Err(AvError::IllegalState(format!(
            "the organisation has {shared} articles in force; the limit is {max}: repeal one first"
        )));
    }
    let full: Vec<(String, i64)> = sqlx::query_as(
        "SELECT repo, count(*) FROM conventions
         WHERE repo <> '' AND article IS NOT NULL AND removed_at IS NULL
         GROUP BY repo HAVING count(*) + $1 >= $2 ORDER BY repo",
    )
    .bind(shared)
    .bind(max)
    .fetch_all(&mut **tx)
    .await
    .map_err(serr)?;
    if !full.is_empty() {
        let names: Vec<String> = full
            .iter()
            .map(|(r, own)| format!("{r} ({own} of its own)"))
            .collect();
        return Err(AvError::IllegalState(format!(
            "an organisation article would put more than {max} articles on {}: with {shared} from \
             the organisation, each would pass the limit; repeal one there first",
            names.join(", ")
        )));
    }
    Ok(())
}
