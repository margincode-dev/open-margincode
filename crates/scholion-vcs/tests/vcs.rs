use std::path::Path;
use std::process::Command;

use scholion_core::{ChangeId, Vcs};
use scholion_vcs::JjVcs;
use tempfile::TempDir;

fn git(dir: &Path, args: &[&str]) {
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
}

/// Fixture git repo: a.py, pkg/b.py, .gitignore (ignoring node_modules/); node_modules/x.js is untracked.
fn fixture_git_repo() -> TempDir {
    let d = TempDir::new().unwrap();
    git(d.path(), &["init", "-q", "-b", "main"]);
    std::fs::write(d.path().join("a.py"), "def a():\n    return 1\n").unwrap();
    std::fs::create_dir_all(d.path().join("pkg")).unwrap();
    std::fs::write(d.path().join("pkg/b.py"), "def b():\n    return 2\n").unwrap();
    std::fs::write(d.path().join(".gitignore"), "node_modules/\n").unwrap();
    std::fs::create_dir_all(d.path().join("node_modules")).unwrap();
    std::fs::write(d.path().join("node_modules/x.js"), "x").unwrap();
    git(d.path(), &["add", "-A"]);
    git(d.path(), &["commit", "-q", "-m", "init"]);
    d
}

struct Env {
    _src: TempDir,
    _store: TempDir,
    _ws: TempDir,
    vcs: JjVcs,
    main: ChangeId,
}

