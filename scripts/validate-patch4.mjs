import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
let ts = null;
try {
  ts = require("typescript");
} catch {
  ts = null;
}

const requiredFiles = [
  "package.json",
  "src/shared/types.ts",
  "src/shared/api.ts",
  "src/modules/inventory/InventoryPage.tsx",
  "src/modules/shell/AppShell.tsx",
  "src-tauri/src/core/inventory.rs",
  "src-tauri/src/core/migrations/004_inventory_ledger_core.sql",
  "src-tauri/src/commands/inventory.rs",
  "src-tauri/src/lib.rs"
];

const requiredSnippets = [
  ["package.json", '"validate:patch4": "node scripts/validate-patch4.mjs"'],
  ["src/shared/types.ts", "export interface InventoryWorkspace"],
  ["src/shared/types.ts", '  | "inventory"'],
  ["src/shared/api.ts", 'invoke<InventoryWorkspace>("load_inventory_workspace")'],
  ["src/modules/inventory/InventoryPage.tsx", "Patch 4 inventory ledger core"],
  ["src/modules/shell/AppShell.tsx", 'key: "inventory"'],
  ["src-tauri/src/core/migrations.rs", "CURRENT_SCHEMA_VERSION: i64 = 4"],
  ["src-tauri/src/core/patching.rs", "P004_inventory_ledger_core"],
  ["src-tauri/src/lib.rs", "commands::inventory::load_inventory_workspace"],
  ["src-tauri/src/lib.rs", "commands::inventory::record_inventory_movement"],
  ["src-tauri/src/lib.rs", "commands::inventory::save_inventory_stock_rule"]
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

for (const relativePath of ["package.json", "src-tauri/tauri.conf.json"]) {
  try {
    JSON.parse(fs.readFileSync(path.resolve(relativePath), "utf8"));
  } catch (error) {
    hasError = true;
    console.error(`[ERROR] Invalid JSON in ${relativePath}: ${error instanceof Error ? error.message : String(error)}`);
  }
}

function walk(dir) {
  const output = [];
  if (!fs.existsSync(dir)) return output;
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

if (ts) {
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
} else {
  console.warn("[WARN] typescript module not installed; skipping TS parse diagnostics.");
}

const migrationSql = fs.readFileSync(
  path.resolve("src-tauri/src/core/migrations/004_inventory_ledger_core.sql"),
  "utf8"
);
for (const token of [
  "inventory_stock_movements",
  "quantity_delta",
  "quantity_after",
  "opening-balance:",
  "opening_balance"
]) {
  if (!migrationSql.includes(token)) {
    console.error(`[ERROR] Migration 004 is missing expected token: ${token}`);
    hasError = true;
  }
}

if (hasError) {
  process.exit(1);
}

console.log("[OK] Patch 4 structural validation passed.");
