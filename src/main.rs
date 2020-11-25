use crate::cli::cli_hooks::configure;
use crate::cli::CliOptions;
use crate::vpn::constants::APP_NAME;
use anyhow::Result;
use confy::load;
use std::io::{stdin, stdout, Write};
use structopt::StructOpt;
use vpn::util::Config;
use CliOptions::*;

fn main() -> Result<()> {
    let opt = CliOptions::from_args();
    // Get stdio handles. These are passed through the entire program
    let stdin = stdin();
    let mut in_lock = stdin.lock();
    let stdout = stdout();
    let mut out_lock = stdout.lock();

    let config_res = load::<Config>(APP_NAME);
    
    match config_res {
        Ok(mut config) => {
            match opt {
                Init => writeln!(&mut out_lock, "You already have initialized")?,
                Connect {
                    connection_option: _,
                    protocol: _,
                } => {}
                Reconnect => {}
                Disconnect => {}
                Status => {}
                Configure => {
                    configure(&mut config.user, &mut in_lock, &mut out_lock)?;
                }
                Refresh => {}
                Examples => {}
            };
            Ok(())
        }
        Err(_) => {
            match opt {
                Init => {}
                _ => writeln!(
                    &mut out_lock,
                    "Unable to load your profile. Try running `protonvpn init` again."
                )?,
            };
            Ok(())
        }
    }
}

mod cli;
mod vpn;
