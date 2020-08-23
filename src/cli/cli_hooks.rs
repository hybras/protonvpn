use crate::vpn::{
    constants::APP_NAME,
    util::{settings::*, UserConfig},
};
use anyhow::{Context, Result};
use confy::store;
use std::io::{BufRead, Write};

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
    let stdin = std::io::stdin();
    let mut opt = String::new();
    stdin.read_line(&mut opt)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{stdin, stdout, BufReader};

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
}
