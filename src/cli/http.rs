use std::path::PathBuf;

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
