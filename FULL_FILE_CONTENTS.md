# FULL_FILE_CONTENTS

## `PATCH-3-README.md`

```md
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
```

## `PATCH_NOTES.md`

```md
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
```

## `README.md`

```md
See `PATCH-3-README.md` for full patch instructions.
```

## `TREE.txt`

```text
PATCH-3-README.md
PATCH_NOTES.md
README.md
TREE.txt
apply_patch.mjs
apply_patch.ps1
apply_patch.sh
files/package.json
files/scripts/validate-patch3.mjs
files/src-tauri/Cargo.toml
files/src-tauri/src/commands/bootstrap.rs
files/src-tauri/src/commands/catalog.rs
files/src-tauri/src/commands/data_center.rs
files/src-tauri/src/commands/mod.rs
files/src-tauri/src/core/catalog.rs
files/src-tauri/src/core/db.rs
files/src-tauri/src/core/migrations.rs
files/src-tauri/src/core/migrations/003_catalog_core.sql
files/src-tauri/src/core/mod.rs
files/src-tauri/src/core/patching.rs
files/src-tauri/src/core/seed.rs
files/src-tauri/src/domain/bootstrap.rs
files/src-tauri/src/domain/models.rs
files/src-tauri/src/lib.rs
files/src-tauri/tauri.conf.json
files/src/app/AppProvider.tsx
files/src/modules/business/BusinessPage.tsx
files/src/modules/catalog/CatalogPage.tsx
files/src/modules/dashboard/DashboardPage.tsx
files/src/modules/data-center/DataCenterPage.tsx
files/src/modules/settings/SettingsPage.tsx
files/src/modules/shell/AppShell.tsx
files/src/shared/api.ts
files/src/shared/types.ts
files/src/shared/utils.ts
files/src/styles.css
migrations/MIGRATION_NOTES.md
patch-manifest.json
rollback.md
validate.md
```

## `apply_patch.mjs`

```js
import fs from "node:fs";
import path from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const manifestPath = path.join(__dirname, "patch-manifest.json");
const filesDir = path.join(__dirname, "files");
const manifest = JSON.parse(fs.readFileSync(manifestPath, "utf8"));

const args = process.argv.slice(2);
const force = args.includes("--force");
const install = args.includes("--install");
const targetArg = args.find((arg) => !arg.startsWith("--"));
const targetPath = path.resolve(targetArg || process.cwd());
const patchBackupRoot = path.join(targetPath, ".patch-backups", manifest.patch_id);
const patchMetaRoot = path.join(targetPath, ".patch-meta");

function ensureDir(dir) {
  fs.mkdirSync(dir, { recursive: true });
}

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

function readFileOrNull(filePath) {
  if (!fs.existsSync(filePath)) return null;
  return fs.readFileSync(filePath);
}

function sameFile(a, b) {
  if (a === null || b === null) return false;
  return Buffer.compare(a, b) === 0;
}

function validatePatch2Base() {
  const packageJsonPath = path.join(targetPath, "package.json");
  const patchingPath = path.join(targetPath, "src-tauri", "src", "core", "patching.rs");
  const migration2Path = path.join(
    targetPath,
    "src-tauri",
    "src",
    "core",
    "migrations",
    "002_multi_business_workspace.sql"
  );

  if (!fs.existsSync(packageJsonPath) || !fs.existsSync(patchingPath) || !fs.existsSync(migration2Path)) {
    throw new Error(
      "Patch 3 expects an existing Patch 2 project. Missing package.json, patching.rs, or migration 002."
    );
  }

  const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));
  const patchingSource = fs.readFileSync(patchingPath, "utf8");
  const looksLikePatch2 =
    String(packageJson.version || "").startsWith("0.2") ||
    patchingSource.includes("P002_multi_business_workspace_settings_core");

  if (!looksLikePatch2 && !force) {
    throw new Error(
      "Target project does not look like a Patch 2 base. Re-run with --force only if you are sure."
    );
  }
}

function backupExistingFile(relativePath, existingContents) {
  const backupPath = path.join(patchBackupRoot, relativePath);
  ensureDir(path.dirname(backupPath));
  fs.writeFileSync(backupPath, existingContents);
}

function copyFile(relativePath) {
  const source = path.join(filesDir, relativePath);
  const target = path.join(targetPath, relativePath);
  ensureDir(path.dirname(target));

  const incoming = fs.readFileSync(source);
  const existing = readFileOrNull(target);

  if (existing !== null && !sameFile(existing, incoming)) {
    backupExistingFile(relativePath, existing);
  }

  fs.writeFileSync(target, incoming);
}

function loadAppliedPatches(metaFile) {
  if (!fs.existsSync(metaFile)) return [];
  try {
    const parsed = JSON.parse(fs.readFileSync(metaFile, "utf8"));
    return Array.isArray(parsed) ? parsed : [];
  } catch {
    return [];
  }
}

function maybeRunInstall() {
  if (!install) return;

  const npmCommand = process.platform === "win32" ? "npm.cmd" : "npm";
  const result = spawnSync(npmCommand, ["install"], {
    cwd: targetPath,
    stdio: "inherit",
    shell: false
  });

  if (result.status !== 0) {
    throw new Error("npm install failed after applying the patch");
  }
}

function main() {
  if (!fs.existsSync(filesDir)) {
    throw new Error(`Patch bundle is missing files directory: ${filesDir}`);
  }

  ensureDir(targetPath);
  ensureDir(patchMetaRoot);
  validatePatch2Base();

  const fileList = listFilesRecursively(filesDir);
  for (const relativePath of fileList) {
    copyFile(relativePath);
  }

  const appliedPatchesFile = path.join(patchMetaRoot, "applied-patches.json");
  const appliedPatches = loadAppliedPatches(appliedPatchesFile);
  appliedPatches.push({
    patchId: manifest.patch_id,
    patchName: manifest.patch_name,
    appliedAt: new Date().toISOString(),
    fileCount: fileList.length,
    dependencies: manifest.dependencies
  });
  fs.writeFileSync(appliedPatchesFile, JSON.stringify(appliedPatches, null, 2) + "\n");

  fs.writeFileSync(
    path.join(patchMetaRoot, "last-applied.json"),
    JSON.stringify(
      {
        patchId: manifest.patch_id,
        patchName: manifest.patch_name,
        targetPath,
        appliedAt: new Date().toISOString(),
        fileCount: fileList.length,
        installRan: install
      },
      null,
      2
    ) + "\n"
  );

  maybeRunInstall();

  console.log(`[OK] Applied ${manifest.patch_id} to ${targetPath}`);
  console.log(`[INFO] Files copied: ${fileList.length}`);
  console.log(`[INFO] Backups stored in ${patchBackupRoot}`);
}

try {
  main();
} catch (error) {
  console.error(`[ERROR] ${error instanceof Error ? error.message : String(error)}`);
  process.exit(1);
}
```

## `apply_patch.ps1`

```powershell
param(
  [string]$TargetDir = "."
)

$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
node "$scriptRoot/apply_patch.mjs" $TargetDir --install
```

## `apply_patch.sh`

```bash
#!/usr/bin/env bash
set -euo pipefail

TARGET_DIR="${1:-.}"
node "$(cd "$(dirname "$0")" && pwd)/apply_patch.mjs" "$TARGET_DIR" --install
```

## `files/package.json`

```json
{
  "name": "local-first-business-manager",
  "private": true,
  "version": "0.3.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "tauri": "tauri",
    "typecheck": "tsc --noEmit",
    "validate:patch3": "node scripts/validate-patch3.mjs"
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
    "@vitejs/plugin-react": "^4.3.3",
    "typescript": "^5.6.3",
    "vite": "^5.4.10"
  }
}
```

## `files/scripts/validate-patch3.mjs`

```js
import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const ts = require("typescript");

const requiredFiles = [
  "package.json",
  "src/shared/types.ts",
  "src/shared/api.ts",
  "src/modules/catalog/CatalogPage.tsx",
  "src/modules/shell/AppShell.tsx",
  "src-tauri/src/core/catalog.rs",
  "src-tauri/src/core/migrations/003_catalog_core.sql",
  "src-tauri/src/commands/catalog.rs",
  "src-tauri/src/lib.rs"
];

const requiredSnippets = [
  ["package.json", '"validate:patch3": "node scripts/validate-patch3.mjs"'],
  ["src/shared/types.ts", "export interface CatalogWorkspace"],
  ["src/shared/types.ts", '  | "catalog"'],
  ["src/shared/api.ts", 'invoke<CatalogWorkspace>("load_catalog_workspace")'],
  ["src/modules/catalog/CatalogPage.tsx", "Patch 3 catalog core"],
  ["src/modules/shell/AppShell.tsx", 'key: "catalog"'],
  ["src-tauri/src/core/migrations.rs", "CURRENT_SCHEMA_VERSION: i64 = 3"],
  ["src-tauri/src/core/patching.rs", "P003_catalog_core"],
  ["src-tauri/src/lib.rs", "commands::catalog::load_catalog_workspace"],
  ["src-tauri/src/lib.rs", "commands::catalog::save_catalog_item"]
];

let hasError = false;

for (const relativePath of requiredFiles) {
  const fullPath = path.resolve(relativePath);
  if (!fs.existsSync(fullPath)) {
    console.error(`[ERROR] Missing required file: ${relativePath}`);
    hasError = true;
  }
}

for (const [relativePath, snippet] of requiredSnippets) {
  const fullPath = path.resolve(relativePath);
  if (!fs.existsSync(fullPath)) continue;
  const contents = fs.readFileSync(fullPath, "utf8");
  if (!contents.includes(snippet)) {
    console.error(`[ERROR] Expected snippet not found in ${relativePath}: ${snippet}`);
    hasError = true;
  }
}

function walk(dir) {
  const output = [];
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      output.push(...walk(fullPath));
      continue;
    }
    if (/\.(ts|tsx)$/.test(entry.name)) {
      output.push(fullPath);
    }
  }
  return output;
}

for (const fullPath of walk(path.resolve("src"))) {
  const source = fs.readFileSync(fullPath, "utf8");
  const scriptKind = fullPath.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS;
  const sourceFile = ts.createSourceFile(fullPath, source, ts.ScriptTarget.Latest, true, scriptKind);
  if (sourceFile.parseDiagnostics.length > 0) {
    hasError = true;
    console.error(`[ERROR] TypeScript parse diagnostics in ${path.relative(process.cwd(), fullPath)}`);
    for (const diagnostic of sourceFile.parseDiagnostics) {
      console.error(ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n"));
    }
  }
}

const migrationSql = fs.readFileSync(path.resolve("src-tauri/src/core/migrations/003_catalog_core.sql"), "utf8");
for (const token of [
  "catalog_categories",
  "catalog_units",
  "catalog_items",
  "catalog_item_barcodes",
  "prices_include_tax",
  "show_email",
  "show_business_code",
  "reset_policy"
]) {
  if (!migrationSql.includes(token)) {
    console.error(`[ERROR] Migration 003 is missing expected token: ${token}`);
    hasError = true;
  }
}

if (hasError) {
  process.exit(1);
}

console.log("[OK] Patch 3 structural validation passed.");
```

## `files/src/app/AppProvider.tsx`

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
  createBusinessWorkspace,
  exportFoundationSnapshot,
  previewImportBundle,
  saveBusinessProfile,
  saveWorkspaceConfiguration,
  switchActiveBusiness
} from "../shared/api";
import type {
  AppBootstrap,
  BackupRecord,
  BusinessProfile,
  ImportPreview,
  NewBusinessWorkspaceInput,
  WorkspaceConfigurationInput
} from "../shared/types";

type AppStatus = "loading" | "ready" | "error";

interface AppContextValue {
  status: AppStatus;
  errorMessage: string | null;
  data: AppBootstrap | null;
  refresh: () => Promise<void>;
  saveProfile: (profile: BusinessProfile) => Promise<BusinessProfile>;
  createBusiness: (input: NewBusinessWorkspaceInput) => Promise<BusinessProfile>;
  switchBusiness: (businessId: string) => Promise<BusinessProfile>;
  saveWorkspace: (input: WorkspaceConfigurationInput) => Promise<void>;
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

  const createBusiness = useCallback(
    async (input: NewBusinessWorkspaceInput) => {
      const created = await createBusinessWorkspace(input);
      await refresh();
      return created;
    },
    [refresh]
  );

  const switchBusiness = useCallback(
    async (businessId: string) => {
      const result = await switchActiveBusiness(businessId);
      await refresh();
      return result;
    },
    [refresh]
  );

