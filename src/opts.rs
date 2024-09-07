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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(long, default_value_t = false)]
    pub header: bool,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    match std::fs::metadata(filename) {
        Ok(_) => Ok(filename.to_string()),
        Err(_) => Err(format!("Invalid input file: {}", filename)),
    }
}
