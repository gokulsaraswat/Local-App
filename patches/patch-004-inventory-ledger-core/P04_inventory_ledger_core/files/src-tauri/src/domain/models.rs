use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessProfile {
    pub id: String,
    pub name: String,
    pub legal_name: Option<String>,
    pub code: String,
    pub business_type: String,
    pub currency_code: String,
    pub tax_mode: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessSettings {
    pub business_id: String,
    pub timezone: String,
    pub locale: String,
    pub date_format: String,
    pub theme: String,
    pub tax_label: String,
    pub default_tax_rate: f64,
    pub prices_include_tax: bool,
    pub receipt_footer: Option<String>,
    pub receipt_show_address: bool,
    pub receipt_show_phone: bool,
    pub auto_backup_enabled: bool,
    pub backup_directory: Option<String>,
    pub module_restaurant_enabled: bool,
    pub module_retail_enabled: bool,
    pub module_inventory_enabled: bool,
    pub module_services_enabled: bool,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxProfile {
    pub id: String,
    pub business_id: String,
    pub name: String,
    pub tax_label: String,
    pub default_rate: f64,
    pub prices_include_tax: bool,
    pub is_default: bool,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptProfile {
    pub id: String,
    pub business_id: String,
    pub name: String,
    pub footer_text: Option<String>,
    pub show_address: bool,
    pub show_phone: bool,
    pub show_email: bool,
    pub show_business_code: bool,
    pub paper_width: String,
    pub is_default: bool,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleFlags {
    pub business_id: String,
    pub restaurant_enabled: bool,
    pub retail_enabled: bool,
    pub inventory_enabled: bool,
    pub services_enabled: bool,
    pub customers_enabled: bool,
    pub suppliers_enabled: bool,
    pub expenses_enabled: bool,
    pub reporting_enabled: bool,
    pub data_center_enabled: bool,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceCounter {
    pub id: String,
    pub business_id: String,
    pub scope: String,
    pub prefix: String,
    pub next_number: i64,
    pub padding: i64,
    pub reset_policy: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewBusinessWorkspaceInput {
    pub name: String,
    pub legal_name: Option<String>,
    pub code: String,
    pub business_type: String,
    pub currency_code: String,
    pub tax_mode: String,
    pub timezone: String,
    pub locale: String,
    pub activate_now: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceConfigurationInput {
    pub business_settings: BusinessSettings,
    pub tax_profile: TaxProfile,
    pub receipt_profile: ReceiptProfile,
    pub module_flags: ModuleFlags,
    pub sequence_counters: Vec<SequenceCounter>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessWorkspaceSummary {
    pub business_id: String,
    pub name: String,
    pub code: String,
    pub business_type: String,
    pub currency_code: String,
    pub theme: String,
    pub timezone: String,
    pub tax_label: String,
    pub default_tax_rate: f64,
    pub next_sale_sequence: String,
    pub active_modules: Vec<String>,
    pub archived_at: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub app_name: String,
    pub product_name: String,
    pub version: String,
    pub schema_version: i64,
    pub patch_level: String,
    pub initialized_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchRecord {
    pub patch_id: String,
    pub patch_name: String,
    pub schema_version: i64,
    pub applied_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupRecord {
    pub id: String,
    pub business_id: Option<String>,
    pub file_name: String,
    pub file_path: String,
    pub checksum: Option<String>,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportJobRecord {
    pub id: String,
    pub business_id: Option<String>,
    pub format: String,
    pub status: String,
    pub target_path: Option<String>,
    pub created_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportPreview {
    pub file_path: String,
    pub valid: bool,
    pub manifest_version: Option<String>,
    pub bundle_type: Option<String>,
    pub source_patch_level: Option<String>,
    pub business_count: usize,
    pub category_count: usize,
    pub item_count: usize,
    pub movement_count: usize,
    pub generated_at: Option<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentActivity {
    pub id: String,
    pub level: String,
    pub category: String,
    pub message: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KpiCard {
    pub id: String,
    pub label: String,
    pub value: String,
    pub note: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleStatus {
    pub id: String,
    pub label: String,
    pub status: String,
    pub note: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardShellData {
    pub hero_title: String,
    pub hero_body: String,
    pub kpis: Vec<KpiCard>,
    pub recent_activity: Vec<RecentActivity>,
    pub module_statuses: Vec<ModuleStatus>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageStatus {
    pub data_dir: String,
    pub config_dir: String,
    pub log_dir: String,
    pub backup_dir: String,
    pub export_dir: String,
    pub database_path: String,
    pub database_exists: bool,
    pub backup_count: usize,
    pub export_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogCategory {
    pub id: String,
    pub business_id: String,
    pub name: String,
    pub code: String,
    pub parent_id: Option<String>,
    pub item_scope: String,
    pub sort_order: i64,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogUnit {
    pub id: String,
    pub business_id: Option<String>,
    pub name: String,
    pub code: String,
    pub symbol: String,
    pub allow_fractional: bool,
    pub is_system: bool,
    pub updated_at: String,
    pub archived_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogItem {
    pub id: String,
    pub business_id: String,
    pub category_id: Option<String>,
    pub unit_id: Option<String>,
    pub tax_profile_id: Option<String>,
    pub item_kind: String,
    pub name: String,
    pub display_name: Option<String>,
    pub sku: Option<String>,
    pub primary_barcode: Option<String>,
    pub description: Option<String>,
    pub selling_price: f64,
    pub cost_price: f64,
    pub track_stock: bool,
    pub stock_quantity: f64,
    pub reorder_level: f64,
    pub image_path: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogBarcode {
    pub id: String,
    pub item_id: String,
    pub barcode: String,
    pub label: Option<String>,
    pub is_primary: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogItemView {
    pub item: CatalogItem,
    pub category_name: Option<String>,
    pub unit_code: Option<String>,
    pub tax_label: Option<String>,
    pub barcodes: Vec<CatalogBarcode>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogSummary {
    pub total_items: usize,
    pub active_items: usize,
    pub archived_items: usize,
    pub category_count: usize,
    pub menu_item_count: usize,
    pub stock_item_count: usize,
    pub service_item_count: usize,
    pub low_stock_candidates: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogWorkspace {
    pub business_id: String,
    pub summary: CatalogSummary,
    pub categories: Vec<CatalogCategory>,
    pub units: Vec<CatalogUnit>,
    pub tax_profiles: Vec<TaxProfile>,
    pub items: Vec<CatalogItemView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveCatalogCategoryInput {
    pub id: Option<String>,
    pub name: String,
    pub code: String,
    pub parent_id: Option<String>,
    pub item_scope: String,
    pub sort_order: i64,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveCatalogUnitInput {
    pub id: Option<String>,
    pub name: String,
    pub code: String,
    pub symbol: String,
    pub allow_fractional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveCatalogItemInput {
    pub id: Option<String>,
    pub category_id: Option<String>,
    pub unit_id: Option<String>,
    pub tax_profile_id: Option<String>,
    pub item_kind: String,
    pub name: String,
    pub display_name: Option<String>,
    pub sku: Option<String>,
    pub barcodes: Vec<String>,
    pub description: Option<String>,
    pub selling_price: f64,
    pub cost_price: f64,
    pub track_stock: bool,
    pub stock_quantity: f64,
    pub reorder_level: f64,
    pub image_path: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventorySummary {
    pub total_tracked_items: usize,
    pub low_stock_items: usize,
    pub movement_count: usize,
    pub total_quantity_on_hand: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryStockItem {
    pub item_id: String,
    pub item_name: String,
    pub item_kind: String,
    pub sku: Option<String>,
    pub category_name: Option<String>,
    pub unit_code: Option<String>,
    pub track_stock: bool,
    pub stock_quantity: f64,
    pub reorder_level: f64,
    pub low_stock: bool,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryMovement {
    pub id: String,
    pub business_id: String,
    pub item_id: String,
    pub item_name: String,
    pub sku: Option<String>,
    pub unit_code: Option<String>,
    pub movement_type: String,
    pub quantity_delta: f64,
    pub quantity_after: f64,
    pub unit_cost: Option<f64>,
    pub note: Option<String>,
    pub occurred_at: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryWorkspace {
    pub business_id: String,
    pub summary: InventorySummary,
    pub stock_items: Vec<InventoryStockItem>,
    pub recent_movements: Vec<InventoryMovement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveInventoryMovementInput {
    pub item_id: String,
    pub movement_type: String,
    pub quantity: f64,
    pub unit_cost: Option<f64>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveInventoryStockRuleInput {
    pub item_id: String,
    pub track_stock: bool,
    pub reorder_level: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBootstrap {
    pub app_info: AppInfo,
    pub active_business: BusinessProfile,
    pub business_settings: BusinessSettings,
    pub active_tax_profile: TaxProfile,
    pub active_receipt_profile: ReceiptProfile,
    pub active_module_flags: ModuleFlags,
    pub active_sequences: Vec<SequenceCounter>,
    pub businesses: Vec<BusinessProfile>,
    pub business_workspaces: Vec<BusinessWorkspaceSummary>,
    pub patch_history: Vec<PatchRecord>,
    pub backups: Vec<BackupRecord>,
    pub storage: StorageStatus,
    pub catalog_summary: CatalogSummary,
    pub inventory_summary: InventorySummary,
    pub dashboard: DashboardShellData,
}
