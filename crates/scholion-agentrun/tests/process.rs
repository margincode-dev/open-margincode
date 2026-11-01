//! Running a child with a deadline and a log.

use std::time::{Duration, Instant};

use scholion_agentrun::process::{run_with_timeout, shell};

#[test]
fn a_finished_child_reports_its_exit_code() {
    let dir = tempfile::tempdir().unwrap();
    let cmd = shell("exit 3", dir.path(), &dir.path().join("log")).unwrap();
    let w = run_with_timeout(cmd, Duration::from_secs(10)).unwrap();
    assert_eq!((w.exit, w.killed), (Some(3), false));
}

#[test]
fn shell_logs_both_streams_in_the_given_directory() {
    let dir = tempfile::tempdir().unwrap();
    let log = dir.path().join("out.log");
    let cmd = shell("pwd; echo to-out; echo to-err >&2", dir.path(), &log).unwrap();
    run_with_timeout(cmd, Duration::from_secs(10)).unwrap();
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
    let script = format!("sleep 60 & echo $! > {}; wait", pidfile.display());
    let cmd = shell(&script, dir.path(), &dir.path().join("log")).unwrap();
    let start = Instant::now();
    let w = run_with_timeout(cmd, Duration::from_millis(500)).unwrap();
    assert!(start.elapsed() < Duration::from_secs(10));
    assert_eq!((w.exit, w.killed), (None, true));
    let pid = std::fs::read_to_string(&pidfile).unwrap().trim().to_owned();
    // the grandchild sleeps in the same process group, so it goes too
    for _ in 0..40 {
        let alive = std::process::Command::new("kill")
            .args(["-0", &pid])
            .status()
            .unwrap()
            .success();
        if !alive {
            return;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    panic!("the grandchild {pid} survived the kill");
}
