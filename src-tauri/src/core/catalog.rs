use std::collections::HashSet;

use rusqlite::{params, Connection, OptionalExtension, Row};
use uuid::Uuid;

use crate::domain::models::{
    CatalogBarcode, CatalogCategory, CatalogItem, CatalogItemView, CatalogSummary, CatalogUnit,
    CatalogWorkspace, SaveCatalogCategoryInput, SaveCatalogItemInput, SaveCatalogUnitInput,
};

use super::{db, error::to_command_error};

const SYSTEM_UNITS: [(&str, &str, &str, bool); 7] = [
    ("system-pcs", "Pieces", "PCS", false),
    ("system-kg", "Kilogram", "KG", true),
    ("system-g", "Gram", "G", true),
    ("system-litre", "Litre", "LTR", true),
    ("system-ml", "Millilitre", "ML", true),
    ("system-hour", "Hour", "HR", true),
    ("system-plate", "Plate", "PLATE", false),
];

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
    Ok(normalize_text(value, field)?.to_uppercase())
}

fn normalize_item_kind(value: &str) -> String {
    match value.trim().to_lowercase().as_str() {
        "menu" => "menu".into(),
        "service" => "service".into(),
        _ => "stock".into(),
    }
}

fn normalize_scope(value: &str) -> String {
    match value.trim().to_lowercase().as_str() {
        "menu" => "menu".into(),
        "stock" => "stock".into(),
        "service" => "service".into(),
        _ => "all".into(),
    }
}

fn category_from_row(row: &Row) -> rusqlite::Result<CatalogCategory> {
    Ok(CatalogCategory {
        id: row.get(0)?,
        business_id: row.get(1)?,
        name: row.get(2)?,
        code: row.get(3)?,
        parent_id: row.get(4)?,
        item_scope: row.get(5)?,
        sort_order: row.get(6)?,
        notes: row.get(7)?,
        created_at: row.get(8)?,
        updated_at: row.get(9)?,
        archived_at: row.get(10)?,
    })
}

fn unit_from_row(row: &Row) -> rusqlite::Result<CatalogUnit> {
    Ok(CatalogUnit {
        id: row.get(0)?,
        business_id: row.get(1)?,
        name: row.get(2)?,
        code: row.get(3)?,
        symbol: row.get(4)?,
        allow_fractional: bool_from_row(row, 5)?,
        is_system: bool_from_row(row, 6)?,
        updated_at: row.get(7)?,
        archived_at: row.get(8)?,
    })
}

fn item_from_row(row: &Row) -> rusqlite::Result<CatalogItem> {
    Ok(CatalogItem {
        id: row.get(0)?,
        business_id: row.get(1)?,
        category_id: row.get(2)?,
        unit_id: row.get(3)?,
        tax_profile_id: row.get(4)?,
        item_kind: row.get(5)?,
        name: row.get(6)?,
        display_name: row.get(7)?,
        sku: row.get(8)?,
        primary_barcode: row.get(9)?,
        description: row.get(10)?,
        selling_price: row.get(11)?,
        cost_price: row.get(12)?,
        track_stock: bool_from_row(row, 13)?,
        stock_quantity: row.get(14)?,
        reorder_level: row.get(15)?,
        image_path: row.get(16)?,
        is_active: bool_from_row(row, 17)?,
        created_at: row.get(18)?,
        updated_at: row.get(19)?,
        archived_at: row.get(20)?,
    })
}

fn barcode_from_row(row: &Row) -> rusqlite::Result<CatalogBarcode> {
    Ok(CatalogBarcode {
        id: row.get(0)?,
        item_id: row.get(1)?,
        barcode: row.get(2)?,
        label: row.get(3)?,
        is_primary: bool_from_row(row, 4)?,
        created_at: row.get(5)?,
    })
}

