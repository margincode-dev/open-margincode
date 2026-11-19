use std::time::Duration;

use margincode_agentrun::process::{run_with_timeout, shell};

use crate::HANG;

#[test]
fn a_finished_child_reports_its_exit_code() {
    let dir = tempfile::tempdir().unwrap();
    let cmd = shell("exit 3", dir.path(), &dir.path().join("log")).unwrap();
    let w = run_with_timeout(cmd, HANG).unwrap();
    assert_eq!((w.exit, w.killed), (Some(3), false));
}

#[test]
fn shell_logs_both_streams_in_the_given_directory() {
    let dir = tempfile::tempdir().unwrap();
    let log = dir.path().join("out.log");
    let cmd = shell("pwd; echo to-out; echo to-err >&2", dir.path(), &log).unwrap();
    run_with_timeout(cmd, HANG).unwrap();
    let text = std::fs::read_to_string(&log).unwrap();
    assert!(text.contains("to-out") && text.contains("to-err"), "{text}");
    let here = dir.path().canonicalize().unwrap();
    assert!(text.contains(here.to_str().unwrap()), "{text}");
}

#[test]
fn a_log_that_cannot_be_created_names_the_path() {
    let dir = tempfile::tempdir().unwrap();
    let bad = dir.path().join("no/such/dir/log");
    let err = shell("true", dir.path(), &bad).unwrap_err();
    assert!(err.contains("no/such/dir/log"), "{err}");
}

#[test]
fn a_child_past_its_deadline_is_killed_with_everything_it_started() {
    let dir = tempfile::tempdir().unwrap();
    let pidfile = dir.path().join("grandchild.pid");
    let script = format!(
        "sleep {} & echo $! > {}; wait",
        HANG.as_secs(),
        pidfile.display()
    );
    // a kill between the redirect and the echo leaves the file empty, and `kill -0 ''` fails
    let pid = crate::deadlines()
        .find_map(|deadline| {
            let cmd = shell(&script, dir.path(), &dir.path().join("log")).unwrap();
            let w = run_with_timeout(cmd, deadline).unwrap();
            assert_eq!((w.exit, w.killed), (None, true), "after {deadline:?}");
            let pid = std::fs::read_to_string(&pidfile).unwrap_or_default();
            pid.ends_with('\n').then(|| pid.trim().to_owned())
        })
        .expect("hung: the script never wrote its grandchild's pid before a deadline");
    // the grandchild sleeps in the same process group, so it goes too
    let t = std::time::Instant::now();
    while std::process::Command::new("kill")
        .args(["-0", &pid])
        .status()
        .unwrap()
        .success()
    {
        assert!(
            t.elapsed() < HANG,
            "hung: the grandchild {pid} survived the kill"
        );
        std::thread::sleep(Duration::from_millis(50));
    }
}

#[test]
fn run_until_kills_at_once_when_the_flag_is_already_raised() {
    use std::sync::atomic::AtomicBool;
    let dir = tempfile::tempdir().unwrap();
    let cmd = shell("sleep 60", dir.path(), &dir.path().join("log")).unwrap();
    let w = margincode_agentrun::process::run_until(
        cmd,
        Duration::from_secs(30),
        &AtomicBool::new(true),
    )
    .unwrap();
    assert_eq!((w.exit, w.killed, w.stopped), (None, false, true));
}
