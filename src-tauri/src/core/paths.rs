use std::fs;
use std::path::PathBuf;

use serde::Serialize;
use tauri::{AppHandle, Manager};

use super::error::to_command_error;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppPaths {
    pub data_dir: String,
    pub config_dir: String,
    pub log_dir: String,
    pub backup_dir: String,
    pub export_dir: String,
    pub database_path: String,
}

impl AppPaths {
    pub fn database_path_buf(&self) -> PathBuf {
        PathBuf::from(&self.database_path)
    }
}

pub fn resolve_paths(app: &AppHandle) -> Result<AppPaths, String> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| to_command_error("failed to resolve app data directory", error))?;
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|error| to_command_error("failed to resolve app config directory", error))?;
    let log_dir = app
        .path()
        .app_log_dir()
        .map_err(|error| to_command_error("failed to resolve app log directory", error))?;

    let backup_dir = data_dir.join("backups");
    let export_dir = data_dir.join("exports");
    let database_path = data_dir.join("local_business_manager.sqlite");

    Ok(AppPaths {
        data_dir: data_dir.to_string_lossy().to_string(),
        config_dir: config_dir.to_string_lossy().to_string(),
        log_dir: log_dir.to_string_lossy().to_string(),
        backup_dir: backup_dir.to_string_lossy().to_string(),
        export_dir: export_dir.to_string_lossy().to_string(),
        database_path: database_path.to_string_lossy().to_string(),
    })
}

pub fn ensure_directories(paths: &AppPaths) -> Result<(), String> {
    for path in [
        &paths.data_dir,
        &paths.config_dir,
        &paths.log_dir,
        &paths.backup_dir,
        &paths.export_dir,
    ] {
        fs::create_dir_all(path)
            .map_err(|error| to_command_error("failed to create local storage directory", error))?;
    }
    Ok(())
}
