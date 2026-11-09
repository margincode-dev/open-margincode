## Task
# ripgrep-b5611fc · ripgrep-b5611fc · open
Goal: ignore: reuse parent repository detection

PR #3556

## Map
- repo map: .cargo/ .github/ .gitignore .ignore .nvim.lua AI_POLICY.md CHANGELOG.md CONTRIBUTING.md COPYING Cargo.lock Cargo.toml FAQ.md (226 files)

## Code
- sym:19645 IgnoreOptions (crates/ignore/src/dir.rs:72-90)
  struct IgnoreOptions
- sym:19651 Ignore.add_parents (crates/ignore/src/dir.rs:192-258)
  pub(crate) fn add_parents<P: AsRef<Path>>( &self, path: P, ) -> (Ignore, Option<Error>)
- sym:18771 NoRequireGit.doc_long (crates/core/flags/defs.rs:4850-4863)
  fn doc_long(&self) -> &'static str
- sym:19650 Ignore.parent (crates/ignore/src/dir.rs:181-186)
  pub(crate) fn parent(&self) -> Option<Ignore>
- sym:19812 respects_git_repository_boundaries (crates/ignore/src/incremental.rs:788-806)
  fn respects_git_repository_boundaries()
- sym:19146 generate_long_flag (crates/core/flags/doc/help.rs:131-208)
  fn generate_long_flag(flag: &dyn Flag, out: &mut String)
- sym:19150 generate_flag (crates/core/flags/doc/man.rs:46-110)
  fn generate_flag(flag: &'static dyn Flag, out: &mut String)
- sym:19390 Glob.as_ref (crates/globset/src/glob.rs:84-86)
  fn as_ref(&self) -> &Glob
- sym:19591 Workspace.write (crates/ignore/benches/workloads.rs:88-92)
  fn write(&self, relative: &str, contents: &str)
- sym:19803 nested_parent_gitignores (crates/ignore/src/incremental.rs:629-643)
- sym:19806 parent_ignore_and_custom_ignore (crates/ignore/src/incremental.rs:679-695)
- sym:20121 test_dirs_in_deep (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:193-318)
- sym:18737 test_no_ignore_parent (crates/core/flags/defs.rs:4667-4677)
- sym:19714 absolute_parent_anchored (crates/ignore/src/dir.rs:1411-1428)
- sym:19814 cached_allowed_parent_matches_children (crates/ignore/src/incremental.rs:833-845)
- sym:19817 compiled_multi_matchers_are_reused (crates/ignore/src/incremental.rs:881-898)
- sym:20100 gitignore_parent (crates/ignore/src/walk.rs:2436-2446)
  fn gitignore_parent()
- sym:19820 parent_loading_can_be_disabled (crates/ignore/src/incremental.rs:958-976)
- sym:20119 test_files_in_deep (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:57-86)

---
tokens: 684/800
