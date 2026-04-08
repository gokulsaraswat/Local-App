use std::{cmp, fs};

use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension, Row};
use tauri::AppHandle;
use uuid::Uuid;

use crate::domain::models::{
    AppInfo, BackupRecord, BusinessProfile, BusinessSettings, ExportJobRecord, ModuleFlag,
    PatchRecord, ReceiptProfile, RecentActivity, SequenceCounter, StorageStatus, TaxProfile,
    NewBusinessInput,
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

fn normalize_optional(value: &Option<String>) -> Option<String> {
    value.as_ref().and_then(|raw| {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
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

fn tax_profile_from_row(row: &Row) -> rusqlite::Result<TaxProfile> {
    Ok(TaxProfile {
        id: row.get(0)?,
        business_id: row.get(1)?,
        name: row.get(2)?,
        label: row.get(3)?,
        rate: row.get(4)?,
        is_default: bool_from_row(row, 5)?,
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
    })
}

fn receipt_profile_from_row(row: &Row) -> rusqlite::Result<ReceiptProfile> {
    Ok(ReceiptProfile {
        id: row.get(0)?,
        business_id: row.get(1)?,
        name: row.get(2)?,
        header_line1: row.get(3)?,
        header_line2: row.get(4)?,
        footer_text: row.get(5)?,
        show_address: bool_from_row(row, 6)?,
        show_phone: bool_from_row(row, 7)?,
        show_tax_breakdown: bool_from_row(row, 8)?,
        paper_width: row.get(9)?,
        copies: row.get(10)?,
        is_default: bool_from_row(row, 11)?,
        created_at: row.get(12)?,
        updated_at: row.get(13)?,
    })
}

fn module_flag_from_row(row: &Row) -> rusqlite::Result<ModuleFlag> {
    Ok(ModuleFlag {
        business_id: row.get(0)?,
        module_key: row.get(1)?,
        enabled: bool_from_row(row, 2)?,
        updated_at: row.get(3)?,
    })
}

fn sequence_counter_from_row(row: &Row) -> rusqlite::Result<SequenceCounter> {
    Ok(SequenceCounter {
        business_id: row.get(0)?,
        counter_key: row.get(1)?,
        prefix: row.get(2)?,
        next_number: row.get(3)?,
        padding: row.get(4)?,
        updated_at: row.get(5)?,
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
             ORDER BY CASE WHEN archived_at IS NULL THEN 0 ELSE 1 END, updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare businesses query", error))?;

    let rows = stmt
        .query_map([], business_from_row)
        .map_err(|error| to_command_error("failed to query businesses", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map businesses", error))
}

fn load_business_by_id(conn: &Connection, business_id: &str) -> Result<Option<BusinessProfile>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                id, name, legal_name, code, business_type, currency_code, tax_mode,
                phone, email, address_line1, address_line2, city, state, postal_code,
                country, created_at, updated_at, archived_at
             FROM businesses WHERE id = ?1 LIMIT 1",
        )
        .map_err(|error| to_command_error("failed to prepare business query", error))?;

    stmt.query_row(params![business_id], business_from_row)
        .optional()
        .map_err(|error| to_command_error("failed to load business by id", error))
}

pub fn get_business_by_id(conn: &Connection, business_id: &str) -> Result<BusinessProfile, String> {
    load_business_by_id(conn, business_id)?
        .ok_or_else(|| format!("business not found: {business_id}"))
}

pub fn get_active_business(conn: &Connection) -> Result<BusinessProfile, String> {
    if let Some(active_id) = get_meta(conn, "active_business_id")? {
        if let Some(business) = load_business_by_id(conn, &active_id)? {
            if business.archived_at.is_none() {
                return Ok(business);
            }
        }
    }

    let fallback = list_businesses(conn)?
        .into_iter()
        .find(|business| business.archived_at.is_none())
        .ok_or_else(|| "no businesses found in local storage".to_string())?;

    set_meta(conn, "active_business_id", &fallback.id)?;
    Ok(fallback)
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

pub fn list_all_business_settings(conn: &Connection) -> Result<Vec<BusinessSettings>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                business_id, timezone, locale, date_format, theme, tax_label,
                default_tax_rate, prices_include_tax, receipt_footer, receipt_show_address,
                receipt_show_phone, auto_backup_enabled, backup_directory,
                module_restaurant_enabled, module_retail_enabled, module_inventory_enabled,
                module_services_enabled, updated_at
             FROM business_settings
             ORDER BY updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare business settings list", error))?;

    let rows = stmt
        .query_map([], settings_from_row)
        .map_err(|error| to_command_error("failed to query business settings", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map business settings", error))
}

fn insert_business_settings_row(conn: &Connection, settings: &BusinessSettings) -> Result<(), String> {
    conn.execute(
        "INSERT INTO business_settings (
            business_id, timezone, locale, date_format, theme, tax_label,
            default_tax_rate, prices_include_tax, receipt_footer,
            receipt_show_address, receipt_show_phone, auto_backup_enabled,
            backup_directory, module_restaurant_enabled, module_retail_enabled,
            module_inventory_enabled, module_services_enabled, updated_at
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6,
            ?7, ?8, ?9,
            ?10, ?11, ?12,
            ?13, ?14, ?15,
            ?16, ?17, ?18
        )",
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
            &settings.backup_directory,
            if settings.module_restaurant_enabled { 1_i64 } else { 0_i64 },
            if settings.module_retail_enabled { 1_i64 } else { 0_i64 },
            if settings.module_inventory_enabled { 1_i64 } else { 0_i64 },
            if settings.module_services_enabled { 1_i64 } else { 0_i64 },
            &settings.updated_at
        ],
    )
    .map_err(|error| to_command_error("failed to insert business settings", error))?;
    Ok(())
}

pub fn create_business_profile(conn: &Connection, input: &NewBusinessInput) -> Result<BusinessProfile, String> {
    let name = input.name.trim().to_string();
    if name.is_empty() {
        return Err("business name cannot be empty".into());
    }

    let code = input.code.trim().to_uppercase();
    if code.is_empty() {
        return Err("business code cannot be empty".into());
    }

    let business_id = Uuid::new_v4().to_string();
    let now = now_iso();

    let template_business_id = if input.copy_from_active_business {
        get_meta(conn, "active_business_id")?
    } else {
        None
    };

    let template_settings = if let Some(template_id) = template_business_id.as_deref() {
        Some(get_business_settings(conn, template_id)?)
    } else {
        None
    };

    conn.execute(
        "INSERT INTO businesses (
            id, name, legal_name, code, business_type, currency_code, tax_mode,
            phone, email, address_line1, address_line2, city, state, postal_code,
            country, created_at, updated_at, archived_at
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7,
            ?8, ?9, ?10, ?11, ?12, ?13, ?14,
            ?15, ?16, ?17, ?18
        )",
        params![
            &business_id,
            &name,
            normalize_optional(&input.legal_name),
            &code,
            input.business_type.trim(),
            input.currency_code.trim().to_uppercase(),
            input.tax_mode.trim(),
            normalize_optional(&input.phone),
            normalize_optional(&input.email),
            normalize_optional(&input.address_line1),
            normalize_optional(&input.address_line2),
            normalize_optional(&input.city),
            normalize_optional(&input.state),
            normalize_optional(&input.postal_code),
            normalize_optional(&input.country),
            &now,
            &now,
            Option::<String>::None
        ],
    )
    .map_err(|error| to_command_error("failed to create business profile", error))?;

    let settings = BusinessSettings {
        business_id: business_id.clone(),
        timezone: template_settings
            .as_ref()
            .map(|value| value.timezone.clone())
            .unwrap_or_else(|| "Asia/Kolkata".into()),
        locale: template_settings
            .as_ref()
            .map(|value| value.locale.clone())
            .unwrap_or_else(|| "en-IN".into()),
        date_format: template_settings
            .as_ref()
            .map(|value| value.date_format.clone())
            .unwrap_or_else(|| "DD-MM-YYYY".into()),
        theme: template_settings
            .as_ref()
            .map(|value| value.theme.clone())
            .unwrap_or_else(|| "system".into()),
        tax_label: template_settings
            .as_ref()
            .map(|value| value.tax_label.clone())
            .unwrap_or_else(|| "GST".into()),
        default_tax_rate: template_settings
            .as_ref()
            .map(|value| value.default_tax_rate)
            .unwrap_or(0.0),
        prices_include_tax: template_settings
            .as_ref()
            .map(|value| value.prices_include_tax)
            .unwrap_or(false),
        receipt_footer: template_settings
            .as_ref()
            .and_then(|value| value.receipt_footer.clone())
            .or_else(|| Some("Thank you for supporting local business.".into())),
        receipt_show_address: template_settings
            .as_ref()
            .map(|value| value.receipt_show_address)
            .unwrap_or(true),
        receipt_show_phone: template_settings
            .as_ref()
            .map(|value| value.receipt_show_phone)
            .unwrap_or(true),
        auto_backup_enabled: template_settings
            .as_ref()
            .map(|value| value.auto_backup_enabled)
            .unwrap_or(false),
        backup_directory: template_settings
            .as_ref()
            .and_then(|value| value.backup_directory.clone()),
        module_restaurant_enabled: template_settings
            .as_ref()
            .map(|value| value.module_restaurant_enabled)
            .unwrap_or(false),
        module_retail_enabled: template_settings
            .as_ref()
            .map(|value| value.module_retail_enabled)
            .unwrap_or(true),
        module_inventory_enabled: template_settings
            .as_ref()
            .map(|value| value.module_inventory_enabled)
            .unwrap_or(true),
        module_services_enabled: template_settings
            .as_ref()
            .map(|value| value.module_services_enabled)
            .unwrap_or(false),
        updated_at: now.clone(),
    };

    insert_business_settings_row(conn, &settings)?;
    seed::ensure_workspace_support_for_business(conn, &business_id)
        .map_err(|error| to_command_error("failed to seed workspace support", error))?;

    if let Some(template_id) = template_business_id.as_deref() {
        if input.copy_from_active_business {
            seed::copy_workspace_preferences_from_template(conn, template_id, &business_id)
                .map_err(|error| to_command_error("failed to copy workspace preferences", error))?;
        }
    }

    if input.create_as_active {
        set_meta(conn, "active_business_id", &business_id)?;
        insert_log(
            conn,
            "INFO",
            "workspace",
            "Active business switched after creation",
            None,
        )?;
    }

    insert_log(conn, "INFO", "workspace", "Business workspace created", None)?;
    get_business_by_id(conn, &business_id)
}

pub fn switch_active_business(conn: &Connection, business_id: &str) -> Result<BusinessProfile, String> {
    let business = get_business_by_id(conn, business_id)?;
    if business.archived_at.is_some() {
        return Err("cannot switch to an archived business".into());
    }

    set_meta(conn, "active_business_id", business_id)?;
    insert_log(conn, "INFO", "workspace", "Active business switched", None)?;
    Ok(business)
}

pub fn save_business_profile(conn: &Connection, profile: &BusinessProfile) -> Result<BusinessProfile, String> {
    let updated_at = now_iso();
    let rows_affected = conn
        .execute(
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
                profile.name.trim(),
                normalize_optional(&profile.legal_name),
                profile.code.trim().to_uppercase(),
                profile.business_type.trim(),
                profile.currency_code.trim().to_uppercase(),
                profile.tax_mode.trim(),
                normalize_optional(&profile.phone),
                normalize_optional(&profile.email),
                normalize_optional(&profile.address_line1),
                normalize_optional(&profile.address_line2),
                normalize_optional(&profile.city),
                normalize_optional(&profile.state),
                normalize_optional(&profile.postal_code),
                normalize_optional(&profile.country),
                &updated_at
            ],
        )
        .map_err(|error| to_command_error("failed to update business profile", error))?;

    if rows_affected == 0 {
        return Err("business profile not found".into());
    }

    insert_log(conn, "INFO", "business", "Business profile updated", None)?;
    get_business_by_id(conn, &profile.id)
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

    let rows_affected = conn
        .execute(
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
                settings.timezone.trim(),
                settings.locale.trim(),
                settings.date_format.trim(),
                settings.theme.trim(),
                settings.tax_label.trim(),
                settings.default_tax_rate,
                if settings.prices_include_tax { 1_i64 } else { 0_i64 },
                normalize_optional(&settings.receipt_footer),
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

    if rows_affected == 0 {
        return Err("business settings not found".into());
    }

    insert_log(conn, "INFO", "settings", "Business settings updated", None)?;
    get_business_settings(conn, &settings.business_id)
}

pub fn list_tax_profiles(conn: &Connection, business_id: &str) -> Result<Vec<TaxProfile>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, business_id, name, label, rate, is_default, created_at, updated_at
             FROM tax_profiles
             WHERE business_id = ?1
             ORDER BY is_default DESC, updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare tax profiles query", error))?;

    let rows = stmt
        .query_map(params![business_id], tax_profile_from_row)
        .map_err(|error| to_command_error("failed to query tax profiles", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map tax profiles", error))
}

fn get_tax_profile_by_id(conn: &Connection, profile_id: &str) -> Result<TaxProfile, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, business_id, name, label, rate, is_default, created_at, updated_at
             FROM tax_profiles
             WHERE id = ?1
             LIMIT 1",
        )
        .map_err(|error| to_command_error("failed to prepare tax profile lookup", error))?;

    stmt.query_row(params![profile_id], tax_profile_from_row)
        .map_err(|error| to_command_error("failed to load tax profile", error))
}

pub fn list_all_tax_profiles(conn: &Connection) -> Result<Vec<TaxProfile>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, business_id, name, label, rate, is_default, created_at, updated_at
             FROM tax_profiles
             ORDER BY business_id ASC, is_default DESC, updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare all tax profiles query", error))?;

    let rows = stmt
        .query_map([], tax_profile_from_row)
        .map_err(|error| to_command_error("failed to query all tax profiles", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map all tax profiles", error))
}

pub fn save_tax_profile(conn: &Connection, profile: &TaxProfile) -> Result<TaxProfile, String> {
    let now = now_iso();
    let profile_id = if profile.id.trim().is_empty() {
        Uuid::new_v4().to_string()
    } else {
        profile.id.clone()
    };

    let created_at = conn
        .query_row(
            "SELECT created_at FROM tax_profiles WHERE id = ?1 LIMIT 1",
            params![&profile_id],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|error| to_command_error("failed to inspect tax profile", error))?
        .unwrap_or_else(|| now.clone());

    if profile.is_default {
        conn.execute(
            "UPDATE tax_profiles SET is_default = 0, updated_at = ?2 WHERE business_id = ?1",
            params![&profile.business_id, &now],
        )
        .map_err(|error| to_command_error("failed to clear default tax profile", error))?;
    }

    conn.execute(
        "INSERT INTO tax_profiles (id, business_id, name, label, rate, is_default, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
         ON CONFLICT(id) DO UPDATE SET
             business_id = excluded.business_id,
             name = excluded.name,
             label = excluded.label,
             rate = excluded.rate,
             is_default = excluded.is_default,
             updated_at = excluded.updated_at",
        params![
            &profile_id,
            &profile.business_id,
            profile.name.trim(),
            profile.label.trim(),
            profile.rate,
            if profile.is_default { 1_i64 } else { 0_i64 },
            &created_at,
            &now
        ],
    )
    .map_err(|error| to_command_error("failed to save tax profile", error))?;

    conn.execute(
        "UPDATE business_settings
         SET tax_label = ?2, default_tax_rate = ?3, updated_at = ?4
         WHERE business_id = ?1",
        params![&profile.business_id, profile.label.trim(), profile.rate, &now],
    )
    .map_err(|error| to_command_error("failed to sync business tax defaults", error))?;

    insert_log(conn, "INFO", "settings", "Default tax profile updated", None)?;
    get_tax_profile_by_id(conn, &profile_id)
}

pub fn list_receipt_profiles(conn: &Connection, business_id: &str) -> Result<Vec<ReceiptProfile>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                id, business_id, name, header_line1, header_line2, footer_text,
                show_address, show_phone, show_tax_breakdown, paper_width,
                copies, is_default, created_at, updated_at
             FROM receipt_profiles
             WHERE business_id = ?1
             ORDER BY is_default DESC, updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare receipt profiles query", error))?;

    let rows = stmt
        .query_map(params![business_id], receipt_profile_from_row)
        .map_err(|error| to_command_error("failed to query receipt profiles", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map receipt profiles", error))
}

fn get_receipt_profile_by_id(conn: &Connection, profile_id: &str) -> Result<ReceiptProfile, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                id, business_id, name, header_line1, header_line2, footer_text,
                show_address, show_phone, show_tax_breakdown, paper_width,
                copies, is_default, created_at, updated_at
             FROM receipt_profiles
             WHERE id = ?1
             LIMIT 1",
        )
        .map_err(|error| to_command_error("failed to prepare receipt profile lookup", error))?;

    stmt.query_row(params![profile_id], receipt_profile_from_row)
        .map_err(|error| to_command_error("failed to load receipt profile", error))
}

pub fn list_all_receipt_profiles(conn: &Connection) -> Result<Vec<ReceiptProfile>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                id, business_id, name, header_line1, header_line2, footer_text,
                show_address, show_phone, show_tax_breakdown, paper_width,
                copies, is_default, created_at, updated_at
             FROM receipt_profiles
             ORDER BY business_id ASC, is_default DESC, updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare all receipt profiles query", error))?;

    let rows = stmt
        .query_map([], receipt_profile_from_row)
        .map_err(|error| to_command_error("failed to query all receipt profiles", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map all receipt profiles", error))
}

pub fn save_receipt_profile(conn: &Connection, profile: &ReceiptProfile) -> Result<ReceiptProfile, String> {
    let now = now_iso();
    let profile_id = if profile.id.trim().is_empty() {
        Uuid::new_v4().to_string()
    } else {
        profile.id.clone()
    };

    let created_at = conn
        .query_row(
            "SELECT created_at FROM receipt_profiles WHERE id = ?1 LIMIT 1",
            params![&profile_id],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|error| to_command_error("failed to inspect receipt profile", error))?
        .unwrap_or_else(|| now.clone());

    if profile.is_default {
        conn.execute(
            "UPDATE receipt_profiles SET is_default = 0, updated_at = ?2 WHERE business_id = ?1",
            params![&profile.business_id, &now],
        )
        .map_err(|error| to_command_error("failed to clear default receipt profile", error))?;
    }

    conn.execute(
        "INSERT INTO receipt_profiles (
            id, business_id, name, header_line1, header_line2, footer_text,
            show_address, show_phone, show_tax_breakdown, paper_width,
            copies, is_default, created_at, updated_at
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6,
            ?7, ?8, ?9, ?10,
            ?11, ?12, ?13, ?14
        )
        ON CONFLICT(id) DO UPDATE SET
            business_id = excluded.business_id,
            name = excluded.name,
            header_line1 = excluded.header_line1,
            header_line2 = excluded.header_line2,
            footer_text = excluded.footer_text,
            show_address = excluded.show_address,
            show_phone = excluded.show_phone,
            show_tax_breakdown = excluded.show_tax_breakdown,
            paper_width = excluded.paper_width,
            copies = excluded.copies,
            is_default = excluded.is_default,
            updated_at = excluded.updated_at",
        params![
            &profile_id,
            &profile.business_id,
            profile.name.trim(),
            normalize_optional(&profile.header_line1),
            normalize_optional(&profile.header_line2),
            normalize_optional(&profile.footer_text),
            if profile.show_address { 1_i64 } else { 0_i64 },
            if profile.show_phone { 1_i64 } else { 0_i64 },
            if profile.show_tax_breakdown { 1_i64 } else { 0_i64 },
            profile.paper_width.trim(),
            cmp::max(1, profile.copies),
            if profile.is_default { 1_i64 } else { 0_i64 },
            &created_at,
            &now
        ],
    )
    .map_err(|error| to_command_error("failed to save receipt profile", error))?;

    conn.execute(
        "UPDATE business_settings
         SET receipt_footer = ?2,
             receipt_show_address = ?3,
             receipt_show_phone = ?4,
             updated_at = ?5
         WHERE business_id = ?1",
        params![
            &profile.business_id,
            normalize_optional(&profile.footer_text),
            if profile.show_address { 1_i64 } else { 0_i64 },
            if profile.show_phone { 1_i64 } else { 0_i64 },
            &now
        ],
    )
    .map_err(|error| to_command_error("failed to sync business receipt defaults", error))?;

    insert_log(conn, "INFO", "settings", "Default receipt profile updated", None)?;
    get_receipt_profile_by_id(conn, &profile_id)
}

pub fn list_module_flags(conn: &Connection, business_id: &str) -> Result<Vec<ModuleFlag>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT business_id, module_key, enabled, updated_at
             FROM module_flags
             WHERE business_id = ?1
             ORDER BY module_key ASC",
        )
        .map_err(|error| to_command_error("failed to prepare module flags query", error))?;

    let rows = stmt
        .query_map(params![business_id], module_flag_from_row)
        .map_err(|error| to_command_error("failed to query module flags", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map module flags", error))
}

pub fn list_all_module_flags(conn: &Connection) -> Result<Vec<ModuleFlag>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT business_id, module_key, enabled, updated_at
             FROM module_flags
             ORDER BY business_id ASC, module_key ASC",
        )
        .map_err(|error| to_command_error("failed to prepare all module flags query", error))?;

    let rows = stmt
        .query_map([], module_flag_from_row)
        .map_err(|error| to_command_error("failed to query all module flags", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map all module flags", error))
}

fn module_flag_value(flags: &[ModuleFlag], key: &str, fallback: bool) -> bool {
    flags.iter()
        .find(|flag| flag.module_key == key)
        .map(|flag| flag.enabled)
        .unwrap_or(fallback)
}

pub fn save_module_flags(
    conn: &Connection,
    business_id: &str,
    flags: &[ModuleFlag],
) -> Result<Vec<ModuleFlag>, String> {
    let now = now_iso();
    for flag in flags {
        conn.execute(
            "INSERT INTO module_flags (business_id, module_key, enabled, updated_at)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(business_id, module_key)
             DO UPDATE SET enabled = excluded.enabled, updated_at = excluded.updated_at",
            params![
                business_id,
                flag.module_key.trim(),
                if flag.enabled { 1_i64 } else { 0_i64 },
                &now
            ],
        )
        .map_err(|error| to_command_error("failed to save module flag", error))?;
    }

    let current_settings = get_business_settings(conn, business_id)?;
    conn.execute(
        "UPDATE business_settings
         SET module_restaurant_enabled = ?2,
             module_retail_enabled = ?3,
             module_inventory_enabled = ?4,
             module_services_enabled = ?5,
             updated_at = ?6
         WHERE business_id = ?1",
        params![
            business_id,
            if module_flag_value(flags, "restaurant", current_settings.module_restaurant_enabled) { 1_i64 } else { 0_i64 },
            if module_flag_value(flags, "retail", current_settings.module_retail_enabled) { 1_i64 } else { 0_i64 },
            if module_flag_value(flags, "inventory", current_settings.module_inventory_enabled) { 1_i64 } else { 0_i64 },
            if module_flag_value(flags, "services", current_settings.module_services_enabled) { 1_i64 } else { 0_i64 },
            &now
        ],
    )
    .map_err(|error| to_command_error("failed to sync module flags to business settings", error))?;

    insert_log(conn, "INFO", "settings", "Module flags updated", None)?;
    list_module_flags(conn, business_id)
}

pub fn list_sequence_counters(conn: &Connection, business_id: &str) -> Result<Vec<SequenceCounter>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT business_id, counter_key, prefix, next_number, padding, updated_at
             FROM sequence_counters
             WHERE business_id = ?1
             ORDER BY counter_key ASC",
        )
        .map_err(|error| to_command_error("failed to prepare sequence counters query", error))?;

    let rows = stmt
        .query_map(params![business_id], sequence_counter_from_row)
        .map_err(|error| to_command_error("failed to query sequence counters", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map sequence counters", error))
}

pub fn list_all_sequence_counters(conn: &Connection) -> Result<Vec<SequenceCounter>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT business_id, counter_key, prefix, next_number, padding, updated_at
             FROM sequence_counters
             ORDER BY business_id ASC, counter_key ASC",
        )
        .map_err(|error| to_command_error("failed to prepare all sequence counters query", error))?;

    let rows = stmt
        .query_map([], sequence_counter_from_row)
        .map_err(|error| to_command_error("failed to query all sequence counters", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map all sequence counters", error))
}

pub fn save_sequence_counters(
    conn: &Connection,
    business_id: &str,
    counters: &[SequenceCounter],
) -> Result<Vec<SequenceCounter>, String> {
    let now = now_iso();
    for counter in counters {
        conn.execute(
            "INSERT INTO sequence_counters (business_id, counter_key, prefix, next_number, padding, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(business_id, counter_key)
             DO UPDATE SET prefix = excluded.prefix, next_number = excluded.next_number, padding = excluded.padding, updated_at = excluded.updated_at",
            params![
                business_id,
                counter.counter_key.trim(),
                counter.prefix.trim().to_uppercase(),
                cmp::max(1, counter.next_number),
                cmp::max(1, counter.padding),
                &now
            ],
        )
        .map_err(|error| to_command_error("failed to save sequence counter", error))?;
    }

    insert_log(conn, "INFO", "settings", "Sequence counters updated", None)?;
    list_sequence_counters(conn, business_id)
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
        version: get_meta(conn, "app_version")?.unwrap_or_else(|| "0.2.0".into()),
        schema_version: get_meta(conn, "schema_version")?
            .and_then(|value| value.parse::<i64>().ok())
            .unwrap_or(CURRENT_SCHEMA_VERSION),
        patch_level: get_meta(conn, "patch_level")?.unwrap_or_else(|| patching::PATCH_ID.into()),
        initialized_at: get_meta(conn, "initialized_at")?.unwrap_or_else(now_iso),
    })
}
