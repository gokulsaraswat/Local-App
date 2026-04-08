use super::models::{
    AppBootstrap, AppInfo, BackupRecord, BusinessProfile, BusinessSettings,
    BusinessWorkspaceSummary, CatalogSummary, DashboardShellData, InventorySummary, KpiCard,
    ModuleFlags, ModuleStatus, PatchRecord, ReceiptProfile, RecentActivity, SequenceCounter,
    StorageStatus, TaxProfile,
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
    inventory_summary: &InventorySummary,
) -> DashboardShellData {
    DashboardShellData {
        hero_title: "Inventory-ready local workspace".into(),
        hero_body: format!(
            "Patch 4 adds a local inventory ledger with stock movements, reorder monitoring, and quantity history while keeping the desktop app fully offline. {} is active, with catalog and inventory now linked through the same local SQLite foundation.",
            active_business.name
        ),
        kpis: vec![
            KpiCard {
                id: "workspace-count".into(),
                label: "Business workspaces".into(),
                value: business_workspaces.len().to_string(),
                note: "Each workspace still keeps isolated settings, numbering, and future sales boundaries.".into(),
            },
            KpiCard {
                id: "catalog-items".into(),
                label: "Catalog items".into(),
                value: catalog_summary.active_items.to_string(),
                note: format!(
                    "{} categories and {} active stock items are available for the inventory ledger.",
                    catalog_summary.category_count, catalog_summary.stock_item_count
                ),
            },
            KpiCard {
                id: "tracked-stock".into(),
                label: "Tracked stock items".into(),
                value: inventory_summary.total_tracked_items.to_string(),
                note: format!(
                    "{} low-stock items are already visible from reorder thresholds.",
                    inventory_summary.low_stock_items
                ),
            },
            KpiCard {
                id: "movement-count".into(),
                label: "Inventory movements".into(),
                value: inventory_summary.movement_count.to_string(),
                note: "Opening balances, manual stock in/out, and catalog sync adjustments stay in the local movement history.".into(),
            },
            KpiCard {
                id: "module-count".into(),
                label: "Active modules".into(),
                value: active_module_count(active_modules).to_string(),
                note: "Inventory is now active alongside workspace settings, catalog, and data portability foundations.".into(),
            },
            KpiCard {
                id: "backup-count".into(),
                label: "Local backups".into(),
                value: backups.len().to_string(),
                note: format!(
                    "{} exports recorded in {}.",
                    storage.export_count, storage.export_dir
                ),
            },
            KpiCard {
                id: "sequence-count".into(),
                label: "Sequence counters".into(),
                value: active_sequences.len().to_string(),
                note: "Document numbering remains ready before billing, purchasing, and return workflows land.".into(),
            },
            KpiCard {
                id: "patch-count".into(),
                label: "Applied patches".into(),
                value: patch_history.len().to_string(),
                note: "Patch history stays inside the local database for traceable upgrades and rollback planning.".into(),
            },
        ],
        recent_activity,
        module_statuses: vec![
            ModuleStatus {
                id: "workspace".into(),
                label: "Workspace core".into(),
                status: "active-foundation".into(),
                note: "Multi-business profiles, settings, tax defaults, receipts, and sequence counters remain active.".into(),
            },
            ModuleStatus {
                id: "catalog".into(),
                label: "Catalog".into(),
                status: "active-foundation".into(),
                note: "Products, menu items, services, categories, units, and barcodes are available locally.".into(),
            },
            ModuleStatus {
                id: "inventory".into(),
                label: "Inventory ledger".into(),
                status: "active-foundation".into(),
                note: "Stock movements, on-hand balances, and reorder alerts are now tracked locally per business.".into(),
            },
            ModuleStatus {
                id: "pos".into(),
                label: "POS / billing".into(),
                status: "coming-next".into(),
                note: "The next patch can build on catalog, inventory, and sequence foundations for checkout and billing flows.".into(),
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
    inventory_summary: InventorySummary,
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
        &inventory_summary,
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
        inventory_summary,
        dashboard,
    }
}
