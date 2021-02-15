use super::util::{ConnectionProtocol, PlanTier, UserConfig};
use anyhow::Result;
use dialoguer::theme::ColorfulTheme;

use std::{
	fmt::Display,
	io::{BufRead, Write},
	mem::replace,
	str::FromStr,
};
use strum::IntoEnumIterator;

/// Encapsulation for mutating ProtonVPN Settings.
///
/// Each "setter" prints options, reads presumptive option from stdin,
/// and writes it to the internal config struct. It does not write the settings to disk.
///
/// In the future, this struct should store stdin/stdout handles for buffering, and write settings upon Drop.
pub(crate) struct Settings<'a, S, R: BufRead, W: Write> {
	settings: S,
	stdout: &'a mut W,
	stdin: &'a mut R,
}

impl<'a, S, R: BufRead, W: Write> Settings<'a, S, R, W> {
	pub fn new(settings: S, stdout: &'a mut W, stdin: &'a mut R) -> Self {
		Self {
			settings,
			stdout,
			stdin,
		}
	}

	fn set_enum_field<T, N>(&mut self, name: N, getter: impl Fn(&mut S) -> &mut T) -> Result<T>
	where
		T: Display + Copy + IntoEnumIterator,
		N: AsRef<str>,
	{
		use dialoguer::Select;

		let options: Vec<T> = T::iter().collect();

		let new_value = Select::with_theme(&ColorfulTheme::default())
			.with_prompt(name.as_ref())
			.default(0)
			.items(&options)
			.interact()?;

		let old_value = replace(getter(&mut self.settings), options[new_value]);
		Ok(old_value)
	}

	fn set_value_field<T, N>(&mut self, name: N, getter: impl Fn(&mut S) -> &mut T) -> Result<T>
	where
		T: Display + FromStr + Clone,
		N: AsRef<str>,
		<T as FromStr>::Err: std::marker::Sync + std::error::Error + std::marker::Send + 'static,
	{
		use dialoguer::Input;

		let new: T = Input::with_theme(&ColorfulTheme::default())
			.with_prompt(name.as_ref())
			.interact()?;

		let old_value = replace(getter(&mut self.settings), new);
		Ok(old_value)
	}

	pub(crate) fn into_inner(self) -> S {
		self.settings
	}
}

/// Adds named setters for UserConfig properties
impl<'a, R: BufRead, W: Write> Settings<'a, UserConfig, R, W> {
	/// Set the ProtonVPN Username
	pub(crate) fn set_username(&mut self) -> Result<String> {
		self.set_value_field("username", |u| &mut u.username)
	}

	/// Set the users ProtonVPN Plan.
	pub(crate) fn set_tier(&mut self) -> Result<PlanTier> {
		self.set_enum_field("Plan Tier", |t| &mut t.tier)
	}

	pub(crate) fn set_protocol(&mut self) -> Result<ConnectionProtocol> {
		self.set_enum_field("Connection Protocol", |u| &mut u.protocol)
	}

	pub(crate) fn set_password(&mut self) -> Result<String> {
		use dialoguer::Password;

		let pass = Password::with_theme(&ColorfulTheme::default())
			.with_prompt("Password")
			.with_confirmation("Confirm password", "Passwords mismatching")
			.interact()?;
		let old = replace(&mut self.settings.password, pass);
		Ok(old)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_set_username() {
		let mut input = "hello\n".as_bytes();
		let mut output = vec![];
		let mut settings = Settings::new(UserConfig::default(), &mut output, &mut input);
		let old = settings.set_username();
		let user_config = settings.into_inner();
		match old {
			Ok(old) => {
				assert_eq!(user_config.username, "hello");
				assert_eq!(old, "");
			}
			Err(_) => assert!(false, "Setting username failed"),
		}
	}

	#[test]
	fn test_set_tier() {
		let mut input = "2\n".as_bytes();
		let mut output = vec![];
		let mut settings = Settings::new(UserConfig::default(), &mut output, &mut input);
		let old = settings.set_tier();
		let user_config = settings.into_inner();
		match old {
			Ok(old) => {
				assert_eq!(user_config.tier, PlanTier::Plus);
				assert_eq!(old, PlanTier::Free);
			}
			Err(_) => assert!(false, "Setting Tier failed"),
		}
	}
	#[test]
	fn test_set_pass() -> Result<()> {
		let mut input = "password\n".as_bytes();
		let mut output = vec![];
		let user = UserConfig::default();
		let mut settings = Settings::new(user, &mut output, &mut input);

		let old = settings.set_password()?;
		let user = settings.into_inner();

		assert_eq!("password", user.password);

		assert_eq!("", old);

		Ok(())
	}
}
