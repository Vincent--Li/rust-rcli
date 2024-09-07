use clap::Parser;
// rcli csv --input input.csv --output output.json --header -d '.'
use csv::Reader;
use rcli::{Opts, SubCommand};
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


