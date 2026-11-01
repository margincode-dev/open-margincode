use std::path::Path;
use std::process::Command;

use scholion_core::{AvError, ChangeId, MergeOutcome, Vcs};
use scholion_vcs::JjVcs;
use tempfile::TempDir;

fn git(dir: &Path, args: &[&str]) -> String {
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
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

/// Fixture: a.py (three lines) and pkg/b.py.
fn fixture_git_repo() -> TempDir {
    let d = TempDir::new().unwrap();
    git(d.path(), &["init", "-q", "-b", "main"]);
    std::fs::write(d.path().join("a.py"), "one = 1\ntwo = 2\nthree = 3\n").unwrap();
    std::fs::create_dir_all(d.path().join("pkg")).unwrap();
    std::fs::write(d.path().join("pkg/b.py"), "def b():\n    return 2\n").unwrap();
    git(d.path(), &["add", "-A"]);
    git(d.path(), &["commit", "-q", "-m", "init"]);
    d
}

struct Env {
    _src: TempDir,
    store: TempDir,
    ws: TempDir,
    vcs: JjVcs,
    main: ChangeId,
}

impl Env {
    fn write(&self, slug: &str, path: &str, text: &str) {
        let p = self.ws.path().join("demo").join(slug).join(path);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, text).unwrap();
    }
    fn read(&self, slug: &str, path: &str) -> String {
        std::fs::read_to_string(self.ws.path().join("demo").join(slug).join(path)).unwrap()
    }
    async fn main_file(&self, path: &str) -> String {
        let trunk = self.vcs.trunk("demo").await.unwrap();
        String::from_utf8(self.vcs.read_file("demo", &trunk, path).await.unwrap()).unwrap()
    }
    fn git_ref(&self) -> String {
        git(
            &self.store.path().join("demo/git"),
            &["rev-parse", "refs/heads/main"],
        )
    }
    /// Starts a workspace, writes `files`, snapshots, and returns the head.
    async fn task(&self, slug: &str, files: &[(&str, &str)]) -> ChangeId {
        self.vcs.create_workspace("demo", slug).await.unwrap();
        for (path, text) in files {
            self.write(slug, path, text);
        }
        self.vcs.snapshot("demo", slug).await.unwrap()
    }
    async fn merge(&self, slug: &str, head: &ChangeId) -> MergeOutcome {
        self.vcs
            .merge_task("demo", slug, head, &format!("merge {slug}"))
            .await
            .unwrap()
    }
}

async fn env() -> Env {
    let src = fixture_git_repo();
    let store = TempDir::new().unwrap();
    let ws = TempDir::new().unwrap();
    let vcs = JjVcs::new(store.path().to_path_buf(), ws.path().to_path_buf());
    let main = vcs.init_repo("demo", src.path()).await.unwrap();
    Env {
        _src: src,
        store,
        ws,
        vcs,
        main,
    }
}

