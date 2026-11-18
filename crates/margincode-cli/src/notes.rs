//! `av notes`: a task's records as a Markdown block for its pull request, posted with `gh`
//! because the daemon is out of a CI job's reach. The block sits between two markers, one per
//! task, so posting again replaces it in place and an unchanged block is not sent.

use std::io::Write as _;
use std::process::{Command, Stdio};
use std::sync::LazyLock;

use regex::Regex;
use serde_json::{Value, json};

use crate::client::{CliError, Result};

pub const END: &str = "<!-- /margincode:notes -->";
/// Changed files whose earlier records the block looks up (`records/at`, one request each).
pub const MAX_FILES: usize = 20;
const MAX_BODY_CHARS: usize = 600;

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

/// The root of a home directory, up to its user name. Windows forms (`C:\Users` and
/// `/mnt/c/Users` in any case, `/c/Users`) are group 2; Unix forms (`/home`,
/// `/var|usr|export/home`, `/Users`, macOS's data volume, WSL's `\\wsl$\DISTRO\home`) are group 3
/// and case-sensitive, so a route such as `/users/42` is not one. Separators may be doubled or JSON-escaped. The root
/// must start a path: not after a path character, `/` or `~` (`/srv/home`, `~/home`,
/// `https://home`), but after a short flag (`-I/home/...`) or `file://`.
static HOME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?m)(^-[a-zA-Z]|^|:/{2,}|[^a-zA-Z0-9_.~/\-]-[a-zA-Z]|[^a-zA-Z0-9_.~/\-])(?:((?i:[a-z]:[\\/]+users)|[\\/]+mnt[\\/]+[a-zA-Z][\\/]+(?i:users)|[\\/]+[a-zA-Z][\\/]+Users)|((?i:[\\/]{2,}wsl(?:\$|\.localhost))[\\/]+[^\\/\s]+[\\/]+home|\\?/(?:(?:var|usr|export)\\?/)?home|\\?/(?:System\\?/Volumes\\?/Data\\?/)?Users))[\\/]+",
    )
    .unwrap()
});

fn is_name_char(c: char) -> bool {
    c.is_alphanumeric() || "._-$@+'%".contains(c)
}

/// The length of the user name at the start of `rest`. A Windows name may hold spaces, but only
/// before a capitalised word that a separator closes: `C:\Users\Ana B\x` is one name, while
/// `cd C:\Users\ana && dir` and `xcopy C:\Users\ana backup\x` are not.
fn home_name(rest: &str, windows: bool) -> usize {
    let word = |s: &str| s.find(|c: char| !is_name_char(c)).unwrap_or(s.len());
    let mut end = word(rest);
    let mut at = end;
    while windows && rest[at..].starts_with(' ') {
        let next = &rest[at + 1..];
        let w = word(next);
        if w == 0 || !next.starts_with(|c: char| c.is_uppercase() || c.is_ascii_digit()) {
            break;
        }
        at += 1 + w;
        if rest[at..].starts_with(['\\', '/']) {
            end = at;
            break;
        }
    }
    rest[..end].trim_end_matches(['.', '\'']).len()
}

/// Every home directory in `text` as `~`. By pattern rather than this machine's home, because the
/// block may hold paths from whichever machine validated.
fn hide_homes(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut last = 0;
    for c in HOME.captures_iter(text) {
        let m = c.get(0).unwrap();
        if m.start() < last {
            continue;
        }
        let name = home_name(&text[m.end()..], c.get(2).is_some());
        if name == 0 {
            continue;
        }
        out.push_str(&text[last..m.start()]);
        out.push_str(&c[1]);
        out.push('~');
        last = m.end() + name;
    }
    out.push_str(&text[last..]);
    out
}

/// A check's command and exit code, from its body.
fn check_run(rec: &Value) -> Option<(String, i64)> {
    if s(&rec["kind"]) != "validation" {
        return None;
    }
    let body = serde_json::from_str::<Value>(s(&rec["body"])).ok()?;
    Some((s(&body["command"]).to_owned(), body["exit_code"].as_i64()?))
}

