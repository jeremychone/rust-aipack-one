// region:    --- Modules

mod cli;
mod error;

use crate::cli::CliArgs;
use clap::Parser;
pub use error::{Error, Result};

// endregion: --- Modules

fn main() -> Result<()> {
	// -- Command arguments
	let args = CliArgs::parse(); // Will fail early, but that’s okay.

	println!("DEBUG: your command: {args:?}");

	Ok(())
}
