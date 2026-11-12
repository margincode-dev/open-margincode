//! `av notes`: a task's records as a Markdown block for its pull request (Plan 13 Phase 2, the PR
//! bot). The daemon runs on the developer's machine, out of a CI job's reach, so the CLI renders
//! the block from `/v1` reads and posts it with `gh` from where the daemon is reachable.
//!
//! The block sits between `<!-- margincode:notes v1 task=<id> -->` and `<!-- /margincode:notes -->`,
//! one per task, so posting again replaces it in place (`replace_block`) and an unchanged block
//! is not sent at all. Rendering is pure (`render`), tested against `tests/golden/notes.md`.

use std::io::Write as _;
use std::process::{Command, Stdio};

use serde_json::{Value, json};

use crate::client::{CliError, Result};

pub const END: &str = "<!-- /margincode:notes -->";
/// Changed files whose earlier records the block looks up (`records/at`, one request each).
pub const MAX_FILES: usize = 20;
/// Body text shown per record; the rest is a `av expand` away.
const MAX_BODY_CHARS: usize = 600;

/// The opening marker of the block for one task.
pub fn start_marker(task_id: &str) -> String {
    format!("<!-- margincode:notes v1 task={task_id} -->")
}

/// What other tasks recorded about one changed file: the `records/at` answer for it.
pub struct FileNotes {
    pub path: String,
    /// The answer's record lines (`- rec:… kind: summary`, with `  anchors:` lines).
    pub lines: Vec<String>,
}

impl FileNotes {
    /// From `GET /v1/tasks/{id}/records/at` (`markdown`: a heading, the record lines, a footer).
    pub fn from_records_at(path: &str, answer: &Value) -> Option<FileNotes> {
        let md = answer["markdown"].as_str().unwrap_or_default();
        let lines: Vec<String> = md
            .lines()
            .filter(|l| l.starts_with("- ") || l.starts_with("  "))
            .map(str::to_owned)
            .collect();
        (!lines.is_empty()).then(|| FileNotes {
            path: path.to_owned(),
            lines,
        })
    }
}

/// Makes record text safe inside the block: it cannot close the block's comment markers, and a
/// `@name` in it does not notify anyone on GitHub.
pub fn sanitize(text: &str) -> String {
    let text = text.replace("<!--", "&lt;!--").replace("-->", "--&gt;");
    let mut out = String::with_capacity(text.len());
    let mut prev: Option<char> = None;
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        out.push(c);
        let after_word = prev.is_some_and(|p| p.is_alphanumeric() || p == '.' || p == '_');
        if c == '@' && !after_word && chars.peek().is_some_and(|n| n.is_ascii_alphanumeric()) {
            out.push('\u{2060}'); // word joiner: reads the same, links no one
        }
        prev = Some(c);
    }
    out
}

fn one_line(text: &str) -> String {
    sanitize(&text.split_whitespace().collect::<Vec<_>>().join(" "))
}

fn short_id(id: &str) -> String {
    format!(
        "rec:{}",
        id.replace('-', "").chars().take(8).collect::<String>()
    )
}

fn s(v: &Value) -> &str {
    v.as_str().unwrap_or_default()
}

fn anchors(rec: &Value) -> String {
    let list: Vec<String> = rec["anchors"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|a| match a["symbol"].as_str().filter(|x| !x.is_empty()) {
            Some(sym) => format!("`{}:{sym}`", s(&a["path"])),
            None => format!("`{}`", s(&a["path"])),
        })
        .collect();
    if list.is_empty() {
        String::new()
    } else {
        format!(" · {}", list.join(", "))
    }
}

/// A record's body as quoted lines under its bullet, clipped to `MAX_BODY_CHARS`.
fn body_lines(body: &str) -> Vec<String> {
    let body = body.trim();
    if body.is_empty() {
        return vec![];
    }
    let mut text: String = body.chars().take(MAX_BODY_CHARS).collect();
    if text.len() < body.len() {
        text.push_str(" …");
    }
    sanitize(&text)
        .lines()
        .map(|l| format!("  > {}", l.trim_end()).trim_end().to_owned())
        .collect()
}

