mod commands;
mod core;
mod domain;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            core::db::initialize(&handle).expect("failed to initialize local workspace core");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::bootstrap::bootstrap_app,
            commands::business::save_business_profile,
            commands::business::create_business_workspace,
            commands::business::switch_active_business,
            commands::settings::save_workspace_configuration,
            commands::data_center::create_backup_snapshot,
            commands::data_center::export_foundation_snapshot,
            commands::data_center::preview_import_bundle
        ])
        .run(tauri::generate_context!())
        .expect("error while running local business manager");
}
