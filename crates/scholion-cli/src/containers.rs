//! Containers are footprint too. Everything a session starts through docker carries the labels
//! `av.task=<task id>` and `av.agent=<agent>` (the session-start hook exports them as
//! `AV_DOCKER_LABELS`); the Stop hook counts what carries the task's label and `av gc
//! --containers` removes what finished tasks left. The daemon never runs docker: all of it is
//! here, in the CLI, and only when `docker` is on `PATH`.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use serde::Serialize;
use serde_json::{Value, json};

use crate::client::Client;

/// The label naming the task a container, image or volume belongs to.
pub const TASK_LABEL: &str = "av.task";
/// The label naming the agent that started it.
pub const AGENT_LABEL: &str = "av.agent";

/// The flags for `docker run`/`docker build`: `--label av.task=<task> --label av.agent=<agent>`.
pub fn labels(task: &str, agent: &str) -> String {
    format!("--label {TASK_LABEL}={task} --label {AGENT_LABEL}={agent}")
}

/// The `docker` executable on `PATH`, if any. Looked up, not run: a hook must cost nothing when
/// there is none.
pub fn docker() -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    let names: &[&str] = if cfg!(windows) {
        &["docker.exe", "docker"]
    } else {
        &["docker"]
    };
    std::env::split_paths(&path)
        .filter(|dir| !dir.as_os_str().is_empty())
        .flat_map(|dir| names.iter().map(move |n| dir.join(n)))
        .find(|p| p.is_file())
}

/// Runs `docker args…` and returns its stdout, or `None` when it fails, exits non-zero or has not
/// ended by `deadline` (then it is killed: a hung docker must not eat a hook's time budget).
pub fn run(docker: &Path, args: &[&str], deadline: Instant) -> Option<String> {
    let mut child = Command::new(docker)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        // nothing reads it, and an unread pipe would stall a chatty docker until the deadline
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    let mut stdout = child.stdout.take()?;
    let reader = std::thread::spawn(move || {
        let mut text = String::new();
        let _ = std::io::Read::read_to_string(&mut stdout, &mut text);
        text
    });
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let text = reader.join().ok()?;
                return status.success().then_some(text);
            }
            Ok(None) if Instant::now() >= deadline => {
                let _ = child.kill();
                let _ = child.wait();
                return None;
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(20)),
            Err(_) => return None,
        }
    }
}

/// `docker ps` columns: id, name, state, size (docker's own text) and the task label.
pub const PS_FORMAT: &str = "{{.ID}}\t{{.Names}}\t{{.State}}\t{{.Size}}\t{{.Label \"av.task\"}}";
/// `docker image inspect` columns: id, task label, size in bytes, tags (comma-separated).
/// `docker images --format` has no label placeholder, so the ids it lists are inspected.
pub const INSPECT_FORMAT: &str =
    "{{.Id}}\t{{index .Config.Labels \"av.task\"}}\t{{.Size}}\t{{join .RepoTags \",\"}}";
/// `docker volume ls` columns: name and the task label.
pub const VOLUME_FORMAT: &str = "{{.Name}}\t{{.Label \"av.task\"}}";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Container {
    pub id: String,
    pub name: String,
    /// `running`, `exited`, `created`, `paused`, …, as `docker ps` says it.
    pub state: String,
    /// As `docker ps --size` prints it (`12MB (virtual 1.2GB)`); empty when not given.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub size: String,
    #[serde(skip)]
    pub task: String,
}

impl Container {
    /// `docker rm` would refuse it: it has to be stopped first, which av never does.
    pub fn is_running(&self) -> bool {
        matches!(self.state.as_str(), "running" | "paused" | "restarting")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Image {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<u64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(skip)]
    pub task: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Volume {
    pub name: String,
    #[serde(skip)]
    pub task: String,
}

/// What carries an `av.task` label on this host's docker.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Report {
    pub containers: Vec<Container>,
    pub images: Vec<Image>,
    pub volumes: Vec<Volume>,
}

impl Report {
    pub fn is_empty(&self) -> bool {
        self.containers.is_empty() && self.images.is_empty() && self.volumes.is_empty()
    }

    pub fn running(&self) -> impl Iterator<Item = &Container> {
        self.containers.iter().filter(|c| c.is_running())
    }

