# PATCH-3-README

## Patch
- Patch ID: `P003_catalog_core`
- Patch name: `Catalog Core`
- Base required: Patch 2 project (`P002_multi_business_workspace_settings_core`)
- Target version after apply: `0.3.0`

## Goal
Patch 3 adds the first real catalog foundation for the local-first desktop app.

It introduces business-scoped:
- item categories
- units of measure
- products / menu items / service items
- barcode foundations
- catalog summary KPIs
- demo catalog seed data
- catalog-aware export bundle support

This patch also normalizes a few Patch 2 schema/model edges so the catalog layer sits on top of a more consistent local data foundation.

## What this patch adds
- `catalog_categories` table
- `catalog_units` table
- `catalog_items` table
- `catalog_item_barcodes` table
- migration 003 with compatibility columns for tax / receipt / sequence tables
- Rust backend catalog core (`core/catalog.rs`)
- Tauri catalog commands
- React catalog page and navigation entry
- catalog counts in bootstrap/dashboard/shell
- export bundle now includes catalog categories, units, items, and barcodes
- import preview now shows category and item counts
- demo seed catalog for the demo business

## What this patch does not add yet
- full POS checkout
- stock movement ledger
- purchase workflow
- advanced variants/modifiers
- recipe/BOM deduction
- report engine

## Added files
- `scripts/validate-patch3.mjs`
- `src-tauri/src/commands/catalog.rs`
- `src-tauri/src/core/catalog.rs`
- `src-tauri/src/core/migrations/003_catalog_core.sql`
- `src/modules/catalog/CatalogPage.tsx`

## Updated files
- `package.json`
- `src-tauri/Cargo.toml`
- `src-tauri/src/commands/bootstrap.rs`
- `src-tauri/src/commands/data_center.rs`
- `src-tauri/src/commands/mod.rs`
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
- `src/modules/business/BusinessPage.tsx`
- `src/modules/dashboard/DashboardPage.tsx`
- `src/modules/data-center/DataCenterPage.tsx`
- `src/modules/settings/SettingsPage.tsx`
- `src/modules/shell/AppShell.tsx`
- `src/shared/api.ts`
- `src/shared/types.ts`
- `src/shared/utils.ts`
- `src/styles.css`

## Migration behavior
This patch introduces schema version `3`.

Migration 003 does two things:
1. adds compatibility columns required by the refined workspace models
2. adds catalog tables and indexes

Compatibility columns added:
- `tax_profiles.prices_include_tax`
- `receipt_profiles.show_email`
- `receipt_profiles.show_business_code`
- `sequence_counters.reset_policy`

Catalog tables added:
- `catalog_categories`
- `catalog_units`
- `catalog_items`
- `catalog_item_barcodes`

## Apply steps
### macOS / Linux
```bash
./apply_patch.sh /path/to/your/project
```

### Windows PowerShell
```powershell
./apply_patch.ps1 -TargetDir C:\path\to\your\project
```

### Direct Node
```bash
node apply_patch.mjs /path/to/your/project --install
```

## Build / run
From the patched project root:

```bash
npm install
npm run validate:patch3
npm run tauri dev
```

## Validation checklist
- patch applies on top of Patch 2 without manual file moves
- `npm run validate:patch3` passes
- app starts and shows Patch 3 shell text
- Catalog page appears in navigation
- active business catalog workspace loads
- demo workspace shows seed categories, units, and items
- item/category/unit save actions work locally
- archive / restore item action updates the item list
- export bundle contains catalog arrays
- import preview shows business/category/item counts

## Rollback
- restore overwritten files from `.patch-backups/P003_catalog_core/`
- restore database from a pre-patch backup if you need to undo schema version 3
- if you already opened the app and migration 003 ran, do not downgrade the database in place without restoring a prior snapshot

## Notes for future patches
Patch 3 prepares the data layer for:
- inventory ledger (stock movements against catalog items)
- POS sales lines referencing catalog items
- supplier purchasing against catalog stock items
- restaurant menu workflows using menu-kind items
