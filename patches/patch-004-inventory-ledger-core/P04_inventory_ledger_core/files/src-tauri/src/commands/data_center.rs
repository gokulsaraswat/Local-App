use std::{fs, path::PathBuf};

use chrono::Utc;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use tauri::AppHandle;
use uuid::Uuid;

use crate::{
    core::{
        catalog, db,
        error::{to_command_error, CommandResult},
        inventory,
        paths::resolve_paths,
    },
    domain::models::{BackupRecord, ExportJobRecord, ImportPreview},
};

fn checksum_for_file(path: &PathBuf) -> Result<String, String> {
    let bytes = fs::read(path)
        .map_err(|error| to_command_error("failed to read file for checksum", error))?;
    let digest = Sha256::digest(&bytes);
    Ok(format!("{digest:x}"))
}

#[tauri::command]
pub fn create_backup_snapshot(app: AppHandle) -> CommandResult<BackupRecord> {
    let paths = resolve_paths(&app)?;
    let (active_business, business_settings) = db::with_connection(&app, |conn, _paths| {
        let active_business = db::get_active_business(conn)?;
        let business_settings = db::get_business_settings(conn, &active_business.id)?;
        Ok((active_business, business_settings))
    })?;

    let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let file_name = format!("workspace_snapshot_{timestamp}.sqlite");
    let backup_dir = business_settings
        .backup_directory
        .clone()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(paths.backup_dir.clone());
    fs::create_dir_all(&backup_dir)
        .map_err(|error| to_command_error("failed to prepare backup directory", error))?;
    let destination = PathBuf::from(backup_dir).join(&file_name);

    fs::copy(paths.database_path_buf(), &destination)
        .map_err(|error| to_command_error("failed to create backup snapshot", error))?;

    let checksum = checksum_for_file(&destination)?;
    let record = BackupRecord {
        id: Uuid::new_v4().to_string(),
        business_id: Some(active_business.id.clone()),
        file_name,
        file_path: destination.to_string_lossy().to_string(),
        checksum: Some(checksum),
        status: "completed".into(),
        created_at: db::now_iso(),
    };

    db::with_connection(&app, |conn, _paths| {
        db::insert_backup_record(conn, &record)?;
        db::insert_log(
            conn,
            "INFO",
            "backup",
            "Local backup snapshot created",
            None,
        )?;
        Ok(record.clone())
    })
}

