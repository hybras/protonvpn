use anyhow::{Context, Result};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::{net::Ipv4Addr, str::FromStr};
use strum_macros::{Display, EnumIter};
use url::Url;

/// Holds all application state
///
/// Holds current connection information and settings for the current (only) user
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
	/// Actual config info
	pub user: UserConfig,
	/// Current connection information
	pub connection_info: Option<ConnectionInfo>,
	/// Random extra info
	pub metadata: MetaData,
}

/// Holds all user settings. See the docs on each field to learn more.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UserConfig {
	pub(crate) username: String,
	pub(crate) password: String,
	pub(crate) tier: PlanTier,
	pub(crate) protocol: ConnectionProtocol,
	/// A recommended security setting that enables using Proton VPN's dns servers, or your own. In other words, don't use the dns servers from your operating system / internet service provider
	pub(crate) dns_leak_protection: bool,
	/// This setting is only referenced if the dns_leak_protection is enabled
	pub(crate) custom_dns: Vec<Ipv4Addr>,
	pub(crate) check_update_interval: u8,
	pub(crate) killswitch: u8,
	pub(crate) split_tunnel: bool,
	// Remove this field. It can't change. Its always the default (see impl Default)
	pub(crate) api_domain: Url,
}

impl UserConfig {
	/// Constructor: All other params assume default values. Use the setters in [super::settings] for mutation
	pub fn new(username: String, password: String) -> Self {
		Self {
			username,
			password,
			..Default::default()
		}
	}
}

/// Creates unusable initial state. Must set the username and password fields (is initially None)
impl Default for UserConfig {
	fn default() -> Self {
		Self {
			username: String::new(),
			password: String::new(),
			tier: PlanTier::Free,
			protocol: ConnectionProtocol::UDP,
			dns_leak_protection: true,
			custom_dns: Vec::with_capacity(3),
			check_update_interval: 3,
			killswitch: 0,
			split_tunnel: false,
			api_domain: Url::parse("https://api.protonvpn.ch")
				.context("Failed to parse protonvpn api url")
				.unwrap(),
		}
	}
}
#[derive(
	Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, EnumIter, Display,
)]
pub(crate) enum PlanTier {
	Free,
	Basic,
	Plus,
	Visionary,
}

impl From<u8> for PlanTier {
	fn from(u: u8) -> Self {
		use PlanTier::*;

		match u {
			0 => Free,
			1 => Basic,
			2 => Plus,
			3 => Visionary,
			_ => panic!("u8 was too big to convert to PlanTier"),
		}
	}
}

/// The connection protocol to use for vpn connections. The default is UDP
#[derive(Debug, Serialize, Deserialize, PartialEq, Copy, Clone, EnumIter, Display)]
pub enum ConnectionProtocol {
	/// Default variant, [User Datagram Protocol](https://www.cloudflare.com/learning/ddos/glossary/user-datagram-protocol-udp/)
	UDP,
	/// [Transmission Control Protocol](https://www.cloudflare.com/learning/ddos/glossary/tcp-ip/)
	TCP,
}

impl Default for ConnectionProtocol {
	fn default() -> Self {
		Self::UDP
	}
}

impl FromStr for ConnectionProtocol {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_ascii_lowercase().as_str() {
			"udp" => Ok(Self::UDP),
			"tcp" => Ok(Self::TCP),
			_ => Err("String must be udp or tcp".into()),
		}
	}
}

/// Random extra info used by the application.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MetaData {
	pub(crate) resolvconf_hash: Option<String>,
	/// Time of the last call to the `/vpn/logicals` api endpoint. See [get_servers()](crate::utils::get_servers).
	///
	/// If config could not be found, this defaults to 0 milliseconds, as a sort of Time::MIN
	pub(crate) last_api_pull: DateTime<Utc>,
}

impl Default for MetaData {
	fn default() -> Self {
		Self {
			resolvconf_hash: None,
			last_api_pull: Utc.timestamp_millis(0),
		}
	}
}

/// Information about the current vpn connection.
#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionInfo {
	pub(crate) server_id: String,
	pub(crate) protocol: ConnectionProtocol,
	pub(crate) dns_server: String,
	pub(crate) connected_time: String,
}
