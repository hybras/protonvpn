use crate::cli::ConnectionProtocol;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use url::{ParseError, Url};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Config {
    user: UserConfig,
    metadata: Option<MetaData>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct UserConfig {
    username: String,
    tier: u8,
    default_protocol: ConnectionProtocol,
    dns_leak_protection: u8,
    custom_dns: Option<String>,
    check_update_interval: u8,
    killswitch: u8,
    split_tunnel: u8,
    api_domain: Url,
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
            tier: 0,
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
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct MetaData {
    connected_server: String,
    connected_proto: ConnectionProtocol,
    dns_server: String,
    connected_time: String,
    resolvconf_hash: String,
    last_update_check: String,
}

fn read_config_from_file(p: &Path) -> Result<Config> {
    use ron::de::from_bytes;
    use std::fs::read;
    let file_bytes = read(p).context("Couldn't read config file's bytes")?;
    let config = from_bytes::<Config>(&file_bytes)?;
    Ok(config)
}

fn config() -> Result<Config> {
    use super::constants::CONFIG_FILE;
    read_config_from_file(&**CONFIG_FILE)
}
#[cfg(test)]
mod tests {
    use super::*;
    use ron::{from_str, ser::to_string_pretty};

    const SERIALIZED_STR: &str = "(
    user: (
        username: \"username\",
        tier: 0,
        default_protocol: UDP,
        dns_leak_protection: 0,
        custom_dns: None,
        check_update_interval: 3,
        killswitch: 0,
        split_tunnel: 0,
        api_domain: \"https://api.protonvpn.ch/\",
    ),
    metadata: None,
)";
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
