import { useAppState } from "../../app/AppProvider";
import type { NavPage } from "../../shared/types";
import { formatDateTime } from "../../shared/utils";

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
          <div className="section-kicker">Patch 1 foundation</div>
          <h2>{data.dashboard.heroTitle}</h2>
          <p>{data.dashboard.heroBody}</p>
        </div>
        <div className="hero-actions">
          <button
            className="primary-button"
            type="button"
            onClick={() => onNavigate("business")}
          >
            Review Business Profile
          </button>
          <button
            className="secondary-button"
            type="button"
            onClick={() => onNavigate("data-center")}
          >
            Open Data Center
          </button>
        </div>
      </section>

      <section className="card-grid">
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
            <h3>Recent local activity</h3>
            <span className="pill neutral">Demo seeded</span>
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
            <h3>Module roadmap from this foundation</h3>
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
