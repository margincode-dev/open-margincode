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
- sym:38985 FileTypes.should_ignore (src/filetypes.rs:21-42)
  pub fn should_ignore(&self, entry: &dir_entry::DirEntry) -> bool
- sym:39065 extract_time_constraints (src/main.rs:501-524)
- sym:39033 FormatTemplate (src/fmt/mod.rs:46-49)
  pub enum FormatTemplate
- sym:38902 DirEntry.file_name (src/dir_entry.rs:146-160)
- sym:39063 extract_command (src/main.rs:400-415)
- sym:38975 is_socket (src/filesystem.rs:83-85)
- sym:38939 CommandTemplate (src/exec/mod.rs:215-217)
  struct CommandTemplate
- sym:38963 merge_exitcodes (src/exit_codes.rs:46-51)
- sym:38883 Config (src/config.rs:14-136)
- sym:39062 construct_config (src/main.rs:253-398)
- sym:38860 Opts (src/cli.rs:31-694)
- sym:38864 Opts.rg_alias_ignore (src/cli.rs:746-748)
- sym:38865 Opts.max_depth (src/cli.rs:750-752)
- sym:38866 Opts.min_depth (src/cli.rs:754-756)
- sym:38867 Opts.threads (src/cli.rs:758-760)
- sym:38868 Opts.max_results (src/cli.rs:762-766)

---
tokens: 708/800
omitted 31 lower-ranked items (expand an id for detail):
- repo map: .cargo/ .github/ .gitignore CHANGELOG.md CONTRIBUTING.md Cargo.lock Cargo.toml Cross.toml LICENSE-APACHE LICENSE-MIT Makefile README.md (60 files)
- sym:38869 Opts.strip_cwd_prefix (src/cli.rs:768-776)
- sym:38873 ColorWhen (src/cli.rs:829-836)
- sym:38875 HyperlinkWhen (src/cli.rs:849-856)
- sym:38889 DirEntry.path (src/dir_entry.rs:44-49)
- sym:38891 DirEntry.stripped_path (src/dir_entry.rs:61-73)
- sym:38892 DirEntry.into_stripped_path (src/dir_entry.rs:76-82)
- … and 24 more
