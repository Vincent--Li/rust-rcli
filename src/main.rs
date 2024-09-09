use std::fs;

use clap::Parser;

// rcli csv --input input.csv --output output.json --header -d '.'
use rcli::{
    process_csv, process_decode, process_encode, process_generate, process_genpass,
    process_text_sign, process_text_verify, Base64SubCommand, Opts, SubCommand, TextSignFormat,
    TextSubCommand,
};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            println!("{:?}", opts);
            process_genpass(
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
                opts.length,
            )?;
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                println!("{:?}", opts);
                let encoded = process_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                println!("{:?}", opts);
                let decoded = process_decode(&opts.input, opts.format)?;
                println!("{}", String::from_utf8(decoded)?);
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let sign = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", sign);
            }
            TextSubCommand::Verify(opts) => {
                process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
            }
            TextSubCommand::Generate(opts) => {
                let spk = process_generate(opts.format)?;
                match opts.format {
                    TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &spk[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        let name = &opts.output;
                        fs::write(name.join("ed25519.sk"), &spk[0])?;
                        fs::write(name.join("ed25519.pk"), &spk[1])?;
                    }
                };
            }
        },
    }

    Ok(())
}
