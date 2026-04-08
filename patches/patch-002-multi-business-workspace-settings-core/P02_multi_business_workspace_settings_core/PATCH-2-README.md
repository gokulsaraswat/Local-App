# Patch 2 - Multi-Business Workspace & Settings Core

Patch ID: `P002_multi_business_workspace_settings_core`

## Purpose
Patch 2 upgrades the Patch 1 foundation into a true multi-business local workspace.

## What this patch adds
- create multiple business workspaces locally
- switch the active business safely
- archive businesses without deleting their records
- isolate settings per business
- add a default tax profile per business
- add a default receipt profile per business
- add sequence counters per business for future document numbering
- expand dashboard messaging to reflect workspace readiness
- expand export bundles to include workspace-scoped metadata

## What this patch does not add yet
- full product catalog
- POS billing flows
- inventory ledger flows
- reporting engine
- full import apply flow
- local users / roles

## Architecture impact
This patch keeps the Patch 1 shell intact and extends it in a patch-friendly way:
- UI stays modular by page/module
- Rust commands remain grouped by business, settings, bootstrap, and data-center concerns
- persistence remains SQLite with additive schema changes only
- workspace support records are backfilled at startup for older Patch 1 data

## New schema elements
Patch 2 introduces:
- `tax_profiles`
- `receipt_profiles`
- `sequence_counters`

## Source files in this patch
See `patch-manifest.json` for the exact list.

## Apply
### macOS / Linux
```bash
./apply_patch.sh /path/to/patch1-project
```

### Windows PowerShell
```powershell
./apply_patch.ps1 -TargetDir C:\path\to\patch1-project
```

### Node directly
```bash
node apply_patch.mjs /path/to/patch1-project --install
```

## Migration behavior
On the first app start after applying Patch 2:
- migration SQL creates new Patch 2 tables
- patch history is updated to Patch 2
- any existing businesses receive default tax/receipt/counter rows if missing
- the current active business is preserved where possible

## Build and run
```bash
cd /path/to/patch1-project
npm install
npm run validate:patch2
npm run tauri dev
```

## Recommended manual test flow
1. Launch the app.
2. Confirm the dashboard shows Patch 2 workspace messaging.
3. Open the Businesses page.
4. Create a second business.
5. Switch between businesses.
6. Open Settings and verify each business has its own defaults.
7. Update tax profile, receipt profile, and numbering for one business.
8. Switch back and verify the other business retains its own values.
9. Export a workspace snapshot.

## Rollback
- restore the source tree from version control, or
- restore files from `.patch-backups/P002_multi_business_workspace_settings_core/`, and
- restore the SQLite file from a backup snapshot if runtime data also needs rollback

## Notes for future patches
Patch 2 intentionally prepares the app for:
- item/catalog master data
- stock and purchasing modules
- POS numbering and receipts
- per-business reports
- business-scoped users/permissions later
