use rusqlite::{params, Connection, OptionalExtension, Row};
use uuid::Uuid;

use crate::domain::models::{
    CatalogItem, InventoryMovement, InventoryStockItem, InventorySummary, InventoryWorkspace,
    SaveInventoryMovementInput, SaveInventoryStockRuleInput,
};

use super::{db, error::to_command_error};

const EPSILON: f64 = 0.000_001;

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

fn normalize_non_negative(value: f64) -> f64 {
    if value.is_finite() && value > 0.0 {
        value
    } else {
        0.0
    }
}

fn is_low_stock(track_stock: bool, reorder_level: f64, stock_quantity: f64) -> bool {
    track_stock && reorder_level > 0.0 && stock_quantity <= reorder_level
}

fn canonical_zero(value: f64) -> f64 {
    if value.abs() < EPSILON {
        0.0
    } else {
        value
    }
}

fn stock_item_from_row(row: &Row) -> rusqlite::Result<InventoryStockItem> {
    let track_stock = bool_from_row(row, 6)?;
    let stock_quantity: f64 = row.get(7)?;
    let reorder_level: f64 = row.get(8)?;

    Ok(InventoryStockItem {
        item_id: row.get(0)?,
        item_name: row.get(1)?,
        item_kind: row.get(2)?,
        sku: row.get(3)?,
        category_name: row.get(4)?,
        unit_code: row.get(5)?,
        track_stock,
        stock_quantity,
        reorder_level,
        low_stock: is_low_stock(track_stock, reorder_level, stock_quantity),
        updated_at: row.get(9)?,
    })
}

fn movement_from_row(row: &Row) -> rusqlite::Result<InventoryMovement> {
    Ok(InventoryMovement {
        id: row.get(0)?,
        business_id: row.get(1)?,
        item_id: row.get(2)?,
        item_name: row.get(3)?,
        sku: row.get(4)?,
        unit_code: row.get(5)?,
        movement_type: row.get(6)?,
        quantity_delta: row.get(7)?,
        quantity_after: row.get(8)?,
        unit_cost: row.get(9)?,
        note: row.get(10)?,
        occurred_at: row.get(11)?,
        created_at: row.get(12)?,
    })
}

fn load_inventory_item(
    conn: &Connection,
    business_id: &str,
    item_id: &str,
) -> Result<InventoryStockItem, String> {
    conn.query_row(
        "SELECT
            i.id,
            i.name,
            i.item_kind,
            i.sku,
            c.name,
            u.code,
            i.track_stock,
            i.stock_quantity,
            i.reorder_level,
            i.updated_at
         FROM catalog_items i
         LEFT JOIN catalog_categories c ON c.id = i.category_id
         LEFT JOIN catalog_units u ON u.id = i.unit_id
         WHERE i.business_id = ?1
           AND i.id = ?2
           AND i.archived_at IS NULL
         LIMIT 1",
        params![business_id, item_id],
        stock_item_from_row,
    )
    .map_err(|error| to_command_error("failed to load inventory item", error))
}

fn get_inventory_movement(
    conn: &Connection,
    business_id: &str,
    movement_id: &str,
) -> Result<InventoryMovement, String> {
    conn.query_row(
        "SELECT
            m.id,
            m.business_id,
            m.item_id,
            i.name,
            i.sku,
            u.code,
            m.movement_type,
            m.quantity_delta,
            m.quantity_after,
            m.unit_cost,
            m.note,
            m.occurred_at,
            m.created_at
         FROM inventory_stock_movements m
         INNER JOIN catalog_items i ON i.id = m.item_id
         LEFT JOIN catalog_units u ON u.id = i.unit_id
         WHERE m.business_id = ?1
           AND m.id = ?2
         LIMIT 1",
        params![business_id, movement_id],
        movement_from_row,
    )
    .map_err(|error| to_command_error("failed to load inventory movement", error))
}

