use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
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

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let input_file = File::open(input)?;
    let mut rdr = Reader::from_reader(input_file);
    let mut ret = Vec::with_capacity(128);
    for result in rdr.deserialize() {
        let record: Player = result?;
        println!("{:?}", record);
        ret.push(record);
    }

    // write to output file as json
    let output_file = File::create(output)?;
    serde_json::to_writer_pretty(output_file, &ret)?;

    Ok(())
}
