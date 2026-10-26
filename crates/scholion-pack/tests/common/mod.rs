#![allow(dead_code)]

use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use scholion_core::*;
use scholion_index::{CodeIndex, PgIndex};
use scholion_pack::{PackRequest, Packer};
use scholion_store::PgStore;
use scholion_testutil::TestDb;

/// In-memory `Vcs`: a change id maps to a complete file set.
#[derive(Default)]
pub struct FakeVcs {
    changes: Mutex<HashMap<String, BTreeMap<String, Vec<u8>>>>,
}

impl FakeVcs {
    pub fn set(&self, change: &str, files: &[(&str, &str)]) {
        let map = files
            .iter()
            .map(|(p, c)| (p.to_string(), c.as_bytes().to_vec()))
            .collect();
        self.changes.lock().unwrap().insert(change.to_owned(), map);
    }

    fn files(&self, change: &ChangeId) -> Result<BTreeMap<String, Vec<u8>>> {
        self.changes
            .lock()
            .unwrap()
            .get(&change.0)
            .cloned()
            .ok_or_else(|| AvError::NotFound(format!("change {}", change.0)))
    }
}

#[async_trait]
impl Vcs for FakeVcs {
    async fn init_repo(&self, _: &str, _: &Path) -> Result<ChangeId> {
        Err(AvError::Vcs("unsupported".into()))
    }
    async fn create_workspace(&self, _: &str, _: &str) -> Result<WorkspaceInfo> {
        Err(AvError::Vcs("unsupported".into()))
    }
    async fn snapshot(&self, _: &str, _: &str) -> Result<ChangeId> {
        Err(AvError::Vcs("unsupported".into()))
    }
    async fn changed_paths(
        &self,
        _: &str,
        base: &ChangeId,
        head: &ChangeId,
    ) -> Result<Vec<String>> {
        let (b, h) = (self.files(base)?, self.files(head)?);
        let mut out: Vec<String> = h
            .iter()
            .filter(|(p, c)| b.get(*p) != Some(c))
            .map(|(p, _)| p.clone())
            .collect();
        out.extend(b.keys().filter(|p| !h.contains_key(*p)).cloned());
        out.sort();
        Ok(out)
    }
    async fn list_files(&self, _: &str, change: &ChangeId) -> Result<Vec<String>> {
        Ok(self.files(change)?.into_keys().collect())
    }
    async fn read_file(&self, _: &str, change: &ChangeId, path: &str) -> Result<Vec<u8>> {
        self.files(change)?
            .remove(path)
            .ok_or_else(|| AvError::NotFound(path.to_owned()))
    }
}

pub const BASE: &[(&str, &str)] = &[
    ("app/util.py", "def format_name(n):\n    return n.title()\n"),
    (
        "app/greeter.py",
        "from util import format_name\n\nclass Greeter:\n    def hello(self, name):\n        return format_name(name)\n\n    def goodbye(self, name):\n        return \"bye \" + name\n",
    ),
    ("app/unrelated.py", "def zebra():\n    return 1\n"),
    ("README.md", "# demo\n"),
];

pub const HEAD: &[(&str, &str)] = &[
    ("app/util.py", "def format_name(n):\n    return n.title()\n"),
    (
        "app/greeter.py",
        "from util import format_name\n\nclass Greeter:\n    def hello(self, name):\n        return format_name(name) + \"!\"\n\n    def goodbye(self, name):\n        return \"bye \" + name\n",
    ),
    ("app/unrelated.py", "def zebra():\n    return 1\n"),
    ("README.md", "# demo\n"),
];

pub struct Fx {
    pub _db: TestDb,
    pub store: Arc<PgStore>,
    pub index: Arc<PgIndex>,
    pub vcs: Arc<FakeVcs>,
    pub packer: Packer,
    pub task: Task,
}

pub fn new_task(title: &str) -> NewTask {
    NewTask {
        repo: "demo".into(),
        title: title.into(),
        goal: "Make hello louder".into(),
        owner_agent: "claude".into(),
    }
}

pub async fn fixture() -> Fx {
    let db = TestDb::new().await;
    let store = Arc::new(PgStore::connect(&db.url).await.unwrap());
    let index = Arc::new(PgIndex::new(store.pool().clone()));
    let vcs = Arc::new(FakeVcs::default());
    vcs.set("base", BASE);
    vcs.set("head", HEAD);
    let packer = Packer::new(vcs.clone(), store.clone(), index.clone());

    let task = store
        .create_task(new_task("Improve greeting"))
        .await
        .unwrap();
    store
        .set_workspace(
            task.id,
            Path::new("/ws/demo/improve-greeting"),
            &ChangeId("base".into()),
        )
        .await
        .unwrap();
    store
        .set_head(
            task.id,
            &ChangeId("head".into()),
            &["app/greeter.py".into()],
        )
        .await
        .unwrap();
    index
        .index_paths(
            "demo",
            &task.slug,
            // like the daemon: only code files are indexed, so README.md is known to the VCS but not the index
            HEAD.iter()
                .filter(|(p, _)| p.ends_with(".py"))
                .map(|(p, c)| (p.to_string(), c.as_bytes().to_vec()))
                .collect(),
        )
        .await
        .unwrap();
    store
        .add_record(record(
            task.id,
            RecordKind::Intent,
            "Make hello louder",
            "",
            vec![],
        ))
        .await
        .unwrap();
    let task = store.get_task(task.id).await.unwrap();
    Fx {
        _db: db,
        store,
        index,
        vcs,
        packer,
        task,
    }
}

pub fn record(
    task: TaskId,
    kind: RecordKind,
    summary: &str,
    body: &str,
    anchors: Vec<Anchor>,
) -> NewRecord {
    NewRecord {
        task_id: task,
        kind,
        summary: summary.into(),
        body: body.into(),
        change_id: None,
        supersedes: None,
        author_agent: "claude".into(),
        anchors,
    }
}

pub fn anchor(path: &str, symbol: Option<&str>) -> Anchor {
    Anchor {
        path: path.into(),
        symbol: symbol.map(str::to_owned),
    }
}

pub fn req(task: &Task, budget: u32, session: &str) -> PackRequest {
    PackRequest {
        task_id: task.id,
        budget_tokens: budget,
        focus: None,
        session: Some(session.into()),
        family: AgentFamily::Other,
    }
}

/// First `prefix` + `n` chars id token found in `text`, e.g. `rec:1a2b3c4d`.
pub fn find_id(text: &str, prefix: &str) -> Option<String> {
    let start = text.find(prefix)?;
    let rest = &text[start + prefix.len()..];
    let n = rest
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric())
        .count();
    Some(format!("{prefix}{}", &rest[..n]))
}
