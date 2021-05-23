use std::{
	fs::File,
	io::{BufRead, BufReader, BufWriter, Write},
	net::Ipv4Addr,
	path::Path,
	process::{Child, Command, Stdio},
};

use anyhow::{Context, Result};
use askama::Template;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use tempfile::{NamedTempFile, TempPath};
use util::{ConnectionProtocol, UserConfig};

use crate::utils::LogicalServer;

/// This module declares all the structs that store application state.
pub mod util;

#[derive(Template)] // this will generate the code...
#[template(path = "openvpn_template.j2")]
struct OpenVpnConfig {
	openvpn_protocol: ConnectionProtocol,
	server_list: Vec<Ipv4Addr>,
	openvpn_ports: Vec<usize>,
	/// Whether to use split tunnel or not
	split: bool,
	ip_nm_pairs: Vec<IpNm>,
	/// Use ipv6 and fallback to ipv4, or only use ipv4. Usefull for older devices and networks
	ipv6_disabled: bool,
}

/// An IPv4 address and a netmask.
#[derive(Serialize, Deserialize)]
struct IpNm {
	ip: Ipv4Addr,
	#[serde(default = "IpNm::default_netmask")]
	nm: Ipv4Addr,
}

impl IpNm {
	fn default_netmask() -> Ipv4Addr {
		"255.255.255.255".parse().unwrap()
	}
}

/// Stores information about the current connection. Its purpose to prevent the passfile path from being dropped (upon drop the corresponding file is unlinked/deleted)
pub struct VpnConnection {
	pub(crate) openvpn_process: Child,
	pub(crate) passfile: TempPath,
}

fn create_openvpn_config<R, W>(
	servers: &Vec<Ipv4Addr>,
	protocol: &ConnectionProtocol,
	ports: &Vec<usize>,
	split_tunnel_file: Option<R>,
	output_file: &mut W,
) -> Result<()>
where
	R: BufRead,
	W: Write,
{
	let ip_nm_pairs: Vec<IpNm> = if let Some(split_tunnel_file) = split_tunnel_file {
		from_reader(split_tunnel_file).context(
			"Failed to deserialize split_tunnel_file. Please check that it is valid json",
		)?
	} else {
		vec![]
	};

	let ovpn_conf = OpenVpnConfig {
		openvpn_protocol: *protocol,
		server_list: servers.clone(),
		openvpn_ports: ports.clone(),
		split: split_tunnel_file.is_some(),
		ip_nm_pairs,
		// TODO check if ipv6 is actually disabled
		ipv6_disabled: false,
	};

	let rendered = ovpn_conf
		.render()
		.context("Rendering config file template failed")?;
	let mut out = BufWriter::new(output_file);
	write!(out, "{}", rendered).context("Failed write")?;
	Ok(())
}

fn connect_helper(
	server: &LogicalServer,
	protocol: &ConnectionProtocol,
	passfile: TempPath,
	config: &Path,
	log: &Path,
) -> Result<VpnConnection> {
	create_openvpn_config::<BufReader<File>, File>(
		&server.servers.iter().map(|s| s.entry_ip).collect(),
		protocol,
		&vec![match protocol {
			ConnectionProtocol::TCP => 443,
			ConnectionProtocol::UDP => 1194,
		} as usize],
		&false,
		None,
		&mut File::create(config)?,
	)?;

	let stdout = File::create(log)?;
	let stderr = stdout.try_clone()?;

	let cmd = Command::new("openvpn")
		.arg("--config")
		.arg(config)
		.arg("--auth-user-pass")
		.arg(&passfile)
		.arg("--dev")
		.arg("proton0")
		.arg("--dev-type")
		.arg("tun")
		.stdin(Stdio::null())
		.stdout(stdout)
		.stderr(stderr)
		.spawn()
		.context("couldn't spawn openvpn")?;

	let connection = VpnConnection {
		openvpn_process: cmd,
		passfile,
	};

	Ok(connection)
}

/// This function wraps the helper, first creating the password tempfile and passing it in.
pub fn connect(
	server: &LogicalServer,
	protocol: &ConnectionProtocol,
	user_config: &UserConfig,
	config_path: &Path,
	log_path: &Path,
) -> Result<VpnConnection> {
	let pass_path = create_passfile(user_config)?;
	connect_helper(server, protocol, pass_path, &config_path, &log_path)
}

fn create_passfile(config: &UserConfig) -> Result<TempPath> {
	let f = NamedTempFile::new()?;
	let mut buf = BufWriter::new(f);
	let client_suffix = "plc";

	write!(
		buf,
		"{}+{}\n{}\n",
		config.username, client_suffix, config.password
	)?;

	Ok(buf.into_inner()?.into_temp_path())
}

#[cfg(test)]
mod tests {
	use std::fs::read;

	use super::*;

	#[test]
	fn test_create_ovpn_conf() -> Result<()> {
		let mut output = vec![];

		let res = create_openvpn_config::<BufReader<File>, Vec<u8>>(
			&vec![Ipv4Addr::new(108, 59, 0, 40)],
			&ConnectionProtocol::UDP,
			&vec![1134],
			&false,
			None,
			&mut output,
		)?;
		Ok(res)
	}

	#[test]
	fn test_passfile() -> Result<()> {
		let user = "user";
		let pass = "pass";
		let path = create_passfile(&UserConfig::new(user.into(), pass.into()))?;

		let buf = read(&path)?;
		let s = String::from_utf8(buf)?;
		assert_eq!(s, format!("{}+plc\n{}\n", user, pass));

		let output = Command::new("/usr/bin/cat").arg(&path).output()?;
		assert!(output.status.success());
		Ok(())
	}
}
