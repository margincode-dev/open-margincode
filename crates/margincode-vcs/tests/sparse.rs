use std::path::Path;
use std::process::Command;

use margincode_core::{ChangeId, Vcs};
use margincode_vcs::JjVcs;
use tempfile::TempDir;

fn git(dir: &Path, args: &[&str]) -> Vec<u8> {
    let out = Command::new("git")
        .current_dir(dir)
        .args([
            "-c",
            "user.name=t",
            "-c",
            "user.email=t@t",
            "-c",
            "commit.gpgsign=false",
        ])
        .args(args)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    out.stdout
}

fn write(root: &Path, rel: &str, body: &str) {
    let p = root.join(rel);
    std::fs::create_dir_all(p.parent().unwrap()).unwrap();
    std::fs::write(p, body).unwrap();
}

struct Env {
    src: TempDir,
    _store: TempDir,
    _ws: TempDir,
    vcs: JjVcs,
}

const LIST: &str = "# kept out of workspaces\nbig/\nbench/results/\n";

/// A repo that lists `big/` and `bench/results/` in `.av/sparse`.
async fn env() -> Env {
    env_with(true).await
}

async fn env_with(list: bool) -> Env {
    let src = TempDir::new().unwrap();
    for (p, c) in [
        ("a.txt", "a\n"),
        ("big/x.txt", "x\n"),
        ("bench/keep.txt", "k\n"),
        ("bench/results/r.txt", "r\n"),
    ] {
        write(src.path(), p, c);
    }
    if list {
        write(src.path(), ".av/sparse", LIST);
    }
    git(src.path(), &["init", "-q", "-b", "main"]);
    git(src.path(), &["add", "-A"]);
    git(src.path(), &["commit", "-q", "-m", "init"]);
    let store = TempDir::new().unwrap();
    let ws = TempDir::new().unwrap();
    let vcs = JjVcs::new(store.path().to_path_buf(), ws.path().to_path_buf());
    vcs.init_repo("demo", src.path()).await.unwrap();
    Env {
        src,
        _store: store,
        _ws: ws,
        vcs,
    }
}

async fn files(e: &Env, head: &ChangeId) -> Vec<String> {
    e.vcs.list_files("demo", head).await.unwrap()
}

