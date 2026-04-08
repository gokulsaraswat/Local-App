# Patch 2 Full File Contents

## `PATCH-2-README.md`

```md
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

```

## `PATCH_NOTES.md`

```md
# Patch 2 Notes

## Goal
Turn the Patch 1 desktop foundation into a true multi-business local workspace.

## Adds
- Multi-business creation flow
- Safe active business switching
- Soft archive support for businesses
- Active business dashboard and navigation updates
- Per-business default tax profile foundation
- Per-business default receipt profile foundation
- Per-business sequence counters foundation
- Expanded export bundle scope for workspace metadata
- Validation script for Patch 2 structure

## Data changes
- Added `tax_profiles`
- Added `receipt_profiles`
- Added `sequence_counters`
- Seed/backfill now ensures every business has default workspace support rows
- Patch level moves from `P001_foundation_base_structure` to `P002_multi_business_workspace_settings_core`

## Deferred
- Product catalog
- POS billing flows
- Inventory ledgers
- Full import apply
- Local users and roles

```

## `TREE.txt`

```
P02_multi_business_workspace_settings_core/PATCH-2-README.md
P02_multi_business_workspace_settings_core/PATCH_NOTES.md
P02_multi_business_workspace_settings_core/TREE.txt
P02_multi_business_workspace_settings_core/apply_patch.mjs
P02_multi_business_workspace_settings_core/apply_patch.ps1
P02_multi_business_workspace_settings_core/apply_patch.sh
P02_multi_business_workspace_settings_core/files/package.json
P02_multi_business_workspace_settings_core/files/scripts/validate-patch2.mjs
P02_multi_business_workspace_settings_core/files/src-tauri/Cargo.toml
P02_multi_business_workspace_settings_core/files/src-tauri/src/commands/bootstrap.rs
P02_multi_business_workspace_settings_core/files/src-tauri/src/commands/business.rs
P02_multi_business_workspace_settings_core/files/src-tauri/src/commands/data_center.rs
P02_multi_business_workspace_settings_core/files/src-tauri/src/commands/settings.rs
P02_multi_business_workspace_settings_core/files/src-tauri/src/core/db.rs
P02_multi_business_workspace_settings_core/files/src-tauri/src/core/migrations.rs
P02_multi_business_workspace_settings_core/files/src-tauri/src/core/migrations/002_multi_business_workspace.sql
P02_multi_business_workspace_settings_core/files/src-tauri/src/core/patching.rs
P02_multi_business_workspace_settings_core/files/src-tauri/src/core/seed.rs
P02_multi_business_workspace_settings_core/files/src-tauri/src/domain/bootstrap.rs
P02_multi_business_workspace_settings_core/files/src-tauri/src/domain/models.rs
P02_multi_business_workspace_settings_core/files/src-tauri/src/lib.rs
P02_multi_business_workspace_settings_core/files/src-tauri/tauri.conf.json
P02_multi_business_workspace_settings_core/files/src/app/AppProvider.tsx
P02_multi_business_workspace_settings_core/files/src/modules/business/BusinessPage.tsx
P02_multi_business_workspace_settings_core/files/src/modules/dashboard/DashboardPage.tsx
P02_multi_business_workspace_settings_core/files/src/modules/data-center/DataCenterPage.tsx
P02_multi_business_workspace_settings_core/files/src/modules/settings/SettingsPage.tsx
P02_multi_business_workspace_settings_core/files/src/modules/shell/AppShell.tsx
P02_multi_business_workspace_settings_core/files/src/shared/api.ts
P02_multi_business_workspace_settings_core/files/src/shared/types.ts
P02_multi_business_workspace_settings_core/files/src/shared/utils.ts
P02_multi_business_workspace_settings_core/files/src/styles.css
P02_multi_business_workspace_settings_core/migrations/MIGRATION_NOTES.md
P02_multi_business_workspace_settings_core/patch-manifest.json
P02_multi_business_workspace_settings_core/rollback.md
P02_multi_business_workspace_settings_core/validate.md

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

function validatePatch1Base() {
  const packageJsonPath = path.join(targetPath, "package.json");
  const patchingPath = path.join(targetPath, "src-tauri", "src", "core", "patching.rs");

  if (!fs.existsSync(packageJsonPath) || !fs.existsSync(patchingPath)) {
    throw new Error(
      "Patch 2 expects an existing Patch 1 project. Missing package.json or src-tauri/src/core/patching.rs."
    );
  }

  const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));
  const patchingSource = fs.readFileSync(patchingPath, "utf8");
  const looksLikePatch1 =
    String(packageJson.version || "").startsWith("0.1") ||
    patchingSource.includes("P001_foundation_base_structure");

  if (!looksLikePatch1 && !force) {
    throw new Error(
      "Target project does not look like a Patch 1 base. Re-run with --force only if you are sure."
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
  validatePatch1Base();

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
  "version": "0.2.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "tauri": "tauri",
    "typecheck": "tsc --noEmit",
    "validate:patch2": "node scripts/validate-patch2.mjs"
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

## `files/scripts/validate-patch2.mjs`

```js
import fs from "node:fs";
import path from "node:path";

const requiredFiles = [
  "package.json",
  "src/shared/types.ts",
  "src/app/AppProvider.tsx",
  "src/modules/business/BusinessPage.tsx",
  "src/modules/settings/SettingsPage.tsx",
  "src-tauri/src/core/migrations/002_multi_business_workspace.sql",
  "src-tauri/src/commands/business.rs",
  "src-tauri/src/commands/settings.rs"
];

