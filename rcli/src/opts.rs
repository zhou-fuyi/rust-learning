use std::path::Path;

use clap::Parser;

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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    /// Input file
    #[arg(short, long, value_name = "INPUT", value_parser = verify_input_file)]
    pub input: String,

    /// Output file
    #[arg(short, long, value_name = "OUTPUT", default_value = "output.json")]
    pub output: String,

    /// Use header row
    #[arg(long, default_value_t = true)]
    pub header: bool,

    /// Delimiter character
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}

fn verify_input_file(input_file_name: &str) -> Result<String, String> {
    if Path::new(input_file_name).exists() {
        Ok(input_file_name.to_string())
    } else {
        Err(format!("Input file '{}' does not exist.", input_file_name))
    }
}
