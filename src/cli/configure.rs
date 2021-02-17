//! The functions in this module are expected to work. They have been tested by hand, but currently can't be tested programmatically because console doesn't have a testing functionality.

use crate::vpn::{settings::Settings, util::UserConfig};
use anyhow::Result;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

/// Sets and saves new configuration settings, OVERWRITING the old options.
///
/// Reads an int to determine what option is being set. Then calls the appropriate setter from [#Settings]. Does not save it to disk.
///
pub fn configure(config: &mut UserConfig, terminal: &Term) -> Result<()> {
	let options = ["Username", "Tier", "Protocol"];
	let opt = Select::with_theme(&ColorfulTheme::default())
		.items(&options)
		.interact_on(terminal)?;
	let mut user_settings = Settings::new(config.clone(), terminal);
	match opt {
		0 => {
			user_settings.set_username()?;
		}
		1 => {
			user_settings.set_password()?;
		}
		2 => {
			user_settings.set_tier()?;
		}
		3 => {
			user_settings.set_protocol()?;
		}
		_ => {}
	}
	*config = user_settings.into_inner();
	Ok(())
}
