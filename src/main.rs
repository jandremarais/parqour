use clap::Parser;

mod cli;
pub mod utils;

fn main() {
    let _args = cli::Args::parse();
}
