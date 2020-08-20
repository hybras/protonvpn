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
struct SettingsMutator {
    user_config: UserConfig,
}

impl From<UserConfig> for SettingsMutator {
    fn from(user_config: UserConfig) -> Self {
        Self { user_config }
    }
}

impl SettingsMutator {
    fn set_enum_field<T: Display + Copy + IntoEnumIterator, N: AsRef<str>>(
        &mut self,
        name: N,
        getter: impl Fn(&UserConfig) -> T,
        setter: impl Fn(&mut UserConfig, T) -> (),
    ) -> Result<T> {
        let options:Vec<T> = T::iter().collect();
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
        let old_value = getter(&mut self.user_config);
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
        setter(&mut self.user_config, options[new_value].clone());
        Ok(old_value)
    }

    fn set_value_field<T, N>(
        &mut self,
        name: N,
        getter: impl Fn(&UserConfig) -> T,
        setter: impl Fn(&mut UserConfig, T) -> (),
    ) -> Result<T>
    where
        T: Display + Clone + FromStr,
        N: AsRef<str>,
        <T as FromStr>::Err: std::marker::Sync + std::error::Error + std::marker::Send + 'static,
    {
        print!("Enter your {}: ", name.as_ref());
        let old_value = getter(&self.user_config).clone();
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
        setter(&mut self.user_config, new);
        Ok(old_value)
    }

    /// Set the ProtonVPN Username
    fn set_username(&mut self) -> Result<String> {
        self.set_value_field("username", |u| u.username.clone(), |u, t| u.username = t)
    }

    /// Set the users ProtonVPN Plan.
    fn set_tier(&mut self) -> Result<PlanTier> {
        use PlanTier::*;
        self.set_enum_field(
            "Plan Tier",
            |t| t.tier,
            |u, t| u.tier = t,
        )
    }

    fn set_protocol(&mut self) -> Result<ConnectionProtocol> {
        use ConnectionProtocol::*;
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
        let mut settings = SettingsMutator::default();
        let old = settings.set_username();
        match old {
            Ok(old) => {
                println!("{}", settings.user_config.username);
                assert_eq!(old, "username");
            }
            Err(_) => assert!(false, "Setting username failed"),
        }
    }

    #[test]
    fn test_set_tier() {
        let mut settings = SettingsMutator::default();
        let old = settings.set_tier();
        match old {
            Ok(old) => {
                println!("{}", settings.user_config.tier);
                assert_eq!(old, PlanTier::Free);
            }
            Err(_) => assert!(false, "Setting Tier failed"),
        }
    }
    #[test]
    fn test_generic_setter() {
        let mut settings = SettingsMutator::default();
        let old = settings.set_enum_field(
            "Connection Protocol",
            |u| u.default_protocol,
            |u, t| u.default_protocol = t,
        );
    }
}
