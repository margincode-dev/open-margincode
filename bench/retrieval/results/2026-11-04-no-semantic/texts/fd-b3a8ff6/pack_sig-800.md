## Task
# fd-b3a8ff6 · fd-b3a8ff6-2 · open

## Map
- repo map (59 files):
  .cargo/ (1 files)
  .github/ (7 files)
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
  doc/ (8 files)
  rustfmt.toml
  scripts/ (3 files)
  src/ (22 files)
  tests/ (2 files)

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
tokens: 555/800
