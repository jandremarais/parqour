use std::path::PathBuf;

use clap::Parser;

use crate::utils::{get_config_dir, get_data_dir};

#[derive(Parser, Debug)]
#[command(version = version(), about = "View parquet files")]
pub struct Args {
    /// Path to parquet file to view
    #[arg(value_name = "FILE")]
    pub path: PathBuf,
}

pub fn version() -> String {
    let author = clap::crate_authors!();

    let commit_hash = env!("PARQOUR_GIT_INFO");

    // let current_exe_path = PathBuf::from(clap::crate_name!()).display().to_string();
    let config_dir_path = get_config_dir().display().to_string();
    let data_dir_path = get_data_dir().display().to_string();

    format!(
        "\
{commit_hash}

Authors: {author}

Config directory: {config_dir_path}
Data directory: {data_dir_path}"
    )
}
