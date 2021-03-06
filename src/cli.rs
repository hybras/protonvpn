use crate::vpn::util::ConnectionProtocol;
use structopt::StructOpt;

mod configure;
mod connect;
mod initialize;

pub use configure::configure;
pub use connect::connect;
pub use initialize::initialize;

/// An enum for all the cli's subcommands
///
/// The enum options correspond one to one with ALL features of the cli (planned or otherwise). Read the docs on each option to learn what they do.
#[derive(StructOpt, Debug)]
pub enum CliOptions {
	/// Initialize a ProtonVPN profile.
	Init,
	/// Connect to a ProtonVPN server.
	Connect(Connect),
	/// Reconnect the currently active session or connect to the last connected server.
	Reconnect,
	/// Disconnect the current session.
	Disconnect,
	/// Print information about the current session.
	Status,
	/// Edit one setting Repeatedly call this if you need to change many settings.
	Configure,
	/// Refresh OpenVPN configuration and server data.
	Refresh,
	/// Print some example commands.
	Examples,
}

/// The struct contains the different variants of the connect subcommand (takes many subcommands as well.)
#[derive(StructOpt, Debug)]
pub struct Connect {
	/// See ConnectOptions for more info
	#[structopt(subcommand, name = "mode")]
	connection_option: ConnectOptions,
	/// Determine the protocol (UDP or TCP).
	#[structopt(long, short)]
	protocol: Option<ConnectionProtocol>,
}

/// Each variant of this enum corresponds to a subcommand of the connect subcommand. Each variant has a corresponding submodule that handles that variant.
#[derive(Debug, StructOpt)]
pub enum ConnectOptions {
	/// Select the fastest ProtonVPN server.
	Fastest,
	/// Determine the country for fastest connect.
	CountryCode {
		/// 2 letter country code, like US or IN. See [COUNTRY_CODES](static@crate::constants::COUNTRY_CODES)
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
	/// Select a specific server (must follow the name format)
	Server {
		/// Country code. Needs to match one the following python regexes
		/// - Short: `^((\w\w)(-|#)?(\d{1,3})-?(TOR)?)$`
		///    - Example: UK-03/HK#5-Tor, for normal and tor servers
		/// - Long: `^(((\w\w)(-|#)?([A-Z]{2}|FREE))(-|#)?(\d{1,3})-?(TOR)?)$`
		///    - Example: IS-DE-01, for Secure-Core/Free/US Servers
		server: String,
	},
}
