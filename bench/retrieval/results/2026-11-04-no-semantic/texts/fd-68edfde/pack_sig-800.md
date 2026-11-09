## Task
# fd-68edfde · fd-68edfde-2 · open
Goal: Use a struct-like variant for broken symlinks and test max/exact depth

Address review feedback. Make DirEntryInner::BrokenSymlink a struct-like
variant with named path and depth fields, so the recorded depth is clear
at every use site. Add regression tests that a broken symlink is filtered
by --max-depth and --exact-depth the same way a normal entry is, both
through the real directory tree and through a followed symlink.
Workspace: /root/bench/ws/rebf815-fd-68edfde/fd-68edfde-2
Changed (0): nothing yet

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
- sym:40950 DirEntryInner (src/dir_entry.rs:12-17)
  enum DirEntryInner
- sym:40960 DirEntry.depth (src/dir_entry.rs:100-105)
  pub fn depth(&self) -> Option<usize>
- sym:40953 DirEntry.broken_symlink (src/dir_entry.rs:36-42)
  pub fn broken_symlink(path: PathBuf, depth: usize) -> Self
- sym:40954 DirEntry.path (src/dir_entry.rs:44-49)
  pub fn path(&self) -> &Path
- sym:40955 DirEntry.into_path (src/dir_entry.rs:51-56)
  pub fn into_path(self) -> PathBuf
- sym:40958 DirEntry.file_type (src/dir_entry.rs:84-89)
  pub fn file_type(&self) -> Option<FileType>
- sym:40959 DirEntry.metadata (src/dir_entry.rs:91-98)
  pub fn metadata(&self) -> Option<&Metadata>
- sym:40951 DirEntry (src/dir_entry.rs:20-24)
  pub struct DirEntry
- sym:40961 DirEntry.style (src/dir_entry.rs:107-111)
  pub fn style(&self, ls_colors: &LsColors) -> Option<&Style>
- sym:40975 OutputBuffer.new (src/exec/command.rs:19-24)
  pub fn new(null_separator: bool) -> Self
- sym:40984 CommandSet.new (src/exec/mod.rs:36-49)
  pub fn new<I, T, S>(input: I) -> Result<CommandSet> where I: IntoIterator<Item = T>, T: IntoIterator<Item = S>, S: AsRef<str>,
- sym:40990 CommandBuilder.new (src/exec/mod.rs:136-162)
  fn new(template: &CommandTemplate, limit: usize) -> io::Result<Self>

---
tokens: 683/800
