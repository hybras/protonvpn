//! The functions in this module are assumed to work, being short, resuable, wrappers around external library. They have been tested by hand, but currently can't be tested programmatically because console doesn't have a testing functionality.

use super::util::{ConnectionProtocol, PlanTier, UserConfig};
use anyhow::Result;
use dialoguer::{console::Term, theme::ColorfulTheme};

use std::{fmt::Display, mem::replace, str::FromStr};
use strum::IntoEnumIterator;

/// Encapsulation for mutating ProtonVPN Settings.
///
/// Each "setter" prints options, reads presumptive option from stdin,
/// and writes it to the internal config struct. It does not write the settings to disk.
///
/// In the future, this struct should store stdin/stdout handles for buffering, and write settings upon Drop.
pub(crate) struct Settings<'a, S> {
	settings: S,
	terminal: &'a Term,
}

impl<'a, S> Settings<'a, S> {
	pub fn new(settings: S, terminal: &'a Term) -> Self {
		Self { settings, terminal }
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
			.interact_on(self.terminal)?;

		let old_value = replace(getter(&mut self.settings), options[new_value]);
		Ok(old_value)
	}

	/// TODO Make this bound clone
	fn set_value_field<T, N>(&mut self, name: N, getter: impl Fn(&mut S) -> &mut T) -> Result<T>
	where
		T: Display + FromStr + Clone,
		N: AsRef<str>,
		<T as FromStr>::Err: std::marker::Sync + std::error::Error + std::marker::Send + 'static,
	{
		use dialoguer::Input;

		let new = Input::<T>::with_theme(&ColorfulTheme::default())
			.with_prompt(name.as_ref())
			.interact_on(self.terminal)?;

		let old_value = replace(getter(&mut self.settings), new);
		Ok(old_value)
	}

	pub(crate) fn into_inner(self) -> S {
		self.settings
	}
}

/// Adds named setters for UserConfig properties
impl<'a> Settings<'a, UserConfig> {
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
			.interact_on(self.terminal)?;
		let old = replace(&mut self.settings.password, pass);
		Ok(old)
	}
}

fn get_enum_field<T, N>(terminal: &Term, name: N) -> Result<T>
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
		.interact_on(terminal)?;

	Ok(options[new_value])
}

fn ask_for<T, N>(terminal: &Term, name: N) -> Result<T>
where
	T: Display + FromStr + Clone,
	N: AsRef<str>,
	<T as FromStr>::Err: std::marker::Sync + std::error::Error + std::marker::Send + 'static,
{
	use dialoguer::Input;

	let new = Input::<T>::with_theme(&ColorfulTheme::default())
		.with_prompt(name.as_ref())
		.interact_on(terminal)?;

	Ok(new)
}
