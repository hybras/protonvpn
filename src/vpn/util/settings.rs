use super::*;
use anyhow::Result;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

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
    /// Set the ProtonVPN Username
    fn set_username(&mut self) -> Result<String> {
        print!("Enter your ProtonVPN OpenVPN username: ");
        let old_username = self.user_config.username.clone();
        // Preamble for all set methods
        // I don't understand lifetimes.
        let stdout = stdout();
        let mut out = BufWriter::new(stdout.lock());
        out.flush()?;
        let stdin = stdin();
        let mut sin = stdin.lock();
        // End preamble
        sin.read_line(&mut self.user_config.username)?;
        Ok(old_username)
    }

    fn set_enum_field<T: ToString + Clone, N: AsRef<str>>(
        &mut self,
        name: N,
        getter: impl Fn(&UserConfig) -> T,
        setter: impl Fn(&mut UserConfig, T) -> (),
        options: &[T],
    ) -> Result<T> {
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

    /// Set the users ProtonVPN Plan.
    fn set_tier(&mut self) -> Result<u8> {
        let protonvpn_plans = ["Free", "Basic", "Plus & Visionary"];
        for (idx, &plan) in protonvpn_plans.iter().enumerate() {
            println!("{}) {}", idx, plan);
        }
        print!("Enter your Tier: ");
        // Preamble for all set methods
        // I don't understand lifetimes.
        let stdout = stdout();
        let mut out = BufWriter::new(stdout.lock());
        out.flush()?;
        let stdin = stdin();
        let mut sin = stdin.lock();
        // End preamble
        let old_tier = self.user_config.tier;
        let mut tier = String::new();
        self.user_config.tier = loop {
            sin.read_line(&mut tier)?;
            let possible_tier: u8 = tier.trim().parse()?;
            if (0..protonvpn_plans.len()).contains(&(possible_tier as usize)) {
                break possible_tier;
            } else {
                println!("Enter a valid tier");
                continue;
            }
        };
        Ok(old_tier)
    }

    fn set_protocol(&mut self) -> Result<ConnectionProtocol> {
        use ConnectionProtocol::*;
        let protocols = [UDP, TCP];
        for (idx, &protocol) in protocols.iter().enumerate() {
            println!("{}) {}", idx, protocol.to_string());
        }
        print!("Enter your protocol: ");
        // Preamble for all set methods
        // I don't understand lifetimes.
        let stdout = stdout();
        let mut out = BufWriter::new(stdout.lock());
        out.flush()?;
        let stdin = stdin();
        let mut sin = stdin.lock();
        // End preamble
        let old_protocol = self.user_config.default_protocol;
        let mut protocol = String::new();
        self.user_config.default_protocol = loop {
            sin.read_line(&mut protocol)?;
            let possible_tier: u8 = protocol.trim().parse()?;
            if (0..protocols.len()).contains(&(possible_tier as usize)) {
                break protocols[possible_tier as usize];
            } else {
                println!("Enter a valid tier");
                continue;
            }
        };
        Ok(old_protocol)
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
                assert_eq!(old, 0);
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
            &[ConnectionProtocol::UDP, ConnectionProtocol::TCP],
        );
    }
}
