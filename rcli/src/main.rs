use clap::Parser;
// rcli csv -i input.csv -o output.json --header -d ','
use rcli::{process_csv, Opts, SubCommand};
use serde::{Serialize, Deserialize};

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


fn main() -> anyhow::Result<()> {
    // println!("Hello, world!");
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                // Default output file name
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?
        },
    }
    Ok(())
}
