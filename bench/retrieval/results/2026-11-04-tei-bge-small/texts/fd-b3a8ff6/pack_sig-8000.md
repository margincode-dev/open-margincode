## Task
# fd-b3a8ff6 · fd-b3a8ff6 · open
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
Workspace: /root/bench/av_tei/ws/rec09fc-fd-b3a8ff6/fd-b3a8ff6
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
- sym:40545 DirEntry.path (src/dir_entry.rs:44-49)
  pub fn path(&self) -> &Path
- sym:40551 DirEntry.depth (src/dir_entry.rs:100-105)
  pub fn depth(&self) -> Option<usize>
- sym:40546 DirEntry.into_path (src/dir_entry.rs:51-56)
  pub fn into_path(self) -> PathBuf
- sym:40775 depth_relative_to_roots (src/walk.rs:670-688)
  fn depth_relative_to_roots(path: &Path, roots: &[PathBuf]) -> usize
- sym:40771 WorkerState.build_walker (src/walk.rs:347-404)
  fn build_walker(&self, paths: &[PathBuf]) -> Result<WalkParallel>
- sym:40550 DirEntry.metadata (src/dir_entry.rs:91-98)
  pub fn metadata(&self) -> Option<&Metadata>
- sym:40539 Config (src/config.rs:14-136)
  pub struct Config
- sym:40773 WorkerState.spawn_senders (src/walk.rs:443-615)
  fn spawn_senders(&self, walker: WalkParallel, roots: &[PathBuf], tx: Sender<Batch>)
- sym:40706 ensure_single_search_pattern_is_not_a_path (src/main.rs:185-216)
  fn ensure_single_search_pattern_is_not_a_path(pattern: &str) -> Result<()>
- sym:40764 ReceiverBuffer.print (src/walk.rs:252-267)
  fn print(&mut self, entry: &DirEntry) -> Result<(), ExitCode>
- sym:40547 DirEntry.stripped_path (src/dir_entry.rs:61-73)
  pub fn stripped_path(&self, config: &Config) -> &Path
- sym:40709 construct_config (src/main.rs:248-393)
  fn construct_config(mut opts: Opts, pattern_regexps: &[String]) -> Result<Config>
- sym:40776 search_str_for_entry (src/walk.rs:690-712)
  fn search_str_for_entry<'a>( entry_path: &'a std::path::Path, full_path_base: Option<&std::path::Path>, ) -> Cow<'a, OsStr>
- sym:40517 Opts.search_paths (src/cli.rs:696-721)
  pub fn search_paths(&self) -> anyhow::Result<Vec<PathBuf>>
- sym:40705 ensure_search_pattern_is_not_a_path (src/main.rs:169-181)
  fn ensure_search_pattern_is_not_a_path(opts: &Opts) -> Result<()>
- sym:40765 ReceiverBuffer.stream (src/walk.rs:270-279)
  fn stream(&mut self) -> Result<(), ExitCode>
- sym:40690 PathUrl (src/hyperlink.rs:5-5)
  pub(crate) struct PathUrl
- sym:40774 WorkerState.scan (src/walk.rs:618-654)
  fn scan(&self, paths: &[PathBuf]) -> Result<ExitCode>
- sym:40869 test_exact_depth_broken_symlink (tests/tests.rs:1333-1364)
  fn test_exact_depth_broken_symlink()
- sym:40867 test_min_depth_broken_symlink_absolute_path (tests/tests.rs:1254-1286)
  fn test_min_depth_broken_symlink_absolute_path()
- sym:40795 TestEnv.create_broken_symlink (tests/testenv/mod.rs:179-197)
  pub fn create_broken_symlink<P: AsRef<Path>>( &mut self, link_path: P, ) -> Result<PathBuf, io::Error>
- sym:40866 test_min_depth_broken_symlink (tests/tests.rs:1216-1246)
  fn test_min_depth_broken_symlink()
- sym:40868 test_max_depth_broken_symlink (tests/tests.rs:1294-1325)
  fn test_max_depth_broken_symlink()
- sym:40880 test_symlink_as_root (tests/tests.rs:1683-1721)
  fn test_symlink_as_root()
- sym:40883 test_symlink_and_full_path (tests/tests.rs:1762-1784)
  fn test_symlink_and_full_path()
- sym:40884 test_symlink_and_full_path_abs_path (tests/tests.rs:1787-1804)
  fn test_symlink_and_full_path_abs_path()
- sym:40862 test_follow_broken_symlink (tests/tests.rs:1112-1135)
  fn test_follow_broken_symlink()
- sym:40882 test_symlink_as_absolute_root (tests/tests.rs:1745-1759)
  fn test_symlink_as_absolute_root()
- sym:40516 Opts (src/cli.rs:32-693)
  pub struct Opts