/// The handoff's state (a `HandoffState` JSON body) as one line.
fn handoff_line(body: &str) -> String {
    let Ok(state) = serde_json::from_str::<Value>(body) else {
        return one_line(body);
    };
    ["done", "remaining", "broken", "questions"]
        .iter()
        .filter_map(|k| {
            let v = state[*k].as_str().unwrap_or_default().trim();
            (!v.is_empty()).then(|| format!("{k}: {}", one_line(v)))
        })
        .collect::<Vec<_>>()
        .join("; ")
}

const SECTIONS: [(&str, &str); 6] = [
    ("decision", "Decisions"),
    ("attempt", "Dead ends"),
    ("note", "Notes"),
    ("review", "Review findings"),
    ("handoff", "Handoffs"),
    ("validation", "Checks"),
];

/// The block for one task: `task` is the `GET /v1/tasks/{id}` answer, `records` the task's
/// records (`GET /v1/tasks/{id}/records`), `files` what other tasks recorded about the files it
/// changes. Superseded records are left out. Ends with a newline.
pub fn render(task: &Value, records: &[Value], files: &[FileNotes]) -> String {
    let t = &task["task"];
    let id = s(&t["id"]);
    let live: Vec<&Value> = records
        .iter()
        .filter(|r| r["superseded_by"].is_null())
        .collect();
    let mut out = vec![
        start_marker(id),
        "### Margincode: why this change".to_owned(),
        String::new(),
    ];
    let mut status = s(&t["status"]).replace('_', " ");
    if let Some(v) = task["verdict"].as_object() {
        let verdict = match v.get("verdict").and_then(Value::as_str) {
            Some("approve") => "approved",
            Some("request_changes") => "changes requested",
            _ => "reviewed",
        };
        status.push_str(&format!(
            "; {verdict} by {}",
            one_line(v.get("reviewer").and_then(Value::as_str).unwrap_or("?"))
        ));
    }
    if let Some(by) = task["merge"]["merged_by"].as_str() {
        status.push_str(&format!("; merged by {}", one_line(by)));
    }
    out.push(format!(
        "**{}** (`{}/{}`, {status}, owner {})",
        one_line(s(&t["title"])),
        s(&t["repo"]),
        s(&t["slug"]),
        one_line(s(&t["owner_agent"]))
    ));
    let goal = s(&t["goal"]).trim();
    if !goal.is_empty() && goal != s(&t["title"]).trim() {
        out.push(String::new());
        for l in sanitize(goal).lines() {
            out.push(format!("> {l}").trim_end().to_owned());
        }
    }
    let mut ids = vec![];
    let mut any = false;
    for (kind, heading) in SECTIONS {
        let recs: Vec<&&Value> = live.iter().filter(|r| s(&r["kind"]) == kind).collect();
        if recs.is_empty() {
            continue;
        }
        any |= kind != "validation";
        out.push(String::new());
        out.push(format!("**{heading}**"));
        for r in recs {
            let rid = short_id(s(&r["id"]));
            let text = if kind == "handoff" {
                let state = handoff_line(s(&r["body"]));
                if state.is_empty() {
                    one_line(s(&r["summary"]))
                } else {
                    state
                }
            } else {
                one_line(s(&r["summary"]))
            };
            out.push(format!(
                "- {text} (`{rid}`, {}){}",
                one_line(s(&r["author_agent"])),
                anchors(r)
            ));
            if !matches!(kind, "handoff" | "validation") {
                out.extend(body_lines(s(&r["body"])));
            }
            ids.push(rid);
        }
    }
    if !any {
        out.push(String::new());
        out.push("_No decisions, dead ends or notes were recorded for this task._".to_owned());
    }
    let changed = task["changed"].as_array().map_or(0, Vec::len);
    if !files.is_empty() {
        let n: usize = files
            .iter()
            .map(|f| f.lines.iter().filter(|l| l.starts_with("- ")).count())
            .sum();
        out.push(String::new());
        out.push(format!(
            "<details><summary>Recorded earlier about the changed files ({} of {changed}, {n} {})</summary>",
            files.len(),
            if n == 1 { "record" } else { "records" }
        ));
        for f in files {
            out.push(String::new());
            out.push(format!("`{}`", f.path));
            out.extend(f.lines.iter().map(|l| sanitize(l)));
        }
        out.push(String::new());
        out.push("</details>".to_owned());
    }
    out.push(String::new());
    let listed = if ids.is_empty() {
        String::new()
    } else {
        format!(" Records: {}.", ids.join(" "))
    };
    out.push(format!(
        "<sub>Written by `av notes` from the task's records.{listed} `av expand <id>` prints one in \
         full; after the merge, `av why path:line` names this task.</sub>"
    ));
    out.push(END.to_owned());
    out.join("\n") + "\n"
}

