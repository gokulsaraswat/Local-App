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

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBootstrap {
    pub app_info: AppInfo,
    pub active_business: BusinessProfile,
    pub business_settings: BusinessSettings,
    pub businesses: Vec<BusinessProfile>,
    pub patch_history: Vec<PatchRecord>,
    pub backups: Vec<BackupRecord>,
    pub storage: StorageStatus,
    pub dashboard: DashboardShellData,
}
