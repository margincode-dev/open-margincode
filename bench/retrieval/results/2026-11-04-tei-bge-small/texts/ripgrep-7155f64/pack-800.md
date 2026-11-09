## Task
# ripgrep-7155f64 · ripgrep-7155f64 · open

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

## Code
- sym:26594 file_name (crates/ignore/src/pathutil.rs:142-171)
  pub(crate) fn file_name<'a, P: AsRef<Path> + ?Sized>( path: &'a P, ) -> Option<&'a OsStr>
- sym:26632 DirEntry.file_name (crates/ignore/src/walk.rs:76-78)
  pub fn file_name(&self) -> &OsStr
- sym:26303 file_name (crates/globset/src/pathutil.rs:9-22)
- sym:26647 DirEntryInner.file_name (crates/ignore/src/walk.rs:200-207)
- sym:26591 is_hidden_path_only (crates/ignore/src/pathutil.rs:85-91)
- sym:26593 is_file_name (crates/ignore/src/pathutil.rs:123-136)
- sym:26475 IncrementalMatch.none (crates/ignore/src/incremental.rs:456-462)
- sym:27528 InnerLiterals.none (crates/regex/src/literal.rs:96-98)
- sym:27854 BinaryDetection.none (crates/searcher/src/searcher/mod.rs:62-64)
- sym:26382 resolve_git_commondir (crates/ignore/src/dir.rs:1061-1105)
- sym:26660 DirEntryRaw.file_name (crates/ignore/src/walk.rs:314-316)
- sym:26472 IncrementalIgnore.match_path (crates/ignore/src/incremental.rs:393-411)
- sym:25204 Hidden.doc_long (crates/core/flags/defs.rs:2791-2808)
- sym:24846 CommandError.is_empty (crates/cli/src/process.rs:33-38)

---
tokens: 719/800
omitted 47 lower-ranked items (expand an id for detail):
- crates/core/search.rs
- crates/globset/src/glob.rs
- crates/globset/src/lib.rs
- crates/ignore/examples/walk.rs
- repo map: .cargo/ .github/ .gitignore .ignore .nvim.lua AI_POLICY.md CHANGELOG.md CONTRIBUTING.md COPYING Cargo.lock Cargo.toml FAQ.md (224 files)
- sym:25848 WithFilenameNo.name_long (crates/core/flags/defs.rs:7467-7469)
- sym:26471 IncrementalIgnore.matched_with_errors_ignore (crates/ignore/src/incremental.rs:310-391)
- sym:24768 DecompressionMatcherBuilder.new (crates/cli/src/decompress.rs:42-44)
- sym:24775 DecompressionMatcher.new (crates/cli/src/decompress.rs:167-171)
- … and 38 more
