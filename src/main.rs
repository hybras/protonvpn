use anyhow::Result;
use confy::load;
use protonvpn::{cli::CliOptions, constants::APP_NAME, main as main_cli, vpn::util::Config};
use std::io::{stdin, stdout};

#[paw::main]
fn main(args: CliOptions) -> Result<()> {
	// Get stdio handles. These are passed through the entire program
	let stdin = stdin();
	let mut in_lock = stdin.lock();
	let stdout = stdout();
	let mut out_lock = stdout.lock();

	let config = load::<Config>(APP_NAME);

	main_cli(args, config, &mut in_lock, &mut out_lock)
}
