## Task
# fd-4a89aff · fd-4a89aff-2 · open

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
- sym:41591 depth_relative_to_roots (src/walk.rs:665-674)
```rust
fn depth_relative_to_roots(path: &Path, roots: &[PathBuf]) -> usize {
    let components = path.components().count();
    let root_components = roots
        .iter()
        .filter(|root| path.starts_with(root))
        .map(|root| root.components().count())
        .max()
        .unwrap_or(0);
    components.saturating_sub(root_components)
}
```
- sym:41357 DirEntryInner (src/dir_entry.rs:12-17)
```rust
enum DirEntryInner {
    Normal(ignore::DirEntry),
    // Broken symlinks are surfaced by the walker as errors that carry no depth,
    // so we record the depth (relative to the search root) at creation time.
    BrokenSymlink(PathBuf, usize),
}
```
- sym:41360 DirEntry.broken_symlink (src/dir_entry.rs:36-42)
```rust
    pub fn broken_symlink(path: PathBuf, depth: usize) -> Self {
        Self {
            inner: DirEntryInner::BrokenSymlink(path, depth),
            metadata: OnceCell::new(),
            style: OnceCell::new(),
        }
    }
```
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
- sym:41682 test_min_depth_broken_symlink (tests/tests.rs:1216-1246)
  fn test_min_depth_broken_symlink()
- sym:41695 test_symlink_as_absolute_root (tests/tests.rs:1627-1641)
  fn test_symlink_as_absolute_root()

---
tokens: 685/800