async fn read(e: &Env, head: &ChangeId, path: &str) -> String {
    String::from_utf8(e.vcs.read_file("demo", head, path).await.unwrap()).unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn listed_paths_stay_off_disk_and_in_every_snapshot() {
    let e = env().await;
    let w = e.vcs.create_workspace("demo", "t").await.unwrap();
    assert!(!w.path.join("big").exists());
    assert!(!w.path.join("bench/results").exists());
    assert!(w.path.join("bench/keep.txt").exists());
    assert!(w.path.join(".av/sparse").exists());
    std::fs::write(w.path.join("a.txt"), "a2\n").unwrap();
    let head = e.vcs.snapshot("demo", "t").await.unwrap();
    assert!(files(&e, &head).await.contains(&"big/x.txt".to_owned()));
    assert!(
        files(&e, &head)
            .await
            .contains(&"bench/results/r.txt".to_owned())
    );
    let changed = e
        .vcs
        .changed_paths("demo", &w.base_change, &head)
        .await
        .unwrap();
    assert_eq!(changed, ["a.txt"]);
}

#[tokio::test(flavor = "multi_thread")]
async fn a_file_written_anywhere_is_snapshotted() {
    let e = env().await;
    let w = e.vcs.create_workspace("demo", "t").await.unwrap();
    for p in [
        "top.txt",
        "newdir/f.txt",
        "bench/new.txt",
        "big/new.txt",
        "bench/results/mine/README.md",
    ] {
        write(&w.path, p, "n\n");
    }
    write(&w.path, "big/x.txt", "agent\n");
    let head = e.vcs.snapshot("demo", "t").await.unwrap();
    let changed = e
        .vcs
        .changed_paths("demo", &w.base_change, &head)
        .await
        .unwrap();
    assert_eq!(
        changed,
        [
            "bench/new.txt",
            "bench/results/mine/README.md",
            "big/new.txt",
            "big/x.txt",
            "newdir/f.txt",
            "top.txt"
        ]
    );
    assert_eq!(read(&e, &head, "big/x.txt").await, "agent\n");
    assert_eq!(read(&e, &head, "bench/results/r.txt").await, "r\n");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_sync_writes_what_main_added_except_under_the_list() {
    let e = env().await;
    let t = e.vcs.create_workspace("demo", "t").await.unwrap();
    e.vcs.create_workspace("demo", "u").await.unwrap();
    let u_dir = t.path.parent().unwrap().join("u");
    write(&u_dir, "added.txt", "added\n");
    write(&u_dir, "big/y.txt", "y\n");
    let u = e.vcs.snapshot("demo", "u").await.unwrap();
    e.vcs.merge_task("demo", "u", &u, "merge u").await.unwrap();
    let out = e.vcs.rebase_workspace("demo", "t").await.unwrap();
    assert!(t.path.join("added.txt").exists());
    assert!(!t.path.join("big").exists());
    assert!(files(&e, &out.head).await.contains(&"big/y.txt".to_owned()));
}

#[tokio::test(flavor = "multi_thread")]
async fn a_conflict_under_the_list_is_written_for_the_agent() {
    let e = env().await;
    let t = e.vcs.create_workspace("demo", "t").await.unwrap();
    e.vcs.create_workspace("demo", "u").await.unwrap();
    let u_dir = t.path.parent().unwrap().join("u");
    write(&u_dir, "big/x.txt", "from u\n");
    let u = e.vcs.snapshot("demo", "u").await.unwrap();
    e.vcs.merge_task("demo", "u", &u, "merge u").await.unwrap();
    write(&t.path, "big/x.txt", "from t\n");
    e.vcs.snapshot("demo", "t").await.unwrap();
    let out = e.vcs.rebase_workspace("demo", "t").await.unwrap();
    assert_eq!(out.conflicts, ["big/x.txt"]);
    let on_disk = std::fs::read_to_string(t.path.join("big/x.txt")).unwrap();
    assert!(
        on_disk.contains("from u") && on_disk.contains("from t"),
        "{on_disk}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn a_restored_workspace_leaves_the_list_off_disk() {
    let e = env().await;
    let t = e.vcs.create_workspace("demo", "t").await.unwrap();
    write(&t.path, "a.txt", "mine\n");
    let head = e.vcs.snapshot("demo", "t").await.unwrap();
    e.vcs.remove_workspace("demo", "t").await.unwrap();
    let r = e.vcs.restore_workspace("demo", "t", &head).await.unwrap();
    assert_eq!(
        std::fs::read_to_string(r.path.join("a.txt")).unwrap(),
        "mine\n"
    );
    assert!(!r.path.join("big").exists());
}

#[tokio::test(flavor = "multi_thread")]
async fn a_patch_from_a_full_checkout_edits_a_listed_file() {
    let e = env().await;
    e.vcs.create_workspace("demo", "t").await.unwrap();
    write(e.src.path(), "big/x.txt", "edited elsewhere\n");
    let patch = git(
        e.src.path(),
        &[
            "diff",
            "--binary",
            "--full-index",
            "--no-renames",
            "--src-prefix=a/",
            "--dst-prefix=b/",
        ],
    );
    let applied = e.vcs.apply_patch("demo", "t", &patch).await.unwrap();
    assert_eq!(applied, ["big/x.txt"]);
    let head = e.vcs.snapshot("demo", "t").await.unwrap();
    assert_eq!(read(&e, &head, "big/x.txt").await, "edited elsewhere\n");
}

#[tokio::test(flavor = "multi_thread")]
async fn without_the_list_every_file_is_on_disk() {
    let e = env().await;
    write(e.src.path(), ".av/sparse", "# nothing left out\n");
    git(e.src.path(), &["commit", "-qam", "empty list"]);
    let store = TempDir::new().unwrap();
    let ws = TempDir::new().unwrap();
    let vcs = JjVcs::new(store.path().to_path_buf(), ws.path().to_path_buf());
    vcs.init_repo("demo", e.src.path()).await.unwrap();
    let w = vcs.create_workspace("demo", "t").await.unwrap();
    assert!(w.path.join("big/x.txt").exists());
    assert!(w.path.join("bench/results/r.txt").exists());
}

#[tokio::test(flavor = "multi_thread")]
async fn a_file_written_under_the_list_then_deleted_is_deleted() {
    let e = env().await;
    let t = e.vcs.create_workspace("demo", "t").await.unwrap();
    write(&t.path, "big/scratch.txt", "s\n");
    let head = e.vcs.snapshot("demo", "t").await.unwrap();
    assert!(
        files(&e, &head)
            .await
            .contains(&"big/scratch.txt".to_owned())
    );
    std::fs::remove_file(t.path.join("big/scratch.txt")).unwrap();
    let head = e.vcs.snapshot("demo", "t").await.unwrap();
    assert!(
        !files(&e, &head)
            .await
            .contains(&"big/scratch.txt".to_owned())
    );
    assert!(files(&e, &head).await.contains(&"big/x.txt".to_owned()));
}

#[tokio::test(flavor = "multi_thread")]
async fn a_patch_from_a_full_checkout_deletes_a_listed_file() {
    let e = env().await;
    e.vcs.create_workspace("demo", "t").await.unwrap();
    git(e.src.path(), &["rm", "-q", "big/x.txt"]);
    let patch = git(
        e.src.path(),
        &[
            "diff",
            "--cached",
            "--binary",
            "--full-index",
            "--no-renames",
            "--src-prefix=a/",
            "--dst-prefix=b/",
        ],
    );
    let applied = e.vcs.apply_patch("demo", "t", &patch).await.unwrap();
    assert_eq!(applied, ["big/x.txt"]);
    let head = e.vcs.snapshot("demo", "t").await.unwrap();
    assert!(!files(&e, &head).await.contains(&"big/x.txt".to_owned()));
    assert!(
        files(&e, &head)
            .await
            .contains(&"bench/results/r.txt".to_owned())
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn an_open_workspace_keeps_its_files_when_main_gains_the_list() {
    let e = env_with(false).await;
    let t = e.vcs.create_workspace("demo", "t").await.unwrap();
    e.vcs.create_workspace("demo", "u").await.unwrap();
    let u_dir = t.path.parent().unwrap().join("u");
    write(&u_dir, ".av/sparse", LIST);
    let u = e.vcs.snapshot("demo", "u").await.unwrap();
    e.vcs.merge_task("demo", "u", &u, "merge u").await.unwrap();
    e.vcs.rebase_workspace("demo", "t").await.unwrap();
    assert!(t.path.join(".av/sparse").exists());
    assert!(t.path.join("big/x.txt").exists());
    assert!(t.path.join("bench/results/r.txt").exists());
    std::fs::remove_file(t.path.join("big/x.txt")).unwrap();
    write(&t.path, "bench/results/new.txt", "n\n");
    let head = e.vcs.snapshot("demo", "t").await.unwrap();
    let base = e.vcs.trunk("demo").await.unwrap();
    let changed = e.vcs.changed_paths("demo", &base, &head).await.unwrap();
    assert_eq!(changed, ["bench/results/new.txt", "big/x.txt"]);
    assert!(!files(&e, &head).await.contains(&"big/x.txt".to_owned()));
    let fresh = e.vcs.create_workspace("demo", "v").await.unwrap();
    assert!(!fresh.path.join("big").exists());
}