fn load_category_name(conn: &Connection, category_id: &Option<String>) -> Result<Option<String>, String> {
    let Some(category_id) = category_id else {
        return Ok(None);
    };

    conn.query_row(
        "SELECT name FROM catalog_categories WHERE id = ?1 LIMIT 1",
        params![category_id],
        |row| row.get::<_, String>(0),
    )
    .optional()
    .map_err(|error| to_command_error("failed to load category name", error))
}

fn load_unit_code(conn: &Connection, unit_id: &Option<String>) -> Result<Option<String>, String> {
    let Some(unit_id) = unit_id else {
        return Ok(None);
    };

    conn.query_row(
        "SELECT code FROM catalog_units WHERE id = ?1 LIMIT 1",
        params![unit_id],
        |row| row.get::<_, String>(0),
    )
    .optional()
    .map_err(|error| to_command_error("failed to load unit code", error))
}

fn load_tax_label(conn: &Connection, tax_profile_id: &Option<String>) -> Result<Option<String>, String> {
    let Some(tax_profile_id) = tax_profile_id else {
        return Ok(None);
    };

    conn.query_row(
        "SELECT label FROM tax_profiles WHERE id = ?1 LIMIT 1",
        params![tax_profile_id],
        |row| row.get::<_, String>(0),
    )
    .optional()
    .map_err(|error| to_command_error("failed to load item tax label", error))
}

fn list_barcodes_for_item(conn: &Connection, item_id: &str) -> Result<Vec<CatalogBarcode>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, item_id, barcode, label, is_primary, created_at
             FROM catalog_item_barcodes
             WHERE item_id = ?1
             ORDER BY is_primary DESC, created_at ASC",
        )
        .map_err(|error| to_command_error("failed to prepare barcode query", error))?;

    let rows = stmt
        .query_map(params![item_id], barcode_from_row)
        .map_err(|error| to_command_error("failed to query barcodes", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map barcodes", error))
}

pub fn ensure_system_units(conn: &Connection) -> rusqlite::Result<()> {
    let now = db::now_iso();
    for (id, name, code, allow_fractional) in SYSTEM_UNITS {
        conn.execute(
            "INSERT OR IGNORE INTO catalog_units (
                id, business_id, name, code, symbol, allow_fractional,
                is_system, created_at, updated_at, archived_at
             ) VALUES (
                ?1, NULL, ?2, ?3, ?4, ?5,
                1, ?6, ?7, NULL
             )",
            params![
                id,
                name,
                code,
                code,
                bool_to_i64(allow_fractional),
                &now,
                &now,
            ],
        )?;
    }
    Ok(())
}

