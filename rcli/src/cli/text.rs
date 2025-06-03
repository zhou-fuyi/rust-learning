use std::{fmt, path::{PathBuf}, str::FromStr};

use clap::Parser;

use crate::cli::verify_path;

use super::verify_file;


#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign a text file or string")]
    Sign(TextSignOpts),
    /// Decode a base64 string
    #[command(about = "Verify a signed text file or string")]
    Verify(TextVerifyOpts),

    #[command(about = "Generate a text file or string")]
    Generate(TextGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_name = "KEY", value_parser = verify_file)]
    pub key: String,
    #[arg(long, value_parser = prese_format, default_value = "blake3")]
    pub format: TextSignFormat,
}
#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_name = "KEY", value_parser = verify_file)]
    pub key: String,
    #[arg(long, value_parser = prese_format, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(short, long)]
    pub sig: String,
}

#[derive(Debug, Parser)]
pub struct TextGenerateOpts {
    #[arg(long, value_parser = prese_format, default_value = "blake3")]
    pub format: TextSignFormat,

    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,

}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn prese_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Unsupported text sign format: '{}'", s)),
        }
    }
}
impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}
impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
