use crate::vpn::{
    constants::APP_NAME,
    util::{settings::*, UserConfig},
};
use anyhow::{Context, Result};
use confy::store;
use std::io::{BufRead, Write};

/// Sets and saves new configuration settings, OVERWRITING the old options.
///
/// Reads an int to determine what option is being set. Then calls the appropriate setter from [#Settings]. Then saves it to disk.
///
pub(crate) fn configure<R, W>(config: &mut UserConfig, r: &mut R, w: &mut W) -> Result<()>
where
    R: BufRead,
    W: Write,
{
    let options = ["Username", "Tier", "Protocol"];
    writeln!(w, "Options: ")?;
    for (idx, &opt) in options.iter().enumerate() {
        writeln!(w, "{}) {}", idx, opt)?;
    }
    writeln!(w, "Enter Option: ")?;
    let mut opt = String::new();
    r.read_line(&mut opt)?;
    let opt = opt
        .trim()
        .parse::<u8>()
        .context("You entered a garbage value")?;
    let mut user_settings = Settings::new(config.clone(), w, r);
    match opt {
        0 => {
            user_settings.set_username()?;
        }
        1 => {
            user_settings.set_tier()?;
        }
        2 => {
            user_settings.set_protocol()?;
        }
        _ => {}
    }
    *config = user_settings.inner();
    store(APP_NAME, config).context("Couldn't save settings")?;
    Ok(())
}

pub(crate) fn initialize<R, W>(config: &mut UserConfig, r: &mut R, w: &mut W) -> Result<()>
where
    R: BufRead,
    W: Write,
{
    let mut user_settings = Settings::new(config.clone(), w, r);
    user_settings.set_username()?;
    user_settings.set_tier()?;
    user_settings.set_protocol()?;
    *config = user_settings.inner();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{stdin, stdout, BufReader};

    /// TODO: This test should use a Cursor for stdin
    #[test]
    fn test_configure() {
        let _input = b"2\n";
        let mut stdin = BufReader::new(stdin());
        let out = stdout();
        let mut stdout = out.lock();
        let mut config = UserConfig::default();
        let res = configure(&mut config, &mut stdin, &mut stdout);
        assert!(res.is_ok());
        writeln!(stdout, "{:#?}", config).expect("Couldn't write to out");
    }

    /// TODO: This test should use a Cursor for stdin
    #[test]
    fn test_initialize() {
        let mut stdin = BufReader::new(stdin());
        let out = stdout();
        let mut stdout = out.lock();
        let mut config = UserConfig::default();
        let res = initialize(&mut config, &mut stdin, &mut stdout);
        assert!(res.is_ok());
        writeln!(stdout, "{:#?}", config).expect("Couldn't write to out");
    }
}
