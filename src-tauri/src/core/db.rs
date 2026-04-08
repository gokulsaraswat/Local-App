use std::fs;

use chrono::Utc;
use rusqlite::{params, Connection, Row};
use tauri::AppHandle;
use uuid::Uuid;

use crate::domain::models::{
    AppInfo, BackupRecord, BusinessProfile, BusinessSettings, ExportJobRecord, PatchRecord,
    RecentActivity, StorageStatus,
};

use super::{
    error::to_command_error,
    migrations::{self, CURRENT_SCHEMA_VERSION},
    patching,
    paths::{ensure_directories, resolve_paths, AppPaths},
    seed,
};

pub fn initialize(app: &AppHandle) -> Result<(), String> {
    let paths = resolve_paths(app)?;
    ensure_directories(&paths)?;
    let conn = open_connection(&paths).map_err(|error| to_command_error("failed to open database", error))?;
    migrations::run(&conn).map_err(|error| to_command_error("failed to run migrations", error))?;
    patching::register_patch(&conn)
        .map_err(|error| to_command_error("failed to register patch history", error))?;
    seed::seed_if_empty(&conn).map_err(|error| to_command_error("failed to seed base data", error))?;
    Ok(())
}

pub fn with_connection<T, F>(app: &AppHandle, action: F) -> Result<T, String>
where
    F: FnOnce(&Connection, &AppPaths) -> Result<T, String>,
{
    let paths = resolve_paths(app)?;
    ensure_directories(&paths)?;
    let conn = open_connection(&paths).map_err(|error| to_command_error("failed to open database", error))?;
    action(&conn, &paths)
}

pub fn open_connection(paths: &AppPaths) -> rusqlite::Result<Connection> {
    let conn = Connection::open(&paths.database_path)?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.pragma_update(None, "journal_mode", "DELETE")?;
    conn.pragma_update(None, "synchronous", "NORMAL")?;
    Ok(conn)
}

fn bool_from_row(row: &Row, index: usize) -> rusqlite::Result<bool> {
    let value: i64 = row.get(index)?;
    Ok(value != 0)
}

fn business_from_row(row: &Row) -> rusqlite::Result<BusinessProfile> {
    Ok(BusinessProfile {
        id: row.get(0)?,
        name: row.get(1)?,
        legal_name: row.get(2)?,
        code: row.get(3)?,
        business_type: row.get(4)?,
        currency_code: row.get(5)?,
        tax_mode: row.get(6)?,
        phone: row.get(7)?,
        email: row.get(8)?,
        address_line1: row.get(9)?,
        address_line2: row.get(10)?,
        city: row.get(11)?,
        state: row.get(12)?,
        postal_code: row.get(13)?,
        country: row.get(14)?,
        created_at: row.get(15)?,
        updated_at: row.get(16)?,
        archived_at: row.get(17)?,
    })
}

fn settings_from_row(row: &Row) -> rusqlite::Result<BusinessSettings> {
    Ok(BusinessSettings {
        business_id: row.get(0)?,
        timezone: row.get(1)?,
        locale: row.get(2)?,
        date_format: row.get(3)?,
        theme: row.get(4)?,
        tax_label: row.get(5)?,
        default_tax_rate: row.get(6)?,
        prices_include_tax: bool_from_row(row, 7)?,
        receipt_footer: row.get(8)?,
        receipt_show_address: bool_from_row(row, 9)?,
        receipt_show_phone: bool_from_row(row, 10)?,
        auto_backup_enabled: bool_from_row(row, 11)?,
        backup_directory: row.get(12)?,
        module_restaurant_enabled: bool_from_row(row, 13)?,
        module_retail_enabled: bool_from_row(row, 14)?,
        module_inventory_enabled: bool_from_row(row, 15)?,
        module_services_enabled: bool_from_row(row, 16)?,
        updated_at: row.get(17)?,
    })
}

fn backup_from_row(row: &Row) -> rusqlite::Result<BackupRecord> {
    Ok(BackupRecord {
        id: row.get(0)?,
        business_id: row.get(1)?,
        file_name: row.get(2)?,
        file_path: row.get(3)?,
        checksum: row.get(4)?,
        status: row.get(5)?,
        created_at: row.get(6)?,
    })
}

