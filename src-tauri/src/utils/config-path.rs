use crate::utils::error::Error;
use std::path::PathBuf;
use tauri::{AppHandle, Runtime};

pub fn app_config_dir<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, Error> {
    match app.path_resolver().app_config_dir() {
        None => Err(Error::ConfigDirNotFound),
        Some(config_dir) => Ok(config_dir),
    }
}

pub fn app_config_dir_ignore<R: Runtime>(app: &AppHandle<R>) -> PathBuf {
    app.path_resolver()
        .app_config_dir()
        .expect("Can not access app_config_dir")
}

pub fn path_mapper(mut app_config_path: PathBuf, db_path: &str) -> String {
    app_config_path.push(db_path);

    format!(
        "sqlite:{}",
        app_config_path
            .to_str()
            .expect("Can not convert app_config_path into str.")
    )
}
