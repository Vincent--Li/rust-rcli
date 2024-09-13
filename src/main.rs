use clap::Parser;

// rcli csv --input input.csv --output output.json --header -d '.'
use rcli::{CmdExecutor, Opts};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    tracing_subscriber::fmt::init();
    opts.cmd.execute().await?;
    Ok(())
}
