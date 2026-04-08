use std::fs;

use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension, Row};
use tauri::AppHandle;
use uuid::Uuid;

use crate::domain::models::{
    AppInfo, BackupRecord, BusinessProfile, BusinessSettings, BusinessWorkspaceSummary,
    ExportJobRecord, ModuleFlags, NewBusinessWorkspaceInput, PatchRecord, ReceiptProfile,
    RecentActivity, SequenceCounter, StorageStatus, TaxProfile, WorkspaceConfigurationInput,
};

use super::{
    error::to_command_error,
    inventory,
    migrations::{self, CURRENT_SCHEMA_VERSION},
    patching,
    paths::{ensure_directories, resolve_paths, AppPaths},
    seed,
};

pub fn initialize(app: &AppHandle) -> Result<(), String> {
    let paths = resolve_paths(app)?;
    ensure_directories(&paths)?;
    let conn = open_connection(&paths)
        .map_err(|error| to_command_error("failed to open local database", error))?;
    migrations::run(&conn).map_err(|error| to_command_error("failed to run migrations", error))?;
    patching::register_patch(&conn)
        .map_err(|error| to_command_error("failed to register patch history", error))?;
    seed::seed_if_empty(&conn)
        .map_err(|error| to_command_error("failed to seed local data", error))?;
    inventory::backfill_opening_balances(&conn)
        .map_err(|error| to_command_error("failed to backfill opening stock balances", error))?;
    Ok(())
}

pub fn with_connection<T, F>(app: &AppHandle, action: F) -> Result<T, String>
where
    F: FnOnce(&Connection, &AppPaths) -> Result<T, String>,
{
    let paths = resolve_paths(app)?;
    ensure_directories(&paths)?;
    let conn = open_connection(&paths)
        .map_err(|error| to_command_error("failed to open local database", error))?;
    action(&conn, &paths)
}

pub fn open_connection(paths: &AppPaths) -> rusqlite::Result<Connection> {
    let conn = Connection::open(&paths.database_path)?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.pragma_update(None, "journal_mode", "DELETE")?;
    conn.pragma_update(None, "synchronous", "NORMAL")?;
    Ok(conn)
}

pub fn now_iso() -> String {
    Utc::now().to_rfc3339()
}

fn bool_from_row(row: &Row, index: usize) -> rusqlite::Result<bool> {
    let value: i64 = row.get(index)?;
    Ok(value != 0)
}

fn bool_to_i64(value: bool) -> i64 {
    if value {
        1
    } else {
        0
    }
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

fn normalize_text(value: &str, field: &str) -> Result<String, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(format!("{field} cannot be empty"));
    }
    Ok(trimmed.to_string())
}

