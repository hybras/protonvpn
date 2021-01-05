use crate::vpn::util::{settings::Settings, UserConfig};
use anyhow::{Context, Result};
use std::io::{BufRead, Write};

pub fn initialize<R, W>(config: &mut UserConfig, r: &mut R, w: &mut W) -> Result<()>
where
	R: BufRead,
	W: Write,
{
	let mut user_settings = Settings::new(config.clone(), w, r);
	user_settings.set_username()?;
	user_settings.set_password()?;
	user_settings.set_tier()?;
	user_settings.set_protocol()?;
	*config = user_settings.into_inner();
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::io::Cursor;

	#[test]
	fn test_initialize() -> Result<()> {
		let mut stdin = Cursor::new("hybras\nshitty password\n2\n1\n");
		let mut stdout = Cursor::new(vec![]);

		let expected = UserConfig {
			username: "hybras".into(),
			password: "shitty password".into(),
			tier: crate::vpn::util::PlanTier::Plus,
			protocol: crate::vpn::util::ConnectionProtocol::TCP,
			..Default::default()
		};
		let mut config = UserConfig::default();

		let _ = initialize(&mut config, &mut stdin, &mut stdout)
			.context("Failed to interact with user to get config")?;
		assert_eq!(expected, config);
		Ok(())
	}
}
