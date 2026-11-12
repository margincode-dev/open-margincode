use std::path::PathBuf;
use std::process::Command;

use margincode_bench::spec::{check_spec, load_spec, spec_files};

fn routing_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../bench/tasks-routing")
}

#[test]
fn pilot_specs_pass_check() {
    let dir = routing_dir().canonicalize().expect("bench/tasks-routing");
    let repo = dir.join("repo");
    if !repo.join(".git").exists() {
        let py = ["python3", "python"]
            .into_iter()
            .find(|bin| {
                Command::new(bin)
                    .arg("--version")
                    .status()
                    .is_ok_and(|s| s.success())
            })
            .expect("python to build bench/tasks-routing/repo");
        let out = Command::new(py)
            .arg(dir.join("make-repo.py"))
            .arg(&repo)
            .output()
            .expect("make-repo.py");
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
    let files = spec_files(&dir).unwrap();
    assert_eq!(files.len(), 6, "{files:?}");
    let mut names = vec![];
    for f in &files {
        let mut spec = load_spec(f).unwrap();
        spec.repo_path = repo.clone();
        check_spec(&spec).unwrap();
        assert!(
            spec.verify_cmd.contains("PYTHONDONTWRITEBYTECODE=1"),
            "{}",
            spec.name
        );
        if spec.name == "chart-json" {
            assert!(!spec.goal.contains("apples"), "{}", spec.goal);
            assert!(!spec.goal.contains("pears"), "{}", spec.goal);
            assert!(spec.created_files.iter().any(|p| p == "assets/q3.json"));
            assert!(spec.relevant_files.iter().any(|p| p == "assets/q3.svg"));
        }
        names.push(spec.name);
    }
    names.sort();
    assert_eq!(
        names,
        [
            "chart-json",
            "exit-codes",
            "least-multiple",
            "settings-form",
            "slug-collapse",
            "version-tests",
        ]
    );
}
