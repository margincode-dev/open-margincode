## Task
# ripgrep-2d4fa85 · ripgrep-2d4fa85 · open
Goal: ignore: skip empty absolute parent matchers (#3557)

When no ancestor above the walk root has ignore rules, we now skip
constructing an absolute path and checking parent matchers. Repository
boundaries still affect Git ignore rules, even when the repository has
no rules of its own.

On Linux x86-64 against 3fce3b5, warm incremental matching improves
13.4% and replay allocates or reallocates about half as often.
Warm-cache walks of the Ruff and uv checkouts improve 1–4%; the
cached ignored-path control is 3.3% slower.

PR #3557
Workspace: /root/bench/ws/rebf815-ripgrep-2d4fa85/ripgrep-2d4fa85
Changed (0): nothing yet

## Map
- repo map (226 files):
  .cargo/ (1 files)
    config.toml
  .github/ (6 files)
    FUNDING.yml
    ISSUE_TEMPLATE/ (3 files)
    workflows/ (2 files)
  .gitignore
  .ignore
  .nvim.lua
  AI_POLICY.md
  CHANGELOG.md
  CONTRIBUTING.md
  COPYING
  Cargo.lock
  Cargo.toml
  FAQ.md
  GUIDE.md
  HomebrewFormula
  LICENSE-MIT
  README.md
  RELEASE-CHECKLIST.md
  UNLICENSE
  benchsuite/ (30 files)
    benchsuite
    runs/ (29 files)
  build.rs
  ci/ (4 files)
    sha256-releases
    test-complete
    ubuntu-install-packages
    utils.sh
  crates/ (139 files)
    cli/ (12 files)
    core/ (27 files)
    globset/ (11 files)
    grep/ (7 files)
    ignore/ (21 files)
    matcher/ (9 files)
    pcre2/ (7 files)
    printer/ (17 files)
    regex/ (13 files)
    searcher/ (15 files)
  fuzz/ (5 files)
    .gitignore
    Cargo.lock
    Cargo.toml
    README.md
    fuzz_targets/ (1 files)
  pkg/ (3 files)
    brew/ (1 files)
    windows/ (2 files)
  rustfmt.toml
  scripts/ (1 files)
    copy-examples
  tests/ (19 files)
    binary.rs
    data/ (9 files)
    feature.rs
    hay.rs
    json.rs
    macros.rs
    misc.rs
    multiline.rs
    regression.rs
    tests.rs
    util.rs

## Code
- sym:16277 Ignore (crates/ignore/src/dir.rs:94-112)
  pub(crate) struct Ignore
- sym:16278 IgnoreInner (crates/ignore/src/dir.rs:115-166)
  struct IgnoreInner
- sym:16247 incremental_ignored_parent (crates/ignore/benches/workloads.rs:380-388)
  fn incremental_ignored_parent(b: &mut test::Bencher)
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
- sym:16682 Work.add_parents (crates/ignore/src/walk.rs:1591-1600)
  fn add_parents(&mut self) -> Option<Error>
- sym:16282 Ignore.add_parents (crates/ignore/src/dir.rs:192-254)
  pub(crate) fn add_parents<P: AsRef<Path>>( &self, path: P, ) -> (Ignore, Option<Error>)
- sym:16409 IncrementalIgnore.matched_with_errors_ignore (crates/ignore/src/incremental.rs:310-391)
  fn matched_with_errors_ignore( &mut self, relative: &Path, is_dir: bool, errs: &mut PartialErrorBuilder, ) -> Option<IncrementalMatch>
- sym:16346 absolute_parent_matchers_are_cached_across_roots (crates/ignore/src/dir.rs:1427-1447)
  fn absolute_parent_matchers_are_cached_across_roots()
- sym:16443 respects_git_repository_boundaries (crates/ignore/src/incremental.rs:788-806)
  fn respects_git_repository_boundaries()
- sym:16449 compiled_multi_child_matchers_are_not_reused (crates/ignore/src/incremental.rs:901-936)
  fn compiled_multi_child_matchers_are_not_reused()
- sym:14661 DecompressionMatcherBuilder.new (crates/cli/src/decompress.rs:42-44)
  pub fn new() -> DecompressionMatcherBuilder
- sym:14668 DecompressionMatcher.new (crates/cli/src/decompress.rs:167-171)
  pub fn new() -> DecompressionMatcher
- sym:14672 DecompressionReaderBuilder.new (crates/cli/src/decompress.rs:205-207)
  pub fn new() -> DecompressionReaderBuilder
- sym:14678 DecompressionReader.new (crates/cli/src/decompress.rs:352-356)
  pub fn new<P: AsRef<Path>>( path: P, ) -> Result<DecompressionReader, CommandError>
- sym:14710 Error.from (crates/cli/src/human.rs:66-68)
  fn from(size_err: ParseSizeError) -> std::io::Error
- sym:14727 Error.from (crates/cli/src/pattern.rs:40-42)
  fn from(paterr: InvalidPatternError) -> io::Error
- sym:14741 CommandError.from (crates/cli/src/process.rs:66-68)
  fn from(ioerr: io::Error) -> CommandError
- sym:16442 ignored_ancestor_wins (crates/ignore/src/incremental.rs:767-783)
  fn ignored_ancestor_wins()
- sym:16740 no_read_permissions (crates/ignore/src/walk.rs:2658-2673)
  fn no_read_permissions()
- sym:16446 cached_ignored_parent_matches_children (crates/ignore/src/incremental.rs:848-860)
  fn cached_ignored_parent_matches_children()
- sym:16731 gitignore_parent (crates/ignore/src/walk.rs:2436-2446)
  fn gitignore_parent()
- sym:16447 cached_parent_matcher_is_used_for_directory (crates/ignore/src/incremental.rs:863-877)
  fn cached_parent_matcher_is_used_for_directory()
- sym:16347 multi_root_gitignore_order_independent (crates/ignore/src/dir.rs:1460-1516)
  fn multi_root_gitignore_order_independent()
- sym:16752 test_dirs_in_deep (crates/ignore/tests/gitignore_matched_path_or_any_parents_tests.rs:193-318)
  fn test_dirs_in_deep()
- sym:16348 multi_root_custom_ignore_order_independent (crates/ignore/src/dir.rs:1523-1563)
  fn multi_root_custom_ignore_order_independent()
- sym:16444 compiled_matchers_are_reused (crates/ignore/src/incremental.rs:811-830)
  fn compiled_matchers_are_reused()
- sym:16445 cached_allowed_parent_matches_children (crates/ignore/src/incremental.rs:833-845)
  fn cached_allowed_parent_matches_children()
- sym:16434 nested_parent_gitignores (crates/ignore/src/incremental.rs:629-643)
  fn nested_parent_gitignores()
- sym:14689 b (crates/cli/src/escape.rs:85-87)
  fn b(bytes: &'static [u8]) -> Vec<u8>

---
tokens: 1896/4000
