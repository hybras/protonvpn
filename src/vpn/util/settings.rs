use super::*;
use anyhow::Result;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
use strum::IntoEnumIterator;

/// Encapsulation for mutating ProtonVPN Settings.
///
/// Each "setter" prints options, reads presumptive option from stdin,
/// and writes it to the internal config struct. It does not write the settings to disk.
///
/// In the future, this struct should store stdin/stdout handles for buffering, and write settings upon Drop.
#[derive(Default)]
pub(crate) struct Settings<S>(S);

impl<S> From<S> for Settings<S> {
    fn from(settings: S) -> Self {
        Self(settings)
    }
}

impl<S> Settings<S> {
    fn set_enum_field<T, N>(
        &mut self,
        name: N,
        getter: impl Fn(&S) -> T,
        setter: impl Fn(&mut S, T) -> (),
    ) -> Result<T>
    where
        T: Display + Copy + IntoEnumIterator,
        N: AsRef<str>,
    {
        let options: Vec<T> = T::iter().collect();
        for (idx, option) in options.iter().enumerate() {
            println!("{}) {}", idx, option.to_string());
        }
        print!("Enter {}: ", name.as_ref());
        // Preamble for all set methods
        // I don't understand lifetimes.
        let stdout = stdout();
        let mut out = BufWriter::new(stdout.lock());
        out.flush()?;
        let stdin = stdin();
        let mut sin = stdin.lock();
        // End preamble
        let old_value = getter(&mut self.0);
        let mut new_value = String::new();
        let new_value = loop {
            sin.read_line(&mut new_value)?;
            let possible_value: usize = new_value.trim().parse()?;
            if (0..options.len()).contains(&(possible_value as usize)) {
                break possible_value;
            } else {
                println!("Enter a valid number");
                continue;
            }
        };
        setter(&mut self.0, options[new_value].clone());
        Ok(old_value)
    }

    fn set_value_field<T, N>(
        &mut self,
        name: N,
        getter: impl Fn(&S) -> T,
        setter: impl Fn(&mut S, T) -> (),
    ) -> Result<T>
    where
        T: Display + Clone + FromStr,
        N: AsRef<str>,
        <T as FromStr>::Err: std::marker::Sync + std::error::Error + std::marker::Send + 'static,
    {
        print!("Enter your {}: ", name.as_ref());
        let old_value = getter(&self.0).clone();
        // Preamble for all set methods
        // I don't understand lifetimes.
        let stdout = stdout();
        let mut out = BufWriter::new(stdout.lock());
        out.flush()?;
        let stdin = stdin();
        let mut sin = stdin.lock();
        // End preamble
        let mut new_value = String::new();
        sin.read_line(&mut new_value)?;
        let new = new_value.trim().parse::<T>()?;
        setter(&mut self.0, new);
        Ok(old_value)
    }

    pub (crate) fn inner(self) -> S {
        self.0
    }
}

/// Adds named setters for UserConfig properties
pub(crate) type UserSettings = Settings<UserConfig>;

impl UserSettings {
    /// Set the ProtonVPN Username
    pub(crate) fn set_username(&mut self) -> Result<String> {
        self.set_value_field("username", |u| u.username.clone(), |u, t| u.username = t)
    }

    /// Set the users ProtonVPN Plan.
    pub(crate) fn set_tier(&mut self) -> Result<PlanTier> {
        self.set_enum_field("Plan Tier", |t| t.tier, |u, t| u.tier = t)
    }

    pub(crate) fn set_protocol(&mut self) -> Result<ConnectionProtocol> {
        self.set_enum_field(
            "Connection Protocol",
            |u| u.default_protocol,
            |u, t| u.default_protocol = t,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_username() {
        let mut settings = UserSettings::default();
        let old = settings.set_username();
        match old {
            Ok(old) => {
                println!("{}", settings.0.username);
                assert_eq!(old, "username");
            }
            Err(_) => assert!(false, "Setting username failed"),
        }
    }

    #[test]
    fn test_set_tier() {
        let mut settings = UserSettings::default();
        let old = settings.set_tier();
        match old {
            Ok(old) => {
                println!("{}", settings.0.tier);
                assert_eq!(old, PlanTier::Free);
            }
            Err(_) => assert!(false, "Setting Tier failed"),
        }
    }
    #[test]
    fn test_generic_setter() {
        let mut settings = UserSettings::default();
        let old = settings.set_enum_field(
            "Connection Protocol",
            |u| u.default_protocol,
            |u, t| u.default_protocol = t,
        );
    }
}
