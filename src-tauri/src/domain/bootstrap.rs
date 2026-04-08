use super::models::{
    AppBootstrap, AppInfo, BackupRecord, BusinessProfile, BusinessSettings,
    BusinessWorkspaceSummary, DashboardShellData, KpiCard, ModuleFlags, ModuleStatus,
    PatchRecord, ReceiptProfile, RecentActivity, SequenceCounter, StorageStatus, TaxProfile,
};

fn active_module_count(module_flags: &ModuleFlags) -> usize {
    [
        module_flags.restaurant_enabled,
        module_flags.retail_enabled,
        module_flags.inventory_enabled,
        module_flags.services_enabled,
        module_flags.customers_enabled,
        module_flags.suppliers_enabled,
        module_flags.expenses_enabled,
        module_flags.reporting_enabled,
        module_flags.data_center_enabled,
    ]
    .into_iter()
    .filter(|enabled| *enabled)
    .count()
}

pub fn compose_dashboard(
    business_workspaces: &[BusinessWorkspaceSummary],
    active_business: &BusinessProfile,
    active_modules: &ModuleFlags,
    active_sequences: &[SequenceCounter],
    backups: &[BackupRecord],
    recent_activity: Vec<RecentActivity>,
    patch_history: &[PatchRecord],
    storage: &StorageStatus,
) -> DashboardShellData {
    DashboardShellData {
        hero_title: "Multi-business local workspace ready".into(),
        hero_body: format!(
            "Patch 2 upgrades the local foundation with isolated business workspaces, active business switching, per-business settings, tax/receipt profiles, module flags, and sequence counters. {} is currently active.",
            active_business.name
        ),
        kpis: vec![
            KpiCard {
                id: "workspace-count".into(),
                label: "Business workspaces".into(),
                value: business_workspaces.len().to_string(),
                note: "Every business keeps its own profile, settings, and future module data boundaries.".into(),
            },
            KpiCard {
                id: "module-count".into(),
                label: "Active modules".into(),
                value: active_module_count(active_modules).to_string(),
                note: "Module flags are now tracked per business for later POS, inventory, and services patches.".into(),
            },
            KpiCard {
                id: "sequence-count".into(),
                label: "Sequence counters".into(),
                value: active_sequences.len().to_string(),
                note: "Document numbering foundations now exist before sales and purchasing modules arrive.".into(),
            },
            KpiCard {
                id: "backup-count".into(),
                label: "Backup snapshots".into(),
                value: backups.len().to_string(),
                note: format!("{} export job(s) are tracked in local storage.", storage.export_count),
            },
            KpiCard {
                id: "patch-count".into(),
                label: "Applied patches".into(),
                value: patch_history.len().to_string(),
                note: "Patch history remains local so future upgrades can stay incremental and traceable.".into(),
            },
        ],
        recent_activity,
        module_statuses: vec![
            ModuleStatus {
                id: "foundation".into(),
                label: "Foundation shell".into(),
                status: "active-foundation".into(),
                note: "Desktop shell, navigation, local storage, and backup/export foundations are active.".into(),
            },
            ModuleStatus {
                id: "workspace".into(),
                label: "Multi-business workspace".into(),
                status: "active-foundation".into(),
                note: "Business switching and per-business settings are now stored locally and isolated.".into(),
            },
            ModuleStatus {
                id: "catalog".into(),
                label: "Catalog / items".into(),
                status: "coming-next".into(),
                note: "The next product patch can attach item data safely to the active business.".into(),
            },
            ModuleStatus {
                id: "pos".into(),
                label: "POS / billing".into(),
                status: "planned".into(),
                note: "Sequences, receipt settings, and module flags are ready for future billing workflows.".into(),
            },
            ModuleStatus {
                id: "inventory".into(),
                label: "Inventory".into(),
                status: "planned".into(),
                note: "Per-business isolation is in place before stock tables and movement ledgers arrive.".into(),
            },
        ],
    }
}

#[allow(clippy::too_many_arguments)]
pub fn build_app_bootstrap(
    app_info: AppInfo,
    active_business: BusinessProfile,
    business_settings: BusinessSettings,
    active_tax_profile: TaxProfile,
    active_receipt_profile: ReceiptProfile,
    active_module_flags: ModuleFlags,
    active_sequences: Vec<SequenceCounter>,
    businesses: Vec<BusinessProfile>,
    business_workspaces: Vec<BusinessWorkspaceSummary>,
    patch_history: Vec<PatchRecord>,
    backups: Vec<BackupRecord>,
    storage: StorageStatus,
    recent_activity: Vec<RecentActivity>,
) -> AppBootstrap {
    let dashboard = compose_dashboard(
        &business_workspaces,
        &active_business,
        &active_module_flags,
        &active_sequences,
        &backups,
        recent_activity,
        &patch_history,
        &storage,
    );

    AppBootstrap {
        app_info,
        active_business,
        business_settings,
        active_tax_profile,
        active_receipt_profile,
        active_module_flags,
        active_sequences,
        businesses,
        business_workspaces,
        patch_history,
        backups,
        storage,
        dashboard,
    }
}
