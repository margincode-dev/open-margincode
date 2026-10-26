//! `av runner`: Scion's "no merge queue, test after merge" on the host. It follows the event feed,
//! materializes every new main through the read API (no jj, no checkout), runs the repo's check
//! command there and posts the result; `av log` and `av health` show it. The command comes from
//! the daemon's database, so it only runs with `--trust`.

use std::path::PathBuf;

use serde_json::{Value, json};

use crate::client::{CliError, Client, Result};

pub struct Runner<'a> {
    pub client: &'a Client,
    pub repo: String,
    pub work_dir: PathBuf,
    pub name: String,
}

impl Runner<'_> {
    /// The repo's check command (`av repo settings --check`).
    fn command(&self) -> Result<String> {
        let s = self
            .client
            .get(&format!("/v1/repos/{}/settings", self.repo), &[])?;
        s["check_command"]
            .as_str()
            .filter(|c| !c.trim().is_empty())
            .map(str::to_owned)
            .ok_or_else(|| {
                CliError::Local(format!(
                    "repo {} has no check command; set one with `av repo settings {} --check <cmd>`",
                    self.repo, self.repo
                ))
            })
    }

    /// Writes the tree at `change` into a fresh directory.
    pub fn materialize(&self, change: &str) -> Result<PathBuf> {
        let dir = self.work_dir.join(&change[..change.len().min(12)]);
        if dir.exists() {
            std::fs::remove_dir_all(&dir).map_err(|e| CliError::Local(e.to_string()))?;
        }
        let tree = self
            .client
            .get(&format!("/v1/repos/{}/tree", self.repo), &[("at", change)])?;
        for f in tree["files"].as_array().into_iter().flatten() {
            let path = f.as_str().unwrap_or_default();
            if path.split('/').any(|seg| seg == ".." || seg.is_empty()) {
                return Err(CliError::Local(format!("refusing path {path:?}")));
            }
            let bytes = self.client.get_bytes(
                &format!("/v1/repos/{}/files/{path}", self.repo),
                &[("at", change), ("raw", "1")],
            )?;
            let target = dir.join(path);
            if let Some(parent) = target.parent() {
                std::fs::create_dir_all(parent).map_err(|e| CliError::Local(e.to_string()))?;
            }
            std::fs::write(&target, bytes).map_err(|e| CliError::Local(e.to_string()))?;
        }
        std::fs::create_dir_all(&dir).map_err(|e| CliError::Local(e.to_string()))?;
        Ok(dir)
    }

    /// Runs the check on `change`, posts the result and returns the exit code.
    pub fn check(&self, change: &str, command: &str) -> Result<i32> {
        let dir = self.materialize(change)?;
        let out = std::process::Command::new("sh")
            .args(["-c", command])
            .current_dir(&dir)
            .output()
            .map_err(|e| CliError::Local(format!("cannot run sh: {e}")))?;
        let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
        text.push_str(&String::from_utf8_lossy(&out.stderr));
        let lines: Vec<&str> = text.lines().collect();
        let tail = lines[lines.len().saturating_sub(30)..].join("\n");
        let code = out.status.code().unwrap_or(-1);
        self.client.post(
            &format!("/v1/repos/{}/checks", self.repo),
            &json!({"change": change, "command": command, "exit_code": code,
                    "output_tail": tail, "runner": self.name}),
        )?;
        let _ = std::fs::remove_dir_all(&dir);
        println!(
            "{} {} (`{command}`, exit {code})",
            &change[..change.len().min(12)],
            if code == 0 { "passed" } else { "FAILED" }
        );
        Ok(code)
    }

    /// Checks main now if it has no result yet; with `once`, stops there. Otherwise follows the
    /// event feed and checks every merge (and undo) as it lands.
    pub fn run(&self, once: bool) -> Result<()> {
        let command = self.command()?;
        let health = self
            .client
            .get(&format!("/v1/repos/{}/health", self.repo), &[])?;
        let main = health["main"].as_str().unwrap_or_default().to_owned();
        if health["checked"].as_str() != Some(main.as_str()) {
            self.check(&main, &command)?;
        }
        if once {
            return Ok(());
        }
        let mut since: Option<i64> = None;
        loop {
            let since_s = since.map(|n| n.to_string());
            let mut q = vec![("repo", self.repo.as_str()), ("wait", "25")];
            if let Some(n) = since_s.as_deref() {
                q.push(("since", n));
            }
            let feed = self.client.get("/v1/events", &q)?;
            for e in feed["events"].as_array().into_iter().flatten() {
                if matches!(e["kind"].as_str(), Some("merged" | "undo")) {
                    let main = self.main()?;
                    self.check(&main, &self.command().unwrap_or_else(|_| command.clone()))?;
                }
            }
            since = feed["next"].as_i64();
        }
    }

    fn main(&self) -> Result<String> {
        let h: Value = self
            .client
            .get(&format!("/v1/repos/{}/health", self.repo), &[])?;
        Ok(h["main"].as_str().unwrap_or_default().to_owned())
    }
}

/// The default scratch directory for materialized trees.
pub fn default_work_dir(repo: &str) -> PathBuf {
    std::env::temp_dir().join(format!("av-runner-{repo}"))
}

/// The index of the first bad commit in `n` commits, oldest first, given that the last one is
/// bad and the commits before the first bad one are good (main's history is linear: one squash
/// commit per task). `is_bad` is asked as few times as a binary search needs; `known` answers
/// first where it can (recorded check results).
pub fn first_bad(
    n: usize,
    known: impl Fn(usize) -> Option<bool>,
    mut is_bad: impl FnMut(usize) -> Result<bool>,
) -> Result<usize> {
    // invariant: everything below `lo` is good, `hi` is bad
    let (mut lo, mut hi) = (0, n - 1);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let bad = match known(mid) {
            Some(b) => b,
            None => is_bad(mid)?,
        };
        if bad {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    Ok(lo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_first_bad_commit_with_few_checks() {
        for first in 0..9 {
            let mut asked = 0;
            let got = first_bad(
                9,
                |_| None,
                |i| {
                    asked += 1;
                    Ok(i >= first)
                },
            )
            .unwrap();
            assert_eq!(got, first);
            assert!(asked <= 4, "binary search: {asked} checks for 9 commits");
        }
    }

    #[test]
    fn recorded_results_are_used_before_running_anything() {
        let known = |i: usize| (i == 4).then_some(true);
        let mut ran = vec![];
        let got = first_bad(9, known, |i| {
            ran.push(i);
            Ok(i >= 3)
        })
        .unwrap();
        assert_eq!(got, 3);
        assert!(!ran.contains(&4), "commit 4 had a recorded result: {ran:?}");
    }
}
