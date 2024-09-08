mod base64;
mod csv;
mod genpass;

pub use base64::*;
pub use csv::*;
pub use genpass::*;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, name = "base64", about = "Encode/Decode Base64")]
    Base64(Base64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if filename == "-" {
        return Ok(filename.to_string());
    }
    match std::fs::metadata(filename) {
        Ok(_) => Ok(filename.to_string()),
        Err(_) => Err(format!("Invalid input file: {}", filename)),
    }
}