    /// Every task id that appears, each once, in order of first appearance.
    pub fn tasks(&self) -> Vec<&str> {
        let mut seen = Vec::new();
        let all = self
            .containers
            .iter()
            .map(|c| c.task.as_str())
            .chain(self.images.iter().map(|i| i.task.as_str()))
            .chain(self.volumes.iter().map(|v| v.task.as_str()));
        for t in all {
            if !t.is_empty() && !seen.contains(&t) {
                seen.push(t);
            }
        }
        seen
    }
}

fn columns(line: &str) -> Vec<&str> {
    line.split('\t').map(str::trim).collect()
}

/// `docker ps` output in `PS_FORMAT`; lines that are not are skipped.
pub fn parse_ps(text: &str) -> Vec<Container> {
    text.lines()
        .map(columns)
        .filter(|c| c.len() >= 5 && !c[0].is_empty())
        .map(|c| Container {
            id: c[0].to_owned(),
            name: c[1].to_owned(),
            state: c[2].to_owned(),
            size: c[3].to_owned(),
            task: c[4].to_owned(),
        })
        .collect()
}

/// `docker image inspect` output in `INSPECT_FORMAT`.
pub fn parse_inspect(text: &str) -> Vec<Image> {
    text.lines()
        .map(columns)
        .filter(|c| c.len() >= 4 && !c[0].is_empty())
        .map(|c| Image {
            id: c[0].to_owned(),
            task: c[1].to_owned(),
            bytes: c[2].parse().ok(),
            tags: c[3]
                .split(',')
                .filter(|t| !t.is_empty() && *t != "<none>:<none>")
                .map(str::to_owned)
                .collect(),
        })
        .collect()
}

/// `docker volume ls` output in `VOLUME_FORMAT`.
pub fn parse_volumes(text: &str) -> Vec<Volume> {
    text.lines()
        .map(columns)
        .filter(|c| c.len() >= 2 && !c[0].is_empty())
        .map(|c| Volume {
            name: c[0].to_owned(),
            task: c[1].to_owned(),
        })
        .collect()
}

/// What to list: one task's things or every task's, and whether only dangling (untagged)
/// images count, as `av gc` removes only those.
#[derive(Debug, Clone, Copy, Default)]
pub struct Scope<'a> {
    pub task: Option<&'a str>,
    pub dangling_images: bool,
}

impl Scope<'_> {
    fn label(&self) -> String {
        match self.task {
            Some(t) => format!("label={TASK_LABEL}={t}"),
            None => format!("label={TASK_LABEL}"),
        }
    }

    fn keeps(&self, task: &str) -> bool {
        self.task.is_none_or(|t| t == task)
    }
}

/// Lists the containers, images and volumes in `scope` (three or four docker calls, all read-only),
/// by `deadline`. What a failing call would have listed is left out.
pub fn scan(docker: &Path, scope: Scope<'_>, deadline: Instant) -> Report {
    let label = scope.label();
    let mut report = Report::default();
    if let Some(text) = run(
        docker,
        &[
            "ps", "-a", "--size", "--filter", &label, "--format", PS_FORMAT,
        ],
        deadline,
    ) {
        report.containers = parse_ps(&text)
            .into_iter()
            .filter(|c| scope.keeps(&c.task))
            .collect();
    }
    let mut image_args = vec!["images", "-q", "--no-trunc", "--filter", label.as_str()];
    if scope.dangling_images {
        image_args.extend(["--filter", "dangling=true"]);
    }
    if let Some(ids) = run(docker, &image_args, deadline) {
        let mut ids: Vec<&str> = ids
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .collect();
        ids.sort_unstable();
        ids.dedup();
        if !ids.is_empty() {
            let mut args = vec!["image", "inspect", "--format", INSPECT_FORMAT];
            args.extend(ids);
            if let Some(text) = run(docker, &args, deadline) {
                report.images = parse_inspect(&text)
                    .into_iter()
                    .filter(|i| scope.keeps(&i.task))
                    .collect();
            }
        }
    }
    if let Some(text) = run(
        docker,
        &[
            "volume",
            "ls",
            "--filter",
            &label,
            "--format",
            VOLUME_FORMAT,
        ],
        deadline,
    ) {
        report.volumes = parse_volumes(&text)
            .into_iter()
            .filter(|v| scope.keeps(&v.task))
            .collect();
    }
    report
}

/// How long `av gc --containers` gives docker in all.
const GC_SECS: u64 = 120;

