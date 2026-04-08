CREATE TABLE IF NOT EXISTS inventory_stock_movements (
  id TEXT PRIMARY KEY,
  business_id TEXT NOT NULL,
  item_id TEXT NOT NULL,
  movement_type TEXT NOT NULL,
  quantity_delta REAL NOT NULL,
  quantity_after REAL NOT NULL,
  unit_cost REAL,
  note TEXT,
  occurred_at TEXT NOT NULL,
  created_at TEXT NOT NULL,
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE CASCADE,
  FOREIGN KEY (item_id) REFERENCES catalog_items(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_inventory_stock_movements_business_time
  ON inventory_stock_movements (business_id, occurred_at DESC, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_inventory_stock_movements_item_time
  ON inventory_stock_movements (item_id, occurred_at DESC, created_at DESC);

INSERT OR IGNORE INTO inventory_stock_movements (
  id,
  business_id,
  item_id,
  movement_type,
  quantity_delta,
  quantity_after,
  unit_cost,
  note,
  occurred_at,
  created_at
)
SELECT
  'opening-balance:' || id,
  business_id,
  id,
  'opening_balance',
  stock_quantity,
  stock_quantity,
  NULLIF(cost_price, 0),
  'Opening balance from existing stock quantity',
  updated_at,
  updated_at
FROM catalog_items
WHERE archived_at IS NULL
  AND item_kind = 'stock'
  AND track_stock = 1
  AND stock_quantity > 0;
