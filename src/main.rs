// rcli csv --input input.csv --output output.json --header -d '.'
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name="rcli", version, author)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name="csv", about = "Convert CSV to JSON")]
    Csv(CsvOpts)
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    input: String,
    #[arg(short, long, default_value = "output.json")]
    output: String,
    #[arg(long, default_value_t = false)]
    header: bool,
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
}

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    match std::fs::metadata(filename) {
        Ok(_) => Ok(filename.to_string()),
        Err(_) => Err(format!("Invalid input file: {}", filename))
    }
}
