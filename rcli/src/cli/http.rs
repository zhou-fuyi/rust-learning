use std::path::PathBuf;
use clap::Parser;
use crate::cli::verify_path;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, default_value = "8080", help = "Port to serve on")]
    pub port: u16,

    #[arg(short, long, default_value = ".", value_parser = verify_path, help = "Directory to serve")]
    pub dir: PathBuf,

    #[arg(long, help = "Enable directory listing")]
    pub list: bool,
}
