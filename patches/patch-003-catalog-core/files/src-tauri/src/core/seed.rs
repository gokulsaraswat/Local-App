use chrono::Utc;
use rusqlite::{params, Connection};
use uuid::Uuid;

use super::catalog;

fn upsert_meta(conn: &Connection, key: &str, value: &str) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO app_meta (key, value, updated_at)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
        params![key, value, Utc::now().to_rfc3339()],
    )?;
    Ok(())
}

fn insert_meta_if_missing(conn: &Connection, key: &str, value: &str) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO app_meta (key, value, updated_at)
         VALUES (?1, ?2, ?3)",
        params![key, value, Utc::now().to_rfc3339()],
    )?;
    Ok(())
}

fn bool_to_i64(value: bool) -> i64 {
    if value {
        1
    } else {
        0
    }
}

pub fn ensure_workspace_support_for_business(
    conn: &Connection,
    business_id: &str,
) -> rusqlite::Result<()> {
    let (
        tax_label,
        default_tax_rate,
        prices_include_tax,
        receipt_footer,
        receipt_show_address,
        receipt_show_phone,
        module_restaurant_enabled,
        module_retail_enabled,
        module_inventory_enabled,
        module_services_enabled,
    ) = conn.query_row(
        "SELECT
            tax_label,
            default_tax_rate,
            prices_include_tax,
            receipt_footer,
            receipt_show_address,
            receipt_show_phone,
            module_restaurant_enabled,
            module_retail_enabled,
            module_inventory_enabled,
            module_services_enabled
         FROM business_settings
         WHERE business_id = ?1
         LIMIT 1",
        params![business_id],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, f64>(1)?,
                row.get::<_, i64>(2)? != 0,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, i64>(4)? != 0,
                row.get::<_, i64>(5)? != 0,
                row.get::<_, i64>(6)? != 0,
                row.get::<_, i64>(7)? != 0,
                row.get::<_, i64>(8)? != 0,
                row.get::<_, i64>(9)? != 0,
            ))
        },
    )?;

    let now = Utc::now().to_rfc3339();

    let tax_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM tax_profiles WHERE business_id = ?1",
        params![business_id],
        |row| row.get(0),
    )?;
    if tax_count == 0 {
        conn.execute(
            "INSERT INTO tax_profiles (
                id, business_id, name, label, rate, prices_include_tax,
                is_default, created_at, updated_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 1, ?7, ?8)",
            params![
                Uuid::new_v4().to_string(),
                business_id,
                "Standard tax",
                tax_label,
                default_tax_rate,
                bool_to_i64(prices_include_tax),
                &now,
                &now,
            ],
        )?;
    }

    let receipt_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM receipt_profiles WHERE business_id = ?1",
        params![business_id],
        |row| row.get(0),
    )?;
    if receipt_count == 0 {
        conn.execute(
            "INSERT INTO receipt_profiles (
                id, business_id, name, header_line1, header_line2, footer_text,
                show_address, show_phone, show_tax_breakdown, paper_width, copies,
                is_default, created_at, updated_at, show_email, show_business_code
             ) VALUES (
                ?1, ?2, ?3, NULL, NULL, ?4,
                ?5, ?6, 1, '80mm', 1,
                1, ?7, ?8, 0, 1
             )",
            params![
                Uuid::new_v4().to_string(),
                business_id,
                "Default receipt",
                receipt_footer,
                bool_to_i64(receipt_show_address),
                bool_to_i64(receipt_show_phone),
                &now,
                &now,
            ],
        )?;
    }

    for (module_key, enabled) in [
        ("restaurant", module_restaurant_enabled),
        ("retail", module_retail_enabled),
        ("inventory", module_inventory_enabled),
        ("services", module_services_enabled),
        ("customers", false),
        ("suppliers", false),
        ("expenses", false),
        ("reporting", false),
        ("data_center", true),
    ] {
        conn.execute(
            "INSERT OR IGNORE INTO module_flags (business_id, module_key, enabled, updated_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![business_id, module_key, bool_to_i64(enabled), &now],
        )?;
    }

    for (counter_key, prefix, padding, reset_policy) in [
        ("sale", "SAL-", 5_i64, "none"),
        ("purchase", "PUR-", 5_i64, "none"),
        ("expense", "EXP-", 5_i64, "none"),
        ("customer", "CUS-", 4_i64, "none"),
        ("supplier", "SUP-", 4_i64, "none"),
    ] {
        conn.execute(
            "INSERT OR IGNORE INTO sequence_counters (
                business_id, counter_key, prefix, next_number, padding, reset_policy, updated_at
             ) VALUES (?1, ?2, ?3, 1, ?4, ?5, ?6)",
            params![business_id, counter_key, prefix, padding, reset_policy, &now],
        )?;
    }

    Ok(())
}

