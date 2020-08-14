use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub enum CliOptions {
    Init,
    Connect {
        protocol: ConnectionProtocol,
        connection_option: ConnectOptions,
    },
    Reconnect,
    Disconnect,
    Status,
    Configure,
    Refresh,
    Examples,
}

#[derive(Debug, StructOpt)]
struct Connect {
    #[structopt(default_value)]
    protocol: ConnectionProtocol,
    connection_option: ConnectOptions,
}

#[derive(Debug, StructOpt)]
enum ConnectOptions {
    Fastest,
    CountryCode(String),
    SecureCore,
    P2P,
    Tor,
    Random,
    Server(String),
}

#[derive(Debug)]
pub enum ConnectionProtocol {
    TCP,
    UDP,
}

impl ToString for ConnectionProtocol {
    fn to_string(&self) -> String {
        match self {
            Self::TCP => "tcp",
            Self::UDP => "udp",
        }
        .into()
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
