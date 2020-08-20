use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::Path, str::FromStr};
use strum_macros::EnumIter;
use url::Url;

pub(crate) mod settings;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct Config {
    pub(crate) user: UserConfig,
    pub(crate) metadata: Option<MetaData>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub(crate) struct UserConfig {
    pub(crate) username: String,
    pub(crate) tier: PlanTier,
    pub(crate) default_protocol: ConnectionProtocol,
    pub(crate) dns_leak_protection: u8,
    pub(crate) custom_dns: Option<String>,
    pub(crate) check_update_interval: u8,
    pub(crate) killswitch: u8,
    pub(crate) split_tunnel: u8,
    pub(crate) api_domain: Url,
}

impl UserConfig {
    pub fn with_user(username: String) -> UserConfig {
        Self {
            username,
            ..Default::default()
        }
    }
}

/// Do not use this directly. It sets the username to a BS value. Use with_user instead
impl Default for UserConfig {
    /// Do not use this directly. It sets the username to a BS value. Use with_user instead
    fn default() -> Self {
        Self {
            username: "username".to_owned(),
            tier: PlanTier::Free,
            default_protocol: ConnectionProtocol::UDP,
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
#[derive(Debug, Serialize, Deserialize, PartialEq, Copy, Clone, EnumIter)]
pub(crate) enum PlanTier {
    Free,
    Basic,
    Plus,
    Visionary,
}

impl Display for PlanTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            PlanTier::Free => "Free",
            PlanTier::Basic => "Basic",
            PlanTier::Plus => "Plus",
            PlanTier::Visionary => "Visionary",
        };
        write!(f, "{}", string)?;
        Ok(())
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Copy, Clone, EnumIter)]
pub enum ConnectionProtocol {
    TCP,
    UDP,
}

impl Display for ConnectionProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::TCP => "tcp",
            Self::UDP => "udp",
        };
        write!(f, "{}", string)?;
        Ok(())
    }
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
    pub(crate) connected_server: String,
    pub(crate) connected_proto: ConnectionProtocol,
    pub(crate) dns_server: String,
    pub(crate) connected_time: String,
    pub(crate) resolvconf_hash: String,
    pub(crate) last_update_check: String,
}

pub(crate) fn read_config_from_file(p: &Path) -> Result<Config> {
    use ron::de::from_bytes;
    use std::fs::read;
    let file_bytes = read(p).context("Couldn't read config file's bytes")?;
    let config = from_bytes::<Config>(&file_bytes)?;
    Ok(config)
}

pub(crate) fn config() -> Result<Config> {
    use super::constants::CONFIG_FILE;
    read_config_from_file(&**CONFIG_FILE)
}
#[cfg(test)]
mod tests {
    use super::*;
    use ron::{from_str, ser::to_string_pretty};

    const SERIALIZED_STR: &str = r#"(
    user: (
        username: "username",
        tier: 0,
        default_protocol: UDP,
        dns_leak_protection: 0,
        custom_dns: None,
        check_update_interval: 3,
        killswitch: 0,
        split_tunnel: 0,
        api_domain: "https://api.protonvpn.ch/",
    ),
    metadata: None,
)"#;
    #[test]
    fn test_serialization() {
        let conf = Config {
            user: Default::default(),
            metadata: None,
        };
        let conf_as_str = to_string_pretty(&conf, Default::default());
        match conf_as_str {
            Ok(as_str) => {
                // Alas indentation is not stripped from multiline strings.

                assert_eq!(
                    SERIALIZED_STR, as_str,
                    "Expected: {}\nActual:{}",
                    SERIALIZED_STR, as_str
                );
            }
            Err(_) => assert!(false, "Serialization failed"),
        }
    }
    #[test]
    fn test_deserialization() {
        let de = from_str::<Config>(SERIALIZED_STR);

        match de {
            Ok(de_conf) => {
                let conf = Config {
                    user: Default::default(),
                    metadata: None,
                };
                assert_eq!(conf, de_conf);
            }
            Err(_) => assert!(false, "Deser failed"),
        }
    }
}
