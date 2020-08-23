use crate::cli::CliOptions;
use anyhow::Result;
use structopt::StructOpt;
fn main() -> Result<()> {
    let opt = CliOptions::from_args();
    opt.do_shit()?;
    Ok(())
}

mod cli;
mod vpn;