async fn env() -> Env {
    let src = fixture_git_repo();
    let store = TempDir::new().unwrap();
    let ws = TempDir::new().unwrap();
    let vcs = JjVcs::new(store.path().to_path_buf(), ws.path().to_path_buf());
    let main = vcs.init_repo("demo", src.path()).await.unwrap();
    Env {
        _src: src,
        _store: store,
        _ws: ws,
        vcs,
        main,
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn init_repo_imports_git_source_and_lists_files() {
    let e = env().await;
    let mut files = e.vcs.list_files("demo", &e.main).await.unwrap();
    files.sort();
    assert_eq!(files, vec![".gitignore", "a.py", "pkg/b.py"]);
}

#[tokio::test(flavor = "multi_thread")]
async fn create_workspace_makes_isolated_dir_per_slug() {
    let e = env().await;
    let w1 = e.vcs.create_workspace("demo", "one").await.unwrap();
    let w2 = e.vcs.create_workspace("demo", "two").await.unwrap();
    assert_ne!(w1.path, w2.path);
    assert_eq!(w1.base_change, e.main);
    assert_eq!(
        std::fs::read_to_string(w1.path.join("a.py")).unwrap(),
        "def a():\n    return 1\n"
    );
    std::fs::write(w1.path.join("only_in_one.txt"), "hi").unwrap();
    assert!(!w2.path.join("only_in_one.txt").exists());
}

#[tokio::test(flavor = "multi_thread")]
async fn snapshot_captures_edits_and_changed_paths_lists_them() {
    let e = env().await;
    let w = e.vcs.create_workspace("demo", "t").await.unwrap();
    std::fs::write(w.path.join("a.py"), "def a():\n    return 42\n").unwrap();
    std::fs::write(w.path.join("b.py"), "def b2(): pass\n").unwrap();
    std::fs::create_dir_all(w.path.join("node_modules")).unwrap();
    std::fs::write(w.path.join("node_modules/y.js"), "ignored").unwrap();
    let head = e.vcs.snapshot("demo", "t").await.unwrap();

    let mut changed = e
        .vcs
        .changed_paths("demo", &w.base_change, &head)
        .await
        .unwrap();
    changed.sort();
    assert_eq!(changed, vec!["a.py", "b.py"]); // node_modules/y.js is gitignored
    assert_eq!(
        e.vcs.read_file("demo", &head, "a.py").await.unwrap(),
        b"def a():\n    return 42\n"
    );
    assert_eq!(
        e.vcs
            .read_file("demo", &w.base_change, "a.py")
            .await
            .unwrap(),
        b"def a():\n    return 1\n"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn snapshot_without_edits_keeps_changed_paths_empty() {
    let e = env().await;
    let w = e.vcs.create_workspace("demo", "idle").await.unwrap();
    let head = e.vcs.snapshot("demo", "idle").await.unwrap();
    assert!(
        e.vcs
            .changed_paths("demo", &w.base_change, &head)
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn concurrent_snapshots_on_same_repo_do_not_corrupt() {
    let e = env().await;
    let slugs = ["w0", "w1", "w2", "w3"];
    let mut paths = vec![];
    for s in slugs {
        paths.push(e.vcs.create_workspace("demo", s).await.unwrap().path);
    }
    for (i, p) in paths.iter().enumerate() {
        std::fs::write(p.join(format!("f{i}.txt")), format!("{i}")).unwrap();
    }
    let vcs = &e.vcs;
    let results = futures::future::join_all(
        slugs
            .iter()
            .chain(slugs.iter())
            .map(|s| async move { vcs.snapshot("demo", s).await }),
    )
    .await;
    assert_eq!(results.len(), 8);
    for (i, r) in results.iter().enumerate() {
        let head = r.as_ref().expect("snapshot failed");
        let slot = i % 4;
        let got = e
            .vcs
            .read_file("demo", head, &format!("f{slot}.txt"))
            .await
            .unwrap();
        assert_eq!(got, format!("{slot}").into_bytes());
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn non_utf8_and_binary_files_snapshot_fine() {
    let e = env().await;
    let w = e.vcs.create_workspace("demo", "bin").await.unwrap();
    let bytes: Vec<u8> = vec![0xff, 0xfe, 0x00, 0x01, 0x80, 0xc3, 0x28];
    std::fs::write(w.path.join("blob.bin"), &bytes).unwrap();
    let head = e.vcs.snapshot("demo", "bin").await.unwrap();
    assert_eq!(
        e.vcs.read_file("demo", &head, "blob.bin").await.unwrap(),
        bytes
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn agent_metadata_file_is_never_snapshotted() {
    let e = env().await;
    let w = e.vcs.create_workspace("demo", "meta").await.unwrap();
    std::fs::write(w.path.join(".av-task"), "{\"task\":\"x\"}").unwrap();
    std::fs::write(w.path.join("real.txt"), "tracked").unwrap();
    let head = e.vcs.snapshot("demo", "meta").await.unwrap();
    assert_eq!(
        e.vcs
            .changed_paths("demo", &w.base_change, &head)
            .await
            .unwrap(),
        vec!["real.txt"]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn unsafe_repo_and_slug_names_are_rejected_without_touching_the_filesystem() {
    let e = env().await;
    let outside = e._store.path().parent().unwrap().to_path_buf();
    for bad in [
        "../escape",
        "a/b",
        "/abs",
        "",
        ".hidden",
        "..",
        "x\\y",
        "with\0nul",
    ] {
        let r = e.vcs.init_repo(bad, e._src.path()).await;
        assert!(
            matches!(r, Err(scholion_core::AvError::Invalid(_))),
            "repo name {bad:?}: {r:?}"
        );
        let r = e.vcs.create_workspace(bad, "ok").await;
        assert!(
            matches!(r, Err(scholion_core::AvError::Invalid(_))),
            "repo name {bad:?}: {r:?}"
        );
        let r = e.vcs.create_workspace("demo", bad).await;
        assert!(
            matches!(r, Err(scholion_core::AvError::Invalid(_))),
            "slug {bad:?}: {r:?}"
        );
        let r = e.vcs.snapshot("demo", bad).await;
        assert!(
            matches!(r, Err(scholion_core::AvError::Invalid(_))),
            "slug {bad:?}: {r:?}"
        );
    }
    assert!(
        !outside.join("escape").exists(),
        "nothing was created outside the store"
    );
    // unicode names are fine (task slugs come from titles, which may be Vietnamese)
    assert!(
        e.vcs
            .create_workspace("demo", "cải-thiện-lời-chào")
            .await
            .is_ok()
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn git_source_must_be_an_existing_local_directory_never_an_option() {
    let e = env().await;
    let marker = e._store.path().join("pwned");
    for bad in [
        format!("--upload-pack=touch {}", marker.display()),
        "--bare".to_string(),
        "https://example.com/x.git".to_string(),
        "ext::sh -c touch pwned".to_string(),
        "/definitely/not/here".to_string(),
    ] {
        let r = e.vcs.init_repo("other", Path::new(&bad)).await;
        assert!(
            matches!(r, Err(scholion_core::AvError::Invalid(_))),
            "source {bad:?}: {r:?}"
        );
    }
    assert!(!marker.exists());
}

#[tokio::test(flavor = "multi_thread")]
async fn failed_init_leaves_nothing_behind_so_a_retry_works() {
    let e = env().await;
    let not_a_repo = TempDir::new().unwrap();
    let r = e.vcs.init_repo("retry", not_a_repo.path()).await;
    assert!(r.is_err(), "an empty directory is not a git repository");
    assert!(
        !e._store.path().join("retry").exists(),
        "half-initialised repo directory was cleaned up"
    );
    assert!(
        e.vcs.init_repo("retry", e._src.path()).await.is_ok(),
        "retry with a real repository works"
    );
}
