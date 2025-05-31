use csv::Reader;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    // #[serde(rename = "Name")]
    name: String,
    // #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    // #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8
}


pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {

    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record: Player = result?;
        ret.push(record);
    }
    // Convert to JSON and write to output file
    let json = serde_json::to_string_pretty(&ret)?;
    std::fs::write(output, json)?;

    Ok(())
}