const requiredSnippets = [
  ["src/shared/types.ts", "businessWorkspaces: BusinessWorkspaceSummary[];"],
  ["src/modules/business/BusinessPage.tsx", "Create Business Workspace"],
  ["src/modules/settings/SettingsPage.tsx", "Sequence counters foundation"],
  ["src-tauri/src/core/patching.rs", "P002_multi_business_workspace_settings_core"],
  ["src-tauri/src/core/migrations.rs", "CURRENT_SCHEMA_VERSION: i64 = 2"],
  ["src-tauri/src/lib.rs", "create_business_workspace"],
  ["src-tauri/src/lib.rs", "save_workspace_configuration"]
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

if (hasError) {
  process.exit(1);
}

console.log("[OK] Patch 2 structural validation passed.");

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
          Initializing migrations, business workspaces, settings profiles, and
          the local patch registry.
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
            <span className="pill warning">Patch 2</span>
          </div>
          <p className="card-note">
            Add another business profile with isolated local settings. Inventory,
            catalog, and sales modules can attach to it in future patches.
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
          <div className="section-kicker">Patch 2 workspace core</div>
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
            onClick={() => onNavigate("settings")}
          >
            Open Settings Profiles
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
        `Bundle OK · type=${preview.bundleType ?? "unknown"} · source=${preview.sourcePatchLevel ?? "unknown"} · generated=${preview.generatedAt ?? "unknown"} · businesses=${preview.businessCount}`
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
            Patch 2 upgrades export scope from a single active business snapshot to
            a multi-business workspace foundation export. Backups still stay fully
            local and file-based.
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
            Patch 2 still stops at preview validation, but it now understands the
            workspace export bundle type and shows cross-business counts.
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
            Patch 2 stores tax defaults, receipt defaults, module flags, and
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
            <span className="pill warning">Patch 2</span>
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
          <div className="brand-badge">P2</div>
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
          </div>
          <div className="muted-text small-text">
            {switchStatus || activeWorkspace?.nextSaleSequence || "Sequence pending"}
          </div>
        </div>

        <div className="sidebar-section-label">Business mode</div>
        <div className="sidebar-card">
          <div className="sidebar-pill success">Multi-business ready</div>
          <div className="sidebar-pill neutral">Settings profiles ready</div>
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
              Patch 2 expands the foundation into a practical multi-business local
              workspace.
            </p>
          </div>
          <div className="workspace-header-meta">
            <span className="meta-chip">Business: {data.activeBusiness.code}</span>
            <span className="meta-chip">
              Tax: {data.activeTaxProfile.taxLabel}
            </span>
            <span className="meta-chip">Schema v{data.appInfo.schemaVersion}</span>
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

## `files/src/shared/api.ts`

```ts
import { invoke } from "@tauri-apps/api/core";
import type {
  AppBootstrap,
  BackupRecord,
  BusinessProfile,
  ImportPreview,
  NewBusinessWorkspaceInput,
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
  dashboard: DashboardShellData;
}

export type NavPage = "dashboard" | "business" | "settings" | "data-center";

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

```

## `files/src-tauri/Cargo.toml`

```
[package]
name = "local-business-manager"
version = "0.2.0"
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

## `files/src-tauri/src/commands/bootstrap.rs`

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
        ))
    })
}

```

## `files/src-tauri/src/commands/business.rs`

```rust
use tauri::AppHandle;

use crate::{
    core::{db, error::CommandResult},
    domain::models::{BusinessProfile, NewBusinessWorkspaceInput},
};

#[tauri::command]
pub fn save_business_profile(app: AppHandle, profile: BusinessProfile) -> CommandResult<BusinessProfile> {
    db::with_connection(&app, |conn, _paths| db::save_business_profile(conn, &profile))
}

#[tauri::command]
pub fn create_business_workspace(
    app: AppHandle,
    input: NewBusinessWorkspaceInput,
) -> CommandResult<BusinessProfile> {
    db::with_connection(&app, |conn, _paths| db::create_business_workspace(conn, &input))
}

#[tauri::command]
pub fn switch_active_business(app: AppHandle, business_id: String) -> CommandResult<BusinessProfile> {
    db::with_connection(&app, |conn, _paths| db::switch_active_business(conn, &business_id))
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
            "bundleVersion": "2.0.0",
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
        format: "json-workspace-foundation".into(),
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
        generated_at: manifest
            .get("generatedAt")
            .and_then(|value| value.as_str())
            .map(ToString::to_string),
        warnings,
    };

    db::with_connection(&app, |conn, _paths| {
        db::insert_import_job(conn, None, "json-workspace-foundation", "previewed", &file_path)?;
        db::insert_log(conn, "INFO", "import", "Import bundle previewed", None)?;
        Ok(())
    })?;

    Ok(preview)
}

```

## `files/src-tauri/src/commands/settings.rs`

```rust
use tauri::AppHandle;

use crate::{
    core::{db, error::CommandResult},
    domain::models::WorkspaceConfigurationInput,
};

#[tauri::command]
pub fn save_workspace_configuration(
    app: AppHandle,
    input: WorkspaceConfigurationInput,
) -> CommandResult<()> {
    db::with_connection(&app, |conn, _paths| db::save_workspace_configuration(conn, &input))
}

```

## `files/src-tauri/src/core/db.rs`

```rust
use std::{cmp, fs};

use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension, Row};
use tauri::AppHandle;
use uuid::Uuid;

use crate::domain::models::{
    AppInfo, BackupRecord, BusinessProfile, BusinessSettings, ExportJobRecord, ModuleFlag,
    PatchRecord, ReceiptProfile, RecentActivity, SequenceCounter, StorageStatus, TaxProfile,
    NewBusinessInput,
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

fn tax_profile_from_row(row: &Row) -> rusqlite::Result<TaxProfile> {
    Ok(TaxProfile {
        id: row.get(0)?,
        business_id: row.get(1)?,
        name: row.get(2)?,
        label: row.get(3)?,
        rate: row.get(4)?,
        is_default: bool_from_row(row, 5)?,
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
    })
}

fn receipt_profile_from_row(row: &Row) -> rusqlite::Result<ReceiptProfile> {
    Ok(ReceiptProfile {
        id: row.get(0)?,
        business_id: row.get(1)?,
        name: row.get(2)?,
        header_line1: row.get(3)?,
        header_line2: row.get(4)?,
        footer_text: row.get(5)?,
        show_address: bool_from_row(row, 6)?,
        show_phone: bool_from_row(row, 7)?,
        show_tax_breakdown: bool_from_row(row, 8)?,
        paper_width: row.get(9)?,
        copies: row.get(10)?,
        is_default: bool_from_row(row, 11)?,
        created_at: row.get(12)?,
        updated_at: row.get(13)?,
    })
}

fn module_flag_from_row(row: &Row) -> rusqlite::Result<ModuleFlag> {
    Ok(ModuleFlag {
        business_id: row.get(0)?,
        module_key: row.get(1)?,
        enabled: bool_from_row(row, 2)?,
        updated_at: row.get(3)?,
    })
}

fn sequence_counter_from_row(row: &Row) -> rusqlite::Result<SequenceCounter> {
    Ok(SequenceCounter {
        business_id: row.get(0)?,
        counter_key: row.get(1)?,
        prefix: row.get(2)?,
        next_number: row.get(3)?,
        padding: row.get(4)?,
        updated_at: row.get(5)?,
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
             ORDER BY CASE WHEN archived_at IS NULL THEN 0 ELSE 1 END, updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare businesses query", error))?;

    let rows = stmt
        .query_map([], business_from_row)
        .map_err(|error| to_command_error("failed to query businesses", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map businesses", error))
}

fn load_business_by_id(conn: &Connection, business_id: &str) -> Result<Option<BusinessProfile>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                id, name, legal_name, code, business_type, currency_code, tax_mode,
                phone, email, address_line1, address_line2, city, state, postal_code,
                country, created_at, updated_at, archived_at
             FROM businesses WHERE id = ?1 LIMIT 1",
        )
        .map_err(|error| to_command_error("failed to prepare business query", error))?;

    stmt.query_row(params![business_id], business_from_row)
        .optional()
        .map_err(|error| to_command_error("failed to load business by id", error))
}

pub fn get_business_by_id(conn: &Connection, business_id: &str) -> Result<BusinessProfile, String> {
    load_business_by_id(conn, business_id)?
        .ok_or_else(|| format!("business not found: {business_id}"))
}

pub fn get_active_business(conn: &Connection) -> Result<BusinessProfile, String> {
    if let Some(active_id) = get_meta(conn, "active_business_id")? {
        if let Some(business) = load_business_by_id(conn, &active_id)? {
            if business.archived_at.is_none() {
                return Ok(business);
            }
        }
    }

    let fallback = list_businesses(conn)?
        .into_iter()
        .find(|business| business.archived_at.is_none())
        .ok_or_else(|| "no businesses found in local storage".to_string())?;

    set_meta(conn, "active_business_id", &fallback.id)?;
    Ok(fallback)
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

pub fn list_all_business_settings(conn: &Connection) -> Result<Vec<BusinessSettings>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                business_id, timezone, locale, date_format, theme, tax_label,
                default_tax_rate, prices_include_tax, receipt_footer, receipt_show_address,
                receipt_show_phone, auto_backup_enabled, backup_directory,
                module_restaurant_enabled, module_retail_enabled, module_inventory_enabled,
                module_services_enabled, updated_at
             FROM business_settings
             ORDER BY updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare business settings list", error))?;

    let rows = stmt
        .query_map([], settings_from_row)
        .map_err(|error| to_command_error("failed to query business settings", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map business settings", error))
}

fn insert_business_settings_row(conn: &Connection, settings: &BusinessSettings) -> Result<(), String> {
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
            &settings.backup_directory,
            if settings.module_restaurant_enabled { 1_i64 } else { 0_i64 },
            if settings.module_retail_enabled { 1_i64 } else { 0_i64 },
            if settings.module_inventory_enabled { 1_i64 } else { 0_i64 },
            if settings.module_services_enabled { 1_i64 } else { 0_i64 },
            &settings.updated_at
        ],
    )
    .map_err(|error| to_command_error("failed to insert business settings", error))?;
    Ok(())
}

pub fn create_business_profile(conn: &Connection, input: &NewBusinessInput) -> Result<BusinessProfile, String> {
    let name = input.name.trim().to_string();
    if name.is_empty() {
        return Err("business name cannot be empty".into());
    }

    let code = input.code.trim().to_uppercase();
    if code.is_empty() {
        return Err("business code cannot be empty".into());
    }

    let business_id = Uuid::new_v4().to_string();
    let now = now_iso();

    let template_business_id = if input.copy_from_active_business {
        get_meta(conn, "active_business_id")?
    } else {
        None
    };

    let template_settings = if let Some(template_id) = template_business_id.as_deref() {
        Some(get_business_settings(conn, template_id)?)
    } else {
        None
    };

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
            input.business_type.trim(),
            input.currency_code.trim().to_uppercase(),
            input.tax_mode.trim(),
            normalize_optional(&input.phone),
            normalize_optional(&input.email),
            normalize_optional(&input.address_line1),
            normalize_optional(&input.address_line2),
            normalize_optional(&input.city),
            normalize_optional(&input.state),
            normalize_optional(&input.postal_code),
            normalize_optional(&input.country),
            &now,
            &now,
            Option::<String>::None
        ],
    )
    .map_err(|error| to_command_error("failed to create business profile", error))?;

    let settings = BusinessSettings {
        business_id: business_id.clone(),
        timezone: template_settings
            .as_ref()
            .map(|value| value.timezone.clone())
            .unwrap_or_else(|| "Asia/Kolkata".into()),
        locale: template_settings
            .as_ref()
            .map(|value| value.locale.clone())
            .unwrap_or_else(|| "en-IN".into()),
        date_format: template_settings
            .as_ref()
            .map(|value| value.date_format.clone())
            .unwrap_or_else(|| "DD-MM-YYYY".into()),
        theme: template_settings
            .as_ref()
            .map(|value| value.theme.clone())
            .unwrap_or_else(|| "system".into()),
        tax_label: template_settings
            .as_ref()
            .map(|value| value.tax_label.clone())
            .unwrap_or_else(|| "GST".into()),
        default_tax_rate: template_settings
            .as_ref()
            .map(|value| value.default_tax_rate)
            .unwrap_or(0.0),
        prices_include_tax: template_settings
            .as_ref()
            .map(|value| value.prices_include_tax)
            .unwrap_or(false),
        receipt_footer: template_settings
            .as_ref()
            .and_then(|value| value.receipt_footer.clone())
            .or_else(|| Some("Thank you for supporting local business.".into())),
        receipt_show_address: template_settings
            .as_ref()
            .map(|value| value.receipt_show_address)
            .unwrap_or(true),
        receipt_show_phone: template_settings
            .as_ref()
            .map(|value| value.receipt_show_phone)
            .unwrap_or(true),
        auto_backup_enabled: template_settings
            .as_ref()
            .map(|value| value.auto_backup_enabled)
            .unwrap_or(false),
        backup_directory: template_settings
            .as_ref()
            .and_then(|value| value.backup_directory.clone()),
        module_restaurant_enabled: template_settings
            .as_ref()
            .map(|value| value.module_restaurant_enabled)
            .unwrap_or(false),
        module_retail_enabled: template_settings
            .as_ref()
            .map(|value| value.module_retail_enabled)
            .unwrap_or(true),
        module_inventory_enabled: template_settings
            .as_ref()
            .map(|value| value.module_inventory_enabled)
            .unwrap_or(true),
        module_services_enabled: template_settings
            .as_ref()
            .map(|value| value.module_services_enabled)
            .unwrap_or(false),
        updated_at: now.clone(),
    };

    insert_business_settings_row(conn, &settings)?;
    seed::ensure_workspace_support_for_business(conn, &business_id)
        .map_err(|error| to_command_error("failed to seed workspace support", error))?;

    if let Some(template_id) = template_business_id.as_deref() {
        if input.copy_from_active_business {
            seed::copy_workspace_preferences_from_template(conn, template_id, &business_id)
                .map_err(|error| to_command_error("failed to copy workspace preferences", error))?;
        }
    }

    if input.create_as_active {
        set_meta(conn, "active_business_id", &business_id)?;
        insert_log(
            conn,
            "INFO",
            "workspace",
            "Active business switched after creation",
            None,
        )?;
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
    let updated_at = now_iso();
    let rows_affected = conn
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
                profile.name.trim(),
                normalize_optional(&profile.legal_name),
                profile.code.trim().to_uppercase(),
                profile.business_type.trim(),
                profile.currency_code.trim().to_uppercase(),
                profile.tax_mode.trim(),
                normalize_optional(&profile.phone),
                normalize_optional(&profile.email),
                normalize_optional(&profile.address_line1),
                normalize_optional(&profile.address_line2),
                normalize_optional(&profile.city),
                normalize_optional(&profile.state),
                normalize_optional(&profile.postal_code),
                normalize_optional(&profile.country),
                &updated_at
            ],
        )
        .map_err(|error| to_command_error("failed to update business profile", error))?;

    if rows_affected == 0 {
        return Err("business profile not found".into());
    }

    insert_log(conn, "INFO", "business", "Business profile updated", None)?;
    get_business_by_id(conn, &profile.id)
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

    let rows_affected = conn
        .execute(
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
                settings.timezone.trim(),
                settings.locale.trim(),
                settings.date_format.trim(),
                settings.theme.trim(),
                settings.tax_label.trim(),
                settings.default_tax_rate,
                if settings.prices_include_tax { 1_i64 } else { 0_i64 },
                normalize_optional(&settings.receipt_footer),
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

    if rows_affected == 0 {
        return Err("business settings not found".into());
    }

    insert_log(conn, "INFO", "settings", "Business settings updated", None)?;
    get_business_settings(conn, &settings.business_id)
}

pub fn list_tax_profiles(conn: &Connection, business_id: &str) -> Result<Vec<TaxProfile>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, business_id, name, label, rate, is_default, created_at, updated_at
             FROM tax_profiles
             WHERE business_id = ?1
             ORDER BY is_default DESC, updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare tax profiles query", error))?;

    let rows = stmt
        .query_map(params![business_id], tax_profile_from_row)
        .map_err(|error| to_command_error("failed to query tax profiles", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map tax profiles", error))
}

fn get_tax_profile_by_id(conn: &Connection, profile_id: &str) -> Result<TaxProfile, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, business_id, name, label, rate, is_default, created_at, updated_at
             FROM tax_profiles
             WHERE id = ?1
             LIMIT 1",
        )
        .map_err(|error| to_command_error("failed to prepare tax profile lookup", error))?;

    stmt.query_row(params![profile_id], tax_profile_from_row)
        .map_err(|error| to_command_error("failed to load tax profile", error))
}

pub fn list_all_tax_profiles(conn: &Connection) -> Result<Vec<TaxProfile>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, business_id, name, label, rate, is_default, created_at, updated_at
             FROM tax_profiles
             ORDER BY business_id ASC, is_default DESC, updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare all tax profiles query", error))?;

    let rows = stmt
        .query_map([], tax_profile_from_row)
        .map_err(|error| to_command_error("failed to query all tax profiles", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map all tax profiles", error))
}

pub fn save_tax_profile(conn: &Connection, profile: &TaxProfile) -> Result<TaxProfile, String> {
    let now = now_iso();
    let profile_id = if profile.id.trim().is_empty() {
        Uuid::new_v4().to_string()
    } else {
        profile.id.clone()
    };

    let created_at = conn
        .query_row(
            "SELECT created_at FROM tax_profiles WHERE id = ?1 LIMIT 1",
            params![&profile_id],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|error| to_command_error("failed to inspect tax profile", error))?
        .unwrap_or_else(|| now.clone());

    if profile.is_default {
        conn.execute(
            "UPDATE tax_profiles SET is_default = 0, updated_at = ?2 WHERE business_id = ?1",
            params![&profile.business_id, &now],
        )
        .map_err(|error| to_command_error("failed to clear default tax profile", error))?;
    }

    conn.execute(
        "INSERT INTO tax_profiles (id, business_id, name, label, rate, is_default, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
         ON CONFLICT(id) DO UPDATE SET
             business_id = excluded.business_id,
             name = excluded.name,
             label = excluded.label,
             rate = excluded.rate,
             is_default = excluded.is_default,
             updated_at = excluded.updated_at",
        params![
            &profile_id,
            &profile.business_id,
            profile.name.trim(),
            profile.label.trim(),
            profile.rate,
            if profile.is_default { 1_i64 } else { 0_i64 },
            &created_at,
            &now
        ],
    )
    .map_err(|error| to_command_error("failed to save tax profile", error))?;

    conn.execute(
        "UPDATE business_settings
         SET tax_label = ?2, default_tax_rate = ?3, updated_at = ?4
         WHERE business_id = ?1",
        params![&profile.business_id, profile.label.trim(), profile.rate, &now],
    )
    .map_err(|error| to_command_error("failed to sync business tax defaults", error))?;

    insert_log(conn, "INFO", "settings", "Default tax profile updated", None)?;
    get_tax_profile_by_id(conn, &profile_id)
}

pub fn list_receipt_profiles(conn: &Connection, business_id: &str) -> Result<Vec<ReceiptProfile>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                id, business_id, name, header_line1, header_line2, footer_text,
                show_address, show_phone, show_tax_breakdown, paper_width,
                copies, is_default, created_at, updated_at
             FROM receipt_profiles
             WHERE business_id = ?1
             ORDER BY is_default DESC, updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare receipt profiles query", error))?;

    let rows = stmt
        .query_map(params![business_id], receipt_profile_from_row)
        .map_err(|error| to_command_error("failed to query receipt profiles", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map receipt profiles", error))
}

fn get_receipt_profile_by_id(conn: &Connection, profile_id: &str) -> Result<ReceiptProfile, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                id, business_id, name, header_line1, header_line2, footer_text,
                show_address, show_phone, show_tax_breakdown, paper_width,
                copies, is_default, created_at, updated_at
             FROM receipt_profiles
             WHERE id = ?1
             LIMIT 1",
        )
        .map_err(|error| to_command_error("failed to prepare receipt profile lookup", error))?;

    stmt.query_row(params![profile_id], receipt_profile_from_row)
        .map_err(|error| to_command_error("failed to load receipt profile", error))
}

pub fn list_all_receipt_profiles(conn: &Connection) -> Result<Vec<ReceiptProfile>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT
                id, business_id, name, header_line1, header_line2, footer_text,
                show_address, show_phone, show_tax_breakdown, paper_width,
                copies, is_default, created_at, updated_at
             FROM receipt_profiles
             ORDER BY business_id ASC, is_default DESC, updated_at DESC",
        )
        .map_err(|error| to_command_error("failed to prepare all receipt profiles query", error))?;

    let rows = stmt
        .query_map([], receipt_profile_from_row)
        .map_err(|error| to_command_error("failed to query all receipt profiles", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map all receipt profiles", error))
}

pub fn save_receipt_profile(conn: &Connection, profile: &ReceiptProfile) -> Result<ReceiptProfile, String> {
    let now = now_iso();
    let profile_id = if profile.id.trim().is_empty() {
        Uuid::new_v4().to_string()
    } else {
        profile.id.clone()
    };

    let created_at = conn
        .query_row(
            "SELECT created_at FROM receipt_profiles WHERE id = ?1 LIMIT 1",
            params![&profile_id],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|error| to_command_error("failed to inspect receipt profile", error))?
        .unwrap_or_else(|| now.clone());

    if profile.is_default {
        conn.execute(
            "UPDATE receipt_profiles SET is_default = 0, updated_at = ?2 WHERE business_id = ?1",
            params![&profile.business_id, &now],
        )
        .map_err(|error| to_command_error("failed to clear default receipt profile", error))?;
    }

    conn.execute(
        "INSERT INTO receipt_profiles (
            id, business_id, name, header_line1, header_line2, footer_text,
            show_address, show_phone, show_tax_breakdown, paper_width,
            copies, is_default, created_at, updated_at
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6,
            ?7, ?8, ?9, ?10,
            ?11, ?12, ?13, ?14
        )
        ON CONFLICT(id) DO UPDATE SET
            business_id = excluded.business_id,
            name = excluded.name,
            header_line1 = excluded.header_line1,
            header_line2 = excluded.header_line2,
            footer_text = excluded.footer_text,
            show_address = excluded.show_address,
            show_phone = excluded.show_phone,
            show_tax_breakdown = excluded.show_tax_breakdown,
            paper_width = excluded.paper_width,
            copies = excluded.copies,
            is_default = excluded.is_default,
            updated_at = excluded.updated_at",
        params![
            &profile_id,
            &profile.business_id,
            profile.name.trim(),
            normalize_optional(&profile.header_line1),
            normalize_optional(&profile.header_line2),
            normalize_optional(&profile.footer_text),
            if profile.show_address { 1_i64 } else { 0_i64 },
            if profile.show_phone { 1_i64 } else { 0_i64 },
            if profile.show_tax_breakdown { 1_i64 } else { 0_i64 },
            profile.paper_width.trim(),
            cmp::max(1, profile.copies),
            if profile.is_default { 1_i64 } else { 0_i64 },
            &created_at,
            &now
        ],
    )
    .map_err(|error| to_command_error("failed to save receipt profile", error))?;

    conn.execute(
        "UPDATE business_settings
         SET receipt_footer = ?2,
             receipt_show_address = ?3,
             receipt_show_phone = ?4,
             updated_at = ?5
         WHERE business_id = ?1",
        params![
            &profile.business_id,
            normalize_optional(&profile.footer_text),
            if profile.show_address { 1_i64 } else { 0_i64 },
            if profile.show_phone { 1_i64 } else { 0_i64 },
            &now
        ],
    )
    .map_err(|error| to_command_error("failed to sync business receipt defaults", error))?;

    insert_log(conn, "INFO", "settings", "Default receipt profile updated", None)?;
    get_receipt_profile_by_id(conn, &profile_id)
}

pub fn list_module_flags(conn: &Connection, business_id: &str) -> Result<Vec<ModuleFlag>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT business_id, module_key, enabled, updated_at
             FROM module_flags
             WHERE business_id = ?1
             ORDER BY module_key ASC",
        )
        .map_err(|error| to_command_error("failed to prepare module flags query", error))?;

    let rows = stmt
        .query_map(params![business_id], module_flag_from_row)
        .map_err(|error| to_command_error("failed to query module flags", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map module flags", error))
}

pub fn list_all_module_flags(conn: &Connection) -> Result<Vec<ModuleFlag>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT business_id, module_key, enabled, updated_at
             FROM module_flags
             ORDER BY business_id ASC, module_key ASC",
        )
        .map_err(|error| to_command_error("failed to prepare all module flags query", error))?;

    let rows = stmt
        .query_map([], module_flag_from_row)
        .map_err(|error| to_command_error("failed to query all module flags", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map all module flags", error))
}

fn module_flag_value(flags: &[ModuleFlag], key: &str, fallback: bool) -> bool {
    flags.iter()
        .find(|flag| flag.module_key == key)
        .map(|flag| flag.enabled)
        .unwrap_or(fallback)
}

pub fn save_module_flags(
    conn: &Connection,
    business_id: &str,
    flags: &[ModuleFlag],
) -> Result<Vec<ModuleFlag>, String> {
    let now = now_iso();
    for flag in flags {
        conn.execute(
            "INSERT INTO module_flags (business_id, module_key, enabled, updated_at)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(business_id, module_key)
             DO UPDATE SET enabled = excluded.enabled, updated_at = excluded.updated_at",
            params![
                business_id,
                flag.module_key.trim(),
                if flag.enabled { 1_i64 } else { 0_i64 },
                &now
            ],
        )
        .map_err(|error| to_command_error("failed to save module flag", error))?;
    }

    let current_settings = get_business_settings(conn, business_id)?;
    conn.execute(
        "UPDATE business_settings
         SET module_restaurant_enabled = ?2,
             module_retail_enabled = ?3,
             module_inventory_enabled = ?4,
             module_services_enabled = ?5,
             updated_at = ?6
         WHERE business_id = ?1",
        params![
            business_id,
            if module_flag_value(flags, "restaurant", current_settings.module_restaurant_enabled) { 1_i64 } else { 0_i64 },
            if module_flag_value(flags, "retail", current_settings.module_retail_enabled) { 1_i64 } else { 0_i64 },
            if module_flag_value(flags, "inventory", current_settings.module_inventory_enabled) { 1_i64 } else { 0_i64 },
            if module_flag_value(flags, "services", current_settings.module_services_enabled) { 1_i64 } else { 0_i64 },
            &now
        ],
    )
    .map_err(|error| to_command_error("failed to sync module flags to business settings", error))?;

    insert_log(conn, "INFO", "settings", "Module flags updated", None)?;
    list_module_flags(conn, business_id)
}

pub fn list_sequence_counters(conn: &Connection, business_id: &str) -> Result<Vec<SequenceCounter>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT business_id, counter_key, prefix, next_number, padding, updated_at
             FROM sequence_counters
             WHERE business_id = ?1
             ORDER BY counter_key ASC",
        )
        .map_err(|error| to_command_error("failed to prepare sequence counters query", error))?;

    let rows = stmt
        .query_map(params![business_id], sequence_counter_from_row)
        .map_err(|error| to_command_error("failed to query sequence counters", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map sequence counters", error))
}

pub fn list_all_sequence_counters(conn: &Connection) -> Result<Vec<SequenceCounter>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT business_id, counter_key, prefix, next_number, padding, updated_at
             FROM sequence_counters
             ORDER BY business_id ASC, counter_key ASC",
        )
        .map_err(|error| to_command_error("failed to prepare all sequence counters query", error))?;

    let rows = stmt
        .query_map([], sequence_counter_from_row)
        .map_err(|error| to_command_error("failed to query all sequence counters", error))?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| to_command_error("failed to map all sequence counters", error))
}

pub fn save_sequence_counters(
    conn: &Connection,
    business_id: &str,
    counters: &[SequenceCounter],
) -> Result<Vec<SequenceCounter>, String> {
    let now = now_iso();
    for counter in counters {
        conn.execute(
            "INSERT INTO sequence_counters (business_id, counter_key, prefix, next_number, padding, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(business_id, counter_key)
             DO UPDATE SET prefix = excluded.prefix, next_number = excluded.next_number, padding = excluded.padding, updated_at = excluded.updated_at",
            params![
                business_id,
                counter.counter_key.trim(),
                counter.prefix.trim().to_uppercase(),
                cmp::max(1, counter.next_number),
                cmp::max(1, counter.padding),
                &now
            ],
        )
        .map_err(|error| to_command_error("failed to save sequence counter", error))?;
    }

    insert_log(conn, "INFO", "settings", "Sequence counters updated", None)?;
    list_sequence_counters(conn, business_id)
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
        version: get_meta(conn, "app_version")?.unwrap_or_else(|| "0.2.0".into()),
        schema_version: get_meta(conn, "schema_version")?
            .and_then(|value| value.parse::<i64>().ok())
            .unwrap_or(CURRENT_SCHEMA_VERSION),
        patch_level: get_meta(conn, "patch_level")?.unwrap_or_else(|| patching::PATCH_ID.into()),
        initialized_at: get_meta(conn, "initialized_at")?.unwrap_or_else(now_iso),
    })
}

```

## `files/src-tauri/src/core/migrations/002_multi_business_workspace.sql`

```sql
CREATE TABLE IF NOT EXISTS tax_profiles (
  id TEXT PRIMARY KEY,
  business_id TEXT NOT NULL,
  name TEXT NOT NULL,
  label TEXT NOT NULL,
  rate REAL NOT NULL DEFAULT 0,
  is_default INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS receipt_profiles (
  id TEXT PRIMARY KEY,
  business_id TEXT NOT NULL,
  name TEXT NOT NULL,
  header_line1 TEXT,
  header_line2 TEXT,
  footer_text TEXT,
  show_address INTEGER NOT NULL DEFAULT 1,
  show_phone INTEGER NOT NULL DEFAULT 1,
  show_tax_breakdown INTEGER NOT NULL DEFAULT 1,
  paper_width TEXT NOT NULL DEFAULT '80mm',
  copies INTEGER NOT NULL DEFAULT 1,
  is_default INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS module_flags (
  business_id TEXT NOT NULL,
  module_key TEXT NOT NULL,
  enabled INTEGER NOT NULL DEFAULT 0,
  updated_at TEXT NOT NULL,
  PRIMARY KEY (business_id, module_key),
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sequence_counters (
  business_id TEXT NOT NULL,
  counter_key TEXT NOT NULL,
  prefix TEXT NOT NULL,
  next_number INTEGER NOT NULL DEFAULT 1,
  padding INTEGER NOT NULL DEFAULT 4,
  updated_at TEXT NOT NULL,
  PRIMARY KEY (business_id, counter_key),
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_tax_profiles_business_default
  ON tax_profiles(business_id, is_default DESC, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_receipt_profiles_business_default
  ON receipt_profiles(business_id, is_default DESC, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_module_flags_business
  ON module_flags(business_id, module_key);

CREATE INDEX IF NOT EXISTS idx_sequence_counters_business
  ON sequence_counters(business_id, counter_key);

```

## `files/src-tauri/src/core/migrations.rs`

```rust
use rusqlite::{Connection, OptionalExtension};

pub const CURRENT_SCHEMA_VERSION: i64 = 2;

const MIGRATION_001: &str = include_str!("migrations/001_base.sql");
const MIGRATION_002: &str = include_str!("migrations/002_multi_business_workspace.sql");

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

    conn.pragma_update(None, "user_version", current_version)?;
    Ok(())
}

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

const PATCHES: [PatchDefinition; 2] = [
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
];

pub const PATCH_ID: &str = "P002_multi_business_workspace_settings_core";
pub const PATCH_NAME: &str = "Multi-Business Workspace & Settings Core";

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
use rusqlite::{params, Connection, OptionalExtension};
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

fn bool_to_i64(value: bool) -> i64 {
    if value { 1 } else { 0 }
}

fn load_legacy_settings(
    conn: &Connection,
    business_id: &str,
) -> rusqlite::Result<(
    String,
    f64,
    Option<String>,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
)> {
    conn.query_row(
        "SELECT
            tax_label,
            default_tax_rate,
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
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get::<_, i64>(3)? != 0,
                row.get::<_, i64>(4)? != 0,
                row.get::<_, i64>(5)? != 0,
                row.get::<_, i64>(6)? != 0,
                row.get::<_, i64>(7)? != 0,
                row.get::<_, i64>(8)? != 0,
            ))
        },
    )
}

pub fn ensure_workspace_support_for_business(
    conn: &Connection,
    business_id: &str,
) -> rusqlite::Result<()> {
    let (
        tax_label,
        default_tax_rate,
        receipt_footer,
        receipt_show_address,
        receipt_show_phone,
        module_restaurant_enabled,
        module_retail_enabled,
        module_inventory_enabled,
        module_services_enabled,
    ) = load_legacy_settings(conn, business_id)?;

    let now = Utc::now().to_rfc3339();

    let tax_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM tax_profiles WHERE business_id = ?1",
        params![business_id],
        |row| row.get(0),
    )?;
    if tax_count == 0 {
        conn.execute(
            "INSERT INTO tax_profiles (id, business_id, name, label, rate, is_default, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                Uuid::new_v4().to_string(),
                business_id,
                "Standard tax",
                tax_label,
                default_tax_rate,
                1_i64,
                now.clone(),
                now.clone()
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
                show_address, show_phone, show_tax_breakdown, paper_width,
                copies, is_default, created_at, updated_at
             ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6,
                ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14
             )",
            params![
                Uuid::new_v4().to_string(),
                business_id,
                "Default receipt",
                Option::<String>::None,
                Option::<String>::None,
                receipt_footer,
                bool_to_i64(receipt_show_address),
                bool_to_i64(receipt_show_phone),
                1_i64,
                "80mm",
                1_i64,
                1_i64,
                now.clone(),
                now.clone()
            ],
        )?;
    }

    for (module_key, enabled) in [
        ("restaurant", module_restaurant_enabled),
        ("retail", module_retail_enabled),
        ("inventory", module_inventory_enabled),
        ("services", module_services_enabled),
    ] {
        conn.execute(
            "INSERT OR IGNORE INTO module_flags (business_id, module_key, enabled, updated_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![business_id, module_key, bool_to_i64(enabled), now.clone()],
        )?;
    }

    for (counter_key, prefix, padding) in [
        ("sale", "SAL-", 5_i64),
        ("purchase", "PUR-", 5_i64),
        ("expense", "EXP-", 5_i64),
        ("customer", "CUS-", 4_i64),
        ("supplier", "SUP-", 4_i64),
    ] {
        conn.execute(
            "INSERT OR IGNORE INTO sequence_counters (business_id, counter_key, prefix, next_number, padding, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![business_id, counter_key, prefix, 1_i64, padding, now.clone()],
        )?;
    }

    Ok(())
}

pub fn copy_workspace_preferences_from_template(
    conn: &Connection,
    template_business_id: &str,
    target_business_id: &str,
) -> rusqlite::Result<()> {
    let now = Utc::now().to_rfc3339();

    let template_tax = conn
        .query_row(
            "SELECT name, label, rate
             FROM tax_profiles
             WHERE business_id = ?1
             ORDER BY is_default DESC, updated_at DESC
             LIMIT 1",
            params![template_business_id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, f64>(2)?,
                ))
            },
        )
        .optional()?;

    if let Some((name, label, rate)) = template_tax {
        conn.execute(
            "UPDATE tax_profiles
             SET name = ?2, label = ?3, rate = ?4, is_default = 1, updated_at = ?5
             WHERE business_id = ?1 AND is_default = 1",
            params![target_business_id, name, label, rate, now.clone()],
        )?;

        conn.execute(
            "UPDATE business_settings
             SET tax_label = ?2, default_tax_rate = ?3, updated_at = ?4
             WHERE business_id = ?1",
            params![target_business_id, label, rate, now.clone()],
        )?;
    }

    let template_receipt = conn
        .query_row(
            "SELECT name, header_line1, header_line2, footer_text, show_address, show_phone, show_tax_breakdown, paper_width, copies
             FROM receipt_profiles
             WHERE business_id = ?1
             ORDER BY is_default DESC, updated_at DESC
             LIMIT 1",
            params![template_business_id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, Option<String>>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, Option<String>>(3)?,
                    row.get::<_, i64>(4)? != 0,
                    row.get::<_, i64>(5)? != 0,
                    row.get::<_, i64>(6)? != 0,
                    row.get::<_, String>(7)?,
                    row.get::<_, i64>(8)?,
                ))
            },
        )
        .optional()?;

    if let Some((name, header_line1, header_line2, footer_text, show_address, show_phone, show_tax_breakdown, paper_width, copies)) = template_receipt {
        conn.execute(
            "UPDATE receipt_profiles
             SET name = ?2,
                 header_line1 = ?3,
                 header_line2 = ?4,
                 footer_text = ?5,
                 show_address = ?6,
                 show_phone = ?7,
                 show_tax_breakdown = ?8,
                 paper_width = ?9,
                 copies = ?10,
                 is_default = 1,
                 updated_at = ?11
             WHERE business_id = ?1 AND is_default = 1",
            params![
                target_business_id,
                name,
                header_line1,
                header_line2,
                footer_text.clone(),
                bool_to_i64(show_address),
                bool_to_i64(show_phone),
                bool_to_i64(show_tax_breakdown),
                paper_width,
                copies,
                now.clone()
            ],
        )?;

        conn.execute(
            "UPDATE business_settings
             SET receipt_footer = ?2,
                 receipt_show_address = ?3,
                 receipt_show_phone = ?4,
                 updated_at = ?5
             WHERE business_id = ?1",
            params![
                target_business_id,
                footer_text,
                bool_to_i64(show_address),
                bool_to_i64(show_phone),
                now.clone()
            ],
        )?;
    }

    let template_modules = conn.prepare(
        "SELECT module_key, enabled FROM module_flags WHERE business_id = ?1 ORDER BY module_key ASC",
    )?;
    let mut rows = template_modules.query(params![template_business_id])?;
    let mut restaurant = false;
    let mut retail = false;
    let mut inventory = false;
    let mut services = false;

    while let Some(row) = rows.next()? {
        let module_key: String = row.get(0)?;
        let enabled = row.get::<_, i64>(1)? != 0;
        conn.execute(
            "INSERT INTO module_flags (business_id, module_key, enabled, updated_at)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(business_id, module_key)
             DO UPDATE SET enabled = excluded.enabled, updated_at = excluded.updated_at",
            params![target_business_id, module_key, bool_to_i64(enabled), now.clone()],
        )?;

        match module_key.as_str() {
            "restaurant" => restaurant = enabled,
            "retail" => retail = enabled,
            "inventory" => inventory = enabled,
            "services" => services = enabled,
            _ => {}
        }
    }

    conn.execute(
        "UPDATE business_settings
         SET module_restaurant_enabled = ?2,
             module_retail_enabled = ?3,
             module_inventory_enabled = ?4,
             module_services_enabled = ?5,
             updated_at = ?6
         WHERE business_id = ?1",
        params![
            target_business_id,
            bool_to_i64(restaurant),
            bool_to_i64(retail),
            bool_to_i64(inventory),
            bool_to_i64(services),
            now.clone()
        ],
    )?;

    let template_counters = conn.prepare(
        "SELECT counter_key, prefix, padding FROM sequence_counters WHERE business_id = ?1 ORDER BY counter_key ASC",
    )?;
    let mut counter_rows = template_counters.query(params![template_business_id])?;
    while let Some(row) = counter_rows.next()? {
        let counter_key: String = row.get(0)?;
        let prefix: String = row.get(1)?;
        let padding: i64 = row.get(2)?;
        conn.execute(
            "INSERT INTO sequence_counters (business_id, counter_key, prefix, next_number, padding, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(business_id, counter_key)
             DO UPDATE SET prefix = excluded.prefix, next_number = excluded.next_number, padding = excluded.padding, updated_at = excluded.updated_at",
            params![target_business_id, counter_key, prefix, 1_i64, padding, now.clone()],
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

pub fn seed_if_empty(conn: &Connection) -> rusqlite::Result<()> {
    let business_count: i64 = conn.query_row("SELECT COUNT(*) FROM businesses", [], |row| row.get(0))?;

    upsert_meta(conn, "app_name", "local-first-business-manager")?;
    upsert_meta(conn, "product_name", "Local Business Manager")?;
    upsert_meta(conn, "app_version", "0.2.0")?;
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
            ("INFO", "patching", "Patch 2 multi-business workspace registered"),
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

    ensure_workspace_support_foundation(conn)?;
    Ok(())
}

```

## `files/src-tauri/src/domain/bootstrap.rs`

```rust
use super::models::{
    AppBootstrap, AppInfo, BackupRecord, BusinessProfile, BusinessSettings,
    BusinessWorkspaceSummary, DashboardShellData, KpiCard, ModuleFlags, ModuleStatus,
    PatchRecord, ReceiptProfile, RecentActivity, SequenceCounter, StorageStatus, TaxProfile,
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
) -> DashboardShellData {
    DashboardShellData {
        hero_title: "Multi-business local workspace ready".into(),
        hero_body: format!(
            "Patch 2 upgrades the local foundation with isolated business workspaces, active business switching, per-business settings, tax/receipt profiles, module flags, and sequence counters. {} is currently active.",
            active_business.name
        ),
        kpis: vec![
            KpiCard {
                id: "workspace-count".into(),
                label: "Business workspaces".into(),
                value: business_workspaces.len().to_string(),
                note: "Every business keeps its own profile, settings, and future module data boundaries.".into(),
            },
            KpiCard {
                id: "module-count".into(),
                label: "Active modules".into(),
                value: active_module_count(active_modules).to_string(),
                note: "Module flags are now tracked per business for later POS, inventory, and services patches.".into(),
            },
            KpiCard {
                id: "sequence-count".into(),
                label: "Sequence counters".into(),
                value: active_sequences.len().to_string(),
                note: "Document numbering foundations now exist before sales and purchasing modules arrive.".into(),
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
                note: "Patch history remains local so future upgrades can stay incremental and traceable.".into(),
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
                note: "Business switching and per-business settings are now stored locally and isolated.".into(),
            },
            ModuleStatus {
                id: "catalog".into(),
                label: "Catalog / items".into(),
                status: "coming-next".into(),
                note: "The next product patch can attach item data safely to the active business.".into(),
            },
            ModuleStatus {
                id: "pos".into(),
                label: "POS / billing".into(),
                status: "planned".into(),
                note: "Sequences, receipt settings, and module flags are ready for future billing workflows.".into(),
            },
            ModuleStatus {
                id: "inventory".into(),
                label: "Inventory".into(),
                status: "planned".into(),
                note: "Per-business isolation is in place before stock tables and movement ledgers arrive.".into(),
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
    pub active_tax_profile: TaxProfile,
    pub active_receipt_profile: ReceiptProfile,
    pub active_module_flags: ModuleFlags,
    pub active_sequences: Vec<SequenceCounter>,
    pub businesses: Vec<BusinessProfile>,
    pub business_workspaces: Vec<BusinessWorkspaceSummary>,
    pub patch_history: Vec<PatchRecord>,
    pub backups: Vec<BackupRecord>,
    pub storage: StorageStatus,
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
  "version": "0.2.0",
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
# Migration Notes - Patch 2

Patch 2 adds new tables only and does not drop or rename Patch 1 tables.

## New tables
- `tax_profiles`
- `receipt_profiles`
- `sequence_counters`

## Backfill behavior on first app start after applying Patch 2
For every existing business row:
- create one default tax profile if missing
- create one default receipt profile if missing
- seed sequence counters for `sale`, `purchase`, `customer`, and `supplier` if missing
- preserve the current `active_business_id` where possible

## Compatibility
Patch 2 expects a project already initialized from Patch 1.

```

## `patch-manifest.json`

```json
{
  "patch_id": "P002_multi_business_workspace_settings_core",
  "patch_name": "Multi-Business Workspace & Settings Core",
  "base_version": "0.1.0",
  "target_version": "0.2.0-workspace-core",
  "description": "Adds multi-business creation, switching, archive flow, per-business settings isolation, default tax and receipt profile foundations, and sequence counters on top of Patch 1.",
  "dependencies": [
    "P001_foundation_base_structure"
  ],
  "safe_on_empty_project": false,
  "migration_required": true,
  "rollback_supported": true,
  "files_root": "files",
  "files_added": [
    "scripts/validate-patch2.mjs",
    "src-tauri/src/core/migrations/002_multi_business_workspace.sql"
  ],
  "files_updated": [
    "package.json",
    "src/app/AppProvider.tsx",
    "src/modules/business/BusinessPage.tsx",
    "src/modules/dashboard/DashboardPage.tsx",
    "src/modules/data-center/DataCenterPage.tsx",
    "src/modules/settings/SettingsPage.tsx",
    "src/modules/shell/AppShell.tsx",
    "src/shared/api.ts",
    "src/shared/types.ts",
    "src/shared/utils.ts",
    "src/styles.css",
    "src-tauri/Cargo.toml",
    "src-tauri/tauri.conf.json",
    "src-tauri/src/commands/bootstrap.rs",
    "src-tauri/src/commands/business.rs",
    "src-tauri/src/commands/data_center.rs",
    "src-tauri/src/commands/settings.rs",
    "src-tauri/src/core/db.rs",
    "src-tauri/src/core/migrations.rs",
    "src-tauri/src/core/patching.rs",
    "src-tauri/src/core/seed.rs",
    "src-tauri/src/domain/bootstrap.rs",
    "src-tauri/src/domain/models.rs",
    "src-tauri/src/lib.rs"
  ],
  "post_apply_steps": [
    "npm install",
    "npm run validate:patch2",
    "npm run tauri dev"
  ]
}

```

## `rollback.md`

```md
# Rollback Notes

## Safe rollback options
1. Restore from version control if the target project is tracked.
2. Restore files from `.patch-backups/P002_multi_business_workspace_settings_core/` created by the patch applier.
3. Restore the SQLite database from a pre-patch backup snapshot if runtime data must also be reverted.

## Notes
- Patch 2 introduces new tables only; it does not remove Patch 1 tables.
- Source rollback does not automatically delete rows created in the local database.
- If you need both code and data rollback, restore both the source tree and the SQLite file.

```

## `validate.md`

```md
# Validation Checklist

## Structural checks completed in this bundle
- [x] `apply_patch.mjs` syntax check passed
- [x] patch manifest JSON parsed successfully
- [x] Tauri config JSON parsed successfully
- [x] `node scripts/validate-patch2.mjs` passed against the patched project tree
- [x] TypeScript structural check passed with local validation stubs
- [ ] Full Rust compile was not executed in this container because the Rust toolchain is unavailable here
- [ ] Full `npm install` and desktop launch were not executed in this container

## Manual validation after apply
- [ ] run `npm install`
- [ ] run `npm run validate:patch2`
- [ ] run `npm run tauri dev`
- [ ] verify dashboard shows Patch 2 workspace messaging
- [ ] create a second business workspace
- [ ] switch active business and confirm settings change with the active workspace
- [ ] archive a non-last active business
- [ ] save default tax profile
- [ ] save default receipt profile
- [ ] save sequence counters
- [ ] export a workspace snapshot and confirm the JSON includes `taxProfiles`, `receiptProfiles`, and `sequenceCounters`

```

