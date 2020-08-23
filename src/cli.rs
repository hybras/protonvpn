use crate::vpn::util::ConnectionProtocol;
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

pub(crate) mod cli_hooks;