fn normalize_code(value: &str, field: &str) -> Result<String, String> {
    let normalized = normalize_text(value, field)?;
    Ok(normalized.to_uppercase())
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

fn tax_profile_from_row(row: &Row) -> rusqlite::Result<TaxProfile> {
    Ok(TaxProfile {
        id: row.get(0)?,
        business_id: row.get(1)?,
        name: row.get(2)?,
        tax_label: row.get(3)?,
        default_rate: row.get(4)?,
        prices_include_tax: bool_from_row(row, 5)?,
        is_default: bool_from_row(row, 6)?,
        updated_at: row.get(7)?,
    })
}

fn receipt_profile_from_row(row: &Row) -> rusqlite::Result<ReceiptProfile> {
    Ok(ReceiptProfile {
        id: row.get(0)?,
        business_id: row.get(1)?,
        name: row.get(2)?,
        footer_text: row.get(3)?,
        show_address: bool_from_row(row, 4)?,
        show_phone: bool_from_row(row, 5)?,
        show_email: bool_from_row(row, 6)?,
        show_business_code: bool_from_row(row, 7)?,
        paper_width: row.get(8)?,
        is_default: bool_from_row(row, 9)?,
        updated_at: row.get(10)?,
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

fn sequence_from_row(row: &Row) -> rusqlite::Result<SequenceCounter> {
    let business_id: String = row.get(0)?;
    let scope: String = row.get(1)?;
    Ok(SequenceCounter {
        id: format!("{business_id}:{scope}"),
        business_id,
        scope,
        prefix: row.get(2)?,
        next_number: row.get(3)?,
        padding: row.get(4)?,
        reset_policy: row.get(5)?,
        updated_at: row.get(6)?,
    })
}

pub fn get_meta(conn: &Connection, key: &str) -> Result<Option<String>, String> {
    conn.query_row(
        "SELECT value FROM app_meta WHERE key = ?1 LIMIT 1",
        params![key],
        |row| row.get::<_, String>(0),
    )
    .optional()
    .map_err(|error| to_command_error("failed to read app meta", error))
}

pub fn set_meta(conn: &Connection, key: &str, value: &str) -> Result<(), String> {
    conn.execute(
        "INSERT INTO app_meta (key, value, updated_at)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
        params![key, value, now_iso()],
    )
    .map_err(|error| to_command_error("failed to write app meta", error))?;
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

pub fn load_app_info(conn: &Connection) -> Result<AppInfo, String> {
    let app_name =
        get_meta(conn, "app_name")?.unwrap_or_else(|| "local-first-business-manager".into());
    let product_name =
        get_meta(conn, "product_name")?.unwrap_or_else(|| "Local Business Manager".into());
    let version = get_meta(conn, "app_version")?.unwrap_or_else(|| "0.4.0".into());
    let initialized_at = get_meta(conn, "initialized_at")?.unwrap_or_else(now_iso);
    let patch_level =
        get_meta(conn, "patch_level")?.unwrap_or_else(|| patching::PATCH_ID.to_string());
    let schema_version = get_meta(conn, "schema_version")?
        .and_then(|value| value.parse::<i64>().ok())
        .unwrap_or(CURRENT_SCHEMA_VERSION);

    Ok(AppInfo {
        app_name,
        product_name,
        version,
        schema_version,
        patch_level,
        initialized_at,
    })
}

pub fn list_patch_history(conn: &Connection) -> Result<Vec<PatchRecord>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT patch_id, patch_name, schema_version, applied_at
             FROM patch_history
             ORDER BY schema_version ASC, applied_at ASC",
        )
        .map_err(|error| to_command_error("failed to prepare patch history query", error))?;

    let rows = stmt
        .query_map([], patch_from_row)
        .map_err(|error| to_command_error("failed to query patch history", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map patch history", error))
}

pub fn list_recent_activity(
    conn: &Connection,
    limit: usize,
) -> Result<Vec<RecentActivity>, String> {
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

pub fn get_business_by_id(conn: &Connection, business_id: &str) -> Result<BusinessProfile, String> {
    conn.query_row(
        "SELECT
            id, name, legal_name, code, business_type, currency_code, tax_mode,
            phone, email, address_line1, address_line2, city, state, postal_code,
            country, created_at, updated_at, archived_at
         FROM businesses
         WHERE id = ?1
         LIMIT 1",
        params![business_id],
        business_from_row,
    )
    .map_err(|error| to_command_error("failed to load business profile", error))
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
        .map_err(|error| to_command_error("failed to prepare business list query", error))?;

    let rows = stmt
        .query_map([], business_from_row)
        .map_err(|error| to_command_error("failed to query businesses", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map businesses", error))
}

pub fn get_active_business(conn: &Connection) -> Result<BusinessProfile, String> {
    if let Some(active_business_id) = get_meta(conn, "active_business_id")? {
        if let Ok(business) = get_business_by_id(conn, &active_business_id) {
            if business.archived_at.is_none() {
                return Ok(business);
            }
        }
    }

    let businesses = list_businesses(conn)?;
    if let Some(business) = businesses
        .iter()
        .find(|item| item.archived_at.is_none())
        .cloned()
        .or_else(|| businesses.first().cloned())
    {
        set_meta(conn, "active_business_id", &business.id)?;
        return Ok(business);
    }

    Err("no business profile is available".into())
}

fn default_module_booleans(business_type: &str) -> (bool, bool, bool, bool) {
    let normalized = business_type.to_lowercase();
    let restaurant = normalized.contains("restaurant")
        || normalized.contains("cafe")
        || normalized.contains("bakery")
        || normalized.contains("food");
    let services = normalized.contains("service");
    let retail = !services;
    (restaurant, retail, true, services)
}

pub fn create_business_workspace(
    conn: &Connection,
    input: &NewBusinessWorkspaceInput,
) -> Result<BusinessProfile, String> {
    let business_id = Uuid::new_v4().to_string();
    let now = now_iso();
    let name = normalize_text(&input.name, "business name")?;
    let code = normalize_code(&input.code, "business code")?;
    let business_type = normalize_text(&input.business_type, "business type")?;
    let currency_code = normalize_code(&input.currency_code, "currency code")?;
    let tax_mode = normalize_text(&input.tax_mode, "tax mode")?;
    let timezone = normalize_text(&input.timezone, "timezone")?;
    let locale = normalize_text(&input.locale, "locale")?;
    let (restaurant_enabled, retail_enabled, inventory_enabled, services_enabled) =
        default_module_booleans(&business_type);

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
            &business_type,
            &currency_code,
            &tax_mode,
            Option::<String>::None,
            Option::<String>::None,
            Option::<String>::None,
            Option::<String>::None,
            Option::<String>::None,
            Option::<String>::None,
            Option::<String>::None,
            Option::<String>::None,
            &now,
            &now,
            Option::<String>::None
        ],
    )
    .map_err(|error| to_command_error("failed to create business profile", error))?;

    let settings = BusinessSettings {
        business_id: business_id.clone(),
        timezone,
        locale,
        date_format: "DD-MM-YYYY".into(),
        theme: "system".into(),
        tax_label: "GST".into(),
        default_tax_rate: 0.0,
        prices_include_tax: false,
        receipt_footer: Some("Thank you for supporting local business.".into()),
        receipt_show_address: true,
        receipt_show_phone: true,
        auto_backup_enabled: false,
        backup_directory: None,
        module_restaurant_enabled: restaurant_enabled,
        module_retail_enabled: retail_enabled,
        module_inventory_enabled: inventory_enabled,
        module_services_enabled: services_enabled,
        updated_at: now.clone(),
    };
    save_business_settings(conn, &settings)?;
    seed::ensure_workspace_support_for_business(conn, &business_id)
        .map_err(|error| to_command_error("failed to seed workspace support", error))?;

    if input.activate_now {
        set_meta(conn, "active_business_id", &business_id)?;
    }

    insert_log(
        conn,
        "INFO",
        "workspace",
        "Business workspace created",
        None,
    )?;
    get_business_by_id(conn, &business_id)
}

pub fn switch_active_business(
    conn: &Connection,
    business_id: &str,
) -> Result<BusinessProfile, String> {
    let business = get_business_by_id(conn, business_id)?;
    if business.archived_at.is_some() {
        return Err("cannot switch to an archived business".into());
    }
    set_meta(conn, "active_business_id", business_id)?;
    insert_log(conn, "INFO", "workspace", "Active business switched", None)?;
    Ok(business)
}

pub fn save_business_profile(
    conn: &Connection,
    profile: &BusinessProfile,
) -> Result<BusinessProfile, String> {
    let now = now_iso();
    let rows = conn
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
                normalize_text(&profile.name, "business name")?,
                normalize_optional(&profile.legal_name),
                normalize_code(&profile.code, "business code")?,
                normalize_text(&profile.business_type, "business type")?,
                normalize_code(&profile.currency_code, "currency code")?,
                normalize_text(&profile.tax_mode, "tax mode")?,
                normalize_optional(&profile.phone),
                normalize_optional(&profile.email),
                normalize_optional(&profile.address_line1),
                normalize_optional(&profile.address_line2),
                normalize_optional(&profile.city),
                normalize_optional(&profile.state),
                normalize_optional(&profile.postal_code),
                normalize_optional(&profile.country),
                &now,
            ],
        )
        .map_err(|error| to_command_error("failed to update business profile", error))?;

    if rows == 0 {
        return Err("business profile was not found".into());
    }

    insert_log(conn, "INFO", "business", "Business profile updated", None)?;
    get_business_by_id(conn, &profile.id)
}

pub fn get_business_settings(
    conn: &Connection,
    business_id: &str,
) -> Result<BusinessSettings, String> {
    conn.query_row(
        "SELECT
            business_id, timezone, locale, date_format, theme, tax_label,
            default_tax_rate, prices_include_tax, receipt_footer,
            receipt_show_address, receipt_show_phone, auto_backup_enabled,
            backup_directory, module_restaurant_enabled, module_retail_enabled,
            module_inventory_enabled, module_services_enabled, updated_at
         FROM business_settings
         WHERE business_id = ?1
         LIMIT 1",
        params![business_id],
        settings_from_row,
    )
    .map_err(|error| to_command_error("failed to load business settings", error))
}

pub fn list_all_business_settings(conn: &Connection) -> Result<Vec<BusinessSettings>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                business_id, timezone, locale, date_format, theme, tax_label,
                default_tax_rate, prices_include_tax, receipt_footer,
                receipt_show_address, receipt_show_phone, auto_backup_enabled,
                backup_directory, module_restaurant_enabled, module_retail_enabled,
                module_inventory_enabled, module_services_enabled, updated_at
             FROM business_settings
             ORDER BY updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare business settings query", error))?;

    let rows = stmt
        .query_map([], settings_from_row)
        .map_err(|error| to_command_error("failed to query business settings", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map business settings", error))
}

pub fn save_business_settings(
    conn: &Connection,
    settings: &BusinessSettings,
) -> Result<(), String> {
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
         )
         ON CONFLICT(business_id) DO UPDATE SET
            timezone = excluded.timezone,
            locale = excluded.locale,
            date_format = excluded.date_format,
            theme = excluded.theme,
            tax_label = excluded.tax_label,
            default_tax_rate = excluded.default_tax_rate,
            prices_include_tax = excluded.prices_include_tax,
            receipt_footer = excluded.receipt_footer,
            receipt_show_address = excluded.receipt_show_address,
            receipt_show_phone = excluded.receipt_show_phone,
            auto_backup_enabled = excluded.auto_backup_enabled,
            backup_directory = excluded.backup_directory,
            module_restaurant_enabled = excluded.module_restaurant_enabled,
            module_retail_enabled = excluded.module_retail_enabled,
            module_inventory_enabled = excluded.module_inventory_enabled,
            module_services_enabled = excluded.module_services_enabled,
            updated_at = excluded.updated_at",
        params![
            &settings.business_id,
            normalize_text(&settings.timezone, "timezone")?,
            normalize_text(&settings.locale, "locale")?,
            normalize_text(&settings.date_format, "date format")?,
            normalize_text(&settings.theme, "theme")?,
            normalize_text(&settings.tax_label, "tax label")?,
            settings.default_tax_rate.max(0.0),
            bool_to_i64(settings.prices_include_tax),
            normalize_optional(&settings.receipt_footer),
            bool_to_i64(settings.receipt_show_address),
            bool_to_i64(settings.receipt_show_phone),
            bool_to_i64(settings.auto_backup_enabled),
            normalize_optional(&settings.backup_directory),
            bool_to_i64(settings.module_restaurant_enabled),
            bool_to_i64(settings.module_retail_enabled),
            bool_to_i64(settings.module_inventory_enabled),
            bool_to_i64(settings.module_services_enabled),
            now_iso(),
        ],
    )
    .map_err(|error| to_command_error("failed to save business settings", error))?;
    Ok(())
}

pub fn list_tax_profiles(conn: &Connection, business_id: &str) -> Result<Vec<TaxProfile>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, business_id, name, label, rate, prices_include_tax, is_default, updated_at
             FROM tax_profiles
             WHERE business_id = ?1
             ORDER BY is_default DESC, updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare tax profile query", error))?;

    let rows = stmt
        .query_map(params![business_id], tax_profile_from_row)
        .map_err(|error| to_command_error("failed to query tax profiles", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map tax profiles", error))
}

pub fn get_default_tax_profile(conn: &Connection, business_id: &str) -> Result<TaxProfile, String> {
    if let Some(profile) = list_tax_profiles(conn, business_id)?.into_iter().next() {
        return Ok(profile);
    }

    seed::ensure_workspace_support_for_business(conn, business_id)
        .map_err(|error| to_command_error("failed to ensure tax profile", error))?;
    list_tax_profiles(conn, business_id)?
        .into_iter()
        .next()
        .ok_or_else(|| "default tax profile was not found".into())
}

pub fn list_all_tax_profiles(conn: &Connection) -> Result<Vec<TaxProfile>, String> {
    let businesses = list_businesses(conn)?;
    let mut profiles = Vec::new();
    for business in businesses {
        profiles.extend(list_tax_profiles(conn, &business.id)?);
    }
    Ok(profiles)
}

pub fn save_default_tax_profile(
    conn: &Connection,
    profile: &TaxProfile,
) -> Result<TaxProfile, String> {
    let name = normalize_text(&profile.name, "tax profile name")?;
    let label = normalize_text(&profile.tax_label, "tax label")?;
    let now = now_iso();

    conn.execute(
        "UPDATE tax_profiles SET is_default = 0, updated_at = ?2 WHERE business_id = ?1",
        params![&profile.business_id, &now],
    )
    .map_err(|error| to_command_error("failed to clear default tax profile", error))?;

    let existing_id = profile.id.trim();
    let target_id = if existing_id.is_empty() {
        Uuid::new_v4().to_string()
    } else {
        existing_id.to_string()
    };

    conn.execute(
        "INSERT INTO tax_profiles (
            id, business_id, name, label, rate, prices_include_tax, is_default, created_at, updated_at
         ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, 1, ?7, ?8
         )
         ON CONFLICT(id) DO UPDATE SET
            name = excluded.name,
            label = excluded.label,
            rate = excluded.rate,
            prices_include_tax = excluded.prices_include_tax,
            is_default = excluded.is_default,
            updated_at = excluded.updated_at",
        params![
            &target_id,
            &profile.business_id,
            &name,
            &label,
            profile.default_rate.max(0.0),
            bool_to_i64(profile.prices_include_tax),
            &now,
            &now,
        ],
    )
    .map_err(|error| to_command_error("failed to save tax profile", error))?;

    get_default_tax_profile(conn, &profile.business_id)
}

pub fn get_default_receipt_profile(
    conn: &Connection,
    business_id: &str,
) -> Result<ReceiptProfile, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, business_id, name, footer_text, show_address, show_phone, show_email, show_business_code, paper_width, is_default, updated_at
             FROM receipt_profiles
             WHERE business_id = ?1
             ORDER BY is_default DESC, updated_at DESC
             LIMIT 1",
        )
        .map_err(|error| to_command_error("failed to prepare receipt profile query", error))?;

    let profile = stmt
        .query_row(params![business_id], receipt_profile_from_row)
        .optional()
        .map_err(|error| to_command_error("failed to query receipt profile", error))?;

    if let Some(profile) = profile {
        return Ok(profile);
    }

    seed::ensure_workspace_support_for_business(conn, business_id)
        .map_err(|error| to_command_error("failed to ensure receipt profile", error))?;

    conn.query_row(
        "SELECT id, business_id, name, footer_text, show_address, show_phone, show_email, show_business_code, paper_width, is_default, updated_at
         FROM receipt_profiles
         WHERE business_id = ?1
         ORDER BY is_default DESC, updated_at DESC
         LIMIT 1",
        params![business_id],
        receipt_profile_from_row,
    )
    .map_err(|error| to_command_error("failed to load default receipt profile", error))
}

pub fn list_all_receipt_profiles(conn: &Connection) -> Result<Vec<ReceiptProfile>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, business_id, name, footer_text, show_address, show_phone, show_email, show_business_code, paper_width, is_default, updated_at
             FROM receipt_profiles
             ORDER BY updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare receipt profile list", error))?;

    let rows = stmt
        .query_map([], receipt_profile_from_row)
        .map_err(|error| to_command_error("failed to query receipt profiles", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map receipt profiles", error))
}

pub fn save_default_receipt_profile(
    conn: &Connection,
    profile: &ReceiptProfile,
) -> Result<ReceiptProfile, String> {
    let now = now_iso();
    let name = normalize_text(&profile.name, "receipt profile name")?;
    let paper_width = normalize_text(&profile.paper_width, "paper width")?;

    conn.execute(
        "UPDATE receipt_profiles SET is_default = 0, updated_at = ?2 WHERE business_id = ?1",
        params![&profile.business_id, &now],
    )
    .map_err(|error| to_command_error("failed to clear receipt defaults", error))?;

    let existing_id = profile.id.trim();
    let target_id = if existing_id.is_empty() {
        Uuid::new_v4().to_string()
    } else {
        existing_id.to_string()
    };

    conn.execute(
        "INSERT INTO receipt_profiles (
            id, business_id, name, header_line1, header_line2, footer_text,
            show_address, show_phone, show_tax_breakdown, paper_width,
            copies, is_default, created_at, updated_at, show_email, show_business_code
         ) VALUES (
            ?1, ?2, ?3, NULL, NULL, ?4,
            ?5, ?6, 1, ?7,
            1, 1, ?8, ?9, ?10, ?11
         )
         ON CONFLICT(id) DO UPDATE SET
            name = excluded.name,
            footer_text = excluded.footer_text,
            show_address = excluded.show_address,
            show_phone = excluded.show_phone,
            paper_width = excluded.paper_width,
            is_default = excluded.is_default,
            updated_at = excluded.updated_at,
            show_email = excluded.show_email,
            show_business_code = excluded.show_business_code",
        params![
            &target_id,
            &profile.business_id,
            &name,
            normalize_optional(&profile.footer_text),
            bool_to_i64(profile.show_address),
            bool_to_i64(profile.show_phone),
            &paper_width,
            &now,
            &now,
            bool_to_i64(profile.show_email),
            bool_to_i64(profile.show_business_code),
        ],
    )
    .map_err(|error| to_command_error("failed to save receipt profile", error))?;

    get_default_receipt_profile(conn, &profile.business_id)
}

fn set_flag(flags: &mut ModuleFlags, key: &str, enabled: bool) {
    match key {
        "restaurant" => flags.restaurant_enabled = enabled,
        "retail" => flags.retail_enabled = enabled,
        "inventory" => flags.inventory_enabled = enabled,
        "services" => flags.services_enabled = enabled,
        "customers" => flags.customers_enabled = enabled,
        "suppliers" => flags.suppliers_enabled = enabled,
        "expenses" => flags.expenses_enabled = enabled,
        "reporting" => flags.reporting_enabled = enabled,
        "data_center" => flags.data_center_enabled = enabled,
        _ => {}
    }
}

fn active_module_keys(flags: &ModuleFlags) -> Vec<String> {
    let mut modules = Vec::new();
    if flags.restaurant_enabled {
        modules.push("restaurant".to_string());
    }
    if flags.retail_enabled {
        modules.push("retail".to_string());
    }
    if flags.inventory_enabled {
        modules.push("inventory".to_string());
    }
    if flags.services_enabled {
        modules.push("services".to_string());
    }
    if flags.customers_enabled {
        modules.push("customers".to_string());
    }
    if flags.suppliers_enabled {
        modules.push("suppliers".to_string());
    }
    if flags.expenses_enabled {
        modules.push("expenses".to_string());
    }
    if flags.reporting_enabled {
        modules.push("reporting".to_string());
    }
    if flags.data_center_enabled {
        modules.push("data_center".to_string());
    }
    modules
}

pub fn get_module_flags(conn: &Connection, business_id: &str) -> Result<ModuleFlags, String> {
    let settings = get_business_settings(conn, business_id)?;
    let mut flags = ModuleFlags {
        business_id: business_id.to_string(),
        restaurant_enabled: settings.module_restaurant_enabled,
        retail_enabled: settings.module_retail_enabled,
        inventory_enabled: settings.module_inventory_enabled,
        services_enabled: settings.module_services_enabled,
        customers_enabled: false,
        suppliers_enabled: false,
        expenses_enabled: false,
        reporting_enabled: false,
        data_center_enabled: true,
        updated_at: settings.updated_at.clone(),
    };

    let mut stmt = conn
        .prepare(
            "SELECT module_key, enabled, updated_at
             FROM module_flags
             WHERE business_id = ?1
             ORDER BY module_key ASC",
        )
        .map_err(|error| to_command_error("failed to prepare module flag query", error))?;

    let mut rows = stmt
        .query(params![business_id])
        .map_err(|error| to_command_error("failed to query module flags", error))?;

    while let Some(row) = rows
        .next()
        .map_err(|error| to_command_error("failed to iterate module flags", error))?
    {
        let module_key: String = row
            .get(0)
            .map_err(|error| to_command_error("failed to read module key", error))?;
        let enabled = row
            .get::<_, i64>(1)
            .map_err(|error| to_command_error("failed to read module enabled value", error))?
            != 0;
        let updated_at: String = row
            .get(2)
            .map_err(|error| to_command_error("failed to read module updated timestamp", error))?;
        set_flag(&mut flags, &module_key, enabled);
        flags.updated_at = updated_at;
    }

    Ok(flags)
}

pub fn list_all_module_flags(conn: &Connection) -> Result<Vec<ModuleFlags>, String> {
    let businesses = list_businesses(conn)?;
    let mut output = Vec::new();
    for business in businesses {
        output.push(get_module_flags(conn, &business.id)?);
    }
    Ok(output)
}

pub fn save_module_flags(conn: &Connection, flags: &ModuleFlags) -> Result<(), String> {
    let now = now_iso();
    for (module_key, enabled) in [
        ("restaurant", flags.restaurant_enabled),
        ("retail", flags.retail_enabled),
        ("inventory", flags.inventory_enabled),
        ("services", flags.services_enabled),
        ("customers", flags.customers_enabled),
        ("suppliers", flags.suppliers_enabled),
        ("expenses", flags.expenses_enabled),
        ("reporting", flags.reporting_enabled),
        ("data_center", flags.data_center_enabled),
    ] {
        conn.execute(
            "INSERT INTO module_flags (business_id, module_key, enabled, updated_at)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(business_id, module_key)
             DO UPDATE SET enabled = excluded.enabled, updated_at = excluded.updated_at",
            params![&flags.business_id, module_key, bool_to_i64(enabled), &now],
        )
        .map_err(|error| to_command_error("failed to save module flags", error))?;
    }
    Ok(())
}

pub fn list_sequence_counters(
    conn: &Connection,
    business_id: &str,
) -> Result<Vec<SequenceCounter>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT business_id, counter_key, prefix, next_number, padding, reset_policy, updated_at
             FROM sequence_counters
             WHERE business_id = ?1
             ORDER BY counter_key ASC",
        )
        .map_err(|error| to_command_error("failed to prepare sequence query", error))?;

    let rows = stmt
        .query_map(params![business_id], sequence_from_row)
        .map_err(|error| to_command_error("failed to query sequence counters", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map sequence counters", error))
}

pub fn list_all_sequence_counters(conn: &Connection) -> Result<Vec<SequenceCounter>, String> {
    let businesses = list_businesses(conn)?;
    let mut output = Vec::new();
    for business in businesses {
        output.extend(list_sequence_counters(conn, &business.id)?);
    }
    Ok(output)
}

pub fn save_sequence_counters(
    conn: &Connection,
    counters: &[SequenceCounter],
) -> Result<(), String> {
    let now = now_iso();
    for counter in counters {
        conn.execute(
            "INSERT INTO sequence_counters (business_id, counter_key, prefix, next_number, padding, reset_policy, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(business_id, counter_key)
             DO UPDATE SET
                prefix = excluded.prefix,
                next_number = excluded.next_number,
                padding = excluded.padding,
                reset_policy = excluded.reset_policy,
                updated_at = excluded.updated_at",
            params![
                &counter.business_id,
                normalize_text(&counter.scope, "sequence scope")?,
                normalize_text(&counter.prefix, "sequence prefix")?.to_uppercase(),
                counter.next_number.max(1),
                counter.padding.max(1),
                normalize_text(&counter.reset_policy, "reset policy")?,
                &now,
            ],
        )
        .map_err(|error| to_command_error("failed to save sequence counters", error))?;
    }
    Ok(())
}

pub fn save_workspace_configuration(
    conn: &Connection,
    input: &WorkspaceConfigurationInput,
) -> Result<(), String> {
    let business_id = input.business_settings.business_id.clone();
    let mut settings = input.business_settings.clone();
    settings.tax_label = input.tax_profile.tax_label.clone();
    settings.default_tax_rate = input.tax_profile.default_rate;
    settings.prices_include_tax = input.tax_profile.prices_include_tax;
    settings.receipt_footer = input.receipt_profile.footer_text.clone();
    settings.receipt_show_address = input.receipt_profile.show_address;
    settings.receipt_show_phone = input.receipt_profile.show_phone;
    settings.module_restaurant_enabled = input.module_flags.restaurant_enabled;
    settings.module_retail_enabled = input.module_flags.retail_enabled;
    settings.module_inventory_enabled = input.module_flags.inventory_enabled;
    settings.module_services_enabled = input.module_flags.services_enabled;
    settings.updated_at = now_iso();

    save_business_settings(conn, &settings)?;

    let tax_profile = TaxProfile {
        business_id: business_id.clone(),
        ..input.tax_profile.clone()
    };
    save_default_tax_profile(conn, &tax_profile)?;

    let receipt_profile = ReceiptProfile {
        business_id: business_id.clone(),
        ..input.receipt_profile.clone()
    };
    save_default_receipt_profile(conn, &receipt_profile)?;

    let module_flags = ModuleFlags {
        business_id: business_id.clone(),
        updated_at: now_iso(),
        ..input.module_flags.clone()
    };
    save_module_flags(conn, &module_flags)?;

    let counters = input
        .sequence_counters
        .iter()
        .cloned()
        .map(|mut counter| {
            counter.business_id = business_id.clone();
            counter
        })
        .collect::<Vec<_>>();
    save_sequence_counters(conn, &counters)?;

    insert_log(
        conn,
        "INFO",
        "settings",
        "Workspace configuration saved",
        None,
    )?;
    Ok(())
}

fn format_sequence_preview(prefix: &str, next_number: i64, padding: i64) -> String {
    let normalized_padding = padding.max(1) as usize;
    let normalized_number = next_number.max(1);
    format!(
        "{prefix}{:0width$}",
        normalized_number,
        width = normalized_padding
    )
}

pub fn list_business_workspace_summaries(
    conn: &Connection,
) -> Result<Vec<BusinessWorkspaceSummary>, String> {
    let businesses = list_businesses(conn)?;
    let mut output = Vec::new();

    for business in businesses {
        let settings = get_business_settings(conn, &business.id)?;
        let tax_profile = get_default_tax_profile(conn, &business.id)?;
        let module_flags = get_module_flags(conn, &business.id)?;
        let sequences = list_sequence_counters(conn, &business.id)?;
        let sale_sequence = sequences
            .iter()
            .find(|counter| counter.scope == "sale")
            .cloned();
        let next_sale_sequence = sale_sequence
            .map(|sequence| {
                format_sequence_preview(&sequence.prefix, sequence.next_number, sequence.padding)
            })
            .unwrap_or_else(|| "SAL-00001".into());

        output.push(BusinessWorkspaceSummary {
            business_id: business.id.clone(),
            name: business.name.clone(),
            code: business.code.clone(),
            business_type: business.business_type.clone(),
            currency_code: business.currency_code.clone(),
            theme: settings.theme.clone(),
            timezone: settings.timezone.clone(),
            tax_label: tax_profile.tax_label.clone(),
            default_tax_rate: tax_profile.default_rate,
            next_sale_sequence,
            active_modules: active_module_keys(&module_flags),
            archived_at: business.archived_at.clone(),
            updated_at: business.updated_at.clone(),
        });
    }

    Ok(output)
}

pub fn build_storage_status(conn: &Connection, paths: &AppPaths) -> Result<StorageStatus, String> {
    let backup_count: usize = conn
        .query_row("SELECT COUNT(*) FROM backup_records", [], |row| {
            row.get::<_, i64>(0)
        })
        .map_err(|error| to_command_error("failed to count backup records", error))?
        as usize;
    let export_count: usize =
        conn.query_row("SELECT COUNT(*) FROM export_jobs", [], |row| {
            row.get::<_, i64>(0)
        })
        .map_err(|error| to_command_error("failed to count export jobs", error))? as usize;

    Ok(StorageStatus {
        data_dir: paths.data_dir.clone(),
        config_dir: paths.config_dir.clone(),
        log_dir: paths.log_dir.clone(),
        backup_dir: paths.backup_dir.clone(),
        export_dir: paths.export_dir.clone(),
        database_path: paths.database_path.clone(),
        database_exists: fs::metadata(&paths.database_path).is_ok(),
        backup_count,
        export_count,
    })
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
            &record.created_at,
        ],
    )
    .map_err(|error| to_command_error("failed to insert backup record", error))?;
    Ok(())
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
            &record.completed_at,
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
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, NULL)",
        params![
            Uuid::new_v4().to_string(),
            business_id,
            format,
            status,
            source_path,
            now_iso(),
        ],
    )
    .map_err(|error| to_command_error("failed to insert import job", error))?;
    Ok(())
}
