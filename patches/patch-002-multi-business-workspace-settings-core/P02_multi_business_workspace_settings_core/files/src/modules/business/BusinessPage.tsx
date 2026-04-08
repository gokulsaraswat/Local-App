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