fn ensure_workspace_support_foundation(conn: &Connection) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare("SELECT id FROM businesses ORDER BY created_at ASC")?;
    let business_ids = stmt.query_map([], |row| row.get::<_, String>(0))?;

    for business_id in business_ids {
        ensure_workspace_support_for_business(conn, &business_id?)?;
    }

    Ok(())
}

fn seed_demo_catalog_foundation(conn: &Connection) -> rusqlite::Result<()> {
    let seeded_demo = conn.query_row(
        "SELECT value FROM app_meta WHERE key = 'seeded_demo_data' LIMIT 1",
        [],
        |row| row.get::<_, String>(0),
    ).unwrap_or_else(|_| "false".to_string());

    if seeded_demo != "true" {
        return Ok(());
    }

    let mut stmt = conn.prepare(
        "SELECT id
         FROM businesses
         WHERE code = 'DEMO-001' OR name LIKE 'Demo %'
         ORDER BY created_at ASC",
    )?;
    let business_ids = stmt.query_map([], |row| row.get::<_, String>(0))?;
    for business_id in business_ids {
        catalog::seed_demo_catalog_for_business(conn, &business_id?)?;
    }

    Ok(())
}

pub fn seed_if_empty(conn: &Connection) -> rusqlite::Result<()> {
    let business_count: i64 = conn.query_row("SELECT COUNT(*) FROM businesses", [], |row| row.get(0))?;

    upsert_meta(conn, "app_name", "local-first-business-manager")?;
    upsert_meta(conn, "product_name", "Local Business Manager")?;
    upsert_meta(conn, "app_version", "0.3.0")?;
    insert_meta_if_missing(conn, "initialized_at", &Utc::now().to_rfc3339())?;

    if business_count == 0 {
        let business_id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

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
                "Demo Cafe & Retail",
                "Demo Cafe & Retail LLP",
                "DEMO-001",
                "Cafe",
                "INR",
                "exclusive",
                "+91-90000-00000",
                "demo@localbusiness.test",
                "12 Market Street",
                "Near Central Square",
                "Bengaluru",
                "Karnataka",
                "560001",
                "India",
                &now,
                &now,
                Option::<String>::None,
            ],
        )?;

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
                &business_id,
                "Asia/Kolkata",
                "en-IN",
                "DD-MM-YYYY",
                "system",
                "GST",
                5.0_f64,
                0_i64,
                "Thank you for supporting local business.",
                1_i64,
                1_i64,
                0_i64,
                Option::<String>::None,
                1_i64,
                1_i64,
                1_i64,
                0_i64,
                &now,
            ],
        )?;

        upsert_meta(conn, "active_business_id", &business_id)?;
        upsert_meta(conn, "seeded_demo_data", "true")?;

        for (level, category, message) in [
            ("INFO", "patching", "Patch 1 foundation registered"),
            ("INFO", "patching", "Patch 2 multi-business workspace registered"),
            ("INFO", "patching", "Patch 3 catalog core registered"),
            ("INFO", "business", "Demo business profile created"),
            ("INFO", "storage", "Local storage directories prepared"),
        ] {
            conn.execute(
                "INSERT INTO app_logs (id, level, category, message, payload_json, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    Uuid::new_v4().to_string(),
                    level,
                    category,
                    message,
                    Option::<String>::None,
                    Utc::now().to_rfc3339(),
                ],
            )?;
        }
    }

    ensure_workspace_support_foundation(conn)?;
    catalog::ensure_system_units(conn)?;
    seed_demo_catalog_foundation(conn)?;
    Ok(())
}
