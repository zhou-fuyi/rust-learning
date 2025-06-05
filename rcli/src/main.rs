use clap::Parser;
// rcli csv -i input.csv -o output.json --header -d ','
use rcli::{process_csv, process_decode, process_encode, process_generate, process_genpass, process_http_serve, process_sign, process_verify, Base64SubCommand, HttpSubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand};
use serde::{Serialize, Deserialize};
use zxcvbn::zxcvbn;

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
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
            // println!("Generating passwords options: {:?}", opts);

            let password = process_genpass(opts.length, opts.uppercase, opts.lowercase, opts.number, opts.symbol)?;
            println!("{}", password);

            let estimate = zxcvbn(&password, &[]);
            eprintln!("Password strength: {:?}", estimate.score());
        },
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                // Call the encode function
                // println!("Encoding: {:?}", opts);
                let encoded = process_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            },
            Base64SubCommand::Decode(opts) => {
                // Call the decode function
                // println!("Decoding: {:?}", opts);
                let decoded = process_decode(&opts.input, opts.format)?;
                let decoded_str = String::from_utf8(decoded)?;
                println!("{}", decoded_str);
            },
        },
        SubCommand::Text(subcmd) => {
            // Here you would call the function to handle text signing or verification
            // For now, we just print the options
            println!("Text subcommand options: {:?}", subcmd);
            match subcmd {
                TextSubCommand::Sign(opts) => {
                    process_sign(&opts.input, &opts.key, opts.format)?;
                },
                TextSubCommand::Verify(opts) => {
                    process_verify(&opts.input, &opts.key, &opts.sig, opts.format)?;
                },
                TextSubCommand::Generate(opts) => {
                    let key = process_generate(opts.format)?;
                    match opts.format {
                        TextSignFormat::Blake3 => {
                            let name = opts.output.join("blake3.key");
                            std::fs::write(name, &key[0])?;
                        },
                        TextSignFormat::Ed25519 => {
                            let name = opts.output;
                            std::fs::write(name.join("ed25519.sk"), &key[0])?;
                            std::fs::write(name.join("ed25519.pk"), &key[1])?;
                        },
                    }
                },
            }
        },
        SubCommand::Http(cmd) => match cmd {
            HttpSubCommand::Serve(opts) => {
                // Here you would call the function to start the HTTP server
                // For now, we just print the options
                println!("Starting HTTP server with options: {:?}", opts);
                process_http_serve(opts.dir, opts.port).await?;
            },
        },
    }
    Ok(())
}
