// rcli csv --input input.csv --output output.json --header -d '.'
use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};
use anyhow;
use serde_json;
use std::fs::File;


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    nationality: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

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

fn main() -> anyhow::Result<()>{
    let opts = Opts::parse();
    
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let input_file = File::open(opts.input)?;
            let mut rdr = Reader::from_reader(input_file);
            let mut ret = Vec::with_capacity(128);
            for result in rdr.deserialize() {
                let record: Player = result?;
                println!("{:?}", record);
                ret.push(record);
            }

            // write to output file as json
            let output_file = File::create(opts.output)?;serde_json::to_writer_pretty(output_file, &ret)?;

        }
    }

    Ok(())
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    match std::fs::metadata(filename) {
        Ok(_) => Ok(filename.to_string()),
        Err(_) => Err(format!("Invalid input file: {}", filename))
    }
}
