use tauri::AppHandle;

use crate::{
    core::{db, error::CommandResult},
    domain::bootstrap::build_app_bootstrap,
};

#[tauri::command]
pub fn bootstrap_app(app: AppHandle) -> CommandResult<crate::domain::models::AppBootstrap> {
    db::with_connection(&app, |conn, paths| {
        let app_info = db::load_app_info(conn)?;
        let active_business = db::get_active_business(conn)?;
        let business_settings = db::get_business_settings(conn, &active_business.id)?;
        let active_tax_profile = db::get_default_tax_profile(conn, &active_business.id)?;
        let active_receipt_profile = db::get_default_receipt_profile(conn, &active_business.id)?;
        let active_module_flags = db::get_module_flags(conn, &active_business.id)?;
        let active_sequences = db::list_sequence_counters(conn, &active_business.id)?;
        let businesses = db::list_businesses(conn)?;
        let business_workspaces = db::list_business_workspace_summaries(conn)?;
        let patch_history = db::list_patch_history(conn)?;
        let backups = db::list_backups(conn)?;
        let storage = db::build_storage_status(conn, paths)?;
        let recent_activity = db::list_recent_activity(conn, 8)?;

        Ok(build_app_bootstrap(
            app_info,
            active_business,
            business_settings,
            active_tax_profile,
            active_receipt_profile,
            active_module_flags,
            active_sequences,
            businesses,
            business_workspaces,
            patch_history,
            backups,
            storage,
            recent_activity,
        ))
    })
}
