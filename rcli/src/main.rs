use clap::Parser;
// rcli csv -i input.csv -o output.json --header -d ','
use rcli::{process_csv, process_decode, process_encode, process_genpass, Base64SubCommand, Opts, SubCommand};
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
        SubCommand::GenPass(opts) => {
            // Here you would call the function to generate passwords
            // For now, we just print the options
            println!("Generating passwords options: {:?}", opts);

            process_genpass(opts.length, opts.uppercase, opts.lowercase, opts.number, opts.symbol)?;
        },
        SubCommand::Base64(subcmd) => match subcmd {
                Base64SubCommand::Encode(opts) => {
                    // Call the encode function
                    // println!("Encoding: {:?}", opts);
                    process_encode(&opts.input, opts.format)?;
                },
                Base64SubCommand::Decode(opts) => {
                    // Call the decode function
                    // println!("Decoding: {:?}", opts);
                    process_decode(&opts.input, opts.format)?;
                },
            },
    }
    Ok(())
}