/// The records the block shows: not superseded, and not a check that failed at an older head
/// when the same command has since passed at the task's head (a failure that was never fixed
/// stays, however the head moved).
pub fn shown<'a>(task: &Value, records: &'a [Value]) -> Vec<&'a Value> {
    let head = s(&task["task"]["head_change"]);
    let at_head = |r: &Value| !head.is_empty() && s(&r["change_id"]) == head;
    let passed: Vec<String> = records
        .iter()
        .filter(|r| at_head(r))
        .filter_map(check_run)
        .filter(|(cmd, code)| *code == 0 && !cmd.is_empty())
        .map(|(cmd, _)| cmd)
        .collect();
    records
        .iter()
        .filter(|r| r["superseded_by"].is_null())
        .filter(|r| {
            let fixed = !at_head(r)
                && !s(&r["change_id"]).is_empty()
                && check_run(r).is_some_and(|(cmd, code)| code != 0 && passed.contains(&cmd));
            !fixed
        })
        .collect()
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
    let live = shown(task, records);
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
    let goal = stated_intent(t, records);
    let goal = goal.trim();
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
    hide_homes(&out.join("\n")) + "\n"
}

/// The daemon's merge message rule (`flow::stated_intent`): once the intent made from the goal
/// was superseded, the newest live intent; until then the goal.
fn stated_intent(task: &Value, records: &[Value]) -> String {
    let intents: Vec<&Value> = records
        .iter()
        .filter(|r| s(&r["kind"]) == "intent")
        .collect();
    let restated = intents
        .first()
        .is_some_and(|r| !r["superseded_by"].is_null());
    match intents.iter().rev().find(|r| r["superseded_by"].is_null()) {
        Some(live) if restated && s(&live["body"]).trim().is_empty() => {
            s(&live["summary"]).to_owned()
        }
        Some(live) if restated => format!("{}\n\n{}", s(&live["summary"]), s(&live["body"]).trim()),
        _ => s(&task["goal"]).to_owned(),
    }
}

pub fn find_block(text: &str, task_id: &str) -> Option<std::ops::Range<usize>> {
    let start = text.find(&start_marker(task_id))?;
    let end = text[start..].find(END)? + start + END.len();
    Some(start..end)
}

