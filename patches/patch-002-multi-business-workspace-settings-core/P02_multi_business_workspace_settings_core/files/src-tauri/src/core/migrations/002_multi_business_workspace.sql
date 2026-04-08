CREATE TABLE IF NOT EXISTS tax_profiles (
  id TEXT PRIMARY KEY,
  business_id TEXT NOT NULL,
  name TEXT NOT NULL,
  label TEXT NOT NULL,
  rate REAL NOT NULL DEFAULT 0,
  is_default INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS receipt_profiles (
  id TEXT PRIMARY KEY,
  business_id TEXT NOT NULL,
  name TEXT NOT NULL,
  header_line1 TEXT,
  header_line2 TEXT,
  footer_text TEXT,
  show_address INTEGER NOT NULL DEFAULT 1,
  show_phone INTEGER NOT NULL DEFAULT 1,
  show_tax_breakdown INTEGER NOT NULL DEFAULT 1,
  paper_width TEXT NOT NULL DEFAULT '80mm',
  copies INTEGER NOT NULL DEFAULT 1,
  is_default INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS module_flags (
  business_id TEXT NOT NULL,
  module_key TEXT NOT NULL,
  enabled INTEGER NOT NULL DEFAULT 0,
  updated_at TEXT NOT NULL,
  PRIMARY KEY (business_id, module_key),
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sequence_counters (
  business_id TEXT NOT NULL,
  counter_key TEXT NOT NULL,
  prefix TEXT NOT NULL,
  next_number INTEGER NOT NULL DEFAULT 1,
  padding INTEGER NOT NULL DEFAULT 4,
  updated_at TEXT NOT NULL,
  PRIMARY KEY (business_id, counter_key),
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_tax_profiles_business_default
  ON tax_profiles(business_id, is_default DESC, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_receipt_profiles_business_default
  ON receipt_profiles(business_id, is_default DESC, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_module_flags_business
  ON module_flags(business_id, module_key);

CREATE INDEX IF NOT EXISTS idx_sequence_counters_business
  ON sequence_counters(business_id, counter_key);
