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