- sym:40518 Opts.normalize_path (src/cli.rs:723-736)
  fn normalize_path(&self, path: &Path) -> PathBuf
- sym:40519 Opts.no_search_paths (src/cli.rs:738-740)
  pub fn no_search_paths(&self) -> bool
- sym:40552 DirEntry.style (src/dir_entry.rs:107-111)
  pub fn style(&self, ls_colors: &LsColors) -> Option<&Style>
- sym:40553 starts_with_dash (src/dir_entry.rs:114-116)
  fn starts_with_dash(path: &Path) -> bool
- sym:40554 DirEntry.eq (src/dir_entry.rs:120-122)
  fn eq(&self, other: &Self) -> bool
- sym:40556 DirEntry.cmp (src/dir_entry.rs:136-138)
  fn cmp(&self, other: &Self) -> std::cmp::Ordering
- sym:40558 DirEntry.file_name (src/dir_entry.rs:146-160)
  fn file_name(&self) -> OsString
- sym:40566 OutputBuffer.new (src/exec/command.rs:19-24)
  pub fn new(null_separator: bool) -> Self
- sym:40575 CommandSet.new (src/exec/mod.rs:36-49)
  pub fn new<I, T, S>(input: I) -> Result<CommandSet> where I: IntoIterator<Item = T>, T: IntoIterator<Item = S>, S: AsRef<str>,
- sym:40579 CommandSet.execute_batch (src/exec/mod.rs:90-120)
  pub fn execute_batch<I>(&self, paths: I, limit: usize, path_separator: Option<&str>) -> ExitCode where I: Iterator<Item = PathBuf>,
- sym:40581 CommandBuilder.new (src/exec/mod.rs:136-162)
  fn new(template: &CommandTemplate, limit: usize) -> io::Result<Self>
- sym:40583 CommandBuilder.push (src/exec/mod.rs:173-189)
  fn push(&mut self, path: &Path, separator: Option<&str>) -> io::Result<()>
- sym:40587 CommandTemplate.new (src/exec/mod.rs:220-256)
  fn new<I, S>(input: I, mode: ExecutionMode) -> Result<CommandTemplate> where I: IntoIterator<Item = S>, S: AsRef<str>,
- sym:40614 path_absolute_form (src/filesystem.rs:14-21)
  pub fn path_absolute_form(path: &Path) -> io::Result<PathBuf>
- sym:40615 absolute_path (src/filesystem.rs:23-36)
  pub fn absolute_path(path: &Path) -> io::Result<PathBuf>
- sym:40914 test_number_parsing_errors (tests/tests.rs:2811-2827)
  fn test_number_parsing_errors()
- sym:40881 test_symlink_and_absolute_path (tests/tests.rs:1724-1742)
  fn test_symlink_and_absolute_path()
- sym:40832 test_and_pattern_with_forward_slash_is_rejected (tests/tests.rs:425-443)
  fn test_and_pattern_with_forward_slash_is_rejected()
- sym:40864 test_max_depth (tests/tests.rs:1155-1188)
  fn test_max_depth()
- sym:40785 create_working_directory (tests/testenv/mod.rs:29-63)
  fn create_working_directory( directories: &[&'static str], files: &[&'static str], ) -> Result<TempDir, io::Error>
- sym:40830 test_pattern_with_forward_slash_is_rejected (tests/tests.rs:383-397)
  fn test_pattern_with_forward_slash_is_rejected()
- sym:40853 test_respect_ignore_files (tests/tests.rs:888-940)
  fn test_respect_ignore_files()
- sym:40782 search_str_no_base_dir_with_plain_relative_path (src/walk.rs:764-769)
  fn search_str_no_base_dir_with_plain_relative_path()
- sym:40920 test_gitignore_parent (tests/tests.rs:2954-2964)
  fn test_gitignore_parent()
- sym:40900 remove_symlink (tests/tests.rs:2534-2543)
  fn remove_symlink<P: AsRef<Path>>(path: P)
- sym:40901 test_modified_relative (tests/tests.rs:2546-2576)
  fn test_modified_relative()
- sym:40831 test_pattern_with_forward_slash_allowed_with_full_path (tests/tests.rs:410-418)
  fn test_pattern_with_forward_slash_allowed_with_full_path()
- sym:40783 search_str_no_base_dir_with_file_in_current_dir (src/walk.rs:772-777)
  fn search_str_no_base_dir_with_file_in_current_dir()
- sym:40861 test_file_system_boundaries (tests/tests.rs:1074-1109)
  fn test_file_system_boundaries()
- sym:40781 search_str_no_base_dir (src/walk.rs:756-761)
  fn search_str_no_base_dir()
- sym:40903 test_modified_absolute (tests/tests.rs:2589-2603)
  fn test_modified_absolute()

---
tokens: 2717/8000