pub fn replace_block(text: &str, task_id: &str, block: &str) -> String {
    let block = block.trim_end_matches('\n');
    match find_block(text, task_id) {
        Some(r) => format!("{}{block}{}", &text[..r.start], &text[r.end..]),
        None if text.trim().is_empty() => format!("{block}\n"),
        None => format!("{}\n\n{block}\n", text.trim_end()),
    }
}

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
    fn a_restated_intent_is_the_goal_quoted() {
        let (task, mut records, files) = fixture();
        records[0]["superseded_by"] = json!("b8000000-0000-4000-8000-000000000000");
        records.push(json!({"id": "b8000000-0000-4000-8000-000000000000", "kind": "intent",
            "summary": "Atomic save through a temp file", "body": "The lock file was the first design.",
            "author_agent": "claude", "anchors": [], "superseded_by": null}));
        let got = render(&task, &records, &files);
        assert!(
            got.contains(
                "> Atomic save through a temp file\n>\n> The lock file was the first design.\n"
            ),
            "{got}"
        );
        assert!(!got.contains("crash cannot truncate"), "{got}");
    }

    #[test]
    fn home_directories_are_not_published() {
        for (text, want) in [
            (
                "cd /home/zoltan/src && cargo test",
                "cd ~/src && cargo test",
            ),
            (
                "PATH=/usr/bin:/home/zoltan/.cargo/bin",
                "PATH=/usr/bin:~/.cargo/bin",
            ),
            ("PATH=/home/zoltan:/usr/bin", "PATH=~:/usr/bin"),
            ("-v /home/zoltan:/work", "-v ~:/work"),
            ("gcc -I/home/zoltan/inc x.c", "gcc -I~/inc x.c"),
            (
                "cat x >/home/zoltan/out 2>/Users/zoltan/err",
                "cat x >~/out 2>~/err",
            ),
            (
                "file:///home/zoltan/x and [/home/zoltan/a]",
                "file://~/x and [~/a]",
            ),
            ("**/home/zoltan/a** {/home/zoltan,/x}", "**~/a** {~,/x}"),
            ("/var/home/zoltan/x", "~/x"),
            ("see /home/zoltan. Next", "see ~. Next"),
            ("`/home/zoltan`, \"/Users/zoltan/z\"", "`~`, \"~/z\""),
            (
                r"logs in C:\Users\Zoltan Q\proj\out.txt",
                r"logs in ~\proj\out.txt",
            ),
            (r"cd C:\Users\zoltan && dir D:\proj", r"cd ~ && dir D:\proj"),
            (r"xcopy C:\Users\zoltan D:\backup\x", r"xcopy ~ D:\backup\x"),
            (r"c:\users\zoltan\x and C:\USERS\zoltan\y", r"~\x and ~\y"),
            (r"\\?\C:\Users\zoltan\x", r"\\?\~\x"),
            (
                r#""C:\\Users\\zoltan\\x" \/home\/zoltan\/x"#,
                r#""~\\x" ~\/x"#,
            ),
            (
                "C:/Users/zoltan/x /c/Users/zoltan/y /mnt/c/Users/zoltan/z",
                "~/x ~/y ~/z",
            ),
            ("file:///C:/Users/zoltan/x", "file:///~/x"),
            (
                "https://example.com/home/page/ /srv/home/x/ ./home/x",
                "https://example.com/home/page/ /srv/home/x/ ./home/x",
            ),
            (
                "already ~/x and /home/ alone",
                "already ~/x and /home/ alone",
            ),
            (
                "GET /users/42 ~/home/foo cd ~/Users/x https://home/dash release-a/home/x",
                "GET /users/42 ~/home/foo cd ~/Users/x https://home/dash release-a/home/x",
            ),
            (
                r"\\wsl$\Ubuntu\home\zoltan\x \\wsl.localhost\Debian\home\zoltan\y",
                r"~\x ~\y",
            ),
            (
                "/usr/home/zoltan/x /export/home/zoltan/y /System/Volumes/Data/Users/zoltan/z",
                "~/x ~/y ~/z",
            ),
            (
                "/Users/O'Zoltan/x '/home/zoltan' file:///C:/Users/Zoltan%20Q/x",
                "~/x '~' file:///~/x",
            ),
            (r"xcopy C:\Users\zoltan backup\x", r"xcopy ~ backup\x"),
            (
                "RUSTFLAGS=\"-L/home/zoltan/lib\" CFLAGS='-I/home/zoltan/inc'",
                "RUSTFLAGS=\"-L~/lib\" CFLAGS='-I~/inc'",
            ),
            ("-I/home/zoltan/x", "-I~/x"),
            ("/mnt/c/users/zoltan/x /c/users/list", "~/x /c/users/list"),
        ] {
            assert_eq!(hide_homes(text), want, "{text}");
        }
        let (task, mut records, files) = fixture();
        records.push(
            json!({"id": "b9000000-0000-4000-8000-000000000000", "kind": "validation",
            "summary": "`cd /home/zoltan/src && cargo test` exit 0", "body": "",
            "author_agent": "claude", "anchors": [], "superseded_by": null}),
        );
        let got = render(&task, &records, &files);
        assert!(
            !got.contains("zoltan") && got.contains("`cd ~/src && cargo test` exit 0"),
            "{got}"
        );
    }

    #[test]
    fn failed_checks_at_older_heads_are_left_out() {
        let (mut task, mut records, files) = fixture();
        task["task"]["head_change"] = json!("h2");
        let check = |n: u8, cmd: &str, code: i64, head: &str| {
            json!({"id": format!("c{n}000000-0000-4000-8000-000000000000"), "kind": "validation",
                   "summary": format!("`{cmd}` exit {code}"),
                   "body": json!({"command": cmd, "exit_code": code, "head_change": head}).to_string(),
                   "change_id": head, "author_agent": "claude", "anchors": [], "superseded_by": null})
        };
        records.extend([
            check(1, "fixed", 1, "h1"),
            check(2, "never-rerun", 1, "h1"),
            check(3, "old-pass", 0, "h1"),
            check(4, "head-fail", 2, "h2"),
            check(5, "fixed", 0, "h2"),
        ]);
        let got = render(&task, &records, &files);
        assert!(
            !got.contains("`fixed` exit 1") && !got.contains("rec:c1000000"),
            "{got}"
        );
        for kept in [
            "`never-rerun` exit 1",
            "`old-pass` exit 0",
            "`head-fail` exit 2",
            "`fixed` exit 0",
        ] {
            assert!(got.contains(kept), "{kept} not in {got}");
        }
        let ids: Vec<&str> = shown(&task, &records).iter().map(|r| s(&r["id"])).collect();
        assert!(
            !ids.contains(&"c1000000-0000-4000-8000-000000000000"),
            "{ids:?}"
        );
        records.retain(|r| s(&r["id"]) != "c5000000-0000-4000-8000-000000000000");
        let got = render(&task, &records, &files);
        assert!(
            got.contains("`fixed` exit 1"),
            "the command has not passed at the head, so its failure stays: {got}"
        );
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
