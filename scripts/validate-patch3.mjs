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
