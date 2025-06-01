use std::{ fmt, path::Path, str::FromStr};

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

    #[command(name = "genpass", about = "Generate random passwords")]
    GenPass(GenpassOpts),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Json,
    Yaml,
    // Toml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    /// Input file
    #[arg(short, long, value_name = "INPUT", value_parser = verify_input_file)]
    pub input: String,

    /// Output file
    #[arg(short, long, value_name = "OUTPUT")]
    pub output: Option<String>,

    /// Output format
    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    /// Use header row
    #[arg(long, default_value_t = true)]
    pub header: bool,

    /// Delimiter character
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}

#[derive(Debug, Parser)]
pub struct GenpassOpts {
    /// Length of the password
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    /// Use uppercase letters
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

fn verify_input_file(input_file_name: &str) -> Result<String, String> {
    if Path::new(input_file_name).exists() {
        Ok(input_file_name.to_string())
    } else {
        Err(format!("Input file '{}' does not exist.", input_file_name))
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl From<OutputFormat> for &str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            // OutputFormat::Toml => "toml",
        }
    }

}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Use the parse_format function to convert the string to OutputFormat
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            // "toml" => Ok(OutputFormat::Toml),
            _ => Err(anyhow::anyhow!("Unsupported output format: '{}'", s)),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }

}
