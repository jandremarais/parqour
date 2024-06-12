use app::App;
use clap::Parser;
use utils::initialize_logging;

mod app;
mod cli;
pub mod tui;
pub mod utils;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    initialize_logging()?;
    let _args = cli::Args::parse();
    App::default().run().await?;
    Ok(())
}
