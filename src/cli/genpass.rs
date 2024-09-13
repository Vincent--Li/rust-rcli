use clap::Parser;

use crate::{process_genpass, CmdExecutor};

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    // length, uppercase, lowercase, number, symbol
    #[arg(short, long, default_value_t = 16)]
    pub length: usize,
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    #[arg(long, default_value_t = true)]
    pub number: bool,
    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

impl CmdExecutor for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("{:?}", self);
        process_genpass(
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbol,
            self.length,
        )?;
        Ok(())
    }
}
