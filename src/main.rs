// rcli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
    }

    Ok(())
}
