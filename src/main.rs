use anyhow::Result;
use confy::load;
use protonvpn::{cli::CliOptions, constants::APP_NAME, main as main_cli, vpn::util::Config};
use std::io::{stdin, stdout};
use structopt::StructOpt;

fn main() -> Result<()> {
    // Get stdio handles. These are passed through the entire program
    let stdin = stdin();
    let mut in_lock = stdin.lock();
    let stdout = stdout();
    let mut out_lock = stdout.lock();

    let opt = CliOptions::from_args();
    let config = load::<Config>(APP_NAME);

    main_cli(opt, config, &mut in_lock, &mut out_lock)
}
