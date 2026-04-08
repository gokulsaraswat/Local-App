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
