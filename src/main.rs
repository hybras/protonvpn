use crate::cli::CliOptions;
use structopt::StructOpt;

fn main() {
    let opt = CliOptions::from_args();
    println!("{:?}", opt);
}

mod cli;
mod vpn;
