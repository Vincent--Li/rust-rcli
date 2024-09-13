use std::path::PathBuf;

use crate::{process_http_serve, CmdExecutor};

use super::verify_path;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(long, default_value_t = 8081)]
    pub port: u16,
}

impl CmdExecutor for HttpSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            HttpSubCommand::Serve(opts) => {
                _ = process_http_serve(opts.dir, opts.port);
                Ok(())
            }
        }
    }
}
