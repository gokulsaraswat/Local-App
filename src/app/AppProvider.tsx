import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
  type PropsWithChildren
} from "react";
import {
  bootstrapApp,
  createBackupSnapshot,
  exportFoundationSnapshot,
  previewImportBundle,
  saveBusinessProfile,
  saveBusinessSettings
} from "../shared/api";
import type {
  AppBootstrap,
  BackupRecord,
  BusinessProfile,
  BusinessSettings,
  ImportPreview
} from "../shared/types";

type AppStatus = "loading" | "ready" | "error";

interface AppContextValue {
  status: AppStatus;
  errorMessage: string | null;
  data: AppBootstrap | null;
  refresh: () => Promise<void>;
  saveProfile: (profile: BusinessProfile) => Promise<BusinessProfile>;
  saveSettings: (settings: BusinessSettings) => Promise<BusinessSettings>;
  createBackup: () => Promise<BackupRecord>;
  exportFoundation: () => Promise<string>;
  previewImport: (filePath: string) => Promise<ImportPreview>;
}

type RefreshMode = "blocking" | "background";

const AppContext = createContext<AppContextValue | undefined>(undefined);

export function AppProvider({ children }: PropsWithChildren) {
  const [status, setStatus] = useState<AppStatus>("loading");
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [data, setData] = useState<AppBootstrap | null>(null);
  const hasLoadedOnceRef = useRef(false);

  const loadBootstrap = useCallback(async (mode: RefreshMode) => {
    if (mode === "blocking" || !hasLoadedOnceRef.current) {
      setStatus("loading");
    }
    setErrorMessage(null);
    try {
      const bootstrap = await bootstrapApp();
      setData(bootstrap);
      hasLoadedOnceRef.current = true;
      setStatus("ready");
    } catch (error) {
      const message =
        error instanceof Error ? error.message : "Failed to bootstrap app";
      setErrorMessage(message);
      setStatus("error");
    }
  }, []);

  const refresh = useCallback(async () => {
    await loadBootstrap("background");
  }, [loadBootstrap]);

  useEffect(() => {
    void loadBootstrap("blocking");
  }, [loadBootstrap]);

  const saveProfile = useCallback(
    async (profile: BusinessProfile) => {
      const saved = await saveBusinessProfile(profile);
      await refresh();
      return saved;
    },
    [refresh]
  );

  const saveSettingsAction = useCallback(
    async (settings: BusinessSettings) => {
      const saved = await saveBusinessSettings(settings);
      await refresh();
      return saved;
    },
    [refresh]
  );

  const createBackupAction = useCallback(async () => {
    const result = await createBackupSnapshot();
    await refresh();
    return result;
  }, [refresh]);

  const exportFoundationAction = useCallback(async () => {
    const result = await exportFoundationSnapshot();
    await refresh();
    return result;
  }, [refresh]);

  const previewImportAction = useCallback(async (filePath: string) => {
    return previewImportBundle(filePath);
  }, []);

  const value = useMemo<AppContextValue>(
    () => ({
      status,
      errorMessage,
      data,
      refresh,
      saveProfile,
      saveSettings: saveSettingsAction,
      createBackup: createBackupAction,
      exportFoundation: exportFoundationAction,
      previewImport: previewImportAction
    }),
    [
      status,
      errorMessage,
      data,
      refresh,
      saveProfile,
      saveSettingsAction,
      createBackupAction,
      exportFoundationAction,
      previewImportAction
    ]
  );

  return <AppContext.Provider value={value}>{children}</AppContext.Provider>;
}

export function useAppState(): AppContextValue {
  const value = useContext(AppContext);
  if (!value) {
    throw new Error("useAppState must be used inside AppProvider");
  }
  return value;
}

export function AppStateView({ children }: PropsWithChildren) {
  const { status, errorMessage, refresh } = useAppState();

  if (status === "loading") {
    return (
      <div className="app-loading-shell">
        <div className="spinner" />
        <h1>Preparing local workspace…</h1>
        <p>Initializing database, patch registry, and demo business profile.</p>
      </div>
    );
  }

  if (status === "error") {
    return (
      <div className="app-loading-shell">
        <h1>Failed to start the app</h1>
        <p>{errorMessage ?? "Unknown startup error"}</p>
        <button className="primary-button" onClick={() => void refresh()}>
          Retry startup
        </button>
      </div>
    );
  }

  return <>{children}</>;
}
