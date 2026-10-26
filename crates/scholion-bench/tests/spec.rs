mod common;

use common::*;
use scholion_bench::spec::{check_spec, load_spec};
use tempfile::TempDir;

fn write_spec(dir: &TempDir, text: &str) -> std::path::PathBuf {
    let p = dir.path().join("task.toml");
    std::fs::write(&p, text).unwrap();
    p
}

#[test]
fn load_spec_parses_all_fields() {
    let repo = TempDir::new().unwrap();
    let (base, fix) = fixture_repo(repo.path());
    let d = TempDir::new().unwrap();
    let s = load_spec(&write_spec(&d, &spec_toml(repo.path(), &base, &fix))).unwrap();
    assert_eq!(s.name, "calc-add");
    assert_eq!(s.repo_path, repo.path());
    assert_eq!(s.base_rev, base);
    assert_eq!(s.fix_rev.as_deref(), Some(fix.as_str()));
    assert_eq!(s.relevant_files, ["calc.py"]);
    assert_eq!(s.created_files, ["test_calc.py"]);
    assert_eq!(s.hidden_tests, ["test_calc.py"]);
    assert_eq!(s.verify_timeout_secs, 20);
    assert_eq!(s.max_turns, 12);
    assert_eq!(s.model, "claude-sonnet-5-5");
    assert_eq!(s.dead_ends.len(), 1);
    assert_eq!(s.dead_ends[0].file_glob, "calc.py");
    assert_eq!(s.session_timeout_secs, 1800, "default");
}

#[test]
fn check_spec_ok_for_valid_task() {
    let repo = TempDir::new().unwrap();
    let (base, fix) = fixture_repo(repo.path());
    let d = TempDir::new().unwrap();
    let s = load_spec(&write_spec(&d, &spec_toml(repo.path(), &base, &fix))).unwrap();
    check_spec(&s).unwrap();
}

#[test]
fn check_rejects_missing_repo_naming_the_task() {
    let repo = TempDir::new().unwrap();
    let (base, fix) = fixture_repo(repo.path());
    let d = TempDir::new().unwrap();
    let text = spec_toml(repo.path(), &base, &fix)
        .replace(&repo.path().display().to_string(), "/nonexistent/repo");
    let s = load_spec(&write_spec(&d, &text)).unwrap();
    let err = check_spec(&s).unwrap_err();
    assert!(
        err.contains("calc-add") && err.contains("/nonexistent/repo"),
        "{err}"
    );
}

#[test]
fn check_rejects_unknown_rev() {
    let repo = TempDir::new().unwrap();
    let (base, fix) = fixture_repo(repo.path());
    let d = TempDir::new().unwrap();
    let text = spec_toml(repo.path(), &base, &fix).replace(&base, "deadbeefdeadbeef");
    let s = load_spec(&write_spec(&d, &text)).unwrap();
    let err = check_spec(&s).unwrap_err();
    assert!(err.contains("deadbeefdeadbeef"), "{err}");
}

#[test]
fn check_rejects_relevant_file_absent_at_rev() {
    let repo = TempDir::new().unwrap();
    let (base, fix) = fixture_repo(repo.path());
    let d = TempDir::new().unwrap();
    let text = spec_toml(repo.path(), &base, &fix).replace(
        r#"relevant_files = ["calc.py"]"#,
        r#"relevant_files = ["nope.py"]"#,
    );
    let s = load_spec(&write_spec(&d, &text)).unwrap();
    let err = check_spec(&s).unwrap_err();
    assert!(err.contains("nope.py"), "{err}");
}

#[test]
fn check_accepts_created_file_absent_at_rev_and_rejects_it_if_present() {
    let repo = TempDir::new().unwrap();
    let (base, fix) = fixture_repo(repo.path());
    let d = TempDir::new().unwrap();
    let ok = load_spec(&write_spec(&d, &spec_toml(repo.path(), &base, &fix))).unwrap();
    check_spec(&ok).unwrap();
    let text = spec_toml(repo.path(), &base, &fix).replace(
        r#"created_files = ["test_calc.py"]"#,
        r#"created_files = ["README.md"]"#,
    );
    let s = load_spec(&write_spec(&d, &text)).unwrap();
    let err = check_spec(&s).unwrap_err();
    assert!(err.contains("README.md"), "{err}");
}

