use anyhow::{Context, Result};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, net::Ipv4Addr, str::FromStr};
use strum_macros::{Display, EnumIter};
use url::Url;
pub mod settings;

/// Holds all application state
///
/// Holds current connection information and settings for the current (only) user
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Config {
    /// Actual config info
    pub user: UserConfig,
    /// Current connection information
    pub connection_info: Option<ConnectionInfo>,
    /// Random extra info
    pub metadata: MetaData,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UserConfig {
    pub(crate) username: Option<String>,
    pub(crate) tier: PlanTier,
    pub(crate) protocol: ConnectionProtocol,
    /// A recommended security setting that enables using Proton VPN's dns servers, or your own. In other words, don't use the dns servers from your operating system / internet service provider
    pub(crate) dns_leak_protection: bool,
    /// This setting is only referenced if the dns_leak_protection is enabled
    pub(crate) custom_dns: Vec<Ipv4Addr>,
    pub(crate) check_update_interval: u8,
    pub(crate) killswitch: u8,
    pub(crate) split_tunnel: u8,
    pub(crate) api_domain: Url,
}

/// Creates unusable initial state. Must set the username field (is initially None)
impl Default for UserConfig {
    fn default() -> Self {
        Self {
            username: None,
            tier: PlanTier::Free,
            protocol: ConnectionProtocol::UDP,
            dns_leak_protection: true,
            custom_dns: Vec::with_capacity(3),
            check_update_interval: 3,
            killswitch: 0,
            split_tunnel: 0,
            api_domain: Url::parse("https://api.protonvpn.ch")
                .context("Failed to parse protonvpn api url")
                .unwrap(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Copy, Clone, EnumIter, Display)]
pub(crate) enum PlanTier {
    Free,
    Basic,
    Plus,
    Visionary,
}

/// Order here is used to indicate default option: UDP
#[derive(Debug, Serialize, Deserialize, PartialEq, Copy, Clone, EnumIter, Display)]
pub enum ConnectionProtocol {
    UDP,
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MetaData {
    pub(crate) resolvconf_hash: Option<String>,
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ConnectionInfo {
    pub(crate) server: String,
    pub(crate) protocol: ConnectionProtocol,
    pub(crate) dns_server: String,
    pub(crate) connected_time: String,
}
