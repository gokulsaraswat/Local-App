use rusqlite::Connection;

pub const CURRENT_SCHEMA_VERSION: i64 = 1;

const MIGRATION_001: &str = include_str!("migrations/001_base.sql");

pub fn run(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(MIGRATION_001)
}
