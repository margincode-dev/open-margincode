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
Workspace: /root/bench/ws/rebf815-fd-b3a8ff6/fd-b3a8ff6
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
- sym:40551 DirEntry.depth (src/dir_entry.rs:100-105)
  pub fn depth(&self) -> Option<usize>
- sym:40539 Config (src/config.rs:14-136)
  pub struct Config
- sym:40775 depth_relative_to_roots (src/walk.rs:670-688)
  fn depth_relative_to_roots(path: &Path, roots: &[PathBuf]) -> usize
- sym:40773 WorkerState.spawn_senders (src/walk.rs:443-615)
  fn spawn_senders(&self, walker: WalkParallel, roots: &[PathBuf], tx: Sender<Batch>)
- sym:40522 Opts.min_depth (src/cli.rs:751-753)
  pub fn min_depth(&self) -> Option<usize>
- sym:40771 WorkerState.build_walker (src/walk.rs:347-404)
  fn build_walker(&self, paths: &[PathBuf]) -> Result<WalkParallel>
- sym:40546 DirEntry.into_path (src/dir_entry.rs:51-56)
  pub fn into_path(self) -> PathBuf
- sym:40867 test_min_depth_broken_symlink_absolute_path (tests/tests.rs:1254-1286)
  fn test_min_depth_broken_symlink_absolute_path()
- sym:40869 test_exact_depth_broken_symlink (tests/tests.rs:1333-1364)
  fn test_exact_depth_broken_symlink()
- sym:40866 test_min_depth_broken_symlink (tests/tests.rs:1216-1246)
  fn test_min_depth_broken_symlink()
- sym:40868 test_max_depth_broken_symlink (tests/tests.rs:1294-1325)
  fn test_max_depth_broken_symlink()
- sym:40795 TestEnv.create_broken_symlink (tests/testenv/mod.rs:179-197)
  pub fn create_broken_symlink<P: AsRef<Path>>( &mut self, link_path: P, ) -> Result<PathBuf, io::Error>
- sym:40880 test_symlink_as_root (tests/tests.rs:1683-1721)
  fn test_symlink_as_root()
- sym:40862 test_follow_broken_symlink (tests/tests.rs:1112-1135)
  fn test_follow_broken_symlink()
- sym:40882 test_symlink_as_absolute_root (tests/tests.rs:1745-1759)
  fn test_symlink_as_absolute_root()
- sym:40884 test_symlink_and_full_path_abs_path (tests/tests.rs:1787-1804)
  fn test_symlink_and_full_path_abs_path()
- sym:40545 DirEntry.path (src/dir_entry.rs:44-49)
  pub fn path(&self) -> &Path
- sym:40550 DirEntry.metadata (src/dir_entry.rs:91-98)
  pub fn metadata(&self) -> Option<&Metadata>
- sym:40552 DirEntry.style (src/dir_entry.rs:107-111)
  pub fn style(&self, ls_colors: &LsColors) -> Option<&Style>
- sym:40566 OutputBuffer.new (src/exec/command.rs:19-24)
  pub fn new(null_separator: bool) -> Self
- sym:40575 CommandSet.new (src/exec/mod.rs:36-49)
  pub fn new<I, T, S>(input: I) -> Result<CommandSet> where I: IntoIterator<Item = T>, T: IntoIterator<Item = S>, S: AsRef<str>,
- sym:40581 CommandBuilder.new (src/exec/mod.rs:136-162)
  fn new(template: &CommandTemplate, limit: usize) -> io::Result<Self>
- sym:40587 CommandTemplate.new (src/exec/mod.rs:220-256)
  fn new<I, S>(input: I, mode: ExecutionMode) -> Result<CommandTemplate> where I: IntoIterator<Item = S>, S: AsRef<str>,
- sym:40914 test_number_parsing_errors (tests/tests.rs:2811-2827)
  fn test_number_parsing_errors()
- sym:40883 test_symlink_and_full_path (tests/tests.rs:1762-1784)
  fn test_symlink_and_full_path()
- sym:40666 TestTime.new (src/filter/time.rs:73-76)
  fn new(time: Zoned) -> Self

---
tokens: 1521/4000
