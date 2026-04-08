import { invoke } from "@tauri-apps/api/core";
import type {
  AppBootstrap,
  BackupRecord,
  BusinessProfile,
  ImportPreview,
  NewBusinessWorkspaceInput,
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
