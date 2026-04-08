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
