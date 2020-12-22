use anyhow::{Context, Result};
use confy::load;
use protonvpn::{
    cli::{cli_hooks::*, CliOptions},
    vpn::constants::APP_NAME,
    vpn::util::Config,
};
use std::io::{stdin, stdout, Write};
use structopt::StructOpt;
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
                Init => {
                    initialize(&mut config.user, &mut in_lock, &mut out_lock)?;
                    confy::store(APP_NAME, config.user)
                        .context("Couldn't store your configuration")?;
                }
                Connect {
                    connection_option: _,
                    protocol: _,
                } => {}
                Reconnect => {}
                Disconnect => {}
                Status => {}
                Configure => {
                    configure(&mut config.user, &mut in_lock, &mut out_lock)?;
                    confy::store(APP_NAME, config.user)
                        .context("Couldn't store your configuration")?;
                }
                Refresh => {}
                Examples => {}
            };
            Ok(())
        }
        Err(_) => {
            match opt {
                Init => {
                    initialize(&mut Default::default(), &mut in_lock, &mut out_lock)?;
                }
                _ => writeln!(
                    &mut out_lock,
                    "Unable to load your profile. Try running `protonvpn init` again."
                )?,
            };
            Ok(())
        }
    }
}
