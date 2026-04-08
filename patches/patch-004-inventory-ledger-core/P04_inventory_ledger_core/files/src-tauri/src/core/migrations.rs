use rusqlite::{Connection, OptionalExtension};

pub const CURRENT_SCHEMA_VERSION: i64 = 4;

const MIGRATION_001: &str = include_str!("migrations/001_base.sql");
const MIGRATION_002: &str = include_str!("migrations/002_multi_business_workspace.sql");
const MIGRATION_003: &str = include_str!("migrations/003_catalog_core.sql");
const MIGRATION_004: &str = include_str!("migrations/004_inventory_ledger_core.sql");

fn table_exists(conn: &Connection, table_name: &str) -> rusqlite::Result<bool> {
    let exists: Option<String> = conn
        .query_row(
            "SELECT name FROM sqlite_master WHERE type = 'table' AND name = ?1 LIMIT 1",
            [table_name],
            |row| row.get(0),
        )
        .optional()?;
    Ok(exists.is_some())
}

fn detect_current_schema_version(conn: &Connection) -> rusqlite::Result<i64> {
    let user_version: i64 = conn.query_row("PRAGMA user_version", [], |row| row.get(0))?;
    if user_version > 0 {
        return Ok(user_version);
    }

    if table_exists(conn, "inventory_stock_movements")? {
        return Ok(4);
    }

    if table_exists(conn, "catalog_items")? {
        return Ok(3);
    }

    if table_exists(conn, "tax_profiles")? {
        return Ok(2);
    }

    if table_exists(conn, "app_meta")? {
        let legacy_version = conn
            .query_row(
                "SELECT value FROM app_meta WHERE key = 'schema_version' LIMIT 1",
                [],
                |row| row.get::<_, String>(0),
            )
            .optional()?;

        if let Some(value) = legacy_version {
            if let Ok(parsed) = value.parse::<i64>() {
                return Ok(parsed);
            }
        }

        return Ok(1);
    }

    if table_exists(conn, "businesses")? {
        return Ok(1);
    }

    Ok(0)
}

pub fn run(conn: &Connection) -> rusqlite::Result<()> {
    let mut current_version = detect_current_schema_version(conn)?;

    if current_version < 1 {
        conn.execute_batch(MIGRATION_001)?;
        current_version = 1;
    }

    if current_version < 2 {
        conn.execute_batch(MIGRATION_002)?;
        current_version = 2;
    }

    if current_version < 3 {
        conn.execute_batch(MIGRATION_003)?;
        current_version = 3;
    }

    if current_version < 4 {
        conn.execute_batch(MIGRATION_004)?;
        current_version = 4;
    }

    conn.pragma_update(None, "user_version", current_version)?;
    Ok(())
}
