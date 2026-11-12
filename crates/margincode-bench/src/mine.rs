//! Mines retrieval tasks (Plan 6, E1) from a repo's history: small, single-parent commits that
//! modify one to a few non-test source files. The commit message is the goal, with trailers and
//! links removed. Messages often name the code they change; `retrieval` flags those tasks
//! (`leak`) and reports them separately.

use std::path::Path;
use std::process::Command;

use margincode_core::history;

use crate::Result;
use crate::retrieval::{RetrievalTask, indexable, is_test_path};

pub struct MineArgs {
    /// What to write as each task's `repo_path` (e.g. `~/eval-repos/click`).
    pub repo_path: String,
    pub prefix: String,
    /// `git log --since` (e.g. `2026-06-01`).
    pub since: Option<String>,
    pub max_files: usize,
    pub max_lines: usize,
    pub limit: usize,
}

const MAX_GOAL: usize = 1500;

fn git(dir: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .map_err(|e| format!("git {args:?}: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "git {args:?}: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// The commit message as a goal: trailers, sign-offs and URLs dropped, blank runs collapsed.
pub fn goal_from_message(subject: &str, body: &str) -> String {
    history::clean_message(subject, body, MAX_GOAL)
}

pub use margincode_core::history::housekeeping;

pub fn mine(repo: &Path, args: &MineArgs) -> Result<Vec<RetrievalTask>> {
    let mut log_args = vec!["log", "--no-merges", "--format=%H%x00%P%x00%s%x00%b%x1e"];
    let since;
    if let Some(s) = &args.since {
        since = format!("--since={s}");
        log_args.push(&since);
    }
    let log = git(repo, &log_args)?;
    let mut out = vec![];
    for rec in log.split('\x1e') {
        if out.len() >= args.limit {
            break;
        }
        let f: Vec<&str> = rec.trim_start_matches('\n').splitn(4, '\0').collect();
        let [sha, parents, subject, body] = f[..] else {
            continue;
        };
        // the same rules the import's cold start uses (`margincode_core::history`)
        let Some(goal) =
            history::worth_mining(parents.split_whitespace().count(), subject, body, MAX_GOAL)
        else {
            continue;
        };
        let numstat = git(
            repo,
            &["diff", "--numstat", "--no-renames", &format!("{sha}^"), sha],
        )?;
        let status = git(
            repo,
            &[
                "diff",
                "--name-status",
                "--no-renames",
                &format!("{sha}^"),
                sha,
            ],
        )?;
        let modified: Vec<&str> = status
            .lines()
            .filter_map(|l| l.split_once('\t'))
            .filter(|(st, p)| st.starts_with('M') && indexable(p) && !is_test_path(p))
            .map(|(_, p)| p)
            .collect();
        if modified.is_empty() || modified.len() > args.max_files {
            continue;
        }
        let lines: usize = numstat
            .lines()
            .filter_map(|l| {
                let mut it = l.split('\t');
                let (a, d, p) = (it.next()?, it.next()?, it.next()?);
                (indexable(p) && !is_test_path(p))
                    .then(|| a.parse::<usize>().unwrap_or(0) + d.parse::<usize>().unwrap_or(0))
            })
            .sum();
        if lines > args.max_lines {
            continue;
        }
        out.push(RetrievalTask {
            name: format!("{}-{}", args.prefix, &sha[..7.min(sha.len())]),
            repo_path: args.repo_path.clone().into(),
            base_rev: format!("{sha}^"),
            fix_rev: sha.to_owned(),
            goal,
        });
    }
    Ok(out)
}