/// `av gc --containers`: removes what docker holds for tasks the daemon says are merged or
/// abandoned (stopped containers, then dangling images, then volumes, each with the task's
/// label), one `docker rm`/`rmi`/`volume rm` each so a failure names its item. Running
/// containers are listed under `running` and never stopped; what belongs to open tasks, or to
/// tasks the daemon does not know, is only counted (`left`). With `dry_run` nothing is removed.
pub fn gc(client: &Client, docker: &Path, dry_run: bool) -> Value {
    let deadline = Instant::now() + Duration::from_secs(GC_SECS);
    let scope = Scope {
        task: None,
        dangling_images: true,
    };
    let report = scan(docker, scope, deadline);
    let mut removed = Vec::new();
    let mut running = Vec::new();
    let mut errors = Vec::new();
    let (mut open, mut unknown) = (0usize, 0usize);
    for task in report.tasks() {
        let containers: Vec<&Container> = report
            .containers
            .iter()
            .filter(|c| c.task == task)
            .collect();
        let images: Vec<&Image> = report.images.iter().filter(|i| i.task == task).collect();
        let volumes: Vec<&Volume> = report.volumes.iter().filter(|v| v.task == task).collect();
        let count = containers.len() + images.len() + volumes.len();
        let Some((repo, slug, finished)) = task_state(client, task) else {
            unknown += count;
            continue;
        };
        if !finished {
            open += count;
            continue;
        }
        let item = |kind: &str, name: &str, id: &str| json!({"kind": kind, "task": task, "repo": repo, "slug": slug, "name": name, "id": id});
        let mut remove = |args: &[&str], mut entry: Value| {
            if !dry_run && run(docker, args, deadline).is_none() {
                entry["error"] = json!(format!("docker {} failed", args.join(" ")));
                errors.push(entry);
            } else {
                removed.push(entry);
            }
        };
        for c in &containers {
            let mut entry = item("container", &c.name, &c.id);
            entry["state"] = json!(c.state);
            if !c.size.is_empty() {
                entry["size"] = json!(c.size);
            }
            if c.is_running() {
                running.push(entry);
            } else {
                remove(&["rm", &c.id], entry);
            }
        }
        for i in &images {
            let mut entry = item("image", &i.id, &i.id);
            if let Some(b) = i.bytes {
                entry["bytes"] = json!(b);
            }
            remove(&["rmi", &i.id], entry);
        }
        for v in &volumes {
            remove(&["volume", "rm", &v.name], item("volume", &v.name, &v.name));
        }
    }
    json!({
        "dry_run": dry_run,
        "removed": removed,
        "running": running,
        "errors": errors,
        "left": {"open": open, "unknown": unknown},
    })
}

/// `(repo, slug, finished)` of a task the daemon knows; `None` for one it does not.
fn task_state(client: &Client, task: &str) -> Option<(String, String, bool)> {
    let t = client.get(&format!("/v1/tasks/{task}"), &[]).ok()?;
    let t = &t["task"];
    Some((
        t["repo"].as_str()?.to_owned(),
        t["slug"].as_str()?.to_owned(),
        matches!(t["status"].as_str(), Some("merged" | "abandoned")),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn docker_output_is_read_by_columns_and_bad_lines_are_skipped() {
        let ps = parse_ps(
            "abc\tav-build\trunning\t12MB (virtual 1.2GB)\tt1\nnot a row\n\
                           def\tav-db\texited\t\tt2\n",
        );
        assert_eq!(ps.len(), 2, "{ps:?}");
        assert!(ps[0].is_running() && !ps[1].is_running());
        assert_eq!((ps[1].name.as_str(), ps[1].size.as_str()), ("av-db", ""));
        let images = parse_inspect(
            "sha256:a\tt1\t1234\tdemo/build:latest,demo/build:v1\n\
                                    sha256:b\tt2\t\t<none>:<none>\n",
        );
        assert_eq!(images[0].bytes, Some(1234));
        assert_eq!(images[0].tags, ["demo/build:latest", "demo/build:v1"]);
        assert_eq!((images[1].bytes, images[1].tags.len()), (None, 0));
        let volumes = parse_volumes("pgdata\tt1\n");
        assert_eq!(volumes[0].name, "pgdata");
        let report = Report {
            containers: ps,
            images,
            volumes,
        };
        assert_eq!(report.tasks(), ["t1", "t2"]);
        assert!(!report.is_empty() && Report::default().is_empty());
    }

    #[test]
    fn labels_name_the_task_and_the_agent() {
        assert_eq!(
            labels("0b2c", "grok"),
            "--label av.task=0b2c --label av.agent=grok"
        );
    }
}
