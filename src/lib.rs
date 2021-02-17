//! This crate is a rust port of the [official ProtonVPN python cli](https://github.com/ProtonVPN/linux-cli). We aim for feature parity and to track changes from the official cli. This crate's only non-rust dependency is `openvpn.` You'll need to install it using your system's package manager.
//!
//! This crate **does not** aim to have the same command line interface, or use the same config files / format.

// TODO examples

#![deny(missing_docs)]
#![deny(broken_intra_doc_links)]

use crate::{
	cli::{configure, connect, initialize, CliOptions},
	constants::APP_NAME,
	utils::project_dirs,
	vpn::util::Config,
};
use anyhow::{Context, Result};
use confy::ConfyError;
use dialoguer::console::Term;
use std::io::{Write};
use CliOptions::*;

/// This module contains all the structs for storing args passed in from the command line.
///
/// `cli`'s submodules each correspond to one of the cli subcommands. Each submodule contains a function which wraps all the others in its module. This module reexports that function for use in the cli.
pub mod cli;

/// Assorted constants for use throughout the crate. Mostly strings and a map for looking up countries.
pub mod constants;
/// Miscellaneous functions. See the documentation for this module's members instead
pub(crate) mod utils;
/// Functions for interacting with the `openvpn` binary, including starting / stopping a connection, and creating config files.
pub mod vpn;

/// The main function in main.rs is a wrapper of this function.
pub fn main(
	opt: CliOptions,
	config_res: Result<Config, ConfyError>,
	terminal: &mut Term,
) -> Result<()> {
	let pdir = project_dirs();

	if let Ok(mut config) = config_res {
		match opt {
			Init => {
				initialize(&mut config.user, &pdir, terminal)?;
				confy::store(APP_NAME, config.user).context("Couldn't store your configuration")?;
			}
			Connect(flags) => {
				let _connection = connect(&flags, &mut config, &pdir)?;
			}
			Reconnect => {}
			Disconnect => {}
			Status => {}
			Configure => {
				configure(&mut config.user, terminal)?;
				confy::store(APP_NAME, config.user).context("Couldn't store your configuration")?;
			}
			Refresh => {}
			Examples => {}
		};
	} else {
		if let Init = opt {
			initialize(&mut Default::default(), &pdir, terminal)?;
		} else {
			writeln!(
				terminal,
				"Unable to load your profile. Try running `protonvpn init` again."
			)?;
		}
	}
	Ok(())
}
