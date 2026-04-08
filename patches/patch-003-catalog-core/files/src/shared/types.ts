export interface BusinessProfile {
  id: string;
  name: string;
  legalName: string | null;
  code: string;
  businessType: string;
  currencyCode: string;
  taxMode: string;
  phone: string | null;
  email: string | null;
  addressLine1: string | null;
  addressLine2: string | null;
  city: string | null;
  state: string | null;
  postalCode: string | null;
  country: string | null;
  createdAt: string;
  updatedAt: string;
  archivedAt: string | null;
}

export interface BusinessSettings {
  businessId: string;
  timezone: string;
  locale: string;
  dateFormat: string;
  theme: string;
  taxLabel: string;
  defaultTaxRate: number;
  pricesIncludeTax: boolean;
  receiptFooter: string | null;
  receiptShowAddress: boolean;
  receiptShowPhone: boolean;
  autoBackupEnabled: boolean;
  backupDirectory: string | null;
  moduleRestaurantEnabled: boolean;
  moduleRetailEnabled: boolean;
  moduleInventoryEnabled: boolean;
  moduleServicesEnabled: boolean;
  updatedAt: string;
}

export interface TaxProfile {
  id: string;
  businessId: string;
  name: string;
  taxLabel: string;
  defaultRate: number;
  pricesIncludeTax: boolean;
  isDefault: boolean;
  updatedAt: string;
}

export interface ReceiptProfile {
  id: string;
  businessId: string;
  name: string;
  footerText: string | null;
  showAddress: boolean;
  showPhone: boolean;
  showEmail: boolean;
  showBusinessCode: boolean;
  paperWidth: string;
  isDefault: boolean;
  updatedAt: string;
}

export interface ModuleFlags {
  businessId: string;
  restaurantEnabled: boolean;
  retailEnabled: boolean;
  inventoryEnabled: boolean;
  servicesEnabled: boolean;
  customersEnabled: boolean;
  suppliersEnabled: boolean;
  expensesEnabled: boolean;
  reportingEnabled: boolean;
  dataCenterEnabled: boolean;
  updatedAt: string;
}

export interface SequenceCounter {
  id: string;
  businessId: string;
  scope: string;
  prefix: string;
  nextNumber: number;
  padding: number;
  resetPolicy: string;
  updatedAt: string;
}

export interface NewBusinessWorkspaceInput {
  name: string;
  legalName: string | null;
  code: string;
  businessType: string;
  currencyCode: string;
  taxMode: string;
  timezone: string;
  locale: string;
  activateNow: boolean;
}

export interface WorkspaceConfigurationInput {
  businessSettings: BusinessSettings;
  taxProfile: TaxProfile;
  receiptProfile: ReceiptProfile;
  moduleFlags: ModuleFlags;
  sequenceCounters: SequenceCounter[];
}

export interface AppInfo {
  appName: string;
  productName: string;
  version: string;
  schemaVersion: number;
  patchLevel: string;
  initializedAt: string;
}

export interface PatchRecord {
  patchId: string;
  patchName: string;
  appliedAt: string;
  schemaVersion: number;
}

export interface BackupRecord {
  id: string;
  businessId: string | null;
  fileName: string;
  filePath: string;
  checksum: string | null;
  status: string;
  createdAt: string;
}

export interface ExportJobRecord {
  id: string;
  businessId: string | null;
  format: string;
  status: string;
  targetPath: string | null;
  createdAt: string;
  completedAt: string | null;
}

export interface ImportPreview {
  filePath: string;
  valid: boolean;
  manifestVersion: string | null;
  bundleType: string | null;
  sourcePatchLevel: string | null;
  businessCount: number;
  categoryCount: number;
  itemCount: number;
  generatedAt: string | null;
  warnings: string[];
}

export interface RecentActivity {
  id: string;
  level: string;
  category: string;
  message: string;
  createdAt: string;
}

export interface KpiCard {
  id: string;
  label: string;
  value: string;
  note: string;
}

export interface ModuleStatus {
  id: string;
  label: string;
  status: "active-foundation" | "planned" | "coming-next";
  note: string;
}

