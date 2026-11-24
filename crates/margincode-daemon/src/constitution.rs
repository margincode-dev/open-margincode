//! A repository's constitution over HTTP: read it (the shared articles too), and amend its own
//! articles. Community has one token and no identities, so who amends is a rule about names:
//! a `by` that names an agent is refused, and the guard is asked before every write. The shared
//! level is written only through `AppState::amend_shared_constitution`.

use axum::extract::{Path, Query, State};
use axum::routing::{get, post, put};
use axum::{Json, Router};
use globset::Glob;
use margincode_core::*;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::error::ApiResult;
use crate::ext::{RuleChange, RuleKind};
use crate::{AppState, Guarded, MemoryAction};

const BY_AN_AGENT: &str =
    "the constitution is amended by a person; an agent proposes a rule with a decision record";

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/repos/{name}/constitution", get(show))
        .route("/v1/repos/{name}/constitution/articles", post(add_article))
        .route(
            "/v1/repos/{name}/constitution/articles/{n}",
            put(amend_article).delete(repeal_article),
        )
}

#[derive(Deserialize)]
struct ShowQuery {
    #[serde(default)]
    history: Option<u8>,
}

/// The articles in force (shared ones first, read-only here), both versions, the free notes and,
/// with `?history=1`, every amendment of both levels, oldest first.
async fn show(
    State(st): State<AppState>,
    Path(repo): Path<String>,
    Query(q): Query<ShowQuery>,
) -> ApiResult<Value> {
    known_repo(&st, &repo).await?;
    let c = st
        .store
        .constitution(ConstitutionLevel::Repo(&repo))
        .await?;
    let mut out = serde_json::to_value(&c).map_err(|e| AvError::Store(e.to_string()))?;
    out["notes"] = json!(rule_notes(&st, &repo).await?);
    if q.history == Some(1) {
        out["history"] = json!(st.store.amendments(ConstitutionLevel::Repo(&repo)).await?);
    }
    Ok(Json(out))
}

#[derive(Deserialize)]
struct WriteBody {
    by: String,
    reason: String,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    text: Option<String>,
    /// `PUT`: absent keeps the check, `""` removes it.
    #[serde(default)]
    check: Option<String>,
}

async fn add_article(
    State(st): State<AppState>,
    Path(repo): Path<String>,
    Json(b): Json<WriteBody>,
) -> ApiResult<Value> {
    let change = ArticleChange::Add {
        title: b.title.clone().unwrap_or_default(),
        text: b.text.clone().unwrap_or_default(),
        check: b.check.clone(),
    };
    write(&st, &repo, change, &b).await
}

async fn amend_article(
    State(st): State<AppState>,
    Path((repo, n)): Path<(String, i32)>,
    Json(b): Json<WriteBody>,
) -> ApiResult<Value> {
    let change = ArticleChange::Amend {
        number: n,
        title: b.title.clone(),
        text: b.text.clone(),
        check: b.check.clone(),
    };
    write(&st, &repo, change, &b).await
}

async fn repeal_article(
    State(st): State<AppState>,
    Path((repo, n)): Path<(String, i32)>,
    Json(b): Json<WriteBody>,
) -> ApiResult<Value> {
    write(&st, &repo, ArticleChange::Repeal { number: n }, &b).await
}

async fn known_repo(st: &AppState, repo: &str) -> Result<()> {
    if st.store.list_repos().await?.iter().any(|r| r.name == repo) {
        Ok(())
    } else {
        Err(AvError::NotFound(format!("repo {repo}")))
    }
}

fn check_of(check: Option<&str>) -> Option<&str> {
    check.map(str::trim).filter(|c| !c.is_empty())
}

