use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

/// The daemon's binary with `arg`, killed if it is still running after 20 s (it started serving).
fn daemon(arg: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_margincode-daemon"))
        .arg(arg)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let start = Instant::now();
    while child.try_wait().unwrap().is_none() {
        if start.elapsed() > Duration::from_secs(20) {
            child.kill().unwrap();
            break;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    child.wait_with_output().unwrap()
}

#[test]
fn version_prints_the_version_and_starts_nothing() {
    for flag in ["--version", "-V"] {
        let out = daemon(flag);
        assert!(out.status.success(), "{flag}: {out:?}");
        assert_eq!(
            String::from_utf8_lossy(&out.stdout),
            format!("margincode-daemon {}\n", env!("CARGO_PKG_VERSION")),
            "{flag}"
        );
    }
}

#[test]
fn an_argument_it_does_not_know_is_refused() {
    let out = daemon("serve");
    assert_eq!(out.status.code(), Some(2), "{out:?}");
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.contains("unexpected argument \"serve\""), "{err}");
}
