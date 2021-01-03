use super::*;
use anyhow::Result;
use rpassword::read_password_with_reader;
use std::{
	io::{BufRead, Write},
	mem::replace,
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
		writeln!(self.stdout, "{}: ", name.as_ref())?;
		let options: Vec<T> = T::iter().collect();
		for (idx, option) in options.iter().enumerate() {
			writeln!(&mut self.stdout, "\t{}) {}", idx, option.to_string())?;
		}

		let mut new_value = String::new();
		let new_value = loop {
			write!(self.stdout, "Enter {}: ", name.as_ref())?;
			self.stdout.flush()?;

			self.stdin.read_line(&mut new_value)?;
			let possible_value: usize = new_value.trim().parse()?;
			if (0..options.len()).contains(&(possible_value as usize)) {
				break possible_value;
			} else {
				writeln!(&mut self.stdout, "Enter a 𝑣𝑎𝑙𝑖𝑑 number")?;
				continue;
			}
		};

		let old_value = replace(getter(&mut self.settings), options[new_value]);
		Ok(old_value)
	}

	fn set_value_field<T, N>(&mut self, name: N, getter: impl Fn(&mut S) -> &mut T) -> Result<T>
	where
		T: Display + FromStr,
		N: AsRef<str>,
		<T as FromStr>::Err: std::marker::Sync + std::error::Error + std::marker::Send + 'static,
	{
		write!(self.stdout, "Enter your {}: ", name.as_ref())?;
		self.stdout.flush()?;
		let mut new_value = String::new();
		self.stdin.read_line(&mut new_value)?;
		let new = new_value.trim().parse::<T>()?;
		let old_value = replace(getter(&mut self.settings), new);
		Ok(old_value)
	}

	pub(crate) fn inner(self) -> S {
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
		writeln!(self.stdout, "Enter password: ")?;
		self.stdout.flush()?;
		let pass = read_password_with_reader(Some(&mut self.stdin))?;
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
		let user_config = settings.inner();
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
		let user_config = settings.inner();
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
		let user = settings.inner();

		assert_eq!("password", user.password);

		let output = String::from_utf8(output)?;
		assert_eq!("", old);

		assert_eq!("Enter password: \n", output);
		Ok(())
	}
}
