use std::{
	fs::File,
	io::{BufReader, BufWriter},
	net::Ipv4Addr,
	path::PathBuf,
};

use anyhow::{Context, Result};
use chrono::{Duration, Utc};

use directories::ProjectDirs;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

use crate::{
	constants::VERSION,
	vpn::util::{Config, PlanTier},
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ServersResponse {
	/// No idea what this field is
	code: i32,
	logical_servers: Vec<LogicalServer>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LogicalServer {
	pub name: String,
	pub entry_country: String,
	pub exit_country: String,
	pub domain: String,
	pub tier: u8,
	#[serde(rename = "ID")]
	pub id: String,
	pub status: i8,
	pub servers: Vec<Server>,
	pub load: i16,
	pub score: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Server {
	#[serde(rename = "EntryIP")]
	pub entry_ip: Ipv4Addr,
	#[serde(rename = "ExitIP")]
	pub exit_ip: Ipv4Addr,
	pub domain: String,
	#[serde(rename = "ID")]
	pub id: String,
	pub status: i8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IpInfo {
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
pub fn get_servers(config: &mut Config, pdir: &ProjectDirs) -> Result<Vec<LogicalServer>> {
	let file_path = {
		let server_info_file = "serverinfo.json";
		let mut path = pdir.config_dir().to_path_buf();
		path.push(server_info_file);
		path
	};

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

		// Write them to the file
		let server_info_file = BufWriter::new(File::create(file_path)?);
		serde_json::to_writer(server_info_file, &servers_resp)?;

		config.metadata.last_api_pull = now;
	} else {
		let server_info_file = BufReader::new(File::open(file_path)?);
		servers_resp = serde_json::from_reader(server_info_file)?;
	}
	servers_resp
		.logical_servers
		.retain(|it| PlanTier::from(it.tier) <= config.user.tier && it.status == 1);
	Ok(servers_resp.logical_servers)
}

/// Return the current public IP Address
pub fn ip_info(config: &Config) -> Result<IpInfo> {
	let mut url = config.user.api_domain.clone();
	url.set_path("/vpn/location");
	let resp = call_endpoint::<IpInfo>(&url)?;
	Ok(resp)
}

pub fn config_path<S>(pdir: &ProjectDirs, filename: S) -> PathBuf
where
	S: AsRef<str>,
{
	let mut config_path = pdir.config_dir().to_path_buf();
	config_path.push(filename.as_ref());
	config_path
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
