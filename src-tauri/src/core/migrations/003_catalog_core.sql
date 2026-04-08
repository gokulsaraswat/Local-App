ALTER TABLE tax_profiles
  ADD COLUMN prices_include_tax INTEGER NOT NULL DEFAULT 0;

ALTER TABLE receipt_profiles
  ADD COLUMN show_email INTEGER NOT NULL DEFAULT 0;

ALTER TABLE receipt_profiles
  ADD COLUMN show_business_code INTEGER NOT NULL DEFAULT 1;

ALTER TABLE sequence_counters
  ADD COLUMN reset_policy TEXT NOT NULL DEFAULT 'none';

UPDATE tax_profiles
SET prices_include_tax = COALESCE(
  (
    SELECT prices_include_tax
    FROM business_settings
    WHERE business_settings.business_id = tax_profiles.business_id
    LIMIT 1
  ),
  0
);

UPDATE receipt_profiles
SET show_email = COALESCE(show_email, 0),
    show_business_code = COALESCE(show_business_code, 1);

UPDATE sequence_counters
SET reset_policy = COALESCE(reset_policy, 'none');

CREATE TABLE IF NOT EXISTS catalog_categories (
  id TEXT PRIMARY KEY,
  business_id TEXT NOT NULL,
  name TEXT NOT NULL,
  code TEXT NOT NULL,
  parent_id TEXT,
  item_scope TEXT NOT NULL DEFAULT 'all',
  sort_order INTEGER NOT NULL DEFAULT 0,
  notes TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  archived_at TEXT,
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE CASCADE,
  FOREIGN KEY (parent_id) REFERENCES catalog_categories(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS catalog_units (
  id TEXT PRIMARY KEY,
  business_id TEXT,
  name TEXT NOT NULL,
  code TEXT NOT NULL,
  symbol TEXT NOT NULL,
  allow_fractional INTEGER NOT NULL DEFAULT 0,
  is_system INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  archived_at TEXT,
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS catalog_items (
  id TEXT PRIMARY KEY,
  business_id TEXT NOT NULL,
  category_id TEXT,
  unit_id TEXT,
  tax_profile_id TEXT,
  item_kind TEXT NOT NULL,
  name TEXT NOT NULL,
  display_name TEXT,
  sku TEXT,
  primary_barcode TEXT,
  description TEXT,
  selling_price REAL NOT NULL DEFAULT 0,
  cost_price REAL NOT NULL DEFAULT 0,
  track_stock INTEGER NOT NULL DEFAULT 1,
  stock_quantity REAL NOT NULL DEFAULT 0,
  reorder_level REAL NOT NULL DEFAULT 0,
  image_path TEXT,
  is_active INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  archived_at TEXT,
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE CASCADE,
  FOREIGN KEY (category_id) REFERENCES catalog_categories(id) ON DELETE SET NULL,
  FOREIGN KEY (unit_id) REFERENCES catalog_units(id) ON DELETE SET NULL,
  FOREIGN KEY (tax_profile_id) REFERENCES tax_profiles(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS catalog_item_barcodes (
  id TEXT PRIMARY KEY,
  item_id TEXT NOT NULL,
  barcode TEXT NOT NULL,
  label TEXT,
  is_primary INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  FOREIGN KEY (item_id) REFERENCES catalog_items(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_catalog_categories_business_code
  ON catalog_categories (business_id, code);

CREATE UNIQUE INDEX IF NOT EXISTS idx_catalog_units_system_code
  ON catalog_units (code)
  WHERE business_id IS NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_catalog_units_business_code
  ON catalog_units (business_id, code)
  WHERE business_id IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_catalog_items_business_sku
  ON catalog_items (business_id, sku)
  WHERE sku IS NOT NULL AND sku != '';

CREATE UNIQUE INDEX IF NOT EXISTS idx_catalog_items_business_primary_barcode
  ON catalog_items (business_id, primary_barcode)
  WHERE primary_barcode IS NOT NULL AND primary_barcode != '';

CREATE INDEX IF NOT EXISTS idx_catalog_categories_business
  ON catalog_categories (business_id, sort_order, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_catalog_units_business
  ON catalog_units (business_id, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_catalog_items_business
  ON catalog_items (business_id, item_kind, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_catalog_items_category
  ON catalog_items (category_id, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_catalog_item_barcodes_item
  ON catalog_item_barcodes (item_id, is_primary DESC);

CREATE INDEX IF NOT EXISTS idx_catalog_item_barcodes_barcode
  ON catalog_item_barcodes (barcode);