#[test]
fn check_rejects_hidden_test_absent_at_fix_rev() {
    let repo = TempDir::new().unwrap();
    let (base, fix) = fixture_repo(repo.path());
    let d = TempDir::new().unwrap();
    let text = spec_toml(repo.path(), &base, &fix).replace(
        r#"hidden_tests = ["test_calc.py"]"#,
        r#"hidden_tests = ["test_other.py"]"#,
    );
    let s = load_spec(&write_spec(&d, &text)).unwrap();
    let err = check_spec(&s).unwrap_err();
    assert!(err.contains("test_other.py"), "{err}");
}

#[test]
fn check_rejects_invalid_regex() {
    let repo = TempDir::new().unwrap();
    let (base, fix) = fixture_repo(repo.path());
    let d = TempDir::new().unwrap();
    let text = spec_toml(repo.path(), &base, &fix).replace("'return b - a'", "'return (b'");
    let s = load_spec(&write_spec(&d, &text)).unwrap();
    let err = check_spec(&s).unwrap_err();
    assert!(err.contains("pattern"), "{err}");
}

#[test]
fn check_rejects_rev_starting_with_dash() {
    let repo = TempDir::new().unwrap();
    let (base, fix) = fixture_repo(repo.path());
    let d = TempDir::new().unwrap();
    let text = spec_toml(repo.path(), &base, &fix).replace(&base, "--output=/tmp/x");
    let s = load_spec(&write_spec(&d, &text)).unwrap();
    let err = check_spec(&s).unwrap_err();
    assert!(err.contains("--output=/tmp/x"), "{err}");
}

#[test]
fn unsafe_task_names_and_unknown_fields_are_rejected() {
    let repo = TempDir::new().unwrap();
    let (base, fix) = fixture_repo(repo.path());
    let d = TempDir::new().unwrap();
    let text =
        spec_toml(repo.path(), &base, &fix).replace(r#"name = "calc-add""#, r#"name = "../x""#);
    let s = load_spec(&write_spec(&d, &text)).unwrap();
    assert!(check_spec(&s).unwrap_err().contains("name"));
    let text = format!(
        "{}\nverfy_cmd = \"typo\"\n",
        spec_toml(repo.path(), &base, &fix)
    );
    assert!(
        load_spec(&write_spec(&d, &text))
            .unwrap_err()
            .contains("verfy_cmd")
    );
}

#[test]
fn repo_root_override_replaces_the_parent_directory() {
    let repo = TempDir::new().unwrap();
    let (base, fix) = fixture_repo(repo.path());
    let d = TempDir::new().unwrap();
    let text = spec_toml(repo.path(), &base, &fix).replace(
        &repo.path().display().to_string(),
        "~/projects/somewhere/calc",
    );
    let p = write_spec(&d, &text);
    let s =
        scholion_bench::spec::load_spec_with_root(&p, Some(repo.path().parent().unwrap())).unwrap();
    assert_eq!(s.repo_path, repo.path().parent().unwrap().join("calc"));
    let home = std::env::var("HOME").unwrap();
    let s = load_spec(&p).unwrap();
    assert_eq!(
        s.repo_path,
        std::path::Path::new(&home).join("projects/somewhere/calc")
    );
}

#[test]
fn local_tests_resolve_next_to_the_spec_and_must_exist() {
    let repo = TempDir::new().unwrap();
    let (base, fix) = fixture_repo(repo.path());
    let d = TempDir::new().unwrap();
    std::fs::create_dir_all(d.path().join("files")).unwrap();
    std::fs::write(d.path().join("files/t.py"), "assert True\n").unwrap();
    // a table array placed before [[dead_end]], so the dead end's keys stay its own
    let text = spec_toml(repo.path(), &base, &fix).replacen(
        "[[dead_end]]",
        "[[local_tests]]\npath = \"tests/test_bench.py\"\nsource = \"files/t.py\"\n\n[[dead_end]]",
        1,
    );
    let s = load_spec(&write_spec(&d, &text)).unwrap();
    assert_eq!(s.local_tests.len(), 1);
    assert_eq!(s.dead_ends.len(), 1);
    check_spec(&s).unwrap();
    std::fs::remove_file(d.path().join("files/t.py")).unwrap();
    assert!(check_spec(&s).unwrap_err().contains("files/t.py"));
}
