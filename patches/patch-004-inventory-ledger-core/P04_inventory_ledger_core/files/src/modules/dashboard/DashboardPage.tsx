import { useAppState } from "../../app/AppProvider";
import type { NavPage } from "../../shared/types";
import {
  formatDateTime,
  formatModuleList,
  formatQuantity,
  titleCaseWords
} from "../../shared/utils";

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
          <div className="section-kicker">Patch 4 inventory ledger core</div>
          <h2>{data.dashboard.heroTitle}</h2>
          <p>{data.dashboard.heroBody}</p>
        </div>
        <div className="hero-actions">
          <button
            className="primary-button"
            type="button"
            onClick={() => onNavigate("inventory")}
          >
            Open Inventory
          </button>
          <button
            className="secondary-button"
            type="button"
            onClick={() => onNavigate("catalog")}
          >
            Open Catalog
          </button>
          <button
            className="secondary-button"
            type="button"
            onClick={() => onNavigate("business")}
          >
            Manage Businesses
          </button>
        </div>
      </section>

      <section className="card-grid card-grid-4">
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
            <h3>Inventory snapshot</h3>
            <span className="pill success">
              {data.inventorySummary.totalTrackedItems} tracked
            </span>
          </div>
          <div className="detail-list">
            <div>
              <span>Tracked items</span>
              <code>{data.inventorySummary.totalTrackedItems}</code>
            </div>
            <div>
              <span>Low stock items</span>
              <code>{data.inventorySummary.lowStockItems}</code>
            </div>
            <div>
              <span>Movement rows</span>
              <code>{data.inventorySummary.movementCount}</code>
            </div>
            <div>
              <span>Quantity on hand</span>
              <code>{formatQuantity(data.inventorySummary.totalQuantityOnHand)}</code>
            </div>
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
              <code>
                {formatModuleList(
                  data.businessWorkspaces.find(
                    (workspace) => workspace.businessId === data.activeBusiness.id
                  )?.activeModules ?? []
                )}
              </code>
            </div>
          </div>
        </article>
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
      </section>

      <section className="card-grid card-grid-4">
        {data.dashboard.moduleStatuses.map((status) => (
          <article className="card" key={status.id}>
            <div className="card-header compact-card-header">
              <h3>{status.label}</h3>
              <span className={`pill ${status.status === "active-foundation" ? "success" : status.status === "coming-next" ? "warning" : "neutral"}`}>
                {status.status}
              </span>
            </div>
            <p className="card-note">{status.note}</p>
          </article>
        ))}
      </section>
    </div>
  );
}