fn merged(outcome: MergeOutcome) -> (ChangeId, ChangeId) {
    match outcome {
        MergeOutcome::Merged { pre_merge, main } => (pre_merge, main),
        other => panic!("expected a merge, got {other:?}"),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn merge_advances_main_and_exports_git_ref() {
    let e = env().await;
    let head = e.task("t1", &[("c.py", "c = 3\n")]).await;
    let (pre, main) = merged(e.merge("t1", &head).await);
    assert_eq!(pre, e.main);
    assert_eq!(e.vcs.trunk("demo").await.unwrap(), main);
    assert_eq!(e.git_ref(), main.0);
    assert_eq!(e.main_file("c.py").await, "c = 3\n");
}

#[tokio::test(flavor = "multi_thread")]
async fn disjoint_tasks_both_merge_onto_a_moved_main() {
    let e = env().await;
    let h1 = e
        .task("t1", &[("a.py", "one = 10\ntwo = 2\nthree = 3\n")])
        .await;
    let h2 = e
        .task("t2", &[("pkg/b.py", "def b():\n    return 20\n")])
        .await;
    merged(e.merge("t1", &h1).await);
    merged(e.merge("t2", &h2).await);
    assert_eq!(e.main_file("a.py").await, "one = 10\ntwo = 2\nthree = 3\n");
    assert_eq!(e.main_file("pkg/b.py").await, "def b():\n    return 20\n");
}

#[tokio::test(flavor = "multi_thread")]
async fn merge_conflict_leaves_main_untouched() {
    let e = env().await;
    let h1 = e
        .task("t1", &[("a.py", "one = 1\ntwo = 22\nthree = 3\n")])
        .await;
    let h2 = e
        .task("t2", &[("a.py", "one = 1\ntwo = 222\nthree = 3\n")])
        .await;
    let (_, after_first) = merged(e.merge("t1", &h1).await);
    match e.merge("t2", &h2).await {
        MergeOutcome::Conflicted { paths } => assert_eq!(paths, vec!["a.py"]),
        other => panic!("expected a conflict, got {other:?}"),
    }
    assert_eq!(e.vcs.trunk("demo").await.unwrap(), after_first);
    assert_eq!(e.git_ref(), after_first.0);
    assert_eq!(e.main_file("a.py").await, "one = 1\ntwo = 22\nthree = 3\n");
}

#[tokio::test(flavor = "multi_thread")]
async fn merge_refuses_when_workspace_changed_after_the_expected_head() {
    let e = env().await;
    let head = e.task("t1", &[("c.py", "c = 3\n")]).await;
    e.write("t1", "c.py", "c = 4\n");
    let err = e
        .vcs
        .merge_task("demo", "t1", &head, "merge")
        .await
        .unwrap_err();
    assert!(matches!(err, AvError::IllegalState(_)), "{err:?}");
    assert_eq!(e.vcs.trunk("demo").await.unwrap(), e.main);
}

#[tokio::test(flavor = "multi_thread")]
async fn rebase_writes_markers_then_a_fix_merges() {
    let e = env().await;
    let h1 = e
        .task("t1", &[("a.py", "one = 1\ntwo = 22\nthree = 3\n")])
        .await;
    e.task("t2", &[("a.py", "one = 1\ntwo = 222\nthree = 3\n")])
        .await;
    merged(e.merge("t1", &h1).await);

    let rebased = e.vcs.rebase_workspace("demo", "t2").await.unwrap();
    assert_eq!(rebased.conflicts, vec!["a.py"]);
    assert_eq!(rebased.base, e.vcs.trunk("demo").await.unwrap());
    assert!(
        e.read("t2", "a.py").contains("<<<<<<<"),
        "{}",
        e.read("t2", "a.py")
    );

    e.write("t2", "a.py", "one = 1\ntwo = 2222\nthree = 3\n");
    let fixed = e.vcs.rebase_workspace("demo", "t2").await.unwrap();
    assert!(fixed.conflicts.is_empty(), "{:?}", fixed.conflicts);
    let (_, main) = merged(e.merge("t2", &fixed.head).await);
    assert_eq!(e.git_ref(), main.0);
    assert_eq!(
        e.main_file("a.py").await,
        "one = 1\ntwo = 2222\nthree = 3\n"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn rebase_without_conflict_brings_in_main() {
    let e = env().await;
    let h1 = e.task("t1", &[("c.py", "c = 3\n")]).await;
    e.task("t2", &[("d.py", "d = 4\n")]).await;
    merged(e.merge("t1", &h1).await);
    let rebased = e.vcs.rebase_workspace("demo", "t2").await.unwrap();
    assert!(rebased.conflicts.is_empty());
    assert_eq!(e.read("t2", "c.py"), "c = 3\n");
    assert_eq!(e.read("t2", "d.py"), "d = 4\n");
    let changed = e
        .vcs
        .changed_paths("demo", &rebased.base, &rebased.head)
        .await
        .unwrap();
    assert_eq!(changed, vec!["d.py"]);
}

#[tokio::test(flavor = "multi_thread")]
async fn undo_restores_main_and_git_ref() {
    let e = env().await;
    let head = e.task("t1", &[("c.py", "c = 3\n")]).await;
    let (pre, main) = merged(e.merge("t1", &head).await);
    e.vcs.undo_merge("demo", &pre, &main).await.unwrap();
    assert_eq!(e.vcs.trunk("demo").await.unwrap(), e.main);
    assert_eq!(e.git_ref(), e.main.0);
    let err = e.vcs.read_file("demo", &e.main, "c.py").await.unwrap_err();
    assert!(matches!(err, AvError::NotFound(_)));
}

#[tokio::test(flavor = "multi_thread")]
async fn undo_is_refused_after_a_later_merge() {
    let e = env().await;
    let h1 = e.task("t1", &[("c.py", "c = 3\n")]).await;
    let h2 = e.task("t2", &[("d.py", "d = 4\n")]).await;
    let (pre1, main1) = merged(e.merge("t1", &h1).await);
    let (_, main2) = merged(e.merge("t2", &h2).await);
    let err = e.vcs.undo_merge("demo", &pre1, &main1).await.unwrap_err();
    assert!(matches!(err, AvError::IllegalState(_)), "{err:?}");
    assert_eq!(e.vcs.trunk("demo").await.unwrap(), main2);
    assert_eq!(e.main_file("d.py").await, "d = 4\n");
}

#[tokio::test(flavor = "multi_thread")]
async fn trunk_log_lists_merges_newest_first_with_their_messages() {
    let e = env().await;
    let a = e
        .task("ta", &[("a.py", "one = 1\ntwo = 22\nthree = 3\n")])
        .await;
    let (_, main_a) = merged(e.merge("ta", &a).await);
    let b = e
        .task("tb", &[("pkg/b.py", "def b():\n    return 3\n")])
        .await;
    let (pre_b, main_b) = merged(e.merge("tb", &b).await);
    let log = e.vcs.trunk_log("demo", 10).await.unwrap();
    assert_eq!(log[0].change, main_b);
    assert_eq!(log[0].parents, vec![pre_b.clone()]);
    assert!(log[0].description.contains("merge tb"), "{:?}", log[0]);
    assert_eq!(log[1].change, main_a);
    assert!(log[1].description.contains("merge ta"));
    assert_eq!(log[2].change, e.main, "then the imported main");
    assert_eq!(e.vcs.trunk_log("demo", 1).await.unwrap().len(), 1);
}

fn bare_remote() -> TempDir {
    let d = TempDir::new().unwrap();
    git(d.path(), &["init", "-q", "--bare", "-b", "main"]);
    d
}

#[tokio::test(flavor = "multi_thread")]
async fn push_backup_to_local_bare_remote_has_main_commit() {
    let e = env().await;
    let remote = bare_remote();
    let url = remote.path().to_str().unwrap().to_owned();
    let a = e
        .task("ta", &[("a.py", "one = 1\ntwo = 22\nthree = 3\n")])
        .await;
    let (_, main) = merged(e.merge("ta", &a).await);
    let pushed = e.vcs.push_backup("demo", &url, None).await.unwrap();
    assert_eq!(pushed, main);
    assert_eq!(
        git(remote.path(), &["rev-parse", "refs/heads/main"]),
        main.0
    );
    // nothing new: pushing again is a no-op that still succeeds
    assert_eq!(e.vcs.push_backup("demo", &url, None).await.unwrap(), main);
}

#[tokio::test(flavor = "multi_thread")]
async fn push_backup_non_fast_forward_is_an_error_not_a_force() {
    let e = env().await;
    let remote = bare_remote();
    let url = remote.path().to_str().unwrap().to_owned();
    e.vcs.push_backup("demo", &url, None).await.unwrap();
    // someone else moves the remote's main to an unrelated history
    let other = TempDir::new().unwrap();
    git(other.path(), &["init", "-q", "-b", "main"]);
    std::fs::write(other.path().join("x"), "x").unwrap();
    git(other.path(), &["add", "-A"]);
    git(other.path(), &["commit", "-q", "-m", "elsewhere"]);
    git(other.path(), &["push", "-q", "--force", &url, "main:main"]);
    let theirs = git(remote.path(), &["rev-parse", "refs/heads/main"]);
    let a = e
        .task("ta", &[("a.py", "one = 1\ntwo = 22\nthree = 3\n")])
        .await;
    merged(e.merge("ta", &a).await);
    let err = e.vcs.push_backup("demo", &url, None).await.unwrap_err();
    assert!(matches!(err, AvError::Vcs(_)), "{err:?}");
    assert_eq!(
        git(remote.path(), &["rev-parse", "refs/heads/main"]),
        theirs,
        "never forced"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn push_backup_rejects_option_like_and_ext_remotes() {
    let e = env().await;
    for bad in [
        "--upload-pack=touch /tmp/x",
        "-x",
        "ext::sh -c touch% /tmp/x",
        "",
    ] {
        let err = e.vcs.push_backup("demo", bad, None).await.unwrap_err();
        assert!(matches!(err, AvError::Invalid(_)), "{bad}: {err:?}");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn push_backup_to_a_named_branch_leaves_the_trunk_name_alone() {
    let e = env().await;
    let remote = bare_remote();
    let url = remote.path().to_str().unwrap().to_owned();
    let a = e
        .task("ta", &[("a.py", "one = 1\ntwo = 22\nthree = 3\n")])
        .await;
    let (_, main) = merged(e.merge("ta", &a).await);
    let pushed = e
        .vcs
        .push_backup("demo", &url, Some("av/phase-1"))
        .await
        .unwrap();
    assert_eq!(pushed, main);
    assert_eq!(
        git(remote.path(), &["rev-parse", "refs/heads/av/phase-1"]),
        main.0
    );
    let trunk = std::process::Command::new("git")
        .current_dir(remote.path())
        .args(["rev-parse", "--verify", "-q", "refs/heads/main"])
        .output()
        .unwrap();
    assert!(
        !trunk.status.success(),
        "main was not pushed under its own name"
    );
    // pushing again is a no-op
    assert_eq!(
        e.vcs
            .push_backup("demo", &url, Some("av/phase-1"))
            .await
            .unwrap(),
        main
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn push_backup_rejects_branch_names_git_would_not_accept() {
    let e = env().await;
    let remote = bare_remote();
    let url = remote.path().to_str().unwrap().to_owned();
    for bad in ["", "-x", "a..b", "a b", "x.lock", "/x", "x/", "a:b", "a~1"] {
        let err = e
            .vcs
            .push_backup("demo", &url, Some(bad))
            .await
            .unwrap_err();
        assert!(matches!(err, AvError::Invalid(_)), "{bad:?}: {err:?}");
    }
}
