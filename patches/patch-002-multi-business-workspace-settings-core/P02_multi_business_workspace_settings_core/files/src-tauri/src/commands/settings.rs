use tauri::AppHandle;

use crate::{
    core::{db, error::CommandResult},
    domain::models::WorkspaceConfigurationInput,
};

#[tauri::command]
pub fn save_workspace_configuration(
    app: AppHandle,
    input: WorkspaceConfigurationInput,
) -> CommandResult<()> {
    db::with_connection(&app, |conn, _paths| db::save_workspace_configuration(conn, &input))
}
