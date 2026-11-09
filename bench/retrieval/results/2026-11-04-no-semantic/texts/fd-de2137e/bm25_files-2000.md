### src/filetypes.rs
```rust
use crate::dir_entry;
use crate::filesystem;

use faccess::PathExt;

/// Whether or not to show
#[derive(Default)]
pub struct FileTypes {
    pub files: bool,
    pub directories: bool,
    pub symlinks: bool,
    pub block_devices: bool,
    pub char_devices: bool,
    pub sockets: bool,
    pub pipes: bool,
    pub executables_only: bool,
    pub empty_only: bool,
}

impl FileTypes {
    pub fn should_ignore(&self, entry: &dir_entry::DirEntry) -> bool {
        if let Some(ref entry_type) = entry.file_type() {
            (!self.files && entry_type.is_file())
                || (!self.directories && entry_type.is_dir())
                || (!self.symlinks && entry_type.is_symlink())
                || (!self.block_devices && filesystem::is_block_device(*entry_type))
                || (!self.char_devices && filesystem::is_char_device(*entry_type))
                || (!self.sockets && filesystem::is_socket(*entry_type))
                || (!self.pipes && filesystem::is_pipe(*entry_type))
                || (self.executables_only && !entry.path().executable())
                || (self.empty_only && !filesystem::is_empty(entry))
                || !(entry_type.is_file()
                    || entry_type.is_dir()
                    || entry_type.is_symlink()
                    || filesystem::is_block_device(*entry_type)
                    || filesystem::is_char_device(*entry_type)
                    || filesystem::is_socket(*entry_type)
                    || filesystem::is_pipe(*entry_type))
        } else {
            true
        }
    }
}
```
### src/main.rs
```rust
// needs to be first because it defines a macro
#[macro_use]
mod error;

mod cli;
mod config;
mod dir_entry;
mod exec;
mod exit_codes;
mod filesystem;
mod filetypes;
mod filter;
mod fmt;
mod hyperlink;
mod output;
mod regex_helper;
mod sanitize;
mod walk;

use std::env;
use std::io::IsTerminal;
use std::path::Path;
use std::sync::Arc;

use anyhow::{Context, Result, anyhow, bail};
use clap::{CommandFactory, Parser};
use globset::GlobBuilder;
use lscolors::LsColors;
use regex::bytes::{Regex, RegexBuilder, RegexSetBuilder};

use crate::cli::{ColorWhen, HyperlinkWhen, Opts};
use crate::config::Config;
use crate::exec::CommandSet;
use crate::exit_codes::ExitCode;
use crate::filetypes::FileTypes;
#[cfg(unix)]
use crate::filter::OwnerFilter;
use crate::filter::TimeFilter;
use crate::regex_helper::{pattern_has_uppercase_char, pattern_matches_strings_with_leading_dot};

// We use jemalloc for performance reasons, see https://github.com/sharkdp/fd/pull/481
// FIXME: re-enable jemalloc on macOS, see comment in Cargo.toml file for more infos
// This has to be kept in sync with the Cargo.toml file section that declares a
// dependency on tikv-jemallocator.
#[cfg(all(
    not(windows),
    not(target_os = "android"),
    not(target_os = "macos"),
    not(target_os = "freebsd"),
    not(target_os = "openbsd"),
    not(target_os = "illumos"),
    not(all(target_env = "musl", target_pointer_width = "32")),
    not(target_arch = "riscv64"),
    feature = "use-jemalloc"
))]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

// vivid --color-mode 8-bit generate molokai
const DEFAULT_LS_COLORS: &str = "
