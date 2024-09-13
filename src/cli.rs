mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use anyhow::anyhow;
use enum_dispatch::enum_dispatch;
use std::path::{Path, PathBuf};

pub use base64::*;
pub use csv::*;
pub use genpass::*;
pub use http::*;
pub use text::*;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, name = "base64", about = "Encode/Decode Base64")]
    Base64(Base64SubCommand),
    #[command(subcommand, name = "text", about = "Sign/Verify text")]
    Text(TextSubCommand),
    #[command(subcommand, name = "http", about = "HTTP client")]
    Http(HttpSubCommand),
}

fn verify_file(filename: &str) -> Result<String, String> {
    if filename == "-" {
        return Ok(filename.to_string());
    }
    match std::fs::metadata(filename) {
        Ok(_) => Ok(filename.to_string()),
        Err(_) => Err(format!("Invalid input file: {}", filename)),
    }
}

fn verify_path(path: &str) -> anyhow::Result<PathBuf> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err(anyhow!("Invalid path: {}", path))
    }
}