export interface DashboardShellData {
  heroTitle: string;
  heroBody: string;
  kpis: KpiCard[];
  recentActivity: RecentActivity[];
  moduleStatuses: ModuleStatus[];
}

export interface StorageStatus {
  dataDir: string;
  configDir: string;
  logDir: string;
  backupDir: string;
  exportDir: string;
  databasePath: string;
  databaseExists: boolean;
  backupCount: number;
  exportCount: number;
}

export interface BusinessWorkspaceSummary {
  businessId: string;
  name: string;
  code: string;
  businessType: string;
  currencyCode: string;
  theme: string;
  timezone: string;
  taxLabel: string;
  defaultTaxRate: number;
  nextSaleSequence: string;
  activeModules: string[];
  archivedAt: string | null;
  updatedAt: string;
}

export interface CatalogCategory {
  id: string;
  businessId: string;
  name: string;
  code: string;
  parentId: string | null;
  itemScope: string;
  sortOrder: number;
  notes: string | null;
  createdAt: string;
  updatedAt: string;
  archivedAt: string | null;
}

export interface CatalogUnit {
  id: string;
  businessId: string | null;
  name: string;
  code: string;
  symbol: string;
  allowFractional: boolean;
  isSystem: boolean;
  updatedAt: string;
  archivedAt: string | null;
}

export interface CatalogItem {
  id: string;
  businessId: string;
  categoryId: string | null;
  unitId: string | null;
  taxProfileId: string | null;
  itemKind: string;
  name: string;
  displayName: string | null;
  sku: string | null;
  primaryBarcode: string | null;
  description: string | null;
  sellingPrice: number;
  costPrice: number;
  trackStock: boolean;
  stockQuantity: number;
  reorderLevel: number;
  imagePath: string | null;
  isActive: boolean;
  createdAt: string;
  updatedAt: string;
  archivedAt: string | null;
}

export interface CatalogBarcode {
  id: string;
  itemId: string;
  barcode: string;
  label: string | null;
  isPrimary: boolean;
  createdAt: string;
}

export interface CatalogItemView {
  item: CatalogItem;
  categoryName: string | null;
  unitCode: string | null;
  taxLabel: string | null;
  barcodes: CatalogBarcode[];
}

export interface CatalogSummary {
  totalItems: number;
  activeItems: number;
  archivedItems: number;
  categoryCount: number;
  menuItemCount: number;
  stockItemCount: number;
  serviceItemCount: number;
  lowStockCandidates: number;
}

export interface CatalogWorkspace {
  businessId: string;
  summary: CatalogSummary;
  categories: CatalogCategory[];
  units: CatalogUnit[];
  taxProfiles: TaxProfile[];
  items: CatalogItemView[];
}

export interface SaveCatalogCategoryInput {
  id?: string | null;
  name: string;
  code: string;
  parentId: string | null;
  itemScope: string;
  sortOrder: number;
  notes: string | null;
}

export interface SaveCatalogUnitInput {
  id?: string | null;
  name: string;
  code: string;
  symbol: string;
  allowFractional: boolean;
}

export interface SaveCatalogItemInput {
  id?: string | null;
  categoryId: string | null;
  unitId: string | null;
  taxProfileId: string | null;
  itemKind: string;
  name: string;
  displayName: string | null;
  sku: string | null;
  barcodes: string[];
  description: string | null;
  sellingPrice: number;
  costPrice: number;
  trackStock: boolean;
  stockQuantity: number;
  reorderLevel: number;
  imagePath: string | null;
  isActive: boolean;
}

export interface AppBootstrap {
  appInfo: AppInfo;
  activeBusiness: BusinessProfile;
  businessSettings: BusinessSettings;
  activeTaxProfile: TaxProfile;
  activeReceiptProfile: ReceiptProfile;
  activeModuleFlags: ModuleFlags;
  activeSequences: SequenceCounter[];
  businesses: BusinessProfile[];
  businessWorkspaces: BusinessWorkspaceSummary[];
  patchHistory: PatchRecord[];
  backups: BackupRecord[];
  storage: StorageStatus;
  catalogSummary: CatalogSummary;
  dashboard: DashboardShellData;
}

export type NavPage =
  | "dashboard"
  | "catalog"
  | "business"
  | "settings"
  | "data-center";