fn patch_from_row(row: &Row) -> rusqlite::Result<PatchRecord> {
    Ok(PatchRecord {
        patch_id: row.get(0)?,
        patch_name: row.get(1)?,
        schema_version: row.get(2)?,
        applied_at: row.get(3)?,
    })
}

fn activity_from_row(row: &Row) -> rusqlite::Result<RecentActivity> {
    Ok(RecentActivity {
        id: row.get(0)?,
        level: row.get(1)?,
        category: row.get(2)?,
        message: row.get(3)?,
        created_at: row.get(4)?,
    })
}

pub fn now_iso() -> String {
    Utc::now().to_rfc3339()
}

pub fn get_meta(conn: &Connection, key: &str) -> Result<Option<String>, String> {
    let mut stmt = conn
        .prepare("SELECT value FROM app_meta WHERE key = ?1")
        .map_err(|error| to_command_error("failed to prepare meta lookup", error))?;

    let mut rows = stmt
        .query(params![key])
        .map_err(|error| to_command_error("failed to query meta value", error))?;

    if let Some(row) = rows
        .next()
        .map_err(|error| to_command_error("failed to iterate meta rows", error))?
    {
        let value: String = row
            .get(0)
            .map_err(|error| to_command_error("failed to read meta row", error))?;
        Ok(Some(value))
    } else {
        Ok(None)
    }
}

pub fn set_meta(conn: &Connection, key: &str, value: &str) -> Result<(), String> {
    conn.execute(
        "INSERT INTO app_meta (key, value, updated_at)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
        params![key, value, now_iso()],
    )
    .map_err(|error| to_command_error("failed to write meta value", error))?;
    Ok(())
}

pub fn insert_log(
    conn: &Connection,
    level: &str,
    category: &str,
    message: &str,
    payload_json: Option<&str>,
) -> Result<(), String> {
    conn.execute(
        "INSERT INTO app_logs (id, level, category, message, payload_json, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            Uuid::new_v4().to_string(),
            level,
            category,
            message,
            payload_json,
            now_iso()
        ],
    )
    .map_err(|error| to_command_error("failed to insert app log", error))?;
    Ok(())
}

pub fn list_recent_activity(conn: &Connection, limit: usize) -> Result<Vec<RecentActivity>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, level, category, message, created_at
             FROM app_logs
             ORDER BY created_at DESC
             LIMIT ?1",
        )
        .map_err(|error| to_command_error("failed to prepare activity query", error))?;

    let rows = stmt
        .query_map(params![limit as i64], activity_from_row)
        .map_err(|error| to_command_error("failed to query recent activity", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map recent activity", error))
}

pub fn list_businesses(conn: &Connection) -> Result<Vec<BusinessProfile>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                id, name, legal_name, code, business_type, currency_code, tax_mode,
                phone, email, address_line1, address_line2, city, state, postal_code,
                country, created_at, updated_at, archived_at
             FROM businesses
             ORDER BY created_at ASC",
        )
        .map_err(|error| to_command_error("failed to prepare businesses query", error))?;

    let rows = stmt
        .query_map([], business_from_row)
        .map_err(|error| to_command_error("failed to query businesses", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map businesses", error))
}

pub fn get_active_business(conn: &Connection) -> Result<BusinessProfile, String> {
    if let Some(active_id) = get_meta(conn, "active_business_id")? {
        let mut stmt = conn
            .prepare(
                "SELECT
                    id, name, legal_name, code, business_type, currency_code, tax_mode,
                    phone, email, address_line1, address_line2, city, state, postal_code,
                    country, created_at, updated_at, archived_at
                 FROM businesses WHERE id = ?1 LIMIT 1",
            )
            .map_err(|error| to_command_error("failed to prepare active business query", error))?;

        let business = stmt
            .query_row(params![active_id], business_from_row)
            .map_err(|error| to_command_error("failed to load active business", error))?;

        return Ok(business);
    }

    let businesses = list_businesses(conn)?;
    businesses
        .into_iter()
        .next()
        .ok_or_else(|| "no businesses found in local storage".to_string())
}

pub fn get_business_settings(conn: &Connection, business_id: &str) -> Result<BusinessSettings, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                business_id, timezone, locale, date_format, theme, tax_label,
                default_tax_rate, prices_include_tax, receipt_footer, receipt_show_address,
                receipt_show_phone, auto_backup_enabled, backup_directory,
                module_restaurant_enabled, module_retail_enabled, module_inventory_enabled,
                module_services_enabled, updated_at
             FROM business_settings
             WHERE business_id = ?1
             LIMIT 1",
        )
        .map_err(|error| to_command_error("failed to prepare business settings query", error))?;

    stmt.query_row(params![business_id], settings_from_row)
        .map_err(|error| to_command_error("failed to load business settings", error))
}

