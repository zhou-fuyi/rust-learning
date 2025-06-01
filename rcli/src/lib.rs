mod opts;
mod process;

pub use opts::{Opts, SubCommand, GenpassOpts};
pub use process::{process_csv, process_genpass};