fn insert_movement_record(
    conn: &Connection,
    movement_id: &str,
    business_id: &str,
    item_id: &str,
    movement_type: &str,
    quantity_delta: f64,
    quantity_after: f64,
    unit_cost: Option<f64>,
    note: Option<&str>,
    occurred_at: &str,
    created_at: &str,
    ignore_conflict: bool,
) -> Result<(), String> {
    let sql = if ignore_conflict {
        "INSERT OR IGNORE INTO inventory_stock_movements (
            id, business_id, item_id, movement_type, quantity_delta, quantity_after,
            unit_cost, note, occurred_at, created_at
         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)"
    } else {
        "INSERT INTO inventory_stock_movements (
            id, business_id, item_id, movement_type, quantity_delta, quantity_after,
            unit_cost, note, occurred_at, created_at
         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)"
    };

    conn.execute(
        sql,
        params![
            movement_id,
            business_id,
            item_id,
            movement_type,
            canonical_zero(quantity_delta),
            canonical_zero(quantity_after),
            unit_cost,
            note,
            occurred_at,
            created_at,
        ],
    )
    .map_err(|error| to_command_error("failed to write inventory movement", error))?;

    Ok(())
}

fn normalize_manual_movement_type(value: &str) -> Result<String, String> {
    match value.trim().to_lowercase().as_str() {
        "stock_in" => Ok("stock_in".into()),
        "stock_out" => Ok("stock_out".into()),
        "adjustment_in" => Ok("adjustment_in".into()),
        "adjustment_out" => Ok("adjustment_out".into()),
        _ => Err("invalid movement type".into()),
    }
}

fn delta_for_movement_type(movement_type: &str, quantity: f64) -> Result<f64, String> {
    let normalized = quantity.abs();
    if !normalized.is_finite() || normalized <= 0.0 {
        return Err("quantity must be greater than zero".into());
    }

    match movement_type {
        "stock_in" | "adjustment_in" | "opening_balance" => Ok(normalized),
        "stock_out" | "adjustment_out" => Ok(-normalized),
        _ => Err("unsupported movement type".into()),
    }
}

