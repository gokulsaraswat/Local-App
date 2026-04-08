use super::models::{
    AppBootstrap, AppInfo, BackupRecord, BusinessProfile, BusinessSettings, DashboardShellData,
    KpiCard, ModuleStatus, PatchRecord, RecentActivity, StorageStatus,
};

pub fn compose_dashboard(
    businesses: &[BusinessProfile],
    backups: &[BackupRecord],
    recent_activity: Vec<RecentActivity>,
    patch_history: &[PatchRecord],
    storage: &StorageStatus,
) -> DashboardShellData {
    DashboardShellData {
        hero_title: "Local-first foundation ready".into(),
        hero_body: "Patch 1 establishes the desktop shell, local database, business profile, settings, backup/export foundations, and a patch registry for future modules.".into(),
        kpis: vec![
            KpiCard {
                id: "business-count".into(),
                label: "Businesses in storage".into(),
                value: businesses.len().to_string(),
                note: "Patch 1 uses a single active business but stores data in a future-ready table.".into(),
            },
            KpiCard {
                id: "backup-count".into(),
                label: "Backup snapshots".into(),
                value: backups.len().to_string(),
                note: "Snapshots are stored locally and tracked in SQLite.".into(),
            },
            KpiCard {
                id: "patch-count".into(),
                label: "Applied patches".into(),
                value: patch_history.len().to_string(),
                note: "Patch history is stored locally to support incremental evolution.".into(),
            },
            KpiCard {
                id: "export-count".into(),
                label: "Exports recorded".into(),
                value: storage.export_count.to_string(),
                note: "Data portability foundation is active from Patch 1.".into(),
            },
        ],
        recent_activity,
        module_statuses: vec![
            ModuleStatus {
                id: "core".into(),
                label: "Foundation shell".into(),
                status: "active-foundation".into(),
                note: "Desktop shell, navigation, settings, and storage are already present.".into(),
            },
            ModuleStatus {
                id: "catalog".into(),
                label: "Catalog / items".into(),
                status: "coming-next".into(),
                note: "Planned as a follow-on patch from this database and navigation base.".into(),
            },
            ModuleStatus {
                id: "pos".into(),
                label: "POS / billing".into(),
                status: "planned".into(),
                note: "Checkout flow intentionally deferred to a later patch.".into(),
            },
            ModuleStatus {
                id: "inventory".into(),
                label: "Inventory".into(),
                status: "planned".into(),
                note: "Stock ledger and movement history are not part of Patch 1 yet.".into(),
            },
            ModuleStatus {
                id: "reports".into(),
                label: "Reports".into(),
                status: "planned".into(),
                note: "Reporting surfaces will build on top of the foundation schema later.".into(),
            },
        ],
    }
}

#[allow(clippy::too_many_arguments)]
pub fn build_app_bootstrap(
    app_info: AppInfo,
    active_business: BusinessProfile,
    business_settings: BusinessSettings,
    businesses: Vec<BusinessProfile>,
    patch_history: Vec<PatchRecord>,
    backups: Vec<BackupRecord>,
    storage: StorageStatus,
    recent_activity: Vec<RecentActivity>,
) -> AppBootstrap {
    let dashboard = compose_dashboard(
        &businesses,
        &backups,
        recent_activity,
        &patch_history,
        &storage,
    );

    AppBootstrap {
        app_info,
        active_business,
        business_settings,
        businesses,
        patch_history,
        backups,
        storage,
        dashboard,
    }
}
