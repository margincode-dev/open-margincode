use std::time::{Duration, Instant};

use scholion_agentrun::slots::{self, Kind};
use tempfile::TempDir;

#[test]
fn a_host_hands_out_no_more_than_n_slots() {
    let dir = TempDir::new().unwrap();
    let a = slots::take(dir.path(), 2).unwrap().expect("first slot");
    let _b = slots::take(dir.path(), 2).unwrap().expect("second slot");
    assert!(slots::take(dir.path(), 2).unwrap().is_none(), "only 2");
    drop(a);
    // free again, though not always at once: a child that another test forks at that moment
    // shares the lock until it execs (see the module doc)
    let t = Instant::now();
    while slots::take(dir.path(), 2).unwrap().is_none() {
        assert!(
            t.elapsed() < Duration::from_secs(5),
            "a freed slot is free again"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
}

#[test]
fn wait_for_returns_once_a_slot_frees_and_says_so_once() {
    let dir = TempDir::new().unwrap();
    let held = slots::take(dir.path(), 1).unwrap().unwrap();
    let release = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(500));
        drop(held);
    });
    let mut said = 0;
    let t = Instant::now();
    let _slot = slots::wait_for(dir.path(), 1, Duration::from_millis(50), || said += 1).unwrap();
    assert!(
        t.elapsed() >= Duration::from_millis(400),
        "{:?}",
        t.elapsed()
    );
    assert_eq!(said, 1);
    release.join().unwrap();
    // a free slot is taken at once, without a word
    let dir = TempDir::new().unwrap();
    let mut said = 0;
    slots::wait_for(dir.path(), 1, Duration::from_millis(50), || said += 1).unwrap();
    assert_eq!(said, 0);
}

#[test]
fn limits_are_positive_counts_or_unset() {
    assert_eq!(slots::limit(None), Ok(None));
    assert_eq!(slots::limit(Some("")), Ok(None));
    assert_eq!(slots::limit(Some(" 3 ")), Ok(Some(3)));
    assert!(slots::limit(Some("0")).is_err());
    assert!(slots::limit(Some("many")).is_err());
    assert_eq!(Kind::Agents.var(), "AV_MAX_AGENTS");
    assert_eq!(Kind::Builds.var(), "AV_MAX_BUILDS");
}

#[test]
fn slot_dirs_live_in_av_home() {
    let home = std::path::Path::new("/h/.scholion");
    assert_eq!(Kind::Agents.dir_in(home), home.join("agents"));
    assert_eq!(Kind::Builds.dir_in(home), home.join("builds"));
}

#[test]
fn av_home_is_av_home_else_the_users_home() {
    let env = |pairs: &'static [(&'static str, &'static str)]| {
        move |k: &str| {
            pairs
                .iter()
                .find(|(n, _)| *n == k)
                .map(|(_, v)| std::ffi::OsString::from(v))
        }
    };
    assert_eq!(
        slots::av_home_from(env(&[("AV_HOME", "/data/av"), ("HOME", "/h")])),
        std::path::PathBuf::from("/data/av")
    );
    assert_eq!(
        slots::av_home_from(env(&[("AV_HOME", ""), ("HOME", "/h")])),
        std::path::PathBuf::from("/h/.scholion")
    );
    assert_eq!(
        slots::home_from(env(&[("HOME", ""), ("USERPROFILE", r"C:\Users\u")])),
        Some(std::path::PathBuf::from(r"C:\Users\u"))
    );
    assert_eq!(slots::home_from(env(&[])), None);
}