pub fn seed_demo_catalog_for_business(conn: &Connection, business_id: &str) -> rusqlite::Result<()> {
    let item_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM catalog_items WHERE business_id = ?1",
        params![business_id],
        |row| row.get(0),
    )?;
    if item_count > 0 {
        return Ok(());
    }

    ensure_system_units(conn)?;

    let now = db::now_iso();
    let id_suffix = business_id.chars().take(6).collect::<String>().to_lowercase();
    let categories = [
        (
            format!("{business_id}-cat-beverages"),
            "Beverages",
            "CAT-BEV",
            "menu",
            10_i64,
            Some("Coffee, tea, juices, and drinks".to_string()),
        ),
        (
            format!("{business_id}-cat-food"),
            "Bakery & Food",
            "CAT-FOOD",
            "menu",
            20_i64,
            Some("Baked goods and ready-to-serve food".to_string()),
        ),
        (
            format!("{business_id}-cat-retail"),
            "Retail Shelf",
            "CAT-RTL",
            "stock",
            30_i64,
            Some("Packaged goods and shelf products".to_string()),
        ),
        (
            format!("{business_id}-cat-service"),
            "Services",
            "CAT-SVC",
            "service",
            40_i64,
            Some("Simple service charges".to_string()),
        ),
    ];

    for (id, name, code, item_scope, sort_order, notes) in categories {
        conn.execute(
            "INSERT OR IGNORE INTO catalog_categories (
                id, business_id, name, code, parent_id, item_scope, sort_order,
                notes, created_at, updated_at, archived_at
             ) VALUES (?1, ?2, ?3, ?4, NULL, ?5, ?6, ?7, ?8, ?9, NULL)",
            params![
                id,
                business_id,
                name,
                code,
                item_scope,
                sort_order,
                notes,
                &now,
                &now,
            ],
        )?;
    }

    let default_tax_profile_id: Option<String> = conn
        .query_row(
            "SELECT id FROM tax_profiles WHERE business_id = ?1 ORDER BY is_default DESC, updated_at DESC LIMIT 1",
            params![business_id],
            |row| row.get(0),
        )
        .optional()?;

    let items = [
        (
            format!("{business_id}-item-cappuccino"),
            format!("{business_id}-cat-beverages"),
            Some("system-pcs".to_string()),
            default_tax_profile_id.clone(),
            "menu",
            "House Cappuccino",
            Some("Cappuccino".to_string()),
            Some("MENU-CAP".to_string()),
            Some(vec![format!("MENU-{id_suffix}-001")]),
            Some("Demo menu item for faster catalog validation.".to_string()),
            120.0_f64,
            35.0_f64,
            false,
            0.0_f64,
            0.0_f64,
        ),
        (
            format!("{business_id}-item-croissant"),
            format!("{business_id}-cat-food"),
            Some("system-pcs".to_string()),
            default_tax_profile_id.clone(),
            "menu",
            "Butter Croissant",
            None,
            Some("MENU-CROI".to_string()),
            Some(vec![format!("MENU-{id_suffix}-002")]),
            Some("Demo bakery item.".to_string()),
            90.0_f64,
            28.0_f64,
            false,
            0.0_f64,
            0.0_f64,
        ),
        (
            format!("{business_id}-item-water"),
            format!("{business_id}-cat-retail"),
            Some("system-pcs".to_string()),
            default_tax_profile_id.clone(),
            "stock",
            "Mineral Water 1L",
            None,
            Some("RTL-WATER-1L".to_string()),
            Some(vec![format!("890{id_suffix}1001")]),
            Some("Demo stock item with reorder tracking.".to_string()),
            25.0_f64,
            12.0_f64,
            true,
            24.0_f64,
            8.0_f64,
        ),
        (
            format!("{business_id}-item-delivery"),
            format!("{business_id}-cat-service"),
            Some("system-hour".to_string()),
            default_tax_profile_id.clone(),
            "service",
            "Local Delivery Charge",
            Some("Delivery".to_string()),
            Some("SVC-DEL".to_string()),
            None,
            Some("Demo service item.".to_string()),
            50.0_f64,
            0.0_f64,
            false,
            0.0_f64,
            0.0_f64,
        ),
    ];

    for (
        item_id,
        category_id,
        unit_id,
        tax_profile_id,
        item_kind,
        name,
        display_name,
        sku,
        barcodes,
        description,
        selling_price,
        cost_price,
        track_stock,
        stock_quantity,
        reorder_level,
    ) in items {
        conn.execute(
            "INSERT OR IGNORE INTO catalog_items (
                id, business_id, category_id, unit_id, tax_profile_id, item_kind,
                name, display_name, sku, primary_barcode, description,
                selling_price, cost_price, track_stock, stock_quantity,
                reorder_level, image_path, is_active, created_at, updated_at, archived_at
             ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6,
                ?7, ?8, ?9, ?10, ?11,
                ?12, ?13, ?14, ?15,
                ?16, NULL, 1, ?17, ?18, NULL
             )",
            params![
                &item_id,
                business_id,
                &category_id,
                unit_id,
                tax_profile_id,
                item_kind,
                name,
                display_name,
                sku,
                barcodes.as_ref().and_then(|list| list.first().cloned()),
                description,
                selling_price,
                cost_price,
                bool_to_i64(track_stock),
                stock_quantity,
                reorder_level,
                &now,
                &now,
            ],
        )?;

        if let Some(barcodes) = barcodes {
            for (index, barcode) in barcodes.iter().enumerate() {
                conn.execute(
                    "INSERT OR IGNORE INTO catalog_item_barcodes (
                        id, item_id, barcode, label, is_primary, created_at
                     ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                    params![
                        format!("{item_id}-barcode-{}", index + 1),
                        &item_id,
                        barcode,
                        Some("demo".to_string()),
                        bool_to_i64(index == 0),
                        &now,
                    ],
                )?;
            }
        }
    }

    Ok(())
}

