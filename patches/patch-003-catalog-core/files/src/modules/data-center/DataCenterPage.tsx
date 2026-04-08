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
