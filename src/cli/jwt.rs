use std::str::FromStr;

use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::CmdExecutor;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum JwtSubCommand {
    #[command(about = "Sign a jwt")]
    Sign(JwtSignOpts),
    #[command(about = "Verify a Jwt")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(long, default_value = "sub")]
    pub sub: String,
    #[arg(long, default_value = "aud")]
    pub aud: String,
    #[arg(long, default_value = "14d", value_parser = parse_exp_format)]
    pub exp: JwtExpFormat,
    #[arg(long, default_value = "HS256")]
    pub alg: String,
    #[arg(long, default_value = "key.json")]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(long)]
    pub token: String,
    #[arg(long)]
    pub key: String,
}

#[derive(Debug, Clone, Copy)]
pub enum JwtExpFormat {
    Days(u8),
    Minute(u8),
    Second(u8),
}

impl FromStr for JwtExpFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (num, unit) = s.split_at(s.len() - 1);
        let num = num
            .parse::<u8>()
            .map_err(|e| format!("Invalid number: {}", e))?;
        match unit {
            "d" => Ok(JwtExpFormat::Days(num)),
            "m" => Ok(JwtExpFormat::Minute(num)),
            "s" => Ok(JwtExpFormat::Second(num)),
            _ => Err("Invalid unit".to_string()),
        }
    }
}

fn parse_exp_format(s: &str) -> Result<JwtExpFormat, String> {
    JwtExpFormat::from_str(s)
}

impl CmdExecutor for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("{:?}", self);
        Ok(())
    }
}

impl CmdExecutor for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("{:?}", self);
        Ok(())
    }
}