#[tauri::command]
pub fn export_foundation_snapshot(app: AppHandle) -> CommandResult<String> {
    let paths = resolve_paths(&app)?;

    let (
        app_info,
        active_business,
        businesses,
        business_settings,
        tax_profiles,
        receipt_profiles,
        module_flags,
        sequence_counters,
        patch_history,
        backups,
        catalog_categories,
        catalog_units,
        catalog_items,
        catalog_barcodes,
        inventory_movements,
    ) = db::with_connection(&app, |conn, _paths| {
        let active_business = db::get_active_business(conn)?;

        Ok((
            db::load_app_info(conn)?,
            active_business,
            db::list_businesses(conn)?,
            db::list_all_business_settings(conn)?,
            db::list_all_tax_profiles(conn)?,
            db::list_all_receipt_profiles(conn)?,
            db::list_all_module_flags(conn)?,
            db::list_all_sequence_counters(conn)?,
            db::list_patch_history(conn)?,
            db::list_backups(conn)?,
            catalog::list_all_catalog_categories(conn)?,
            catalog::list_all_catalog_units(conn)?,
            catalog::list_all_catalog_items(conn)?,
            catalog::list_all_catalog_barcodes(conn)?,
            inventory::list_all_inventory_movements(conn)?,
        ))
    })?;

    let generated_at = db::now_iso();
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let export_path =
        PathBuf::from(&paths.export_dir).join(format!("workspace_export_{timestamp}.json"));

    let source_patch_level = app_info.patch_level.clone();
    let product_name = app_info.product_name.clone();
    let active_business_id = active_business.id.clone();

    let bundle = json!({
        "manifest": {
            "bundleVersion": "4.0.0",
            "bundleType": "workspace-foundation-export",
            "sourcePatchLevel": source_patch_level,
            "schemaVersion": app_info.schema_version,
            "generatedAt": generated_at.clone(),
            "productName": product_name
        },
        "appInfo": app_info,
        "activeBusinessId": active_business_id.clone(),
        "businesses": businesses,
        "businessSettings": business_settings,
        "taxProfiles": tax_profiles,
        "receiptProfiles": receipt_profiles,
        "moduleFlags": module_flags,
        "sequenceCounters": sequence_counters,
        "patchHistory": patch_history,
        "backupRecords": backups,
        "catalogCategories": catalog_categories,
        "catalogUnits": catalog_units,
        "catalogItems": catalog_items,
        "catalogBarcodes": catalog_barcodes,
        "inventoryMovements": inventory_movements
    });

    fs::write(
        &export_path,
        serde_json::to_string_pretty(&bundle)
            .map_err(|error| to_command_error("failed to serialize export bundle", error))?,
    )
    .map_err(|error| to_command_error("failed to write export bundle", error))?;

    let export_job = ExportJobRecord {
        id: Uuid::new_v4().to_string(),
        business_id: Some(active_business_id),
        format: "json-workspace-foundation-v4".into(),
        status: "completed".into(),
        target_path: Some(export_path.to_string_lossy().to_string()),
        created_at: generated_at.clone(),
        completed_at: Some(generated_at),
    };

    db::with_connection(&app, |conn, _paths| {
        db::insert_export_job(conn, &export_job)?;
        db::insert_log(
            conn,
            "INFO",
            "export",
            "Workspace export bundle created",
            None,
        )?;
        Ok(())
    })?;

    Ok(export_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn preview_import_bundle(app: AppHandle, file_path: String) -> CommandResult<ImportPreview> {
    let raw = fs::read_to_string(&file_path)
        .map_err(|error| to_command_error("failed to read import bundle", error))?;

    let parsed: Value = serde_json::from_str(&raw)
        .map_err(|error| to_command_error("invalid JSON bundle", error))?;

    let manifest = parsed.get("manifest").cloned().unwrap_or_else(|| json!({}));

    let businesses = parsed
        .get("businesses")
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default();

    let categories = parsed
        .get("catalogCategories")
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default();

    let items = parsed
        .get("catalogItems")
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default();

    let movements = parsed
        .get("inventoryMovements")
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default();

    let mut warnings = Vec::new();

    if manifest.get("bundleType").is_none() {
        warnings.push("Bundle is missing manifest.bundleType".into());
    }

    if manifest.get("bundleVersion").is_none() {
        warnings.push("Bundle is missing manifest.bundleVersion".into());
    }

    if parsed.get("businessSettings").is_none() {
        warnings.push("Bundle is missing businessSettings".into());
    }

    if parsed.get("inventoryMovements").is_none() {
        warnings.push("Bundle does not include inventoryMovements".into());
    }

    let preview = ImportPreview {
        file_path: file_path.clone(),
        valid: warnings.is_empty(),
        manifest_version: manifest
            .get("bundleVersion")
            .and_then(|value| value.as_str())
            .map(ToString::to_string),
        bundle_type: manifest
            .get("bundleType")
            .and_then(|value| value.as_str())
            .map(ToString::to_string),
        source_patch_level: manifest
            .get("sourcePatchLevel")
            .and_then(|value| value.as_str())
            .map(ToString::to_string),
        business_count: businesses.len(),
        category_count: categories.len(),
        item_count: items.len(),
        movement_count: movements.len(),
        generated_at: manifest
            .get("generatedAt")
            .and_then(|value| value.as_str())
            .map(ToString::to_string),
        warnings,
    };

    db::with_connection(&app, |conn, _paths| {
        db::insert_import_job(
            conn,
            None,
            "json-workspace-foundation-v4",
            "previewed",
            &file_path,
        )?;
        db::insert_log(conn, "INFO", "import", "Import bundle previewed", None)?;
        Ok(())
    })?;

    Ok(preview)
}
