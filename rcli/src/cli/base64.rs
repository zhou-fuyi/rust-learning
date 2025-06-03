use std::{fmt, str::FromStr};

use clap::Parser;

use super::verify_file;


#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    /// Encode a string to base64
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),
    /// Decode a base64 string
    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    /// The string to encode
    #[arg(short, long, value_name = "STRING", value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = prese_base64_format, default_value = "standard")]
    pub format: Base64Format,
}
#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    /// The base64 string to decode
    #[arg(short, long, value_name = "BASE64_STRING", value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = prese_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn prese_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Unsupported base64 format: '{}'", s)),
        }
    }

}

impl From<Base64Format> for &str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }

}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }

}
