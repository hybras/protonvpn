use super::*;
use anyhow::{Context, Result};
use std::io::{
    stdin, stdout, BufRead, BufReader, BufWriter, Read, Stdin, StdinLock, Stdout, StdoutLock, Write,
};

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
        // Preamble for all set methods
        // I don't understand lifetimes.
        let stdin = stdin();
        let mut sin = stdin.lock();
        let stdout = stdout();
        let mut out = stdout.lock();
        // End preamble
        out.write_all(b"Enter your ProtonVPN OpenVPN username: ")?;
        let old_username = self.user_config.username.clone();
        sin.read_line(&mut self.user_config.username)?;
        Ok(old_username)
    }
 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_username() {
        let mut settings = SettingsMutator {
            user_config: UserConfig::with_user("default".into()),
        };
        let old = settings.set_username();
        match old {
            Ok(old) => {
                println!("{}", settings.user_config.username);
                assert_eq!(old, "default");
            }
            Err(_) => assert!(false, "Setting username failed"),
        }
    }
}
