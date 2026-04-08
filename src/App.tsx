import { AppProvider, AppStateView } from "./app/AppProvider";
import { AppShell } from "./modules/shell/AppShell";

function App() {
  return (
    <AppProvider>
      <AppStateView>
        <AppShell />
      </AppStateView>
    </AppProvider>
  );
}

export default App;
