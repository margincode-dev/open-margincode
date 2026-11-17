use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, mpsc};
use std::time::Duration;

use margincode_core::{ChangeId, Vcs};
use margincode_vcs::JjVcs;
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

/// A write hook that, after letting `skip` writes of workspace `slug` through, holds the next one
/// until the test sends on `release`; `entered` hears when it is held.
fn hold(
    slug: &'static str,
    skip: usize,
) -> (
    mpsc::Receiver<()>,
    mpsc::Sender<()>,
    impl Fn(&str) + Send + Sync + 'static,
) {
    let (entered_tx, entered) = mpsc::channel();
    let (release, release_rx) = mpsc::channel::<()>();
    let release_rx = Mutex::new(release_rx);
    let seen = AtomicUsize::new(0);
    let hook = move |s: &str| {
        if s == slug && seen.fetch_add(1, Ordering::SeqCst) == skip {
            entered_tx.send(()).unwrap();
            let _ = release_rx
                .lock()
                .unwrap()
                .recv_timeout(Duration::from_secs(120));
        }
    };
    (entered, release, hook)
}

/// A `JjVcs` with `hook` on `src` as repo `demo`, and its store and workspace directories.
async fn held_env(
    src: &TempDir,
    hook: impl Fn(&str) + Send + Sync + 'static,
) -> ([TempDir; 2], JjVcs) {
    let store = TempDir::new().unwrap();
    let ws = TempDir::new().unwrap();
    let vcs = JjVcs::new(store.path().to_path_buf(), ws.path().to_path_buf()).with_write_hook(hook);
    vcs.init_repo("demo", src.path()).await.unwrap();
    ([store, ws], vcs)
}

