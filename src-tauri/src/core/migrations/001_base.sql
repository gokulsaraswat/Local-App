CREATE TABLE IF NOT EXISTS app_meta (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS patch_history (
  patch_id TEXT PRIMARY KEY,
  patch_name TEXT NOT NULL,
  schema_version INTEGER NOT NULL,
  applied_at TEXT NOT NULL,
  manifest_json TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS app_logs (
  id TEXT PRIMARY KEY,
  level TEXT NOT NULL,
  category TEXT NOT NULL,
  message TEXT NOT NULL,
  payload_json TEXT,
  created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS businesses (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  legal_name TEXT,
  code TEXT NOT NULL UNIQUE,
  business_type TEXT NOT NULL,
  currency_code TEXT NOT NULL,
  tax_mode TEXT NOT NULL,
  phone TEXT,
  email TEXT,
  address_line1 TEXT,
  address_line2 TEXT,
  city TEXT,
  state TEXT,
  postal_code TEXT,
  country TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  archived_at TEXT
);

CREATE TABLE IF NOT EXISTS business_settings (
  business_id TEXT PRIMARY KEY,
  timezone TEXT NOT NULL,
  locale TEXT NOT NULL,
  date_format TEXT NOT NULL,
  theme TEXT NOT NULL,
  tax_label TEXT NOT NULL,
  default_tax_rate REAL NOT NULL DEFAULT 0,
  prices_include_tax INTEGER NOT NULL DEFAULT 0,
  receipt_footer TEXT,
  receipt_show_address INTEGER NOT NULL DEFAULT 1,
  receipt_show_phone INTEGER NOT NULL DEFAULT 1,
  auto_backup_enabled INTEGER NOT NULL DEFAULT 0,
  backup_directory TEXT,
  module_restaurant_enabled INTEGER NOT NULL DEFAULT 0,
  module_retail_enabled INTEGER NOT NULL DEFAULT 1,
  module_inventory_enabled INTEGER NOT NULL DEFAULT 1,
  module_services_enabled INTEGER NOT NULL DEFAULT 0,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS backup_records (
  id TEXT PRIMARY KEY,
  business_id TEXT,
  file_name TEXT NOT NULL,
  file_path TEXT NOT NULL,
  checksum TEXT,
  status TEXT NOT NULL,
  created_at TEXT NOT NULL,
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS export_jobs (
  id TEXT PRIMARY KEY,
  business_id TEXT,
  format TEXT NOT NULL,
  status TEXT NOT NULL,
  target_path TEXT,
  created_at TEXT NOT NULL,
  completed_at TEXT,
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS import_jobs (
  id TEXT PRIMARY KEY,
  business_id TEXT,
  format TEXT NOT NULL,
  status TEXT NOT NULL,
  source_path TEXT,
  created_at TEXT NOT NULL,
  completed_at TEXT,
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_businesses_code ON businesses(code);
CREATE INDEX IF NOT EXISTS idx_app_logs_created_at ON app_logs(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_backup_records_created_at ON backup_records(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_export_jobs_created_at ON export_jobs(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_import_jobs_created_at ON import_jobs(created_at DESC);
