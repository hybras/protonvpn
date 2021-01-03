use std::{
	fs::File,
	io::{BufRead, BufReader, BufWriter, Write},
	net::Ipv4Addr,
	path::Path,
	process::{Child, Command},
};

use anyhow::{Context, Result};
use askama::Template;
use tempfile::{NamedTempFile, TempPath};
use util::{ConnectionProtocol, UserConfig};

use crate::{
	constants::{OVPN_FILE, OVPN_LOG},
	utils::Server,
};

pub mod util;

#[derive(Template)] // this will generate the code...
#[template(path = "openvpn_template.j2")]
struct OpenVpnConfig {
	openvpn_protocol: ConnectionProtocol,
	serverlist: Vec<Ipv4Addr>,
	openvpn_ports: Vec<usize>,
	split: bool,
	ip_nm_pairs: Vec<IpNm>,
	ipv6_disabled: bool,
}

/// An IPv4 address and a netmask.
struct IpNm {
	ip: Ipv4Addr,
	nm: Ipv4Addr,
}

fn create_openvpn_config<R, W>(
	servers: &Vec<Ipv4Addr>,
	protocol: &ConnectionProtocol,
	ports: &Vec<usize>,
	split_tunnel: &bool,
	split_tunnel_file: Option<R>,
	output_file: &mut W,
) -> Result<()>
where
	R: BufRead,
	W: Write,
{
	let mut ip_nm_pairs = vec![];

	if *split_tunnel {
		if let Some(split_tunnel_file) = split_tunnel_file {
			for line in split_tunnel_file.lines() {
				let line = line.context("line unwrap")?;
				// TODO String.split_once() once stabilized
				let tokens = line.splitn(2, "/").collect::<Vec<_>>();
				let ip_nm = match tokens.as_slice() {
					[ip, nm] => IpNm {
						ip: ip.parse()?,
						nm: nm.parse()?,
					},
					[ip] => IpNm {
						ip: ip.parse()?,
						nm: "255.255.255.255".parse()?,
					},
					_ => {
						continue;
					}
				};
				ip_nm_pairs.push(ip_nm);
			}
		}
	}

	let ovpn_conf = OpenVpnConfig {
		openvpn_protocol: *protocol,
		serverlist: servers.clone(),
		openvpn_ports: ports.clone(),
		split: *split_tunnel,
		ip_nm_pairs,
		// TODO check if ipv6 is actually disabled
		ipv6_disabled: false,
	};

	let rendered = ovpn_conf.render().context("render template")?;
	let mut out = BufWriter::new(output_file);
	write!(out, "{}", rendered).context("Failed write")?;
	Ok(())
}

fn connect_helper(
	server: &Server,
	protocol: &ConnectionProtocol,
	passfile: &Path,
	config: &Path,
	log: &Path,
) -> Result<Child> {
	create_openvpn_config::<BufReader<File>, File>(
		&vec![server.entry_ip],
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
		.arg(passfile)
		.arg("--dev")
		.arg("proton0")
		.arg("--dev-type")
		.arg("tun")
		.stdout(stdout)
		.stderr(stderr)
		.spawn()
		.context("couldn't spawn openvpn")?;

	Ok(cmd)
}

fn connect(server: &Server, protocol: &ConnectionProtocol, config: &UserConfig) -> Result<Child> {
	let pass_path = create_passfile(config)?;
	connect_helper(server, protocol, &pass_path, &OVPN_FILE, &OVPN_LOG)
}

fn create_passfile(config: &UserConfig) -> Result<TempPath> {
	let f = NamedTempFile::new()?;
	let mut buf = BufWriter::new(f);
	let client_suffix = "plc";

	write!(
		buf,
		"{}+{}\n{}\n",
		config.username.as_deref().unwrap(),
		client_suffix,
		config.password.as_deref().unwrap()
	)?;

	let (f, path) = buf.into_inner()?.into_parts();
	let mut perms = f.metadata()?.permissions();
	perms.set_readonly(true);
	f.set_permissions(perms)?;

	Ok(path)
}

#[cfg(test)]
mod tests {
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
	fn test_connect() {
		let res = connect(
			&Server {
				entry_ip: Ipv4Addr::new(108, 59, 0, 40),
				exit_ip: Ipv4Addr::new(0, 0, 0, 0),
				domain: "".into(),
				id: "".into(),
				status: 1,
			},
			&ConnectionProtocol::UDP,
		);
		println!("{:?}", res);
		assert!(res.is_ok());
	}
}
