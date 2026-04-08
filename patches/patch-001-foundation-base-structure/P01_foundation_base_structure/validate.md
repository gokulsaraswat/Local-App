# Validation Steps — Patch 1

## Prerequisites
- Node.js 18+
- npm
- Rust stable toolchain
- Tauri OS prerequisites

## Apply the patch
```bash
node apply_patch.mjs /path/to/project
```

## Install dependencies
```bash
cd /path/to/project
npm install
```

## Run the app
```bash
npm run tauri dev
```

## Validate manually
1. The desktop app opens to a dashboard shell.
2. A demo business is visible.
3. Dashboard shows local storage locations and recent activity.
4. Sidebar navigation works between Dashboard, Business, Settings, and Data Center.
5. Business profile edits save and persist after restart.
6. Settings edits save and persist after restart.
7. Clicking **Create Backup Snapshot** creates a new backup record.
8. Clicking **Export Foundation Snapshot** creates a JSON export record.
9. Previewing an export bundle by path returns bundle metadata.
10. Reopening the app preserves the SQLite data locally.

## Database validation
Check these tables exist:
- `app_meta`
- `patch_history`
- `app_logs`
- `businesses`
- `business_settings`
- `backup_records`
- `export_jobs`
- `import_jobs`

## Patch history validation
Confirm `patch_history` contains:
- `P001_foundation_base_structure`
