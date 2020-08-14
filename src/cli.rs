use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub enum CliOptions {
    Connect { protocol: ConnectionProtocol },
    Reconnect,
    Disconnect,
    Status,
    Configure,
    Refresh,
    Examples,
}

#[derive(Debug)]
pub enum ConnectionProtocol {
    TCP,
    UDP,
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
