use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;

use crate::opts::OutputFormat;

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

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let input_file = File::open(input)?;
    let mut rdr = Reader::from_reader(input_file);
    let mut ret = Vec::with_capacity(128);
    let headers = rdr.headers()?.clone();
    for result in rdr.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        OutputFormat::Toml => {
            todo!()
        }
    };

    // write to output file as json
    std::fs::write(output, content)?;

    Ok(())
}