/// The daemon's checks, then the guard, then the store (which checks the limits again under its
/// lock, against a concurrent amendment).
async fn write(
    st: &AppState,
    repo: &str,
    change: ArticleChange,
    b: &WriteBody,
) -> ApiResult<Value> {
    known_repo(st, repo).await?;
    let by = b.by.trim();
    if by.is_empty() {
        return Err(AvError::Invalid("by must not be blank".into()).into());
    }
    within_limit("the reason", &b.reason, MAX_AMENDMENT_REASON)?;
    if st.pg.names_an_agent(repo, by).await? {
        return Err(AvError::IllegalState(BY_AN_AGENT.into()).into());
    }
    let level = ConstitutionLevel::Repo(repo);
    let now = st.store.constitution(level).await?;
    let own = |n: i32| {
        now.articles
            .iter()
            .find(|a| !a.shared && a.number == n)
            .ok_or_else(|| {
                AvError::NotFound(format!("{} is not in force", article_handle(false, n)))
            })
    };
    let next = st
        .store
        .amendments(level)
        .await?
        .iter()
        .filter(|a| !a.repo.is_empty())
        .map(|a| a.article)
        .max()
        .unwrap_or(0)
        + 1;
    let (title, text, current, number);
    let rule = match &change {
        ArticleChange::Add {
            title: t,
            text: x,
            check,
        } => {
            title = within_limit("the title", t, MAX_ARTICLE_TITLE)?;
            text = within_limit("the text", x, MAX_ARTICLE_TEXT)?;
            number = next;
            RuleChange::Add {
                number: Some(number),
                title: Some(&title),
                text: &text,
                scope_glob: "**",
                check: check_of(check.as_deref()),
            }
        }
        ArticleChange::Amend {
            number: n,
            title: t,
            text: x,
            check,
        } => {
            let old = own(*n)?;
            title = within_limit(
                "the title",
                t.as_deref().unwrap_or(&old.title),
                MAX_ARTICLE_TITLE,
            )?;
            text = within_limit(
                "the text",
                x.as_deref().unwrap_or(&old.text),
                MAX_ARTICLE_TEXT,
            )?;
            number = *n;
            RuleChange::Amend {
                number,
                title: &title,
                text: &text,
                check: match check {
                    Some(c) => check_of(Some(c)),
                    None => old.check.as_deref(),
                },
            }
        }
        ArticleChange::Repeal { number: n } => {
            let old = own(*n)?;
            current = Convention {
                id: old.convention_id,
                repo: repo.to_owned(),
                scope_glob: "**".into(),
                text: old.text.clone(),
                required: true,
                check_command: old.check.clone(),
                source: String::new(),
                article: Some(old.number),
                title: Some(old.title.clone()),
            };
            number = *n;
            RuleChange::Remove {
                number: Some(number),
                convention: &current,
            }
        }
    };
    let action = Guarded::MemoryWrite {
        repo,
        action: MemoryAction::Rule {
            kind: RuleKind::Article,
            change: rule,
        },
        by,
        dream: None,
    };
    if let Err(reason) = st.guard.check(action).await {
        // an add's number is only the next free one: the article does not exist yet
        let what = match &change {
            ArticleChange::Add { .. } => "add an article".to_owned(),
            ArticleChange::Amend { .. } => format!("amend {}", article_handle(false, number)),
            ArticleChange::Repeal { .. } => format!("repeal {}", article_handle(false, number)),
        };
        return Err(AvError::IllegalState(format!("cannot {what}: {reason}")).into());
    }
    let amendment = st
        .store
        .amend_constitution(level, &change, by, &b.reason)
        .await?;
    Ok(Json(json!({
        "amendment": amendment,
        "constitution": st.store.constitution(level).await?,
    })))
}

/// Asks the guard about a required convention's add or removal; ordinary ones are not asked.
pub(crate) async fn guard_required(
    st: &AppState,
    repo: &str,
    change: RuleChange<'_>,
    by: &str,
) -> Result<()> {
    let action = Guarded::MemoryWrite {
        repo,
        action: MemoryAction::Rule {
            kind: RuleKind::RequiredConvention,
            change,
        },
        by,
        dream: None,
    };
    st.guard.check(action).await.map_err(|reason| {
        let what = match change {
            RuleChange::Remove { convention, .. } => {
                format!("remove the required convention {}", convention.handle())
            }
            _ => "add a required convention".to_owned(),
        };
        AvError::IllegalState(format!("cannot {what}: {reason}"))
    })
}

fn normalized(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// Notes, never errors, on a repository's rules (articles and conventions): a convention whose
/// glob matches no file on `main`, two rules with the same text (whitespace and case aside), and
/// an article and a convention that run the same check.
pub(crate) async fn rule_notes(st: &AppState, repo: &str) -> Result<Vec<String>> {
    let articles = st
        .store
        .constitution(ConstitutionLevel::Repo(repo))
        .await?
        .articles;
    let conventions = st.store.conventions(repo).await?;
    let mut notes = vec![];
    let files = match st.vcs.trunk(repo).await {
        Ok(main) => st.vcs.list_files(repo, &main).await.unwrap_or_default(),
        Err(_) => vec![],
    };
    if !files.is_empty() {
        for c in &conventions {
            let Ok(glob) = Glob::new(&c.scope_glob) else {
                continue;
            };
            let m = glob.compile_matcher();
            if !files.iter().any(|f| m.is_match(f)) {
                notes.push(format!(
                    "{} ({}) matches no file on main",
                    c.handle(),
                    c.scope_glob
                ));
            }
        }
    }
    let rules: Vec<(String, &str, Option<&str>, bool)> = articles
        .iter()
        .map(|a| (a.handle.clone(), a.text.as_str(), a.check.as_deref(), true))
        .chain(conventions.iter().map(|c| {
            (
                c.handle(),
                c.text.as_str(),
                c.check_command.as_deref(),
                false,
            )
        }))
        .collect();
    for (i, (a, a_text, a_check, a_article)) in rules.iter().enumerate() {
        for (b, b_text, b_check, b_article) in &rules[i + 1..] {
            if normalized(a_text) == normalized(b_text) {
                notes.push(format!("{a} and {b} have the same text"));
            }
            if a_article != b_article
                && let (Some(x), Some(y)) = (a_check, b_check)
                && x.trim() == y.trim()
            {
                notes.push(format!("{a} and {b} run the same check `{}`", x.trim()));
            }
        }
    }
    Ok(notes)
}
