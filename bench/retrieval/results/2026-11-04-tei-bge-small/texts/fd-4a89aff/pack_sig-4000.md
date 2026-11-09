## Task
# fd-4a89aff · fd-4a89aff · open
Goal: Make broken symlink depth robust under absolute paths

The depth for a broken symlink was derived by stripping the search root
from its path and counting the remaining components. Compare the path
and the roots in absolute form before stripping, so the match holds
whether or not --absolute-path has already made the roots absolute, and
pick the deepest matching root. This avoids an absolute path failing to
match a relative root and falling back to an inflated depth.

Add a regression test that exercises the broken symlink case together
with --absolute-path.
Workspace: /root/bench/av_tei/ws/rec09fc-fd-4a89aff/fd-4a89aff
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
- sym:41357 DirEntryInner (src/dir_entry.rs:12-17)
  enum DirEntryInner
- sym:41360 DirEntry.broken_symlink (src/dir_entry.rs:36-42)
  pub fn broken_symlink(path: PathBuf, depth: usize) -> Self
- sym:41367 DirEntry.depth (src/dir_entry.rs:100-105)
  pub fn depth(&self) -> Option<usize>
- sym:41591 depth_relative_to_roots (src/walk.rs:665-674)
  fn depth_relative_to_roots(path: &Path, roots: &[PathBuf]) -> usize
- sym:41355 Config (src/config.rs:14-136)
  pub struct Config
- sym:41361 DirEntry.path (src/dir_entry.rs:44-49)
  pub fn path(&self) -> &Path
- sym:41501 FormatTemplate.replace_separator (src/fmt/mod.rs:147-196)
  fn replace_separator<'a>(path: &'a OsStr, path_separator: Option<&str>) -> Cow<'a, OsStr>
- sym:41592 search_str_for_entry (src/walk.rs:676-698)
  fn search_str_for_entry<'a>( entry_path: &'a std::path::Path, full_path_base: Option<&std::path::Path>, ) -> Cow<'a, OsStr>
- sym:41362 DirEntry.into_path (src/dir_entry.rs:51-56)
  pub fn into_path(self) -> PathBuf
- sym:41335 Opts.no_search_paths (src/cli.rs:738-740)
  pub fn no_search_paths(&self) -> bool
- sym:41431 absolute_path (src/filesystem.rs:23-36)
  pub fn absolute_path(path: &Path) -> io::Result<PathBuf>
- sym:41333 Opts.search_paths (src/cli.rs:696-721)
  pub fn search_paths(&self) -> anyhow::Result<Vec<PathBuf>>
- sym:41430 path_absolute_form (src/filesystem.rs:14-21)
  pub fn path_absolute_form(path: &Path) -> io::Result<PathBuf>
- sym:41693 test_symlink_as_root (tests/tests.rs:1565-1603)
  fn test_symlink_as_root()
- sym:41682 test_min_depth_broken_symlink (tests/tests.rs:1216-1246)
  fn test_min_depth_broken_symlink()
- sym:41695 test_symlink_as_absolute_root (tests/tests.rs:1627-1641)
  fn test_symlink_as_absolute_root()
- sym:41611 TestEnv.create_broken_symlink (tests/testenv/mod.rs:179-197)
  pub fn create_broken_symlink<P: AsRef<Path>>( &mut self, link_path: P, ) -> Result<PathBuf, io::Error>
- sym:41696 test_symlink_and_full_path (tests/tests.rs:1644-1666)
  fn test_symlink_and_full_path()
- sym:41366 DirEntry.metadata (src/dir_entry.rs:91-98)
  pub fn metadata(&self) -> Option<&Metadata>
- sym:41368 DirEntry.style (src/dir_entry.rs:107-111)
  pub fn style(&self, ls_colors: &LsColors) -> Option<&Style>
- sym:41382 OutputBuffer.new (src/exec/command.rs:19-24)
  pub fn new(null_separator: bool) -> Self
- sym:41391 CommandSet.new (src/exec/mod.rs:36-49)
  pub fn new<I, T, S>(input: I) -> Result<CommandSet> where I: IntoIterator<Item = T>, T: IntoIterator<Item = S>, S: AsRef<str>,
- sym:41397 CommandBuilder.new (src/exec/mod.rs:136-162)
  fn new(template: &CommandTemplate, limit: usize) -> io::Result<Self>
- sym:41403 CommandTemplate.new (src/exec/mod.rs:220-256)
  fn new<I, S>(input: I, mode: ExecutionMode) -> Result<CommandTemplate> where I: IntoIterator<Item = S>, S: AsRef<str>,
- sym:41507 PathUrl.new (src/hyperlink.rs:8-10)
  pub(crate) fn new(path: &Path) -> Option<PathUrl>
- sym:41694 test_symlink_and_absolute_path (tests/tests.rs:1606-1624)
  fn test_symlink_and_absolute_path()
- sym:41697 test_symlink_and_full_path_abs_path (tests/tests.rs:1669-1686)
  fn test_symlink_and_full_path_abs_path()
- sym:41614 TestEnv.system_root (tests/testenv/mod.rs:211-214)
  pub fn system_root(&self) -> PathBuf
- sym:41685 test_absolute_path (tests/tests.rs:1297-1330)
  fn test_absolute_path()
- sym:41627 get_absolute_root_path (tests/tests.rs:32-46)
  fn get_absolute_root_path(env: &TestEnv) -> String
- sym:41596 search_str_for_entry_with_absolute_path (src/walk.rs:733-739)
  fn search_str_for_entry_with_absolute_path()
- sym:41482 TestTime.new (src/filter/time.rs:73-76)
  fn new(time: Zoned) -> Self

---
tokens: 1625/4000
