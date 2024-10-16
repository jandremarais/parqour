use std::process;

use clap::Parser;
use parqour::{args::Args, error::Result};

fn main() -> Result<()> {
    let args = Args::parse();

    match parqour::run(args) {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1)
        }
    }
}
