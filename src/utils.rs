use std::{
	fs::File,
	io::{BufReader, BufWriter},
	net::Ipv4Addr,
};

use anyhow::{Context, Result};
use chrono::{Duration, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

use crate::{
	constants::{SERVER_INFO_FILE, VERSION},
	vpn::util::{Config, PlanTier},
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ServersResponse {
	/// No idea what this field is
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
pub(crate) struct Server {
	#[serde(rename = "EntryIP")]
	pub(crate) entry_ip: Ipv4Addr,
	#[serde(rename = "ExitIP")]
	pub(crate) exit_ip: Ipv4Addr,
	pub(crate) domain: String,
	#[serde(rename = "ID")]
	pub(crate) id: String,
	pub(crate) status: i8,
}

#[derive(Serialize, Deserialize, Debug)]
struct IpInfo {
	#[serde(rename = "IP")]
	ip: Ipv4Addr,
	#[serde(rename = "ISP")]
	isp: String,
}

/// This function adds the protonvpn api headers and deserializes the response.
fn call_endpoint<T>(url: &Url) -> Result<T>
where
	T: DeserializeOwned,
{
	ureq::get(url.as_str())
		.set("x-pm-appversion", format!("LinuxVPN_{}", VERSION).as_ref())
		.set("x-pm-apiversion", "3")
		.set("Accept", "application/vnd.protonmail.v1+json")
		.call()?
		.into_json::<T>()
		.context("couldn't deserialize api response")
}

/// Calls the protonvpn api endpoint `vpn/logicals`, and stores the result in the [server info file](#crate::vpn::constants::SERVER_INFO_FILE). Returns servers that are available to the user are currently up.
fn get_servers(config: &mut Config) -> Result<Vec<LogicalServer>> {
	// If its been at least 15 mins since the last server check
	let now = Utc::now();
	let mut servers_resp: ServersResponse;
	if now - config.metadata.last_api_pull > Duration::minutes(15) {
		// Download the list of servers
		servers_resp = call_endpoint({
			config.user.api_domain.set_path("vpn/logicals");
			&config.user.api_domain
		})
		.context("failed to call vpn/logicals endpoint")?;

		config.metadata.last_api_pull = now;

		// Write them to the file
		let server_info_file = BufWriter::new(File::create(SERVER_INFO_FILE.as_path())?);
		serde_json::to_writer(server_info_file, &servers_resp)?;
	} else {
		let server_info_file = BufReader::new(File::open(SERVER_INFO_FILE.as_path())?);
		servers_resp = serde_json::from_reader(server_info_file)?;
	}
	servers_resp
		.logical_servers
		.retain(|it| PlanTier::from(it.tier) <= config.user.tier && it.status == 1);
	Ok(servers_resp.logical_servers)
}

/// Return the current public IP Address
fn ip_info(config: &Config) -> Result<IpInfo> {
	let mut url = config.user.api_domain.clone();
	url.set_path("/vpn/location");
	let resp = call_endpoint::<IpInfo>(&url)?;
	Ok(resp)
}

#[cfg(test)]
mod tests {

	use super::*;

	#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
	struct Test {}

	#[test]
	fn test_call_endpoint() {
		let url = Url::parse("https://api.protonvpn.ch").unwrap();
		let t = call_endpoint::<Test>(&url);
		assert!(t.is_err());
	}

	#[test]
	fn test_ip_info() -> Result<()> {
		let _ip_info = ip_info(&mut Default::default())?;
		Ok(())
	}
}
