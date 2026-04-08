use chrono::Utc;
use rusqlite::{params, Connection};
use serde_json::json;

use super::migrations::CURRENT_SCHEMA_VERSION;

struct PatchDefinition {
    patch_id: &'static str,
    patch_name: &'static str,
    schema_version: i64,
    notes: &'static str,
}

const PATCHES: [PatchDefinition; 2] = [
    PatchDefinition {
        patch_id: "P001_foundation_base_structure",
        patch_name: "Foundation Base Structure",
        schema_version: 1,
        notes: "Initial desktop foundation with local storage, business/settings shell, backup/export foundation.",
    },
    PatchDefinition {
        patch_id: "P002_multi_business_workspace_settings_core",
        patch_name: "Multi-Business Workspace & Settings Core",
        schema_version: 2,
        notes: "Adds multi-business switching, business-scoped settings profiles, normalized module flags, receipt profiles, tax profiles, and sequence counters.",
    },
];

pub const PATCH_ID: &str = "P002_multi_business_workspace_settings_core";
pub const PATCH_NAME: &str = "Multi-Business Workspace & Settings Core";

pub fn register_patch(conn: &Connection) -> rusqlite::Result<()> {
    for patch in PATCHES {
        let now = Utc::now().to_rfc3339();
        let manifest = json!({
            "patch_id": patch.patch_id,
            "patch_name": patch.patch_name,
            "schema_version": patch.schema_version,
            "applied_at": now.clone(),
            "notes": patch.notes,
        });

        conn.execute(
            "INSERT OR IGNORE INTO patch_history (patch_id, patch_name, schema_version, applied_at, manifest_json)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                patch.patch_id,
                patch.patch_name,
                patch.schema_version,
                now,
                manifest.to_string()
            ],
        )?;
    }

    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO app_meta (key, value, updated_at)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
        params!["schema_version", CURRENT_SCHEMA_VERSION.to_string(), now.clone()],
    )?;

    conn.execute(
        "INSERT INTO app_meta (key, value, updated_at)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
        params!["patch_level", PATCH_ID, now],
    )?;

    Ok(())
}
