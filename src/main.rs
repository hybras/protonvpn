use anyhow::Result;
use confy::load;
use console::Term;
use protonvpn::{cli::CliOptions, constants::APP_NAME, main as main_cli, vpn::util::Config};

#[paw::main]
fn main(args: CliOptions) -> Result<()> {
	// Stdio handle is passed through the entire program
	let mut terminal = Term::buffered_stdout();

	let config = load::<Config>(APP_NAME);

	main_cli(args, config, &mut terminal)
}
