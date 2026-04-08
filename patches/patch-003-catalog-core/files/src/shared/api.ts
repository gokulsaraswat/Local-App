import { invoke } from "@tauri-apps/api/core";
import type {
  AppBootstrap,
  BackupRecord,
  BusinessProfile,
  CatalogCategory,
  CatalogItem,
  CatalogUnit,
  CatalogWorkspace,
  ImportPreview,
  NewBusinessWorkspaceInput,
  SaveCatalogCategoryInput,
  SaveCatalogItemInput,
  SaveCatalogUnitInput,
  WorkspaceConfigurationInput
} from "./types";

export async function bootstrapApp(): Promise<AppBootstrap> {
  return invoke<AppBootstrap>("bootstrap_app");
}

export async function saveBusinessProfile(
  profile: BusinessProfile
): Promise<BusinessProfile> {
  return invoke<BusinessProfile>("save_business_profile", { profile });
}

export async function createBusinessWorkspace(
  input: NewBusinessWorkspaceInput
): Promise<BusinessProfile> {
  return invoke<BusinessProfile>("create_business_workspace", { input });
}

export async function switchActiveBusiness(
  businessId: string
): Promise<BusinessProfile> {
  return invoke<BusinessProfile>("switch_active_business", {
    businessId
  });
}

export async function saveWorkspaceConfiguration(
  input: WorkspaceConfigurationInput
): Promise<void> {
  return invoke<void>("save_workspace_configuration", { input });
}

export async function loadCatalogWorkspace(): Promise<CatalogWorkspace> {
  return invoke<CatalogWorkspace>("load_catalog_workspace");
}

export async function saveCatalogCategory(
  input: SaveCatalogCategoryInput
): Promise<CatalogCategory> {
  return invoke<CatalogCategory>("save_catalog_category", { input });
}

export async function saveCatalogUnit(
  input: SaveCatalogUnitInput
): Promise<CatalogUnit> {
  return invoke<CatalogUnit>("save_catalog_unit", { input });
}

export async function saveCatalogItem(
  input: SaveCatalogItemInput
): Promise<CatalogItem> {
  return invoke<CatalogItem>("save_catalog_item", { input });
}

export async function setCatalogItemArchived(
  itemId: string,
  archived: boolean
): Promise<CatalogItem> {
  return invoke<CatalogItem>("set_catalog_item_archived", {
    itemId,
    archived
  });
}

export async function createBackupSnapshot(): Promise<BackupRecord> {
  return invoke<BackupRecord>("create_backup_snapshot");
}

export async function exportFoundationSnapshot(): Promise<string> {
  return invoke<string>("export_foundation_snapshot");
}

export async function previewImportBundle(
  filePath: string
): Promise<ImportPreview> {
  return invoke<ImportPreview>("preview_import_bundle", { filePath });
}
