## Task
# ripgrep-2d4fa85 · ripgrep-2d4fa85-2 · open

## Map
- repo map: .cargo/ .github/ .gitignore .ignore .nvim.lua AI_POLICY.md CHANGELOG.md CONTRIBUTING.md COPYING Cargo.lock Cargo.toml FAQ.md (226 files)

## Code
- sym:16703 check_symlink_loop (crates/ignore/src/walk.rs:2038-2059)
```rust
fn check_symlink_loop(
    ig_parent: &Ignore,
    child_path: &Path,
    child_depth: usize,
) -> Result<(), Error> {
    let hchild = Handle::from_path(child_path).map_err(|err| {
        Error::from(err).with_path(child_path).with_depth(child_depth)
    })?;
    for ig in ig_parent.parents().take_while(|ig| !ig.is_absolute_parent()) {
        let h = Handle::from_path(ig.path()).map_err(|err| {
            Error::from(err).with_path(child_path).with_depth(child_depth)
        })?;
        if hchild == h {
            return Err(Error::Loop {
                ancestor: ig.path().to_path_buf(),
                child: child_path.to_path_buf(),
            }
            .with_depth(child_depth));
        }
    }
    Ok(())
}
```
- sym:16654 Walk.next (crates/ignore/src/walk.rs:1190-1261)
  fn next(&mut self) -> Option<Result<DirEntry, Error>>
- sym:16367 Gitignore.matched_path_or_any_parents (crates/ignore/src/gitignore.rs:234-256)
  pub fn matched_path_or_any_parents<P: AsRef<Path>>( &self, path: P, is_dir: bool, ) -> Match<&Glob>
- sym:16278 IgnoreInner (crates/ignore/src/dir.rs:115-166)
  struct IgnoreInner
- sym:16682 Work.add_parents (crates/ignore/src/walk.rs:1591-1600)
  fn add_parents(&mut self) -> Option<Error>
- sym:14710 Error.from (crates/cli/src/human.rs:66-68)
  fn from(size_err: ParseSizeError) -> std::io::Error
- sym:14727 Error.from (crates/cli/src/pattern.rs:40-42)
  fn from(paterr: InvalidPatternError) -> io::Error
- sym:14741 CommandError.from (crates/cli/src/process.rs:66-68)
  fn from(ioerr: io::Error) -> CommandError
- sym:14742 Error.from (crates/cli/src/process.rs:72-79)
  fn from(cmderr: CommandError) -> io::Error
- sym:16443 respects_git_repository_boundaries (crates/ignore/src/incremental.rs:788-806)
  fn respects_git_repository_boundaries()
- sym:16346 absolute_parent_matchers_are_cached_across_roots (crates/ignore/src/dir.rs:1427-1447)
  fn absolute_parent_matchers_are_cached_across_roots()
- sym:16740 no_read_permissions (crates/ignore/src/walk.rs:2658-2673)

---
tokens: 687/800
