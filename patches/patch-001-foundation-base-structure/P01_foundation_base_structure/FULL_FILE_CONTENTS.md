# PATCH-1-README.md

```md
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

```

# PATCH_NOTES.md

```md
# Patch Notes — P01 Foundation Base Structure

## Added
- Tauri desktop shell
- React frontend shell
- SQLite initialization and migrations
- Patch history registration
- Business profile management
- Settings management
- Dashboard shell
- Data Center shell
- Backup snapshot command
- Export foundation snapshot command
- Import preview command
- Demo seed data

## Introduced module boundaries
### Frontend
- `src/app` for runtime state/bootstrap
- `src/modules/*` for feature shells
- `src/shared/*` for types, API bridge, shared helpers

### Backend
- `src-tauri/src/commands` for Tauri command surface
- `src-tauri/src/core` for DB/migrations/paths/patching/bootstrap internals
- `src-tauri/src/domain` for serializable business-facing models and dashboard composition

## Future extension points intentionally left open
- product catalog module
- inventory ledger module
- POS transaction module
- reporting/query module
- import/apply migrations
- backup scheduling
- multi-user access layer
- restaurant/retail workflow packs

## Compatibility
- Base project only
- No prior patch required

```

# README.md

```md
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

```

# apply_patch.mjs

```js
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const manifestPath = path.join(__dirname, "patch-manifest.json");
const filesDir = path.join(__dirname, "files");
const manifest = JSON.parse(fs.readFileSync(manifestPath, "utf8"));

const args = process.argv.slice(2);
const force = args.includes("--force");
const targetArg = args.find((arg) => !arg.startsWith("--"));
const targetPath = path.resolve(targetArg || process.cwd());
const patchBackupRoot = path.join(targetPath, ".patch-backups", manifest.patch_id);
const patchMetaRoot = path.join(targetPath, ".patch-meta");

function listFilesRecursively(dir, prefix = "") {
  const results = [];
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const entryRel = path.join(prefix, entry.name);
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      results.push(...listFilesRecursively(full, entryRel));
    } else {
      results.push(entryRel);
    }
  }
  return results;
}

function ensureDir(dir) {
  fs.mkdirSync(dir, { recursive: true });
}

function readFileOrNull(filePath) {
  if (!fs.existsSync(filePath)) return null;
  return fs.readFileSync(filePath);
}

function sameFile(a, b) {
  if (a === null || b === null) return false;
  return Buffer.compare(a, b) === 0;
}

function copyFileWithBackup(relativePath) {
  const source = path.join(filesDir, relativePath);
  const target = path.join(targetPath, relativePath);
  const targetDir = path.dirname(target);
  ensureDir(targetDir);

  const incoming = fs.readFileSync(source);
  const existing = readFileOrNull(target);

  if (existing !== null && !sameFile(existing, incoming)) {
    if (!force) {
      throw new Error(
        `Conflict detected for ${relativePath}. Re-run with --force to back up and overwrite existing files.`
      );
    }

    const backupPath = path.join(patchBackupRoot, relativePath);
    ensureDir(path.dirname(backupPath));
    fs.writeFileSync(backupPath, existing);
  }

  fs.writeFileSync(target, incoming);
}

function main() {
  if (!fs.existsSync(filesDir)) {
    throw new Error(`Patch bundle is missing files directory: ${filesDir}`);
  }

  ensureDir(targetPath);
  ensureDir(patchMetaRoot);

  const fileList = listFilesRecursively(filesDir);
  for (const relativePath of fileList) {
    copyFileWithBackup(relativePath);
  }

  const applyRecord = {
    patchId: manifest.patch_id,
    patchName: manifest.patch_name,
    targetPath,
    appliedAt: new Date().toISOString(),
    forced: force,
    fileCount: fileList.length
  };

  fs.writeFileSync(
    path.join(patchMetaRoot, "last-applied.json"),
    JSON.stringify(applyRecord, null, 2) + "\n"
  );

  console.log(`[OK] Applied ${manifest.patch_id} to ${targetPath}`);
  console.log(`[INFO] Files copied: ${fileList.length}`);
  if (force) {
    console.log(`[INFO] Backups stored in ${patchBackupRoot}`);
  }
}

try {
  main();
} catch (error) {
  console.error(`[ERROR] ${error.message}`);
  process.exit(1);
}

```

# apply_patch.ps1

```powershell
param(
  [string]$TargetPath = (Get-Location).Path,
  [switch]$Force
)

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$nodeArgs = @("$scriptDir/apply_patch.mjs", $TargetPath)

if ($Force) {
  $nodeArgs += "--force"
}

node @nodeArgs
if ($LASTEXITCODE -ne 0) {
  exit $LASTEXITCODE
}

```

# apply_patch.sh

```bash
#!/usr/bin/env sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
TARGET_DIR="${1:-$PWD}"

node "$SCRIPT_DIR/apply_patch.mjs" "$TARGET_DIR"

```

# files/.editorconfig

```
root = true

[*]
charset = utf-8
end_of_line = lf
insert_final_newline = true
indent_style = space
indent_size = 2
trim_trailing_whitespace = true

[*.rs]
indent_size = 4

```

# files/.gitignore

```
node_modules/
dist/
.vite/
npm-debug.log*
yarn-debug.log*
yarn-error.log*
pnpm-debug.log*

src-tauri/target/
src-tauri/gen/
src-tauri/.cargo/
*.db
*.sqlite
*.sqlite3
*.log

.patch-backups/
.patch-meta/
.DS_Store

```

# files/index.html

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta
      name="viewport"
      content="width=device-width, initial-scale=1.0"
    />
    <title>Local Business Manager</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>

```

# files/package.json

```json
{
  "name": "local-first-business-manager",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "tauri": "tauri",
    "typecheck": "tsc --noEmit"
  },
  "dependencies": {
    "@tauri-apps/api": "^2.0.0",
    "react": "^18.3.1",
    "react-dom": "^18.3.1"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2.0.0",
    "@types/react": "^18.3.7",
    "@types/react-dom": "^18.3.3",
    "typescript": "^5.6.3",
    "vite": "^5.4.10",
    "@vitejs/plugin-react": "^4.3.3"
  }
}

```

# files/src/App.tsx

```tsx
import { AppProvider, AppStateView } from "./app/AppProvider";
import { AppShell } from "./modules/shell/AppShell";

function App() {
  return (
    <AppProvider>
      <AppStateView>
        <AppShell />
      </AppStateView>
    </AppProvider>
  );
}

export default App;

```

# files/src/app/AppProvider.tsx

```tsx
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
  type PropsWithChildren
} from "react";
import {
  bootstrapApp,
  createBackupSnapshot,
  exportFoundationSnapshot,
  previewImportBundle,
  saveBusinessProfile,
  saveBusinessSettings
} from "../shared/api";
import type {
  AppBootstrap,
  BackupRecord,
  BusinessProfile,
  BusinessSettings,
  ImportPreview
} from "../shared/types";

type AppStatus = "loading" | "ready" | "error";

interface AppContextValue {
  status: AppStatus;
  errorMessage: string | null;
  data: AppBootstrap | null;
  refresh: () => Promise<void>;
  saveProfile: (profile: BusinessProfile) => Promise<BusinessProfile>;
  saveSettings: (settings: BusinessSettings) => Promise<BusinessSettings>;
  createBackup: () => Promise<BackupRecord>;
  exportFoundation: () => Promise<string>;
  previewImport: (filePath: string) => Promise<ImportPreview>;
}

type RefreshMode = "blocking" | "background";

const AppContext = createContext<AppContextValue | undefined>(undefined);

export function AppProvider({ children }: PropsWithChildren) {
  const [status, setStatus] = useState<AppStatus>("loading");
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [data, setData] = useState<AppBootstrap | null>(null);
  const hasLoadedOnceRef = useRef(false);

  const loadBootstrap = useCallback(async (mode: RefreshMode) => {
    if (mode === "blocking" || !hasLoadedOnceRef.current) {
      setStatus("loading");
    }
    setErrorMessage(null);
    try {
      const bootstrap = await bootstrapApp();
      setData(bootstrap);
      hasLoadedOnceRef.current = true;
      setStatus("ready");
    } catch (error) {
      const message =
        error instanceof Error ? error.message : "Failed to bootstrap app";
      setErrorMessage(message);
      setStatus("error");
    }
  }, []);

  const refresh = useCallback(async () => {
    await loadBootstrap("background");
  }, [loadBootstrap]);

  useEffect(() => {
    void loadBootstrap("blocking");
  }, [loadBootstrap]);

  const saveProfile = useCallback(
    async (profile: BusinessProfile) => {
      const saved = await saveBusinessProfile(profile);
      await refresh();
      return saved;
    },
    [refresh]
  );

  const saveSettingsAction = useCallback(
    async (settings: BusinessSettings) => {
      const saved = await saveBusinessSettings(settings);
      await refresh();
      return saved;
    },
    [refresh]
  );

  const createBackupAction = useCallback(async () => {
    const result = await createBackupSnapshot();
    await refresh();
    return result;
  }, [refresh]);

  const exportFoundationAction = useCallback(async () => {
    const result = await exportFoundationSnapshot();
    await refresh();
    return result;
  }, [refresh]);

  const previewImportAction = useCallback(async (filePath: string) => {
    return previewImportBundle(filePath);
  }, []);

  const value = useMemo<AppContextValue>(
    () => ({
      status,
      errorMessage,
      data,
      refresh,
      saveProfile,
      saveSettings: saveSettingsAction,
      createBackup: createBackupAction,
      exportFoundation: exportFoundationAction,
      previewImport: previewImportAction
    }),
    [
      status,
      errorMessage,
      data,
      refresh,
      saveProfile,
      saveSettingsAction,
      createBackupAction,
      exportFoundationAction,
      previewImportAction
    ]
  );

  return <AppContext.Provider value={value}>{children}</AppContext.Provider>;
}

export function useAppState(): AppContextValue {
  const value = useContext(AppContext);
  if (!value) {
    throw new Error("useAppState must be used inside AppProvider");
  }
  return value;
}

export function AppStateView({ children }: PropsWithChildren) {
  const { status, errorMessage, refresh } = useAppState();

  if (status === "loading") {
    return (
      <div className="app-loading-shell">
        <div className="spinner" />
        <h1>Preparing local workspace…</h1>
        <p>Initializing database, patch registry, and demo business profile.</p>
      </div>
    );
  }

  if (status === "error") {
    return (
      <div className="app-loading-shell">
        <h1>Failed to start the app</h1>
        <p>{errorMessage ?? "Unknown startup error"}</p>
        <button className="primary-button" onClick={() => void refresh()}>
          Retry startup
        </button>
      </div>
    );
  }

  return <>{children}</>;
}

```

# files/src/main.tsx

```tsx
import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

```

# files/src/modules/business/BusinessPage.tsx

```tsx
import { useEffect, useMemo, useState } from "react";
import { useAppState } from "../../app/AppProvider";
import type { BusinessProfile } from "../../shared/types";
import { formatDateTime } from "../../shared/utils";

export function BusinessPage() {
  const { data, saveProfile } = useAppState();
  const [form, setForm] = useState<BusinessProfile | null>(null);
  const [statusMessage, setStatusMessage] = useState<string>("");

  useEffect(() => {
    if (data?.activeBusiness) {
      setForm(data.activeBusiness);
    }
  }, [data?.activeBusiness]);

  const profileCompleteness = useMemo(() => {
    if (!form) return 0;
    const fields = [
      form.name,
      form.legalName,
      form.code,
      form.businessType,
      form.currencyCode,
      form.phone,
      form.email,
      form.addressLine1,
      form.city,
      form.state,
      form.country
    ];
    const filled = fields.filter((value) => value && String(value).trim()).length;
    return Math.round((filled / fields.length) * 100);
  }, [form]);

  if (!data || !form) return null;

  function update<K extends keyof BusinessProfile>(key: K, value: BusinessProfile[K]) {
    setForm((current) => (current ? { ...current, [key]: value } : current));
  }

  async function handleSave() {
    setStatusMessage("Saving business profile…");
    try {
      await saveProfile(form);
      setStatusMessage("Business profile saved locally.");
    } catch (error) {
      setStatusMessage(
        error instanceof Error ? error.message : "Failed to save profile."
      );
    }
  }

  return (
    <div className="page-grid">
      <section className="split-grid">
        <article className="card">
          <div className="card-header">
            <h2>Business profile foundation</h2>
            <span className="pill success">{profileCompleteness}% complete</span>
          </div>
          <p className="card-note">
            This is the base identity record future catalog, POS, inventory, and
            reporting modules will attach to.
          </p>

          <div className="form-grid">
            <label>
              <span>Business name</span>
              <input
                value={form.name}
                onChange={(event) => update("name", event.target.value)}
              />
            </label>

            <label>
              <span>Legal name</span>
              <input
                value={form.legalName ?? ""}
                onChange={(event) => update("legalName", event.target.value)}
              />
            </label>

            <label>
              <span>Business code</span>
              <input
                value={form.code}
                onChange={(event) => update("code", event.target.value)}
              />
            </label>

            <label>
              <span>Business type</span>
              <select
                value={form.businessType}
                onChange={(event) => update("businessType", event.target.value)}
              >
                <option value="Restaurant">Restaurant</option>
                <option value="Cafe">Cafe</option>
                <option value="Bakery">Bakery</option>
                <option value="Retail">Retail</option>
                <option value="Wholesale">Wholesale</option>
                <option value="Service">Service</option>
                <option value="General">General</option>
              </select>
            </label>

            <label>
              <span>Currency code</span>
              <input
                value={form.currencyCode}
                onChange={(event) => update("currencyCode", event.target.value.toUpperCase())}
              />
            </label>

            <label>
              <span>Tax mode</span>
              <select
                value={form.taxMode}
                onChange={(event) => update("taxMode", event.target.value)}
              >
                <option value="exclusive">Exclusive</option>
                <option value="inclusive">Inclusive</option>
                <option value="none">No tax</option>
              </select>
            </label>

            <label>
              <span>Phone</span>
              <input
                value={form.phone ?? ""}
                onChange={(event) => update("phone", event.target.value)}
              />
            </label>

            <label>
              <span>Email</span>
              <input
                value={form.email ?? ""}
                onChange={(event) => update("email", event.target.value)}
              />
            </label>

            <label className="form-span-2">
              <span>Address line 1</span>
              <input
                value={form.addressLine1 ?? ""}
                onChange={(event) => update("addressLine1", event.target.value)}
              />
            </label>

            <label className="form-span-2">
              <span>Address line 2</span>
              <input
                value={form.addressLine2 ?? ""}
                onChange={(event) => update("addressLine2", event.target.value)}
              />
            </label>

            <label>
              <span>City</span>
              <input
                value={form.city ?? ""}
                onChange={(event) => update("city", event.target.value)}
              />
            </label>

            <label>
              <span>State</span>
              <input
                value={form.state ?? ""}
                onChange={(event) => update("state", event.target.value)}
              />
            </label>

            <label>
              <span>Postal code</span>
              <input
                value={form.postalCode ?? ""}
                onChange={(event) => update("postalCode", event.target.value)}
              />
            </label>

            <label>
              <span>Country</span>
              <input
                value={form.country ?? ""}
                onChange={(event) => update("country", event.target.value)}
              />
            </label>
          </div>

          <div className="inline-actions">
            <button className="primary-button" type="button" onClick={() => void handleSave()}>
              Save profile
            </button>
            <span className="muted-text">{statusMessage}</span>
          </div>
        </article>

        <article className="card">
          <div className="card-header">
            <h3>Foundation record metadata</h3>
          </div>
          <div className="detail-list">
            <div>
              <span>Business ID</span>
              <code>{form.id}</code>
            </div>
            <div>
              <span>Created</span>
              <code>{formatDateTime(form.createdAt)}</code>
            </div>
            <div>
              <span>Updated</span>
              <code>{formatDateTime(form.updatedAt)}</code>
            </div>
            <div>
              <span>Archived</span>
              <code>{formatDateTime(form.archivedAt)}</code>
            </div>
          </div>
        </article>
      </section>
    </div>
  );
}

```

# files/src/modules/dashboard/DashboardPage.tsx

```tsx
import { useAppState } from "../../app/AppProvider";
import type { NavPage } from "../../shared/types";
import { formatDateTime } from "../../shared/utils";

interface DashboardPageProps {
  onNavigate: (page: NavPage) => void;
}

export function DashboardPage({ onNavigate }: DashboardPageProps) {
  const { data } = useAppState();

  if (!data) return null;

  return (
    <div className="page-grid">
      <section className="hero-card">
        <div>
          <div className="section-kicker">Patch 1 foundation</div>
          <h2>{data.dashboard.heroTitle}</h2>
          <p>{data.dashboard.heroBody}</p>
        </div>
        <div className="hero-actions">
          <button
            className="primary-button"
            type="button"
            onClick={() => onNavigate("business")}
          >
            Review Business Profile
          </button>
          <button
            className="secondary-button"
            type="button"
            onClick={() => onNavigate("data-center")}
          >
            Open Data Center
          </button>
        </div>
      </section>

      <section className="card-grid">
        {data.dashboard.kpis.map((kpi) => (
          <article className="card" key={kpi.id}>
            <div className="card-label">{kpi.label}</div>
            <div className="kpi-value">{kpi.value}</div>
            <p className="card-note">{kpi.note}</p>
          </article>
        ))}
      </section>

      <section className="split-grid">
        <article className="card">
          <div className="card-header">
            <h3>Recent local activity</h3>
            <span className="pill neutral">Demo seeded</span>
          </div>
          <div className="stack-list">
            {data.dashboard.recentActivity.map((activity) => (
              <div className="list-row" key={activity.id}>
                <div>
                  <strong>{activity.message}</strong>
                  <div className="muted-text">
                    {activity.category} · {activity.level}
                  </div>
                </div>
                <span className="muted-text">
                  {formatDateTime(activity.createdAt)}
                </span>
              </div>
            ))}
          </div>
        </article>

        <article className="card">
          <div className="card-header">
            <h3>Module roadmap from this foundation</h3>
            <span className="pill success">Patch-ready</span>
          </div>
          <div className="stack-list">
            {data.dashboard.moduleStatuses.map((module) => (
              <div className="list-row" key={module.id}>
                <div>
                  <strong>{module.label}</strong>
                  <div className="muted-text">{module.note}</div>
                </div>
                <span
                  className={`pill ${
                    module.status === "active-foundation"
                      ? "success"
                      : module.status === "coming-next"
                      ? "warning"
                      : "neutral"
                  }`}
                >
                  {module.status.replace("-", " ")}
                </span>
              </div>
            ))}
          </div>
        </article>
      </section>

      <section className="split-grid">
        <article className="card">
          <div className="card-header">
            <h3>Storage foundation</h3>
          </div>
          <div className="detail-list">
            <div>
              <span>Database</span>
              <code>{data.storage.databasePath}</code>
            </div>
            <div>
              <span>Backups</span>
              <code>{data.storage.backupDir}</code>
            </div>
            <div>
              <span>Exports</span>
              <code>{data.storage.exportDir}</code>
            </div>
            <div>
              <span>Logs</span>
              <code>{data.storage.logDir}</code>
            </div>
          </div>
        </article>

        <article className="card">
          <div className="card-header">
            <h3>Patch history</h3>
          </div>
          <div className="stack-list">
            {data.patchHistory.map((patch) => (
              <div className="list-row" key={patch.patchId}>
                <div>
                  <strong>{patch.patchName}</strong>
                  <div className="muted-text">Schema v{patch.schemaVersion}</div>
                </div>
                <span className="muted-text">
                  {formatDateTime(patch.appliedAt)}
                </span>
              </div>
            ))}
          </div>
        </article>
      </section>
    </div>
  );
}

```

# files/src/modules/data-center/DataCenterPage.tsx

```tsx
import { useMemo, useState } from "react";
import { useAppState } from "../../app/AppProvider";
import { formatDateTime } from "../../shared/utils";

export function DataCenterPage() {
  const { data, createBackup, exportFoundation, previewImport } = useAppState();
  const [statusMessage, setStatusMessage] = useState("");
  const [importPath, setImportPath] = useState("");
  const [previewResult, setPreviewResult] = useState<string>("");

  const latestBackup = useMemo(() => {
    return data?.backups[0] ?? null;
  }, [data?.backups]);

  if (!data) return null;

  async function handleBackup() {
    setStatusMessage("Creating backup snapshot…");
    try {
      const result = await createBackup();
      setStatusMessage(`Backup created: ${result.fileName}`);
    } catch (error) {
      setStatusMessage(
        error instanceof Error ? error.message : "Failed to create backup"
      );
    }
  }

  async function handleExport() {
    setStatusMessage("Exporting foundation snapshot…");
    try {
      const targetPath = await exportFoundation();
      setStatusMessage(`Export written to: ${targetPath}`);
    } catch (error) {
      setStatusMessage(
        error instanceof Error ? error.message : "Failed to export foundation data"
      );
    }
  }

  async function handlePreviewImport() {
    setPreviewResult("Previewing bundle…");
    try {
      const preview = await previewImport(importPath);
      if (!preview.valid) {
        setPreviewResult(
          `Invalid bundle. Warnings: ${preview.warnings.join(" | ")}`
        );
        return;
      }

      setPreviewResult(
        `Bundle OK · type=${preview.bundleType ?? "unknown"} · generated=${preview.generatedAt ?? "unknown"} · businesses=${preview.businessCount}`
      );
    } catch (error) {
      setPreviewResult(
        error instanceof Error ? error.message : "Preview failed."
      );
    }
  }

  return (
    <div className="page-grid">
      <section className="split-grid">
        <article className="card">
          <div className="card-header">
            <h2>Backup and transfer foundation</h2>
            <span className="pill success">Local only</span>
          </div>
          <p className="card-note">
            Patch 1 establishes the storage directories, backup snapshot copy,
            export JSON package, and import preview validation surface.
          </p>

          <div className="inline-actions">
            <button className="primary-button" type="button" onClick={() => void handleBackup()}>
              Create Backup Snapshot
            </button>
            <button className="secondary-button" type="button" onClick={() => void handleExport()}>
              Export Foundation Snapshot
            </button>
          </div>

          <div className="status-banner">{statusMessage || "No action run yet."}</div>

          <div className="detail-list">
            <div>
              <span>Database path</span>
              <code>{data.storage.databasePath}</code>
            </div>
            <div>
              <span>Backup directory</span>
              <code>{data.storage.backupDir}</code>
            </div>
            <div>
              <span>Export directory</span>
              <code>{data.storage.exportDir}</code>
            </div>
          </div>
        </article>

        <article className="card">
          <div className="card-header">
            <h3>Latest backup</h3>
            <span className="pill neutral">{data.storage.backupCount} total</span>
          </div>
          {latestBackup ? (
            <div className="detail-list">
              <div>
                <span>File name</span>
                <code>{latestBackup.fileName}</code>
              </div>
              <div>
                <span>Created</span>
                <code>{formatDateTime(latestBackup.createdAt)}</code>
              </div>
              <div>
                <span>Status</span>
                <code>{latestBackup.status}</code>
              </div>
              <div>
                <span>Checksum</span>
                <code>{latestBackup.checksum ?? "—"}</code>
              </div>
            </div>
          ) : (
            <p className="muted-text">No backup has been created yet.</p>
          )}
        </article>
      </section>

      <section className="split-grid">
        <article className="card">
          <div className="card-header">
            <h3>Import preview interface</h3>
            <span className="pill warning">Preview only</span>
          </div>
          <p className="card-note">
            Patch 1 validates export bundle metadata. Full import apply is deferred
            to a later data portability patch.
          </p>

          <label className="form-span-2">
            <span>Existing export bundle path</span>
            <input
              placeholder={data.storage.exportDir}
              value={importPath}
              onChange={(event) => setImportPath(event.target.value)}
            />
          </label>

          <div className="inline-actions">
            <button
              className="secondary-button"
              type="button"
              onClick={() => void handlePreviewImport()}
              disabled={!importPath.trim()}
            >
              Preview Import Bundle
            </button>
          </div>

          <div className="status-banner">{previewResult || "No preview run yet."}</div>
        </article>

        <article className="card">
          <div className="card-header">
            <h3>Recorded backups</h3>
          </div>
          <div className="stack-list">
            {data.backups.length > 0 ? (
              data.backups.map((backup) => (
                <div className="list-row" key={backup.id}>
                  <div>
                    <strong>{backup.fileName}</strong>
                    <div className="muted-text">{backup.status}</div>
                  </div>
                  <span className="muted-text">
                    {formatDateTime(backup.createdAt)}
                  </span>
                </div>
              ))
            ) : (
              <p className="muted-text">No backup rows yet.</p>
            )}
          </div>
        </article>
      </section>
    </div>
  );
}

```

# files/src/modules/settings/SettingsPage.tsx

```tsx
import { useEffect, useState } from "react";
import { useAppState } from "../../app/AppProvider";
import type { BusinessSettings } from "../../shared/types";
import { formatDateTime, humanizeBoolean } from "../../shared/utils";

export function SettingsPage() {
  const { data, saveSettings } = useAppState();
  const [form, setForm] = useState<BusinessSettings | null>(null);
  const [statusMessage, setStatusMessage] = useState("");

  useEffect(() => {
    if (data?.businessSettings) {
      setForm(data.businessSettings);
    }
  }, [data?.businessSettings]);

  if (!data || !form) return null;

  function update<K extends keyof BusinessSettings>(
    key: K,
    value: BusinessSettings[K]
  ) {
    setForm((current) => (current ? { ...current, [key]: value } : current));
  }

  async function handleSave() {
    setStatusMessage("Saving settings…");
    try {
      await saveSettings(form);
      setStatusMessage("Settings saved locally.");
    } catch (error) {
      setStatusMessage(
        error instanceof Error ? error.message : "Failed to save settings."
      );
    }
  }

  return (
    <div className="page-grid">
      <section className="split-grid">
        <article className="card">
          <div className="card-header">
            <h2>Settings foundation</h2>
            <span className="pill neutral">
              Updated {formatDateTime(form.updatedAt)}
            </span>
          </div>

          <div className="form-grid">
            <label>
              <span>Timezone</span>
              <input
                value={form.timezone}
                onChange={(event) => update("timezone", event.target.value)}
              />
            </label>

            <label>
              <span>Locale</span>
              <input
                value={form.locale}
                onChange={(event) => update("locale", event.target.value)}
              />
            </label>

            <label>
              <span>Date format</span>
              <input
                value={form.dateFormat}
                onChange={(event) => update("dateFormat", event.target.value)}
              />
            </label>

            <label>
              <span>Theme preference</span>
              <select
                value={form.theme}
                onChange={(event) => update("theme", event.target.value)}
              >
                <option value="system">System</option>
                <option value="light">Light</option>
                <option value="dark">Dark</option>
              </select>
            </label>

            <label>
              <span>Tax label</span>
              <input
                value={form.taxLabel}
                onChange={(event) => update("taxLabel", event.target.value)}
              />
            </label>

            <label>
              <span>Default tax rate (%)</span>
              <input
                type="number"
                step="0.01"
                value={form.defaultTaxRate}
                onChange={(event) =>
                  update("defaultTaxRate", Number(event.target.value))
                }
              />
            </label>

            <label className="form-span-2">
              <span>Receipt footer</span>
              <textarea
                rows={4}
                value={form.receiptFooter ?? ""}
                onChange={(event) => update("receiptFooter", event.target.value)}
              />
            </label>

            <label className="toggle-row">
              <input
                type="checkbox"
                checked={form.pricesIncludeTax}
                onChange={(event) =>
                  update("pricesIncludeTax", event.target.checked)
                }
              />
              <span>Prices include tax</span>
            </label>

            <label className="toggle-row">
              <input
                type="checkbox"
                checked={form.receiptShowAddress}
                onChange={(event) =>
                  update("receiptShowAddress", event.target.checked)
                }
              />
              <span>Show address on receipt</span>
            </label>

            <label className="toggle-row">
              <input
                type="checkbox"
                checked={form.receiptShowPhone}
                onChange={(event) =>
                  update("receiptShowPhone", event.target.checked)
                }
              />
              <span>Show phone on receipt</span>
            </label>

            <label className="toggle-row">
              <input
                type="checkbox"
                checked={form.autoBackupEnabled}
                onChange={(event) =>
                  update("autoBackupEnabled", event.target.checked)
                }
              />
              <span>Auto backup preference</span>
            </label>

            <label className="form-span-2">
              <span>Backup directory override</span>
              <input
                value={form.backupDirectory ?? ""}
                onChange={(event) => update("backupDirectory", event.target.value)}
              />
            </label>
          </div>

          <div className="card-header">
            <h3>Module toggles foundation</h3>
            <span className="muted-text">Stored locally for future patches</span>
          </div>

          <div className="toggle-stack">
            <label className="toggle-row">
              <input
                type="checkbox"
                checked={form.moduleRestaurantEnabled}
                onChange={(event) =>
                  update("moduleRestaurantEnabled", event.target.checked)
                }
              />
              <span>Restaurant mode</span>
            </label>

            <label className="toggle-row">
              <input
                type="checkbox"
                checked={form.moduleRetailEnabled}
                onChange={(event) =>
                  update("moduleRetailEnabled", event.target.checked)
                }
              />
              <span>Retail mode</span>
            </label>

            <label className="toggle-row">
              <input
                type="checkbox"
                checked={form.moduleInventoryEnabled}
                onChange={(event) =>
                  update("moduleInventoryEnabled", event.target.checked)
                }
              />
              <span>Inventory mode</span>
            </label>

            <label className="toggle-row">
              <input
                type="checkbox"
                checked={form.moduleServicesEnabled}
                onChange={(event) =>
                  update("moduleServicesEnabled", event.target.checked)
                }
              />
              <span>Service mode</span>
            </label>
          </div>

          <div className="inline-actions">
            <button className="primary-button" type="button" onClick={() => void handleSave()}>
              Save settings
            </button>
            <span className="muted-text">{statusMessage}</span>
          </div>
        </article>

        <article className="card">
          <div className="card-header">
            <h3>Current effective settings</h3>
          </div>
          <div className="detail-list">
            <div>
              <span>Theme</span>
              <code>{form.theme}</code>
            </div>
            <div>
              <span>Restaurant mode</span>
              <code>{humanizeBoolean(form.moduleRestaurantEnabled)}</code>
            </div>
            <div>
              <span>Retail mode</span>
              <code>{humanizeBoolean(form.moduleRetailEnabled)}</code>
            </div>
            <div>
              <span>Inventory mode</span>
              <code>{humanizeBoolean(form.moduleInventoryEnabled)}</code>
            </div>
            <div>
              <span>Service mode</span>
              <code>{humanizeBoolean(form.moduleServicesEnabled)}</code>
            </div>
          </div>
        </article>
      </section>
    </div>
  );
}

```

# files/src/modules/shell/AppShell.tsx

```tsx
import { useEffect, useMemo, useState } from "react";
import { useAppState } from "../../app/AppProvider";
import type { NavPage } from "../../shared/types";
import { classNames } from "../../shared/utils";
import { DashboardPage } from "../dashboard/DashboardPage";
import { BusinessPage } from "../business/BusinessPage";
import { SettingsPage } from "../settings/SettingsPage";
import { DataCenterPage } from "../data-center/DataCenterPage";

const NAV_STORAGE_KEY = "lfbm.activeNavPage";

const navItems: Array<{ key: NavPage; label: string; description: string }> = [
  {
    key: "dashboard",
    label: "Dashboard",
    description: "Local workspace summary"
  },
  {
    key: "business",
    label: "Business",
    description: "Profile and identity"
  },
  {
    key: "settings",
    label: "Settings",
    description: "Defaults and module toggles"
  },
  {
    key: "data-center",
    label: "Data Center",
    description: "Backup and transfer foundation"
  }
];

export function AppShell() {
  const { data } = useAppState();
  const [activePage, setActivePage] = useState<NavPage>(() => {
    const stored = window.localStorage.getItem(NAV_STORAGE_KEY) as NavPage | null;
    return stored ?? "dashboard";
  });

  useEffect(() => {
    window.localStorage.setItem(NAV_STORAGE_KEY, activePage);
  }, [activePage]);

  const pageTitle = useMemo(() => {
    return navItems.find((item) => item.key === activePage)?.label ?? "Dashboard";
  }, [activePage]);

  if (!data) {
    return null;
  }

  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="sidebar-brand">
          <div className="brand-badge">P1</div>
          <div>
            <strong>Local Business Manager</strong>
            <div className="muted-text">
              {data.appInfo.version} · {data.appInfo.patchLevel}
            </div>
          </div>
        </div>

        <div className="sidebar-section-label">Workspace</div>
        <nav className="nav-list">
          {navItems.map((item) => (
            <button
              key={item.key}
              type="button"
              className={classNames(
                "nav-item",
                activePage === item.key && "nav-item-active"
              )}
              onClick={() => setActivePage(item.key)}
            >
              <span className="nav-title">{item.label}</span>
              <span className="nav-description">{item.description}</span>
            </button>
          ))}
        </nav>

        <div className="sidebar-section-label">Active business</div>
        <div className="sidebar-card">
          <div className="sidebar-card-title">{data.activeBusiness.name}</div>
          <div className="muted-text">{data.activeBusiness.businessType}</div>
          <div className="muted-text">{data.activeBusiness.currencyCode}</div>
        </div>

        <div className="sidebar-section-label">Foundation status</div>
        <div className="sidebar-card">
          <div className="sidebar-pill success">Local-first</div>
          <div className="sidebar-pill neutral">SQLite ready</div>
          <div className="sidebar-pill neutral">Patch registry ready</div>
        </div>
      </aside>

      <main className="workspace">
        <header className="workspace-header">
          <div>
            <h1>{pageTitle}</h1>
            <p>
              Foundation shell for a local-first business desktop application.
            </p>
          </div>
          <div className="workspace-header-meta">
            <span className="meta-chip">Business: {data.activeBusiness.code}</span>
            <span className="meta-chip">
              Schema v{data.appInfo.schemaVersion}
            </span>
          </div>
        </header>

        <section className="workspace-content">
          {activePage === "dashboard" && (
            <DashboardPage onNavigate={setActivePage} />
          )}
          {activePage === "business" && <BusinessPage />}
          {activePage === "settings" && <SettingsPage />}
          {activePage === "data-center" && <DataCenterPage />}
        </section>
      </main>
    </div>
  );
}

```

# files/src/shared/api.ts

```ts
import { invoke } from "@tauri-apps/api/core";
import type {
  AppBootstrap,
  BackupRecord,
  BusinessProfile,
  BusinessSettings,
  ImportPreview
} from "./types";

export async function bootstrapApp(): Promise<AppBootstrap> {
  return invoke<AppBootstrap>("bootstrap_app");
}

export async function saveBusinessProfile(
  profile: BusinessProfile
): Promise<BusinessProfile> {
  return invoke<BusinessProfile>("save_business_profile", { profile });
}

export async function saveBusinessSettings(
  settings: BusinessSettings
): Promise<BusinessSettings> {
  return invoke<BusinessSettings>("save_business_settings", { settings });
}

export async function createBackupSnapshot(): Promise<BackupRecord> {
  return invoke<BackupRecord>("create_backup_snapshot");
}

export async function exportFoundationSnapshot(): Promise<string> {
  return invoke<string>("export_foundation_snapshot");
}

export async function previewImportBundle(
  filePath: string
): Promise<ImportPreview> {
  return invoke<ImportPreview>("preview_import_bundle", { filePath });
}

```

# files/src/shared/types.ts

```ts
export interface BusinessProfile {
  id: string;
  name: string;
  legalName: string | null;
  code: string;
  businessType: string;
  currencyCode: string;
  taxMode: string;
  phone: string | null;
  email: string | null;
  addressLine1: string | null;
  addressLine2: string | null;
  city: string | null;
  state: string | null;
  postalCode: string | null;
  country: string | null;
  createdAt: string;
  updatedAt: string;
  archivedAt: string | null;
}

export interface BusinessSettings {
  businessId: string;
  timezone: string;
  locale: string;
  dateFormat: string;
  theme: string;
  taxLabel: string;
  defaultTaxRate: number;
  pricesIncludeTax: boolean;
  receiptFooter: string | null;
  receiptShowAddress: boolean;
  receiptShowPhone: boolean;
  autoBackupEnabled: boolean;
  backupDirectory: string | null;
  moduleRestaurantEnabled: boolean;
  moduleRetailEnabled: boolean;
  moduleInventoryEnabled: boolean;
  moduleServicesEnabled: boolean;
  updatedAt: string;
}

export interface AppInfo {
  appName: string;
  productName: string;
  version: string;
  schemaVersion: number;
  patchLevel: string;
  initializedAt: string;
}

export interface PatchRecord {
  patchId: string;
  patchName: string;
  appliedAt: string;
  schemaVersion: number;
}

export interface BackupRecord {
  id: string;
  businessId: string | null;
  fileName: string;
  filePath: string;
  checksum: string | null;
  status: string;
  createdAt: string;
}

export interface ExportJobRecord {
  id: string;
  businessId: string | null;
  format: string;
  status: string;
  targetPath: string | null;
  createdAt: string;
  completedAt: string | null;
}

export interface ImportPreview {
  filePath: string;
  valid: boolean;
  manifestVersion: string | null;
  bundleType: string | null;
  sourcePatchLevel: string | null;
  businessCount: number;
  generatedAt: string | null;
  warnings: string[];
}

export interface RecentActivity {
  id: string;
  level: string;
  category: string;
  message: string;
  createdAt: string;
}

export interface KpiCard {
  id: string;
  label: string;
  value: string;
  note: string;
}

export interface ModuleStatus {
  id: string;
  label: string;
  status: "active-foundation" | "planned" | "coming-next";
  note: string;
}

export interface DashboardShellData {
  heroTitle: string;
  heroBody: string;
  kpis: KpiCard[];
  recentActivity: RecentActivity[];
  moduleStatuses: ModuleStatus[];
}

export interface StorageStatus {
  dataDir: string;
  configDir: string;
  logDir: string;
  backupDir: string;
  exportDir: string;
  databasePath: string;
  databaseExists: boolean;
  backupCount: number;
  exportCount: number;
}

export interface AppBootstrap {
  appInfo: AppInfo;
  activeBusiness: BusinessProfile;
  businessSettings: BusinessSettings;
  businesses: BusinessProfile[];
  patchHistory: PatchRecord[];
  backups: BackupRecord[];
  storage: StorageStatus;
  dashboard: DashboardShellData;
}

export type NavPage = "dashboard" | "business" | "settings" | "data-center";

```

# files/src/shared/utils.ts

```ts
export function formatDateTime(value: string | null | undefined): string {
  if (!value) return "—";
  const parsed = new Date(value);
  if (Number.isNaN(parsed.getTime())) return value;
  return new Intl.DateTimeFormat(undefined, {
    dateStyle: "medium",
    timeStyle: "short"
  }).format(parsed);
}

export function humanizeBoolean(value: boolean): string {
  return value ? "Enabled" : "Disabled";
}

export function classNames(...values: Array<string | false | null | undefined>): string {
  return values.filter(Boolean).join(" ");
}

```

# files/src/styles.css

```css
:root {
  color-scheme: light;
  font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont,
    "Segoe UI", sans-serif;
  background: #f4f6fb;
  color: #172033;
  line-height: 1.5;
  font-weight: 400;
}

* {
  box-sizing: border-box;
}

html,
body,
#root {
  margin: 0;
  min-height: 100%;
}

body {
  background:
    radial-gradient(circle at top right, #e8f0ff 0, transparent 26%),
    linear-gradient(180deg, #f7f9fd 0%, #eef3fb 100%);
}

button,
input,
select,
textarea {
  font: inherit;
}

button {
  cursor: pointer;
}

code {
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,
    "Liberation Mono", monospace;
  word-break: break-all;
}

.app-loading-shell {
  min-height: 100vh;
  display: grid;
  place-items: center;
  align-content: center;
  gap: 0.8rem;
  padding: 2rem;
  text-align: center;
}

.spinner {
  width: 3rem;
  height: 3rem;
  border-radius: 999px;
  border: 3px solid #d3def6;
  border-top-color: #305fdb;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.app-shell {
  min-height: 100vh;
  display: grid;
  grid-template-columns: 290px minmax(0, 1fr);
}

.sidebar {
  padding: 1.5rem;
  background: #0f172a;
  color: #e5ecfa;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  border-right: 1px solid rgba(255, 255, 255, 0.08);
}

.sidebar-brand {
  display: flex;
  align-items: center;
  gap: 0.85rem;
}

.brand-badge {
  width: 2.4rem;
  height: 2.4rem;
  border-radius: 0.8rem;
  display: grid;
  place-items: center;
  background: linear-gradient(135deg, #60a5fa, #2563eb);
  font-weight: 700;
}

.sidebar-section-label {
  font-size: 0.78rem;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  opacity: 0.7;
  margin-top: 0.35rem;
}

.nav-list {
  display: grid;
  gap: 0.65rem;
}

.nav-item {
  background: rgba(255, 255, 255, 0.05);
  color: inherit;
  border: 1px solid transparent;
  border-radius: 0.95rem;
  padding: 0.9rem 1rem;
  text-align: left;
  display: grid;
  gap: 0.2rem;
  transition: 0.18s ease;
}

.nav-item:hover {
  border-color: rgba(255, 255, 255, 0.15);
  transform: translateY(-1px);
}

.nav-item-active {
  background: rgba(37, 99, 235, 0.24);
  border-color: rgba(96, 165, 250, 0.4);
}

.nav-title {
  font-weight: 700;
}

.nav-description {
  color: #9fb2d7;
  font-size: 0.88rem;
}

.sidebar-card {
  background: rgba(255, 255, 255, 0.05);
  border-radius: 0.95rem;
  padding: 1rem;
  display: grid;
  gap: 0.5rem;
}

.sidebar-card-title {
  font-weight: 700;
}

.sidebar-pill {
  display: inline-flex;
  align-items: center;
  width: fit-content;
  padding: 0.35rem 0.6rem;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.08);
  font-size: 0.85rem;
}

.workspace {
  min-width: 0;
  padding: 1.5rem 2rem 2rem;
}

.workspace-header {
  display: flex;
  justify-content: space-between;
  gap: 1rem;
  align-items: flex-start;
  margin-bottom: 1.25rem;
}

.workspace-header h1 {
  margin: 0 0 0.25rem;
  font-size: 2rem;
}

.workspace-header p {
  margin: 0;
  color: #52607a;
}

.workspace-header-meta {
  display: flex;
  gap: 0.6rem;
  flex-wrap: wrap;
}

.meta-chip,
.pill {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 0.35rem 0.7rem;
  border-radius: 999px;
  font-size: 0.84rem;
  border: 1px solid transparent;
}

.meta-chip {
  background: #ffffff;
  color: #23304a;
  border-color: #d6e0f2;
}

.pill.success {
  background: #eaf8ef;
  color: #0f7a37;
  border-color: #bfe6c9;
}

.pill.warning {
  background: #fff6e5;
  color: #a75d00;
  border-color: #f4d39c;
}

.pill.neutral {
  background: #f0f4fb;
  color: #40506b;
  border-color: #d8e0f0;
}

.workspace-content {
  display: grid;
  gap: 1.25rem;
}

.page-grid {
  display: grid;
  gap: 1.25rem;
}

.hero-card,
.card {
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(8px);
  border: 1px solid #dbe5f4;
  border-radius: 1.2rem;
  padding: 1.2rem 1.25rem;
  box-shadow: 0 14px 35px rgba(15, 23, 42, 0.05);
}

.hero-card {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 1rem;
}

.hero-card h2,
.card h2,
.card h3 {
  margin: 0;
}

.hero-card p,
.card-note {
  color: #5c6c89;
  margin-top: 0.5rem;
}

.hero-actions,
.inline-actions {
  display: flex;
  gap: 0.75rem;
  align-items: center;
  flex-wrap: wrap;
}

.primary-button,
.secondary-button {
  border: none;
  border-radius: 0.9rem;
  padding: 0.75rem 1rem;
  font-weight: 700;
}

.primary-button {
  background: linear-gradient(135deg, #2563eb, #1d4ed8);
  color: white;
}

.primary-button:hover {
  filter: brightness(1.04);
}

.secondary-button {
  background: white;
  color: #24324a;
  border: 1px solid #d3def3;
}

.secondary-button:hover {
  background: #f5f8fd;
}

.section-kicker {
  color: #3357aa;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  font-size: 0.78rem;
  font-weight: 700;
}

.card-grid {
  display: grid;
  gap: 1rem;
  grid-template-columns: repeat(4, minmax(0, 1fr));
}

.card-label {
  color: #60708d;
  font-size: 0.92rem;
}

.kpi-value {
  font-size: 1.9rem;
  font-weight: 800;
  margin-top: 0.2rem;
}

.split-grid {
  display: grid;
  gap: 1rem;
  grid-template-columns: repeat(2, minmax(0, 1fr));
}

.card-header {
  display: flex;
  justify-content: space-between;
  gap: 0.8rem;
  align-items: center;
  margin-bottom: 0.75rem;
}

.stack-list {
  display: grid;
  gap: 0.75rem;
}

.list-row {
  display: flex;
  justify-content: space-between;
  gap: 0.75rem;
  align-items: flex-start;
  padding: 0.8rem 0;
  border-top: 1px solid #edf2fb;
}

.list-row:first-child {
  border-top: none;
  padding-top: 0;
}

.detail-list {
  display: grid;
  gap: 0.75rem;
}

.detail-list > div {
  display: grid;
  gap: 0.25rem;
}

.detail-list span {
  font-size: 0.88rem;
  color: #5f708c;
}

.form-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 0.9rem;
}

.form-grid label,
.form-span-2 {
  display: grid;
  gap: 0.35rem;
}

.form-span-2 {
  grid-column: 1 / -1;
}

input,
select,
textarea {
  width: 100%;
  border: 1px solid #ccd8ee;
  border-radius: 0.85rem;
  padding: 0.8rem 0.9rem;
  background: #fbfcff;
  color: #172033;
}

input:focus,
select:focus,
textarea:focus {
  outline: 3px solid rgba(37, 99, 235, 0.14);
  border-color: #78a6f8;
}

.toggle-stack {
  display: grid;
  gap: 0.7rem;
}

.toggle-row {
  display: flex;
  align-items: center;
  gap: 0.6rem;
  padding: 0.7rem 0.1rem;
}

.toggle-row input[type="checkbox"] {
  width: auto;
}

.status-banner {
  padding: 0.8rem 0.95rem;
  background: #f3f7fd;
  border: 1px solid #dae4f5;
  border-radius: 0.95rem;
  color: #31405e;
}

.muted-text {
  color: #6a7a96;
  font-size: 0.92rem;
}

@media (max-width: 1120px) {
  .card-grid,
  .split-grid {
    grid-template-columns: 1fr 1fr;
  }
}

@media (max-width: 920px) {
  .app-shell {
    grid-template-columns: 1fr;
  }

  .sidebar {
    border-right: none;
    border-bottom: 1px solid rgba(15, 23, 42, 0.12);
  }

  .workspace {
    padding: 1.25rem;
  }

  .workspace-header,
  .hero-card,
  .split-grid,
  .card-grid,
  .form-grid {
    grid-template-columns: 1fr;
    display: grid;
  }

  .workspace-header {
    gap: 1rem;
  }

  .hero-card {
    align-items: stretch;
  }
}

```

# files/src-tauri/.gitignore

```
/target

```

# files/src-tauri/Cargo.toml

```toml
[package]
name = "local-business-manager"
version = "0.1.0"
description = "Local-first business management desktop app foundation"
authors = ["OpenAI"]
edition = "2021"

[lib]
name = "local_business_manager"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build = { version = "2.0.0", features = [] }

[dependencies]
chrono = { version = "0.4.38", features = ["serde"] }
rusqlite = { version = "0.32.1", features = ["bundled"] }
serde = { version = "1.0.215", features = ["derive"] }
serde_json = "1.0.133"
sha2 = "0.10.8"
tauri = { version = "2.0.0", features = [] }
uuid = { version = "1.11.0", features = ["v4", "serde"] }

```

# files/src-tauri/build.rs

```rust
fn main() {
    tauri_build::build()
}

```

# files/src-tauri/capabilities/default.json

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "description": "Base capability for the single local application window.",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "core:app:default",
    "core:event:default",
    "core:path:default",
    "core:window:default"
  ]
}

```

# files/src-tauri/src/commands/bootstrap.rs

```rust
use tauri::AppHandle;

use crate::{
    core::{db, error::CommandResult},
    domain::bootstrap::build_app_bootstrap,
};

#[tauri::command]
pub fn bootstrap_app(app: AppHandle) -> CommandResult<crate::domain::models::AppBootstrap> {
    db::with_connection(&app, |conn, paths| {
        let app_info = db::load_app_info(conn)?;
        let active_business = db::get_active_business(conn)?;
        let business_settings = db::get_business_settings(conn, &active_business.id)?;
        let businesses = db::list_businesses(conn)?;
        let patch_history = db::list_patch_history(conn)?;
        let backups = db::list_backups(conn)?;
        let storage = db::build_storage_status(conn, paths)?;
        let recent_activity = db::list_recent_activity(conn, 6)?;

        Ok(build_app_bootstrap(
            app_info,
            active_business,
            business_settings,
            businesses,
            patch_history,
            backups,
            storage,
            recent_activity,
        ))
    })
}

```

# files/src-tauri/src/commands/business.rs

```rust
use tauri::AppHandle;

use crate::{
    core::{db, error::CommandResult},
    domain::models::BusinessProfile,
};

#[tauri::command]
pub fn save_business_profile(app: AppHandle, profile: BusinessProfile) -> CommandResult<BusinessProfile> {
    db::with_connection(&app, |conn, _paths| db::save_business_profile(conn, &profile))
}

```

# files/src-tauri/src/commands/data_center.rs

```rust
use std::{fs, path::PathBuf};

use chrono::Utc;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use tauri::AppHandle;
use uuid::Uuid;

use crate::{
    core::{
        db,
        error::{to_command_error, CommandResult},
        paths::resolve_paths,
    },
    domain::models::{BackupRecord, ExportJobRecord, ImportPreview},
};

fn checksum_for_file(path: &PathBuf) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|error| to_command_error("failed to read file for checksum", error))?;
    let digest = Sha256::digest(&bytes);
    Ok(format!("{digest:x}"))
}

#[tauri::command]
pub fn create_backup_snapshot(app: AppHandle) -> CommandResult<BackupRecord> {
    let paths = resolve_paths(&app)?;
    let (active_business, business_settings) = db::with_connection(&app, |conn, _paths| {
        let active_business = db::get_active_business(conn)?;
        let business_settings = db::get_business_settings(conn, &active_business.id)?;
        Ok((active_business, business_settings))
    })?;

    let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let file_name = format!("foundation_snapshot_{timestamp}.sqlite");
    let backup_dir = business_settings
        .backup_directory
        .clone()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(paths.backup_dir.clone());
    fs::create_dir_all(&backup_dir)
        .map_err(|error| to_command_error("failed to prepare backup directory", error))?;
    let destination = PathBuf::from(backup_dir).join(&file_name);

    fs::copy(paths.database_path_buf(), &destination)
        .map_err(|error| to_command_error("failed to create backup snapshot", error))?;

    let checksum = checksum_for_file(&destination)?;
    let record = BackupRecord {
        id: Uuid::new_v4().to_string(),
        business_id: Some(active_business.id.clone()),
        file_name,
        file_path: destination.to_string_lossy().to_string(),
        checksum: Some(checksum),
        status: "completed".into(),
        created_at: db::now_iso(),
    };

    db::with_connection(&app, |conn, _paths| {
        db::insert_backup_record(conn, &record)?;
        db::insert_log(conn, "INFO", "backup", "Local backup snapshot created", None)?;
        Ok(record.clone())
    })
}

#[tauri::command]
pub fn export_foundation_snapshot(app: AppHandle) -> CommandResult<String> {
    let paths = resolve_paths(&app)?;

    let (app_info, active_business, business_settings, businesses, patch_history, backups) =
        db::with_connection(&app, |conn, _paths| {
            let active_business = db::get_active_business(conn)?;
            let business_settings = db::get_business_settings(conn, &active_business.id)?;

            Ok((
                db::load_app_info(conn)?,
                active_business,
                business_settings,
                db::list_businesses(conn)?,
                db::list_patch_history(conn)?,
                db::list_backups(conn)?,
            ))
        })?;

    let generated_at = db::now_iso();
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let export_path = PathBuf::from(&paths.export_dir).join(format!("foundation_export_{timestamp}.json"));

    let source_patch_level = app_info.patch_level.clone();
    let product_name = app_info.product_name.clone();
    let active_business_id = active_business.id.clone();

    let bundle = json!({
        "manifest": {
            "bundleVersion": "1.0.0",
            "bundleType": "foundation-export",
            "sourcePatchLevel": source_patch_level,
            "schemaVersion": app_info.schema_version,
            "generatedAt": generated_at.clone(),
            "productName": product_name
        },
        "appInfo": app_info,
        "activeBusinessId": active_business_id.clone(),
        "businesses": businesses,
        "businessSettings": business_settings,
        "patchHistory": patch_history,
        "backupRecords": backups
    });

    fs::write(
        &export_path,
        serde_json::to_string_pretty(&bundle)
            .map_err(|error| to_command_error("failed to serialize export bundle", error))?,
    )
    .map_err(|error| to_command_error("failed to write export bundle", error))?;

    let export_job = ExportJobRecord {
        id: Uuid::new_v4().to_string(),
        business_id: Some(active_business_id),
        format: "json-foundation".into(),
        status: "completed".into(),
        target_path: Some(export_path.to_string_lossy().to_string()),
        created_at: generated_at.clone(),
        completed_at: Some(generated_at),
    };

    db::with_connection(&app, |conn, _paths| {
        db::insert_export_job(conn, &export_job)?;
        db::insert_log(conn, "INFO", "export", "Foundation export bundle created", None)?;
        Ok(())
    })?;

    Ok(export_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn preview_import_bundle(app: AppHandle, file_path: String) -> CommandResult<ImportPreview> {
    let raw = fs::read_to_string(&file_path)
        .map_err(|error| to_command_error("failed to read import bundle", error))?;

    let parsed: Value =
        serde_json::from_str(&raw).map_err(|error| to_command_error("invalid JSON bundle", error))?;

    let manifest = parsed.get("manifest").cloned().unwrap_or_else(|| json!({}));

    let businesses = parsed
        .get("businesses")
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default();

    let mut warnings = Vec::new();

    if manifest.get("bundleType").is_none() {
        warnings.push("Bundle is missing manifest.bundleType".into());
    }

    if manifest.get("bundleVersion").is_none() {
        warnings.push("Bundle is missing manifest.bundleVersion".into());
    }

    let preview = ImportPreview {
        file_path: file_path.clone(),
        valid: warnings.is_empty(),
        manifest_version: manifest
            .get("bundleVersion")
            .and_then(|value| value.as_str())
            .map(ToString::to_string),
        bundle_type: manifest
            .get("bundleType")
            .and_then(|value| value.as_str())
            .map(ToString::to_string),
        source_patch_level: manifest
            .get("sourcePatchLevel")
            .and_then(|value| value.as_str())
            .map(ToString::to_string),
        business_count: businesses.len(),
        generated_at: manifest
            .get("generatedAt")
            .and_then(|value| value.as_str())
            .map(ToString::to_string),
        warnings,
    };

    db::with_connection(&app, |conn, _paths| {
        db::insert_import_job(conn, None, "json-foundation", "previewed", &file_path)?;
        db::insert_log(conn, "INFO", "import", "Import bundle previewed", None)?;
        Ok(())
    })?;

    Ok(preview)
}

```

# files/src-tauri/src/commands/mod.rs

```rust
pub mod bootstrap;
pub mod business;
pub mod data_center;
pub mod settings;

```

# files/src-tauri/src/commands/settings.rs

```rust
use tauri::AppHandle;

use crate::{
    core::{db, error::CommandResult},
    domain::models::BusinessSettings,
};

#[tauri::command]
pub fn save_business_settings(
    app: AppHandle,
    settings: BusinessSettings,
) -> CommandResult<BusinessSettings> {
    db::with_connection(&app, |conn, _paths| db::save_business_settings(conn, &settings))
}

```

# files/src-tauri/src/core/db.rs

```rust
use std::fs;

use chrono::Utc;
use rusqlite::{params, Connection, Row};
use tauri::AppHandle;
use uuid::Uuid;

use crate::domain::models::{
    AppInfo, BackupRecord, BusinessProfile, BusinessSettings, ExportJobRecord, PatchRecord,
    RecentActivity, StorageStatus,
};

use super::{
    error::to_command_error,
    migrations::{self, CURRENT_SCHEMA_VERSION},
    patching,
    paths::{ensure_directories, resolve_paths, AppPaths},
    seed,
};

pub fn initialize(app: &AppHandle) -> Result<(), String> {
    let paths = resolve_paths(app)?;
    ensure_directories(&paths)?;
    let conn = open_connection(&paths).map_err(|error| to_command_error("failed to open database", error))?;
    migrations::run(&conn).map_err(|error| to_command_error("failed to run migrations", error))?;
    patching::register_patch(&conn)
        .map_err(|error| to_command_error("failed to register patch history", error))?;
    seed::seed_if_empty(&conn).map_err(|error| to_command_error("failed to seed base data", error))?;
    Ok(())
}

pub fn with_connection<T, F>(app: &AppHandle, action: F) -> Result<T, String>
where
    F: FnOnce(&Connection, &AppPaths) -> Result<T, String>,
{
    let paths = resolve_paths(app)?;
    ensure_directories(&paths)?;
    let conn = open_connection(&paths).map_err(|error| to_command_error("failed to open database", error))?;
    action(&conn, &paths)
}

pub fn open_connection(paths: &AppPaths) -> rusqlite::Result<Connection> {
    let conn = Connection::open(&paths.database_path)?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.pragma_update(None, "journal_mode", "DELETE")?;
    conn.pragma_update(None, "synchronous", "NORMAL")?;
    Ok(conn)
}

fn bool_from_row(row: &Row, index: usize) -> rusqlite::Result<bool> {
    let value: i64 = row.get(index)?;
    Ok(value != 0)
}

fn business_from_row(row: &Row) -> rusqlite::Result<BusinessProfile> {
    Ok(BusinessProfile {
        id: row.get(0)?,
        name: row.get(1)?,
        legal_name: row.get(2)?,
        code: row.get(3)?,
        business_type: row.get(4)?,
        currency_code: row.get(5)?,
        tax_mode: row.get(6)?,
        phone: row.get(7)?,
        email: row.get(8)?,
        address_line1: row.get(9)?,
        address_line2: row.get(10)?,
        city: row.get(11)?,
        state: row.get(12)?,
        postal_code: row.get(13)?,
        country: row.get(14)?,
        created_at: row.get(15)?,
        updated_at: row.get(16)?,
        archived_at: row.get(17)?,
    })
}

fn settings_from_row(row: &Row) -> rusqlite::Result<BusinessSettings> {
    Ok(BusinessSettings {
        business_id: row.get(0)?,
        timezone: row.get(1)?,
        locale: row.get(2)?,
        date_format: row.get(3)?,
        theme: row.get(4)?,
        tax_label: row.get(5)?,
        default_tax_rate: row.get(6)?,
        prices_include_tax: bool_from_row(row, 7)?,
        receipt_footer: row.get(8)?,
        receipt_show_address: bool_from_row(row, 9)?,
        receipt_show_phone: bool_from_row(row, 10)?,
        auto_backup_enabled: bool_from_row(row, 11)?,
        backup_directory: row.get(12)?,
        module_restaurant_enabled: bool_from_row(row, 13)?,
        module_retail_enabled: bool_from_row(row, 14)?,
        module_inventory_enabled: bool_from_row(row, 15)?,
        module_services_enabled: bool_from_row(row, 16)?,
        updated_at: row.get(17)?,
    })
}

fn backup_from_row(row: &Row) -> rusqlite::Result<BackupRecord> {
    Ok(BackupRecord {
        id: row.get(0)?,
        business_id: row.get(1)?,
        file_name: row.get(2)?,
        file_path: row.get(3)?,
        checksum: row.get(4)?,
        status: row.get(5)?,
        created_at: row.get(6)?,
    })
}

fn patch_from_row(row: &Row) -> rusqlite::Result<PatchRecord> {
    Ok(PatchRecord {
        patch_id: row.get(0)?,
        patch_name: row.get(1)?,
        schema_version: row.get(2)?,
        applied_at: row.get(3)?,
    })
}

fn activity_from_row(row: &Row) -> rusqlite::Result<RecentActivity> {
    Ok(RecentActivity {
        id: row.get(0)?,
        level: row.get(1)?,
        category: row.get(2)?,
        message: row.get(3)?,
        created_at: row.get(4)?,
    })
}

pub fn now_iso() -> String {
    Utc::now().to_rfc3339()
}

pub fn get_meta(conn: &Connection, key: &str) -> Result<Option<String>, String> {
    let mut stmt = conn
        .prepare("SELECT value FROM app_meta WHERE key = ?1")
        .map_err(|error| to_command_error("failed to prepare meta lookup", error))?;

    let mut rows = stmt
        .query(params![key])
        .map_err(|error| to_command_error("failed to query meta value", error))?;

    if let Some(row) = rows
        .next()
        .map_err(|error| to_command_error("failed to iterate meta rows", error))?
    {
        let value: String = row
            .get(0)
            .map_err(|error| to_command_error("failed to read meta row", error))?;
        Ok(Some(value))
    } else {
        Ok(None)
    }
}

pub fn set_meta(conn: &Connection, key: &str, value: &str) -> Result<(), String> {
    conn.execute(
        "INSERT INTO app_meta (key, value, updated_at)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
        params![key, value, now_iso()],
    )
    .map_err(|error| to_command_error("failed to write meta value", error))?;
    Ok(())
}

pub fn insert_log(
    conn: &Connection,
    level: &str,
    category: &str,
    message: &str,
    payload_json: Option<&str>,
) -> Result<(), String> {
    conn.execute(
        "INSERT INTO app_logs (id, level, category, message, payload_json, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            Uuid::new_v4().to_string(),
            level,
            category,
            message,
            payload_json,
            now_iso()
        ],
    )
    .map_err(|error| to_command_error("failed to insert app log", error))?;
    Ok(())
}

pub fn list_recent_activity(conn: &Connection, limit: usize) -> Result<Vec<RecentActivity>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, level, category, message, created_at
             FROM app_logs
             ORDER BY created_at DESC
             LIMIT ?1",
        )
        .map_err(|error| to_command_error("failed to prepare activity query", error))?;

    let rows = stmt
        .query_map(params![limit as i64], activity_from_row)
        .map_err(|error| to_command_error("failed to query recent activity", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map recent activity", error))
}

pub fn list_businesses(conn: &Connection) -> Result<Vec<BusinessProfile>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                id, name, legal_name, code, business_type, currency_code, tax_mode,
                phone, email, address_line1, address_line2, city, state, postal_code,
                country, created_at, updated_at, archived_at
             FROM businesses
             ORDER BY created_at ASC",
        )
        .map_err(|error| to_command_error("failed to prepare businesses query", error))?;

    let rows = stmt
        .query_map([], business_from_row)
        .map_err(|error| to_command_error("failed to query businesses", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map businesses", error))
}

pub fn get_active_business(conn: &Connection) -> Result<BusinessProfile, String> {
    if let Some(active_id) = get_meta(conn, "active_business_id")? {
        let mut stmt = conn
            .prepare(
                "SELECT
                    id, name, legal_name, code, business_type, currency_code, tax_mode,
                    phone, email, address_line1, address_line2, city, state, postal_code,
                    country, created_at, updated_at, archived_at
                 FROM businesses WHERE id = ?1 LIMIT 1",
            )
            .map_err(|error| to_command_error("failed to prepare active business query", error))?;

        let business = stmt
            .query_row(params![active_id], business_from_row)
            .map_err(|error| to_command_error("failed to load active business", error))?;

        return Ok(business);
    }

    let businesses = list_businesses(conn)?;
    businesses
        .into_iter()
        .next()
        .ok_or_else(|| "no businesses found in local storage".to_string())
}

pub fn get_business_settings(conn: &Connection, business_id: &str) -> Result<BusinessSettings, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                business_id, timezone, locale, date_format, theme, tax_label,
                default_tax_rate, prices_include_tax, receipt_footer, receipt_show_address,
                receipt_show_phone, auto_backup_enabled, backup_directory,
                module_restaurant_enabled, module_retail_enabled, module_inventory_enabled,
                module_services_enabled, updated_at
             FROM business_settings
             WHERE business_id = ?1
             LIMIT 1",
        )
        .map_err(|error| to_command_error("failed to prepare business settings query", error))?;

    stmt.query_row(params![business_id], settings_from_row)
        .map_err(|error| to_command_error("failed to load business settings", error))
}

pub fn save_business_profile(conn: &Connection, profile: &BusinessProfile) -> Result<BusinessProfile, String> {
    let updated_at = now_iso();

    conn.execute(
        "UPDATE businesses
         SET name = ?2,
             legal_name = ?3,
             code = ?4,
             business_type = ?5,
             currency_code = ?6,
             tax_mode = ?7,
             phone = ?8,
             email = ?9,
             address_line1 = ?10,
             address_line2 = ?11,
             city = ?12,
             state = ?13,
             postal_code = ?14,
             country = ?15,
             updated_at = ?16
         WHERE id = ?1",
        params![
            &profile.id,
            &profile.name,
            &profile.legal_name,
            &profile.code,
            &profile.business_type,
            &profile.currency_code,
            &profile.tax_mode,
            &profile.phone,
            &profile.email,
            &profile.address_line1,
            &profile.address_line2,
            &profile.city,
            &profile.state,
            &profile.postal_code,
            &profile.country,
            &updated_at
        ],
    )
    .map_err(|error| to_command_error("failed to update business profile", error))?;

    insert_log(conn, "INFO", "business", "Business profile updated", None)?;
    get_active_business(conn)
}

pub fn save_business_settings(
    conn: &Connection,
    settings: &BusinessSettings,
) -> Result<BusinessSettings, String> {
    let updated_at = now_iso();
    let backup_directory = settings
        .backup_directory
        .clone()
        .and_then(|value| if value.trim().is_empty() { None } else { Some(value) });

    conn.execute(
        "UPDATE business_settings
         SET timezone = ?2,
             locale = ?3,
             date_format = ?4,
             theme = ?5,
             tax_label = ?6,
             default_tax_rate = ?7,
             prices_include_tax = ?8,
             receipt_footer = ?9,
             receipt_show_address = ?10,
             receipt_show_phone = ?11,
             auto_backup_enabled = ?12,
             backup_directory = ?13,
             module_restaurant_enabled = ?14,
             module_retail_enabled = ?15,
             module_inventory_enabled = ?16,
             module_services_enabled = ?17,
             updated_at = ?18
         WHERE business_id = ?1",
        params![
            &settings.business_id,
            &settings.timezone,
            &settings.locale,
            &settings.date_format,
            &settings.theme,
            &settings.tax_label,
            settings.default_tax_rate,
            if settings.prices_include_tax { 1_i64 } else { 0_i64 },
            &settings.receipt_footer,
            if settings.receipt_show_address { 1_i64 } else { 0_i64 },
            if settings.receipt_show_phone { 1_i64 } else { 0_i64 },
            if settings.auto_backup_enabled { 1_i64 } else { 0_i64 },
            &backup_directory,
            if settings.module_restaurant_enabled { 1_i64 } else { 0_i64 },
            if settings.module_retail_enabled { 1_i64 } else { 0_i64 },
            if settings.module_inventory_enabled { 1_i64 } else { 0_i64 },
            if settings.module_services_enabled { 1_i64 } else { 0_i64 },
            &updated_at
        ],
    )
    .map_err(|error| to_command_error("failed to update business settings", error))?;

    insert_log(conn, "INFO", "settings", "Business settings updated", None)?;
    get_business_settings(conn, &settings.business_id)
}

pub fn list_patch_history(conn: &Connection) -> Result<Vec<PatchRecord>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT patch_id, patch_name, schema_version, applied_at
             FROM patch_history
             ORDER BY applied_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare patch history query", error))?;

    let rows = stmt
        .query_map([], patch_from_row)
        .map_err(|error| to_command_error("failed to query patch history", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map patch history", error))
}

pub fn list_backups(conn: &Connection) -> Result<Vec<BackupRecord>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, business_id, file_name, file_path, checksum, status, created_at
             FROM backup_records
             ORDER BY created_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare backup query", error))?;

    let rows = stmt
        .query_map([], backup_from_row)
        .map_err(|error| to_command_error("failed to query backups", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map backups", error))
}

pub fn insert_backup_record(conn: &Connection, record: &BackupRecord) -> Result<(), String> {
    conn.execute(
        "INSERT INTO backup_records (id, business_id, file_name, file_path, checksum, status, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            &record.id,
            &record.business_id,
            &record.file_name,
            &record.file_path,
            &record.checksum,
            &record.status,
            &record.created_at
        ],
    )
    .map_err(|error| to_command_error("failed to insert backup record", error))?;
    Ok(())
}

pub fn insert_export_job(conn: &Connection, record: &ExportJobRecord) -> Result<(), String> {
    conn.execute(
        "INSERT INTO export_jobs (id, business_id, format, status, target_path, created_at, completed_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            &record.id,
            &record.business_id,
            &record.format,
            &record.status,
            &record.target_path,
            &record.created_at,
            &record.completed_at
        ],
    )
    .map_err(|error| to_command_error("failed to insert export job", error))?;
    Ok(())
}

pub fn insert_import_job(
    conn: &Connection,
    business_id: Option<&str>,
    format: &str,
    status: &str,
    source_path: &str,
) -> Result<(), String> {
    conn.execute(
        "INSERT INTO import_jobs (id, business_id, format, status, source_path, created_at, completed_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            Uuid::new_v4().to_string(),
            business_id,
            format,
            status,
            source_path,
            now_iso(),
            Option::<String>::None
        ],
    )
    .map_err(|error| to_command_error("failed to insert import job", error))?;
    Ok(())
}

pub fn build_storage_status(conn: &Connection, paths: &AppPaths) -> Result<StorageStatus, String> {
    let export_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM export_jobs", [], |row| row.get(0))
        .map_err(|error| to_command_error("failed to count export jobs", error))?;

    let backup_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM backup_records", [], |row| row.get(0))
        .map_err(|error| to_command_error("failed to count backup records", error))?;

    Ok(StorageStatus {
        data_dir: paths.data_dir.clone(),
        config_dir: paths.config_dir.clone(),
        log_dir: paths.log_dir.clone(),
        backup_dir: paths.backup_dir.clone(),
        export_dir: paths.export_dir.clone(),
        database_path: paths.database_path.clone(),
        database_exists: fs::metadata(&paths.database_path).is_ok(),
        backup_count: backup_count as usize,
        export_count: export_count as usize,
    })
}

pub fn load_app_info(conn: &Connection) -> Result<AppInfo, String> {
    Ok(AppInfo {
        app_name: get_meta(conn, "app_name")?.unwrap_or_else(|| "local-first-business-manager".into()),
        product_name: get_meta(conn, "product_name")?.unwrap_or_else(|| "Local Business Manager".into()),
        version: get_meta(conn, "app_version")?.unwrap_or_else(|| "0.1.0".into()),
        schema_version: get_meta(conn, "schema_version")?
            .and_then(|value| value.parse::<i64>().ok())
            .unwrap_or(CURRENT_SCHEMA_VERSION),
        patch_level: get_meta(conn, "patch_level")?.unwrap_or_else(|| "P001_foundation_base_structure".into()),
        initialized_at: get_meta(conn, "initialized_at")?.unwrap_or_else(now_iso),
    })
}

```

# files/src-tauri/src/core/error.rs

```rust
pub type CommandResult<T> = Result<T, String>;

pub fn to_command_error<E: std::fmt::Display>(context: &str, error: E) -> String {
    format!("{context}: {error}")
}

```

# files/src-tauri/src/core/migrations/001_base.sql

```sql
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

```

# files/src-tauri/src/core/migrations.rs

```rust
use rusqlite::Connection;

pub const CURRENT_SCHEMA_VERSION: i64 = 1;

const MIGRATION_001: &str = include_str!("migrations/001_base.sql");

pub fn run(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(MIGRATION_001)
}

```

# files/src-tauri/src/core/mod.rs

```rust
pub mod db;
pub mod error;
pub mod migrations;
pub mod patching;
pub mod paths;
pub mod seed;

```

# files/src-tauri/src/core/patching.rs

```rust
use chrono::Utc;
use rusqlite::{params, Connection};
use serde_json::json;

use super::migrations::CURRENT_SCHEMA_VERSION;

pub const PATCH_ID: &str = "P001_foundation_base_structure";
pub const PATCH_NAME: &str = "Foundation Base Structure";

pub fn register_patch(conn: &Connection) -> rusqlite::Result<()> {
    let now = Utc::now().to_rfc3339();
    let manifest = json!({
        "patch_id": PATCH_ID,
        "patch_name": PATCH_NAME,
        "schema_version": CURRENT_SCHEMA_VERSION,
        "applied_at": now.clone(),
        "notes": "Initial desktop foundation with local storage, business/settings shell, backup/export foundation."
    });

    conn.execute(
        "INSERT OR IGNORE INTO patch_history (patch_id, patch_name, schema_version, applied_at, manifest_json) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![PATCH_ID, PATCH_NAME, CURRENT_SCHEMA_VERSION, now, manifest.to_string()],
    )?;

    conn.execute(
        "INSERT INTO app_meta (key, value, updated_at)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
        params!["schema_version", CURRENT_SCHEMA_VERSION.to_string(), Utc::now().to_rfc3339()],
    )?;

    conn.execute(
        "INSERT INTO app_meta (key, value, updated_at)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
        params!["patch_level", PATCH_ID, Utc::now().to_rfc3339()],
    )?;

    Ok(())
}

```

# files/src-tauri/src/core/paths.rs

```rust
use std::fs;
use std::path::PathBuf;

use serde::Serialize;
use tauri::{AppHandle, Manager};

use super::error::to_command_error;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppPaths {
    pub data_dir: String,
    pub config_dir: String,
    pub log_dir: String,
    pub backup_dir: String,
    pub export_dir: String,
    pub database_path: String,
}

impl AppPaths {
    pub fn database_path_buf(&self) -> PathBuf {
        PathBuf::from(&self.database_path)
    }
}

pub fn resolve_paths(app: &AppHandle) -> Result<AppPaths, String> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| to_command_error("failed to resolve app data directory", error))?;
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|error| to_command_error("failed to resolve app config directory", error))?;
    let log_dir = app
        .path()
        .app_log_dir()
        .map_err(|error| to_command_error("failed to resolve app log directory", error))?;

    let backup_dir = data_dir.join("backups");
    let export_dir = data_dir.join("exports");
    let database_path = data_dir.join("local_business_manager.sqlite");

    Ok(AppPaths {
        data_dir: data_dir.to_string_lossy().to_string(),
        config_dir: config_dir.to_string_lossy().to_string(),
        log_dir: log_dir.to_string_lossy().to_string(),
        backup_dir: backup_dir.to_string_lossy().to_string(),
        export_dir: export_dir.to_string_lossy().to_string(),
        database_path: database_path.to_string_lossy().to_string(),
    })
}

pub fn ensure_directories(paths: &AppPaths) -> Result<(), String> {
    for path in [
        &paths.data_dir,
        &paths.config_dir,
        &paths.log_dir,
        &paths.backup_dir,
        &paths.export_dir,
    ] {
        fs::create_dir_all(path)
            .map_err(|error| to_command_error("failed to create local storage directory", error))?;
    }
    Ok(())
}

```

# files/src-tauri/src/core/seed.rs

```rust
use chrono::Utc;
use rusqlite::{params, Connection};
use uuid::Uuid;

fn upsert_meta(conn: &Connection, key: &str, value: &str) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO app_meta (key, value, updated_at)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
        params![key, value, Utc::now().to_rfc3339()],
    )?;
    Ok(())
}

fn insert_meta_if_missing(conn: &Connection, key: &str, value: &str) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO app_meta (key, value, updated_at)
         VALUES (?1, ?2, ?3)",
        params![key, value, Utc::now().to_rfc3339()],
    )?;
    Ok(())
}

pub fn seed_if_empty(conn: &Connection) -> rusqlite::Result<()> {
    let business_count: i64 =
        conn.query_row("SELECT COUNT(*) FROM businesses", [], |row| row.get(0))?;

    upsert_meta(conn, "app_name", "local-first-business-manager")?;
    upsert_meta(conn, "product_name", "Local Business Manager")?;
    upsert_meta(conn, "app_version", "0.1.0")?;
    insert_meta_if_missing(conn, "initialized_at", &Utc::now().to_rfc3339())?;

    if business_count == 0 {
        let business_id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO businesses (
                id, name, legal_name, code, business_type, currency_code, tax_mode,
                phone, email, address_line1, address_line2, city, state, postal_code,
                country, created_at, updated_at, archived_at
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7,
                ?8, ?9, ?10, ?11, ?12, ?13, ?14,
                ?15, ?16, ?17, ?18
            )",
            params![
                &business_id,
                "Demo Cafe & Retail",
                "Demo Cafe & Retail LLP",
                "DEMO-001",
                "Cafe",
                "INR",
                "exclusive",
                "+91-90000-00000",
                "demo@localbusiness.test",
                "12 Market Street",
                "Near Central Square",
                "Bengaluru",
                "Karnataka",
                "560001",
                "India",
                &now,
                &now,
                Option::<String>::None
            ],
        )?;

        conn.execute(
            "INSERT INTO business_settings (
                business_id, timezone, locale, date_format, theme, tax_label,
                default_tax_rate, prices_include_tax, receipt_footer,
                receipt_show_address, receipt_show_phone, auto_backup_enabled,
                backup_directory, module_restaurant_enabled, module_retail_enabled,
                module_inventory_enabled, module_services_enabled, updated_at
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6,
                ?7, ?8, ?9,
                ?10, ?11, ?12,
                ?13, ?14, ?15,
                ?16, ?17, ?18
            )",
            params![
                &business_id,
                "Asia/Kolkata",
                "en-IN",
                "DD-MM-YYYY",
                "system",
                "GST",
                5.0_f64,
                0_i64,
                "Thank you for supporting local business.",
                1_i64,
                1_i64,
                0_i64,
                Option::<String>::None,
                1_i64,
                1_i64,
                1_i64,
                0_i64,
                &now
            ],
        )?;

        upsert_meta(conn, "active_business_id", &business_id)?;
        upsert_meta(conn, "seeded_demo_data", "true")?;

        for (level, category, message) in [
            ("INFO", "patching", "Patch 1 foundation registered"),
            ("INFO", "business", "Demo business profile created"),
            ("INFO", "storage", "Local storage directories prepared"),
        ] {
            conn.execute(
                "INSERT INTO app_logs (id, level, category, message, payload_json, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    Uuid::new_v4().to_string(),
                    level,
                    category,
                    message,
                    Option::<String>::None,
                    Utc::now().to_rfc3339()
                ],
            )?;
        }
    }

    Ok(())
}

```

# files/src-tauri/src/domain/bootstrap.rs

```rust
use super::models::{
    AppBootstrap, AppInfo, BackupRecord, BusinessProfile, BusinessSettings, DashboardShellData,
    KpiCard, ModuleStatus, PatchRecord, RecentActivity, StorageStatus,
};

pub fn compose_dashboard(
    businesses: &[BusinessProfile],
    backups: &[BackupRecord],
    recent_activity: Vec<RecentActivity>,
    patch_history: &[PatchRecord],
    storage: &StorageStatus,
) -> DashboardShellData {
    DashboardShellData {
        hero_title: "Local-first foundation ready".into(),
        hero_body: "Patch 1 establishes the desktop shell, local database, business profile, settings, backup/export foundations, and a patch registry for future modules.".into(),
        kpis: vec![
            KpiCard {
                id: "business-count".into(),
                label: "Businesses in storage".into(),
                value: businesses.len().to_string(),
                note: "Patch 1 uses a single active business but stores data in a future-ready table.".into(),
            },
            KpiCard {
                id: "backup-count".into(),
                label: "Backup snapshots".into(),
                value: backups.len().to_string(),
                note: "Snapshots are stored locally and tracked in SQLite.".into(),
            },
            KpiCard {
                id: "patch-count".into(),
                label: "Applied patches".into(),
                value: patch_history.len().to_string(),
                note: "Patch history is stored locally to support incremental evolution.".into(),
            },
            KpiCard {
                id: "export-count".into(),
                label: "Exports recorded".into(),
                value: storage.export_count.to_string(),
                note: "Data portability foundation is active from Patch 1.".into(),
            },
        ],
        recent_activity,
        module_statuses: vec![
            ModuleStatus {
                id: "core".into(),
                label: "Foundation shell".into(),
                status: "active-foundation".into(),
                note: "Desktop shell, navigation, settings, and storage are already present.".into(),
            },
            ModuleStatus {
                id: "catalog".into(),
                label: "Catalog / items".into(),
                status: "coming-next".into(),
                note: "Planned as a follow-on patch from this database and navigation base.".into(),
            },
            ModuleStatus {
                id: "pos".into(),
                label: "POS / billing".into(),
                status: "planned".into(),
                note: "Checkout flow intentionally deferred to a later patch.".into(),
            },
            ModuleStatus {
                id: "inventory".into(),
                label: "Inventory".into(),
                status: "planned".into(),
                note: "Stock ledger and movement history are not part of Patch 1 yet.".into(),
            },
            ModuleStatus {
                id: "reports".into(),
                label: "Reports".into(),
                status: "planned".into(),
                note: "Reporting surfaces will build on top of the foundation schema later.".into(),
            },
        ],
    }
}

#[allow(clippy::too_many_arguments)]
pub fn build_app_bootstrap(
    app_info: AppInfo,
    active_business: BusinessProfile,
    business_settings: BusinessSettings,
    businesses: Vec<BusinessProfile>,
    patch_history: Vec<PatchRecord>,
    backups: Vec<BackupRecord>,
    storage: StorageStatus,
    recent_activity: Vec<RecentActivity>,
) -> AppBootstrap {
    let dashboard = compose_dashboard(
        &businesses,
        &backups,
        recent_activity,
        &patch_history,
        &storage,
    );

    AppBootstrap {
        app_info,
        active_business,
        business_settings,
        businesses,
        patch_history,
        backups,
        storage,
        dashboard,
    }
}

```

# files/src-tauri/src/domain/mod.rs

```rust
pub mod bootstrap;
pub mod models;

```

# files/src-tauri/src/domain/models.rs

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessProfile {
    pub id: String,
    pub name: String,
    pub legal_name: Option<String>,
    pub code: String,
    pub business_type: String,
    pub currency_code: String,
    pub tax_mode: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessSettings {
    pub business_id: String,
    pub timezone: String,
    pub locale: String,
    pub date_format: String,
    pub theme: String,
    pub tax_label: String,
    pub default_tax_rate: f64,
    pub prices_include_tax: bool,
    pub receipt_footer: Option<String>,
    pub receipt_show_address: bool,
    pub receipt_show_phone: bool,
    pub auto_backup_enabled: bool,
    pub backup_directory: Option<String>,
    pub module_restaurant_enabled: bool,
    pub module_retail_enabled: bool,
    pub module_inventory_enabled: bool,
    pub module_services_enabled: bool,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub app_name: String,
    pub product_name: String,
    pub version: String,
    pub schema_version: i64,
    pub patch_level: String,
    pub initialized_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchRecord {
    pub patch_id: String,
    pub patch_name: String,
    pub schema_version: i64,
    pub applied_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupRecord {
    pub id: String,
    pub business_id: Option<String>,
    pub file_name: String,
    pub file_path: String,
    pub checksum: Option<String>,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportJobRecord {
    pub id: String,
    pub business_id: Option<String>,
    pub format: String,
    pub status: String,
    pub target_path: Option<String>,
    pub created_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportPreview {
    pub file_path: String,
    pub valid: bool,
    pub manifest_version: Option<String>,
    pub bundle_type: Option<String>,
    pub source_patch_level: Option<String>,
    pub business_count: usize,
    pub generated_at: Option<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentActivity {
    pub id: String,
    pub level: String,
    pub category: String,
    pub message: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KpiCard {
    pub id: String,
    pub label: String,
    pub value: String,
    pub note: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleStatus {
    pub id: String,
    pub label: String,
    pub status: String,
    pub note: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardShellData {
    pub hero_title: String,
    pub hero_body: String,
    pub kpis: Vec<KpiCard>,
    pub recent_activity: Vec<RecentActivity>,
    pub module_statuses: Vec<ModuleStatus>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageStatus {
    pub data_dir: String,
    pub config_dir: String,
    pub log_dir: String,
    pub backup_dir: String,
    pub export_dir: String,
    pub database_path: String,
    pub database_exists: bool,
    pub backup_count: usize,
    pub export_count: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBootstrap {
    pub app_info: AppInfo,
    pub active_business: BusinessProfile,
    pub business_settings: BusinessSettings,
    pub businesses: Vec<BusinessProfile>,
    pub patch_history: Vec<PatchRecord>,
    pub backups: Vec<BackupRecord>,
    pub storage: StorageStatus,
    pub dashboard: DashboardShellData,
}

```

# files/src-tauri/src/lib.rs

```rust
mod commands;
mod core;
mod domain;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            core::db::initialize(&handle).expect("failed to initialize local storage foundation");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::bootstrap::bootstrap_app,
            commands::business::save_business_profile,
            commands::settings::save_business_settings,
            commands::data_center::create_backup_snapshot,
            commands::data_center::export_foundation_snapshot,
            commands::data_center::preview_import_bundle
        ])
        .run(tauri::generate_context!())
        .expect("error while running local business manager");
}

```

# files/src-tauri/src/main.rs

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    local_business_manager::run();
}

```

# files/src-tauri/tauri.conf.json

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "Local Business Manager",
  "version": "0.1.0",
  "identifier": "com.localfirst.businessmanager",
  "build": {
    "beforeBuildCommand": "npm run build",
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "label": "main",
        "title": "Local Business Manager",
        "width": 1380,
        "height": 860,
        "minWidth": 1100,
        "minHeight": 720,
        "resizable": true
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": false
  }
}

```

# files/tsconfig.json

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["DOM", "DOM.Iterable", "ES2020"],
    "allowJs": false,
    "skipLibCheck": true,
    "esModuleInterop": true,
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx"
  },
  "include": ["src"],
  "references": [{ "path": "./tsconfig.node.json" }]
}

```

# files/tsconfig.node.json

```json
{
  "compilerOptions": {
    "composite": true,
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "allowSyntheticDefaultImports": true
  },
  "include": ["vite.config.ts"]
}

```

# files/vite.config.ts

```ts
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  clearScreen: false,
  plugins: [react()],
  server: {
    port: 5173,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421
        }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"]
    }
  },
  envPrefix: ["VITE_", "TAURI_ENV_*"],
  build: {
    target:
      process.env.TAURI_ENV_PLATFORM === "windows" ? "chrome105" : "safari13",
    minify: !process.env.TAURI_ENV_DEBUG ? "esbuild" : false,
    sourcemap: !!process.env.TAURI_ENV_DEBUG
  }
});

```

# migrations/MIGRATION_NOTES.md

```md
# Migration Notes — Patch 1

## Database migration impact
This is the initial schema bootstrap.

## Applied schema artifacts
- `001_base.sql`

## Existing data impact
- None expected when applying to an empty/new project.
- If forcing over an existing directory, no automatic merge is performed.

## First-run initialization
On first application launch, the backend:
1. resolves local app directories
2. creates required folders
3. creates the SQLite database
4. applies the base migration
5. records patch metadata
6. seeds demo business/settings/activity if no business exists

```

# patch-manifest.json

```json
{
  "patch_id": "P001_foundation_base_structure",
  "patch_name": "Foundation Base Structure",
  "base_version": "0.0.0",
  "target_version": "0.1.0-foundation",
  "description": "Initial local-first desktop app foundation with Tauri, React, SQLite, business profile/settings shell, backup/export foundations, and patch-ready architecture.",
  "dependencies": [],
  "safe_on_empty_project": true,
  "migration_required": true,
  "rollback_supported": true,
  "files_root": "files",
  "post_apply_steps": [
    "cd <target>",
    "npm install",
    "npm run tauri dev"
  ]
}

```

# rollback.md

```md
# Rollback Notes — Patch 1

## If the patch was applied to an empty project
Delete the target project directory.

## If the patch was applied with `--force`
Restore files from:
```text
.patch-backups/P001_foundation_base_structure/
```

## If you also want to remove local seeded data
Delete the app data directory created by Tauri for the app identifier:
```text
com.localfirst.businessmanager
```

That removes:
- the SQLite database
- export files
- backup files
- runtime logs

```

# validate.md

```md
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

```
