use tauri::AppHandle;

use crate::{
    core::{db, error::CommandResult},
    domain::models::BusinessProfile,
};

#[tauri::command]
pub fn save_business_profile(app: AppHandle, profile: BusinessProfile) -> CommandResult<BusinessProfile> {
    db::with_connection(&app, |conn, _paths| db::save_business_profile(conn, &profile))
}
