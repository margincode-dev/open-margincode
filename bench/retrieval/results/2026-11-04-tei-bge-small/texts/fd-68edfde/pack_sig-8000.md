## Task
# fd-68edfde · fd-68edfde · open
Goal: Use a struct-like variant for broken symlinks and test max/exact depth

Address review feedback. Make DirEntryInner::BrokenSymlink a struct-like
variant with named path and depth fields, so the recorded depth is clear
at every use site. Add regression tests that a broken symlink is filtered
by --max-depth and --exact-depth the same way a normal entry is, both
through the real directory tree and through a followed symlink.
Workspace: /root/bench/av_tei/ws/rec09fc-fd-68edfde/fd-68edfde
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
- sym:40950 DirEntryInner (src/dir_entry.rs:12-17)
  enum DirEntryInner
- sym:40953 DirEntry.broken_symlink (src/dir_entry.rs:36-42)
  pub fn broken_symlink(path: PathBuf, depth: usize) -> Self
- sym:40960 DirEntry.depth (src/dir_entry.rs:100-105)
  pub fn depth(&self) -> Option<usize>
- sym:40954 DirEntry.path (src/dir_entry.rs:44-49)
  pub fn path(&self) -> &Path
- sym:40955 DirEntry.into_path (src/dir_entry.rs:51-56)
  pub fn into_path(self) -> PathBuf
- sym:40959 DirEntry.metadata (src/dir_entry.rs:91-98)
  pub fn metadata(&self) -> Option<&Metadata>
- sym:40958 DirEntry.file_type (src/dir_entry.rs:84-89)
  pub fn file_type(&self) -> Option<FileType>
- sym:40967 DirEntry.file_name (src/dir_entry.rs:146-160)
  fn file_name(&self) -> OsString
- sym:40930 Opts.max_depth (src/cli.rs:747-749)
  pub fn max_depth(&self) -> Option<usize>
- sym:40951 DirEntry (src/dir_entry.rs:20-24)
  pub struct DirEntry
- sym:40952 DirEntry.normal (src/dir_entry.rs:28-34)
  pub fn normal(e: ignore::DirEntry) -> Self
- sym:40948 Config (src/config.rs:14-136)
  pub struct Config
- sym:40931 Opts.min_depth (src/cli.rs:751-753)
  pub fn min_depth(&self) -> Option<usize>
- sym:41155 WorkerResult (src/walk.rs:40-45)
  pub enum WorkerResult
- sym:41099 PathUrl (src/hyperlink.rs:5-5)
  pub(crate) struct PathUrl
- sym:41041 FileTypes.should_ignore (src/filetypes.rs:21-42)
  pub fn should_ignore(&self, entry: &dir_entry::DirEntry) -> bool
- sym:41172 ReceiverBuffer.poll (src/walk.rs:198-249)
  fn poll(&mut self) -> Result<(), ExitCode>
- sym:41040 FileTypes (src/filetypes.rs:8-18)
  pub struct FileTypes
- sym:41115 ensure_single_search_pattern_is_not_a_path (src/main.rs:185-216)
  fn ensure_single_search_pattern_is_not_a_path(pattern: &str) -> Result<()>
- sym:41180 WorkerState.build_walker (src/walk.rs:347-404)
  fn build_walker(&self, paths: &[PathBuf]) -> Result<WalkParallel>
- sym:41117 check_path_separator_length (src/main.rs:234-246)
  fn check_path_separator_length(path_separator: Option<&str>) -> Result<()>
- sym:40925 Opts (src/cli.rs:32-693)
  pub struct Opts
- sym:41185 search_str_for_entry (src/walk.rs:690-712)
  fn search_str_for_entry<'a>( entry_path: &'a std::path::Path, full_path_base: Option<&std::path::Path>, ) -> Cow<'a, OsStr>
- sym:40940 HyperlinkWhen (src/cli.rs:846-853)
  pub enum HyperlinkWhen
- sym:41182 WorkerState.spawn_senders (src/walk.rs:443-615)
  fn spawn_senders(&self, walker: WalkParallel, roots: &[PathBuf], tx: Sender<Batch>)
- sym:41173 ReceiverBuffer.print (src/walk.rs:252-267)
  fn print(&mut self, entry: &DirEntry) -> Result<(), ExitCode>
- sym:41184 depth_relative_to_roots (src/walk.rs:670-688)
  fn depth_relative_to_roots(path: &Path, roots: &[PathBuf]) -> usize
