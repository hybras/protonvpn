use super::*;
use anyhow::Result;
use std::io::{stdin, BufRead, BufReader, stdout, BufWriter, Write};

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

    /// Set the users ProtonVPN Plan.
    fn set_tier(&mut self) -> Result<u8> {
        let protonvpn_plans = ["Free", "Basic", "Plus & Visionary",];
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
}
