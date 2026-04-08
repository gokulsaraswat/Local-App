import { useAppState } from "../../app/AppProvider";
import type { NavPage } from "../../shared/types";
import { formatDateTime, formatModuleList, titleCaseWords } from "../../shared/utils";

interface DashboardPageProps {
  onNavigate: (page: NavPage) => void;
}

export function DashboardPage({ onNavigate }: DashboardPageProps) {
  const { data } = useAppState();

  if (!data) return null;

  return (
    <div className="page-grid">
      <section className="hero-card">
        <div>
          <div className="section-kicker">Patch 3 catalog core</div>
          <h2>{data.dashboard.heroTitle}</h2>
          <p>{data.dashboard.heroBody}</p>
        </div>
        <div className="hero-actions">
          <button
            className="primary-button"
            type="button"
            onClick={() => onNavigate("business")}
          >
            Manage Businesses
          </button>
          <button
            className="secondary-button"
            type="button"
            onClick={() => onNavigate("catalog")}
          >
            Open Catalog
          </button>
        </div>
      </section>

      <section className="card-grid card-grid-5">
        {data.dashboard.kpis.map((kpi) => (
          <article className="card" key={kpi.id}>
            <div className="card-label">{kpi.label}</div>
            <div className="kpi-value">{kpi.value}</div>
            <p className="card-note">{kpi.note}</p>
          </article>
        ))}
      </section>

      <section className="split-grid">
        <article className="card">
          <div className="card-header">
            <h3>Business workspaces</h3>
            <span className="pill success">
              {data.businessWorkspaces.length} configured
            </span>
          </div>
          <div className="stack-list">
            {data.businessWorkspaces.map((workspace) => (
              <div className="list-row" key={workspace.businessId}>
                <div>
                  <strong>
                    {workspace.name}
                    {workspace.businessId === data.activeBusiness.id ? " · active" : ""}
                  </strong>
                  <div className="muted-text">
                    {workspace.code} · {workspace.currencyCode} · {workspace.timezone}
                  </div>
                  <div className="tag-list compact-tags">
                    {workspace.activeModules.map((module) => (
                      <span className="tag" key={`${workspace.businessId}-${module}`}>
                        {titleCaseWords(module)}
                      </span>
                    ))}
                  </div>
                </div>
                <span className="muted-text">{workspace.nextSaleSequence}</span>
              </div>
            ))}
          </div>
        </article>

        <article className="card">
          <div className="card-header">
            <h3>Current business configuration</h3>
            <span className="pill neutral">{data.activeBusiness.code}</span>
          </div>
          <div className="detail-list">
            <div>
              <span>Theme</span>
              <code>{data.businessSettings.theme}</code>
            </div>
            <div>
              <span>Tax profile</span>
              <code>
                {data.activeTaxProfile.name} · {data.activeTaxProfile.taxLabel} · {data.activeTaxProfile.defaultRate}%
              </code>
            </div>
            <div>
              <span>Receipt profile</span>
              <code>
                {data.activeReceiptProfile.name} · {data.activeReceiptProfile.paperWidth}
              </code>
            </div>
            <div>
              <span>Enabled modules</span>
              <code>{formatModuleList(data.businessWorkspaces.find((workspace) => workspace.businessId === data.activeBusiness.id)?.activeModules ?? [])}</code>
            </div>
          </div>
        </article>
      </section>

      <section className="split-grid">
        <article className="card">
          <div className="card-header">
            <h3>Recent local activity</h3>
            <span className="pill neutral">Local only</span>
          </div>
          <div className="stack-list">
            {data.dashboard.recentActivity.map((activity) => (
              <div className="list-row" key={activity.id}>
                <div>
                  <strong>{activity.message}</strong>
                  <div className="muted-text">
                    {activity.category} · {activity.level}
                  </div>
                </div>
                <span className="muted-text">
                  {formatDateTime(activity.createdAt)}
                </span>
              </div>
            ))}
          </div>
        </article>

        <article className="card">
          <div className="card-header">
            <h3>Module roadmap from this workspace</h3>
            <span className="pill success">Patch-ready</span>
          </div>
          <div className="stack-list">
            {data.dashboard.moduleStatuses.map((module) => (
              <div className="list-row" key={module.id}>
                <div>
                  <strong>{module.label}</strong>
                  <div className="muted-text">{module.note}</div>
                </div>
                <span
                  className={`pill ${
                    module.status === "active-foundation"
                      ? "success"
                      : module.status === "coming-next"
                      ? "warning"
                      : "neutral"
                  }`}
                >
                  {module.status.replace("-", " ")}
                </span>
              </div>
            ))}
          </div>
        </article>
      </section>

      <section className="split-grid">
        <article className="card">
          <div className="card-header">
            <h3>Storage foundation</h3>
          </div>
          <div className="detail-list">
            <div>
              <span>Database</span>
              <code>{data.storage.databasePath}</code>
            </div>
            <div>
              <span>Backups</span>
              <code>{data.storage.backupDir}</code>
            </div>
            <div>
              <span>Exports</span>
              <code>{data.storage.exportDir}</code>
            </div>
            <div>
              <span>Logs</span>
              <code>{data.storage.logDir}</code>
            </div>
          </div>
        </article>

        <article className="card">
          <div className="card-header">
            <h3>Patch history</h3>
          </div>
          <div className="stack-list">
            {data.patchHistory.map((patch) => (
              <div className="list-row" key={patch.patchId}>
                <div>
                  <strong>{patch.patchName}</strong>
                  <div className="muted-text">Schema v{patch.schemaVersion}</div>
                </div>
                <span className="muted-text">
                  {formatDateTime(patch.appliedAt)}
                </span>
              </div>
            ))}
          </div>
        </article>
      </section>
    </div>
  );
}
