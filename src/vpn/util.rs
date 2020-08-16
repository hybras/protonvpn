use crate::cli::ConnectionProtocol;
use url::{ParseError, Url};

struct Config {
    user: UserConfig,
    metadata: Option<MetaData>
}

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

impl Default for UserConfig {

    // Do not use this directly. It sets the username to a BS value. Use with_user instead
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
            api_domain: Url::parse("https://api.protonvpn.ch").unwrap(),
        }
    }
}

struct MetaData {
    connected_server: String,
    connected_proto: ConnectionProtocol,
    dns_server: String,
    connected_time: String,
    resolvconf_hash: String,
    last_update_check: String,
}