pub fn list_inventory_stock_items(
    conn: &Connection,
    business_id: &str,
) -> Result<Vec<InventoryStockItem>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                i.id,
                i.name,
                i.item_kind,
                i.sku,
                c.name,
                u.code,
                i.track_stock,
                i.stock_quantity,
                i.reorder_level,
                i.updated_at
             FROM catalog_items i
             LEFT JOIN catalog_categories c ON c.id = i.category_id
             LEFT JOIN catalog_units u ON u.id = i.unit_id
             WHERE i.business_id = ?1
               AND i.archived_at IS NULL
               AND i.item_kind = 'stock'
             ORDER BY
                CASE
                  WHEN i.track_stock = 1 AND i.reorder_level > 0 AND i.stock_quantity <= i.reorder_level THEN 0
                  ELSE 1
                END,
                i.name COLLATE NOCASE ASC",
        )
        .map_err(|error| to_command_error("failed to prepare inventory item query", error))?;

    let rows = stmt
        .query_map(params![business_id], stock_item_from_row)
        .map_err(|error| to_command_error("failed to query inventory items", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map inventory items", error))
}

pub fn list_recent_inventory_movements(
    conn: &Connection,
    business_id: &str,
    limit: usize,
) -> Result<Vec<InventoryMovement>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                m.id,
                m.business_id,
                m.item_id,
                i.name,
                i.sku,
                u.code,
                m.movement_type,
                m.quantity_delta,
                m.quantity_after,
                m.unit_cost,
                m.note,
                m.occurred_at,
                m.created_at
             FROM inventory_stock_movements m
             INNER JOIN catalog_items i ON i.id = m.item_id
             LEFT JOIN catalog_units u ON u.id = i.unit_id
             WHERE m.business_id = ?1
             ORDER BY m.occurred_at DESC, m.created_at DESC
             LIMIT ?2",
        )
        .map_err(|error| {
            to_command_error("failed to prepare recent inventory movement query", error)
        })?;

    let rows = stmt
        .query_map(params![business_id, limit as i64], movement_from_row)
        .map_err(|error| to_command_error("failed to query recent inventory movements", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map recent inventory movements", error))
}

pub fn list_all_inventory_movements(conn: &Connection) -> Result<Vec<InventoryMovement>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                m.id,
                m.business_id,
                m.item_id,
                i.name,
                i.sku,
                u.code,
                m.movement_type,
                m.quantity_delta,
                m.quantity_after,
                m.unit_cost,
                m.note,
                m.occurred_at,
                m.created_at
             FROM inventory_stock_movements m
             INNER JOIN catalog_items i ON i.id = m.item_id
             LEFT JOIN catalog_units u ON u.id = i.unit_id
             ORDER BY m.occurred_at DESC, m.created_at DESC",
        )
        .map_err(|error| {
            to_command_error("failed to prepare full inventory movement query", error)
        })?;

    let rows = stmt
        .query_map([], movement_from_row)
        .map_err(|error| to_command_error("failed to query all inventory movements", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map all inventory movements", error))
}

pub fn build_inventory_summary(
    conn: &Connection,
    business_id: &str,
) -> Result<InventorySummary, String> {
    let total_tracked_items: usize = conn
        .query_row(
            "SELECT COUNT(*)
             FROM catalog_items
             WHERE business_id = ?1
               AND archived_at IS NULL
               AND item_kind = 'stock'
               AND track_stock = 1",
            params![business_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| to_command_error("failed to count tracked stock items", error))?
        as usize;

    let low_stock_items: usize = conn
        .query_row(
            "SELECT COUNT(*)
             FROM catalog_items
             WHERE business_id = ?1
               AND archived_at IS NULL
               AND item_kind = 'stock'
               AND track_stock = 1
               AND reorder_level > 0
               AND stock_quantity <= reorder_level",
            params![business_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| to_command_error("failed to count low stock items", error))?
        as usize;

    let movement_count: usize = conn
        .query_row(
            "SELECT COUNT(*)
             FROM inventory_stock_movements
             WHERE business_id = ?1",
            params![business_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| to_command_error("failed to count inventory movements", error))?
        as usize;

    let total_quantity_on_hand: f64 = conn
        .query_row(
            "SELECT COALESCE(SUM(stock_quantity), 0)
             FROM catalog_items
             WHERE business_id = ?1
               AND archived_at IS NULL
               AND item_kind = 'stock'
               AND track_stock = 1",
            params![business_id],
            |row| row.get(0),
        )
        .map_err(|error| to_command_error("failed to total stock quantity", error))?;

    Ok(InventorySummary {
        total_tracked_items,
        low_stock_items,
        movement_count,
        total_quantity_on_hand: canonical_zero(total_quantity_on_hand),
    })
}

pub fn load_inventory_workspace(
    conn: &Connection,
    business_id: &str,
) -> Result<InventoryWorkspace, String> {
    Ok(InventoryWorkspace {
        business_id: business_id.to_string(),
        summary: build_inventory_summary(conn, business_id)?,
        stock_items: list_inventory_stock_items(conn, business_id)?,
        recent_movements: list_recent_inventory_movements(conn, business_id, 20)?,
    })
}

pub fn record_inventory_movement(
    conn: &Connection,
    business_id: &str,
    input: &SaveInventoryMovementInput,
) -> Result<InventoryMovement, String> {
    let item = load_inventory_item(conn, business_id, &input.item_id)?;
    if item.item_kind != "stock" {
        return Err("only stock items can be used in the inventory ledger".into());
    }
    if !item.track_stock {
        return Err("enable stock tracking for this item before recording movements".into());
    }

    let movement_type = normalize_manual_movement_type(&input.movement_type)?;
    let quantity_delta = delta_for_movement_type(&movement_type, input.quantity)?;
    let quantity_after = canonical_zero(item.stock_quantity + quantity_delta);

    if quantity_after < 0.0 {
        return Err("movement would push stock below zero".into());
    }

    let now = db::now_iso();
    let normalized_note = normalize_optional(&input.note);
    let normalized_unit_cost = input.unit_cost.and_then(|value| {
        if value.is_finite() && value >= 0.0 {
            Some(value)
        } else {
            None
        }
    });
    let movement_id = Uuid::new_v4().to_string();

    conn.execute(
        "UPDATE catalog_items
         SET stock_quantity = ?3,
             updated_at = ?4
         WHERE business_id = ?1 AND id = ?2",
        params![business_id, &item.item_id, quantity_after, &now],
    )
    .map_err(|error| to_command_error("failed to update stock quantity", error))?;

    insert_movement_record(
        conn,
        &movement_id,
        business_id,
        &item.item_id,
        &movement_type,
        quantity_delta,
        quantity_after,
        normalized_unit_cost,
        normalized_note.as_deref(),
        &now,
        &now,
        false,
    )?;

    db::insert_log(
        conn,
        "INFO",
        "inventory",
        "Inventory movement recorded",
        None,
    )?;

    get_inventory_movement(conn, business_id, &movement_id)
}

pub fn save_inventory_stock_rule(
    conn: &Connection,
    business_id: &str,
    input: &SaveInventoryStockRuleInput,
) -> Result<InventoryStockItem, String> {
    let item = load_inventory_item(conn, business_id, &input.item_id)?;
    if item.item_kind != "stock" {
        return Err("only stock items can be configured in the inventory workspace".into());
    }

    let now = db::now_iso();
    let track_stock = input.track_stock;
    let reorder_level = if track_stock {
        normalize_non_negative(input.reorder_level)
    } else {
        0.0
    };

    conn.execute(
        "UPDATE catalog_items
         SET track_stock = ?3,
             reorder_level = ?4,
             updated_at = ?5
         WHERE business_id = ?1 AND id = ?2",
        params![
            business_id,
            &input.item_id,
            if track_stock { 1_i64 } else { 0_i64 },
            reorder_level,
            &now,
        ],
    )
    .map_err(|error| to_command_error("failed to update stock rule", error))?;

    db::insert_log(
        conn,
        "INFO",
        "inventory",
        "Inventory stock rule updated",
        None,
    )?;

    load_inventory_item(conn, business_id, &input.item_id)
}

pub fn sync_catalog_stock(
    conn: &Connection,
    business_id: &str,
    previous_item: Option<&CatalogItem>,
    saved_item: &CatalogItem,
) -> Result<(), String> {
    let previous_quantity = previous_item
        .filter(|item| item.business_id == business_id)
        .map(|item| {
            if item.item_kind == "stock" && item.track_stock {
                item.stock_quantity
            } else {
                0.0
            }
        })
        .unwrap_or(0.0);

    let next_quantity = if saved_item.business_id == business_id
        && saved_item.item_kind == "stock"
        && saved_item.track_stock
    {
        saved_item.stock_quantity
    } else {
        0.0
    };

    let quantity_delta = canonical_zero(next_quantity - previous_quantity);
    if quantity_delta.abs() < EPSILON {
        return Ok(());
    }

    let now = db::now_iso();
    let note = if previous_item.is_some() {
        Some("Catalog stock quantity synced from item editor")
    } else {
        Some("Catalog item created with opening stock")
    };
    let movement_id = Uuid::new_v4().to_string();
    let unit_cost = if saved_item.cost_price > 0.0 {
        Some(saved_item.cost_price)
    } else {
        None
    };

    insert_movement_record(
        conn,
        &movement_id,
        business_id,
        &saved_item.id,
        "catalog_sync",
        quantity_delta,
        next_quantity,
        unit_cost,
        note,
        &now,
        &now,
        false,
    )?;

    db::insert_log(
        conn,
        "INFO",
        "inventory",
        "Catalog stock quantity sync recorded",
        None,
    )?;

    Ok(())
}

pub fn backfill_opening_balances(conn: &Connection) -> Result<(), String> {
    let mut stmt = conn
        .prepare(
            "SELECT business_id, id, stock_quantity, cost_price
             FROM catalog_items
             WHERE archived_at IS NULL
               AND item_kind = 'stock'
               AND track_stock = 1
               AND stock_quantity > 0",
        )
        .map_err(|error| to_command_error("failed to prepare opening balance backfill", error))?;

    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, f64>(2)?,
                row.get::<_, f64>(3)?,
            ))
        })
        .map_err(|error| to_command_error("failed to query opening balance candidates", error))?;

    let mut inserted_count = 0_usize;

    for row in rows {
        let (business_id, item_id, stock_quantity, cost_price) = row
            .map_err(|error| to_command_error("failed to map opening balance candidate", error))?;

        let movement_id = format!("opening-balance:{item_id}");
        let exists: Option<String> = conn
            .query_row(
                "SELECT id FROM inventory_stock_movements WHERE id = ?1 LIMIT 1",
                params![&movement_id],
                |db_row| db_row.get(0),
            )
            .optional()
            .map_err(|error| to_command_error("failed to check opening balance movement", error))?;

        if exists.is_some() {
            continue;
        }

        let now = db::now_iso();
        insert_movement_record(
            conn,
            &movement_id,
            &business_id,
            &item_id,
            "opening_balance",
            stock_quantity,
            stock_quantity,
            if cost_price > 0.0 {
                Some(cost_price)
            } else {
                None
            },
            Some("Opening balance from existing stock quantity"),
            &now,
            &now,
            true,
        )?;
        inserted_count += 1;
    }

    if inserted_count > 0 {
        db::insert_log(
            conn,
            "INFO",
            "inventory",
            "Opening balance movements backfilled",
            None,
        )?;
    }

    Ok(())
}
