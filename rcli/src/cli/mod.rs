mod csv;
mod genpass;
mod base64;
mod text;
mod http;

use std::path::{Path, PathBuf};

use clap::Parser;
use self::{csv::CsvOpts, genpass::GenpassOpts};

pub use self::csv::OutputFormat;
pub use self::base64::{Base64SubCommand, Base64Format};
pub use self::text::{TextSubCommand, TextSignFormat};
pub use self::http::{HttpSubCommand};

#[derive(Debug, Parser)]
#[command(name="rcli", author, version, about, long_about = None)]
pub struct Opts {

    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {

    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "Generate random passwords")]
    GenPass(GenpassOpts),

    #[command(subcommand)]
    Base64(Base64SubCommand),

    #[command(subcommand)]
    Text(TextSubCommand),

    #[command(subcommand)]
    Http(HttpSubCommand),
}

fn verify_file(input_file_name: &str) -> Result<String, String> {
    // if input file name is "-" or file is exists
    if input_file_name == "-" || Path::new(input_file_name).exists(){
        Ok(input_file_name.into())
    } else {
        Err(format!("Input file '{}' does not exist.", input_file_name))
    }
}

fn verify_path(path: &str) -> Result<PathBuf, String> {
    // if path is a valid path
    let p = Path::new(path);
    if p.exists() && p.is_dir(){
        Ok(path.into())
    } else {
        Err(format!("Path '{}' does not exist or is not a dir.", path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("non_existent_file.txt"), Err("Input file 'non_existent_file.txt' does not exist.".into()));
        // You can create a temporary file for testing purposes if needed
    }
}
