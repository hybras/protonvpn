use crate::vpn::util::{Config, ConnectionProtocol};
use anyhow::Result;
use std::io::{stdin, stdout, BufReader};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub enum CliOptions {
    /// Initialize a ProtonVPN profile.
    Init,
    /// Connect to a ProtonVPN server.
    Connect {
        /// See ConnectOptions for more info
        #[structopt(subcommand, name = "mode")]
        connection_option: ConnectOptions,
        /// Determine the protocol (UDP or TCP).
        #[structopt(long, short)]
        protocol: Option<ConnectionProtocol>,
    },
    /// Reconnect the currently active session or connect to the last connected server.
    Reconnect,
    /// Disconnect the current session.
    Disconnect,
    /// Print information about the current session.
    Status,
    /// Show connection status.
    Configure,
    /// Refresh OpenVPN configuration and server data.
    Refresh,
    /// Print some example commands.
    Examples,
}

#[derive(Debug, StructOpt)]
pub enum ConnectOptions {
    /// Select the fastest ProtonVPN server.
    Fastest,
    /// Determine the country for fastest connect.
    CountryCode {
        cc: String,
    },
    /// Connect to the fastest Secure-Core server.
    SecureCore,
    /// Connect to the fastest torrent server.
    P2P,
    /// Connect to the fastest Tor server.
    Tor,
    /// Select a random ProtonVPN server.
    Random,
    Server {
        server: String,
    },
}

mod cli_hooks;

impl CliOptions {
    /// TODO:This needs to be moved to main
    pub fn do_shit(self) -> Result<()> {
        use crate::vpn::constants::APP_NAME;
        use cli_hooks::configure;
        use confy::{load, store};
        use CliOptions::*;
        use std::io::Write;

        let mut stdin = BufReader::new(stdin());
        let out = stdout();
        let mut stdout = out.lock();
        let config = load::<Config>(APP_NAME);
        match config {
            Ok(mut config) => {
                match self {
                    Init => writeln!(&mut stdout, "You already have initialized")?,
                    Connect {
                        connection_option: _,
                        protocol: _,
                    } => {}
                    Reconnect => {}
                    Disconnect => {}
                    Status => {}
                    Configure => {
                        configure(&mut config.user, &mut stdin, &mut stdout)?;
                        store(APP_NAME, config)?;
                    }
                    Refresh => {}
                    Examples => {}
                };
                Ok(())
            }
            Err(_) => {
                match self {
                    Init => {}
                    _ => {
                        writeln!(&mut stdout, "Unable to load your profile. Try running `protonvpn init` again.")?
                    }
                };
                Ok(())
            }
        }
    }
}