/// Where the block of `task_id` is in `text` (marker to end marker, inclusive).
pub fn find_block(text: &str, task_id: &str) -> Option<std::ops::Range<usize>> {
    let start = text.find(&start_marker(task_id))?;
    let end = text[start..].find(END)? + start + END.len();
    Some(start..end)
}

/// `text` with the block of `task_id` replaced by `block`, or `block` appended when there is
/// none. Other tasks' blocks and any text around it stay as they are.
pub fn replace_block(text: &str, task_id: &str, block: &str) -> String {
    let block = block.trim_end_matches('\n');
    match find_block(text, task_id) {
        Some(r) => format!("{}{block}{}", &text[..r.start], &text[r.end..]),
        None if text.trim().is_empty() => format!("{block}\n"),
        None => format!("{}\n\n{block}\n", text.trim_end()),
    }
}

/// What posting did.
#[derive(Debug, PartialEq)]
pub enum Posted {
    Created,
    Updated(u64),
    Unchanged(u64),
}

/// The `gh` program (`AV_GH` overrides it, e.g. in tests).
fn gh_program() -> String {
    std::env::var("AV_GH")
        .ok()
        .filter(|g| !g.trim().is_empty())
        .unwrap_or_else(|| "gh".to_owned())
}

fn gh(args: &[&str], gh_repo: Option<&str>, stdin: Option<&str>) -> Result<String> {
    let prog = gh_program();
    let mut cmd = Command::new(&prog);
    cmd.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());
    if let Some(r) = gh_repo {
        cmd.env("GH_REPO", r);
    }
    cmd.stdin(if stdin.is_some() {
        Stdio::piped()
    } else {
        Stdio::null()
    });
    let mut child = cmd.spawn().map_err(|e| {
        CliError::Local(format!(
            "cannot run {prog} (the GitHub CLI; install it or set AV_GH): {e}"
        ))
    })?;
    if let (Some(text), Some(mut pipe)) = (stdin, child.stdin.take()) {
        pipe.write_all(text.as_bytes())
            .map_err(|e| CliError::Local(format!("{prog}: {e}")))?;
    }
    let out = child
        .wait_with_output()
        .map_err(|e| CliError::Local(format!("{prog}: {e}")))?;
    if !out.status.success() {
        return Err(CliError::Local(format!(
            "{prog} {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        )));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// Every JSON object in `gh api --paginate` output: page arrays back to back, or one value per
/// line.
fn comments_of(output: &str) -> Vec<Value> {
    let mut out = vec![];
    for v in serde_json::Deserializer::from_str(output)
        .into_iter::<Value>()
        .map_while(std::result::Result::ok)
    {
        match v {
            Value::Array(items) => out.extend(items),
            other => out.push(other),
        }
    }
    out
}

/// The pull request of the current branch, as `gh pr view` finds it.
pub fn current_pr(gh_repo: Option<&str>) -> Result<u64> {
    let out = gh(
        &["pr", "view", "--json", "number", "--jq", ".number"],
        gh_repo,
        None,
    )?;
    out.trim().parse().map_err(|_| {
        CliError::Local(format!(
            "gh pr view did not name a pull request for this branch ({:?}); pass --pr N",
            out.trim()
        ))
    })
}

/// Posts `block` as a comment on pull request `pr`, or updates the comment that already holds
/// the block of `task_id` (left alone when the block is the same).
pub fn post(pr: u64, gh_repo: Option<&str>, task_id: &str, block: &str) -> Result<Posted> {
    let listed = gh(
        &[
            "api",
            "--paginate",
            &format!("repos/{{owner}}/{{repo}}/issues/{pr}/comments"),
        ],
        gh_repo,
        None,
    )?;
    let marker = start_marker(task_id);
    let existing = comments_of(&listed).into_iter().find(|c| {
        c["body"]
            .as_str()
            .is_some_and(|b| b.contains(&marker) && b.contains(END))
    });
    match existing {
        Some(c) => {
            let id = c["id"].as_u64().ok_or_else(|| {
                CliError::Local("gh api: a comment without a numeric id".to_owned())
            })?;
            let body = s(&c["body"]).replace("\r\n", "\n");
            let new = replace_block(&body, task_id, block);
            if new == body {
                return Ok(Posted::Unchanged(id));
            }
            gh(
                &[
                    "api",
                    "-X",
                    "PATCH",
                    &format!("repos/{{owner}}/{{repo}}/issues/comments/{id}"),
                    "--input",
                    "-",
                ],
                gh_repo,
                Some(&json!({ "body": new }).to_string()),
            )?;
            Ok(Posted::Updated(id))
        }
        None => {
            gh(
                &[
                    "api",
                    "-X",
                    "POST",
                    &format!("repos/{{owner}}/{{repo}}/issues/{pr}/comments"),
                    "--input",
                    "-",
                ],
                gh_repo,
                Some(&json!({ "body": block }).to_string()),
            )?;
            Ok(Posted::Created)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TASK: &str = "3f0c2b1a-0000-4000-8000-000000000001";

    fn fixture() -> (Value, Vec<Value>, Vec<FileNotes>) {
        let task = json!({
            "task": {"id": TASK, "repo": "pantry", "slug": "atomic-save", "title": "Atomic save",
                     "goal": "Write pantry.json atomically so a crash cannot truncate it.",
                     "status": "in_review", "owner_agent": "claude"},
            "changed": ["pantry/store.py", "pantry/cli.py", "tests/test_store.py"],
            "merge": {"merged_by": null},
            "verdict": {"reviewer": "codex", "verdict": "approve"},
        });
        let rec = |id: &str, kind: &str, summary: &str, body: &str, anchors: Value| {
            json!({"id": id, "kind": kind, "summary": summary, "body": body,
                   "author_agent": "claude", "anchors": anchors, "superseded_by": null})
        };
        let records = vec![
            rec(
                "a1000000-0000-4000-8000-000000000000",
                "intent",
                "Atomic save",
                "",
                json!([]),
            ),
            rec(
                "b2000000-0000-4000-8000-000000000000",
                "decision",
                "Write to a temp file in the same dir, then os.replace",
                "os.replace is atomic on POSIX and Windows\nwhen both paths are on one volume.",
                json!([{"path": "pantry/store.py", "symbol": "save"}]),
            ),
            rec(
                "c3000000-0000-4000-8000-000000000000",
                "attempt",
                "tried os.rename; fails on Windows if the target exists",
                "",
                json!([{"path": "pantry/store.py", "symbol": null}]),
            ),
            {
                let mut old = rec(
                    "d4000000-0000-4000-8000-000000000000",
                    "note",
                    "superseded: fsync is not needed",
                    "",
                    json!([]),
                );
                old["superseded_by"] = json!("e5000000-0000-4000-8000-000000000000");
                old
            },
            rec(
                "e5000000-0000-4000-8000-000000000000",
                "note",
                "fsync the temp file before replace; ask @alice about NFS <!-- x -->",
                "",
                json!([]),
            ),
            {
                let mut r = rec(
                    "f6000000-0000-4000-8000-000000000000",
                    "review",
                    "missing a test for a crash between write and replace",
                    "",
                    json!([{"path": "tests/test_store.py", "symbol": null}]),
                );
                r["author_agent"] = json!("codex");
                r
            },
            rec(
                "a7000000-0000-4000-8000-000000000000",
                "validation",
                "`pytest -q` exit 0",
                "",
                json!([]),
            ),
        ];
        let files = vec![FileNotes {
            path: "pantry/store.py".into(),
            lines: vec![
                "- rec:99990000 decision: load() returns {} for a missing file".into(),
                "  anchors: pantry/store.py:load".into(),
            ],
        }];
        (task, records, files)
    }

    #[test]
    fn render_matches_the_golden_file() {
        let (task, records, files) = fixture();
        let got = render(&task, &records, &files);
        let want = include_str!("../tests/golden/notes.md").replace("\r\n", "\n");
        if got != want {
            let _ = std::fs::write(
                concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/notes.md.actual"),
                &got,
            );
        }
        assert_eq!(got, want, "see tests/golden/notes.md.actual");
    }

    #[test]
    fn render_without_records_says_so_and_still_closes_the_block() {
        let (task, _, _) = fixture();
        let md = render(&task, &[], &[]);
        assert!(md.starts_with(&start_marker(TASK)), "{md}");
        assert!(md.trim_end().ends_with(END), "{md}");
        assert!(md.contains("No decisions, dead ends or notes"), "{md}");
    }

    #[test]
    fn sanitize_keeps_markers_closed_and_mentions_quiet() {
        let s = sanitize("a --> b <!-- c @bob mail me@x.org @");
        assert!(!s.contains("-->") && !s.contains("<!--"), "{s}");
        assert!(s.contains("@\u{2060}bob"), "{s}");
        assert!(s.contains("me@x.org"), "an address is not a mention: {s}");
        assert!(s.ends_with('@'), "{s}");
    }

    #[test]
    fn replacing_a_block_is_idempotent_and_leaves_the_rest() {
        let (task, records, files) = fixture();
        let block = render(&task, &records, &files);
        let other = "<!-- margincode:notes v1 task=other -->\nold\n<!-- /margincode:notes -->";
        let text = format!("Intro by a human.\n\n{other}\n");
        let once = replace_block(&text, TASK, &block);
        assert!(once.starts_with("Intro by a human.") && once.contains(other));
        assert_eq!(
            replace_block(&once, TASK, &block),
            once,
            "same block, same text"
        );
        let newer = block.replace("Atomic save**", "Atomic save v2**");
        let twice = replace_block(&once, TASK, &newer);
        assert_eq!(twice.matches(&start_marker(TASK)).count(), 1, "{twice}");
        assert!(twice.contains("Atomic save v2") && twice.contains(other));
        assert_eq!(replace_block("", TASK, &block), block);
    }

    #[test]
    fn comments_are_read_from_paginated_and_line_output() {
        let pages = r#"[{"id":1,"body":"a"}][{"id":2,"body":"b"}]"#;
        assert_eq!(comments_of(pages).len(), 2);
        assert_eq!(comments_of("{\"id\":1}\n{\"id\":2}\n").len(), 2);
        assert!(comments_of("").is_empty());
    }

    #[test]
    fn records_at_answer_keeps_only_record_lines() {
        let answer = json!({"markdown": "**av: recorded about `a.py`** (by other tasks)\n- rec:1 note: x\n  anchors: a.py\nFull text: `av expand <id>`.", "records": ["rec:1"]});
        let f = FileNotes::from_records_at("a.py", &answer).unwrap();
        assert_eq!(f.lines, vec!["- rec:1 note: x", "  anchors: a.py"]);
        assert!(FileNotes::from_records_at("a.py", &json!({"markdown": ""})).is_none());
    }
}
