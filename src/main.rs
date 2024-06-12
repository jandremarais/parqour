use clap::Parser;
use utils::initialize_logging;

mod cli;
pub mod utils;

fn main() -> color_eyre::Result<()> {
    initialize_logging()?;
    let _args = cli::Args::parse();
    Ok(())
}
