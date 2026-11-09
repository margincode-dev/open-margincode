### src/dir_entry.rs
```rust
use std::cell::OnceCell;
use std::ffi::OsString;
use std::fs::{FileType, Metadata};
use std::path::{Path, PathBuf};

use lscolors::{Colorable, LsColors, Style};

use crate::config::Config;
use crate::filesystem::strip_current_dir;

#[derive(Debug)]
enum DirEntryInner {
    Normal(ignore::DirEntry),
    // Broken symlinks are surfaced by the walker as errors that carry no depth,
    // so we record the depth (relative to the search root) at creation time.
    BrokenSymlink(PathBuf, usize),
}

#[derive(Debug)]
pub struct DirEntry {
    inner: DirEntryInner,
    metadata: OnceCell<Option<Metadata>>,
    style: OnceCell<Option<Style>>,
}

impl DirEntry {
    #[inline]
    pub fn normal(e: ignore::DirEntry) -> Self {
        Self {
            inner: DirEntryInner::Normal(e),
            metadata: OnceCell::new(),
            style: OnceCell::new(),
        }
    }

    pub fn broken_symlink(path: PathBuf, depth: usize) -> Self {
        Self {
            inner: DirEntryInner::BrokenSymlink(path, depth),
            metadata: OnceCell::new(),
            style: OnceCell::new(),
        }
    }

    pub fn path(&self) -> &Path {
        match &self.inner {
            DirEntryInner::Normal(e) => e.path(),
            DirEntryInner::BrokenSymlink(pathbuf, _) => pathbuf.as_path(),
        }
    }

    pub fn into_path(self) -> PathBuf {
        match self.inner {
            DirEntryInner::Normal(e) => e.into_path(),
            DirEntryInner::BrokenSymlink(p, _) => p,
        }
    }

    /// Returns the path as it should be presented to the user.
    /// When stripping `./` would leave the path starting with `-`, keep the original
    /// (with `./`) so downstream tools don't interpret it as an option.
    pub fn stripped_path(&self, config: &Config) -> &Path {
        let path = self.path();
        if config.strip_cwd_prefix {
            let stripped = strip_current_dir(path);
            if starts_with_dash(stripped) {
                path
            } else {
                stripped
            }
        } else {
            path
        }
    }

    /// Returns the path as it should be presented to the user.
    pub fn into_stripped_path(self, config: &Config) -> PathBuf {
        if config.strip_cwd_prefix {
            self.stripped_path(config).to_path_buf()
        } else {
            self.into_path()
        }
    }

    pub fn file_type(&self) -> Option<FileType> {
        match &self.inner {
            DirEntryInner::Normal(e) => e.file_type(),
            DirEntryInner::BrokenSymlink(..) => self.metadata().map(|m| m.file_type()),
        }
    }

    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata
            .get_or_init(|| match &self.inner {
                DirEntryInner::Normal(e) => e.metadata().ok(),
                DirEntryInner::BrokenSymlink(path, _) => path.symlink_metadata().ok(),
            })
            .as_ref()
    }

    pub fn depth(&self) -> Option<usize> {
        match &self.inner {
            DirEntryInner::Normal(e) => Some(e.depth()),
            DirEntryInner::BrokenSymlink(_, depth) => Some(*depth),
        }
    }

    pub fn style(&self, ls_colors: &LsColors) -> Option<&Style> {
        self.style
            .get_or_init(|| ls_colors.style_for(self).cloned())
            .as_ref()
    }
}

fn starts_with_dash(path: &Path) -> bool {
    path.as_os_str().as_encoded_bytes().first() == Some(&b'-')
}

impl PartialEq for DirEntry {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.path() == other.path()
    }
}

impl Eq for DirEntry {}

impl PartialOrd for DirEntry {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DirEntry {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.path().cmp(other.path())
    }
}

impl Colorable for DirEntry {
    fn path(&self) -> PathBuf {
        self.path().to_owned()
    }

    fn file_name(&self) -> OsString {
        let name = match &self.inner {
            DirEntryInner::Normal(e) => e.file_name(),
            DirEntryInner::BrokenSymlink(path, _) => {
                // Path::file_name() only works if the last component is Normal,
                // but we want it for all component types, so we open code it.
                // Copied from LsColors::style_for_path_with_metadata().
                path.components()
                    .next_back()
                    .map(|c| c.as_os_str())
                    .unwrap_or_else(|| path.as_os_str())
            }
        };
        name.to_owned()
    }

    fn file_type(&self) -> Option<FileType> {
        self.file_type()
    }

    fn metadata(&self) -> Option<Metadata> {
        self.metadata().cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::starts_with_dash;
    use std::path::Path;

    #[test]
    fn dash_prefixed_paths_detected() {
        assert!(starts_with_dash(Path::new("-rf")));
        assert!(starts_with_dash(Path::new("--delete")));
        assert!(starts_with_dash(Path::new("-")));
    }

    #[test]
    fn safe_paths_not_flagged() {
        assert!(!starts_with_dash(Path::new("foo")));
        assert!(!starts_with_dash(Path::new("./foo")));
        assert!(!starts_with_dash(Path::new("sub/-rf")));
        assert!(!starts_with_dash(Path::new("")));
        assert!(!starts_with_dash(Path::new(" -rf")));
    }
}
```
### tests/tests.rs
```rust
mod testenv;

#[cfg(unix)]
use nix::unistd::{Gid, Group, Uid, User};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::{Duration, SystemTime};
use test_case::test_case;

use jiff::Timestamp;
use normpath::PathExt;
use regex::escape;

use crate::testenv::TestEnv;

static DEFAULT_DIRS: &[&str] = &["one/two/three", "one/two/three/directory_foo"];

static DEFAULT_FILES: &[&str] = &[
    "a.foo",
    "one/b.foo",
    "one/two/c.foo",
    "one/two/C.Foo2",
    "one/two/three/d.foo",
    "fdignored.foo",
    "gitignored.foo",
    ".hidden.foo",
    "e1 e2",
];

#[allow(clippy::let_and_return)]
fn get_absolute_root_path(env: &TestEnv) -> String {
    let path = env
        .test_root()
        .normalize()
        .expect("absolute path")
        .as_path()
        .to_str()
        .expect("string")
        .to_string();

    #[cfg(windows)]
    let path = path.trim_start_matches(r"\\?\").to_string();

    path
}

#[cfg(test)]
fn get_test_env_with_abs_path(dirs: &[&'static str], files: &[&'static str]) -> (TestEnv, String) {
    let env = TestEnv::new(dirs, files);
    let root_path = get_absolute_root_path(&env);
    (env, root_path)
}

#[cfg(test)]
fn create_file_with_size<P: AsRef<Path>>(path: P, size_in_bytes: usize) {
    let content = "#".repeat(size_in_bytes);
    let mut f = fs::File::create::<P>(path).unwrap();
    f.write_all(content.as_bytes()).unwrap();
}

/// Simple test
#[test]
fn test_simple() {
    let te = TestEnv::new(DEFAULT_DIRS, DEFAULT_FILES);

    te.assert_output(&["a.foo"], "a.foo");
    te.assert_output(&["b.foo"], "one/b.foo");
    te.assert_output(&["d.foo"], "one/two/three/d.foo");

    te.assert_output(
        &["foo"],
        "a.foo
        one/b.foo
        one/two/c.foo
        one/two/C.Foo2
        one/two/three/d.foo
        one/two/three/directory_foo/",
    );
}

static AND_EXTRA_FILES: &[&str] = &[
    "a.foo",
    "one/b.foo",
    "one/two/c.foo",
    "one/two/C.Foo2",
    "one/two/three/baz-quux",
    "one/two/three/Baz-Quux2",
    "one/two/three/d.foo",
    "fdignored.foo",
    "gitignored.foo",
    ".hidden.foo",
    "A-B.jpg",
    "A-C.png",
    "B-A.png",
    "B-C.png",
    "C-A.jpg",
    "C-B.png",
    "e1 e2",
];

/// AND test
#[test]
fn test_and_basic() {
    let te = TestEnv::new(DEFAULT_DIRS, AND_EXTRA_FILES);

    te.assert_output(
        &["foo", "--and", "c"],
        "one/two/C.Foo2
        one/two/c.foo
        one/two/three/directory_foo/",
    );

    te.assert_output(
        &["f", "--and", "[ad]", "--and", "[_]"],
        "one/two/three/directory_foo/",
    );

    te.assert_output(
        &["f", "--and", "[ad]", "--and", "[.]"],
        "a.foo
        one/two/three/d.foo",
    );

    te.assert_output(&["Foo", "--and", "C"], "one/two/C.Foo2");

    te.assert_output(&["foo", "--and", "asdasdasdsadasd"], "");
}

#[test]
fn test_and_empty_pattern() {
    let te = TestEnv::new(DEFAULT_DIRS, AND_EXTRA_FILES);
    te.assert_output(&["Foo", "--and", "2", "--and", ""], "one/two/C.Foo2");
}

#[test]
fn test_and_bad_pattern() {
    let te = TestEnv::new(DEFAULT_DIRS, AND_EXTRA_FILES);

    te.assert_failure(&["Foo", "--and", "2", "--and", "[", "--and", "C"]);
    te.assert_failure(&["Foo", "--and", "[", "--and", "2", "--and", "C"]);
    te.assert_failure(&["Foo", "--and", "2", "--and", "C", "--and", "["]);
    te.assert_failure(&["[", "--and", "2", "--and", "C", "--and", "Foo"]);
}

#[test]
fn test_and_pattern_starts_with_dash() {
    let te = TestEnv::new(DEFAULT_DIRS, AND_EXTRA_FILES);

    te.assert_output(
        &["baz", "--and", "quux"],
        "one/two/three/Baz-Quux2
        one/two/three/baz-quux",
    );
    te.assert_output(
        &["baz", "--and", "-"],
        "one/two/three/Baz-Quux2
        one/two/three/baz-quux",
    );
    te.assert_output(
        &["Quu", "--and", "x", "--and", "-"],
        "one/two/three/Baz-Quux2",
    );
}

#[test]
fn test_and_plus_extension() {
    let te = TestEnv::new(DEFAULT_DIRS, AND_EXTRA_FILES);

    te.assert_output(
        &[
            "A",
            "--and",
            "B",
            "--extension",
            "jpg",
            "--extension",
            "png",
        ],
        "A-B.jpg
        B-A.png",
    );

    te.assert_output(
        &[
            "A",
            "--extension",
            "jpg",
            "--and",
            "B",
            "--extension",
            "png",
        ],
        "A-B.jpg
        B-A.png",
    );
}

#[test]
fn test_and_plus_type() {
    let te = TestEnv::new(DEFAULT_DIRS, AND_EXTRA_FILES);

    te.assert_output(
        &["c", "--type", "d", "--and", "foo"],
        "one/two/three/directory_foo/",
    );

    te.assert_output(
        &["c", "--type", "f", "--and", "foo"],
        "one/two/C.Foo2
        one/two/c.foo",
    );
}

#[test]
fn test_and_plus_glob() {
    let te = TestEnv::new(DEFAULT_DIRS, AND_EXTRA_FILES);

    te.assert_output(&["*foo", "--glob", "--and", "c*"], "one/two/c.foo");
}

#[test]
fn test_and_plus_fixed_strings() {
    let te = TestEnv::new(DEFAULT_DIRS, AND_EXTRA_FILES);

    te.assert_output(
        &["foo", "--fixed-strings", "--and", "c", "--and", "."],
        "one/two/c.foo
        one/two/C.Foo2",
    );

    te.assert_output(
        &["foo", "--fixed-strings", "--and", "[c]", "--and", "."],
        "",
    );

    te.assert_output(
        &["Foo", "--fixed-strings", "--and", "C", "--and", "."],
        "one/two/C.Foo2",
    );
}

#[test]
fn test_and_plus_ignore_case() {
    let te = TestEnv::new(DEFAULT_DIRS, AND_EXTRA_FILES);

    te.assert_output(
        &["Foo", "--ignore-case", "--and", "C", "--and", "[.]"],
        "one/two/C.Foo2
        one/two/c.foo",
    );
}

#[test]
fn test_and_plus_case_sensitive() {
    let te = TestEnv::new(DEFAULT_DIRS, AND_EXTRA_FILES);

    te.assert_output(
        &["foo", "--case-sensitive", "--and", "c", "--and", "[.]"],
        "one/two/c.foo",
    );
}

#[test]
fn test_and_plus_full_path() {
    let te = TestEnv::new(DEFAULT_DIRS, AND_EXTRA_FILES);

    te.assert_output(
        &[
            "three",
            "--full-path",
            "--and",
            "_foo",
            "--and",
            r"[/\\]dir",
        ],
        "one/two/three/directory_foo/",
    );

    te.assert_output(
        &[
            "three",
            "--full-path",
            "--and",
            r"[/\\]two",
            "--and",
            r"[/\\]dir",
        ],
        "one/two/three/directory_foo/",
    );
}

/// Test each pattern type with an empty pattern.
#[test]
fn test_empty_pattern() {
    let te = TestEnv::new(DEFAULT_DIRS, DEFAULT_FILES);
    let expected = "a.foo
    e1 e2
    one/
    one/b.foo
    one/two/
    one/two/c.foo
    one/two/C.Foo2
    one/two/three/
    one/two/three/d.foo
    one/two/three/directory_foo/
    symlink";

    te.assert_output(&["--regex"], expected);
    te.assert_output(&["--fixed-strings"], expected);
    te.assert_output(&["--glob"], expected);
}

/// Test multiple directory searches
#[test]
fn test_multi_file() {
    let dirs = &["test1", "test2"];
    let files = &["test1/a.foo", "test1/b.foo", "test2/a.foo"];
    let te = TestEnv::new(dirs, files);
    te.assert_output(
        &["a.foo", "test1", "test2"],
        "test1/a.foo
        test2/a.foo",
    );

    te.assert_output(
        &["", "test1", "test2"],
        "test1/a.foo
        test2/a.foo
        test1/b.foo",
    );

    te.assert_output(&["a.foo", "test1"], "test1/a.foo");

    te.assert_output(&["b.foo", "test1", "test2"], "test1/b.foo");
}

/// Test search over multiple directory with missing
#[test]
fn test_multi_file_with_missing() {
    let dirs = &["real"];
    let files = &["real/a.foo", "real/b.foo"];
    let te = TestEnv::new(dirs, files);
    te.assert_output(&["a.foo", "real", "fake"], "real/a.foo");

    te.assert_error(
        &["a.foo", "real", "fake"],
        "[fd error]: Search path 'fake' is not a directory.",
    );

    te.assert_output(
        &["", "real", "fake"],
        "real/a.foo
        real/b.foo",
    );

    te.assert_output(
        &["", "real", "fake1", "fake2"],
        "real/a.foo
        real/b.foo",
    );

    te.assert_error(
        &["", "real", "fake1", "fake2"],
        "[fd error]: Search path 'fake1' is not a directory.
        [fd error]: Search path 'fake2' is not a directory.",
    );

    te.assert_failure_with_error(
        &["", "fake1", "fake2"],
        "[fd error]: Search path 'fake1' is not a directory.
        [fd error]: Search path 'fake2' is not a directory.
        [fd error]: No valid search paths given.",
    );
}

/// Without --full-path, a pattern containing '/' should always produce the
/// path-separator diagnostic, even if the pattern does not name an existing
/// directory. Before the fix for sharkdp/fd#1873 this only fired when the
/// pattern happened to resolve to a real directory, so the common typo of
/// pasting a full path silently returned zero matches.
#[test]
fn test_pattern_with_forward_slash_is_rejected() {
    let te = TestEnv::new(DEFAULT_DIRS, DEFAULT_FILES);

    // Pattern that is NOT a real directory; old behaviour: no warning.
    te.assert_failure_with_error(
        &["nonexistent/path"],
