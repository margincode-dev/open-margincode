## Task
# fd-b3a8ff6 · fd-b3a8ff6-2 · open
Goal: Take the broken symlink depth from the walker

The depth of a broken symlink was derived from its path by stripping the
matching search root, because the walker did not report a depth on the
errors that carry broken symlinks. ignore 0.4.28 fills that depth in, so
read it from the error instead and drop the path arithmetic.

The path based version could not express one case. When two search roots
overlap, the walker visits the same broken symlink once per root at a
different depth each time, and both visits carry the same path, so a
single path derived answer had to serve both. Searching roots a and a/b
with --min-depth 2 dropped a/b/blink even though a real file in its place
was kept.
Workspace: /root/bench/ws/rebf815-fd-b3a8ff6/fd-b3a8ff6-2
Changed (0): nothing yet

## Map
- repo map (59 files):
  .cargo/ (1 files)
    config.toml
  .github/ (7 files)
    FUNDING.yml
    ISSUE_TEMPLATE/ (4 files)
    dependabot.yml
    workflows/ (1 files)
  .gitignore
  CHANGELOG.md
  CONTRIBUTING.md
  Cargo.lock
  Cargo.toml
  Cross.toml
  LICENSE-APACHE
  LICENSE-MIT
  Makefile
  README.md
  SECURITY.md
  contrib/ (4 files)
    completion/ (4 files)
  doc/ (8 files)
    .gitattributes
    fd.1
    logo.png
    logo.svg
    release-checklist.md
    screencast.sh
    screencast.svg
    sponsors.md
  rustfmt.toml
  scripts/ (3 files)
    create-deb.sh
    update-help.awk
    version-bump.sh
  src/ (22 files)
    cli.rs
    config.rs
    dir_entry.rs
    error.rs
    exec/ (3 files)
    exit_codes.rs
    filesystem.rs
    filetypes.rs
    filter/ (4 files)
    fmt/ (2 files)
    hyperlink.rs
    main.rs
    output.rs
    regex_helper.rs
    sanitize.rs
    walk.rs
  tests/ (2 files)
    testenv/ (1 files)
    tests.rs

## Code
- sym:40541 DirEntryInner (src/dir_entry.rs:12-17)
  enum DirEntryInner
- sym:40544 DirEntry.broken_symlink (src/dir_entry.rs:36-42)
  pub fn broken_symlink(path: PathBuf, depth: usize) -> Self
- sym:40539 Config (src/config.rs:14-136)
  pub struct Config
- sym:40545 DirEntry.path (src/dir_entry.rs:44-49)
  pub fn path(&self) -> &Path
- sym:40550 DirEntry.metadata (src/dir_entry.rs:91-98)
  pub fn metadata(&self) -> Option<&Metadata>
- sym:40551 DirEntry.depth (src/dir_entry.rs:100-105)
  pub fn depth(&self) -> Option<usize>
- sym:40552 DirEntry.style (src/dir_entry.rs:107-111)
  pub fn style(&self, ls_colors: &LsColors) -> Option<&Style>
- sym:40869 test_exact_depth_broken_symlink (tests/tests.rs:1333-1364)
  fn test_exact_depth_broken_symlink()
- sym:40867 test_min_depth_broken_symlink_absolute_path (tests/tests.rs:1254-1286)
  fn test_min_depth_broken_symlink_absolute_path()
- sym:40795 TestEnv.create_broken_symlink (tests/testenv/mod.rs:179-197)
  pub fn create_broken_symlink<P: AsRef<Path>>( &mut self, link_path: P, ) -> Result<PathBuf, io::Error>
- sym:40868 test_max_depth_broken_symlink (tests/tests.rs:1294-1325)
  fn test_max_depth_broken_symlink()
- sym:40866 test_min_depth_broken_symlink (tests/tests.rs:1216-1246)
  fn test_min_depth_broken_symlink()

---
tokens: 938/2000
