## Task
# fd-de2137e · fd-de2137e · open
Goal: refactor(filetypes): extract FileTypes construction into impl block

## Map
- src/filetypes.rs
- src/config.rs
- src/main.rs
- src/cli.rs
- src/dir_entry.rs
- src/error.rs
- src/exec/command.rs
- src/exec/job.rs
- src/exec/mod.rs
- src/exit_codes.rs
- src/filesystem.rs
- src/filter/owner.rs
- src/filter/size.rs
- src/filter/time.rs
- src/fmt/mod.rs
- src/hyperlink.rs

## Code
- sym:38984 FileTypes (src/filetypes.rs:8-18)
  pub struct FileTypes
- sym:38872 FileType (src/cli.rs:806-826)
  pub enum FileType
- sym:38971 is_block_device (src/filesystem.rs:63-65)
  pub fn is_block_device(ft: fs::FileType) -> bool
- sym:38928 CommandSet.new (src/exec/mod.rs:36-49)
- sym:38893 DirEntry.file_type (src/dir_entry.rs:84-89)
  pub fn file_type(&self) -> Option<FileType>
- sym:38985 FileTypes.should_ignore (src/filetypes.rs:21-42)
- sym:38902 DirEntry.file_name (src/dir_entry.rs:146-160)
  fn file_name(&self) -> OsString
- sym:39060 build_pattern_regex (src/main.rs:223-237)
- sym:39063 extract_command (src/main.rs:400-415)
- sym:39065 extract_time_constraints (src/main.rs:501-524)
- sym:38932 CommandSet.execute_batch (src/exec/mod.rs:90-120)
- sym:38963 merge_exitcodes (src/exit_codes.rs:46-51)
- sym:39106 Batch.IntoIter (src/walk.rs:66-66)
- sym:38883 Config (src/config.rs:14-136)
  pub struct Config
- sym:39062 construct_config (src/main.rs:253-398)
- sym:38860 Opts (src/cli.rs:31-694)
- sym:38864 Opts.rg_alias_ignore (src/cli.rs:746-748)
- sym:38865 Opts.max_depth (src/cli.rs:750-752)
- sym:38866 Opts.min_depth (src/cli.rs:754-756)

---
tokens: 724/800
omitted 33 lower-ranked items (expand an id for detail):
- repo map: .cargo/ .github/ .gitignore CHANGELOG.md CONTRIBUTING.md Cargo.lock Cargo.toml Cross.toml LICENSE-APACHE LICENSE-MIT Makefile README.md (60 files)
- sym:38867 Opts.threads (src/cli.rs:758-760)
- sym:38868 Opts.max_results (src/cli.rs:762-766)
- sym:38869 Opts.strip_cwd_prefix (src/cli.rs:768-776)
- sym:38873 ColorWhen (src/cli.rs:829-836)
- sym:38875 HyperlinkWhen (src/cli.rs:849-856)
- sym:38889 DirEntry.path (src/dir_entry.rs:44-49)
- sym:38891 DirEntry.stripped_path (src/dir_entry.rs:61-73)
- … and 25 more
