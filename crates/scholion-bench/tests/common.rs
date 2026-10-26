#![allow(dead_code)]

use std::path::Path;
use std::process::Command;

pub fn git(dir: &Path, args: &[&str]) -> String {
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

/// A repo with a base commit (`calc.py` with a bug, `README.md`) and a fix commit that corrects
/// `calc.py` and adds `test_calc.py`. Returns (base, fix).
pub fn fixture_repo(dir: &Path) -> (String, String) {
    git(dir, &["init", "-q", "-b", "main"]);
    std::fs::write(dir.join("calc.py"), "def add(a, b):\n    return a - b\n").unwrap();
    std::fs::write(dir.join("README.md"), "# calc\n").unwrap();
    git(dir, &["add", "-A"]);
    git(dir, &["commit", "-q", "-m", "base"]);
    let base = git(dir, &["rev-parse", "HEAD"]);
    std::fs::write(dir.join("calc.py"), "def add(a, b):\n    return a + b\n").unwrap();
    std::fs::write(
        dir.join("test_calc.py"),
        "from calc import add\nassert add(2, 3) == 5\nprint('ok')\n",
    )
    .unwrap();
    git(dir, &["add", "-A"]);
    git(dir, &["commit", "-q", "-m", "fix add"]);
    let fix = git(dir, &["rev-parse", "HEAD"]);
    (base, fix)
}

pub fn spec_toml(repo: &Path, base: &str, fix: &str) -> String {
    format!(
        r#"name = "calc-add"
repo_path = "{repo}"
base_rev = "{base}"
fix_rev = "{fix}"
goal = "add() returns the wrong result for positive numbers. Fix it."
relevant_files = ["calc.py"]
created_files = ["test_calc.py"]
hidden_tests = ["test_calc.py"]
verify_cmd = "python3 test_calc.py"
verify_timeout_secs = 20
max_turns = 12
model = "claude-sonnet-5-5"
setup_cmd = ""

[[dead_end]]
description = "Swapping the operands of the subtraction does not fix add()."
file_glob = "calc.py"
pattern = 'return b - a'
"#,
        repo = repo.display()
    )
}
