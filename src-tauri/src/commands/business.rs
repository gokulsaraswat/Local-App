use tauri::AppHandle;

use crate::{
    core::{db, error::CommandResult},
    domain::models::{BusinessProfile, NewBusinessWorkspaceInput},
};

#[tauri::command]
pub fn save_business_profile(app: AppHandle, profile: BusinessProfile) -> CommandResult<BusinessProfile> {
    db::with_connection(&app, |conn, _paths| db::save_business_profile(conn, &profile))
}

#[tauri::command]
pub fn create_business_workspace(
    app: AppHandle,
    input: NewBusinessWorkspaceInput,
) -> CommandResult<BusinessProfile> {
    db::with_connection(&app, |conn, _paths| db::create_business_workspace(conn, &input))
}

#[tauri::command]
pub fn switch_active_business(app: AppHandle, business_id: String) -> CommandResult<BusinessProfile> {
    db::with_connection(&app, |conn, _paths| db::switch_active_business(conn, &business_id))
}
