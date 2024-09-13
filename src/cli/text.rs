use std::{fmt::Display, path::PathBuf, str::FromStr};

use crate::{process_generate, process_text_sign, process_text_verify, CmdExecutor};

use super::{verify_file, verify_path};
use clap::Parser;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with a private key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a message with a public key")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a key pair")]
    Generate(TextKeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long)]
    pub sig: String,
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

impl CmdExecutor for TextSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            TextSubCommand::Sign(opts) => {
                _ = process_text_sign(&opts.input, &opts.key, opts.format);
                Ok(())
            }
            TextSubCommand::Verify(opts) => {
                _ = process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig);
                Ok(())
            }
            TextSubCommand::Generate(opts) => {
                _ = process_generate(opts.format);
                Ok(())
            }
        }
    }
}

fn parse_format(s: &str) -> Result<TextSignFormat, anyhow::Error> {
    s.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<TextSignFormat> for String {
    fn from(value: TextSignFormat) -> Self {
        match value {
            TextSignFormat::Blake3 => "blake3".to_string(),
            TextSignFormat::Ed25519 => "ed25519".to_string(),
        }
    }
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextSignFormat::Blake3 => write!(f, "blake3"),
            TextSignFormat::Ed25519 => write!(f, "ed25519"),
        }
    }
}
