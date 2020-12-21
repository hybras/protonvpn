use std::net::Ipv4Addr;

use anyhow::{Context, Result};
use reqwest::{Client, Url, get};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::vpn::constants::VERSION;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ServersResponse {
    code: i32,
    logical_servers: Vec<LogicalServer>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct LogicalServer {
    name: String,
    entry_country: String,
    exit_country: String,
    domain: String,
    tier: u8,
    #[serde(rename = "ID")]
    id: String,
    status: i8,
    servers: Vec<Server>,
    load: i16,
    score: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Server {
    #[serde(rename = "EntryIP")]
    entry_ip: Ipv4Addr,
    #[serde(rename = "ExitIP")]
    exit_ip: Ipv4Addr,
    domain: String,
    #[serde(rename = "ID")]
    id: String,
    status: i8,
}

/// This function adds the protonvpn api headers and deserializes the response.
async fn call_endpoint<T>(url: Url, client: Client) -> Result<T>
where
    T: DeserializeOwned,
{
    client
        .get(url)
        .header("x-pm-appversion", format!("LinuxVPN_{}", VERSION,))
        .header("x-pm-apiversion", "3")
        .header("Accept", "application/vnd.protonmail.v1+json")
        .send()
        .await?
        .json::<T>()
        .await
        .context("couldn't deserialize api response")
}

#[cfg(test)]
mod tests {
    use tokio::test;

    use super::*;

    #[test]
    async fn test_call_endpoint() -> Result<()> {
        let client = Client::new();
        let url = Url::parse("https://api.protonvpn.ch/vpn/logicals")?;
        dbg!(call_endpoint::<ServersResponse>(url, client).await?);
        Ok(())
    }
}
