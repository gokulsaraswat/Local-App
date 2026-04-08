use super::models::{
    AppBootstrap, AppInfo, BackupRecord, BusinessProfile, BusinessSettings,
    BusinessWorkspaceSummary, CatalogSummary, DashboardShellData, KpiCard, ModuleFlags,
    ModuleStatus, PatchRecord, ReceiptProfile, RecentActivity, SequenceCounter, StorageStatus,
    TaxProfile,
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
    catalog_summary: &CatalogSummary,
) -> DashboardShellData {
    DashboardShellData {
        hero_title: "Catalog-ready local workspace".into(),
        hero_body: format!(
            "Patch 3 adds catalog structure for products, menu items, and services while keeping the app fully local-first. {} is active, with categories, units, and item master data now scoped safely to the active business.",
            active_business.name
        ),
        kpis: vec![
            KpiCard {
                id: "workspace-count".into(),
                label: "Business workspaces".into(),
                value: business_workspaces.len().to_string(),
                note: "Each workspace keeps its own profile, settings, and future sales or inventory data boundaries.".into(),
            },
            KpiCard {
                id: "catalog-items".into(),
                label: "Catalog items".into(),
                value: catalog_summary.active_items.to_string(),
                note: format!(
                    "{} categories and {} low-stock candidates are visible before the deeper inventory ledger patch.",
                    catalog_summary.category_count, catalog_summary.low_stock_candidates
                ),
            },
            KpiCard {
                id: "module-count".into(),
                label: "Active modules".into(),
                value: active_module_count(active_modules).to_string(),
                note: "Module flags remain business-scoped for future POS, inventory, and service workflows.".into(),
            },
            KpiCard {
                id: "sequence-count".into(),
                label: "Sequence counters".into(),
                value: active_sequences.len().to_string(),
                note: "Document numbering is ready before POS billing and purchasing flows arrive.".into(),
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
                note: "Patch history is stored locally to keep upgrades traceable and recovery-friendly.".into(),
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
                note: "Business switching and per-business settings stay isolated locally.".into(),
            },
            ModuleStatus {
                id: "catalog".into(),
                label: "Catalog / items".into(),
                status: "active-foundation".into(),
                note: "Products, menu items, services, categories, units, and barcode-ready item master data are now available.".into(),
            },
            ModuleStatus {
                id: "inventory".into(),
                label: "Inventory ledger".into(),
                status: "coming-next".into(),
                note: "Current stock fields and reorder points are ready for movement-ledger expansion.".into(),
            },
            ModuleStatus {
                id: "pos".into(),
                label: "POS / billing".into(),
                status: "planned".into(),
                note: "Catalog records, tax defaults, and sequences are in place for future billing workflows.".into(),
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
    catalog_summary: CatalogSummary,
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
        &catalog_summary,
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
        catalog_summary,
        dashboard,
    }
}
