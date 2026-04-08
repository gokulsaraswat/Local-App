# PATCH_NOTES

## Patch 3 summary
Catalog Core adds the first reusable item master layer for the application.

### Functional additions
- business-scoped catalog categories
- system and custom units of measure
- stock / menu / service item records
- multi-barcode foundation per item
- catalog workspace loader
- basic archive / restore behavior for items
- catalog summary KPIs for the dashboard and shell
- catalog data included in export preview scope

### Structural changes
- schema version moved from `2` to `3`
- patch registry now records `P003_catalog_core`
- project/package version updated to `0.3.0`
- frontend shell adds a Catalog navigation page
- seed flow creates system units and a demo catalog for the demo business

### Compatibility fixes bundled into this patch
To keep Patch 3 safe on top of Patch 2, this patch also aligns a few data-model edges:
- tax profiles now carry `prices_include_tax`
- receipt profiles now carry `show_email` and `show_business_code`
- sequence counters now carry `reset_policy`

### Files added
- scripts/validate-patch3.mjs
- src-tauri/src/commands/catalog.rs
- src-tauri/src/core/catalog.rs
- src-tauri/src/core/migrations/003_catalog_core.sql
- src/modules/catalog/CatalogPage.tsx

### Files updated
- package.json
- src-tauri/Cargo.toml
- src-tauri/src/commands/bootstrap.rs
- src-tauri/src/commands/data_center.rs
- src-tauri/src/commands/mod.rs
- src-tauri/src/core/db.rs
- src-tauri/src/core/migrations.rs
- src-tauri/src/core/mod.rs
- src-tauri/src/core/patching.rs
- src-tauri/src/core/seed.rs
- src-tauri/src/domain/bootstrap.rs
- src-tauri/src/domain/models.rs
- src-tauri/src/lib.rs
- src-tauri/tauri.conf.json
- src/app/AppProvider.tsx
- src/modules/business/BusinessPage.tsx
- src/modules/dashboard/DashboardPage.tsx
- src/modules/data-center/DataCenterPage.tsx
- src/modules/settings/SettingsPage.tsx
- src/modules/shell/AppShell.tsx
- src/shared/api.ts
- src/shared/types.ts
- src/shared/utils.ts
- src/styles.css
