use super::ConnectOptions::*;
use crate::{
	constants::{OVPN_FILE, OVPN_LOG},
	utils::{config_path, get_servers},
	vpn::{self, util::Config, VpnConnection},
};
use anyhow::{anyhow, Context, Result};
use directories::ProjectDirs;
use vpn::{connect as vpn_connect, util::ConnectionProtocol};

use super::Connect;

/// Conncts to a server based on which variant of ConnectOptions Connect::connection_option is.
pub fn connect(
	flags: &Connect,
	mut config: &mut Config,
	pdir: &ProjectDirs,
) -> Result<VpnConnection> {
	let Connect {
		connection_option,
		protocol,
	} = flags;

	if let Server {
		server: server_name,
	} = connection_option
	{
		server(
			server_name,
			&protocol.unwrap_or(config.user.protocol),
			&mut config,
			pdir,
		)
	} else {
		Err(anyhow!(
			"Operations besides connecting to a server not currently supported"
		))
	}
}

/// Connect to the server specified on the command line
fn server<S>(
	server: S,
	protocol: &ConnectionProtocol,
	mut config: &mut Config,
	pdir: &ProjectDirs,
) -> Result<VpnConnection>
where
	S: AsRef<str>,
{
	let servers = get_servers(&mut config, pdir)?;
	let server = servers
		.iter()
		.find(|s| s.name == server.as_ref())
		.with_context(|| format!("Couldn't find server {}", server.as_ref()))
		.unwrap();
	let log_path = config_path(pdir, OVPN_LOG);
	let config_path = config_path(pdir, OVPN_FILE);
	vpn_connect(server, protocol, &config.user, &config_path, &log_path)
}

#[cfg(test)]
mod tests {

	use chrono::Utc;
	use vpn::util::UserConfig;

	use super::*;
	use crate::utils::project_dirs;
	use crate::vpn::util::MetaData;

	// #[test]
	fn test_server() -> Result<()> {
		let pdir = project_dirs();

		let mut config = Config {
			user: UserConfig::new("".into(), "".into()),
			connection_info: None,
			metadata: MetaData {
				resolvconf_hash: None,
				last_api_pull: Utc::now(),
			},
		};

		let mut connection = server(
			String::from("US-FREE#1"),
			&ConnectionProtocol::UDP,
			&mut config,
			&pdir,
		)?;
		let res = connection.openvpn_process.wait()?;
		Ok(assert!(res.success()))
	}
}
