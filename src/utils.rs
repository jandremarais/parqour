use std::path::PathBuf;

use color_eyre::eyre;
use directories::ProjectDirs;

pub fn get_data_dir() -> eyre::Result<PathBuf> {
    let directory = if let Ok(s) = std::env::var("PARQOUR_DATA") {
        PathBuf::from(s)
    } else if let Some(proj_dirs) = ProjectDirs::from("com", "", "parqour") {
        proj_dirs.data_local_dir().to_path_buf()
    } else {
        return Err(eyre::eyre!("Unable to find data directory for parqour"));
    };
    Ok(directory)
}

pub fn get_config_dir() -> eyre::Result<PathBuf> {
    let directory = if let Ok(s) = std::env::var("PARQOUR_CONFIG") {
        PathBuf::from(s)
    } else if let Some(proj_dirs) = ProjectDirs::from("com", "jandremarais", "parqour") {
        proj_dirs.config_local_dir().to_path_buf()
    } else {
        return Err(eyre::eyre!("Unable to find config directory for parqour"));
    };
    Ok(directory)
}
