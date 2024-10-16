use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Parquet file to view
    #[arg(name = "FILE")]
    pub filename: PathBuf,
}