pub fn list_catalog_categories(conn: &Connection, business_id: &str) -> Result<Vec<CatalogCategory>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, business_id, name, code, parent_id, item_scope, sort_order, notes, created_at, updated_at, archived_at
             FROM catalog_categories
             WHERE business_id = ?1
             ORDER BY CASE WHEN archived_at IS NULL THEN 0 ELSE 1 END, sort_order ASC, name COLLATE NOCASE ASC",
        )
        .map_err(|error| to_command_error("failed to prepare category query", error))?;

    let rows = stmt
        .query_map(params![business_id], category_from_row)
        .map_err(|error| to_command_error("failed to query catalog categories", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map catalog categories", error))
}

pub fn list_catalog_units(conn: &Connection, business_id: &str) -> Result<Vec<CatalogUnit>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, business_id, name, code, symbol, allow_fractional, is_system, updated_at, archived_at
             FROM catalog_units
             WHERE archived_at IS NULL AND (business_id IS NULL OR business_id = ?1)
             ORDER BY CASE WHEN business_id IS NULL THEN 0 ELSE 1 END, name COLLATE NOCASE ASC",
        )
        .map_err(|error| to_command_error("failed to prepare unit query", error))?;

    let rows = stmt
        .query_map(params![business_id], unit_from_row)
        .map_err(|error| to_command_error("failed to query catalog units", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map catalog units", error))
}

pub fn list_catalog_items(conn: &Connection, business_id: &str) -> Result<Vec<CatalogItem>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                id, business_id, category_id, unit_id, tax_profile_id, item_kind,
                name, display_name, sku, primary_barcode, description,
                selling_price, cost_price, track_stock, stock_quantity,
                reorder_level, image_path, is_active, created_at, updated_at, archived_at
             FROM catalog_items
             WHERE business_id = ?1
             ORDER BY CASE WHEN archived_at IS NULL THEN 0 ELSE 1 END, updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare item query", error))?;

    let rows = stmt
        .query_map(params![business_id], item_from_row)
        .map_err(|error| to_command_error("failed to query catalog items", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map catalog items", error))
}

fn get_catalog_item(conn: &Connection, business_id: &str, item_id: &str) -> Result<CatalogItem, String> {
    conn.query_row(
        "SELECT
            id, business_id, category_id, unit_id, tax_profile_id, item_kind,
            name, display_name, sku, primary_barcode, description,
            selling_price, cost_price, track_stock, stock_quantity,
            reorder_level, image_path, is_active, created_at, updated_at, archived_at
         FROM catalog_items
         WHERE business_id = ?1 AND id = ?2
         LIMIT 1",
        params![business_id, item_id],
        item_from_row,
    )
    .map_err(|error| to_command_error("failed to load catalog item", error))
}

pub fn build_catalog_summary(conn: &Connection, business_id: &str) -> Result<CatalogSummary, String> {
    let total_items: usize = conn
        .query_row(
            "SELECT COUNT(*) FROM catalog_items WHERE business_id = ?1",
            params![business_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| to_command_error("failed to count catalog items", error))?
        as usize;

    let active_items: usize = conn
        .query_row(
            "SELECT COUNT(*) FROM catalog_items WHERE business_id = ?1 AND archived_at IS NULL AND is_active = 1",
            params![business_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| to_command_error("failed to count active items", error))?
        as usize;

    let archived_items: usize = conn
        .query_row(
            "SELECT COUNT(*) FROM catalog_items WHERE business_id = ?1 AND archived_at IS NOT NULL",
            params![business_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| to_command_error("failed to count archived items", error))?
        as usize;

    let category_count: usize = conn
        .query_row(
            "SELECT COUNT(*) FROM catalog_categories WHERE business_id = ?1 AND archived_at IS NULL",
            params![business_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| to_command_error("failed to count categories", error))?
        as usize;

    let menu_item_count: usize = conn
        .query_row(
            "SELECT COUNT(*) FROM catalog_items WHERE business_id = ?1 AND item_kind = 'menu' AND archived_at IS NULL",
            params![business_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| to_command_error("failed to count menu items", error))?
        as usize;

    let stock_item_count: usize = conn
        .query_row(
            "SELECT COUNT(*) FROM catalog_items WHERE business_id = ?1 AND item_kind = 'stock' AND archived_at IS NULL",
            params![business_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| to_command_error("failed to count stock items", error))?
        as usize;

    let service_item_count: usize = conn
        .query_row(
            "SELECT COUNT(*) FROM catalog_items WHERE business_id = ?1 AND item_kind = 'service' AND archived_at IS NULL",
            params![business_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| to_command_error("failed to count service items", error))?
        as usize;

    let low_stock_candidates: usize = conn
        .query_row(
            "SELECT COUNT(*)
             FROM catalog_items
             WHERE business_id = ?1
               AND archived_at IS NULL
               AND is_active = 1
               AND track_stock = 1
               AND reorder_level > 0
               AND stock_quantity <= reorder_level",
            params![business_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| to_command_error("failed to count low stock candidates", error))?
        as usize;

    Ok(CatalogSummary {
        total_items,
        active_items,
        archived_items,
        category_count,
        menu_item_count,
        stock_item_count,
        service_item_count,
        low_stock_candidates,
    })
}

pub fn load_catalog_workspace(conn: &Connection, business_id: &str) -> Result<CatalogWorkspace, String> {
    let items = list_catalog_items(conn, business_id)?;
    let item_views = items
        .into_iter()
        .map(|item| {
            let category_name = load_category_name(conn, &item.category_id)?;
            let unit_code = load_unit_code(conn, &item.unit_id)?;
            let tax_label = load_tax_label(conn, &item.tax_profile_id)?;
            let barcodes = list_barcodes_for_item(conn, &item.id)?;
            Ok(CatalogItemView {
                category_name,
                unit_code,
                tax_label,
                barcodes,
                item,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;

    Ok(CatalogWorkspace {
        business_id: business_id.to_string(),
        summary: build_catalog_summary(conn, business_id)?,
        categories: list_catalog_categories(conn, business_id)?,
        units: list_catalog_units(conn, business_id)?,
        tax_profiles: db::list_tax_profiles(conn, business_id)?,
        items: item_views,
    })
}

fn validate_category_scope(conn: &Connection, business_id: &str, category_id: &Option<String>) -> Result<(), String> {
    if let Some(category_id) = category_id {
        let exists: Option<String> = conn
            .query_row(
                "SELECT id FROM catalog_categories WHERE business_id = ?1 AND id = ?2 LIMIT 1",
                params![business_id, category_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(|error| to_command_error("failed to validate category", error))?;
        if exists.is_none() {
            return Err("selected category does not belong to the active business".into());
        }
    }
    Ok(())
}

fn validate_unit_scope(conn: &Connection, business_id: &str, unit_id: &Option<String>) -> Result<(), String> {
    if let Some(unit_id) = unit_id {
        let exists: Option<String> = conn
            .query_row(
                "SELECT id FROM catalog_units WHERE id = ?1 AND archived_at IS NULL AND (business_id IS NULL OR business_id = ?2) LIMIT 1",
                params![unit_id, business_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(|error| to_command_error("failed to validate unit", error))?;
        if exists.is_none() {
            return Err("selected unit is not available in the active business".into());
        }
    }
    Ok(())
}

fn validate_tax_scope(conn: &Connection, business_id: &str, tax_profile_id: &Option<String>) -> Result<(), String> {
    if let Some(tax_profile_id) = tax_profile_id {
        let exists: Option<String> = conn
            .query_row(
                "SELECT id FROM tax_profiles WHERE business_id = ?1 AND id = ?2 LIMIT 1",
                params![business_id, tax_profile_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(|error| to_command_error("failed to validate tax profile", error))?;
        if exists.is_none() {
            return Err("selected tax profile does not belong to the active business".into());
        }
    }
    Ok(())
}

pub fn save_catalog_category(
    conn: &Connection,
    business_id: &str,
    input: &SaveCatalogCategoryInput,
) -> Result<CatalogCategory, String> {
    let name = normalize_text(&input.name, "category name")?;
    let code = normalize_code(&input.code, "category code")?;
    let item_scope = normalize_scope(&input.item_scope);
    let now = db::now_iso();
    let category_id = input.id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());
    let parent_id = normalize_optional(&input.parent_id);
    validate_category_scope(conn, business_id, &parent_id)?;

    if parent_id.as_deref() == Some(category_id.as_str()) {
        return Err("a category cannot be its own parent".into());
    }

    conn.execute(
        "INSERT INTO catalog_categories (
            id, business_id, name, code, parent_id, item_scope, sort_order,
            notes, created_at, updated_at, archived_at
         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, NULL)
         ON CONFLICT(id) DO UPDATE SET
            name = excluded.name,
            code = excluded.code,
            parent_id = excluded.parent_id,
            item_scope = excluded.item_scope,
            sort_order = excluded.sort_order,
            notes = excluded.notes,
            updated_at = excluded.updated_at",
        params![
            &category_id,
            business_id,
            &name,
            &code,
            parent_id,
            &item_scope,
            input.sort_order.max(0),
            normalize_optional(&input.notes),
            &now,
            &now,
        ],
    )
    .map_err(|error| to_command_error("failed to save catalog category", error))?;

    db::insert_log(conn, "INFO", "catalog", "Catalog category saved", None)?;

    conn.query_row(
        "SELECT id, business_id, name, code, parent_id, item_scope, sort_order, notes, created_at, updated_at, archived_at
         FROM catalog_categories
         WHERE business_id = ?1 AND id = ?2
         LIMIT 1",
        params![business_id, &category_id],
        category_from_row,
    )
    .map_err(|error| to_command_error("failed to reload catalog category", error))
}

pub fn save_catalog_unit(
    conn: &Connection,
    business_id: &str,
    input: &SaveCatalogUnitInput,
) -> Result<CatalogUnit, String> {
    let name = normalize_text(&input.name, "unit name")?;
    let code = normalize_code(&input.code, "unit code")?;
    let symbol = normalize_text(&input.symbol, "unit symbol")?;
    let now = db::now_iso();
    let unit_id = input.id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());

    let system_unit: Option<String> = conn
        .query_row(
            "SELECT id FROM catalog_units WHERE id = ?1 AND is_system = 1 LIMIT 1",
            params![&unit_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|error| to_command_error("failed to validate unit update", error))?;
    if system_unit.is_some() {
        return Err("system units cannot be edited".into());
    }

    let duplicate: Option<String> = conn
        .query_row(
            "SELECT id
             FROM catalog_units
             WHERE archived_at IS NULL
               AND code = ?1
               AND ((business_id = ?2 AND is_system = 0) OR business_id IS NULL)
               AND id != ?3
             LIMIT 1",
            params![&code, business_id, &unit_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|error| to_command_error("failed to validate unit code", error))?;
    if duplicate.is_some() {
        return Err("another unit already uses this code".into());
    }

    conn.execute(
        "INSERT INTO catalog_units (
            id, business_id, name, code, symbol, allow_fractional,
            is_system, created_at, updated_at, archived_at
         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0, ?7, ?8, NULL)
         ON CONFLICT(id) DO UPDATE SET
            name = excluded.name,
            code = excluded.code,
            symbol = excluded.symbol,
            allow_fractional = excluded.allow_fractional,
            updated_at = excluded.updated_at",
        params![
            &unit_id,
            business_id,
            &name,
            &code,
            &symbol,
            bool_to_i64(input.allow_fractional),
            &now,
            &now,
        ],
    )
    .map_err(|error| to_command_error("failed to save catalog unit", error))?;

    db::insert_log(conn, "INFO", "catalog", "Catalog unit saved", None)?;

    conn.query_row(
        "SELECT id, business_id, name, code, symbol, allow_fractional, is_system, updated_at, archived_at
         FROM catalog_units
         WHERE id = ?1 AND (business_id = ?2 OR business_id IS NULL)
         LIMIT 1",
        params![&unit_id, business_id],
        unit_from_row,
    )
    .map_err(|error| to_command_error("failed to reload catalog unit", error))
}

fn normalize_barcodes(values: &[String]) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut barcodes = Vec::new();
    for value in values {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            continue;
        }
        let normalized = trimmed.to_string();
        if seen.insert(normalized.clone()) {
            barcodes.push(normalized);
        }
    }
    barcodes
}

pub fn save_catalog_item(
    conn: &Connection,
    business_id: &str,
    input: &SaveCatalogItemInput,
) -> Result<CatalogItem, String> {
    let item_id = input.id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());
    let name = normalize_text(&input.name, "item name")?;
    let item_kind = normalize_item_kind(&input.item_kind);
    let category_id = normalize_optional(&input.category_id);
    let unit_id = normalize_optional(&input.unit_id);
    let tax_profile_id = normalize_optional(&input.tax_profile_id);
    validate_category_scope(conn, business_id, &category_id)?;
    validate_unit_scope(conn, business_id, &unit_id)?;
    validate_tax_scope(conn, business_id, &tax_profile_id)?;
    let display_name = normalize_optional(&input.display_name);
    let sku = input.sku.as_ref().map(|value| value.trim().to_uppercase()).filter(|value| !value.is_empty());
    let description = normalize_optional(&input.description);
    let image_path = normalize_optional(&input.image_path);
    let barcodes = normalize_barcodes(&input.barcodes);
    let primary_barcode = barcodes.first().cloned();
    let selling_price = input.selling_price.max(0.0);
    let cost_price = input.cost_price.max(0.0);
    let track_stock = input.track_stock && item_kind == "stock";
    let stock_quantity = if track_stock { input.stock_quantity.max(0.0) } else { 0.0 };
    let reorder_level = if track_stock { input.reorder_level.max(0.0) } else { 0.0 };
    let now = db::now_iso();

    conn.execute(
        "INSERT INTO catalog_items (
            id, business_id, category_id, unit_id, tax_profile_id, item_kind,
            name, display_name, sku, primary_barcode, description,
            selling_price, cost_price, track_stock, stock_quantity,
            reorder_level, image_path, is_active, created_at, updated_at, archived_at
         ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6,
            ?7, ?8, ?9, ?10, ?11,
            ?12, ?13, ?14, ?15,
            ?16, ?17, ?18, ?19, ?20, NULL
         )
         ON CONFLICT(id) DO UPDATE SET
            category_id = excluded.category_id,
            unit_id = excluded.unit_id,
            tax_profile_id = excluded.tax_profile_id,
            item_kind = excluded.item_kind,
            name = excluded.name,
            display_name = excluded.display_name,
            sku = excluded.sku,
            primary_barcode = excluded.primary_barcode,
            description = excluded.description,
            selling_price = excluded.selling_price,
            cost_price = excluded.cost_price,
            track_stock = excluded.track_stock,
            stock_quantity = excluded.stock_quantity,
            reorder_level = excluded.reorder_level,
            image_path = excluded.image_path,
            is_active = excluded.is_active,
            updated_at = excluded.updated_at",
        params![
            &item_id,
            business_id,
            category_id,
            unit_id,
            tax_profile_id,
            &item_kind,
            &name,
            display_name,
            sku,
            primary_barcode,
            description,
            selling_price,
            cost_price,
            bool_to_i64(track_stock),
            stock_quantity,
            reorder_level,
            image_path,
            bool_to_i64(input.is_active),
            &now,
            &now,
        ],
    )
    .map_err(|error| to_command_error("failed to save catalog item", error))?;

    conn.execute(
        "DELETE FROM catalog_item_barcodes WHERE item_id = ?1",
        params![&item_id],
    )
    .map_err(|error| to_command_error("failed to reset item barcodes", error))?;

    for (index, barcode) in barcodes.iter().enumerate() {
        conn.execute(
            "INSERT INTO catalog_item_barcodes (id, item_id, barcode, label, is_primary, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                Uuid::new_v4().to_string(),
                &item_id,
                barcode,
                Some("manual".to_string()),
                bool_to_i64(index == 0),
                &now,
            ],
        )
        .map_err(|error| to_command_error("failed to save item barcode", error))?;
    }

    db::insert_log(conn, "INFO", "catalog", "Catalog item saved", None)?;
    get_catalog_item(conn, business_id, &item_id)
}

pub fn set_catalog_item_archived(
    conn: &Connection,
    business_id: &str,
    item_id: &str,
    archived: bool,
) -> Result<CatalogItem, String> {
    let archived_at = if archived { Some(db::now_iso()) } else { None };
    conn.execute(
        "UPDATE catalog_items
         SET archived_at = ?3,
             is_active = ?4,
             updated_at = ?5
         WHERE business_id = ?1 AND id = ?2",
        params![
            business_id,
            item_id,
            archived_at,
            bool_to_i64(!archived),
            db::now_iso(),
        ],
    )
    .map_err(|error| to_command_error("failed to archive catalog item", error))?;

    db::insert_log(conn, "INFO", "catalog", "Catalog item archive state changed", None)?;
    get_catalog_item(conn, business_id, item_id)
}

pub fn list_all_catalog_categories(conn: &Connection) -> Result<Vec<CatalogCategory>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, business_id, name, code, parent_id, item_scope, sort_order, notes, created_at, updated_at, archived_at
             FROM catalog_categories
             ORDER BY updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare all-category query", error))?;

    let rows = stmt
        .query_map([], category_from_row)
        .map_err(|error| to_command_error("failed to query all categories", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map all categories", error))
}

pub fn list_all_catalog_units(conn: &Connection) -> Result<Vec<CatalogUnit>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, business_id, name, code, symbol, allow_fractional, is_system, updated_at, archived_at
             FROM catalog_units
             ORDER BY updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare all-unit query", error))?;

    let rows = stmt
        .query_map([], unit_from_row)
        .map_err(|error| to_command_error("failed to query all units", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map all units", error))
}

pub fn list_all_catalog_items(conn: &Connection) -> Result<Vec<CatalogItem>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                id, business_id, category_id, unit_id, tax_profile_id, item_kind,
                name, display_name, sku, primary_barcode, description,
                selling_price, cost_price, track_stock, stock_quantity,
                reorder_level, image_path, is_active, created_at, updated_at, archived_at
             FROM catalog_items
             ORDER BY updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare all-item query", error))?;

    let rows = stmt
        .query_map([], item_from_row)
        .map_err(|error| to_command_error("failed to query all items", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map all items", error))
}

pub fn list_all_catalog_barcodes(conn: &Connection) -> Result<Vec<CatalogBarcode>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, item_id, barcode, label, is_primary, created_at
             FROM catalog_item_barcodes
             ORDER BY created_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare all-barcode query", error))?;

    let rows = stmt
        .query_map([], barcode_from_row)
        .map_err(|error| to_command_error("failed to query all barcodes", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map all barcodes", error))
}
