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
