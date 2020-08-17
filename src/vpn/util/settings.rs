use super::*;
use anyhow::{Context, Result};
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Read, Write};

/// Set the ProtonVPN Username
fn set_username(user_config: &mut UserConfig) -> Result<String> {
    let mut stdin = stdin();
    let mut sin = stdin.lock();
    let mut stdout = stdout();
    let mut out = stdout.lock();
    out.write_all(b"Enter your ProtonVPN OpenVPN username: ")?;
    let mut username = String::new();
    let old_username = user_config.username.clone();
    sin.read_line(&mut user_config.username)?;
    Ok(old_username)
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn test_set_username() {
        let mut user_config = UserConfig::with_user("default".into());
        let old = set_username(&mut user_config);
        match old {
            Ok(old) => {
                println!("{}", user_config.username);
                assert_eq!(old, "default");
            }
            Err(_) => assert!(false, "Setting username failed"),
        }
    }
}