pub fn save_business_profile(conn: &Connection, profile: &BusinessProfile) -> Result<BusinessProfile, String> {
    let updated_at = now_iso();

    conn.execute(
        "UPDATE businesses
         SET name = ?2,
             legal_name = ?3,
             code = ?4,
             business_type = ?5,
             currency_code = ?6,
             tax_mode = ?7,
             phone = ?8,
             email = ?9,
             address_line1 = ?10,
             address_line2 = ?11,
             city = ?12,
             state = ?13,
             postal_code = ?14,
             country = ?15,
             updated_at = ?16
         WHERE id = ?1",
        params![
            &profile.id,
            &profile.name,
            &profile.legal_name,
            &profile.code,
            &profile.business_type,
            &profile.currency_code,
            &profile.tax_mode,
            &profile.phone,
            &profile.email,
            &profile.address_line1,
            &profile.address_line2,
            &profile.city,
            &profile.state,
            &profile.postal_code,
            &profile.country,
            &updated_at
        ],
    )
    .map_err(|error| to_command_error("failed to update business profile", error))?;

    insert_log(conn, "INFO", "business", "Business profile updated", None)?;
    get_active_business(conn)
}

pub fn save_business_settings(
    conn: &Connection,
    settings: &BusinessSettings,
) -> Result<BusinessSettings, String> {
    let updated_at = now_iso();
    let backup_directory = settings
        .backup_directory
        .clone()
        .and_then(|value| if value.trim().is_empty() { None } else { Some(value) });

    conn.execute(
        "UPDATE business_settings
         SET timezone = ?2,
             locale = ?3,
             date_format = ?4,
             theme = ?5,
             tax_label = ?6,
             default_tax_rate = ?7,
             prices_include_tax = ?8,
             receipt_footer = ?9,
             receipt_show_address = ?10,
             receipt_show_phone = ?11,
             auto_backup_enabled = ?12,
             backup_directory = ?13,
             module_restaurant_enabled = ?14,
             module_retail_enabled = ?15,
             module_inventory_enabled = ?16,
             module_services_enabled = ?17,
             updated_at = ?18
         WHERE business_id = ?1",
        params![
            &settings.business_id,
            &settings.timezone,
            &settings.locale,
            &settings.date_format,
            &settings.theme,
            &settings.tax_label,
            settings.default_tax_rate,
            if settings.prices_include_tax { 1_i64 } else { 0_i64 },
            &settings.receipt_footer,
            if settings.receipt_show_address { 1_i64 } else { 0_i64 },
            if settings.receipt_show_phone { 1_i64 } else { 0_i64 },
            if settings.auto_backup_enabled { 1_i64 } else { 0_i64 },
            &backup_directory,
            if settings.module_restaurant_enabled { 1_i64 } else { 0_i64 },
            if settings.module_retail_enabled { 1_i64 } else { 0_i64 },
            if settings.module_inventory_enabled { 1_i64 } else { 0_i64 },
            if settings.module_services_enabled { 1_i64 } else { 0_i64 },
            &updated_at
        ],
    )
    .map_err(|error| to_command_error("failed to update business settings", error))?;

    insert_log(conn, "INFO", "settings", "Business settings updated", None)?;
    get_business_settings(conn, &settings.business_id)
}

