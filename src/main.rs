use clap::Parser;
use rcli::{process_base64, process_csv, process_xlsx, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(&opts)?,
        SubCommand::Xlsx(opts) => process_xlsx(&opts)?,
        SubCommand::Base64(cmd) => process_base64(&cmd)?,
    }

    Ok(())
}
