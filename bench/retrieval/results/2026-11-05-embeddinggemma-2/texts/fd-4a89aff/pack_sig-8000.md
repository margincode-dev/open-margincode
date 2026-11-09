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
Workspace: /root/bench/av_gemma/ws/rec728b-fd-4a89aff/fd-4a89aff
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
- sym:41367 DirEntry.depth (src/dir_entry.rs:100-105)
  pub fn depth(&self) -> Option<usize>
- sym:41357 DirEntryInner (src/dir_entry.rs:12-17)
  enum DirEntryInner
- sym:41360 DirEntry.broken_symlink (src/dir_entry.rs:36-42)
  pub fn broken_symlink(path: PathBuf, depth: usize) -> Self
- sym:41355 Config (src/config.rs:14-136)
  pub struct Config
- sym:41501 FormatTemplate.replace_separator (src/fmt/mod.rs:147-196)
  fn replace_separator<'a>(path: &'a OsStr, path_separator: Option<&str>) -> Cow<'a, OsStr>
- sym:41592 search_str_for_entry (src/walk.rs:676-698)
  fn search_str_for_entry<'a>( entry_path: &'a std::path::Path, full_path_base: Option<&std::path::Path>, ) -> Cow<'a, OsStr>
- sym:41335 Opts.no_search_paths (src/cli.rs:738-740)
  pub fn no_search_paths(&self) -> bool
- sym:41333 Opts.search_paths (src/cli.rs:696-721)
  pub fn search_paths(&self) -> anyhow::Result<Vec<PathBuf>>
- sym:41518 run (src/main.rs:75-112)
  fn run() -> Result<ExitCode>
- sym:41499 FormatTemplate.parse (src/fmt/mod.rs:58-107)
  pub fn parse(fmt: &str) -> Self
- sym:41374 DirEntry.file_name (src/dir_entry.rs:146-160)
  fn file_name(&self) -> OsString
- sym:41587 WorkerState.build_walker (src/walk.rs:347-404)
  fn build_walker(&self, paths: &[PathBuf]) -> Result<WalkParallel>
- sym:41430 path_absolute_form (src/filesystem.rs:14-21)
  pub fn path_absolute_form(path: &Path) -> io::Result<PathBuf>
- sym:41366 DirEntry.metadata (src/dir_entry.rs:91-98)
  pub fn metadata(&self) -> Option<&Metadata>
- sym:41361 DirEntry.path (src/dir_entry.rs:44-49)
  pub fn path(&self) -> &Path
- sym:41693 test_symlink_as_root (tests/tests.rs:1565-1603)
  fn test_symlink_as_root()
- sym:41682 test_min_depth_broken_symlink (tests/tests.rs:1216-1246)
  fn test_min_depth_broken_symlink()
- sym:41695 test_symlink_as_absolute_root (tests/tests.rs:1627-1641)
  fn test_symlink_as_absolute_root()
- sym:41696 test_symlink_and_full_path (tests/tests.rs:1644-1666)
  fn test_symlink_and_full_path()
- sym:41697 test_symlink_and_full_path_abs_path (tests/tests.rs:1669-1686)
  fn test_symlink_and_full_path_abs_path()
- sym:41596 search_str_for_entry_with_absolute_path (src/walk.rs:733-739)
  fn search_str_for_entry_with_absolute_path()
- sym:41685 test_absolute_path (tests/tests.rs:1297-1330)
  fn test_absolute_path()
- sym:41694 test_symlink_and_absolute_path (tests/tests.rs:1606-1624)
  fn test_symlink_and_absolute_path()
- sym:41680 test_max_depth (tests/tests.rs:1155-1188)
  fn test_max_depth()
- sym:41611 TestEnv.create_broken_symlink (tests/testenv/mod.rs:179-197)
  pub fn create_broken_symlink<P: AsRef<Path>>( &mut self, link_path: P, ) -> Result<PathBuf, io::Error>
- sym:41678 test_follow_broken_symlink (tests/tests.rs:1112-1135)
  fn test_follow_broken_symlink()
- sym:41716 test_modified_absolute (tests/tests.rs:2471-2485)
  fn test_modified_absolute()
- sym:41589 WorkerState.spawn_senders (src/walk.rs:443-615)
  fn spawn_senders(&self, walker: WalkParallel, roots: &[PathBuf], tx: Sender<Batch>)
- sym:41686 test_implicit_absolute_path (tests/tests.rs:1334-1349)
  fn test_implicit_absolute_path()
- sym:41614 TestEnv.system_root (tests/testenv/mod.rs:211-214)
  pub fn system_root(&self) -> PathBuf
- sym:41683 test_exact_depth (tests/tests.rs:1250-1259)
  fn test_exact_depth()
- sym:41594 search_str_for_entry_with_relative_path (src/walk.rs:715-721)
  fn search_str_for_entry_with_relative_path()
- sym:41681 test_min_depth (tests/tests.rs:1192-1209)
  fn test_min_depth()
- sym:41661 test_full_path (tests/tests.rs:667-678)
  fn test_full_path()
- sym:41687 test_normalized_absolute_path (tests/tests.rs:1353-1369)
  fn test_normalized_absolute_path()
- sym:41627 get_absolute_root_path (tests/tests.rs:32-46)
  fn get_absolute_root_path(env: &TestEnv) -> String
- sym:41723 test_max_results (tests/tests.rs:2595-2627)
  fn test_max_results()
- sym:41645 test_multi_file_with_missing (tests/tests.rs:340-375)
  fn test_multi_file_with_missing()
- sym:41677 test_file_system_boundaries (tests/tests.rs:1074-1109)
  fn test_file_system_boundaries()
- sym:41643 test_empty_pattern (tests/tests.rs:295-312)
  fn test_empty_pattern()
- sym:41692 test_no_extension (tests/tests.rs:1537-1561)
  fn test_no_extension()
- sym:41377 dash_prefixed_paths_detected (src/dir_entry.rs:177-181)
  fn dash_prefixed_paths_detected()
- sym:41595 search_str_for_entry_strips_dot_prefix (src/walk.rs:724-730)
  fn search_str_for_entry_strips_dot_prefix()
- sym:41628 get_test_env_with_abs_path (tests/tests.rs:49-53)
  fn get_test_env_with_abs_path(dirs: &[&'static str], files: &[&'static str]) -> (TestEnv, String)
- sym:41597 search_str_no_base_dir (src/walk.rs:742-747)
  fn search_str_no_base_dir()
- sym:41722 test_base_directory (tests/tests.rs:2546-2592)
  fn test_base_directory()
- sym:41601 create_working_directory (tests/testenv/mod.rs:29-63)
  fn create_working_directory( directories: &[&'static str], files: &[&'static str], ) -> Result<TempDir, io::Error>
- sym:41734 test_hyperlink (tests/tests.rs:2849-2864)
  fn test_hyperlink()

---
tokens: 2060/8000
