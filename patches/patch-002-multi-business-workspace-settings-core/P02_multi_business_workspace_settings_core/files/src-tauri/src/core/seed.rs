use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use uuid::Uuid;

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
    if value { 1 } else { 0 }
}

fn load_legacy_settings(
    conn: &Connection,
    business_id: &str,
) -> rusqlite::Result<(
    String,
    f64,
    Option<String>,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
)> {
    conn.query_row(
        "SELECT
            tax_label,
            default_tax_rate,
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
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get::<_, i64>(3)? != 0,
                row.get::<_, i64>(4)? != 0,
                row.get::<_, i64>(5)? != 0,
                row.get::<_, i64>(6)? != 0,
                row.get::<_, i64>(7)? != 0,
                row.get::<_, i64>(8)? != 0,
            ))
        },
    )
}

pub fn ensure_workspace_support_for_business(
    conn: &Connection,
    business_id: &str,
) -> rusqlite::Result<()> {
    let (
        tax_label,
        default_tax_rate,
        receipt_footer,
        receipt_show_address,
        receipt_show_phone,
        module_restaurant_enabled,
        module_retail_enabled,
        module_inventory_enabled,
        module_services_enabled,
    ) = load_legacy_settings(conn, business_id)?;

    let now = Utc::now().to_rfc3339();

    let tax_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM tax_profiles WHERE business_id = ?1",
        params![business_id],
        |row| row.get(0),
    )?;
    if tax_count == 0 {
        conn.execute(
            "INSERT INTO tax_profiles (id, business_id, name, label, rate, is_default, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                Uuid::new_v4().to_string(),
                business_id,
                "Standard tax",
                tax_label,
                default_tax_rate,
                1_i64,
                now.clone(),
                now.clone()
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
                show_address, show_phone, show_tax_breakdown, paper_width,
                copies, is_default, created_at, updated_at
             ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6,
                ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14
             )",
            params![
                Uuid::new_v4().to_string(),
                business_id,
                "Default receipt",
                Option::<String>::None,
                Option::<String>::None,
                receipt_footer,
                bool_to_i64(receipt_show_address),
                bool_to_i64(receipt_show_phone),
                1_i64,
                "80mm",
                1_i64,
                1_i64,
                now.clone(),
                now.clone()
            ],
        )?;
    }

    for (module_key, enabled) in [
        ("restaurant", module_restaurant_enabled),
        ("retail", module_retail_enabled),
        ("inventory", module_inventory_enabled),
        ("services", module_services_enabled),
    ] {
        conn.execute(
            "INSERT OR IGNORE INTO module_flags (business_id, module_key, enabled, updated_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![business_id, module_key, bool_to_i64(enabled), now.clone()],
        )?;
    }

    for (counter_key, prefix, padding) in [
        ("sale", "SAL-", 5_i64),
        ("purchase", "PUR-", 5_i64),
        ("expense", "EXP-", 5_i64),
        ("customer", "CUS-", 4_i64),
        ("supplier", "SUP-", 4_i64),
    ] {
        conn.execute(
            "INSERT OR IGNORE INTO sequence_counters (business_id, counter_key, prefix, next_number, padding, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![business_id, counter_key, prefix, 1_i64, padding, now.clone()],
        )?;
    }

    Ok(())
}

