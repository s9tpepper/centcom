use std::{fs, path::PathBuf};

use directories::ProjectDirs;

pub fn get_project_directory<'a>(app: &'a str, path: &'a str) -> anyhow::Result<PathBuf> {
    let requested_path = ProjectDirs::from("com", "s9tpepper", app)
        .map(|project_dirs| project_dirs.data_dir().join(path));

    let path = requested_path.ok_or(anyhow::Error::msg("Could not build requested path"))?;
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    Ok(path)
}

pub fn get_app_dir(path: &str) -> anyhow::Result<PathBuf> {
    // FIXME: Update application name here
    get_project_directory("Centcom", path)
}
