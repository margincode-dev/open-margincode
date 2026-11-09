## Task
# ripgrep-7155f64 · ripgrep-7155f64 · open
Goal: ignore: treat dotfiles ending in a dot as hidden (#3564)

`file_name()` returned `None` for any path ending in `.` on Unix,
which was meant to cover the `.` and `..` components but also matched
real names like `.foo.`. The hidden check then found no file name and
treated those files as visible. Only a final component that is exactly
`.` or `..` has no file name now.

PR #3564

## Map
- crates/globset/src/pathutil.rs
- crates/ignore/src/incremental.rs
- crates/ignore/src/pathutil.rs
- crates/ignore/src/walk.rs
- crates/regex/src/literal.rs
- crates/searcher/src/searcher/mod.rs
- crates/cli/src/decompress.rs
- crates/cli/src/pattern.rs
- crates/cli/src/process.rs
- crates/core/flags/hiargs.rs
- crates/core/flags/lowargs.rs
- crates/core/flags/parse.rs
- crates/core/haystack.rs
- crates/core/search.rs
- crates/globset/src/glob.rs
- crates/globset/src/lib.rs
- crates/ignore/examples/walk.rs
  L5: fn main()
  L52: enum DirEntry
  L58: fn path(&self) -> &Path
- repo map: .cargo/ .github/ .gitignore .ignore .nvim.lua AI_POLICY.md CHANGELOG.md CONTRIBUTING.md COPYING Cargo.lock Cargo.toml FAQ.md (224 files)

## Code
- sym:26594 file_name (crates/ignore/src/pathutil.rs:142-171)
  pub(crate) fn file_name<'a, P: AsRef<Path> + ?Sized>( path: &'a P, ) -> Option<&'a OsStr>
- sym:26632 DirEntry.file_name (crates/ignore/src/walk.rs:76-78)
  pub fn file_name(&self) -> &OsStr
- sym:26303 file_name (crates/globset/src/pathutil.rs:9-22)
  pub(crate) fn file_name<'a>(path: &Cow<'a, [u8]>) -> Option<Cow<'a, [u8]>>
- sym:26647 DirEntryInner.file_name (crates/ignore/src/walk.rs:200-207)
  fn file_name(&self) -> &OsStr
- sym:26591 is_hidden_path_only (crates/ignore/src/pathutil.rs:85-91)
  fn is_hidden_path_only(path: &Path) -> bool
- sym:26593 is_file_name (crates/ignore/src/pathutil.rs:123-136)
  pub(crate) fn is_file_name<P: AsRef<Path>>(path: P) -> bool
- sym:26475 IncrementalMatch.none (crates/ignore/src/incremental.rs:456-462)
  fn none(is_dir: bool) -> IncrementalMatch
- sym:27528 InnerLiterals.none (crates/regex/src/literal.rs:96-98)
  pub(crate) fn none() -> InnerLiterals
- sym:27854 BinaryDetection.none (crates/searcher/src/searcher/mod.rs:62-64)
- sym:26382 resolve_git_commondir (crates/ignore/src/dir.rs:1061-1105)
- sym:26660 DirEntryRaw.file_name (crates/ignore/src/walk.rs:314-316)
- sym:26472 IncrementalIgnore.match_path (crates/ignore/src/incremental.rs:393-411)
- sym:25204 Hidden.doc_long (crates/core/flags/defs.rs:2791-2808)
- sym:25848 WithFilenameNo.name_long (crates/core/flags/defs.rs:7467-7469)
- sym:26471 IncrementalIgnore.matched_with_errors_ignore (crates/ignore/src/incremental.rs:310-391)
- sym:24768 DecompressionMatcherBuilder.new (crates/cli/src/decompress.rs:42-44)
- sym:24775 DecompressionMatcher.new (crates/cli/src/decompress.rs:167-171)
- sym:24779 DecompressionReaderBuilder.new (crates/cli/src/decompress.rs:205-207)
- sym:24785 DecompressionReader.new (crates/cli/src/decompress.rs:352-356)
- sym:24846 CommandError.is_empty (crates/cli/src/process.rs:33-38)
- sym:24851 CommandReaderBuilder.new (crates/cli/src/process.rs:90-92)
- sym:24855 CommandReader.new (crates/cli/src/process.rs:195-199)
- sym:25924 State.new (crates/core/flags/hiargs.rs:950-960)
- sym:25930 BinaryDetection (crates/core/flags/hiargs.rs:1152-1155)
  struct BinaryDetection
- sym:25931 BinaryDetection.from_low_args (crates/core/flags/hiargs.rs:1159-1175)
- sym:25932 BinaryDetection.is_none (crates/core/flags/hiargs.rs:1179-1182)
- sym:25970 ContextSeparator.new (crates/core/flags/lowargs.rs:483-491)
- sym:25977 FieldContextSeparator.new (crates/core/flags/lowargs.rs:561-569)
- sym:25981 FieldMatchSeparator.new (crates/core/flags/lowargs.rs:595-603)
- sym:26017 Parser.new (crates/core/flags/parse.rs:176-216)
- sym:26026 FlagMap.new (crates/core/flags/parse.rs:374-387)
- sym:26034 HaystackBuilder.new (crates/core/haystack.rs:22-24)
- sym:26039 Haystack.path (crates/core/haystack.rs:108-114)
- sym:26042 Haystack.is_dir (crates/core/haystack.rs:143-154)
- sym:26072 Config.default (crates/core/search.rs:28-36)
- sym:26075 SearchWorkerBuilder.new (crates/core/search.rs:54-59)
- sym:26126 MatchStrategy.new (crates/globset/src/glob.rs:51-67)
- sym:26128 Glob.as_ref (crates/globset/src/glob.rs:84-86)
- sym:26150 Glob.new (crates/globset/src/glob.rs:283-285)
- sym:26162 GlobBuilder.new (crates/globset/src/glob.rs:574-576)
- sym:26214 GlobSet.is_empty (crates/globset/src/lib.rs:331-333)
- sym:26215 GlobSet.len (crates/globset/src/lib.rs:337-339)
- sym:26224 GlobSet.new (crates/globset/src/lib.rs:461-553)
- sym:26227 GlobSetBuilder.new (crates/globset/src/lib.rs:574-576)
- sym:26232 Candidate.new (crates/globset/src/lib.rs:617-619)

---
tokens: 1806/2000
omitted 11 lower-ranked items (expand an id for detail):
- sym:26233 Candidate.from_bytes (crates/globset/src/lib.rs:629-631)
- sym:26234 Candidate.from_cow (crates/globset/src/lib.rs:633-638)
- sym:26242 LiteralStrategy.new (crates/globset/src/lib.rs:713-715)
- sym:26248 BasenameLiteralStrategy.new (crates/globset/src/lib.rs:745-747)
- sym:26254 ExtensionStrategy.new (crates/globset/src/lib.rs:783-785)
- sym:26278 MultiStrategyBuilder.new (crates/globset/src/lib.rs:1023-1025)
- sym:26283 MultiStrategyBuilder.is_empty (crates/globset/src/lib.rs:1063-1065)
- sym:26285 RequiredExtensionStrategyBuilder.new (crates/globset/src/lib.rs:1074-1076)
- sym:26328 DirEntry.path (crates/ignore/examples/walk.rs:58-63)
- sym:26526 descendants_of_hidden_directories_are_ignored (crates/ignore/src/incremental.rs:1236-1250)
- … and 1 more
