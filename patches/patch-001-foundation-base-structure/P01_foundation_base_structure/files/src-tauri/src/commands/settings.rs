use tauri::AppHandle;

use crate::{
    core::{db, error::CommandResult},
    domain::models::BusinessSettings,
};

#[tauri::command]
pub fn save_business_settings(
    app: AppHandle,
    settings: BusinessSettings,
) -> CommandResult<BusinessSettings> {
    db::with_connection(&app, |conn, _paths| db::save_business_settings(conn, &settings))
}
