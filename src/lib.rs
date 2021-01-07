use crate::{
	cli::{configure, initialize, initialize::project_dirs, CliOptions},
	constants::APP_NAME,
	vpn::util::Config,
};
use anyhow::{Context, Result};
use confy::ConfyError;
use std::io::{BufRead, Write};
use CliOptions::*;

pub mod cli;
pub mod constants;
pub(crate) mod utils;
pub mod vpn;

pub fn main<R, W>(
	opt: CliOptions,
	config_res: Result<Config, ConfyError>,
	mut r: &mut R,
	mut w: &mut W,
) -> Result<()>
where
	R: BufRead,
	W: Write,
{
	let pdir = project_dirs();

	if let Ok(mut config) = config_res {
		match opt {
			Init => {
				initialize(&mut config.user, &pdir, &mut r, &mut w)?;
				confy::store(APP_NAME, config.user).context("Couldn't store your configuration")?;
			}
			Connect(_flags) => {}
			Reconnect => {}
			Disconnect => {}
			Status => {}
			Configure => {
				configure(&mut config.user, &mut r, &mut w)?;
				confy::store(APP_NAME, config.user).context("Couldn't store your configuration")?;
			}
			Refresh => {}
			Examples => {}
		};
	} else {
		if let Init = opt {
			initialize(&mut Default::default(), &pdir, &mut r, &mut w)?;
		} else {
			writeln!(
				&mut w,
				"Unable to load your profile. Try running `protonvpn init` again."
			)?;
		}
	}
	Ok(())
}
