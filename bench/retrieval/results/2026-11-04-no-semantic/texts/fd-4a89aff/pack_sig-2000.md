## Task
# fd-4a89aff · fd-4a89aff-2 · open
Goal: Make broken symlink depth robust under absolute paths

The depth for a broken symlink was derived by stripping the search root
from its path and counting the remaining components. Compare the path
and the roots in absolute form before stripping, so the match holds
whether or not --absolute-path has already made the roots absolute, and
pick the deepest matching root. This avoids an absolute path failing to
match a relative root and falling back to an inflated depth.

Add a regression test that exercises the broken symlink case together
with --absolute-path.
Workspace: /root/bench/ws/rebf815-fd-4a89aff/fd-4a89aff-2
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
- sym:41591 depth_relative_to_roots (src/walk.rs:665-674)
  fn depth_relative_to_roots(path: &Path, roots: &[PathBuf]) -> usize
- sym:41357 DirEntryInner (src/dir_entry.rs:12-17)
  enum DirEntryInner
- sym:41360 DirEntry.broken_symlink (src/dir_entry.rs:36-42)
  pub fn broken_symlink(path: PathBuf, depth: usize) -> Self
- sym:41355 Config (src/config.rs:14-136)
  pub struct Config
- sym:41361 DirEntry.path (src/dir_entry.rs:44-49)
  pub fn path(&self) -> &Path
- sym:41366 DirEntry.metadata (src/dir_entry.rs:91-98)
  pub fn metadata(&self) -> Option<&Metadata>
- sym:41367 DirEntry.depth (src/dir_entry.rs:100-105)
  pub fn depth(&self) -> Option<usize>
- sym:41368 DirEntry.style (src/dir_entry.rs:107-111)
  pub fn style(&self, ls_colors: &LsColors) -> Option<&Style>
- sym:41693 test_symlink_as_root (tests/tests.rs:1565-1603)
  fn test_symlink_as_root()
- sym:41611 TestEnv.create_broken_symlink (tests/testenv/mod.rs:179-197)
  pub fn create_broken_symlink<P: AsRef<Path>>( &mut self, link_path: P, ) -> Result<PathBuf, io::Error>
- sym:41682 test_min_depth_broken_symlink (tests/tests.rs:1216-1246)
  fn test_min_depth_broken_symlink()
- sym:41695 test_symlink_as_absolute_root (tests/tests.rs:1627-1641)
  fn test_symlink_as_absolute_root()

---
tokens: 882/2000
