## Task
# ripgrep-2d4fa85 · ripgrep-2d4fa85 · open

## Map
- repo map: .cargo/ .github/ .gitignore .ignore .nvim.lua AI_POLICY.md CHANGELOG.md CONTRIBUTING.md COPYING Cargo.lock Cargo.toml FAQ.md (226 files)

## Code
- sym:16277 Ignore (crates/ignore/src/dir.rs:94-112)
  pub(crate) struct Ignore
- sym:16703 check_symlink_loop (crates/ignore/src/walk.rs:2038-2059)
  fn check_symlink_loop( ig_parent: &Ignore, child_path: &Path, child_depth: usize, ) -> Result<(), Error>
- sym:16291 Ignore.matched_ignore (crates/ignore/src/dir.rs:544-681)
  pub(crate) fn matched_ignore<'a>( &'a self, path: &Path, is_dir: bool, ) -> Match<IgnoreMatch<'a>>
- sym:16654 Walk.next (crates/ignore/src/walk.rs:1190-1261)
  fn next(&mut self) -> Option<Result<DirEntry, Error>>
- sym:16399 IncrementalIgnore (crates/ignore/src/incremental.rs:57-68)
  pub struct IncrementalIgnore
- sym:16262 incremental_gitignore_only (crates/ignore/benches/workloads.rs:563-576)
  fn incremental_gitignore_only(b: &mut test::Bencher)
- sym:16367 Gitignore.matched_path_or_any_parents (crates/ignore/src/gitignore.rs:234-256)
  pub fn matched_path_or_any_parents<P: AsRef<Path>>( &self, path: P, is_dir: bool, ) -> Match<&Glob>
- sym:16278 IgnoreInner (crates/ignore/src/dir.rs:115-166)
  struct IgnoreInner
- sym:16682 Work.add_parents (crates/ignore/src/walk.rs:1591-1600)
  fn add_parents(&mut self) -> Option<Error>
- sym:16346 absolute_parent_matchers_are_cached_across_roots (crates/ignore/src/dir.rs:1427-1447)
- sym:14710 Error.from (crates/cli/src/human.rs:66-68)
- sym:14727 Error.from (crates/cli/src/pattern.rs:40-42)
- sym:14741 CommandError.from (crates/cli/src/process.rs:66-68)
- sym:14742 Error.from (crates/cli/src/process.rs:72-79)
- sym:16443 respects_git_repository_boundaries (crates/ignore/src/incremental.rs:788-806)
- sym:16442 ignored_ancestor_wins (crates/ignore/src/incremental.rs:767-783)
- sym:16740 no_read_permissions (crates/ignore/src/walk.rs:2658-2673)
- sym:16446 cached_ignored_parent_matches_children (crates/ignore/src/incremental.rs:848-860)
- sym:16449 compiled_multi_child_matchers_are_not_reused (crates/ignore/src/incremental.rs:901-936)

---
tokens: 683/800
