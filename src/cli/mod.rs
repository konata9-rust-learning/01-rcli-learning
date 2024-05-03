mod base64_opts;
mod csv_opts;
mod genpass_opts;
mod text_opts;

use std::path::Path;

use csv_opts::CSVOpts;
use genpass_opts::GenPassOpts;

pub use base64_opts::Base64Format;
pub use base64_opts::Base64Subcommand;
pub use csv_opts::OutputFormat;
pub use text_opts::{TextSubcommand, TextSignFormat};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name="rcli", version="0.1.0", author="Rust CLI", about, long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    CSV(CSVOpts),

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),

    #[command(subcommand)]
    Base64(Base64Subcommand),

    #[command(subcommand)]
    Text(TextSubcommand),
}

fn verify_file(filename: &str) -> Result<String, String> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        return Ok(filename.into());
    } else {
        return Err("File not found".into());
    }
}

// UT
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File not found".into()));
    }
}
