import { useEffect, useMemo, useState } from "react";
import { useAppState } from "../../app/AppProvider";
import type { BusinessProfile } from "../../shared/types";
import { formatDateTime } from "../../shared/utils";

export function BusinessPage() {
  const { data, saveProfile } = useAppState();
  const [form, setForm] = useState<BusinessProfile | null>(null);
  const [statusMessage, setStatusMessage] = useState<string>("");

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

  function update<K extends keyof BusinessProfile>(key: K, value: BusinessProfile[K]) {
    setForm((current) => (current ? { ...current, [key]: value } : current));
  }

  async function handleSave() {
    setStatusMessage("Saving business profile…");
    try {
      await saveProfile(form);
      setStatusMessage("Business profile saved locally.");
    } catch (error) {
      setStatusMessage(
        error instanceof Error ? error.message : "Failed to save profile."
      );
    }
  }

  return (
    <div className="page-grid">
      <section className="split-grid">
        <article className="card">
          <div className="card-header">
            <h2>Business profile foundation</h2>
            <span className="pill success">{profileCompleteness}% complete</span>
          </div>
          <p className="card-note">
            This is the base identity record future catalog, POS, inventory, and
            reporting modules will attach to.
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
                onChange={(event) => update("code", event.target.value)}
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
              Save profile
            </button>
            <span className="muted-text">{statusMessage}</span>
          </div>
        </article>

        <article className="card">
          <div className="card-header">
            <h3>Foundation record metadata</h3>
          </div>
          <div className="detail-list">
            <div>
              <span>Business ID</span>
              <code>{form.id}</code>
            </div>
            <div>
              <span>Created</span>
              <code>{formatDateTime(form.createdAt)}</code>
            </div>
            <div>
              <span>Updated</span>
              <code>{formatDateTime(form.updatedAt)}</code>
            </div>
            <div>
              <span>Archived</span>
              <code>{formatDateTime(form.archivedAt)}</code>
            </div>
          </div>
        </article>
      </section>
    </div>
  );
}
