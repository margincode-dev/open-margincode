mod jj;
pub mod patch;
/// `.av/sparse`'s rules, public so another `Vcs` (Margincode Cloud's engine) leaves the same
/// paths off disk.
pub mod sparse;

pub use jj::{ExecBit, JjVcs};