- sym:41114 ensure_search_pattern_is_not_a_path (src/main.rs:169-181)
  fn ensure_search_pattern_is_not_a_path(opts: &Opts) -> Result<()>
- sym:41275 test_min_depth_broken_symlink (tests/tests.rs:1216-1246)
  fn test_min_depth_broken_symlink()
- sym:41287 test_symlink_as_root (tests/tests.rs:1605-1643)
  fn test_symlink_as_root()
- sym:41276 test_min_depth_broken_symlink_absolute_path (tests/tests.rs:1254-1286)
  fn test_min_depth_broken_symlink_absolute_path()
- sym:41273 test_max_depth (tests/tests.rs:1155-1188)
  fn test_max_depth()
- sym:41204 TestEnv.create_broken_symlink (tests/testenv/mod.rs:179-197)
  pub fn create_broken_symlink<P: AsRef<Path>>( &mut self, link_path: P, ) -> Result<PathBuf, io::Error>
- sym:41277 test_exact_depth (tests/tests.rs:1290-1299)
  fn test_exact_depth()
- sym:40961 DirEntry.style (src/dir_entry.rs:107-111)
  pub fn style(&self, ls_colors: &LsColors) -> Option<&Style>
- sym:40975 OutputBuffer.new (src/exec/command.rs:19-24)
  pub fn new(null_separator: bool) -> Self
- sym:40984 CommandSet.new (src/exec/mod.rs:36-49)
  pub fn new<I, T, S>(input: I) -> Result<CommandSet> where I: IntoIterator<Item = T>, T: IntoIterator<Item = S>, S: AsRef<str>,
- sym:40990 CommandBuilder.new (src/exec/mod.rs:136-162)
  fn new(template: &CommandTemplate, limit: usize) -> io::Result<Self>
- sym:40996 CommandTemplate.new (src/exec/mod.rs:220-256)
  fn new<I, S>(input: I, mode: ExecutionMode) -> Result<CommandTemplate> where I: IntoIterator<Item = S>, S: AsRef<str>,
- sym:41100 PathUrl.new (src/hyperlink.rs:8-10)
  pub(crate) fn new(path: &Path) -> Option<PathUrl>
- sym:41157 Batch.new (src/walk.rs:54-58)
  fn new() -> Self
- sym:41163 BatchSender.new (src/walk.rs:82-88)
  fn new(tx: Sender<Batch>, limit: usize) -> Self
- sym:41169 ReceiverBuffer.new (src/walk.rs:153-171)
  fn new(state: &'a WorkerState, rx: Receiver<Batch>, stdout: W) -> Self
- sym:41178 WorkerState.new (src/walk.rs:318-328)
  fn new(patterns: Vec<Regex>, config: Config) -> Self
- sym:41290 test_symlink_and_full_path (tests/tests.rs:1684-1706)
  fn test_symlink_and_full_path()
- sym:41321 test_number_parsing_errors (tests/tests.rs:2733-2749)
  fn test_number_parsing_errors()
- sym:41271 test_follow_broken_symlink (tests/tests.rs:1112-1135)
  fn test_follow_broken_symlink()
- sym:41291 test_symlink_and_full_path_abs_path (tests/tests.rs:1709-1726)
  fn test_symlink_and_full_path_abs_path()
- sym:41288 test_symlink_and_absolute_path (tests/tests.rs:1646-1664)
  fn test_symlink_and_absolute_path()
- sym:41105 Encoded (src/hyperlink.rs:69-69)
  struct Encoded
- sym:41264 test_no_ignore_vcs_child_dir (tests/tests.rs:961-973)
  fn test_no_ignore_vcs_child_dir()
- sym:41289 test_symlink_as_absolute_root (tests/tests.rs:1667-1681)
  fn test_symlink_as_absolute_root()
- sym:41260 test_no_ignore_parent_inner_git (tests/tests.rs:822-858)
  fn test_no_ignore_parent_inner_git()
- sym:41270 test_file_system_boundaries (tests/tests.rs:1074-1109)
  fn test_file_system_boundaries()
- sym:41193 TestEnv (tests/testenv/mod.rs:14-26)
  pub struct TestEnv
- sym:41075 TestTime.new (src/filter/time.rs:73-76)
  fn new(time: Zoned) -> Self
- sym:41201 TestEnv.new (tests/testenv/mod.rs:148-158)
  pub fn new(directories: &[&'static str], files: &[&'static str]) -> TestEnv

---
tokens: 2367/8000
