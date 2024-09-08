use clap::Parser;

// rcli csv --input input.csv --output output.json --header -d '.'
use rcli::{process_csv, process_genpass, Base64SubCommand, Opts, SubCommand};

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
                println!("{:?}", opts)
            }
            Base64SubCommand::Decode(opts) => {
                println!("{:?}", opts)
            }
        },
    }

    Ok(())
}