pub fn list_patch_history(conn: &Connection) -> Result<Vec<PatchRecord>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT patch_id, patch_name, schema_version, applied_at
             FROM patch_history
             ORDER BY applied_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare patch history query", error))?;

    let rows = stmt
        .query_map([], patch_from_row)
        .map_err(|error| to_command_error("failed to query patch history", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map patch history", error))
}

pub fn list_backups(conn: &Connection) -> Result<Vec<BackupRecord>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, business_id, file_name, file_path, checksum, status, created_at
             FROM backup_records
             ORDER BY created_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare backup query", error))?;

    let rows = stmt
        .query_map([], backup_from_row)
        .map_err(|error| to_command_error("failed to query backups", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map backups", error))
}

pub fn insert_backup_record(conn: &Connection, record: &BackupRecord) -> Result<(), String> {
    conn.execute(
        "INSERT INTO backup_records (id, business_id, file_name, file_path, checksum, status, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            &record.id,
            &record.business_id,
            &record.file_name,
            &record.file_path,
            &record.checksum,
            &record.status,
            &record.created_at
        ],
    )
    .map_err(|error| to_command_error("failed to insert backup record", error))?;
    Ok(())
}

pub fn insert_export_job(conn: &Connection, record: &ExportJobRecord) -> Result<(), String> {
    conn.execute(
        "INSERT INTO export_jobs (id, business_id, format, status, target_path, created_at, completed_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            &record.id,
            &record.business_id,
            &record.format,
            &record.status,
            &record.target_path,
            &record.created_at,
            &record.completed_at
        ],
    )
    .map_err(|error| to_command_error("failed to insert export job", error))?;
    Ok(())
}

pub fn insert_import_job(
    conn: &Connection,
    business_id: Option<&str>,
    format: &str,
    status: &str,
    source_path: &str,
) -> Result<(), String> {
    conn.execute(
        "INSERT INTO import_jobs (id, business_id, format, status, source_path, created_at, completed_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            Uuid::new_v4().to_string(),
            business_id,
            format,
            status,
            source_path,
            now_iso(),
            Option::<String>::None
        ],
    )
    .map_err(|error| to_command_error("failed to insert import job", error))?;
    Ok(())
}

pub fn build_storage_status(conn: &Connection, paths: &AppPaths) -> Result<StorageStatus, String> {
    let export_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM export_jobs", [], |row| row.get(0))
        .map_err(|error| to_command_error("failed to count export jobs", error))?;

    let backup_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM backup_records", [], |row| row.get(0))
        .map_err(|error| to_command_error("failed to count backup records", error))?;

    Ok(StorageStatus {
        data_dir: paths.data_dir.clone(),
        config_dir: paths.config_dir.clone(),
        log_dir: paths.log_dir.clone(),
        backup_dir: paths.backup_dir.clone(),
        export_dir: paths.export_dir.clone(),
        database_path: paths.database_path.clone(),
        database_exists: fs::metadata(&paths.database_path).is_ok(),
        backup_count: backup_count as usize,
        export_count: export_count as usize,
    })
}

pub fn load_app_info(conn: &Connection) -> Result<AppInfo, String> {
    Ok(AppInfo {
        app_name: get_meta(conn, "app_name")?.unwrap_or_else(|| "local-first-business-manager".into()),
        product_name: get_meta(conn, "product_name")?.unwrap_or_else(|| "Local Business Manager".into()),
        version: get_meta(conn, "app_version")?.unwrap_or_else(|| "0.1.0".into()),
        schema_version: get_meta(conn, "schema_version")?
            .and_then(|value| value.parse::<i64>().ok())
            .unwrap_or(CURRENT_SCHEMA_VERSION),
        patch_level: get_meta(conn, "patch_level")?.unwrap_or_else(|| "P001_foundation_base_structure".into()),
        initialized_at: get_meta(conn, "initialized_at")?.unwrap_or_else(now_iso),
    })
}
