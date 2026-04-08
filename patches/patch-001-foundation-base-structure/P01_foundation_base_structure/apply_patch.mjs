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
