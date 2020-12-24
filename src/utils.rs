use std::{fs::File, io::BufWriter, net::Ipv4Addr};

use anyhow::{Context, Result};
use chrono::{Duration, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use ureq::Agent;
use url::Url;

use crate::vpn::{
    constants::{SERVER_INFO_FILE, VERSION},
    util::Config,
};

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

/// This function adds the protonvpn api headers and deserializes the response. TODO: Add the headers to the agent itself
fn call_endpoint<T>(url: &Url, agent: &Agent) -> Result<T>
where
    T: DeserializeOwned,
{
    agent
        .request_url("GET", &url)
        .set("x-pm-appversion", format!("LinuxVPN_{}", VERSION).as_ref())
        .set("x-pm-apiversion", "3")
        .set("Accept", "application/vnd.protonmail.v1+json")
        .call()?
        .into_json::<T>()
        .context("couldn't deserialize api response")
}

/// Calls the protonvpn api endpoint `vpn/logicals`, and stores the result in the [server info file](#crate::vpn::constants::SERVER_INFO_FILE)
fn pull_server_data(config: &mut Config, agent: &Agent) -> Result<()> {
    // If its been at least 15 mins since the last server check
    if Utc::now() - config.metadata.last_api_pull > Duration::minutes(15) {
        // Download the list of servers
        let response: ServersResponse = call_endpoint(
            {
                config.user.api_domain.set_path("vpn/logicals");
                &config.user.api_domain
            },
            agent,
        )
        .context("failed to call vpn/logicals endpoint")?;

        // Write them to the file
        let server_info_file = BufWriter::new(File::open(SERVER_INFO_FILE.as_path())?);
        serde_json::to_writer(server_info_file, &response)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use ureq::agent;

    use super::*;

    #[test]
    fn test_call_endpoint() -> Result<()> {
        let agent = agent();
        let url = Url::parse("https://api.protonvpn.ch/vpn/logicals")?;
        call_endpoint(&url, &agent)?;
        Ok(())
    }
}
