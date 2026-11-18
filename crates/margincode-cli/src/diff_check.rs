//! `av diff --check`: what `git diff --check` reports, read from the daemon's diff, since a task's
//! jj workspace has no `.git` for git to read.

/// Like the daemon's submit check (`flow.rs` `MARKERS`), plus git's diff3 base marker: a bare
/// `=======` underlines a Markdown or reStructuredText heading, so only labelled markers count.
const MARKERS: [&str; 5] = ["<<<<<<< ", ">>>>>>> ", "||||||| ", "%%%%%%% ", "+++++++ "];

/// The problems the added lines of one file's unified diff bring, as `path:line: what`, numbered
/// in the new file.
pub fn problems(path: &str, diff: &str) -> Vec<String> {
    let mut out = vec![];
    let mut line = 0;
    let mut in_hunk = false;
    let mut last_added = false;
    for l in diff.lines() {
        if let Some(header) = l.strip_prefix("@@ ") {
            line = header
                .split_whitespace()
                .find_map(|p| p.strip_prefix('+'))
                .and_then(|p| p.split(',').next()?.parse().ok())
                .unwrap_or(0);
            in_hunk = true;
            last_added = false;
            continue;
        }
        if !in_hunk {
            continue;
        }
        if let Some(text) = l.strip_prefix('+') {
            if text.ends_with([' ', '\t']) {
                out.push(format!("{path}:{line}: trailing whitespace"));
            }
            if MARKERS.iter().any(|m| text.starts_with(m)) {
                out.push(format!("{path}:{line}: leftover conflict marker"));
            }
            line += 1;
            last_added = true;
        } else if l.starts_with('\\') {
            if last_added {
                out.push(format!("{path}:{}: no newline at end of file", line - 1));
            }
        } else {
            if l.starts_with(' ') {
                line += 1;
            }
            last_added = false;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::problems;

    #[test]
    fn added_lines_are_checked_and_numbered_in_the_new_file() {
        let diff = "--- a/doc.md\n+++ b/doc.md\n@@ -1,3 +1,6 @@\n same\n-gone\n+tail \n+tab\t\n\
                    +<<<<<<< conflict 1 of 1\n+%%%%%%% diff from: x\n kept\n@@ -9,1 +12,2 @@\n more\n+last\n\
                    \\ No newline at end of file\n";
        assert_eq!(
            problems("doc.md", diff),
            [
                "doc.md:2: trailing whitespace",
                "doc.md:3: trailing whitespace",
                "doc.md:4: leftover conflict marker",
                "doc.md:5: leftover conflict marker",
                "doc.md:13: no newline at end of file",
            ]
        );
    }

    #[test]
    fn removed_lines_context_and_look_alikes_are_not_problems() {
        let diff = "--- a/x\n+++ b/x\n@@ -1,4 +1,4 @@\n-old \n-<<<<<<< x\n ctx \n+Summary\n+=======\n\
                    +a <<<<<<< b\n-end\n\\ No newline at end of file\n+end\n";
        assert_eq!(problems("x", diff), Vec::<String>::new());
    }
}
