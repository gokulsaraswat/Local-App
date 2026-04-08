use chrono::Utc;
use rusqlite::{params, Connection};
use serde_json::json;

use super::migrations::CURRENT_SCHEMA_VERSION;

pub const PATCH_ID: &str = "P001_foundation_base_structure";
pub const PATCH_NAME: &str = "Foundation Base Structure";

pub fn register_patch(conn: &Connection) -> rusqlite::Result<()> {
    let now = Utc::now().to_rfc3339();
    let manifest = json!({
        "patch_id": PATCH_ID,
        "patch_name": PATCH_NAME,
        "schema_version": CURRENT_SCHEMA_VERSION,
        "applied_at": now.clone(),
        "notes": "Initial desktop foundation with local storage, business/settings shell, backup/export foundation."
    });

    conn.execute(
        "INSERT OR IGNORE INTO patch_history (patch_id, patch_name, schema_version, applied_at, manifest_json) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![PATCH_ID, PATCH_NAME, CURRENT_SCHEMA_VERSION, now, manifest.to_string()],
    )?;

    conn.execute(
        "INSERT INTO app_meta (key, value, updated_at)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
        params!["schema_version", CURRENT_SCHEMA_VERSION.to_string(), Utc::now().to_rfc3339()],
    )?;

    conn.execute(
        "INSERT INTO app_meta (key, value, updated_at)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
        params!["patch_level", PATCH_ID, Utc::now().to_rfc3339()],
    )?;

    Ok(())
}
