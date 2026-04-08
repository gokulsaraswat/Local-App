import { useEffect, useMemo, useState } from "react";
import { useAppState } from "../../app/AppProvider";
import type { NavPage } from "../../shared/types";
import { classNames, formatModuleList } from "../../shared/utils";
import { BusinessPage } from "../business/BusinessPage";
import { CatalogPage } from "../catalog/CatalogPage";
import { DashboardPage } from "../dashboard/DashboardPage";
import { DataCenterPage } from "../data-center/DataCenterPage";
import { InventoryPage } from "../inventory/InventoryPage";
import { SettingsPage } from "../settings/SettingsPage";

const NAV_STORAGE_KEY = "lfbm.activeNavPage";

const navItems: Array<{ key: NavPage; label: string; description: string }> = [
  {
    key: "dashboard",
    label: "Dashboard",
    description: "Local workspace summary"
  },
  {
    key: "catalog",
    label: "Catalog",
    description: "Products, menu, services"
  },
  {
    key: "inventory",
    label: "Inventory",
    description: "Stock ledger and balances"
  },
  {
    key: "business",
    label: "Businesses",
    description: "Profiles and switching"
  },
  {
    key: "settings",
    label: "Settings",
    description: "Tax, receipt, modules"
  },
  {
    key: "data-center",
    label: "Data Center",
    description: "Backup and transfer foundation"
  }
];

export function AppShell() {
  const { data, switchBusiness } = useAppState();
  const [activePage, setActivePage] = useState<NavPage>(() => {
    const stored = window.localStorage.getItem(NAV_STORAGE_KEY) as NavPage | null;
    return stored ?? "dashboard";
  });
  const [switchStatus, setSwitchStatus] = useState<string>("");

  useEffect(() => {
    window.localStorage.setItem(NAV_STORAGE_KEY, activePage);
  }, [activePage]);

  useEffect(() => {
    setSwitchStatus("");
  }, [data?.activeBusiness.id]);

  const pageTitle = useMemo(() => {
    return navItems.find((item) => item.key === activePage)?.label ?? "Dashboard";
  }, [activePage]);

  const activeWorkspace = useMemo(() => {
    return data?.businessWorkspaces.find(
      (workspace) => workspace.businessId === data.activeBusiness.id
    );
  }, [data]);

  if (!data) {
    return null;
  }

  async function handleBusinessSwitch(nextBusinessId: string) {
    if (nextBusinessId === data.activeBusiness.id) {
      return;
    }

    setSwitchStatus("Switching active business…");
    try {
      const switched = await switchBusiness(nextBusinessId);
      setSwitchStatus(`Switched to ${switched.name}.`);
    } catch (error) {
      setSwitchStatus(
        error instanceof Error ? error.message : "Failed to switch business."
      );
    }
  }

  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="sidebar-brand">
          <div className="brand-badge">P4</div>
          <div>
            <strong>Local Business Manager</strong>
            <div className="muted-text">
              {data.appInfo.version} · {data.appInfo.patchLevel}
            </div>
          </div>
        </div>

        <div className="sidebar-section-label">Workspace</div>
        <nav className="nav-list">
          {navItems.map((item) => (
            <button
              key={item.key}
              type="button"
              className={classNames(
                "nav-item",
                activePage === item.key && "nav-item-active"
              )}
              onClick={() => setActivePage(item.key)}
            >
              <span className="nav-title">{item.label}</span>
              <span className="nav-description">{item.description}</span>
            </button>
          ))}
        </nav>

        <div className="sidebar-section-label">Active business</div>
        <div className="sidebar-card">
          <div className="sidebar-card-title">{data.activeBusiness.name}</div>
          <div className="muted-text">
            {data.activeBusiness.businessType} · {data.activeBusiness.currencyCode}
          </div>
          <label className="sidebar-field">
            <span>Switch workspace</span>
            <select
              className="sidebar-select"
              value={data.activeBusiness.id}
              onChange={(event) => void handleBusinessSwitch(event.target.value)}
            >
              {data.businessWorkspaces.map((workspace) => (
                <option key={workspace.businessId} value={workspace.businessId}>
                  {workspace.name} ({workspace.code})
                </option>
              ))}
            </select>
          </label>
          <div className="sidebar-metadata-grid">
            <div>
              <span>Timezone</span>
              <strong>{data.businessSettings.timezone}</strong>
            </div>
            <div>
              <span>Tax</span>
              <strong>
                {data.activeTaxProfile.taxLabel} · {data.activeTaxProfile.defaultRate}%
              </strong>
            </div>
            <div>
              <span>Catalog items</span>
              <strong>{data.catalogSummary.activeItems}</strong>
            </div>
            <div>
              <span>Low stock</span>
              <strong>{data.inventorySummary.lowStockItems}</strong>
            </div>
            <div>
              <span>Tracked items</span>
              <strong>{data.inventorySummary.totalTrackedItems}</strong>
            </div>
            <div>
              <span>Movements</span>
              <strong>{data.inventorySummary.movementCount}</strong>
            </div>
          </div>
          <div className="muted-text small-text">
            {switchStatus || activeWorkspace?.nextSaleSequence || "Sequence pending"}
          </div>
        </div>

        <div className="sidebar-section-label">Business mode</div>
        <div className="sidebar-card">
          <div className="sidebar-pill success">Inventory ledger ready</div>
          <div className="sidebar-pill neutral">Catalog core ready</div>
          <div className="sidebar-pill neutral">Multi-business ready</div>
          <div className="muted-text small-text">
            {formatModuleList(activeWorkspace?.activeModules ?? [])}
          </div>
        </div>
      </aside>

      <main className="workspace">
        <header className="workspace-header">
          <div>
            <h1>{pageTitle}</h1>
            <p>
              Patch 4 links catalog stock items to a real local inventory ledger with
              stock movements, reorder thresholds, recent adjustments, and exportable
              inventory history, while still staying fully offline.
            </p>
          </div>
          <div className="workspace-header-meta">
            <span className="meta-chip">Business: {data.activeBusiness.code}</span>
            <span className="meta-chip">
              Tracked stock: {data.inventorySummary.totalTrackedItems}
            </span>
            <span className="meta-chip">
              Movements: {data.inventorySummary.movementCount}
            </span>
            <span className="meta-chip">Schema v{data.appInfo.schemaVersion}</span>
          </div>
        </header>

        <section className="workspace-content">
          {activePage === "dashboard" && <DashboardPage onNavigate={setActivePage} />}
          {activePage === "catalog" && <CatalogPage />}
          {activePage === "inventory" && <InventoryPage />}
          {activePage === "business" && <BusinessPage />}
          {activePage === "settings" && <SettingsPage />}
          {activePage === "data-center" && <DataCenterPage />}
        </section>
      </main>
    </div>
  );
}
