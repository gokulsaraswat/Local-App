import { useEffect, useMemo, useState } from "react";
import { useAppState } from "../../app/AppProvider";
import type { NavPage } from "../../shared/types";
import { classNames } from "../../shared/utils";
import { DashboardPage } from "../dashboard/DashboardPage";
import { BusinessPage } from "../business/BusinessPage";
import { SettingsPage } from "../settings/SettingsPage";
import { DataCenterPage } from "../data-center/DataCenterPage";

const NAV_STORAGE_KEY = "lfbm.activeNavPage";

const navItems: Array<{ key: NavPage; label: string; description: string }> = [
  {
    key: "dashboard",
    label: "Dashboard",
    description: "Local workspace summary"
  },
  {
    key: "business",
    label: "Business",
    description: "Profile and identity"
  },
  {
    key: "settings",
    label: "Settings",
    description: "Defaults and module toggles"
  },
  {
    key: "data-center",
    label: "Data Center",
    description: "Backup and transfer foundation"
  }
];

export function AppShell() {
  const { data } = useAppState();
  const [activePage, setActivePage] = useState<NavPage>(() => {
    const stored = window.localStorage.getItem(NAV_STORAGE_KEY) as NavPage | null;
    return stored ?? "dashboard";
  });

  useEffect(() => {
    window.localStorage.setItem(NAV_STORAGE_KEY, activePage);
  }, [activePage]);

  const pageTitle = useMemo(() => {
    return navItems.find((item) => item.key === activePage)?.label ?? "Dashboard";
  }, [activePage]);

  if (!data) {
    return null;
  }

  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="sidebar-brand">
          <div className="brand-badge">P1</div>
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
          <div className="muted-text">{data.activeBusiness.businessType}</div>
          <div className="muted-text">{data.activeBusiness.currencyCode}</div>
        </div>

        <div className="sidebar-section-label">Foundation status</div>
        <div className="sidebar-card">
          <div className="sidebar-pill success">Local-first</div>
          <div className="sidebar-pill neutral">SQLite ready</div>
          <div className="sidebar-pill neutral">Patch registry ready</div>
        </div>
      </aside>

      <main className="workspace">
        <header className="workspace-header">
          <div>
            <h1>{pageTitle}</h1>
            <p>
              Foundation shell for a local-first business desktop application.
            </p>
          </div>
          <div className="workspace-header-meta">
            <span className="meta-chip">Business: {data.activeBusiness.code}</span>
            <span className="meta-chip">
              Schema v{data.appInfo.schemaVersion}
            </span>
          </div>
        </header>

        <section className="workspace-content">
          {activePage === "dashboard" && (
            <DashboardPage onNavigate={setActivePage} />
          )}
          {activePage === "business" && <BusinessPage />}
          {activePage === "settings" && <SettingsPage />}
          {activePage === "data-center" && <DataCenterPage />}
        </section>
      </main>
    </div>
  );
}