/// Waits until the held write begins, then snapshots workspace `one` while it is held, then
/// releases it. The timeouts only bound a failure.
async fn snapshot_while_held(
    vcs: &JjVcs,
    entered: mpsc::Receiver<()>,
    release: mpsc::Sender<()>,
) -> ChangeId {
    tokio::task::spawn_blocking(move || entered.recv_timeout(Duration::from_secs(60)))
        .await
        .unwrap()
        .expect("the held write never began");
    let other = tokio::time::timeout(Duration::from_secs(60), vcs.snapshot("demo", "one")).await;
    release.send(()).unwrap();
    other
        .expect("a snapshot of another task waited for the held write")
        .unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn a_workspace_copy_does_not_hold_up_other_tasks_in_the_repo() {
    let src = fixture_git_repo();
    let (entered, release, hook) = hold("late", 0);
    let (_dirs, vcs) = held_env(&src, hook).await;
    let one = vcs.create_workspace("demo", "one").await.unwrap();
    std::fs::write(one.path.join("new.txt"), "n\n").unwrap();
    let late = tokio::spawn({
        let vcs = vcs.clone();
        async move { vcs.create_workspace("demo", "late").await }
    });
    let head = snapshot_while_held(&vcs, entered, release).await;
    assert_eq!(
        vcs.changed_paths("demo", &one.base_change, &head)
            .await
            .unwrap(),
        vec!["new.txt"]
    );
    let late = late.await.unwrap().unwrap();
    assert_eq!(
        std::fs::read_to_string(late.path.join("pkg/b.py")).unwrap(),
        "def b():\n    return 2\n"
    );
    let h = vcs.snapshot("demo", "late").await.unwrap();
    assert!(
        vcs.changed_paths("demo", &late.base_change, &h)
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn a_widen_does_not_hold_up_other_tasks_in_the_repo() {
    let src = fixture_git_repo();
    std::fs::create_dir_all(src.path().join(".av")).unwrap();
    std::fs::write(src.path().join(".av/sparse"), "pkg/\n").unwrap();
    git(src.path(), &["add", "-A"]);
    git(src.path(), &["commit", "-q", "-m", "list"]);
    let (entered, release, hook) = hold("wide", 1);
    let (_dirs, vcs) = held_env(&src, hook).await;
    let one = vcs.create_workspace("demo", "one").await.unwrap();
    let wide = vcs.create_workspace("demo", "wide").await.unwrap();
    assert!(!wide.path.join("pkg/b.py").exists());
    std::fs::write(one.path.join("new.txt"), "n\n").unwrap();
    let widen = tokio::spawn({
        let vcs = vcs.clone();
        async move {
            vcs.widen_workspace("demo", "wide", &["pkg".to_owned()])
                .await
        }
    });
    let head = snapshot_while_held(&vcs, entered, release).await;
    assert_eq!(
        vcs.changed_paths("demo", &one.base_change, &head)
            .await
            .unwrap(),
        vec!["new.txt"]
    );
    assert_eq!(widen.await.unwrap().unwrap(), vec!["pkg/b.py"]);
    assert!(wide.path.join("pkg/b.py").exists());
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
async fn build_output_is_never_snapshotted_even_when_the_repo_does_not_ignore_it() {
    let e = env().await;
    // the fixture's .gitignore names only node_modules
    let w = e.vcs.create_workspace("demo", "built").await.unwrap();
    std::fs::create_dir_all(w.path.join("target/debug/deps")).unwrap();
    std::fs::write(w.path.join("target/debug/deps/big-1234"), vec![0u8; 8192]).unwrap();
    // only a workspace's own top-level target/ is build output
    std::fs::create_dir_all(w.path.join("pkg/target")).unwrap();
    std::fs::write(w.path.join("pkg/target/kept.py"), "x = 1\n").unwrap();
    let head = e.vcs.snapshot("demo", "built").await.unwrap();
    assert_eq!(
        e.vcs
            .changed_paths("demo", &w.base_change, &head)
            .await
            .unwrap(),
        vec!["pkg/target/kept.py"]
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
            matches!(r, Err(margincode_core::AvError::Invalid(_))),
            "repo name {bad:?}: {r:?}"
        );
        let r = e.vcs.create_workspace(bad, "ok").await;
        assert!(
            matches!(r, Err(margincode_core::AvError::Invalid(_))),
            "repo name {bad:?}: {r:?}"
        );
        let r = e.vcs.create_workspace("demo", bad).await;
        assert!(
            matches!(r, Err(margincode_core::AvError::Invalid(_))),
            "slug {bad:?}: {r:?}"
        );
        let r = e.vcs.snapshot("demo", bad).await;
        assert!(
            matches!(r, Err(margincode_core::AvError::Invalid(_))),
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
            matches!(r, Err(margincode_core::AvError::Invalid(_))),
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

#[tokio::test(flavor = "multi_thread")]
async fn list_repos_names_every_imported_repo_sorted() {
    let e = env().await;
    let other = fixture_git_repo();
    e.vcs.init_repo("alpha", other.path()).await.unwrap();
    // a stray directory in the store is not a repo
    std::fs::create_dir_all(e._store.path().join("stray")).unwrap();
    assert_eq!(e.vcs.list_repos().await.unwrap(), vec!["alpha", "demo"]);
}

#[tokio::test(flavor = "multi_thread")]
async fn remove_workspace_deletes_the_dir_and_forgets_it() {
    let e = env().await;
    let w = e.vcs.create_workspace("demo", "gone").await.unwrap();
    std::fs::write(w.path.join("a.py"), "def a():\n    return 7\n").unwrap();
    e.vcs.snapshot("demo", "gone").await.unwrap();

    assert!(e.vcs.remove_workspace("demo", "gone").await.unwrap());
    assert!(!w.path.exists());
    let r = e.vcs.snapshot("demo", "gone").await;
    assert!(
        matches!(r, Err(margincode_core::AvError::NotFound(_))),
        "{r:?}"
    );
    // jj forgot the name: a new workspace of the same slug starts fresh from trunk
    let again = e.vcs.create_workspace("demo", "gone").await.unwrap();
    assert_eq!(again.base_change, e.main);
    assert_eq!(
        std::fs::read_to_string(again.path.join("a.py")).unwrap(),
        "def a():\n    return 1\n"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn remove_workspace_twice_is_a_no_op() {
    let e = env().await;
    e.vcs.create_workspace("demo", "twice").await.unwrap();
    assert!(e.vcs.remove_workspace("demo", "twice").await.unwrap());
    assert!(!e.vcs.remove_workspace("demo", "twice").await.unwrap());
    assert!(!e.vcs.remove_workspace("demo", "never-made").await.unwrap());
}

#[tokio::test(flavor = "multi_thread")]
async fn remove_workspace_refuses_the_stores_own_workspace() {
    let e = env().await;
    let r = e.vcs.remove_workspace("demo", "main").await;
    assert!(
        matches!(r, Err(margincode_core::AvError::Invalid(_))),
        "{r:?}"
    );
    let r = e.vcs.remove_workspace("demo", "../escape").await;
    assert!(
        matches!(r, Err(margincode_core::AvError::Invalid(_))),
        "{r:?}"
    );
    assert_eq!(e.vcs.trunk("demo").await.unwrap(), e.main);
}

#[tokio::test(flavor = "multi_thread")]
async fn restore_workspace_brings_back_the_head() {
    let e = env().await;
    let w = e.vcs.create_workspace("demo", "back").await.unwrap();
    std::fs::write(w.path.join("a.py"), "def a():\n    return 9\n").unwrap();
    std::fs::write(w.path.join("new.txt"), "new").unwrap();
    let head = e.vcs.snapshot("demo", "back").await.unwrap();
    assert!(e.vcs.remove_workspace("demo", "back").await.unwrap());

    let r = e
        .vcs
        .restore_workspace("demo", "back", &head)
        .await
        .unwrap();
    assert_eq!(r.path, w.path);
    assert_eq!(r.base_change, w.base_change, "a task head's first parent");
    assert_eq!(
        std::fs::read_to_string(r.path.join("a.py")).unwrap(),
        "def a():\n    return 9\n"
    );
    assert_eq!(
        std::fs::read_to_string(r.path.join("new.txt")).unwrap(),
        "new"
    );
    // the working copy is the head itself: nothing changed, same head
    assert_eq!(e.vcs.snapshot("demo", "back").await.unwrap(), head);
    std::fs::write(r.path.join("more.txt"), "more").unwrap();
    let next = e.vcs.snapshot("demo", "back").await.unwrap();
    assert_ne!(next, head);
    let mut changed = e
        .vcs
        .changed_paths("demo", &w.base_change, &next)
        .await
        .unwrap();
    changed.sort();
    assert_eq!(changed, vec!["a.py", "more.txt", "new.txt"]);

    let again = e.vcs.restore_workspace("demo", "back", &head).await;
    assert!(
        matches!(again, Err(margincode_core::AvError::Invalid(_))),
        "the dir exists: {again:?}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn restore_workspace_after_the_dir_was_deleted_by_hand() {
    let e = env().await;
    let w = e.vcs.create_workspace("demo", "rmrf").await.unwrap();
    std::fs::write(w.path.join("a.py"), "def a():\n    return 5\n").unwrap();
    let head = e.vcs.snapshot("demo", "rmrf").await.unwrap();
    std::fs::remove_dir_all(&w.path).unwrap();

    let r = e
        .vcs
        .restore_workspace("demo", "rmrf", &head)
        .await
        .unwrap();
    assert_eq!(
        std::fs::read_to_string(r.path.join("a.py")).unwrap(),
        "def a():\n    return 5\n"
    );
    assert_eq!(e.vcs.snapshot("demo", "rmrf").await.unwrap(), head);
}

#[tokio::test(flavor = "multi_thread")]
async fn restore_workspace_works_for_an_empty_head_jj_abandoned() {
    let e = env().await;
    let w = e.vcs.create_workspace("demo", "empty").await.unwrap();
    let head = e.vcs.snapshot("demo", "empty").await.unwrap();
    // forgetting an empty working-copy commit abandons it in jj; it is still in the store
    assert!(e.vcs.remove_workspace("demo", "empty").await.unwrap());
    let r = e
        .vcs
        .restore_workspace("demo", "empty", &head)
        .await
        .unwrap();
    assert_eq!(r.base_change, w.base_change);
    assert_eq!(e.vcs.snapshot("demo", "empty").await.unwrap(), head);
}

fn git_out(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .current_dir(dir)
        .args(["-c", "core.autocrlf=false"])
        .args(args)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

fn text_blob(s: &str) -> margincode_vcs::patch::Blob {
    margincode_vcs::patch::Blob {
        mode: margincode_vcs::patch::FILE,
        bytes: s.as_bytes().to_vec(),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn apply_patch_writes_the_workspace_and_a_snapshot_records_it() {
    use margincode_vcs::patch::{Blob, EXECUTABLE, FileDiff, render};
    let e = env().await;
    let w = e.vcs.create_workspace("demo", "remote").await.unwrap();
    let p = render(&[
        FileDiff {
            path: "a.py".into(),
            before: Some(text_blob("def a():\n    return 1\n")),
            after: Some(text_blob("def a():\n    return 42\n")),
        },
        FileDiff {
            path: "pkg/b.py".into(),
            before: Some(text_blob("def b():\n    return 2\n")),
            after: None,
        },
        FileDiff {
            path: "tools/run.sh".into(),
            before: None,
            after: Some(Blob {
                mode: EXECUTABLE,
                bytes: b"#!/bin/sh\necho hi\n".to_vec(),
            }),
        },
    ]);
    let mut touched = e.vcs.apply_patch("demo", "remote", &p).await.unwrap();
    touched.sort();
    assert_eq!(touched, vec!["a.py", "pkg/b.py", "tools/run.sh"]);
    assert!(
        !w.path.join("pkg").exists(),
        "an emptied directory goes too"
    );
    let head = e.vcs.snapshot("demo", "remote").await.unwrap();
    assert_eq!(
        e.vcs.read_file("demo", &head, "a.py").await.unwrap(),
        b"def a():\n    return 42\n"
    );
    assert_eq!(
        e.vcs
            .changed_paths("demo", &w.base_change, &head)
            .await
            .unwrap(),
        vec!["a.py", "pkg/b.py", "tools/run.sh"]
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let m = std::fs::metadata(w.path.join("tools/run.sh")).unwrap();
        assert_eq!(m.permissions().mode() & 0o111, 0o111);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn apply_patch_applies_whole_or_not_at_all() {
    use margincode_vcs::patch::{FileDiff, render};
    let e = env().await;
    let w = e.vcs.create_workspace("demo", "whole").await.unwrap();
    let p = render(&[
        FileDiff {
            path: "a.py".into(),
            before: Some(text_blob("def a():\n    return 1\n")),
            after: Some(text_blob("def a():\n    return 7\n")),
        },
        FileDiff {
            // not what the workspace holds
            path: "pkg/b.py".into(),
            before: Some(text_blob("def b():\n    return 99\n")),
            after: Some(text_blob("def b():\n    return 3\n")),
        },
    ]);
    let r = e.vcs.apply_patch("demo", "whole", &p).await;
    assert!(
        matches!(r, Err(margincode_core::AvError::Invalid(_))),
        "{r:?}"
    );
    assert_eq!(
        std::fs::read_to_string(w.path.join("a.py")).unwrap(),
        "def a():\n    return 1\n",
        "nothing was written"
    );
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread")]
async fn apply_patch_never_writes_through_a_symlink() {
    use margincode_vcs::patch::{FileDiff, render};
    let e = env().await;
    let w = e.vcs.create_workspace("demo", "link").await.unwrap();
    let outside = TempDir::new().unwrap();
    std::os::unix::fs::symlink(outside.path(), w.path.join("out")).unwrap();
    std::os::unix::fs::symlink(w.path.join("a.py"), w.path.join("a-link.py")).unwrap();
    for path in ["out/evil.txt", "a-link.py"] {
        let before = (path == "a-link.py").then(|| text_blob("def a():\n    return 1\n"));
        let p = render(&[FileDiff {
            path: path.into(),
            before,
            after: Some(text_blob("pwned\n")),
        }]);
        let r = e.vcs.apply_patch("demo", "link", &p).await;
        assert!(
            matches!(r, Err(margincode_core::AvError::Invalid(_))),
            "{path}: {r:?}"
        );
    }
    assert_eq!(std::fs::read_dir(outside.path()).unwrap().count(), 0);
    assert_eq!(
        std::fs::read_to_string(w.path.join("a.py")).unwrap(),
        "def a():\n    return 1\n"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn diff_patch_from_nothing_rebuilds_the_tree_git_has() {
    let e = env().await;
    let p = e.vcs.diff_patch("demo", None, &e.main).await.unwrap();
    let dir = TempDir::new().unwrap();
    git_out(dir.path(), &["init", "-q"]);
    let file = dir.path().join(".git/p.diff");
    std::fs::write(&file, &p).unwrap();
    git_out(dir.path(), &["apply", "--cached", file.to_str().unwrap()]);
    let tree = git_out(dir.path(), &["write-tree"]);
    assert_eq!(tree, git_out(e._src.path(), &["rev-parse", "HEAD^{tree}"]));
    assert_eq!(e.vcs.tree_id("demo", &e.main).await.unwrap(), tree);
}

#[tokio::test(flavor = "multi_thread")]
async fn diff_patch_between_two_heads_applies_with_git() {
    let e = env().await;
    let w = e.vcs.create_workspace("demo", "two").await.unwrap();
    std::fs::write(w.path.join("a.py"), "def a():\n    return 2\n").unwrap();
    let h1 = e.vcs.snapshot("demo", "two").await.unwrap();
    std::fs::write(w.path.join("a.py"), "def a():\n    return 3\n").unwrap();
    std::fs::write(w.path.join("img.bin"), [0u8, 1, 2, 255, 0, 7]).unwrap();
    std::fs::remove_file(w.path.join("pkg/b.py")).unwrap();
    let h2 = e.vcs.snapshot("demo", "two").await.unwrap();

    let dir = TempDir::new().unwrap();
    git_out(dir.path(), &["init", "-q"]);
    let file = dir.path().join(".git/p.diff");
    std::fs::write(&file, e.vcs.diff_patch("demo", None, &h1).await.unwrap()).unwrap();
    git_out(dir.path(), &["apply", "--index", file.to_str().unwrap()]);
    assert_eq!(
        git_out(dir.path(), &["write-tree"]),
        e.vcs.tree_id("demo", &h1).await.unwrap()
    );
    let step = e.vcs.diff_patch("demo", Some(&h1), &h2).await.unwrap();
    std::fs::write(&file, &step).unwrap();
    git_out(dir.path(), &["apply", "--index", file.to_str().unwrap()]);
    assert_eq!(
        git_out(dir.path(), &["write-tree"]),
        e.vcs.tree_id("demo", &h2).await.unwrap()
    );
    assert!(
        e.vcs
            .diff_patch("demo", Some(&h2), &h2)
            .await
            .unwrap()
            .is_empty()
    );
}

/// A repo holding a plain `a.py` and an executable `run.sh`, imported with `exec_bit`.
#[cfg(unix)]
async fn exec_env(exec_bit: margincode_vcs::ExecBit) -> (Env, std::path::PathBuf) {
    let src = TempDir::new().unwrap();
    git(src.path(), &["init", "-q", "-b", "main"]);
    std::fs::write(src.path().join("a.py"), "def a():\n    return 1\n").unwrap();
    std::fs::write(src.path().join("run.sh"), "#!/bin/sh\necho hi\n").unwrap();
    git(src.path(), &["add", "-A"]);
    git(src.path(), &["update-index", "--chmod=+x", "run.sh"]);
    git(src.path(), &["commit", "-q", "-m", "init"]);
    let store = TempDir::new().unwrap();
    let ws = TempDir::new().unwrap();
    let vcs =
        JjVcs::new(store.path().to_path_buf(), ws.path().to_path_buf()).with_exec_bit(exec_bit);
    let main = vcs.init_repo("demo", src.path()).await.unwrap();
    let w = vcs.create_workspace("demo", "modes").await.unwrap();
    let e = Env {
        _src: src,
        _store: store,
        _ws: ws,
        vcs,
        main,
    };
    (e, w.path)
}

#[cfg(unix)]
fn chmod(path: &Path, mode: u32) {
    use std::os::unix::fs::PermissionsExt as _;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode)).unwrap();
}

/// What a Windows-created file looks like on a Docker Desktop bind mount: every file reads 777.
#[cfg(unix)]
#[tokio::test(flavor = "multi_thread")]
async fn ignored_exec_bit_keeps_each_files_mode_from_the_repo() {
    let (e, dir) = exec_env(margincode_vcs::ExecBit::Ignore).await;
    std::fs::write(dir.join("a.py"), "def a():\n    return 2\n").unwrap();
    chmod(&dir.join("a.py"), 0o777);
    std::fs::write(dir.join("run.sh"), "#!/bin/sh\necho bye\n").unwrap();
    chmod(&dir.join("run.sh"), 0o644);
    std::fs::write(dir.join("new.py"), "x = 1\n").unwrap();
    chmod(&dir.join("new.py"), 0o777);
    let head = e.vcs.snapshot("demo", "modes").await.unwrap();
    let diff = String::from_utf8(
        e.vcs
            .diff_patch("demo", Some(&e.main), &head)
            .await
            .unwrap(),
    )
    .unwrap();
    assert!(!diff.contains("old mode"), "{diff}");
    assert!(diff.contains("new file mode 100644"), "{diff}");
    assert!(diff.contains(" 100755\n--- a/run.sh"), "{diff}");
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread")]
async fn exec_bit_changes_are_recorded_by_default() {
    let (e, dir) = exec_env(margincode_vcs::ExecBit::default()).await;
    chmod(&dir.join("a.py"), 0o755);
    std::fs::write(dir.join("new.sh"), "#!/bin/sh\n").unwrap();
    chmod(&dir.join("new.sh"), 0o755);
    let head = e.vcs.snapshot("demo", "modes").await.unwrap();
    let diff = String::from_utf8(
        e.vcs
            .diff_patch("demo", Some(&e.main), &head)
            .await
            .unwrap(),
    )
    .unwrap();
    assert!(diff.contains("old mode 100644\nnew mode 100755"), "{diff}");
    assert!(diff.contains("new file mode 100755"), "{diff}");
}

#[test]
fn exec_bit_parses_the_three_settings() {
    use margincode_vcs::ExecBit;
    assert_eq!("auto".parse::<ExecBit>().unwrap(), ExecBit::Auto);
    assert_eq!(" Ignore ".parse::<ExecBit>().unwrap(), ExecBit::Ignore);
    assert_eq!("respect".parse::<ExecBit>().unwrap(), ExecBit::Respect);
    assert!("off".parse::<ExecBit>().is_err());
    assert_eq!(ExecBit::default(), ExecBit::Auto);
}