  const saveWorkspace = useCallback(
    async (input: WorkspaceConfigurationInput) => {
      await saveWorkspaceConfiguration(input);
      await refresh();
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
      createBusiness,
      switchBusiness,
      saveWorkspace,
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
      createBusiness,
      switchBusiness,
      saveWorkspace,
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
        <p>
          Initializing migrations, business workspaces, settings profiles,
          catalog foundations, and the local patch registry.
        </p>
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

## `files/src/modules/business/BusinessPage.tsx`

```tsx
import { useEffect, useMemo, useState } from "react";
import { useAppState } from "../../app/AppProvider";
import type {
  BusinessProfile,
  NewBusinessWorkspaceInput
} from "../../shared/types";
import {
  formatDateTime,
  formatModuleList,
  titleCaseWords
} from "../../shared/utils";

const emptyCreateForm: NewBusinessWorkspaceInput = {
  name: "",
  legalName: null,
  code: "",
  businessType: "Retail",
  currencyCode: "INR",
  taxMode: "exclusive",
  timezone: "Asia/Kolkata",
  locale: "en-IN",
  activateNow: true
};

export function BusinessPage() {
  const { data, saveProfile, createBusiness, switchBusiness } = useAppState();
  const [form, setForm] = useState<BusinessProfile | null>(null);
  const [createForm, setCreateForm] =
    useState<NewBusinessWorkspaceInput>(emptyCreateForm);
  const [statusMessage, setStatusMessage] = useState<string>("");
  const [createStatus, setCreateStatus] = useState<string>("");
  const [switchStatus, setSwitchStatus] = useState<string>("");

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

  function update<K extends keyof BusinessProfile>(
    key: K,
    value: BusinessProfile[K]
  ) {
    setForm((current) => (current ? { ...current, [key]: value } : current));
  }

  function updateCreate<K extends keyof NewBusinessWorkspaceInput>(
    key: K,
    value: NewBusinessWorkspaceInput[K]
  ) {
    setCreateForm((current) => ({ ...current, [key]: value }));
  }

  async function handleSave() {
    setStatusMessage("Saving business profile…");
    try {
      await saveProfile(form);
      setStatusMessage("Active business profile saved locally.");
    } catch (error) {
      setStatusMessage(
        error instanceof Error ? error.message : "Failed to save profile."
      );
    }
  }

  async function handleCreate() {
    setCreateStatus("Creating business workspace…");
    try {
      const created = await createBusiness({
        ...createForm,
        code: createForm.code.toUpperCase(),
        currencyCode: createForm.currencyCode.toUpperCase(),
        legalName:
          createForm.legalName && createForm.legalName.trim()
            ? createForm.legalName
            : null
      });
      setCreateForm({
        ...emptyCreateForm,
        currencyCode: created.currencyCode,
        timezone: data.businessSettings.timezone,
        locale: data.businessSettings.locale
      });
      setCreateStatus(
        `Workspace created: ${created.name} (${created.code}).`
      );
    } catch (error) {
      setCreateStatus(
        error instanceof Error ? error.message : "Failed to create workspace."
      );
    }
  }

  async function handleSwitch(businessId: string) {
    if (businessId === data.activeBusiness.id) {
      return;
    }

    setSwitchStatus("Switching active business…");
    try {
      const switched = await switchBusiness(businessId);
      setSwitchStatus(`Switched to ${switched.name}.`);
    } catch (error) {
      setSwitchStatus(
        error instanceof Error ? error.message : "Failed to switch workspace."
      );
    }
  }

  return (
    <div className="page-grid">
      <section className="split-grid">
        <article className="card">
          <div className="card-header">
            <h2>Business workspace directory</h2>
            <span className="pill success">
              {data.businessWorkspaces.length} total
            </span>
          </div>
          <p className="card-note">
            Every business keeps its own settings, tax profile, receipt profile,
            and sequence counters. Future POS, item, and inventory data will use
            the active business boundary.
          </p>

          <div className="directory-grid">
            {data.businessWorkspaces.map((workspace) => (
              <article
                key={workspace.businessId}
                className={`workspace-directory-card ${
                  workspace.businessId === data.activeBusiness.id ? "active" : ""
                }`}
              >
                <div className="card-header compact-card-header">
                  <div>
                    <h3>{workspace.name}</h3>
                    <div className="muted-text">
                      {workspace.code} · {workspace.businessType} · {workspace.currencyCode}
                    </div>
                  </div>
                  <span
                    className={`pill ${
                      workspace.businessId === data.activeBusiness.id
                        ? "success"
                        : "neutral"
                    }`}
                  >
                    {workspace.businessId === data.activeBusiness.id
                      ? "Active"
                      : "Available"}
                  </span>
                </div>

                <div className="detail-list compact-detail-list">
                  <div>
                    <span>Timezone</span>
                    <code>{workspace.timezone}</code>
                  </div>
                  <div>
                    <span>Tax profile</span>
                    <code>
                      {workspace.taxLabel} · {workspace.defaultTaxRate}%
                    </code>
                  </div>
                  <div>
                    <span>Next sale number</span>
                    <code>{workspace.nextSaleSequence}</code>
                  </div>
                  <div>
                    <span>Modules</span>
                    <code>{formatModuleList(workspace.activeModules)}</code>
                  </div>
                  <div>
                    <span>Updated</span>
                    <code>{formatDateTime(workspace.updatedAt)}</code>
                  </div>
                </div>

                <div className="tag-list compact-tags">
                  {workspace.activeModules.map((module) => (
                    <span className="tag" key={`${workspace.businessId}-${module}`}>
                      {titleCaseWords(module)}
                    </span>
                  ))}
                </div>

                <div className="inline-actions">
                  <button
                    className="secondary-button"
                    type="button"
                    disabled={workspace.businessId === data.activeBusiness.id}
                    onClick={() => void handleSwitch(workspace.businessId)}
                  >
                    {workspace.businessId === data.activeBusiness.id
                      ? "Already active"
                      : "Switch here"}
                  </button>
                </div>
              </article>
            ))}
          </div>

          <div className="status-banner">
            {switchStatus || "Switching updates the active workspace immediately."}
          </div>
        </article>

        <article className="card">
          <div className="card-header">
            <h2>Create Business Workspace</h2>
            <span className="pill warning">Patch 3</span>
          </div>
          <p className="card-note">
            Add another business profile with isolated local settings. The catalog
            foundation in Patch 3 can now attach categories, units, and items to it,
            while deeper sales and inventory flows arrive in later patches.
          </p>

          <div className="form-grid">
            <label>
              <span>Business name</span>
              <input
                value={createForm.name}
                onChange={(event) => updateCreate("name", event.target.value)}
              />
            </label>

            <label>
              <span>Legal name</span>
              <input
                value={createForm.legalName ?? ""}
                onChange={(event) => updateCreate("legalName", event.target.value)}
              />
            </label>

            <label>
              <span>Business code</span>
              <input
                value={createForm.code}
                onChange={(event) =>
                  updateCreate("code", event.target.value.toUpperCase())
                }
              />
            </label>

            <label>
              <span>Business type</span>
              <select
                value={createForm.businessType}
                onChange={(event) =>
                  updateCreate("businessType", event.target.value)
                }
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
                value={createForm.currencyCode}
                onChange={(event) =>
                  updateCreate("currencyCode", event.target.value.toUpperCase())
                }
              />
            </label>

            <label>
              <span>Tax mode</span>
              <select
                value={createForm.taxMode}
                onChange={(event) => updateCreate("taxMode", event.target.value)}
              >
                <option value="exclusive">Exclusive</option>
                <option value="inclusive">Inclusive</option>
                <option value="none">No tax</option>
              </select>
            </label>

            <label>
              <span>Timezone</span>
              <input
                value={createForm.timezone}
                onChange={(event) => updateCreate("timezone", event.target.value)}
              />
            </label>

            <label>
              <span>Locale</span>
              <input
                value={createForm.locale}
                onChange={(event) => updateCreate("locale", event.target.value)}
              />
            </label>

            <label className="toggle-row form-span-2">
              <input
                type="checkbox"
                checked={createForm.activateNow}
                onChange={(event) =>
                  updateCreate("activateNow", event.target.checked)
                }
              />
              <span>Make the new business active immediately</span>
            </label>
          </div>

          <div className="inline-actions">
            <button className="primary-button" type="button" onClick={() => void handleCreate()}>
              Create Workspace
            </button>
            <span className="muted-text">{createStatus}</span>
          </div>
        </article>
      </section>

      <section className="card">
        <div className="card-header">
          <h2>Active business profile</h2>
          <span className="pill success">{profileCompleteness}% complete</span>
        </div>
        <p className="card-note">
          This is the identity record the active workspace will expose to later
          catalog, POS, inventory, and reporting modules.
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
              onChange={(event) => update("code", event.target.value.toUpperCase())}
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
            Save Active Business
          </button>
          <span className="muted-text">{statusMessage}</span>
        </div>

        <div className="detail-list two-column-detail-list spaced-detail-list">
          <div>
            <span>Created</span>
            <code>{formatDateTime(form.createdAt)}</code>
          </div>
          <div>
            <span>Last updated</span>
            <code>{formatDateTime(form.updatedAt)}</code>
          </div>
        </div>
      </section>
    </div>
  );
}
```

## `files/src/modules/catalog/CatalogPage.tsx`

```tsx
import { useCallback, useEffect, useMemo, useState } from "react";
import { useAppState } from "../../app/AppProvider";
import {
  loadCatalogWorkspace,
  saveCatalogCategory,
  saveCatalogItem,
  saveCatalogUnit,
  setCatalogItemArchived
} from "../../shared/api";
import type {
  CatalogItemView,
  CatalogWorkspace,
  SaveCatalogCategoryInput,
  SaveCatalogItemInput,
  SaveCatalogUnitInput
} from "../../shared/types";
import {
  formatCurrency,
  formatDateTime,
  linesFromMultilineValue,
  multilineValueFromLines,
  titleCaseWords
} from "../../shared/utils";

const emptyCategoryForm: SaveCatalogCategoryInput = {
  id: undefined,
  name: "",
  code: "",
  parentId: null,
  itemScope: "all",
  sortOrder: 10,
  notes: null
};

const emptyUnitForm: SaveCatalogUnitInput = {
  id: undefined,
  name: "",
  code: "",
  symbol: "",
  allowFractional: false
};

const emptyItemForm: SaveCatalogItemInput = {
  id: undefined,
  categoryId: null,
  unitId: null,
  taxProfileId: null,
  itemKind: "stock",
  name: "",
  displayName: null,
  sku: null,
  barcodes: [],
  description: null,
  sellingPrice: 0,
  costPrice: 0,
  trackStock: true,
  stockQuantity: 0,
  reorderLevel: 0,
  imagePath: null,
  isActive: true
};

function StockIndicator({ item }: { item: CatalogItemView["item"] }) {
  if (!item.trackStock || item.archivedAt) return <span className="pill neutral">No stock tracking</span>;
  if (item.reorderLevel > 0 && item.stockQuantity <= item.reorderLevel) {
    return <span className="pill warning">Low stock</span>;
  }
  return <span className="pill success">Stock okay</span>;
}

export function CatalogPage() {
  const { data, refresh } = useAppState();
  const [workspace, setWorkspace] = useState<CatalogWorkspace | null>(null);
  const [loading, setLoading] = useState(false);
  const [statusMessage, setStatusMessage] = useState("");
  const [search, setSearch] = useState("");
  const [kindFilter, setKindFilter] = useState("all");
  const [showArchived, setShowArchived] = useState(false);
  const [categoryForm, setCategoryForm] =
    useState<SaveCatalogCategoryInput>(emptyCategoryForm);
  const [unitForm, setUnitForm] = useState<SaveCatalogUnitInput>(emptyUnitForm);
  const [itemForm, setItemForm] = useState<SaveCatalogItemInput>(emptyItemForm);
  const [barcodeText, setBarcodeText] = useState("");

  const loadWorkspace = useCallback(async () => {
    setLoading(true);
    try {
      const next = await loadCatalogWorkspace();
      setWorkspace(next);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    void loadWorkspace();
  }, [loadWorkspace, data?.activeBusiness.id]);

  const filteredItems = useMemo(() => {
    if (!workspace) return [];
    const query = search.trim().toLowerCase();
    return workspace.items.filter((entry) => {
      const matchesArchived = showArchived
        ? true
        : entry.item.archivedAt === null;
      const matchesKind = kindFilter === "all" || entry.item.itemKind === kindFilter;
      const haystack = [
        entry.item.name,
        entry.item.displayName,
        entry.item.sku,
        entry.item.primaryBarcode,
        entry.categoryName,
        entry.taxLabel,
        entry.unitCode
      ]
        .filter(Boolean)
        .join(" ")
        .toLowerCase();

      const matchesQuery = !query || haystack.includes(query);
      return matchesArchived && matchesKind && matchesQuery;
    });
  }, [kindFilter, search, showArchived, workspace]);

  if (!data) return null;

  function resetCategoryForm() {
    setCategoryForm(emptyCategoryForm);
  }

  function resetUnitForm() {
    setUnitForm(emptyUnitForm);
  }

  function resetItemForm() {
    setItemForm({
      ...emptyItemForm,
      taxProfileId: data.activeTaxProfile.id
    });
    setBarcodeText("");
  }

  function editCategory(categoryId: string) {
    const selected = workspace?.categories.find((category) => category.id === categoryId);
    if (!selected) return;
    setCategoryForm({
      id: selected.id,
      name: selected.name,
      code: selected.code,
      parentId: selected.parentId,
      itemScope: selected.itemScope,
      sortOrder: selected.sortOrder,
      notes: selected.notes
    });
  }

  function editUnit(unitId: string) {
    const selected = workspace?.units.find((unit) => unit.id === unitId);
    if (!selected || selected.isSystem) return;
    setUnitForm({
      id: selected.id,
      name: selected.name,
      code: selected.code,
      symbol: selected.symbol,
      allowFractional: selected.allowFractional
    });
  }

  function editItem(entry: CatalogItemView) {
    setItemForm({
      id: entry.item.id,
      categoryId: entry.item.categoryId,
      unitId: entry.item.unitId,
      taxProfileId: entry.item.taxProfileId,
      itemKind: entry.item.itemKind,
      name: entry.item.name,
      displayName: entry.item.displayName,
      sku: entry.item.sku,
      barcodes: entry.barcodes.map((barcode) => barcode.barcode),
      description: entry.item.description,
      sellingPrice: entry.item.sellingPrice,
      costPrice: entry.item.costPrice,
      trackStock: entry.item.trackStock,
      stockQuantity: entry.item.stockQuantity,
      reorderLevel: entry.item.reorderLevel,
      imagePath: entry.item.imagePath,
      isActive: entry.item.isActive
    });
    setBarcodeText(multilineValueFromLines(entry.barcodes.map((barcode) => barcode.barcode)));
  }

  async function handleCategorySave() {
    setStatusMessage("Saving category…");
    try {
      await saveCatalogCategory({
        ...categoryForm,
        code: categoryForm.code.toUpperCase(),
        notes: categoryForm.notes?.trim() ? categoryForm.notes : null
      });
      resetCategoryForm();
      await loadWorkspace();
      await refresh();
      setStatusMessage("Catalog category saved locally.");
    } catch (error) {
      setStatusMessage(
        error instanceof Error ? error.message : "Failed to save category."
      );
    }
  }

  async function handleUnitSave() {
    setStatusMessage("Saving unit…");
    try {
      await saveCatalogUnit({
        ...unitForm,
        code: unitForm.code.toUpperCase(),
        symbol: unitForm.symbol.toUpperCase()
      });
      resetUnitForm();
      await loadWorkspace();
      await refresh();
      setStatusMessage("Catalog unit saved locally.");
    } catch (error) {
      setStatusMessage(
        error instanceof Error ? error.message : "Failed to save unit."
      );
    }
  }

  async function handleItemSave() {
    setStatusMessage("Saving item…");
    try {
      await saveCatalogItem({
        ...itemForm,
        sku: itemForm.sku?.trim() ? itemForm.sku : null,
        barcodes: linesFromMultilineValue(barcodeText),
        description: itemForm.description?.trim() ? itemForm.description : null,
        imagePath: itemForm.imagePath?.trim() ? itemForm.imagePath : null,
        displayName: itemForm.displayName?.trim() ? itemForm.displayName : null
      });
      resetItemForm();
      await loadWorkspace();
      await refresh();
      setStatusMessage("Catalog item saved locally.");
    } catch (error) {
      setStatusMessage(
        error instanceof Error ? error.message : "Failed to save item."
      );
    }
  }

  async function toggleArchive(entry: CatalogItemView) {
    setStatusMessage(
      entry.item.archivedAt ? "Restoring item…" : "Archiving item…"
    );
    try {
      await setCatalogItemArchived(entry.item.id, !entry.item.archivedAt);
      await loadWorkspace();
      await refresh();
      setStatusMessage(
        entry.item.archivedAt
          ? "Catalog item restored locally."
          : "Catalog item archived locally."
      );
    } catch (error) {
      setStatusMessage(
        error instanceof Error ? error.message : "Failed to update item state."
      );
    }
  }

  return (
    <div className="page-grid">
      <section className="hero-card">
        <div>
          <div className="section-kicker">Patch 3 catalog core</div>
          <h2>{data.activeBusiness.name} item master</h2>
          <p>
            Build a clean local catalog for retail items, menu items, and services.
            This patch keeps stock quantities lightweight and prepares the data shape for
            deeper inventory and POS patches.
          </p>
        </div>
        <div className="hero-actions align-start">
          <span className="meta-chip">Categories: {workspace?.summary.categoryCount ?? 0}</span>
          <span className="meta-chip">Active items: {workspace?.summary.activeItems ?? 0}</span>
          <span className="meta-chip">Low stock: {workspace?.summary.lowStockCandidates ?? 0}</span>
        </div>
      </section>

      <section className="card-grid card-grid-4">
        <article className="card">
          <div className="card-label">Menu items</div>
          <div className="kpi-value">{workspace?.summary.menuItemCount ?? 0}</div>
          <p className="card-note">Prepared or food-facing items without full recipe deduction yet.</p>
        </article>
        <article className="card">
          <div className="card-label">Stock items</div>
          <div className="kpi-value">{workspace?.summary.stockItemCount ?? 0}</div>
          <p className="card-note">Tracked products with current stock and reorder thresholds.</p>
        </article>
        <article className="card">
          <div className="card-label">Service items</div>
          <div className="kpi-value">{workspace?.summary.serviceItemCount ?? 0}</div>
          <p className="card-note">Simple non-stock services that can be billed in later patches.</p>
        </article>
        <article className="card">
          <div className="card-label">Archived items</div>
          <div className="kpi-value">{workspace?.summary.archivedItems ?? 0}</div>
          <p className="card-note">Archived records stay local and restorable for safer operations.</p>
        </article>
      </section>

      <section className="split-grid catalog-primary-grid">
        <article className="card">
          <div className="card-header">
            <h2>Item catalog</h2>
            <span className="pill success">{filteredItems.length} shown</span>
          </div>

          <div className="catalog-filter-bar">
            <input
              placeholder="Search name, SKU, barcode, category…"
              value={search}
              onChange={(event) => setSearch(event.target.value)}
            />
            <select
              value={kindFilter}
              onChange={(event) => setKindFilter(event.target.value)}
            >
              <option value="all">All types</option>
              <option value="stock">Stock</option>
              <option value="menu">Menu</option>
              <option value="service">Service</option>
            </select>
            <label className="toggle-row inline-toggle">
              <input
                type="checkbox"
                checked={showArchived}
                onChange={(event) => setShowArchived(event.target.checked)}
              />
              <span>Show archived</span>
            </label>
          </div>

          <div className="status-banner">
            {loading ? "Loading catalog workspace…" : statusMessage || "Catalog ready."}
          </div>

          <div className="catalog-list">
            {filteredItems.length > 0 ? (
              filteredItems.map((entry) => (
                <article className="catalog-item-card" key={entry.item.id}>
                  <div className="card-header compact-card-header">
                    <div>
                      <h3>{entry.item.displayName || entry.item.name}</h3>
                      <div className="muted-text">
                        {titleCaseWords(entry.item.itemKind)}
                        {entry.categoryName ? ` · ${entry.categoryName}` : ""}
                        {entry.unitCode ? ` · ${entry.unitCode}` : ""}
                      </div>
                    </div>
                    <div className="tag-list compact-tags">
                      <StockIndicator item={entry.item} />
                      <span className={`pill ${entry.item.isActive ? "success" : "neutral"}`}>
                        {entry.item.isActive ? "Active" : "Inactive"}
                      </span>
                    </div>
                  </div>

                  <div className="detail-list compact-detail-list">
                    <div>
                      <span>Selling price</span>
                      <code>
                        {formatCurrency(entry.item.sellingPrice, data.activeBusiness.currencyCode)}
                      </code>
                    </div>
                    <div>
                      <span>Cost price</span>
                      <code>
                        {formatCurrency(entry.item.costPrice, data.activeBusiness.currencyCode)}
                      </code>
                    </div>
                    <div>
                      <span>SKU</span>
                      <code>{entry.item.sku || "—"}</code>
                    </div>
                    <div>
                      <span>Barcode</span>
                      <code>{entry.item.primaryBarcode || "—"}</code>
                    </div>
                    <div>
                      <span>Stock</span>
                      <code>
                        {entry.item.trackStock ? entry.item.stockQuantity : "Not tracked"}
                      </code>
                    </div>
                    <div>
                      <span>Updated</span>
                      <code>{formatDateTime(entry.item.updatedAt)}</code>
                    </div>
                  </div>

                  <div className="catalog-item-description muted-text">
                    {entry.item.description || "No description saved yet."}
                  </div>

                  <div className="inline-actions">
                    <button
                      className="secondary-button"
                      type="button"
                      onClick={() => editItem(entry)}
                    >
                      Edit item
                    </button>
                    <button
                      className="secondary-button"
                      type="button"
                      onClick={() => void toggleArchive(entry)}
                    >
                      {entry.item.archivedAt ? "Restore" : "Archive"}
                    </button>
                  </div>
                </article>
              ))
            ) : (
              <p className="muted-text">No items match the current filter yet.</p>
            )}
          </div>
        </article>

        <article className="card">
          <div className="card-header">
            <h2>{itemForm.id ? "Edit item" : "Add catalog item"}</h2>
            <span className="pill warning">Local only</span>
          </div>

          <div className="form-grid">
            <label>
              <span>Item type</span>
              <select
                value={itemForm.itemKind}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    itemKind: event.target.value,
                    trackStock: event.target.value === "stock" ? current.trackStock : false
                  }))
                }
              >
                <option value="stock">Stock / retail item</option>
                <option value="menu">Menu item</option>
                <option value="service">Service item</option>
              </select>
            </label>

            <label>
              <span>Name</span>
              <input
                value={itemForm.name}
                onChange={(event) =>
                  setItemForm((current) => ({ ...current, name: event.target.value }))
                }
              />
            </label>

            <label>
              <span>Display name</span>
              <input
                value={itemForm.displayName ?? ""}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    displayName: event.target.value
                  }))
                }
              />
            </label>

            <label>
              <span>SKU</span>
              <input
                value={itemForm.sku ?? ""}
                onChange={(event) =>
                  setItemForm((current) => ({ ...current, sku: event.target.value }))
                }
              />
            </label>

            <label>
              <span>Category</span>
              <select
                value={itemForm.categoryId ?? ""}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    categoryId: event.target.value || null
                  }))
                }
              >
                <option value="">No category</option>
                {workspace?.categories
                  .filter((category) => !category.archivedAt)
                  .map((category) => (
                    <option key={category.id} value={category.id}>
                      {category.name}
                    </option>
                  ))}
              </select>
            </label>

            <label>
              <span>Unit</span>
              <select
                value={itemForm.unitId ?? ""}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    unitId: event.target.value || null
                  }))
                }
              >
                <option value="">No unit</option>
                {workspace?.units.map((unit) => (
                  <option key={unit.id} value={unit.id}>
                    {unit.name} ({unit.code})
                  </option>
                ))}
              </select>
            </label>

            <label>
              <span>Tax profile</span>
              <select
                value={itemForm.taxProfileId ?? ""}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    taxProfileId: event.target.value || null
                  }))
                }
              >
                <option value="">No tax profile</option>
                {workspace?.taxProfiles.map((profile) => (
                  <option key={profile.id} value={profile.id}>
                    {profile.name} · {profile.taxLabel} · {profile.defaultRate}%
                  </option>
                ))}
              </select>
            </label>

            <label>
              <span>Selling price</span>
              <input
                type="number"
                min="0"
                step="0.01"
                value={itemForm.sellingPrice}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    sellingPrice: Number(event.target.value)
                  }))
                }
              />
            </label>

            <label>
              <span>Cost price</span>
              <input
                type="number"
                min="0"
                step="0.01"
                value={itemForm.costPrice}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    costPrice: Number(event.target.value)
                  }))
                }
              />
            </label>

            <label className="toggle-row">
              <input
                type="checkbox"
                checked={itemForm.trackStock}
                disabled={itemForm.itemKind !== "stock"}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    trackStock: event.target.checked
                  }))
                }
              />
              <span>Track stock quantity</span>
            </label>

            <label className="toggle-row">
              <input
                type="checkbox"
                checked={itemForm.isActive}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    isActive: event.target.checked
                  }))
                }
              />
              <span>Item is active</span>
            </label>

            <label>
              <span>Current stock</span>
              <input
                type="number"
                min="0"
                step="0.01"
                value={itemForm.stockQuantity}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    stockQuantity: Number(event.target.value)
                  }))
                }
              />
            </label>

            <label>
              <span>Reorder level</span>
              <input
                type="number"
                min="0"
                step="0.01"
                value={itemForm.reorderLevel}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    reorderLevel: Number(event.target.value)
                  }))
                }
              />
            </label>

            <label className="form-span-2">
              <span>Barcodes (one per line)</span>
              <textarea
                rows={4}
                value={barcodeText}
                onChange={(event) => setBarcodeText(event.target.value)}
              />
            </label>

            <label className="form-span-2">
              <span>Image path</span>
              <input
                value={itemForm.imagePath ?? ""}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    imagePath: event.target.value
                  }))
                }
              />
            </label>

            <label className="form-span-2">
              <span>Description</span>
              <textarea
                rows={4}
                value={itemForm.description ?? ""}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    description: event.target.value
                  }))
                }
              />
            </label>
          </div>

          <div className="inline-actions spacing-top">
            <button className="primary-button" type="button" onClick={() => void handleItemSave()}>
              {itemForm.id ? "Save item changes" : "Add catalog item"}
            </button>
            <button className="secondary-button" type="button" onClick={resetItemForm}>
              Clear form
            </button>
          </div>
        </article>
      </section>

      <section className="split-grid catalog-secondary-grid">
        <article className="card">
          <div className="card-header">
            <h2>Categories</h2>
            <span className="pill neutral">{workspace?.categories.length ?? 0} total</span>
          </div>

          <div className="stack-list compact-stack-list">
            {workspace?.categories.map((category) => (
              <div className="list-row" key={category.id}>
                <div>
                  <strong>{category.name}</strong>
                  <div className="muted-text">
                    {category.code} · {titleCaseWords(category.itemScope)}
                  </div>
                </div>
                <button
                  className="secondary-button"
                  type="button"
                  onClick={() => editCategory(category.id)}
                >
                  Edit
                </button>
              </div>
            ))}
          </div>

          <div className="card-header spacing-top">
            <h3>{categoryForm.id ? "Edit category" : "Add category"}</h3>
          </div>
          <div className="form-grid compact-form-grid">
            <label>
              <span>Name</span>
              <input
                value={categoryForm.name}
                onChange={(event) =>
                  setCategoryForm((current) => ({ ...current, name: event.target.value }))
                }
              />
            </label>
            <label>
              <span>Code</span>
              <input
                value={categoryForm.code}
                onChange={(event) =>
                  setCategoryForm((current) => ({ ...current, code: event.target.value }))
                }
              />
            </label>
            <label>
              <span>Parent category</span>
              <select
                value={categoryForm.parentId ?? ""}
                onChange={(event) =>
                  setCategoryForm((current) => ({
                    ...current,
                    parentId: event.target.value || null
                  }))
                }
              >
                <option value="">No parent</option>
                {workspace?.categories
                  .filter((category) => category.id !== categoryForm.id)
                  .map((category) => (
                    <option key={category.id} value={category.id}>
                      {category.name}
                    </option>
                  ))}
              </select>
            </label>
            <label>
              <span>Scope</span>
              <select
                value={categoryForm.itemScope}
                onChange={(event) =>
                  setCategoryForm((current) => ({
                    ...current,
                    itemScope: event.target.value
                  }))
                }
              >
                <option value="all">All</option>
                <option value="stock">Stock</option>
                <option value="menu">Menu</option>
                <option value="service">Service</option>
              </select>
            </label>
            <label>
              <span>Sort order</span>
              <input
                type="number"
                min="0"
                value={categoryForm.sortOrder}
                onChange={(event) =>
                  setCategoryForm((current) => ({
                    ...current,
                    sortOrder: Number(event.target.value)
                  }))
                }
              />
            </label>
            <label className="form-span-2">
              <span>Notes</span>
              <textarea
                rows={3}
                value={categoryForm.notes ?? ""}
                onChange={(event) =>
                  setCategoryForm((current) => ({
                    ...current,
                    notes: event.target.value
                  }))
                }
              />
            </label>
          </div>
          <div className="inline-actions spacing-top">
            <button className="primary-button" type="button" onClick={() => void handleCategorySave()}>
              {categoryForm.id ? "Save category" : "Add category"}
            </button>
            <button className="secondary-button" type="button" onClick={resetCategoryForm}>
              Clear form
            </button>
          </div>
        </article>

        <article className="card">
          <div className="card-header">
            <h2>Units</h2>
            <span className="pill neutral">{workspace?.units.length ?? 0} available</span>
          </div>

          <div className="stack-list compact-stack-list">
            {workspace?.units.map((unit) => (
              <div className="list-row" key={unit.id}>
                <div>
                  <strong>{unit.name}</strong>
                  <div className="muted-text">
                    {unit.code} · {unit.allowFractional ? "Fractional" : "Whole only"}
                  </div>
                </div>
                {unit.isSystem ? (
                  <span className="pill success">System</span>
                ) : (
                  <button
                    className="secondary-button"
                    type="button"
                    onClick={() => editUnit(unit.id)}
                  >
                    Edit
                  </button>
                )}
              </div>
            ))}
          </div>

          <div className="card-header spacing-top">
            <h3>{unitForm.id ? "Edit custom unit" : "Add custom unit"}</h3>
          </div>
          <div className="form-grid compact-form-grid">
            <label>
              <span>Unit name</span>
              <input
                value={unitForm.name}
                onChange={(event) =>
                  setUnitForm((current) => ({ ...current, name: event.target.value }))
                }
              />
            </label>
            <label>
              <span>Code</span>
              <input
                value={unitForm.code}
                onChange={(event) =>
                  setUnitForm((current) => ({ ...current, code: event.target.value }))
                }
              />
            </label>
            <label>
              <span>Symbol</span>
              <input
                value={unitForm.symbol}
                onChange={(event) =>
                  setUnitForm((current) => ({ ...current, symbol: event.target.value }))
                }
              />
            </label>
            <label className="toggle-row">
              <input
                type="checkbox"
                checked={unitForm.allowFractional}
                onChange={(event) =>
                  setUnitForm((current) => ({
                    ...current,
                    allowFractional: event.target.checked
                  }))
                }
              />
              <span>Allow fractional quantities</span>
            </label>
          </div>
          <div className="inline-actions spacing-top">
            <button className="primary-button" type="button" onClick={() => void handleUnitSave()}>
              {unitForm.id ? "Save unit" : "Add custom unit"}
            </button>
            <button className="secondary-button" type="button" onClick={resetUnitForm}>
              Clear form
            </button>
          </div>
        </article>
      </section>
    </div>
  );
}
```

## `files/src/modules/dashboard/DashboardPage.tsx`

```tsx
import { useAppState } from "../../app/AppProvider";
import type { NavPage } from "../../shared/types";
import { formatDateTime, formatModuleList, titleCaseWords } from "../../shared/utils";

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
          <div className="section-kicker">Patch 3 catalog core</div>
          <h2>{data.dashboard.heroTitle}</h2>
          <p>{data.dashboard.heroBody}</p>
        </div>
        <div className="hero-actions">
          <button
            className="primary-button"
            type="button"
            onClick={() => onNavigate("business")}
          >
            Manage Businesses
          </button>
          <button
            className="secondary-button"
            type="button"
            onClick={() => onNavigate("catalog")}
          >
            Open Catalog
          </button>
        </div>
      </section>

      <section className="card-grid card-grid-5">
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
            <h3>Business workspaces</h3>
            <span className="pill success">
              {data.businessWorkspaces.length} configured
            </span>
          </div>
          <div className="stack-list">
            {data.businessWorkspaces.map((workspace) => (
              <div className="list-row" key={workspace.businessId}>
                <div>
                  <strong>
                    {workspace.name}
                    {workspace.businessId === data.activeBusiness.id ? " · active" : ""}
                  </strong>
                  <div className="muted-text">
                    {workspace.code} · {workspace.currencyCode} · {workspace.timezone}
                  </div>
                  <div className="tag-list compact-tags">
                    {workspace.activeModules.map((module) => (
                      <span className="tag" key={`${workspace.businessId}-${module}`}>
                        {titleCaseWords(module)}
                      </span>
                    ))}
                  </div>
                </div>
                <span className="muted-text">{workspace.nextSaleSequence}</span>
              </div>
            ))}
          </div>
        </article>

        <article className="card">
          <div className="card-header">
            <h3>Current business configuration</h3>
            <span className="pill neutral">{data.activeBusiness.code}</span>
          </div>
          <div className="detail-list">
            <div>
              <span>Theme</span>
              <code>{data.businessSettings.theme}</code>
            </div>
            <div>
              <span>Tax profile</span>
              <code>
                {data.activeTaxProfile.name} · {data.activeTaxProfile.taxLabel} · {data.activeTaxProfile.defaultRate}%
              </code>
            </div>
            <div>
              <span>Receipt profile</span>
              <code>
                {data.activeReceiptProfile.name} · {data.activeReceiptProfile.paperWidth}
              </code>
            </div>
            <div>
              <span>Enabled modules</span>
              <code>{formatModuleList(data.businessWorkspaces.find((workspace) => workspace.businessId === data.activeBusiness.id)?.activeModules ?? [])}</code>
            </div>
          </div>
        </article>
      </section>

      <section className="split-grid">
        <article className="card">
          <div className="card-header">
            <h3>Recent local activity</h3>
            <span className="pill neutral">Local only</span>
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
            <h3>Module roadmap from this workspace</h3>
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

## `files/src/modules/data-center/DataCenterPage.tsx`

```tsx
import { useMemo, useState } from "react";
import { useAppState } from "../../app/AppProvider";
import { formatDateTime } from "../../shared/utils";

export function DataCenterPage() {
  const { data, createBackup, exportFoundation, previewImport } = useAppState();
  const [statusMessage, setStatusMessage] = useState("");
  const [importPath, setImportPath] = useState("");
  const [previewResult, setPreviewResult] = useState("");

  const latestBackup = useMemo(() => {
    return data?.backups[0] ?? null;
  }, [data?.backups]);

  if (!data) return null;

  async function handleBackup() {
    setStatusMessage("Creating workspace backup snapshot…");
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
    setStatusMessage("Exporting workspace foundation snapshot…");
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
        `Bundle OK · type=${preview.bundleType ?? "unknown"} · source=${preview.sourcePatchLevel ?? "unknown"} · generated=${preview.generatedAt ?? "unknown"} · businesses=${preview.businessCount} · categories=${preview.categoryCount} · items=${preview.itemCount}`
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
            Patch 3 expands the export scope again so the workspace bundle now carries
            category, unit, item, and barcode foundations alongside the multi-business
            workspace data. Backups still stay fully local and file-based.
          </p>

          <div className="inline-actions">
            <button className="primary-button" type="button" onClick={() => void handleBackup()}>
              Create Backup Snapshot
            </button>
            <button className="secondary-button" type="button" onClick={() => void handleExport()}>
              Export Workspace Snapshot
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
            <div>
              <span>Business workspaces</span>
              <code>{data.businessWorkspaces.length}</code>
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
            Patch 3 still stops at preview validation, but it now understands the
            catalog-aware workspace export bundle and shows business, category, and
            item counts before any real import is attempted.
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

## `files/src/modules/settings/SettingsPage.tsx`

```tsx
import { useEffect, useMemo, useState } from "react";
import { useAppState } from "../../app/AppProvider";
import type {
  BusinessSettings,
  ModuleFlags,
  ReceiptProfile,
  SequenceCounter,
  TaxProfile,
  WorkspaceConfigurationInput
} from "../../shared/types";
import {
  formatDateTime,
  formatSequencePreview,
  humanizeBoolean,
  titleCaseWords
} from "../../shared/utils";

export function SettingsPage() {
  const { data, saveWorkspace } = useAppState();
  const [settings, setSettings] = useState<BusinessSettings | null>(null);
  const [taxProfile, setTaxProfile] = useState<TaxProfile | null>(null);
  const [receiptProfile, setReceiptProfile] = useState<ReceiptProfile | null>(null);
  const [moduleFlags, setModuleFlags] = useState<ModuleFlags | null>(null);
  const [sequences, setSequences] = useState<SequenceCounter[]>([]);
  const [statusMessage, setStatusMessage] = useState("");

  useEffect(() => {
    if (!data) return;
    setSettings(data.businessSettings);
    setTaxProfile(data.activeTaxProfile);
    setReceiptProfile(data.activeReceiptProfile);
    setModuleFlags(data.activeModuleFlags);
    setSequences(data.activeSequences);
  }, [data]);

  const nextSaleNumber = useMemo(() => {
    const saleSequence = sequences.find((sequence) => sequence.scope === "sale");
    if (!saleSequence) return "INV00001";
    return formatSequencePreview(
      saleSequence.prefix,
      saleSequence.nextNumber,
      saleSequence.padding
    );
  }, [sequences]);

  if (!data || !settings || !taxProfile || !receiptProfile || !moduleFlags) {
    return null;
  }

  function updateSettings<K extends keyof BusinessSettings>(
    key: K,
    value: BusinessSettings[K]
  ) {
    setSettings((current) => (current ? { ...current, [key]: value } : current));
  }

  function updateTaxProfile<K extends keyof TaxProfile>(
    key: K,
    value: TaxProfile[K]
  ) {
    setTaxProfile((current) => (current ? { ...current, [key]: value } : current));
  }

  function updateReceiptProfile<K extends keyof ReceiptProfile>(
    key: K,
    value: ReceiptProfile[K]
  ) {
    setReceiptProfile((current) => (current ? { ...current, [key]: value } : current));
  }

  function updateModuleFlags<K extends keyof ModuleFlags>(
    key: K,
    value: ModuleFlags[K]
  ) {
    setModuleFlags((current) => (current ? { ...current, [key]: value } : current));
  }

  function updateSequence<K extends keyof SequenceCounter>(
    index: number,
    key: K,
    value: SequenceCounter[K]
  ) {
    setSequences((current) =>
      current.map((sequence, sequenceIndex) =>
        sequenceIndex === index ? { ...sequence, [key]: value } : sequence
      )
    );
  }

  async function handleSave() {
    setStatusMessage("Saving workspace configuration…");
    try {
      const payload: WorkspaceConfigurationInput = {
        businessSettings: settings,
        taxProfile,
        receiptProfile,
        moduleFlags,
        sequenceCounters: sequences
      };
      await saveWorkspace(payload);
      setStatusMessage("Workspace configuration saved locally.");
    } catch (error) {
      setStatusMessage(
        error instanceof Error ? error.message : "Failed to save workspace."
      );
    }
  }

  return (
    <div className="page-grid">
      <section className="hero-card">
        <div>
          <div className="section-kicker">Per-business settings core</div>
          <h2>{data.activeBusiness.name}</h2>
          <p>
            Patch 3 keeps tax defaults, receipt defaults, module flags, and
            sequence counters per business so later modules can stay isolated.
          </p>
        </div>
        <div className="hero-actions align-start">
          <span className="meta-chip">Next sale number: {nextSaleNumber}</span>
          <span className="meta-chip">Updated {formatDateTime(settings.updatedAt)}</span>
        </div>
      </section>

      <section className="split-grid">
        <article className="card">
          <div className="card-header">
            <h2>General local settings</h2>
            <span className="pill neutral">Business scoped</span>
          </div>

          <div className="form-grid">
            <label>
              <span>Timezone</span>
              <input
                value={settings.timezone}
                onChange={(event) => updateSettings("timezone", event.target.value)}
              />
            </label>

            <label>
              <span>Locale</span>
              <input
                value={settings.locale}
                onChange={(event) => updateSettings("locale", event.target.value)}
              />
            </label>

            <label>
              <span>Date format</span>
              <input
                value={settings.dateFormat}
                onChange={(event) => updateSettings("dateFormat", event.target.value)}
              />
            </label>

            <label>
              <span>Theme preference</span>
              <select
                value={settings.theme}
                onChange={(event) => updateSettings("theme", event.target.value)}
              >
                <option value="system">System</option>
                <option value="light">Light</option>
                <option value="dark">Dark</option>
              </select>
            </label>

            <label className="toggle-row">
              <input
                type="checkbox"
                checked={settings.autoBackupEnabled}
                onChange={(event) =>
                  updateSettings("autoBackupEnabled", event.target.checked)
                }
              />
              <span>Auto backup preference</span>
            </label>

            <label className="form-span-2">
              <span>Backup directory override</span>
              <input
                value={settings.backupDirectory ?? ""}
                onChange={(event) =>
                  updateSettings("backupDirectory", event.target.value)
                }
              />
            </label>
          </div>

          <div className="card-header spacing-top">
            <h3>Default tax profile</h3>
          </div>
          <div className="form-grid">
            <label>
              <span>Profile name</span>
              <input
                value={taxProfile.name}
                onChange={(event) => updateTaxProfile("name", event.target.value)}
              />
            </label>

            <label>
              <span>Tax label</span>
              <input
                value={taxProfile.taxLabel}
                onChange={(event) =>
                  updateTaxProfile("taxLabel", event.target.value)
                }
              />
            </label>

            <label>
              <span>Default tax rate (%)</span>
              <input
                type="number"
                step="0.01"
                value={taxProfile.defaultRate}
                onChange={(event) =>
                  updateTaxProfile("defaultRate", Number(event.target.value))
                }
              />
            </label>

            <label className="toggle-row">
              <input
                type="checkbox"
                checked={taxProfile.pricesIncludeTax}
                onChange={(event) =>
                  updateTaxProfile("pricesIncludeTax", event.target.checked)
                }
              />
              <span>Prices include tax</span>
            </label>
          </div>
        </article>

        <article className="card">
          <div className="card-header">
            <h2>Receipt profile foundation</h2>
            <span className="pill success">Local print defaults</span>
          </div>

          <div className="form-grid">
            <label>
              <span>Profile name</span>
              <input
                value={receiptProfile.name}
                onChange={(event) =>
                  updateReceiptProfile("name", event.target.value)
                }
              />
            </label>

            <label>
              <span>Paper width</span>
              <select
                value={receiptProfile.paperWidth}
                onChange={(event) =>
                  updateReceiptProfile("paperWidth", event.target.value)
                }
              >
                <option value="80mm">80mm</option>
                <option value="58mm">58mm</option>
                <option value="A4">A4</option>
              </select>
            </label>

            <label className="form-span-2">
              <span>Receipt footer</span>
              <textarea
                rows={4}
                value={receiptProfile.footerText ?? ""}
                onChange={(event) =>
                  updateReceiptProfile("footerText", event.target.value)
                }
              />
            </label>

            <label className="toggle-row">
              <input
                type="checkbox"
                checked={receiptProfile.showAddress}
                onChange={(event) =>
                  updateReceiptProfile("showAddress", event.target.checked)
                }
              />
              <span>Show address</span>
            </label>

            <label className="toggle-row">
              <input
                type="checkbox"
                checked={receiptProfile.showPhone}
                onChange={(event) =>
                  updateReceiptProfile("showPhone", event.target.checked)
                }
              />
              <span>Show phone</span>
            </label>

            <label className="toggle-row">
              <input
                type="checkbox"
                checked={receiptProfile.showEmail}
                onChange={(event) =>
                  updateReceiptProfile("showEmail", event.target.checked)
                }
              />
              <span>Show email</span>
            </label>

            <label className="toggle-row">
              <input
                type="checkbox"
                checked={receiptProfile.showBusinessCode}
                onChange={(event) =>
                  updateReceiptProfile("showBusinessCode", event.target.checked)
                }
              />
              <span>Show business code</span>
            </label>
          </div>

          <div className="card-header spacing-top">
            <h3>Module flags foundation</h3>
            <span className="muted-text">Future patch toggles</span>
          </div>
          <div className="toggle-grid">
            {[
              ["restaurantEnabled", "Restaurant mode"],
              ["retailEnabled", "Retail mode"],
              ["inventoryEnabled", "Inventory mode"],
              ["servicesEnabled", "Service mode"],
              ["customersEnabled", "Customers"],
              ["suppliersEnabled", "Suppliers"],
              ["expensesEnabled", "Expenses"],
              ["reportingEnabled", "Reporting"],
              ["dataCenterEnabled", "Data center"]
            ].map(([key, label]) => (
              <label className="toggle-row" key={key}>
                <input
                  type="checkbox"
                  checked={moduleFlags[key as keyof ModuleFlags] as boolean}
                  onChange={(event) =>
                    updateModuleFlags(
                      key as keyof ModuleFlags,
                      event.target.checked as ModuleFlags[keyof ModuleFlags]
                    )
                  }
                />
                <span>{label}</span>
              </label>
            ))}
          </div>
        </article>
      </section>

      <section className="split-grid">
        <article className="card">
          <div className="card-header">
            <h2>Sequence counters foundation</h2>
            <span className="pill warning">Patch 3</span>
          </div>
          <p className="card-note">
            These document number seeds are stored now so POS, purchases, and
            ledger records can reuse them later without breaking numbering.
          </p>

          <div className="sequence-list">
            {sequences.map((sequence, index) => (
              <div className="sequence-row" key={sequence.id || `${sequence.scope}-${index}`}>
                <div className="sequence-row-header">
                  <div>
                    <strong>{titleCaseWords(sequence.scope)}</strong>
                    <div className="muted-text">
                      Preview {formatSequencePreview(sequence.prefix, sequence.nextNumber, sequence.padding)}
                    </div>
                  </div>
                  <span className="pill neutral">{sequence.resetPolicy}</span>
                </div>

                <div className="form-grid compact-form-grid">
                  <label>
                    <span>Prefix</span>
                    <input
                      value={sequence.prefix}
                      onChange={(event) =>
                        updateSequence(index, "prefix", event.target.value.toUpperCase())
                      }
                    />
                  </label>

                  <label>
                    <span>Next number</span>
                    <input
                      type="number"
                      min="1"
                      value={sequence.nextNumber}
                      onChange={(event) =>
                        updateSequence(index, "nextNumber", Number(event.target.value))
                      }
                    />
                  </label>

                  <label>
                    <span>Padding</span>
                    <input
                      type="number"
                      min="1"
                      value={sequence.padding}
                      onChange={(event) =>
                        updateSequence(index, "padding", Number(event.target.value))
                      }
                    />
                  </label>

                  <label>
                    <span>Reset policy</span>
                    <select
                      value={sequence.resetPolicy}
                      onChange={(event) =>
                        updateSequence(index, "resetPolicy", event.target.value)
                      }
                    >
                      <option value="none">No reset</option>
                      <option value="daily">Daily</option>
                      <option value="monthly">Monthly</option>
                      <option value="yearly">Yearly</option>
                    </select>
                  </label>
                </div>
              </div>
            ))}
          </div>
        </article>

        <article className="card">
          <div className="card-header">
            <h2>Current effective settings</h2>
          </div>
          <div className="detail-list">
            <div>
              <span>Theme</span>
              <code>{settings.theme}</code>
            </div>
            <div>
              <span>Auto backup</span>
              <code>{humanizeBoolean(settings.autoBackupEnabled)}</code>
            </div>
            <div>
              <span>Receipt paper width</span>
              <code>{receiptProfile.paperWidth}</code>
            </div>
            <div>
              <span>Receipt shows email</span>
              <code>{humanizeBoolean(receiptProfile.showEmail)}</code>
            </div>
            <div>
              <span>Reporting module</span>
              <code>{humanizeBoolean(moduleFlags.reportingEnabled)}</code>
            </div>
            <div>
              <span>Next sale sequence</span>
              <code>{nextSaleNumber}</code>
            </div>
          </div>
        </article>
      </section>

      <div className="inline-actions">
        <button className="primary-button" type="button" onClick={() => void handleSave()}>
          Save Workspace Settings
        </button>
        <span className="muted-text">{statusMessage}</span>
      </div>
    </div>
  );
}
```

## `files/src/modules/shell/AppShell.tsx`

```tsx
import { useEffect, useMemo, useState } from "react";
import { useAppState } from "../../app/AppProvider";
import type { NavPage } from "../../shared/types";
import { classNames, formatModuleList } from "../../shared/utils";
import { DashboardPage } from "../dashboard/DashboardPage";
import { CatalogPage } from "../catalog/CatalogPage";
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
    key: "catalog",
    label: "Catalog",
    description: "Products, menu, services"
  },
  {
    key: "business",
    label: "Businesses",
    description: "Profiles and switching"
  },
  {
    key: "settings",
    label: "Settings",
    description: "Tax, receipt, modules"
  },
  {
    key: "data-center",
    label: "Data Center",
    description: "Backup and transfer foundation"
  }
];

export function AppShell() {
  const { data, switchBusiness } = useAppState();
  const [activePage, setActivePage] = useState<NavPage>(() => {
    const stored = window.localStorage.getItem(NAV_STORAGE_KEY) as NavPage | null;
    return stored ?? "dashboard";
  });
  const [switchStatus, setSwitchStatus] = useState<string>("");

  useEffect(() => {
    window.localStorage.setItem(NAV_STORAGE_KEY, activePage);
  }, [activePage]);

  useEffect(() => {
    setSwitchStatus("");
  }, [data?.activeBusiness.id]);

  const pageTitle = useMemo(() => {
    return navItems.find((item) => item.key === activePage)?.label ?? "Dashboard";
  }, [activePage]);

  const activeWorkspace = useMemo(() => {
    return data?.businessWorkspaces.find(
      (workspace) => workspace.businessId === data.activeBusiness.id
    );
  }, [data]);

  if (!data) {
    return null;
  }

  async function handleBusinessSwitch(nextBusinessId: string) {
    if (nextBusinessId === data.activeBusiness.id) {
      return;
    }

    setSwitchStatus("Switching active business…");
    try {
      const switched = await switchBusiness(nextBusinessId);
      setSwitchStatus(`Switched to ${switched.name}.`);
    } catch (error) {
      setSwitchStatus(
        error instanceof Error ? error.message : "Failed to switch business."
      );
    }
  }

  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="sidebar-brand">
          <div className="brand-badge">P3</div>
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
          <div className="muted-text">
            {data.activeBusiness.businessType} · {data.activeBusiness.currencyCode}
          </div>
          <label className="sidebar-field">
            <span>Switch workspace</span>
            <select
              className="sidebar-select"
              value={data.activeBusiness.id}
              onChange={(event) => void handleBusinessSwitch(event.target.value)}
            >
              {data.businessWorkspaces.map((workspace) => (
                <option key={workspace.businessId} value={workspace.businessId}>
                  {workspace.name} ({workspace.code})
                </option>
              ))}
            </select>
          </label>
          <div className="sidebar-metadata-grid">
            <div>
              <span>Timezone</span>
              <strong>{data.businessSettings.timezone}</strong>
            </div>
            <div>
              <span>Tax</span>
              <strong>
                {data.activeTaxProfile.taxLabel} · {data.activeTaxProfile.defaultRate}%
              </strong>
            </div>
            <div>
              <span>Catalog items</span>
              <strong>{data.catalogSummary.activeItems}</strong>
            </div>
            <div>
              <span>Low stock flags</span>
              <strong>{data.catalogSummary.lowStockCandidates}</strong>
            </div>
          </div>
          <div className="muted-text small-text">
            {switchStatus || activeWorkspace?.nextSaleSequence || "Sequence pending"}
          </div>
        </div>

        <div className="sidebar-section-label">Business mode</div>
        <div className="sidebar-card">
          <div className="sidebar-pill success">Catalog core ready</div>
          <div className="sidebar-pill neutral">Multi-business ready</div>
          <div className="sidebar-pill neutral">Patch registry ready</div>
          <div className="muted-text small-text">
            {formatModuleList(activeWorkspace?.activeModules ?? [])}
          </div>
        </div>
      </aside>

      <main className="workspace">
        <header className="workspace-header">
          <div>
            <h1>{pageTitle}</h1>
            <p>
              Patch 3 adds local catalog structure for products, menu items, services,
              barcodes, categories, and units without bringing in full POS or inventory
              ledgers yet.
            </p>
          </div>
          <div className="workspace-header-meta">
            <span className="meta-chip">Business: {data.activeBusiness.code}</span>
            <span className="meta-chip">
              Items: {data.catalogSummary.activeItems}
            </span>
            <span className="meta-chip">Schema v{data.appInfo.schemaVersion}</span>
          </div>
        </header>

        <section className="workspace-content">
          {activePage === "dashboard" && (
            <DashboardPage onNavigate={setActivePage} />
          )}
          {activePage === "catalog" && <CatalogPage />}
          {activePage === "business" && <BusinessPage />}
          {activePage === "settings" && <SettingsPage />}
          {activePage === "data-center" && <DataCenterPage />}
        </section>
      </main>
    </div>
  );
}
```

## `files/src/shared/api.ts`

```ts
import { invoke } from "@tauri-apps/api/core";
import type {
  AppBootstrap,
  BackupRecord,
  BusinessProfile,
  CatalogCategory,
  CatalogItem,
  CatalogUnit,
  CatalogWorkspace,
  ImportPreview,
  NewBusinessWorkspaceInput,
  SaveCatalogCategoryInput,
  SaveCatalogItemInput,
  SaveCatalogUnitInput,
  WorkspaceConfigurationInput
} from "./types";

export async function bootstrapApp(): Promise<AppBootstrap> {
  return invoke<AppBootstrap>("bootstrap_app");
}

export async function saveBusinessProfile(
  profile: BusinessProfile
): Promise<BusinessProfile> {
  return invoke<BusinessProfile>("save_business_profile", { profile });
}

export async function createBusinessWorkspace(
  input: NewBusinessWorkspaceInput
): Promise<BusinessProfile> {
  return invoke<BusinessProfile>("create_business_workspace", { input });
}

export async function switchActiveBusiness(
  businessId: string
): Promise<BusinessProfile> {
  return invoke<BusinessProfile>("switch_active_business", {
    businessId
  });
}

export async function saveWorkspaceConfiguration(
  input: WorkspaceConfigurationInput
): Promise<void> {
  return invoke<void>("save_workspace_configuration", { input });
}

export async function loadCatalogWorkspace(): Promise<CatalogWorkspace> {
  return invoke<CatalogWorkspace>("load_catalog_workspace");
}

export async function saveCatalogCategory(
  input: SaveCatalogCategoryInput
): Promise<CatalogCategory> {
  return invoke<CatalogCategory>("save_catalog_category", { input });
}

export async function saveCatalogUnit(
  input: SaveCatalogUnitInput
): Promise<CatalogUnit> {
  return invoke<CatalogUnit>("save_catalog_unit", { input });
}

export async function saveCatalogItem(
  input: SaveCatalogItemInput
): Promise<CatalogItem> {
  return invoke<CatalogItem>("save_catalog_item", { input });
}

export async function setCatalogItemArchived(
  itemId: string,
  archived: boolean
): Promise<CatalogItem> {
  return invoke<CatalogItem>("set_catalog_item_archived", {
    itemId,
    archived
  });
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

## `files/src/shared/types.ts`

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

export interface TaxProfile {
  id: string;
  businessId: string;
  name: string;
  taxLabel: string;
  defaultRate: number;
  pricesIncludeTax: boolean;
  isDefault: boolean;
  updatedAt: string;
}

export interface ReceiptProfile {
  id: string;
  businessId: string;
  name: string;
  footerText: string | null;
  showAddress: boolean;
  showPhone: boolean;
  showEmail: boolean;
  showBusinessCode: boolean;
  paperWidth: string;
  isDefault: boolean;
  updatedAt: string;
}

export interface ModuleFlags {
  businessId: string;
  restaurantEnabled: boolean;
  retailEnabled: boolean;
  inventoryEnabled: boolean;
  servicesEnabled: boolean;
  customersEnabled: boolean;
  suppliersEnabled: boolean;
  expensesEnabled: boolean;
  reportingEnabled: boolean;
  dataCenterEnabled: boolean;
  updatedAt: string;
}

export interface SequenceCounter {
  id: string;
  businessId: string;
  scope: string;
  prefix: string;
  nextNumber: number;
  padding: number;
  resetPolicy: string;
  updatedAt: string;
}

export interface NewBusinessWorkspaceInput {
  name: string;
  legalName: string | null;
  code: string;
  businessType: string;
  currencyCode: string;
  taxMode: string;
  timezone: string;
  locale: string;
  activateNow: boolean;
}

export interface WorkspaceConfigurationInput {
  businessSettings: BusinessSettings;
  taxProfile: TaxProfile;
  receiptProfile: ReceiptProfile;
  moduleFlags: ModuleFlags;
  sequenceCounters: SequenceCounter[];
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
  categoryCount: number;
  itemCount: number;
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

export interface BusinessWorkspaceSummary {
  businessId: string;
  name: string;
  code: string;
  businessType: string;
  currencyCode: string;
  theme: string;
  timezone: string;
  taxLabel: string;
  defaultTaxRate: number;
  nextSaleSequence: string;
  activeModules: string[];
  archivedAt: string | null;
  updatedAt: string;
}

export interface CatalogCategory {
  id: string;
  businessId: string;
  name: string;
  code: string;
  parentId: string | null;
  itemScope: string;
  sortOrder: number;
  notes: string | null;
  createdAt: string;
  updatedAt: string;
  archivedAt: string | null;
}

export interface CatalogUnit {
  id: string;
  businessId: string | null;
  name: string;
  code: string;
  symbol: string;
  allowFractional: boolean;
  isSystem: boolean;
  updatedAt: string;
  archivedAt: string | null;
}

export interface CatalogItem {
  id: string;
  businessId: string;
  categoryId: string | null;
  unitId: string | null;
  taxProfileId: string | null;
  itemKind: string;
  name: string;
  displayName: string | null;
  sku: string | null;
  primaryBarcode: string | null;
  description: string | null;
  sellingPrice: number;
  costPrice: number;
  trackStock: boolean;
  stockQuantity: number;
  reorderLevel: number;
  imagePath: string | null;
  isActive: boolean;
  createdAt: string;
  updatedAt: string;
  archivedAt: string | null;
}

export interface CatalogBarcode {
  id: string;
  itemId: string;
  barcode: string;
  label: string | null;
  isPrimary: boolean;
  createdAt: string;
}

export interface CatalogItemView {
  item: CatalogItem;
  categoryName: string | null;
  unitCode: string | null;
  taxLabel: string | null;
  barcodes: CatalogBarcode[];
}

export interface CatalogSummary {
  totalItems: number;
  activeItems: number;
  archivedItems: number;
  categoryCount: number;
  menuItemCount: number;
  stockItemCount: number;
  serviceItemCount: number;
  lowStockCandidates: number;
}

export interface CatalogWorkspace {
  businessId: string;
  summary: CatalogSummary;
  categories: CatalogCategory[];
  units: CatalogUnit[];
  taxProfiles: TaxProfile[];
  items: CatalogItemView[];
}

export interface SaveCatalogCategoryInput {
  id?: string | null;
  name: string;
  code: string;
  parentId: string | null;
  itemScope: string;
  sortOrder: number;
  notes: string | null;
}

export interface SaveCatalogUnitInput {
  id?: string | null;
  name: string;
  code: string;
  symbol: string;
  allowFractional: boolean;
}

export interface SaveCatalogItemInput {
  id?: string | null;
  categoryId: string | null;
  unitId: string | null;
  taxProfileId: string | null;
  itemKind: string;
  name: string;
  displayName: string | null;
  sku: string | null;
  barcodes: string[];
  description: string | null;
  sellingPrice: number;
  costPrice: number;
  trackStock: boolean;
  stockQuantity: number;
  reorderLevel: number;
  imagePath: string | null;
  isActive: boolean;
}

export interface AppBootstrap {
  appInfo: AppInfo;
  activeBusiness: BusinessProfile;
  businessSettings: BusinessSettings;
  activeTaxProfile: TaxProfile;
  activeReceiptProfile: ReceiptProfile;
  activeModuleFlags: ModuleFlags;
  activeSequences: SequenceCounter[];
  businesses: BusinessProfile[];
  businessWorkspaces: BusinessWorkspaceSummary[];
  patchHistory: PatchRecord[];
  backups: BackupRecord[];
  storage: StorageStatus;
  catalogSummary: CatalogSummary;
  dashboard: DashboardShellData;
}

export type NavPage =
  | "dashboard"
  | "catalog"
  | "business"
  | "settings"
  | "data-center";
```

## `files/src/shared/utils.ts`

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

export function titleCaseWords(value: string): string {
  return value
    .split(/[-_\s]+/)
    .filter(Boolean)
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(" ");
}

export function formatSequencePreview(
  prefix: string,
  nextNumber: number,
  padding: number
): string {
  const normalizedPadding = Math.max(1, Number.isFinite(padding) ? padding : 1);
  const normalizedNumber = Math.max(1, Number.isFinite(nextNumber) ? nextNumber : 1);
  return `${prefix}${String(normalizedNumber).padStart(normalizedPadding, "0")}`;
}

export function formatModuleList(modules: string[]): string {
  if (modules.length === 0) return "No modules enabled";
  return modules.map(titleCaseWords).join(", ");
}

export function formatCurrency(value: number, currencyCode = "INR"): string {
  return new Intl.NumberFormat(undefined, {
    style: "currency",
    currency: currencyCode,
    maximumFractionDigits: 2
  }).format(Number.isFinite(value) ? value : 0);
}

export function linesFromMultilineValue(value: string): string[] {
  return value
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter(Boolean);
}

export function multilineValueFromLines(values: string[]): string {
  return values.join("\n");
}
```

## `files/src/styles.css`

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
  grid-template-columns: 310px minmax(0, 1fr);
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
  width: 2.55rem;
  height: 2.55rem;
  border-radius: 0.85rem;
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
  gap: 0.7rem;
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

.sidebar-field {
  display: grid;
  gap: 0.35rem;
  color: #d8e3fb;
  font-size: 0.88rem;
}

.sidebar-select {
  width: 100%;
  border-radius: 0.8rem;
  border: 1px solid rgba(255, 255, 255, 0.14);
  background: rgba(255, 255, 255, 0.08);
  color: #f5f9ff;
  padding: 0.7rem 0.8rem;
}

.sidebar-select option {
  color: #172033;
}

.sidebar-metadata-grid {
  display: grid;
  gap: 0.55rem;
  grid-template-columns: repeat(2, minmax(0, 1fr));
}

.sidebar-metadata-grid span {
  display: block;
  color: #9fb2d7;
  font-size: 0.78rem;
  margin-bottom: 0.2rem;
}

.sidebar-metadata-grid strong {
  font-size: 0.9rem;
}

.small-text {
  font-size: 0.82rem;
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

.align-start {
  align-items: flex-start;
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

.secondary-button:disabled {
  opacity: 0.65;
  cursor: not-allowed;
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

.card-grid-5 {
  grid-template-columns: repeat(5, minmax(0, 1fr));
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

.compact-card-header {
  margin-bottom: 0.5rem;
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

.compact-detail-list {
  gap: 0.6rem;
}

.two-column-detail-list {
  grid-template-columns: repeat(2, minmax(0, 1fr));
}

.spaced-detail-list {
  margin-top: 1rem;
}

.form-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 0.9rem;
}

.compact-form-grid {
  grid-template-columns: repeat(4, minmax(0, 1fr));
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

.toggle-stack,
.toggle-grid {
  display: grid;
  gap: 0.7rem;
}

.toggle-grid {
  grid-template-columns: repeat(2, minmax(0, 1fr));
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

.directory-grid {
  display: grid;
  gap: 1rem;
}

.workspace-directory-card {
  border: 1px solid #d8e2f2;
  border-radius: 1rem;
  padding: 1rem;
  background: linear-gradient(180deg, #ffffff 0%, #f8fbff 100%);
  display: grid;
  gap: 0.9rem;
}

.workspace-directory-card.active {
  border-color: #8db4ff;
  box-shadow: inset 0 0 0 1px rgba(37, 99, 235, 0.1);
}

.tag-list {
  display: flex;
  flex-wrap: wrap;
  gap: 0.45rem;
}

.compact-tags {
  margin-top: 0.25rem;
}

.tag {
  display: inline-flex;
  align-items: center;
  padding: 0.28rem 0.58rem;
  border-radius: 999px;
  background: #edf3ff;
  border: 1px solid #d6e2ff;
  color: #34508c;
  font-size: 0.8rem;
}

.sequence-list {
  display: grid;
  gap: 0.95rem;
}

.sequence-row {
  border: 1px solid #deebf9;
  border-radius: 1rem;
  padding: 0.95rem;
  background: #fbfdff;
}

.sequence-row-header {
  display: flex;
  justify-content: space-between;
  gap: 0.8rem;
  align-items: flex-start;
  margin-bottom: 0.8rem;
}

.spacing-top {
  margin-top: 1.2rem;
}

@media (max-width: 1280px) {
  .card-grid-5 {
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }

  .compact-form-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}

@media (max-width: 1120px) {
  .card-grid,
  .card-grid-5,
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
  .card-grid-5,
  .form-grid,
  .compact-form-grid,
  .toggle-grid,
  .two-column-detail-list,
  .sidebar-metadata-grid {
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

.card-grid-4 {
  grid-template-columns: repeat(4, minmax(0, 1fr));
}

.catalog-primary-grid,
.catalog-secondary-grid {
  grid-template-columns: 1.4fr 1fr;
}

.catalog-filter-bar {
  display: grid;
  grid-template-columns: minmax(0, 1.8fr) 180px auto;
  gap: 0.75rem;
  margin-bottom: 1rem;
  align-items: center;
}

.inline-toggle {
  min-height: 100%;
  padding: 0.7rem 0.85rem;
  border: 1px solid #d9e4f6;
  border-radius: 0.85rem;
  background: #fbfdff;
}

.catalog-list {
  display: grid;
  gap: 0.9rem;
}

.catalog-item-card {
  border: 1px solid #dbe5f5;
  border-radius: 1rem;
  padding: 1rem;
  background: linear-gradient(180deg, #ffffff 0%, #f9fbff 100%);
}

.catalog-item-description {
  margin: 0.65rem 0 0;
  color: #43516b;
  font-size: 0.94rem;
}

.compact-stack-list {
  gap: 0.55rem;
}

@media (max-width: 1280px) {
  .card-grid-4 {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}

@media (max-width: 1120px) {
  .catalog-primary-grid,
  .catalog-secondary-grid {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 920px) {
  .catalog-filter-bar,
  .card-grid-4 {
    grid-template-columns: 1fr;
  }
}
```

## `files/src-tauri/Cargo.toml`

```toml
[package]
name = "local-business-manager"
version = "0.3.0"
description = "Local-first business management desktop app catalog core"
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

## `files/src-tauri/src/commands/bootstrap.rs`

```rust
use tauri::AppHandle;

use crate::{
    core::{catalog, db, error::CommandResult},
    domain::bootstrap::build_app_bootstrap,
};

#[tauri::command]
pub fn bootstrap_app(app: AppHandle) -> CommandResult<crate::domain::models::AppBootstrap> {
    db::with_connection(&app, |conn, paths| {
        let app_info = db::load_app_info(conn)?;
        let active_business = db::get_active_business(conn)?;
        let business_settings = db::get_business_settings(conn, &active_business.id)?;
        let active_tax_profile = db::get_default_tax_profile(conn, &active_business.id)?;
        let active_receipt_profile = db::get_default_receipt_profile(conn, &active_business.id)?;
        let active_module_flags = db::get_module_flags(conn, &active_business.id)?;
        let active_sequences = db::list_sequence_counters(conn, &active_business.id)?;
        let businesses = db::list_businesses(conn)?;
        let business_workspaces = db::list_business_workspace_summaries(conn)?;
        let patch_history = db::list_patch_history(conn)?;
        let backups = db::list_backups(conn)?;
        let storage = db::build_storage_status(conn, paths)?;
        let recent_activity = db::list_recent_activity(conn, 8)?;
        let catalog_summary = catalog::build_catalog_summary(conn, &active_business.id)?;

        Ok(build_app_bootstrap(
            app_info,
            active_business,
            business_settings,
            active_tax_profile,
            active_receipt_profile,
            active_module_flags,
            active_sequences,
            businesses,
            business_workspaces,
            patch_history,
            backups,
            storage,
            recent_activity,
            catalog_summary,
        ))
    })
}
```

## `files/src-tauri/src/commands/catalog.rs`

```rust
use tauri::AppHandle;

use crate::{
    core::{catalog, db, error::CommandResult},
    domain::models::{
        CatalogCategory, CatalogItem, CatalogUnit, CatalogWorkspace, SaveCatalogCategoryInput,
        SaveCatalogItemInput, SaveCatalogUnitInput,
    },
};

#[tauri::command]
pub fn load_catalog_workspace(app: AppHandle) -> CommandResult<CatalogWorkspace> {
    db::with_connection(&app, |conn, _paths| {
        let active_business = db::get_active_business(conn)?;
        catalog::load_catalog_workspace(conn, &active_business.id)
    })
}

#[tauri::command]
pub fn save_catalog_category(
    app: AppHandle,
    input: SaveCatalogCategoryInput,
) -> CommandResult<CatalogCategory> {
    db::with_connection(&app, |conn, _paths| {
        let active_business = db::get_active_business(conn)?;
        catalog::save_catalog_category(conn, &active_business.id, &input)
    })
}

#[tauri::command]
pub fn save_catalog_unit(
    app: AppHandle,
    input: SaveCatalogUnitInput,
) -> CommandResult<CatalogUnit> {
    db::with_connection(&app, |conn, _paths| {
        let active_business = db::get_active_business(conn)?;
        catalog::save_catalog_unit(conn, &active_business.id, &input)
    })
}

#[tauri::command]
pub fn save_catalog_item(
    app: AppHandle,
    input: SaveCatalogItemInput,
) -> CommandResult<CatalogItem> {
    db::with_connection(&app, |conn, _paths| {
        let active_business = db::get_active_business(conn)?;
        catalog::save_catalog_item(conn, &active_business.id, &input)
    })
}

#[tauri::command]
pub fn set_catalog_item_archived(
    app: AppHandle,
    item_id: String,
    archived: bool,
) -> CommandResult<CatalogItem> {
    db::with_connection(&app, |conn, _paths| {
        let active_business = db::get_active_business(conn)?;
        catalog::set_catalog_item_archived(conn, &active_business.id, &item_id, archived)
    })
}
```

## `files/src-tauri/src/commands/data_center.rs`

```rust
use std::{fs, path::PathBuf};

use chrono::Utc;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use tauri::AppHandle;
use uuid::Uuid;

use crate::{
    core::{
        catalog, db,
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
    let file_name = format!("workspace_snapshot_{timestamp}.sqlite");
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

    let (
        app_info,
        active_business,
        businesses,
        business_settings,
        tax_profiles,
        receipt_profiles,
        module_flags,
        sequence_counters,
        patch_history,
        backups,
        catalog_categories,
        catalog_units,
        catalog_items,
        catalog_barcodes,
    ) = db::with_connection(&app, |conn, _paths| {
        let active_business = db::get_active_business(conn)?;

        Ok((
            db::load_app_info(conn)?,
            active_business,
            db::list_businesses(conn)?,
            db::list_all_business_settings(conn)?,
            db::list_all_tax_profiles(conn)?,
            db::list_all_receipt_profiles(conn)?,
            db::list_all_module_flags(conn)?,
            db::list_all_sequence_counters(conn)?,
            db::list_patch_history(conn)?,
            db::list_backups(conn)?,
            catalog::list_all_catalog_categories(conn)?,
            catalog::list_all_catalog_units(conn)?,
            catalog::list_all_catalog_items(conn)?,
            catalog::list_all_catalog_barcodes(conn)?,
        ))
    })?;

    let generated_at = db::now_iso();
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let export_path = PathBuf::from(&paths.export_dir).join(format!("workspace_export_{timestamp}.json"));

    let source_patch_level = app_info.patch_level.clone();
    let product_name = app_info.product_name.clone();
    let active_business_id = active_business.id.clone();

    let bundle = json!({
        "manifest": {
            "bundleVersion": "3.0.0",
            "bundleType": "workspace-foundation-export",
            "sourcePatchLevel": source_patch_level,
            "schemaVersion": app_info.schema_version,
            "generatedAt": generated_at.clone(),
            "productName": product_name
        },
        "appInfo": app_info,
        "activeBusinessId": active_business_id.clone(),
        "businesses": businesses,
        "businessSettings": business_settings,
        "taxProfiles": tax_profiles,
        "receiptProfiles": receipt_profiles,
        "moduleFlags": module_flags,
        "sequenceCounters": sequence_counters,
        "patchHistory": patch_history,
        "backupRecords": backups,
        "catalogCategories": catalog_categories,
        "catalogUnits": catalog_units,
        "catalogItems": catalog_items,
        "catalogBarcodes": catalog_barcodes
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
        format: "json-workspace-foundation-v3".into(),
        status: "completed".into(),
        target_path: Some(export_path.to_string_lossy().to_string()),
        created_at: generated_at.clone(),
        completed_at: Some(generated_at),
    };

    db::with_connection(&app, |conn, _paths| {
        db::insert_export_job(conn, &export_job)?;
        db::insert_log(conn, "INFO", "export", "Workspace export bundle created", None)?;
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

    let categories = parsed
        .get("catalogCategories")
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default();

    let items = parsed
        .get("catalogItems")
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

    if parsed.get("businessSettings").is_none() {
        warnings.push("Bundle is missing businessSettings".into());
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
        category_count: categories.len(),
        item_count: items.len(),
        generated_at: manifest
            .get("generatedAt")
            .and_then(|value| value.as_str())
            .map(ToString::to_string),
        warnings,
    };

    db::with_connection(&app, |conn, _paths| {
        db::insert_import_job(conn, None, "json-workspace-foundation-v3", "previewed", &file_path)?;
        db::insert_log(conn, "INFO", "import", "Import bundle previewed", None)?;
        Ok(())
    })?;

    Ok(preview)
}
```

## `files/src-tauri/src/commands/mod.rs`

```rust
pub mod bootstrap;
pub mod business;
pub mod catalog;
pub mod data_center;
pub mod settings;
```

## `files/src-tauri/src/core/catalog.rs`

```rust
use std::collections::HashSet;

use rusqlite::{params, Connection, OptionalExtension, Row};
use uuid::Uuid;

use crate::domain::models::{
    CatalogBarcode, CatalogCategory, CatalogItem, CatalogItemView, CatalogSummary, CatalogUnit,
    CatalogWorkspace, SaveCatalogCategoryInput, SaveCatalogItemInput, SaveCatalogUnitInput,
};

use super::{db, error::to_command_error};

const SYSTEM_UNITS: [(&str, &str, &str, bool); 7] = [
    ("system-pcs", "Pieces", "PCS", false),
    ("system-kg", "Kilogram", "KG", true),
    ("system-g", "Gram", "G", true),
    ("system-litre", "Litre", "LTR", true),
    ("system-ml", "Millilitre", "ML", true),
    ("system-hour", "Hour", "HR", true),
    ("system-plate", "Plate", "PLATE", false),
];

fn bool_from_row(row: &Row, index: usize) -> rusqlite::Result<bool> {
    let value: i64 = row.get(index)?;
    Ok(value != 0)
}

fn bool_to_i64(value: bool) -> i64 {
    if value {
        1
    } else {
        0
    }
}

fn normalize_optional(value: &Option<String>) -> Option<String> {
    value.as_ref().and_then(|raw| {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

fn normalize_text(value: &str, field: &str) -> Result<String, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(format!("{field} cannot be empty"));
    }
    Ok(trimmed.to_string())
}

fn normalize_code(value: &str, field: &str) -> Result<String, String> {
    Ok(normalize_text(value, field)?.to_uppercase())
}

fn normalize_item_kind(value: &str) -> String {
    match value.trim().to_lowercase().as_str() {
        "menu" => "menu".into(),
        "service" => "service".into(),
        _ => "stock".into(),
    }
}

fn normalize_scope(value: &str) -> String {
    match value.trim().to_lowercase().as_str() {
        "menu" => "menu".into(),
        "stock" => "stock".into(),
        "service" => "service".into(),
        _ => "all".into(),
    }
}

fn category_from_row(row: &Row) -> rusqlite::Result<CatalogCategory> {
    Ok(CatalogCategory {
        id: row.get(0)?,
        business_id: row.get(1)?,
        name: row.get(2)?,
        code: row.get(3)?,
        parent_id: row.get(4)?,
        item_scope: row.get(5)?,
        sort_order: row.get(6)?,
        notes: row.get(7)?,
        created_at: row.get(8)?,
        updated_at: row.get(9)?,
        archived_at: row.get(10)?,
    })
}

fn unit_from_row(row: &Row) -> rusqlite::Result<CatalogUnit> {
    Ok(CatalogUnit {
        id: row.get(0)?,
        business_id: row.get(1)?,
        name: row.get(2)?,
        code: row.get(3)?,
        symbol: row.get(4)?,
        allow_fractional: bool_from_row(row, 5)?,
        is_system: bool_from_row(row, 6)?,
        updated_at: row.get(7)?,
        archived_at: row.get(8)?,
    })
}

fn item_from_row(row: &Row) -> rusqlite::Result<CatalogItem> {
    Ok(CatalogItem {
        id: row.get(0)?,
        business_id: row.get(1)?,
        category_id: row.get(2)?,
        unit_id: row.get(3)?,
        tax_profile_id: row.get(4)?,
        item_kind: row.get(5)?,
        name: row.get(6)?,
        display_name: row.get(7)?,
        sku: row.get(8)?,
        primary_barcode: row.get(9)?,
        description: row.get(10)?,
        selling_price: row.get(11)?,
        cost_price: row.get(12)?,
        track_stock: bool_from_row(row, 13)?,
        stock_quantity: row.get(14)?,
        reorder_level: row.get(15)?,
        image_path: row.get(16)?,
        is_active: bool_from_row(row, 17)?,
        created_at: row.get(18)?,
        updated_at: row.get(19)?,
        archived_at: row.get(20)?,
    })
}

fn barcode_from_row(row: &Row) -> rusqlite::Result<CatalogBarcode> {
    Ok(CatalogBarcode {
        id: row.get(0)?,
        item_id: row.get(1)?,
        barcode: row.get(2)?,
        label: row.get(3)?,
        is_primary: bool_from_row(row, 4)?,
        created_at: row.get(5)?,
    })
}

fn load_category_name(conn: &Connection, category_id: &Option<String>) -> Result<Option<String>, String> {
    let Some(category_id) = category_id else {
        return Ok(None);
    };

    conn.query_row(
        "SELECT name FROM catalog_categories WHERE id = ?1 LIMIT 1",
        params![category_id],
        |row| row.get::<_, String>(0),
    )
    .optional()
    .map_err(|error| to_command_error("failed to load category name", error))
}

fn load_unit_code(conn: &Connection, unit_id: &Option<String>) -> Result<Option<String>, String> {
    let Some(unit_id) = unit_id else {
        return Ok(None);
    };

    conn.query_row(
        "SELECT code FROM catalog_units WHERE id = ?1 LIMIT 1",
        params![unit_id],
        |row| row.get::<_, String>(0),
    )
    .optional()
    .map_err(|error| to_command_error("failed to load unit code", error))
}

fn load_tax_label(conn: &Connection, tax_profile_id: &Option<String>) -> Result<Option<String>, String> {
    let Some(tax_profile_id) = tax_profile_id else {
        return Ok(None);
    };

    conn.query_row(
        "SELECT label FROM tax_profiles WHERE id = ?1 LIMIT 1",
        params![tax_profile_id],
        |row| row.get::<_, String>(0),
    )
    .optional()
    .map_err(|error| to_command_error("failed to load item tax label", error))
}

fn list_barcodes_for_item(conn: &Connection, item_id: &str) -> Result<Vec<CatalogBarcode>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, item_id, barcode, label, is_primary, created_at
             FROM catalog_item_barcodes
             WHERE item_id = ?1
             ORDER BY is_primary DESC, created_at ASC",
        )
        .map_err(|error| to_command_error("failed to prepare barcode query", error))?;

    let rows = stmt
        .query_map(params![item_id], barcode_from_row)
        .map_err(|error| to_command_error("failed to query barcodes", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map barcodes", error))
}

pub fn ensure_system_units(conn: &Connection) -> rusqlite::Result<()> {
    let now = db::now_iso();
    for (id, name, code, allow_fractional) in SYSTEM_UNITS {
        conn.execute(
            "INSERT OR IGNORE INTO catalog_units (
                id, business_id, name, code, symbol, allow_fractional,
                is_system, created_at, updated_at, archived_at
             ) VALUES (
                ?1, NULL, ?2, ?3, ?4, ?5,
                1, ?6, ?7, NULL
             )",
            params![
                id,
                name,
                code,
                code,
                bool_to_i64(allow_fractional),
                &now,
                &now,
            ],
        )?;
    }
    Ok(())
}

pub fn seed_demo_catalog_for_business(conn: &Connection, business_id: &str) -> rusqlite::Result<()> {
    let item_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM catalog_items WHERE business_id = ?1",
        params![business_id],
        |row| row.get(0),
    )?;
    if item_count > 0 {
        return Ok(());
    }

    ensure_system_units(conn)?;

    let now = db::now_iso();
    let id_suffix = business_id.chars().take(6).collect::<String>().to_lowercase();
    let categories = [
        (
            format!("{business_id}-cat-beverages"),
            "Beverages",
            "CAT-BEV",
            "menu",
            10_i64,
            Some("Coffee, tea, juices, and drinks".to_string()),
        ),
        (
            format!("{business_id}-cat-food"),
            "Bakery & Food",
            "CAT-FOOD",
            "menu",
            20_i64,
            Some("Baked goods and ready-to-serve food".to_string()),
        ),
        (
            format!("{business_id}-cat-retail"),
            "Retail Shelf",
            "CAT-RTL",
            "stock",
            30_i64,
            Some("Packaged goods and shelf products".to_string()),
        ),
        (
            format!("{business_id}-cat-service"),
            "Services",
            "CAT-SVC",
            "service",
            40_i64,
            Some("Simple service charges".to_string()),
        ),
    ];

    for (id, name, code, item_scope, sort_order, notes) in categories {
        conn.execute(
            "INSERT OR IGNORE INTO catalog_categories (
                id, business_id, name, code, parent_id, item_scope, sort_order,
                notes, created_at, updated_at, archived_at
             ) VALUES (?1, ?2, ?3, ?4, NULL, ?5, ?6, ?7, ?8, ?9, NULL)",
            params![
                id,
                business_id,
                name,
                code,
                item_scope,
                sort_order,
                notes,
                &now,
                &now,
            ],
        )?;
    }

    let default_tax_profile_id: Option<String> = conn
        .query_row(
            "SELECT id FROM tax_profiles WHERE business_id = ?1 ORDER BY is_default DESC, updated_at DESC LIMIT 1",
            params![business_id],
            |row| row.get(0),
        )
        .optional()?;

    let items = [
        (
            format!("{business_id}-item-cappuccino"),
            format!("{business_id}-cat-beverages"),
            Some("system-pcs".to_string()),
            default_tax_profile_id.clone(),
            "menu",
            "House Cappuccino",
            Some("Cappuccino".to_string()),
            Some("MENU-CAP".to_string()),
            Some(vec![format!("MENU-{id_suffix}-001")]),
            Some("Demo menu item for faster catalog validation.".to_string()),
            120.0_f64,
            35.0_f64,
            false,
            0.0_f64,
            0.0_f64,
        ),
        (
            format!("{business_id}-item-croissant"),
            format!("{business_id}-cat-food"),
            Some("system-pcs".to_string()),
            default_tax_profile_id.clone(),
            "menu",
            "Butter Croissant",
            None,
            Some("MENU-CROI".to_string()),
            Some(vec![format!("MENU-{id_suffix}-002")]),
            Some("Demo bakery item.".to_string()),
            90.0_f64,
            28.0_f64,
            false,
            0.0_f64,
            0.0_f64,
        ),
        (
            format!("{business_id}-item-water"),
            format!("{business_id}-cat-retail"),
            Some("system-pcs".to_string()),
            default_tax_profile_id.clone(),
            "stock",
            "Mineral Water 1L",
            None,
            Some("RTL-WATER-1L".to_string()),
            Some(vec![format!("890{id_suffix}1001")]),
            Some("Demo stock item with reorder tracking.".to_string()),
            25.0_f64,
            12.0_f64,
            true,
            24.0_f64,
            8.0_f64,
        ),
        (
            format!("{business_id}-item-delivery"),
            format!("{business_id}-cat-service"),
            Some("system-hour".to_string()),
            default_tax_profile_id.clone(),
            "service",
            "Local Delivery Charge",
            Some("Delivery".to_string()),
            Some("SVC-DEL".to_string()),
            None,
            Some("Demo service item.".to_string()),
            50.0_f64,
            0.0_f64,
            false,
            0.0_f64,
            0.0_f64,
        ),
    ];

    for (
        item_id,
        category_id,
        unit_id,
        tax_profile_id,
        item_kind,
        name,
        display_name,
        sku,
        barcodes,
        description,
        selling_price,
        cost_price,
        track_stock,
        stock_quantity,
        reorder_level,
    ) in items {
        conn.execute(
            "INSERT OR IGNORE INTO catalog_items (
                id, business_id, category_id, unit_id, tax_profile_id, item_kind,
                name, display_name, sku, primary_barcode, description,
                selling_price, cost_price, track_stock, stock_quantity,
                reorder_level, image_path, is_active, created_at, updated_at, archived_at
             ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6,
                ?7, ?8, ?9, ?10, ?11,
                ?12, ?13, ?14, ?15,
                ?16, NULL, 1, ?17, ?18, NULL
             )",
            params![
                &item_id,
                business_id,
                &category_id,
                unit_id,
                tax_profile_id,
                item_kind,
                name,
                display_name,
                sku,
                barcodes.as_ref().and_then(|list| list.first().cloned()),
                description,
                selling_price,
                cost_price,
                bool_to_i64(track_stock),
                stock_quantity,
                reorder_level,
                &now,
                &now,
            ],
        )?;

        if let Some(barcodes) = barcodes {
            for (index, barcode) in barcodes.iter().enumerate() {
                conn.execute(
                    "INSERT OR IGNORE INTO catalog_item_barcodes (
                        id, item_id, barcode, label, is_primary, created_at
                     ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                    params![
                        format!("{item_id}-barcode-{}", index + 1),
                        &item_id,
                        barcode,
                        Some("demo".to_string()),
                        bool_to_i64(index == 0),
                        &now,
                    ],
                )?;
            }
        }
    }

    Ok(())
}

pub fn list_catalog_categories(conn: &Connection, business_id: &str) -> Result<Vec<CatalogCategory>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, business_id, name, code, parent_id, item_scope, sort_order, notes, created_at, updated_at, archived_at
             FROM catalog_categories
             WHERE business_id = ?1
             ORDER BY CASE WHEN archived_at IS NULL THEN 0 ELSE 1 END, sort_order ASC, name COLLATE NOCASE ASC",
        )
        .map_err(|error| to_command_error("failed to prepare category query", error))?;

    let rows = stmt
        .query_map(params![business_id], category_from_row)
        .map_err(|error| to_command_error("failed to query catalog categories", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map catalog categories", error))
}

pub fn list_catalog_units(conn: &Connection, business_id: &str) -> Result<Vec<CatalogUnit>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, business_id, name, code, symbol, allow_fractional, is_system, updated_at, archived_at
             FROM catalog_units
             WHERE archived_at IS NULL AND (business_id IS NULL OR business_id = ?1)
             ORDER BY CASE WHEN business_id IS NULL THEN 0 ELSE 1 END, name COLLATE NOCASE ASC",
        )
        .map_err(|error| to_command_error("failed to prepare unit query", error))?;

    let rows = stmt
        .query_map(params![business_id], unit_from_row)
        .map_err(|error| to_command_error("failed to query catalog units", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map catalog units", error))
}

pub fn list_catalog_items(conn: &Connection, business_id: &str) -> Result<Vec<CatalogItem>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                id, business_id, category_id, unit_id, tax_profile_id, item_kind,
                name, display_name, sku, primary_barcode, description,
                selling_price, cost_price, track_stock, stock_quantity,
                reorder_level, image_path, is_active, created_at, updated_at, archived_at
             FROM catalog_items
             WHERE business_id = ?1
             ORDER BY CASE WHEN archived_at IS NULL THEN 0 ELSE 1 END, updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare item query", error))?;

    let rows = stmt
        .query_map(params![business_id], item_from_row)
        .map_err(|error| to_command_error("failed to query catalog items", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map catalog items", error))
}

fn get_catalog_item(conn: &Connection, business_id: &str, item_id: &str) -> Result<CatalogItem, String> {
    conn.query_row(
        "SELECT
            id, business_id, category_id, unit_id, tax_profile_id, item_kind,
            name, display_name, sku, primary_barcode, description,
            selling_price, cost_price, track_stock, stock_quantity,
            reorder_level, image_path, is_active, created_at, updated_at, archived_at
         FROM catalog_items
         WHERE business_id = ?1 AND id = ?2
         LIMIT 1",
        params![business_id, item_id],
        item_from_row,
    )
    .map_err(|error| to_command_error("failed to load catalog item", error))
}

pub fn build_catalog_summary(conn: &Connection, business_id: &str) -> Result<CatalogSummary, String> {
    let total_items: usize = conn
        .query_row(
            "SELECT COUNT(*) FROM catalog_items WHERE business_id = ?1",
            params![business_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| to_command_error("failed to count catalog items", error))?
        as usize;

    let active_items: usize = conn
        .query_row(
            "SELECT COUNT(*) FROM catalog_items WHERE business_id = ?1 AND archived_at IS NULL AND is_active = 1",
            params![business_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| to_command_error("failed to count active items", error))?
        as usize;

    let archived_items: usize = conn
        .query_row(
            "SELECT COUNT(*) FROM catalog_items WHERE business_id = ?1 AND archived_at IS NOT NULL",
            params![business_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| to_command_error("failed to count archived items", error))?
        as usize;

    let category_count: usize = conn
        .query_row(
            "SELECT COUNT(*) FROM catalog_categories WHERE business_id = ?1 AND archived_at IS NULL",
            params![business_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| to_command_error("failed to count categories", error))?
        as usize;

    let menu_item_count: usize = conn
        .query_row(
            "SELECT COUNT(*) FROM catalog_items WHERE business_id = ?1 AND item_kind = 'menu' AND archived_at IS NULL",
            params![business_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| to_command_error("failed to count menu items", error))?
        as usize;

    let stock_item_count: usize = conn
        .query_row(
            "SELECT COUNT(*) FROM catalog_items WHERE business_id = ?1 AND item_kind = 'stock' AND archived_at IS NULL",
            params![business_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| to_command_error("failed to count stock items", error))?
        as usize;

    let service_item_count: usize = conn
        .query_row(
            "SELECT COUNT(*) FROM catalog_items WHERE business_id = ?1 AND item_kind = 'service' AND archived_at IS NULL",
            params![business_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| to_command_error("failed to count service items", error))?
        as usize;

    let low_stock_candidates: usize = conn
        .query_row(
            "SELECT COUNT(*)
             FROM catalog_items
             WHERE business_id = ?1
               AND archived_at IS NULL
               AND is_active = 1
               AND track_stock = 1
               AND reorder_level > 0
               AND stock_quantity <= reorder_level",
            params![business_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| to_command_error("failed to count low stock candidates", error))?
        as usize;

    Ok(CatalogSummary {
        total_items,
        active_items,
        archived_items,
        category_count,
        menu_item_count,
        stock_item_count,
        service_item_count,
        low_stock_candidates,
    })
}

pub fn load_catalog_workspace(conn: &Connection, business_id: &str) -> Result<CatalogWorkspace, String> {
    let items = list_catalog_items(conn, business_id)?;
    let item_views = items
        .into_iter()
        .map(|item| {
            let category_name = load_category_name(conn, &item.category_id)?;
            let unit_code = load_unit_code(conn, &item.unit_id)?;
            let tax_label = load_tax_label(conn, &item.tax_profile_id)?;
            let barcodes = list_barcodes_for_item(conn, &item.id)?;
            Ok(CatalogItemView {
                category_name,
                unit_code,
                tax_label,
                barcodes,
                item,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;

    Ok(CatalogWorkspace {
        business_id: business_id.to_string(),
        summary: build_catalog_summary(conn, business_id)?,
        categories: list_catalog_categories(conn, business_id)?,
        units: list_catalog_units(conn, business_id)?,
        tax_profiles: db::list_tax_profiles(conn, business_id)?,
        items: item_views,
    })
}

fn validate_category_scope(conn: &Connection, business_id: &str, category_id: &Option<String>) -> Result<(), String> {
    if let Some(category_id) = category_id {
        let exists: Option<String> = conn
            .query_row(
                "SELECT id FROM catalog_categories WHERE business_id = ?1 AND id = ?2 LIMIT 1",
                params![business_id, category_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(|error| to_command_error("failed to validate category", error))?;
        if exists.is_none() {
            return Err("selected category does not belong to the active business".into());
        }
    }
    Ok(())
}

fn validate_unit_scope(conn: &Connection, business_id: &str, unit_id: &Option<String>) -> Result<(), String> {
    if let Some(unit_id) = unit_id {
        let exists: Option<String> = conn
            .query_row(
                "SELECT id FROM catalog_units WHERE id = ?1 AND archived_at IS NULL AND (business_id IS NULL OR business_id = ?2) LIMIT 1",
                params![unit_id, business_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(|error| to_command_error("failed to validate unit", error))?;
        if exists.is_none() {
            return Err("selected unit is not available in the active business".into());
        }
    }
    Ok(())
}

fn validate_tax_scope(conn: &Connection, business_id: &str, tax_profile_id: &Option<String>) -> Result<(), String> {
    if let Some(tax_profile_id) = tax_profile_id {
        let exists: Option<String> = conn
            .query_row(
                "SELECT id FROM tax_profiles WHERE business_id = ?1 AND id = ?2 LIMIT 1",
                params![business_id, tax_profile_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(|error| to_command_error("failed to validate tax profile", error))?;
        if exists.is_none() {
            return Err("selected tax profile does not belong to the active business".into());
        }
    }
    Ok(())
}

pub fn save_catalog_category(
    conn: &Connection,
    business_id: &str,
    input: &SaveCatalogCategoryInput,
) -> Result<CatalogCategory, String> {
    let name = normalize_text(&input.name, "category name")?;
    let code = normalize_code(&input.code, "category code")?;
    let item_scope = normalize_scope(&input.item_scope);
    let now = db::now_iso();
    let category_id = input.id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());
    let parent_id = normalize_optional(&input.parent_id);
    validate_category_scope(conn, business_id, &parent_id)?;

    if parent_id.as_deref() == Some(category_id.as_str()) {
        return Err("a category cannot be its own parent".into());
    }

    conn.execute(
        "INSERT INTO catalog_categories (
            id, business_id, name, code, parent_id, item_scope, sort_order,
            notes, created_at, updated_at, archived_at
         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, NULL)
         ON CONFLICT(id) DO UPDATE SET
            name = excluded.name,
            code = excluded.code,
            parent_id = excluded.parent_id,
            item_scope = excluded.item_scope,
            sort_order = excluded.sort_order,
            notes = excluded.notes,
            updated_at = excluded.updated_at",
        params![
            &category_id,
            business_id,
            &name,
            &code,
            parent_id,
            &item_scope,
            input.sort_order.max(0),
            normalize_optional(&input.notes),
            &now,
            &now,
        ],
    )
    .map_err(|error| to_command_error("failed to save catalog category", error))?;

    db::insert_log(conn, "INFO", "catalog", "Catalog category saved", None)?;

    conn.query_row(
        "SELECT id, business_id, name, code, parent_id, item_scope, sort_order, notes, created_at, updated_at, archived_at
         FROM catalog_categories
         WHERE business_id = ?1 AND id = ?2
         LIMIT 1",
        params![business_id, &category_id],
        category_from_row,
    )
    .map_err(|error| to_command_error("failed to reload catalog category", error))
}

pub fn save_catalog_unit(
    conn: &Connection,
    business_id: &str,
    input: &SaveCatalogUnitInput,
) -> Result<CatalogUnit, String> {
    let name = normalize_text(&input.name, "unit name")?;
    let code = normalize_code(&input.code, "unit code")?;
    let symbol = normalize_text(&input.symbol, "unit symbol")?;
    let now = db::now_iso();
    let unit_id = input.id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());

    let system_unit: Option<String> = conn
        .query_row(
            "SELECT id FROM catalog_units WHERE id = ?1 AND is_system = 1 LIMIT 1",
            params![&unit_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|error| to_command_error("failed to validate unit update", error))?;
    if system_unit.is_some() {
        return Err("system units cannot be edited".into());
    }

    let duplicate: Option<String> = conn
        .query_row(
            "SELECT id
             FROM catalog_units
             WHERE archived_at IS NULL
               AND code = ?1
               AND ((business_id = ?2 AND is_system = 0) OR business_id IS NULL)
               AND id != ?3
             LIMIT 1",
            params![&code, business_id, &unit_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|error| to_command_error("failed to validate unit code", error))?;
    if duplicate.is_some() {
        return Err("another unit already uses this code".into());
    }

    conn.execute(
        "INSERT INTO catalog_units (
            id, business_id, name, code, symbol, allow_fractional,
            is_system, created_at, updated_at, archived_at
         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0, ?7, ?8, NULL)
         ON CONFLICT(id) DO UPDATE SET
            name = excluded.name,
            code = excluded.code,
            symbol = excluded.symbol,
            allow_fractional = excluded.allow_fractional,
            updated_at = excluded.updated_at",
        params![
            &unit_id,
            business_id,
            &name,
            &code,
            &symbol,
            bool_to_i64(input.allow_fractional),
            &now,
            &now,
        ],
    )
    .map_err(|error| to_command_error("failed to save catalog unit", error))?;

    db::insert_log(conn, "INFO", "catalog", "Catalog unit saved", None)?;

    conn.query_row(
        "SELECT id, business_id, name, code, symbol, allow_fractional, is_system, updated_at, archived_at
         FROM catalog_units
         WHERE id = ?1 AND (business_id = ?2 OR business_id IS NULL)
         LIMIT 1",
        params![&unit_id, business_id],
        unit_from_row,
    )
    .map_err(|error| to_command_error("failed to reload catalog unit", error))
}

fn normalize_barcodes(values: &[String]) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut barcodes = Vec::new();
    for value in values {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            continue;
        }
        let normalized = trimmed.to_string();
        if seen.insert(normalized.clone()) {
            barcodes.push(normalized);
        }
    }
    barcodes
}

pub fn save_catalog_item(
    conn: &Connection,
    business_id: &str,
    input: &SaveCatalogItemInput,
) -> Result<CatalogItem, String> {
    let item_id = input.id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());
    let name = normalize_text(&input.name, "item name")?;
    let item_kind = normalize_item_kind(&input.item_kind);
    let category_id = normalize_optional(&input.category_id);
    let unit_id = normalize_optional(&input.unit_id);
    let tax_profile_id = normalize_optional(&input.tax_profile_id);
    validate_category_scope(conn, business_id, &category_id)?;
    validate_unit_scope(conn, business_id, &unit_id)?;
    validate_tax_scope(conn, business_id, &tax_profile_id)?;
    let display_name = normalize_optional(&input.display_name);
    let sku = input.sku.as_ref().map(|value| value.trim().to_uppercase()).filter(|value| !value.is_empty());
    let description = normalize_optional(&input.description);
    let image_path = normalize_optional(&input.image_path);
    let barcodes = normalize_barcodes(&input.barcodes);
    let primary_barcode = barcodes.first().cloned();
    let selling_price = input.selling_price.max(0.0);
    let cost_price = input.cost_price.max(0.0);
    let track_stock = input.track_stock && item_kind == "stock";
    let stock_quantity = if track_stock { input.stock_quantity.max(0.0) } else { 0.0 };
    let reorder_level = if track_stock { input.reorder_level.max(0.0) } else { 0.0 };
    let now = db::now_iso();

    conn.execute(
        "INSERT INTO catalog_items (
            id, business_id, category_id, unit_id, tax_profile_id, item_kind,
            name, display_name, sku, primary_barcode, description,
            selling_price, cost_price, track_stock, stock_quantity,
            reorder_level, image_path, is_active, created_at, updated_at, archived_at
         ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6,
            ?7, ?8, ?9, ?10, ?11,
            ?12, ?13, ?14, ?15,
            ?16, ?17, ?18, ?19, ?20, NULL
         )
         ON CONFLICT(id) DO UPDATE SET
            category_id = excluded.category_id,
            unit_id = excluded.unit_id,
            tax_profile_id = excluded.tax_profile_id,
            item_kind = excluded.item_kind,
            name = excluded.name,
            display_name = excluded.display_name,
            sku = excluded.sku,
            primary_barcode = excluded.primary_barcode,
            description = excluded.description,
            selling_price = excluded.selling_price,
            cost_price = excluded.cost_price,
            track_stock = excluded.track_stock,
            stock_quantity = excluded.stock_quantity,
            reorder_level = excluded.reorder_level,
            image_path = excluded.image_path,
            is_active = excluded.is_active,
            updated_at = excluded.updated_at",
        params![
            &item_id,
            business_id,
            category_id,
            unit_id,
            tax_profile_id,
            &item_kind,
            &name,
            display_name,
            sku,
            primary_barcode,
            description,
            selling_price,
            cost_price,
            bool_to_i64(track_stock),
            stock_quantity,
            reorder_level,
            image_path,
            bool_to_i64(input.is_active),
            &now,
            &now,
        ],
    )
    .map_err(|error| to_command_error("failed to save catalog item", error))?;

    conn.execute(
        "DELETE FROM catalog_item_barcodes WHERE item_id = ?1",
        params![&item_id],
    )
    .map_err(|error| to_command_error("failed to reset item barcodes", error))?;

    for (index, barcode) in barcodes.iter().enumerate() {
        conn.execute(
            "INSERT INTO catalog_item_barcodes (id, item_id, barcode, label, is_primary, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                Uuid::new_v4().to_string(),
                &item_id,
                barcode,
                Some("manual".to_string()),
                bool_to_i64(index == 0),
                &now,
            ],
        )
        .map_err(|error| to_command_error("failed to save item barcode", error))?;
    }

    db::insert_log(conn, "INFO", "catalog", "Catalog item saved", None)?;
    get_catalog_item(conn, business_id, &item_id)
}

pub fn set_catalog_item_archived(
    conn: &Connection,
    business_id: &str,
    item_id: &str,
    archived: bool,
) -> Result<CatalogItem, String> {
    let archived_at = if archived { Some(db::now_iso()) } else { None };
    conn.execute(
        "UPDATE catalog_items
         SET archived_at = ?3,
             is_active = ?4,
             updated_at = ?5
         WHERE business_id = ?1 AND id = ?2",
        params![
            business_id,
            item_id,
            archived_at,
            bool_to_i64(!archived),
            db::now_iso(),
        ],
    )
    .map_err(|error| to_command_error("failed to archive catalog item", error))?;

    db::insert_log(conn, "INFO", "catalog", "Catalog item archive state changed", None)?;
    get_catalog_item(conn, business_id, item_id)
}

pub fn list_all_catalog_categories(conn: &Connection) -> Result<Vec<CatalogCategory>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, business_id, name, code, parent_id, item_scope, sort_order, notes, created_at, updated_at, archived_at
             FROM catalog_categories
             ORDER BY updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare all-category query", error))?;

    let rows = stmt
        .query_map([], category_from_row)
        .map_err(|error| to_command_error("failed to query all categories", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map all categories", error))
}

pub fn list_all_catalog_units(conn: &Connection) -> Result<Vec<CatalogUnit>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, business_id, name, code, symbol, allow_fractional, is_system, updated_at, archived_at
             FROM catalog_units
             ORDER BY updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare all-unit query", error))?;

    let rows = stmt
        .query_map([], unit_from_row)
        .map_err(|error| to_command_error("failed to query all units", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map all units", error))
}

pub fn list_all_catalog_items(conn: &Connection) -> Result<Vec<CatalogItem>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                id, business_id, category_id, unit_id, tax_profile_id, item_kind,
                name, display_name, sku, primary_barcode, description,
                selling_price, cost_price, track_stock, stock_quantity,
                reorder_level, image_path, is_active, created_at, updated_at, archived_at
             FROM catalog_items
             ORDER BY updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare all-item query", error))?;

    let rows = stmt
        .query_map([], item_from_row)
        .map_err(|error| to_command_error("failed to query all items", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map all items", error))
}

pub fn list_all_catalog_barcodes(conn: &Connection) -> Result<Vec<CatalogBarcode>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, item_id, barcode, label, is_primary, created_at
             FROM catalog_item_barcodes
             ORDER BY created_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare all-barcode query", error))?;

    let rows = stmt
        .query_map([], barcode_from_row)
        .map_err(|error| to_command_error("failed to query all barcodes", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map all barcodes", error))
}
```

## `files/src-tauri/src/core/db.rs`

```rust
use std::fs;

use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension, Row};
use tauri::AppHandle;
use uuid::Uuid;

use crate::domain::models::{
    AppInfo, BackupRecord, BusinessProfile, BusinessSettings, BusinessWorkspaceSummary,
    ExportJobRecord, ModuleFlags, NewBusinessWorkspaceInput, PatchRecord, ReceiptProfile,
    RecentActivity, SequenceCounter, StorageStatus, TaxProfile, WorkspaceConfigurationInput,
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
    let conn = open_connection(&paths)
        .map_err(|error| to_command_error("failed to open local database", error))?;
    migrations::run(&conn).map_err(|error| to_command_error("failed to run migrations", error))?;
    patching::register_patch(&conn)
        .map_err(|error| to_command_error("failed to register patch history", error))?;
    seed::seed_if_empty(&conn).map_err(|error| to_command_error("failed to seed local data", error))?;
    Ok(())
}

pub fn with_connection<T, F>(app: &AppHandle, action: F) -> Result<T, String>
where
    F: FnOnce(&Connection, &AppPaths) -> Result<T, String>,
{
    let paths = resolve_paths(app)?;
    ensure_directories(&paths)?;
    let conn = open_connection(&paths)
        .map_err(|error| to_command_error("failed to open local database", error))?;
    action(&conn, &paths)
}

pub fn open_connection(paths: &AppPaths) -> rusqlite::Result<Connection> {
    let conn = Connection::open(&paths.database_path)?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.pragma_update(None, "journal_mode", "DELETE")?;
    conn.pragma_update(None, "synchronous", "NORMAL")?;
    Ok(conn)
}

pub fn now_iso() -> String {
    Utc::now().to_rfc3339()
}

fn bool_from_row(row: &Row, index: usize) -> rusqlite::Result<bool> {
    let value: i64 = row.get(index)?;
    Ok(value != 0)
}

fn bool_to_i64(value: bool) -> i64 {
    if value {
        1
    } else {
        0
    }
}

fn normalize_optional(value: &Option<String>) -> Option<String> {
    value.as_ref().and_then(|raw| {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

fn normalize_text(value: &str, field: &str) -> Result<String, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(format!("{field} cannot be empty"));
    }
    Ok(trimmed.to_string())
}

fn normalize_code(value: &str, field: &str) -> Result<String, String> {
    let normalized = normalize_text(value, field)?;
    Ok(normalized.to_uppercase())
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

fn tax_profile_from_row(row: &Row) -> rusqlite::Result<TaxProfile> {
    Ok(TaxProfile {
        id: row.get(0)?,
        business_id: row.get(1)?,
        name: row.get(2)?,
        tax_label: row.get(3)?,
        default_rate: row.get(4)?,
        prices_include_tax: bool_from_row(row, 5)?,
        is_default: bool_from_row(row, 6)?,
        updated_at: row.get(7)?,
    })
}

fn receipt_profile_from_row(row: &Row) -> rusqlite::Result<ReceiptProfile> {
    Ok(ReceiptProfile {
        id: row.get(0)?,
        business_id: row.get(1)?,
        name: row.get(2)?,
        footer_text: row.get(3)?,
        show_address: bool_from_row(row, 4)?,
        show_phone: bool_from_row(row, 5)?,
        show_email: bool_from_row(row, 6)?,
        show_business_code: bool_from_row(row, 7)?,
        paper_width: row.get(8)?,
        is_default: bool_from_row(row, 9)?,
        updated_at: row.get(10)?,
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

fn sequence_from_row(row: &Row) -> rusqlite::Result<SequenceCounter> {
    let business_id: String = row.get(0)?;
    let scope: String = row.get(1)?;
    Ok(SequenceCounter {
        id: format!("{business_id}:{scope}"),
        business_id,
        scope,
        prefix: row.get(2)?,
        next_number: row.get(3)?,
        padding: row.get(4)?,
        reset_policy: row.get(5)?,
        updated_at: row.get(6)?,
    })
}

pub fn get_meta(conn: &Connection, key: &str) -> Result<Option<String>, String> {
    conn.query_row(
        "SELECT value FROM app_meta WHERE key = ?1 LIMIT 1",
        params![key],
        |row| row.get::<_, String>(0),
    )
    .optional()
    .map_err(|error| to_command_error("failed to read app meta", error))
}

pub fn set_meta(conn: &Connection, key: &str, value: &str) -> Result<(), String> {
    conn.execute(
        "INSERT INTO app_meta (key, value, updated_at)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
        params![key, value, now_iso()],
    )
    .map_err(|error| to_command_error("failed to write app meta", error))?;
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

pub fn load_app_info(conn: &Connection) -> Result<AppInfo, String> {
    let app_name = get_meta(conn, "app_name")?.unwrap_or_else(|| "local-first-business-manager".into());
    let product_name = get_meta(conn, "product_name")?.unwrap_or_else(|| "Local Business Manager".into());
    let version = get_meta(conn, "app_version")?.unwrap_or_else(|| "0.3.0".into());
    let initialized_at = get_meta(conn, "initialized_at")?.unwrap_or_else(now_iso);
    let patch_level = get_meta(conn, "patch_level")?.unwrap_or_else(|| patching::PATCH_ID.to_string());
    let schema_version = get_meta(conn, "schema_version")?
        .and_then(|value| value.parse::<i64>().ok())
        .unwrap_or(CURRENT_SCHEMA_VERSION);

    Ok(AppInfo {
        app_name,
        product_name,
        version,
        schema_version,
        patch_level,
        initialized_at,
    })
}

pub fn list_patch_history(conn: &Connection) -> Result<Vec<PatchRecord>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT patch_id, patch_name, schema_version, applied_at
             FROM patch_history
             ORDER BY schema_version ASC, applied_at ASC",
        )
        .map_err(|error| to_command_error("failed to prepare patch history query", error))?;

    let rows = stmt
        .query_map([], patch_from_row)
        .map_err(|error| to_command_error("failed to query patch history", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map patch history", error))
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

pub fn get_business_by_id(conn: &Connection, business_id: &str) -> Result<BusinessProfile, String> {
    conn.query_row(
        "SELECT
            id, name, legal_name, code, business_type, currency_code, tax_mode,
            phone, email, address_line1, address_line2, city, state, postal_code,
            country, created_at, updated_at, archived_at
         FROM businesses
         WHERE id = ?1
         LIMIT 1",
        params![business_id],
        business_from_row,
    )
    .map_err(|error| to_command_error("failed to load business profile", error))
}

pub fn list_businesses(conn: &Connection) -> Result<Vec<BusinessProfile>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                id, name, legal_name, code, business_type, currency_code, tax_mode,
                phone, email, address_line1, address_line2, city, state, postal_code,
                country, created_at, updated_at, archived_at
             FROM businesses
             ORDER BY CASE WHEN archived_at IS NULL THEN 0 ELSE 1 END, updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare business list query", error))?;

    let rows = stmt
        .query_map([], business_from_row)
        .map_err(|error| to_command_error("failed to query businesses", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map businesses", error))
}

pub fn get_active_business(conn: &Connection) -> Result<BusinessProfile, String> {
    if let Some(active_business_id) = get_meta(conn, "active_business_id")? {
        if let Ok(business) = get_business_by_id(conn, &active_business_id) {
            if business.archived_at.is_none() {
                return Ok(business);
            }
        }
    }

    let businesses = list_businesses(conn)?;
    if let Some(business) = businesses
        .iter()
        .find(|item| item.archived_at.is_none())
        .cloned()
        .or_else(|| businesses.first().cloned())
    {
        set_meta(conn, "active_business_id", &business.id)?;
        return Ok(business);
    }

    Err("no business profile is available".into())
}

fn default_module_booleans(business_type: &str) -> (bool, bool, bool, bool) {
    let normalized = business_type.to_lowercase();
    let restaurant = normalized.contains("restaurant")
        || normalized.contains("cafe")
        || normalized.contains("bakery")
        || normalized.contains("food");
    let services = normalized.contains("service");
    let retail = !services;
    (restaurant, retail, true, services)
}

pub fn create_business_workspace(
    conn: &Connection,
    input: &NewBusinessWorkspaceInput,
) -> Result<BusinessProfile, String> {
    let business_id = Uuid::new_v4().to_string();
    let now = now_iso();
    let name = normalize_text(&input.name, "business name")?;
    let code = normalize_code(&input.code, "business code")?;
    let business_type = normalize_text(&input.business_type, "business type")?;
    let currency_code = normalize_code(&input.currency_code, "currency code")?;
    let tax_mode = normalize_text(&input.tax_mode, "tax mode")?;
    let timezone = normalize_text(&input.timezone, "timezone")?;
    let locale = normalize_text(&input.locale, "locale")?;
    let (restaurant_enabled, retail_enabled, inventory_enabled, services_enabled) =
        default_module_booleans(&business_type);

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
            &name,
            normalize_optional(&input.legal_name),
            &code,
            &business_type,
            &currency_code,
            &tax_mode,
            Option::<String>::None,
            Option::<String>::None,
            Option::<String>::None,
            Option::<String>::None,
            Option::<String>::None,
            Option::<String>::None,
            Option::<String>::None,
            Option::<String>::None,
            &now,
            &now,
            Option::<String>::None
        ],
    )
    .map_err(|error| to_command_error("failed to create business profile", error))?;

    let settings = BusinessSettings {
        business_id: business_id.clone(),
        timezone,
        locale,
        date_format: "DD-MM-YYYY".into(),
        theme: "system".into(),
        tax_label: "GST".into(),
        default_tax_rate: 0.0,
        prices_include_tax: false,
        receipt_footer: Some("Thank you for supporting local business.".into()),
        receipt_show_address: true,
        receipt_show_phone: true,
        auto_backup_enabled: false,
        backup_directory: None,
        module_restaurant_enabled: restaurant_enabled,
        module_retail_enabled: retail_enabled,
        module_inventory_enabled: inventory_enabled,
        module_services_enabled: services_enabled,
        updated_at: now.clone(),
    };
    save_business_settings(conn, &settings)?;
    seed::ensure_workspace_support_for_business(conn, &business_id)
        .map_err(|error| to_command_error("failed to seed workspace support", error))?;

    if input.activate_now {
        set_meta(conn, "active_business_id", &business_id)?;
    }

    insert_log(conn, "INFO", "workspace", "Business workspace created", None)?;
    get_business_by_id(conn, &business_id)
}

pub fn switch_active_business(conn: &Connection, business_id: &str) -> Result<BusinessProfile, String> {
    let business = get_business_by_id(conn, business_id)?;
    if business.archived_at.is_some() {
        return Err("cannot switch to an archived business".into());
    }
    set_meta(conn, "active_business_id", business_id)?;
    insert_log(conn, "INFO", "workspace", "Active business switched", None)?;
    Ok(business)
}

pub fn save_business_profile(conn: &Connection, profile: &BusinessProfile) -> Result<BusinessProfile, String> {
    let now = now_iso();
    let rows = conn
        .execute(
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
                normalize_text(&profile.name, "business name")?,
                normalize_optional(&profile.legal_name),
                normalize_code(&profile.code, "business code")?,
                normalize_text(&profile.business_type, "business type")?,
                normalize_code(&profile.currency_code, "currency code")?,
                normalize_text(&profile.tax_mode, "tax mode")?,
                normalize_optional(&profile.phone),
                normalize_optional(&profile.email),
                normalize_optional(&profile.address_line1),
                normalize_optional(&profile.address_line2),
                normalize_optional(&profile.city),
                normalize_optional(&profile.state),
                normalize_optional(&profile.postal_code),
                normalize_optional(&profile.country),
                &now,
            ],
        )
        .map_err(|error| to_command_error("failed to update business profile", error))?;

    if rows == 0 {
        return Err("business profile was not found".into());
    }

    insert_log(conn, "INFO", "business", "Business profile updated", None)?;
    get_business_by_id(conn, &profile.id)
}

pub fn get_business_settings(conn: &Connection, business_id: &str) -> Result<BusinessSettings, String> {
    conn.query_row(
        "SELECT
            business_id, timezone, locale, date_format, theme, tax_label,
            default_tax_rate, prices_include_tax, receipt_footer,
            receipt_show_address, receipt_show_phone, auto_backup_enabled,
            backup_directory, module_restaurant_enabled, module_retail_enabled,
            module_inventory_enabled, module_services_enabled, updated_at
         FROM business_settings
         WHERE business_id = ?1
         LIMIT 1",
        params![business_id],
        settings_from_row,
    )
    .map_err(|error| to_command_error("failed to load business settings", error))
}

pub fn list_all_business_settings(conn: &Connection) -> Result<Vec<BusinessSettings>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                business_id, timezone, locale, date_format, theme, tax_label,
                default_tax_rate, prices_include_tax, receipt_footer,
                receipt_show_address, receipt_show_phone, auto_backup_enabled,
                backup_directory, module_restaurant_enabled, module_retail_enabled,
                module_inventory_enabled, module_services_enabled, updated_at
             FROM business_settings
             ORDER BY updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare business settings query", error))?;

    let rows = stmt
        .query_map([], settings_from_row)
        .map_err(|error| to_command_error("failed to query business settings", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map business settings", error))
}

pub fn save_business_settings(conn: &Connection, settings: &BusinessSettings) -> Result<(), String> {
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
         )
         ON CONFLICT(business_id) DO UPDATE SET
            timezone = excluded.timezone,
            locale = excluded.locale,
            date_format = excluded.date_format,
            theme = excluded.theme,
            tax_label = excluded.tax_label,
            default_tax_rate = excluded.default_tax_rate,
            prices_include_tax = excluded.prices_include_tax,
            receipt_footer = excluded.receipt_footer,
            receipt_show_address = excluded.receipt_show_address,
            receipt_show_phone = excluded.receipt_show_phone,
            auto_backup_enabled = excluded.auto_backup_enabled,
            backup_directory = excluded.backup_directory,
            module_restaurant_enabled = excluded.module_restaurant_enabled,
            module_retail_enabled = excluded.module_retail_enabled,
            module_inventory_enabled = excluded.module_inventory_enabled,
            module_services_enabled = excluded.module_services_enabled,
            updated_at = excluded.updated_at",
        params![
            &settings.business_id,
            normalize_text(&settings.timezone, "timezone")?,
            normalize_text(&settings.locale, "locale")?,
            normalize_text(&settings.date_format, "date format")?,
            normalize_text(&settings.theme, "theme")?,
            normalize_text(&settings.tax_label, "tax label")?,
            settings.default_tax_rate.max(0.0),
            bool_to_i64(settings.prices_include_tax),
            normalize_optional(&settings.receipt_footer),
            bool_to_i64(settings.receipt_show_address),
            bool_to_i64(settings.receipt_show_phone),
            bool_to_i64(settings.auto_backup_enabled),
            normalize_optional(&settings.backup_directory),
            bool_to_i64(settings.module_restaurant_enabled),
            bool_to_i64(settings.module_retail_enabled),
            bool_to_i64(settings.module_inventory_enabled),
            bool_to_i64(settings.module_services_enabled),
            now_iso(),
        ],
    )
    .map_err(|error| to_command_error("failed to save business settings", error))?;
    Ok(())
}

pub fn list_tax_profiles(conn: &Connection, business_id: &str) -> Result<Vec<TaxProfile>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, business_id, name, label, rate, prices_include_tax, is_default, updated_at
             FROM tax_profiles
             WHERE business_id = ?1
             ORDER BY is_default DESC, updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare tax profile query", error))?;

    let rows = stmt
        .query_map(params![business_id], tax_profile_from_row)
        .map_err(|error| to_command_error("failed to query tax profiles", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map tax profiles", error))
}

pub fn get_default_tax_profile(conn: &Connection, business_id: &str) -> Result<TaxProfile, String> {
    if let Some(profile) = list_tax_profiles(conn, business_id)?.into_iter().next() {
        return Ok(profile);
    }

    seed::ensure_workspace_support_for_business(conn, business_id)
        .map_err(|error| to_command_error("failed to ensure tax profile", error))?;
    list_tax_profiles(conn, business_id)?
        .into_iter()
        .next()
        .ok_or_else(|| "default tax profile was not found".into())
}

pub fn list_all_tax_profiles(conn: &Connection) -> Result<Vec<TaxProfile>, String> {
    let businesses = list_businesses(conn)?;
    let mut profiles = Vec::new();
    for business in businesses {
        profiles.extend(list_tax_profiles(conn, &business.id)?);
    }
    Ok(profiles)
}

pub fn save_default_tax_profile(conn: &Connection, profile: &TaxProfile) -> Result<TaxProfile, String> {
    let name = normalize_text(&profile.name, "tax profile name")?;
    let label = normalize_text(&profile.tax_label, "tax label")?;
    let now = now_iso();

    conn.execute(
        "UPDATE tax_profiles SET is_default = 0, updated_at = ?2 WHERE business_id = ?1",
        params![&profile.business_id, &now],
    )
    .map_err(|error| to_command_error("failed to clear default tax profile", error))?;

    let existing_id = profile.id.trim();
    let target_id = if existing_id.is_empty() {
        Uuid::new_v4().to_string()
    } else {
        existing_id.to_string()
    };

    conn.execute(
        "INSERT INTO tax_profiles (
            id, business_id, name, label, rate, prices_include_tax, is_default, created_at, updated_at
         ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, 1, ?7, ?8
         )
         ON CONFLICT(id) DO UPDATE SET
            name = excluded.name,
            label = excluded.label,
            rate = excluded.rate,
            prices_include_tax = excluded.prices_include_tax,
            is_default = excluded.is_default,
            updated_at = excluded.updated_at",
        params![
            &target_id,
            &profile.business_id,
            &name,
            &label,
            profile.default_rate.max(0.0),
            bool_to_i64(profile.prices_include_tax),
            &now,
            &now,
        ],
    )
    .map_err(|error| to_command_error("failed to save tax profile", error))?;

    get_default_tax_profile(conn, &profile.business_id)
}

pub fn get_default_receipt_profile(conn: &Connection, business_id: &str) -> Result<ReceiptProfile, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, business_id, name, footer_text, show_address, show_phone, show_email, show_business_code, paper_width, is_default, updated_at
             FROM receipt_profiles
             WHERE business_id = ?1
             ORDER BY is_default DESC, updated_at DESC
             LIMIT 1",
        )
        .map_err(|error| to_command_error("failed to prepare receipt profile query", error))?;

    let profile = stmt
        .query_row(params![business_id], receipt_profile_from_row)
        .optional()
        .map_err(|error| to_command_error("failed to query receipt profile", error))?;

    if let Some(profile) = profile {
        return Ok(profile);
    }

    seed::ensure_workspace_support_for_business(conn, business_id)
        .map_err(|error| to_command_error("failed to ensure receipt profile", error))?;

    conn.query_row(
        "SELECT id, business_id, name, footer_text, show_address, show_phone, show_email, show_business_code, paper_width, is_default, updated_at
         FROM receipt_profiles
         WHERE business_id = ?1
         ORDER BY is_default DESC, updated_at DESC
         LIMIT 1",
        params![business_id],
        receipt_profile_from_row,
    )
    .map_err(|error| to_command_error("failed to load default receipt profile", error))
}

pub fn list_all_receipt_profiles(conn: &Connection) -> Result<Vec<ReceiptProfile>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, business_id, name, footer_text, show_address, show_phone, show_email, show_business_code, paper_width, is_default, updated_at
             FROM receipt_profiles
             ORDER BY updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare receipt profile list", error))?;

    let rows = stmt
        .query_map([], receipt_profile_from_row)
        .map_err(|error| to_command_error("failed to query receipt profiles", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map receipt profiles", error))
}

pub fn save_default_receipt_profile(
    conn: &Connection,
    profile: &ReceiptProfile,
) -> Result<ReceiptProfile, String> {
    let now = now_iso();
    let name = normalize_text(&profile.name, "receipt profile name")?;
    let paper_width = normalize_text(&profile.paper_width, "paper width")?;

    conn.execute(
        "UPDATE receipt_profiles SET is_default = 0, updated_at = ?2 WHERE business_id = ?1",
        params![&profile.business_id, &now],
    )
    .map_err(|error| to_command_error("failed to clear receipt defaults", error))?;

    let existing_id = profile.id.trim();
    let target_id = if existing_id.is_empty() {
        Uuid::new_v4().to_string()
    } else {
        existing_id.to_string()
    };

    conn.execute(
        "INSERT INTO receipt_profiles (
            id, business_id, name, header_line1, header_line2, footer_text,
            show_address, show_phone, show_tax_breakdown, paper_width,
            copies, is_default, created_at, updated_at, show_email, show_business_code
         ) VALUES (
            ?1, ?2, ?3, NULL, NULL, ?4,
            ?5, ?6, 1, ?7,
            1, 1, ?8, ?9, ?10, ?11
         )
         ON CONFLICT(id) DO UPDATE SET
            name = excluded.name,
            footer_text = excluded.footer_text,
            show_address = excluded.show_address,
            show_phone = excluded.show_phone,
            paper_width = excluded.paper_width,
            is_default = excluded.is_default,
            updated_at = excluded.updated_at,
            show_email = excluded.show_email,
            show_business_code = excluded.show_business_code",
        params![
            &target_id,
            &profile.business_id,
            &name,
            normalize_optional(&profile.footer_text),
            bool_to_i64(profile.show_address),
            bool_to_i64(profile.show_phone),
            &paper_width,
            &now,
            &now,
            bool_to_i64(profile.show_email),
            bool_to_i64(profile.show_business_code),
        ],
    )
    .map_err(|error| to_command_error("failed to save receipt profile", error))?;

    get_default_receipt_profile(conn, &profile.business_id)
}

fn set_flag(flags: &mut ModuleFlags, key: &str, enabled: bool) {
    match key {
        "restaurant" => flags.restaurant_enabled = enabled,
        "retail" => flags.retail_enabled = enabled,
        "inventory" => flags.inventory_enabled = enabled,
        "services" => flags.services_enabled = enabled,
        "customers" => flags.customers_enabled = enabled,
        "suppliers" => flags.suppliers_enabled = enabled,
        "expenses" => flags.expenses_enabled = enabled,
        "reporting" => flags.reporting_enabled = enabled,
        "data_center" => flags.data_center_enabled = enabled,
        _ => {}
    }
}

fn active_module_keys(flags: &ModuleFlags) -> Vec<String> {
    let mut modules = Vec::new();
    if flags.restaurant_enabled {
        modules.push("restaurant".to_string());
    }
    if flags.retail_enabled {
        modules.push("retail".to_string());
    }
    if flags.inventory_enabled {
        modules.push("inventory".to_string());
    }
    if flags.services_enabled {
        modules.push("services".to_string());
    }
    if flags.customers_enabled {
        modules.push("customers".to_string());
    }
    if flags.suppliers_enabled {
        modules.push("suppliers".to_string());
    }
    if flags.expenses_enabled {
        modules.push("expenses".to_string());
    }
    if flags.reporting_enabled {
        modules.push("reporting".to_string());
    }
    if flags.data_center_enabled {
        modules.push("data_center".to_string());
    }
    modules
}

pub fn get_module_flags(conn: &Connection, business_id: &str) -> Result<ModuleFlags, String> {
    let settings = get_business_settings(conn, business_id)?;
    let mut flags = ModuleFlags {
        business_id: business_id.to_string(),
        restaurant_enabled: settings.module_restaurant_enabled,
        retail_enabled: settings.module_retail_enabled,
        inventory_enabled: settings.module_inventory_enabled,
        services_enabled: settings.module_services_enabled,
        customers_enabled: false,
        suppliers_enabled: false,
        expenses_enabled: false,
        reporting_enabled: false,
        data_center_enabled: true,
        updated_at: settings.updated_at.clone(),
    };

    let mut stmt = conn
        .prepare(
            "SELECT module_key, enabled, updated_at
             FROM module_flags
             WHERE business_id = ?1
             ORDER BY module_key ASC",
        )
        .map_err(|error| to_command_error("failed to prepare module flag query", error))?;

    let mut rows = stmt
        .query(params![business_id])
        .map_err(|error| to_command_error("failed to query module flags", error))?;

    while let Some(row) = rows
        .next()
        .map_err(|error| to_command_error("failed to iterate module flags", error))?
    {
        let module_key: String = row
            .get(0)
            .map_err(|error| to_command_error("failed to read module key", error))?;
        let enabled = row
            .get::<_, i64>(1)
            .map_err(|error| to_command_error("failed to read module enabled value", error))?
            != 0;
        let updated_at: String = row
            .get(2)
            .map_err(|error| to_command_error("failed to read module updated timestamp", error))?;
        set_flag(&mut flags, &module_key, enabled);
        flags.updated_at = updated_at;
    }

    Ok(flags)
}

pub fn list_all_module_flags(conn: &Connection) -> Result<Vec<ModuleFlags>, String> {
    let businesses = list_businesses(conn)?;
    let mut output = Vec::new();
    for business in businesses {
        output.push(get_module_flags(conn, &business.id)?);
    }
    Ok(output)
}

pub fn save_module_flags(conn: &Connection, flags: &ModuleFlags) -> Result<(), String> {
    let now = now_iso();
    for (module_key, enabled) in [
        ("restaurant", flags.restaurant_enabled),
        ("retail", flags.retail_enabled),
        ("inventory", flags.inventory_enabled),
        ("services", flags.services_enabled),
        ("customers", flags.customers_enabled),
        ("suppliers", flags.suppliers_enabled),
        ("expenses", flags.expenses_enabled),
        ("reporting", flags.reporting_enabled),
        ("data_center", flags.data_center_enabled),
    ] {
        conn.execute(
            "INSERT INTO module_flags (business_id, module_key, enabled, updated_at)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(business_id, module_key)
             DO UPDATE SET enabled = excluded.enabled, updated_at = excluded.updated_at",
            params![&flags.business_id, module_key, bool_to_i64(enabled), &now],
        )
        .map_err(|error| to_command_error("failed to save module flags", error))?;
    }
    Ok(())
}

pub fn list_sequence_counters(conn: &Connection, business_id: &str) -> Result<Vec<SequenceCounter>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT business_id, counter_key, prefix, next_number, padding, reset_policy, updated_at
             FROM sequence_counters
             WHERE business_id = ?1
             ORDER BY counter_key ASC",
        )
        .map_err(|error| to_command_error("failed to prepare sequence query", error))?;

    let rows = stmt
        .query_map(params![business_id], sequence_from_row)
        .map_err(|error| to_command_error("failed to query sequence counters", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map sequence counters", error))
}

pub fn list_all_sequence_counters(conn: &Connection) -> Result<Vec<SequenceCounter>, String> {
    let businesses = list_businesses(conn)?;
    let mut output = Vec::new();
    for business in businesses {
        output.extend(list_sequence_counters(conn, &business.id)?);
    }
    Ok(output)
}

pub fn save_sequence_counters(conn: &Connection, counters: &[SequenceCounter]) -> Result<(), String> {
    let now = now_iso();
    for counter in counters {
        conn.execute(
            "INSERT INTO sequence_counters (business_id, counter_key, prefix, next_number, padding, reset_policy, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(business_id, counter_key)
             DO UPDATE SET
                prefix = excluded.prefix,
                next_number = excluded.next_number,
                padding = excluded.padding,
                reset_policy = excluded.reset_policy,
                updated_at = excluded.updated_at",
            params![
                &counter.business_id,
                normalize_text(&counter.scope, "sequence scope")?,
                normalize_text(&counter.prefix, "sequence prefix")?.to_uppercase(),
                counter.next_number.max(1),
                counter.padding.max(1),
                normalize_text(&counter.reset_policy, "reset policy")?,
                &now,
            ],
        )
        .map_err(|error| to_command_error("failed to save sequence counters", error))?;
    }
    Ok(())
}

pub fn save_workspace_configuration(
    conn: &Connection,
    input: &WorkspaceConfigurationInput,
) -> Result<(), String> {
    let business_id = input.business_settings.business_id.clone();
    let mut settings = input.business_settings.clone();
    settings.tax_label = input.tax_profile.tax_label.clone();
    settings.default_tax_rate = input.tax_profile.default_rate;
    settings.prices_include_tax = input.tax_profile.prices_include_tax;
    settings.receipt_footer = input.receipt_profile.footer_text.clone();
    settings.receipt_show_address = input.receipt_profile.show_address;
    settings.receipt_show_phone = input.receipt_profile.show_phone;
    settings.module_restaurant_enabled = input.module_flags.restaurant_enabled;
    settings.module_retail_enabled = input.module_flags.retail_enabled;
    settings.module_inventory_enabled = input.module_flags.inventory_enabled;
    settings.module_services_enabled = input.module_flags.services_enabled;
    settings.updated_at = now_iso();

    save_business_settings(conn, &settings)?;

    let tax_profile = TaxProfile {
        business_id: business_id.clone(),
        ..input.tax_profile.clone()
    };
    save_default_tax_profile(conn, &tax_profile)?;

    let receipt_profile = ReceiptProfile {
        business_id: business_id.clone(),
        ..input.receipt_profile.clone()
    };
    save_default_receipt_profile(conn, &receipt_profile)?;

    let module_flags = ModuleFlags {
        business_id: business_id.clone(),
        updated_at: now_iso(),
        ..input.module_flags.clone()
    };
    save_module_flags(conn, &module_flags)?;

    let counters = input
        .sequence_counters
        .iter()
        .cloned()
        .map(|mut counter| {
            counter.business_id = business_id.clone();
            counter
        })
        .collect::<Vec<_>>();
    save_sequence_counters(conn, &counters)?;

    insert_log(conn, "INFO", "settings", "Workspace configuration saved", None)?;
    Ok(())
}

fn format_sequence_preview(prefix: &str, next_number: i64, padding: i64) -> String {
    let normalized_padding = padding.max(1) as usize;
    let normalized_number = next_number.max(1);
    format!("{prefix}{:0width$}", normalized_number, width = normalized_padding)
}

pub fn list_business_workspace_summaries(
    conn: &Connection,
) -> Result<Vec<BusinessWorkspaceSummary>, String> {
    let businesses = list_businesses(conn)?;
    let mut output = Vec::new();

    for business in businesses {
        let settings = get_business_settings(conn, &business.id)?;
        let tax_profile = get_default_tax_profile(conn, &business.id)?;
        let module_flags = get_module_flags(conn, &business.id)?;
        let sequences = list_sequence_counters(conn, &business.id)?;
        let sale_sequence = sequences
            .iter()
            .find(|counter| counter.scope == "sale")
            .cloned();
        let next_sale_sequence = sale_sequence
            .map(|sequence| {
                format_sequence_preview(&sequence.prefix, sequence.next_number, sequence.padding)
            })
            .unwrap_or_else(|| "SAL-00001".into());

        output.push(BusinessWorkspaceSummary {
            business_id: business.id.clone(),
            name: business.name.clone(),
            code: business.code.clone(),
            business_type: business.business_type.clone(),
            currency_code: business.currency_code.clone(),
            theme: settings.theme.clone(),
            timezone: settings.timezone.clone(),
            tax_label: tax_profile.tax_label.clone(),
            default_tax_rate: tax_profile.default_rate,
            next_sale_sequence,
            active_modules: active_module_keys(&module_flags),
            archived_at: business.archived_at.clone(),
            updated_at: business.updated_at.clone(),
        });
    }

    Ok(output)
}

pub fn build_storage_status(conn: &Connection, paths: &AppPaths) -> Result<StorageStatus, String> {
    let backup_count: usize = conn
        .query_row("SELECT COUNT(*) FROM backup_records", [], |row| row.get::<_, i64>(0))
        .map_err(|error| to_command_error("failed to count backup records", error))?
        as usize;
    let export_count: usize = conn
        .query_row("SELECT COUNT(*) FROM export_jobs", [], |row| row.get::<_, i64>(0))
        .map_err(|error| to_command_error("failed to count export jobs", error))?
        as usize;

    Ok(StorageStatus {
        data_dir: paths.data_dir.clone(),
        config_dir: paths.config_dir.clone(),
        log_dir: paths.log_dir.clone(),
        backup_dir: paths.backup_dir.clone(),
        export_dir: paths.export_dir.clone(),
        database_path: paths.database_path.clone(),
        database_exists: fs::metadata(&paths.database_path).is_ok(),
        backup_count,
        export_count,
    })
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
            &record.created_at,
        ],
    )
    .map_err(|error| to_command_error("failed to insert backup record", error))?;
    Ok(())
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
            &record.completed_at,
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
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, NULL)",
        params![
            Uuid::new_v4().to_string(),
            business_id,
            format,
            status,
            source_path,
            now_iso(),
        ],
    )
    .map_err(|error| to_command_error("failed to insert import job", error))?;
    Ok(())
}
```

## `files/src-tauri/src/core/migrations/003_catalog_core.sql`

```sql
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
```

## `files/src-tauri/src/core/migrations.rs`

```rust
use rusqlite::{Connection, OptionalExtension};

pub const CURRENT_SCHEMA_VERSION: i64 = 3;

const MIGRATION_001: &str = include_str!("migrations/001_base.sql");
const MIGRATION_002: &str = include_str!("migrations/002_multi_business_workspace.sql");
const MIGRATION_003: &str = include_str!("migrations/003_catalog_core.sql");

fn table_exists(conn: &Connection, table_name: &str) -> rusqlite::Result<bool> {
    let exists: Option<String> = conn
        .query_row(
            "SELECT name FROM sqlite_master WHERE type = 'table' AND name = ?1 LIMIT 1",
            [table_name],
            |row| row.get(0),
        )
        .optional()?;
    Ok(exists.is_some())
}

fn detect_current_schema_version(conn: &Connection) -> rusqlite::Result<i64> {
    let user_version: i64 = conn.query_row("PRAGMA user_version", [], |row| row.get(0))?;
    if user_version > 0 {
        return Ok(user_version);
    }

    if table_exists(conn, "catalog_items")? {
        return Ok(3);
    }

    if table_exists(conn, "tax_profiles")? {
        return Ok(2);
    }

    if table_exists(conn, "app_meta")? {
        let legacy_version = conn
            .query_row(
                "SELECT value FROM app_meta WHERE key = 'schema_version' LIMIT 1",
                [],
                |row| row.get::<_, String>(0),
            )
            .optional()?;

        if let Some(value) = legacy_version {
            if let Ok(parsed) = value.parse::<i64>() {
                return Ok(parsed);
            }
        }

        return Ok(1);
    }

    if table_exists(conn, "businesses")? {
        return Ok(1);
    }

    Ok(0)
}

pub fn run(conn: &Connection) -> rusqlite::Result<()> {
    let mut current_version = detect_current_schema_version(conn)?;

    if current_version < 1 {
        conn.execute_batch(MIGRATION_001)?;
        current_version = 1;
    }

    if current_version < 2 {
        conn.execute_batch(MIGRATION_002)?;
        current_version = 2;
    }

    if current_version < 3 {
        conn.execute_batch(MIGRATION_003)?;
        current_version = 3;
    }

    conn.pragma_update(None, "user_version", current_version)?;
    Ok(())
}
```

## `files/src-tauri/src/core/mod.rs`

```rust
pub mod catalog;
pub mod db;
pub mod error;
pub mod migrations;
pub mod patching;
pub mod paths;
pub mod seed;
```

## `files/src-tauri/src/core/patching.rs`

```rust
use chrono::Utc;
use rusqlite::{params, Connection};
use serde_json::json;

use super::migrations::CURRENT_SCHEMA_VERSION;

struct PatchDefinition {
    patch_id: &'static str,
    patch_name: &'static str,
    schema_version: i64,
    notes: &'static str,
}

const PATCHES: [PatchDefinition; 3] = [
    PatchDefinition {
        patch_id: "P001_foundation_base_structure",
        patch_name: "Foundation Base Structure",
        schema_version: 1,
        notes: "Initial desktop foundation with local storage, business/settings shell, backup/export foundation.",
    },
    PatchDefinition {
        patch_id: "P002_multi_business_workspace_settings_core",
        patch_name: "Multi-Business Workspace & Settings Core",
        schema_version: 2,
        notes: "Adds multi-business switching, business-scoped settings profiles, normalized module flags, receipt profiles, tax profiles, and sequence counters.",
    },
    PatchDefinition {
        patch_id: "P003_catalog_core",
        patch_name: "Catalog Core",
        schema_version: 3,
        notes: "Adds categories, units, item master data, barcode foundations, and catalog-aware export scope for local businesses.",
    },
];

pub const PATCH_ID: &str = "P003_catalog_core";
pub const PATCH_NAME: &str = "Catalog Core";

pub fn register_patch(conn: &Connection) -> rusqlite::Result<()> {
    for patch in PATCHES {
        let now = Utc::now().to_rfc3339();
        let manifest = json!({
            "patch_id": patch.patch_id,
            "patch_name": patch.patch_name,
            "schema_version": patch.schema_version,
            "applied_at": now.clone(),
            "notes": patch.notes,
        });

        conn.execute(
            "INSERT OR IGNORE INTO patch_history (patch_id, patch_name, schema_version, applied_at, manifest_json)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                patch.patch_id,
                patch.patch_name,
                patch.schema_version,
                now,
                manifest.to_string()
            ],
        )?;
    }

    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO app_meta (key, value, updated_at)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
        params!["schema_version", CURRENT_SCHEMA_VERSION.to_string(), now.clone()],
    )?;

    conn.execute(
        "INSERT INTO app_meta (key, value, updated_at)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
        params!["patch_level", PATCH_ID, now],
    )?;

    Ok(())
}
```

## `files/src-tauri/src/core/seed.rs`

```rust
use chrono::Utc;
use rusqlite::{params, Connection};
use uuid::Uuid;

use super::catalog;

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

fn bool_to_i64(value: bool) -> i64 {
    if value {
        1
    } else {
        0
    }
}

pub fn ensure_workspace_support_for_business(
    conn: &Connection,
    business_id: &str,
) -> rusqlite::Result<()> {
    let (
        tax_label,
        default_tax_rate,
        prices_include_tax,
        receipt_footer,
        receipt_show_address,
        receipt_show_phone,
        module_restaurant_enabled,
        module_retail_enabled,
        module_inventory_enabled,
        module_services_enabled,
    ) = conn.query_row(
        "SELECT
            tax_label,
            default_tax_rate,
            prices_include_tax,
            receipt_footer,
            receipt_show_address,
            receipt_show_phone,
            module_restaurant_enabled,
            module_retail_enabled,
            module_inventory_enabled,
            module_services_enabled
         FROM business_settings
         WHERE business_id = ?1
         LIMIT 1",
        params![business_id],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, f64>(1)?,
                row.get::<_, i64>(2)? != 0,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, i64>(4)? != 0,
                row.get::<_, i64>(5)? != 0,
                row.get::<_, i64>(6)? != 0,
                row.get::<_, i64>(7)? != 0,
                row.get::<_, i64>(8)? != 0,
                row.get::<_, i64>(9)? != 0,
            ))
        },
    )?;

    let now = Utc::now().to_rfc3339();

    let tax_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM tax_profiles WHERE business_id = ?1",
        params![business_id],
        |row| row.get(0),
    )?;
    if tax_count == 0 {
        conn.execute(
            "INSERT INTO tax_profiles (
                id, business_id, name, label, rate, prices_include_tax,
                is_default, created_at, updated_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 1, ?7, ?8)",
            params![
                Uuid::new_v4().to_string(),
                business_id,
                "Standard tax",
                tax_label,
                default_tax_rate,
                bool_to_i64(prices_include_tax),
                &now,
                &now,
            ],
        )?;
    }

    let receipt_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM receipt_profiles WHERE business_id = ?1",
        params![business_id],
        |row| row.get(0),
    )?;
    if receipt_count == 0 {
        conn.execute(
            "INSERT INTO receipt_profiles (
                id, business_id, name, header_line1, header_line2, footer_text,
                show_address, show_phone, show_tax_breakdown, paper_width, copies,
                is_default, created_at, updated_at, show_email, show_business_code
             ) VALUES (
                ?1, ?2, ?3, NULL, NULL, ?4,
                ?5, ?6, 1, '80mm', 1,
                1, ?7, ?8, 0, 1
             )",
            params![
                Uuid::new_v4().to_string(),
                business_id,
                "Default receipt",
                receipt_footer,
                bool_to_i64(receipt_show_address),
                bool_to_i64(receipt_show_phone),
                &now,
                &now,
            ],
        )?;
    }

    for (module_key, enabled) in [
        ("restaurant", module_restaurant_enabled),
        ("retail", module_retail_enabled),
        ("inventory", module_inventory_enabled),
        ("services", module_services_enabled),
        ("customers", false),
        ("suppliers", false),
        ("expenses", false),
        ("reporting", false),
        ("data_center", true),
    ] {
        conn.execute(
            "INSERT OR IGNORE INTO module_flags (business_id, module_key, enabled, updated_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![business_id, module_key, bool_to_i64(enabled), &now],
        )?;
    }

    for (counter_key, prefix, padding, reset_policy) in [
        ("sale", "SAL-", 5_i64, "none"),
        ("purchase", "PUR-", 5_i64, "none"),
        ("expense", "EXP-", 5_i64, "none"),
        ("customer", "CUS-", 4_i64, "none"),
        ("supplier", "SUP-", 4_i64, "none"),
    ] {
        conn.execute(
            "INSERT OR IGNORE INTO sequence_counters (
                business_id, counter_key, prefix, next_number, padding, reset_policy, updated_at
             ) VALUES (?1, ?2, ?3, 1, ?4, ?5, ?6)",
            params![business_id, counter_key, prefix, padding, reset_policy, &now],
        )?;
    }

    Ok(())
}

fn ensure_workspace_support_foundation(conn: &Connection) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare("SELECT id FROM businesses ORDER BY created_at ASC")?;
    let business_ids = stmt.query_map([], |row| row.get::<_, String>(0))?;

    for business_id in business_ids {
        ensure_workspace_support_for_business(conn, &business_id?)?;
    }

    Ok(())
}

fn seed_demo_catalog_foundation(conn: &Connection) -> rusqlite::Result<()> {
    let seeded_demo = conn.query_row(
        "SELECT value FROM app_meta WHERE key = 'seeded_demo_data' LIMIT 1",
        [],
        |row| row.get::<_, String>(0),
    ).unwrap_or_else(|_| "false".to_string());

    if seeded_demo != "true" {
        return Ok(());
    }

    let mut stmt = conn.prepare(
        "SELECT id
         FROM businesses
         WHERE code = 'DEMO-001' OR name LIKE 'Demo %'
         ORDER BY created_at ASC",
    )?;
    let business_ids = stmt.query_map([], |row| row.get::<_, String>(0))?;
    for business_id in business_ids {
        catalog::seed_demo_catalog_for_business(conn, &business_id?)?;
    }

    Ok(())
}

pub fn seed_if_empty(conn: &Connection) -> rusqlite::Result<()> {
    let business_count: i64 = conn.query_row("SELECT COUNT(*) FROM businesses", [], |row| row.get(0))?;

    upsert_meta(conn, "app_name", "local-first-business-manager")?;
    upsert_meta(conn, "product_name", "Local Business Manager")?;
    upsert_meta(conn, "app_version", "0.3.0")?;
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
                Option::<String>::None,
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
                &now,
            ],
        )?;

        upsert_meta(conn, "active_business_id", &business_id)?;
        upsert_meta(conn, "seeded_demo_data", "true")?;

        for (level, category, message) in [
            ("INFO", "patching", "Patch 1 foundation registered"),
            ("INFO", "patching", "Patch 2 multi-business workspace registered"),
            ("INFO", "patching", "Patch 3 catalog core registered"),
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
                    Utc::now().to_rfc3339(),
                ],
            )?;
        }
    }

    ensure_workspace_support_foundation(conn)?;
    catalog::ensure_system_units(conn)?;
    seed_demo_catalog_foundation(conn)?;
    Ok(())
}
```

## `files/src-tauri/src/domain/bootstrap.rs`

```rust
use super::models::{
    AppBootstrap, AppInfo, BackupRecord, BusinessProfile, BusinessSettings,
    BusinessWorkspaceSummary, CatalogSummary, DashboardShellData, KpiCard, ModuleFlags,
    ModuleStatus, PatchRecord, ReceiptProfile, RecentActivity, SequenceCounter, StorageStatus,
    TaxProfile,
};

fn active_module_count(module_flags: &ModuleFlags) -> usize {
    [
        module_flags.restaurant_enabled,
        module_flags.retail_enabled,
        module_flags.inventory_enabled,
        module_flags.services_enabled,
        module_flags.customers_enabled,
        module_flags.suppliers_enabled,
        module_flags.expenses_enabled,
        module_flags.reporting_enabled,
        module_flags.data_center_enabled,
    ]
    .into_iter()
    .filter(|enabled| *enabled)
    .count()
}

pub fn compose_dashboard(
    business_workspaces: &[BusinessWorkspaceSummary],
    active_business: &BusinessProfile,
    active_modules: &ModuleFlags,
    active_sequences: &[SequenceCounter],
    backups: &[BackupRecord],
    recent_activity: Vec<RecentActivity>,
    patch_history: &[PatchRecord],
    storage: &StorageStatus,
    catalog_summary: &CatalogSummary,
) -> DashboardShellData {
    DashboardShellData {
        hero_title: "Catalog-ready local workspace".into(),
        hero_body: format!(
            "Patch 3 adds catalog structure for products, menu items, and services while keeping the app fully local-first. {} is active, with categories, units, and item master data now scoped safely to the active business.",
            active_business.name
        ),
        kpis: vec![
            KpiCard {
                id: "workspace-count".into(),
                label: "Business workspaces".into(),
                value: business_workspaces.len().to_string(),
                note: "Each workspace keeps its own profile, settings, and future sales or inventory data boundaries.".into(),
            },
            KpiCard {
                id: "catalog-items".into(),
                label: "Catalog items".into(),
                value: catalog_summary.active_items.to_string(),
                note: format!(
                    "{} categories and {} low-stock candidates are visible before the deeper inventory ledger patch.",
                    catalog_summary.category_count, catalog_summary.low_stock_candidates
                ),
            },
            KpiCard {
                id: "module-count".into(),
                label: "Active modules".into(),
                value: active_module_count(active_modules).to_string(),
                note: "Module flags remain business-scoped for future POS, inventory, and service workflows.".into(),
            },
            KpiCard {
                id: "sequence-count".into(),
                label: "Sequence counters".into(),
                value: active_sequences.len().to_string(),
                note: "Document numbering is ready before POS billing and purchasing flows arrive.".into(),
            },
            KpiCard {
                id: "backup-count".into(),
                label: "Backup snapshots".into(),
                value: backups.len().to_string(),
                note: format!("{} export job(s) are tracked in local storage.", storage.export_count),
            },
            KpiCard {
                id: "patch-count".into(),
                label: "Applied patches".into(),
                value: patch_history.len().to_string(),
                note: "Patch history is stored locally to keep upgrades traceable and recovery-friendly.".into(),
            },
        ],
        recent_activity,
        module_statuses: vec![
            ModuleStatus {
                id: "foundation".into(),
                label: "Foundation shell".into(),
                status: "active-foundation".into(),
                note: "Desktop shell, navigation, local storage, and backup/export foundations are active.".into(),
            },
            ModuleStatus {
                id: "workspace".into(),
                label: "Multi-business workspace".into(),
                status: "active-foundation".into(),
                note: "Business switching and per-business settings stay isolated locally.".into(),
            },
            ModuleStatus {
                id: "catalog".into(),
                label: "Catalog / items".into(),
                status: "active-foundation".into(),
                note: "Products, menu items, services, categories, units, and barcode-ready item master data are now available.".into(),
            },
            ModuleStatus {
                id: "inventory".into(),
                label: "Inventory ledger".into(),
                status: "coming-next".into(),
                note: "Current stock fields and reorder points are ready for movement-ledger expansion.".into(),
            },
            ModuleStatus {
                id: "pos".into(),
                label: "POS / billing".into(),
                status: "planned".into(),
                note: "Catalog records, tax defaults, and sequences are in place for future billing workflows.".into(),
            },
        ],
    }
}

#[allow(clippy::too_many_arguments)]
pub fn build_app_bootstrap(
    app_info: AppInfo,
    active_business: BusinessProfile,
    business_settings: BusinessSettings,
    active_tax_profile: TaxProfile,
    active_receipt_profile: ReceiptProfile,
    active_module_flags: ModuleFlags,
    active_sequences: Vec<SequenceCounter>,
    businesses: Vec<BusinessProfile>,
    business_workspaces: Vec<BusinessWorkspaceSummary>,
    patch_history: Vec<PatchRecord>,
    backups: Vec<BackupRecord>,
    storage: StorageStatus,
    recent_activity: Vec<RecentActivity>,
    catalog_summary: CatalogSummary,
) -> AppBootstrap {
    let dashboard = compose_dashboard(
        &business_workspaces,
        &active_business,
        &active_module_flags,
        &active_sequences,
        &backups,
        recent_activity,
        &patch_history,
        &storage,
        &catalog_summary,
    );

    AppBootstrap {
        app_info,
        active_business,
        business_settings,
        active_tax_profile,
        active_receipt_profile,
        active_module_flags,
        active_sequences,
        businesses,
        business_workspaces,
        patch_history,
        backups,
        storage,
        catalog_summary,
        dashboard,
    }
}
```

## `files/src-tauri/src/domain/models.rs`

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxProfile {
    pub id: String,
    pub business_id: String,
    pub name: String,
    pub tax_label: String,
    pub default_rate: f64,
    pub prices_include_tax: bool,
    pub is_default: bool,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptProfile {
    pub id: String,
    pub business_id: String,
    pub name: String,
    pub footer_text: Option<String>,
    pub show_address: bool,
    pub show_phone: bool,
    pub show_email: bool,
    pub show_business_code: bool,
    pub paper_width: String,
    pub is_default: bool,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleFlags {
    pub business_id: String,
    pub restaurant_enabled: bool,
    pub retail_enabled: bool,
    pub inventory_enabled: bool,
    pub services_enabled: bool,
    pub customers_enabled: bool,
    pub suppliers_enabled: bool,
    pub expenses_enabled: bool,
    pub reporting_enabled: bool,
    pub data_center_enabled: bool,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceCounter {
    pub id: String,
    pub business_id: String,
    pub scope: String,
    pub prefix: String,
    pub next_number: i64,
    pub padding: i64,
    pub reset_policy: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewBusinessWorkspaceInput {
    pub name: String,
    pub legal_name: Option<String>,
    pub code: String,
    pub business_type: String,
    pub currency_code: String,
    pub tax_mode: String,
    pub timezone: String,
    pub locale: String,
    pub activate_now: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceConfigurationInput {
    pub business_settings: BusinessSettings,
    pub tax_profile: TaxProfile,
    pub receipt_profile: ReceiptProfile,
    pub module_flags: ModuleFlags,
    pub sequence_counters: Vec<SequenceCounter>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessWorkspaceSummary {
    pub business_id: String,
    pub name: String,
    pub code: String,
    pub business_type: String,
    pub currency_code: String,
    pub theme: String,
    pub timezone: String,
    pub tax_label: String,
    pub default_tax_rate: f64,
    pub next_sale_sequence: String,
    pub active_modules: Vec<String>,
    pub archived_at: Option<String>,
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
    pub category_count: usize,
    pub item_count: usize,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogCategory {
    pub id: String,
    pub business_id: String,
    pub name: String,
    pub code: String,
    pub parent_id: Option<String>,
    pub item_scope: String,
    pub sort_order: i64,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogUnit {
    pub id: String,
    pub business_id: Option<String>,
    pub name: String,
    pub code: String,
    pub symbol: String,
    pub allow_fractional: bool,
    pub is_system: bool,
    pub updated_at: String,
    pub archived_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogItem {
    pub id: String,
    pub business_id: String,
    pub category_id: Option<String>,
    pub unit_id: Option<String>,
    pub tax_profile_id: Option<String>,
    pub item_kind: String,
    pub name: String,
    pub display_name: Option<String>,
    pub sku: Option<String>,
    pub primary_barcode: Option<String>,
    pub description: Option<String>,
    pub selling_price: f64,
    pub cost_price: f64,
    pub track_stock: bool,
    pub stock_quantity: f64,
    pub reorder_level: f64,
    pub image_path: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogBarcode {
    pub id: String,
    pub item_id: String,
    pub barcode: String,
    pub label: Option<String>,
    pub is_primary: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogItemView {
    pub item: CatalogItem,
    pub category_name: Option<String>,
    pub unit_code: Option<String>,
    pub tax_label: Option<String>,
    pub barcodes: Vec<CatalogBarcode>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogSummary {
    pub total_items: usize,
    pub active_items: usize,
    pub archived_items: usize,
    pub category_count: usize,
    pub menu_item_count: usize,
    pub stock_item_count: usize,
    pub service_item_count: usize,
    pub low_stock_candidates: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogWorkspace {
    pub business_id: String,
    pub summary: CatalogSummary,
    pub categories: Vec<CatalogCategory>,
    pub units: Vec<CatalogUnit>,
    pub tax_profiles: Vec<TaxProfile>,
    pub items: Vec<CatalogItemView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveCatalogCategoryInput {
    pub id: Option<String>,
    pub name: String,
    pub code: String,
    pub parent_id: Option<String>,
    pub item_scope: String,
    pub sort_order: i64,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveCatalogUnitInput {
    pub id: Option<String>,
    pub name: String,
    pub code: String,
    pub symbol: String,
    pub allow_fractional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveCatalogItemInput {
    pub id: Option<String>,
    pub category_id: Option<String>,
    pub unit_id: Option<String>,
    pub tax_profile_id: Option<String>,
    pub item_kind: String,
    pub name: String,
    pub display_name: Option<String>,
    pub sku: Option<String>,
    pub barcodes: Vec<String>,
    pub description: Option<String>,
    pub selling_price: f64,
    pub cost_price: f64,
    pub track_stock: bool,
    pub stock_quantity: f64,
    pub reorder_level: f64,
    pub image_path: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBootstrap {
    pub app_info: AppInfo,
    pub active_business: BusinessProfile,
    pub business_settings: BusinessSettings,
    pub active_tax_profile: TaxProfile,
    pub active_receipt_profile: ReceiptProfile,
    pub active_module_flags: ModuleFlags,
    pub active_sequences: Vec<SequenceCounter>,
    pub businesses: Vec<BusinessProfile>,
    pub business_workspaces: Vec<BusinessWorkspaceSummary>,
    pub patch_history: Vec<PatchRecord>,
    pub backups: Vec<BackupRecord>,
    pub storage: StorageStatus,
    pub catalog_summary: CatalogSummary,
    pub dashboard: DashboardShellData,
}
```

## `files/src-tauri/src/lib.rs`

```rust
mod commands;
mod core;
mod domain;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            core::db::initialize(&handle).expect("failed to initialize local workspace core");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::bootstrap::bootstrap_app,
            commands::business::save_business_profile,
            commands::business::create_business_workspace,
            commands::business::switch_active_business,
            commands::settings::save_workspace_configuration,
            commands::catalog::load_catalog_workspace,
            commands::catalog::save_catalog_category,
            commands::catalog::save_catalog_unit,
            commands::catalog::save_catalog_item,
            commands::catalog::set_catalog_item_archived,
            commands::data_center::create_backup_snapshot,
            commands::data_center::export_foundation_snapshot,
            commands::data_center::preview_import_bundle
        ])
        .run(tauri::generate_context!())
        .expect("error while running local business manager");
}
```

## `files/src-tauri/tauri.conf.json`

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "Local Business Manager",
  "version": "0.3.0",
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

## `migrations/MIGRATION_NOTES.md`

```md
# MIGRATION_NOTES

## Migration file
- `src-tauri/src/core/migrations/003_catalog_core.sql`

## Schema version change
- previous schema version: `2`
- new schema version: `3`

## Migration actions
1. add compatibility columns:
   - `tax_profiles.prices_include_tax`
   - `receipt_profiles.show_email`
   - `receipt_profiles.show_business_code`
   - `sequence_counters.reset_policy`
2. backfill defaults for those new columns
3. create catalog tables:
   - `catalog_categories`
   - `catalog_units`
   - `catalog_items`
   - `catalog_item_barcodes`
4. create business-scoped indexes for category codes, item SKUs, and primary barcodes

## Seed changes after migration
On bootstrap, Patch 3 also ensures:
- system units exist
- the demo business gets a starter catalog if demo data is enabled

## Downgrade note
No automatic reverse migration is supplied. Use a pre-patch database backup if rollback is needed.
```

## `patch-manifest.json`

```json
{
  "patch_id": "P003_catalog_core",
  "patch_name": "Catalog Core",
  "base_version": "0.2.0",
  "target_version": "0.3.0",
  "description": "Adds business-scoped catalog structure for products, menu items, and services, including categories, units, items, barcode foundations, demo catalog seeding, and catalog-aware export/preview support.",
  "dependencies": [
    "P002_multi_business_workspace_settings_core"
  ],
  "safe_on_empty_project": false,
  "migration_required": true,
  "rollback_supported": true,
  "files_root": "files",
  "files_added": [
    "scripts/validate-patch3.mjs",
    "src-tauri/src/commands/catalog.rs",
    "src-tauri/src/core/catalog.rs",
    "src-tauri/src/core/migrations/003_catalog_core.sql",
    "src/modules/catalog/CatalogPage.tsx"
  ],
  "files_updated": [
    "package.json",
    "src-tauri/Cargo.toml",
    "src-tauri/src/commands/bootstrap.rs",
    "src-tauri/src/commands/data_center.rs",
    "src-tauri/src/commands/mod.rs",
    "src-tauri/src/core/db.rs",
    "src-tauri/src/core/migrations.rs",
    "src-tauri/src/core/mod.rs",
    "src-tauri/src/core/patching.rs",
    "src-tauri/src/core/seed.rs",
    "src-tauri/src/domain/bootstrap.rs",
    "src-tauri/src/domain/models.rs",
    "src-tauri/src/lib.rs",
    "src-tauri/tauri.conf.json",
    "src/app/AppProvider.tsx",
    "src/modules/business/BusinessPage.tsx",
    "src/modules/dashboard/DashboardPage.tsx",
    "src/modules/data-center/DataCenterPage.tsx",
    "src/modules/settings/SettingsPage.tsx",
    "src/modules/shell/AppShell.tsx",
    "src/shared/api.ts",
    "src/shared/types.ts",
    "src/shared/utils.ts",
    "src/styles.css"
  ],
  "files_deleted": [],
  "post_apply_steps": [
    "cd <target>",
    "npm install",
    "npm run validate:patch3",
    "npm run tauri dev"
  ]
}
```

## `rollback.md`

```md
# Rollback notes

## File rollback
The patch applier stores overwritten files in:

```text
.patch-backups/P003_catalog_core/
```

You can restore those files manually if needed.

## Database rollback
Patch 3 includes schema migration 003. Once the app starts after patch apply, the local database may be upgraded to schema version 3.

Recommended rollback path:
1. close the app
2. restore the pre-patch SQLite snapshot from your backup folder
3. restore overwritten source files from `.patch-backups/P003_catalog_core/`
4. reinstall dependencies if required

Do not attempt an in-place SQL downgrade unless you fully control the target database and have tested the downgrade separately.
```

## `validate.md`

```md
# Validation checklist

Run after applying the patch from the target project root:

```bash
npm install
npm run validate:patch3
npm run tauri dev
```

Manual checks:
- [ ] App starts without bootstrap errors
- [ ] Sidebar shows Catalog navigation
- [ ] Dashboard hero reflects Patch 3 catalog core
- [ ] Catalog page loads categories, units, taxes, and items for the active business
- [ ] Demo workspace contains seed items such as beverages / retail / service entries
- [ ] Creating a category saves and appears immediately after refresh
- [ ] Creating a unit saves and is selectable in the item form
- [ ] Creating an item updates catalog summary counts
- [ ] Archiving an item removes it from the default active list
- [ ] Export bundle now includes `catalogCategories`, `catalogUnits`, `catalogItems`, and `catalogBarcodes`
- [ ] Import preview reports business, category, and item counts
```

