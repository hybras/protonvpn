use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use strum_macros::{Display, EnumIter};
use url::Url;

pub mod settings;

/// Holds all application state
///
/// Holds current connection information and settings for the current(only) user
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub(crate) struct Config {
    /// Actual config info
    pub(crate) user: UserConfig,
    /// Current connection information
    pub(crate) metadata: Option<MetaData>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub(crate) struct UserConfig {
    pub(crate) username: Option<String>,
    pub(crate) tier: PlanTier,
    pub(crate) protocol: ConnectionProtocol,
    /// TODO: Once the inner working of the official cli are understood, convert this to enum
    pub(crate) dns_leak_protection: u8,
    pub(crate) custom_dns: Option<String>,
    pub(crate) check_update_interval: u8,
    pub(crate) killswitch: u8,
    pub(crate) split_tunnel: u8,
    pub(crate) api_domain: Url,
}

/// Creates unusable initial state. Must set the username field (is intially None)
impl Default for UserConfig {
    fn default() -> Self {
        Self {
            username: None,
            tier: PlanTier::Free,
            protocol: ConnectionProtocol::UDP,
            dns_leak_protection: 0,
            custom_dns: None,
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
pub(crate) struct MetaData {
    pub(crate) server: String,
    pub(crate) protocol: ConnectionProtocol,
    pub(crate) dns_server: String,
    pub(crate) connected_time: String,
    pub(crate) resolvconf_hash: String,
    pub(crate) last_update_check: String,
}
