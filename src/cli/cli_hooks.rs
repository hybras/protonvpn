use crate::vpn::util::{settings::*, Config};
use anyhow::{Context, Result};

pub(crate) fn configure(config: &mut Config) -> Result<()> {
    let mut user_settings = UserSettings::from(config.user.clone());
    let options = ["Username", "Tier", "Protocol"];
    println!("Options: ");
    for (idx, &opt) in options.iter().enumerate() {
        println!("{}) {}", idx, opt);
    }
    println!("Enter Option: ");
    let stdin = std::io::stdin();
    let mut opt = String::new();
    stdin.read_line(&mut opt)?;
    let opt = opt
        .trim()
        .parse::<u8>()
        .context("You entered a garbage value")?;
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
        _ => println!("Enter an in range value"),
    }
    Ok(())
}
