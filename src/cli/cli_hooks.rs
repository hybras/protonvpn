use crate::vpn::util::{settings::*, UserConfig};
use anyhow::{Context, Result};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

/// Sets and saves new configuration settings, OVERWRITING the old options.
///
/// Reads an int to determine what option is being set. Then calls the appropriate setter from [#Settings]. Does not save it to disk.
///
pub async fn configure<R, W>(config: &mut UserConfig, r: &mut R, w: &mut W) -> Result<()>
where
    R: AsyncBufReadExt + Unpin,
    W: AsyncWriteExt + Unpin,
{
    let options = ["Username", "Tier", "Protocol"];
    w.write(b"Options: \n").await?;
    for (idx, &opt) in options.iter().enumerate() {
        w.write(format!("{}) {}", idx, opt).as_bytes()).await?;
    }
    w.write(b"Enter Option: ").await?;
    let mut opt = String::new();
    r.read_line(&mut opt).await?;
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
    Ok(())
}

pub fn initialize<R, W>(config: &mut UserConfig, r: &mut R, w: &mut W) -> Result<()>
where
    R: AsyncBufReadExt + Unpin,
    W: AsyncWriteExt + Unpin,
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
    use std::io::Cursor;

    #[test]
    fn test_configure() -> Result<()> {
        let mut stdin = Cursor::new("0\nhybras\n");
        let mut stdout = Cursor::new(vec![]);

        let expected = UserConfig {
            username: Some("hybras".into()),
            ..Default::default()
        };
        let mut config = UserConfig::default();

        let _ = configure(&mut config, &mut stdin, &mut stdout)
            .context("Failed to interact with user to get config")?;
        assert_eq!(expected, config);
        Ok(())
    }

    #[test]
    fn test_initialize() -> Result<()> {
        let mut stdin = Cursor::new("hybras\n2\n1\n");
        let mut stdout = Cursor::new(vec![]);

        let expected = UserConfig {
            username: Some("hybras".into()),
            tier: crate::vpn::util::PlanTier::Plus,
            protocol: crate::vpn::util::ConnectionProtocol::TCP,
            ..Default::default()
        };
        let mut config = UserConfig::default();

        let _ = initialize(&mut config, &mut stdin, &mut stdout)
            .context("Failed to interact with user to get config")?;
        assert_eq!(expected, config);
        Ok(())
    }
}
