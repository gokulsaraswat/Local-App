# Patch 1 — Foundation Base Structure

## Patch name
P01 — Foundation Base Structure

## Goal
Create the initial local-first desktop application foundation for a business management app that can grow safely patch by patch.

## What this patch adds
- A **Tauri + React + TypeScript** desktop scaffold.
- A **SQLite-backed local storage foundation** managed from the Rust backend.
- Core **domain models** for app metadata, patch history, business profile, business settings, logs, backups, and import/export jobs.
- A **business profile foundation** with editable fields.
- A **dashboard shell** with system KPIs, recent local activity, and module placeholders for future patches.
- An **app navigation shell** with Dashboard, Business, Settings, and Data Center views.
- A **settings foundation** for locale, tax label, receipt footer, theme preference, module toggles, and backup settings.
- **Seed/demo data** on first run.
- **Backup foundation** that creates a local SQLite snapshot copy.
- **Export foundation** that writes a JSON snapshot package.
- **Import foundation interface** with bundle preview/validation only (no full import apply yet).
- A **patch-friendly architecture** with clear frontend/backend module boundaries and patch history registration.

## What this patch intentionally does not add
This patch does **not** yet implement full:
- POS checkout
- inventory movement workflows
- reporting engine
- customer/supplier modules
- purchasing
- returns/refunds
- advanced restaurant/retail workflows

Those are intentionally deferred to later patches.

## Tech stack used in this patch
- Desktop shell: **Tauri 2**
- Frontend: **React + TypeScript + Vite**
- Backend: **Rust**
- Local database: **SQLite** via `rusqlite`

## Project structure added by this patch
```text
.
├── index.html
├── package.json
├── tsconfig.json
├── tsconfig.node.json
├── vite.config.ts
├── src
│   ├── App.tsx
│   ├── main.tsx
│   ├── styles.css
│   ├── app
│   │   └── AppProvider.tsx
│   ├── modules
│   │   ├── business
│   │   │   └── BusinessPage.tsx
│   │   ├── dashboard
│   │   │   └── DashboardPage.tsx
│   │   ├── data-center
│   │   │   └── DataCenterPage.tsx
│   │   ├── settings
│   │   │   └── SettingsPage.tsx
│   │   └── shell
│   │       └── AppShell.tsx
│   └── shared
│       ├── api.ts
│       ├── types.ts
│       └── utils.ts
└── src-tauri
    ├── Cargo.toml
    ├── build.rs
    ├── tauri.conf.json
    ├── capabilities
    │   └── default.json
    └── src
        ├── lib.rs
        ├── main.rs
        ├── commands
        │   ├── bootstrap.rs
        │   ├── business.rs
        │   ├── data_center.rs
        │   ├── mod.rs
        │   └── settings.rs
        ├── core
        │   ├── db.rs
        │   ├── error.rs
        │   ├── migrations
        │   │   └── 001_base.sql
        │   ├── migrations.rs
        │   ├── mod.rs
        │   ├── patching.rs
        │   ├── paths.rs
        │   └── seed.rs
        └── domain
            ├── bootstrap.rs
            ├── mod.rs
            └── models.rs
```

## Database foundation created
This patch initializes these SQLite tables:

- `app_meta`
- `patch_history`
- `app_logs`
- `businesses`
- `business_settings`
- `backup_records`
- `export_jobs`
- `import_jobs`

## Seed data behavior
On first startup:
- a demo business is created
- default settings are created
- patch history is registered
- a small local activity log is seeded

## Backup foundation behavior
The **Create Backup Snapshot** action:
- copies the SQLite database to the app backup directory
- records metadata in `backup_records`
- computes a SHA-256 checksum for the copied file

## Export foundation behavior
The **Export Foundation Snapshot** action:
- writes a JSON export package to the app export directory
- includes manifest/app metadata/business/settings/patch history/backups
- creates an `export_jobs` record

## Import foundation behavior
The **Preview Import Bundle** action:
- reads a JSON export bundle from a user-provided path
- validates basic structure
- shows manifest details
- creates an `import_jobs` preview record

This patch does **not** apply imports yet.

## Safe apply assumptions
This patch is designed to apply onto:
- an empty directory, or
- a new repository root for this project

If files already exist, the patch applier:
- stops on conflicts by default
- supports `--force`
- stores overwritten files in a local patch backup folder when forced

## One-click apply
### macOS / Linux
```bash
./apply_patch.sh /path/to/project
```

### Windows PowerShell
```powershell
./apply_patch.ps1 -TargetPath C:\path\to\project
```

### Cross-platform Node
```bash
node apply_patch.mjs /path/to/project
```

## Initial setup after applying the patch
```bash
cd /path/to/project
npm install
npm run tauri dev
```

## Suggested next patch targets
Good Patch 2 candidates:
- multi-business switching
- product catalog base
- stronger settings segmentation
- sequence counters and business isolation rules

## Rollback
Because this is Patch 1 on a new base project, rollback is simple:
1. delete the applied project files, or restore from `.patch-backups/P001_foundation_base_structure` if `--force` was used
2. remove the local Tauri app data directory if you want to clear demo data and SQLite files
3. re-apply a corrected patch if needed