pub fn copy_workspace_preferences_from_template(
    conn: &Connection,
    template_business_id: &str,
    target_business_id: &str,
) -> rusqlite::Result<()> {
    let now = Utc::now().to_rfc3339();

    let template_tax = conn
        .query_row(
            "SELECT name, label, rate
             FROM tax_profiles
             WHERE business_id = ?1
             ORDER BY is_default DESC, updated_at DESC
             LIMIT 1",
            params![template_business_id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, f64>(2)?,
                ))
            },
        )
        .optional()?;

    if let Some((name, label, rate)) = template_tax {
        conn.execute(
            "UPDATE tax_profiles
             SET name = ?2, label = ?3, rate = ?4, is_default = 1, updated_at = ?5
             WHERE business_id = ?1 AND is_default = 1",
            params![target_business_id, name, label, rate, now.clone()],
        )?;

        conn.execute(
            "UPDATE business_settings
             SET tax_label = ?2, default_tax_rate = ?3, updated_at = ?4
             WHERE business_id = ?1",
            params![target_business_id, label, rate, now.clone()],
        )?;
    }

    let template_receipt = conn
        .query_row(
            "SELECT name, header_line1, header_line2, footer_text, show_address, show_phone, show_tax_breakdown, paper_width, copies
             FROM receipt_profiles
             WHERE business_id = ?1
             ORDER BY is_default DESC, updated_at DESC
             LIMIT 1",
            params![template_business_id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, Option<String>>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, Option<String>>(3)?,
                    row.get::<_, i64>(4)? != 0,
                    row.get::<_, i64>(5)? != 0,
                    row.get::<_, i64>(6)? != 0,
                    row.get::<_, String>(7)?,
                    row.get::<_, i64>(8)?,
                ))
            },
        )
        .optional()?;

    if let Some((name, header_line1, header_line2, footer_text, show_address, show_phone, show_tax_breakdown, paper_width, copies)) = template_receipt {
        conn.execute(
            "UPDATE receipt_profiles
             SET name = ?2,
                 header_line1 = ?3,
                 header_line2 = ?4,
                 footer_text = ?5,
                 show_address = ?6,
                 show_phone = ?7,
                 show_tax_breakdown = ?8,
                 paper_width = ?9,
                 copies = ?10,
                 is_default = 1,
                 updated_at = ?11
             WHERE business_id = ?1 AND is_default = 1",
            params![
                target_business_id,
                name,
                header_line1,
                header_line2,
                footer_text.clone(),
                bool_to_i64(show_address),
                bool_to_i64(show_phone),
                bool_to_i64(show_tax_breakdown),
                paper_width,
                copies,
                now.clone()
            ],
        )?;

        conn.execute(
            "UPDATE business_settings
             SET receipt_footer = ?2,
                 receipt_show_address = ?3,
                 receipt_show_phone = ?4,
                 updated_at = ?5
             WHERE business_id = ?1",
            params![
                target_business_id,
                footer_text,
                bool_to_i64(show_address),
                bool_to_i64(show_phone),
                now.clone()
            ],
        )?;
    }

    let template_modules = conn.prepare(
        "SELECT module_key, enabled FROM module_flags WHERE business_id = ?1 ORDER BY module_key ASC",
    )?;
    let mut rows = template_modules.query(params![template_business_id])?;
    let mut restaurant = false;
    let mut retail = false;
    let mut inventory = false;
    let mut services = false;

    while let Some(row) = rows.next()? {
        let module_key: String = row.get(0)?;
        let enabled = row.get::<_, i64>(1)? != 0;
        conn.execute(
            "INSERT INTO module_flags (business_id, module_key, enabled, updated_at)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(business_id, module_key)
             DO UPDATE SET enabled = excluded.enabled, updated_at = excluded.updated_at",
            params![target_business_id, module_key, bool_to_i64(enabled), now.clone()],
        )?;

        match module_key.as_str() {
            "restaurant" => restaurant = enabled,
            "retail" => retail = enabled,
            "inventory" => inventory = enabled,
            "services" => services = enabled,
            _ => {}
        }
    }

    conn.execute(
        "UPDATE business_settings
         SET module_restaurant_enabled = ?2,
             module_retail_enabled = ?3,
             module_inventory_enabled = ?4,
             module_services_enabled = ?5,
             updated_at = ?6
         WHERE business_id = ?1",
        params![
            target_business_id,
            bool_to_i64(restaurant),
            bool_to_i64(retail),
            bool_to_i64(inventory),
            bool_to_i64(services),
            now.clone()
        ],
    )?;

    let template_counters = conn.prepare(
        "SELECT counter_key, prefix, padding FROM sequence_counters WHERE business_id = ?1 ORDER BY counter_key ASC",
    )?;
    let mut counter_rows = template_counters.query(params![template_business_id])?;
    while let Some(row) = counter_rows.next()? {
        let counter_key: String = row.get(0)?;
        let prefix: String = row.get(1)?;
        let padding: i64 = row.get(2)?;
        conn.execute(
            "INSERT INTO sequence_counters (business_id, counter_key, prefix, next_number, padding, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(business_id, counter_key)
             DO UPDATE SET prefix = excluded.prefix, next_number = excluded.next_number, padding = excluded.padding, updated_at = excluded.updated_at",
            params![target_business_id, counter_key, prefix, 1_i64, padding, now.clone()],
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

pub fn seed_if_empty(conn: &Connection) -> rusqlite::Result<()> {
    let business_count: i64 = conn.query_row("SELECT COUNT(*) FROM businesses", [], |row| row.get(0))?;

    upsert_meta(conn, "app_name", "local-first-business-manager")?;
    upsert_meta(conn, "product_name", "Local Business Manager")?;
    upsert_meta(conn, "app_version", "0.2.0")?;
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
                Option::<String>::None
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
                &now
            ],
        )?;

        upsert_meta(conn, "active_business_id", &business_id)?;
        upsert_meta(conn, "seeded_demo_data", "true")?;

        for (level, category, message) in [
            ("INFO", "patching", "Patch 1 foundation registered"),
            ("INFO", "patching", "Patch 2 multi-business workspace registered"),
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
                    Utc::now().to_rfc3339()
                ],
            )?;
        }
    }

    ensure_workspace_support_foundation(conn)?;
    Ok(())
}
