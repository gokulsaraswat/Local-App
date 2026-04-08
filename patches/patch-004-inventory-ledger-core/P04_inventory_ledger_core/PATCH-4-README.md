# Patch 4 - Inventory Ledger Core

Patch ID: `P004_inventory_ledger_core`

## Goal

Add the first real local inventory layer on top of Patch 3.

This patch introduces a movement-ledger foundation for stock items while keeping the app fully local-first and offline-capable.

## What this patch adds

- local inventory ledger table and migration
- inventory workspace page in the desktop UI
- stock movement recording for stock in / stock out / adjustments
- per-item reorder level and stock-tracking rules
- inventory summary in app bootstrap and dashboard
- opening-balance backfill for existing Patch 3 stock quantities
- inventory movements included in export bundles and import preview counts
- catalog-to-inventory sync when stock quantity is edited from the catalog page

## What this patch does not add

- POS checkout
- sales deductions from stock
- purchase entries
- supplier-linked receiving
- batch / expiry logic
- reports beyond the updated dashboard and export preview

## Data model changes

### New table

`inventory_stock_movements`

Columns:
- `id`
- `business_id`
- `item_id`
- `movement_type`
- `quantity_delta`
- `quantity_after`
- `unit_cost`
- `note`
- `occurred_at`
- `created_at`

### Backfill behavior

During migration and startup backfill, existing tracked stock items with non-zero on-hand quantity receive an `opening_balance` movement row so Patch 3 stock quantities gain inventory history safely.

### API / bootstrap shape updates

- `AppBootstrap.inventorySummary`
- `ImportPreview.movementCount`
- new inventory commands:
  - `load_inventory_workspace`
  - `record_inventory_movement`
  - `save_inventory_stock_rule`

## Files added

- `scripts/validate-patch4.mjs`
- `src-tauri/src/commands/inventory.rs`
- `src-tauri/src/core/inventory.rs`
- `src-tauri/src/core/migrations/004_inventory_ledger_core.sql`
- `src/modules/inventory/InventoryPage.tsx`

## Files updated

- `package.json`
- `src-tauri/Cargo.toml`
- `src-tauri/src/commands/bootstrap.rs`
- `src-tauri/src/commands/data_center.rs`
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/core/catalog.rs`
- `src-tauri/src/core/db.rs`
- `src-tauri/src/core/migrations.rs`
- `src-tauri/src/core/mod.rs`
- `src-tauri/src/core/patching.rs`
- `src-tauri/src/core/seed.rs`
- `src-tauri/src/domain/bootstrap.rs`
- `src-tauri/src/domain/models.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/tauri.conf.json`
- `src/app/AppProvider.tsx`
- `src/modules/catalog/CatalogPage.tsx`
- `src/modules/dashboard/DashboardPage.tsx`
- `src/modules/data-center/DataCenterPage.tsx`
- `src/modules/shell/AppShell.tsx`
- `src/shared/api.ts`
- `src/shared/types.ts`
- `src/shared/utils.ts`
- `src/styles.css`

## Apply instructions

Apply this patch on top of a working Patch 3 project.

### macOS / Linux

```bash
./apply_patch.sh /path/to/patch3-project
```

### Windows PowerShell

```powershell
./apply_patch.ps1 -TargetDir C:\path	o\patch3-project
```

### Direct Node

```bash
node apply_patch.mjs /path/to/patch3-project --install
```

## Post-apply steps

```bash
npm install
npm run validate:patch4
npm run tauri dev
```

## Validation notes

This patch bundle was structurally validated before packaging:

- `node --check apply_patch.mjs` passed
- `node scripts/validate-patch4.mjs` passed
- SQLite migration smoke test for `001 + 002 + 003 + 004` passed
- migration backfill test inserted an `opening_balance` row for an existing stock item
- `rustfmt --check` passed on the modified Rust files

A full `cargo check` could not be completed in this container because Rust crates are not cached offline here.

## Rollback notes

- restore the files backed up under `.patch-backups/P004_inventory_ledger_core/`
- restore the SQLite database from a pre-patch backup if migration 004 has already been executed in a live workspace
- remove Patch 4 metadata entries from `.patch-meta/` only if you are performing a manual rollback and understand the consequences

## Known limitations

- manual inventory movement entry only; no auto-deduction from POS yet
- import remains preview-only
- stock ledger currently focuses on stock items and not recipe/BOM consumption yet
