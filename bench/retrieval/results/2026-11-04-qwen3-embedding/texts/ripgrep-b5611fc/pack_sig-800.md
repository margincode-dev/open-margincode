## Task
# ripgrep-b5611fc · ripgrep-b5611fc · open
Goal: ignore: reuse parent repository detection

PR #3556
Workspace: /root/bench/ws/rebf815-ripgrep-b5611fc/ripgrep-b5611fc
Changed (0): nothing yet

## Map
- repo map: .cargo/ .github/ .gitignore .ignore .nvim.lua AI_POLICY.md CHANGELOG.md CONTRIBUTING.md COPYING Cargo.lock Cargo.toml FAQ.md (226 files)

## Code
- sym:19645 IgnoreOptions (crates/ignore/src/dir.rs:72-90)
  struct IgnoreOptions
- sym:18771 NoRequireGit.doc_long (crates/core/flags/defs.rs:4850-4863)
  fn doc_long(&self) -> &'static str
- sym:19682 IgnoreBuilder.parents (crates/ignore/src/dir.rs:946-949)
  pub(crate) fn parents(&mut self, yes: bool) -> &mut IgnoreBuilder
- sym:19650 Ignore.parent (crates/ignore/src/dir.rs:181-186)
  pub(crate) fn parent(&self) -> Option<Ignore>
- sym:19646 Ignore (crates/ignore/src/dir.rs:94-112)
  pub(crate) struct Ignore
- sym:19812 respects_git_repository_boundaries (crates/ignore/src/incremental.rs:788-806)
  fn respects_git_repository_boundaries()
- sym:19146 generate_long_flag (crates/core/flags/doc/help.rs:131-208)
  fn generate_long_flag(flag: &dyn Flag, out: &mut String)
- sym:19150 generate_flag (crates/core/flags/doc/man.rs:46-110)
  fn generate_flag(flag: &'static dyn Flag, out: &mut String)
- sym:19390 Glob.as_ref (crates/globset/src/glob.rs:84-86)
- sym:19591 Workspace.write (crates/ignore/benches/workloads.rs:88-92)
- sym:19820 parent_loading_can_be_disabled (crates/ignore/src/incremental.rs:958-976)
- sym:19713 absolute_parent (crates/ignore/src/dir.rs:1388-1408)
  fn absolute_parent()
- sym:19715 absolute_parent_matchers_are_cached_across_roots (crates/ignore/src/dir.rs:1431-1451)
- sym:20121 test_dirs_in_deep (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:193-318)
- sym:18737 test_no_ignore_parent (crates/core/flags/defs.rs:4667-4677)
- sym:19714 absolute_parent_anchored (crates/ignore/src/dir.rs:1411-1428)
- sym:19803 nested_parent_gitignores (crates/ignore/src/incremental.rs:629-643)
- sym:19817 compiled_multi_matchers_are_reused (crates/ignore/src/incremental.rs:881-898)
- sym:20119 test_files_in_deep (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:57-86)

---
tokens: 684/800
