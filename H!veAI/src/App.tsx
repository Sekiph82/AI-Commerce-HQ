import { BrowserRouter, Route, Routes } from "react-router-dom";
import { AppShell } from "./components/Shell";
import { RegistryProvider } from "./registryContext";
import {
  ActivityPage,
  Agents,
  Audits,
  CommandCenter,
  ProjectCockpit,
  PromptEnginePage,
  Projects,
  Settings,
  Tasks,
} from "./pages";

export default function App() {
  return (
    <RegistryProvider>
      <BrowserRouter>
        <AppShell>
          <Routes>
            <Route path="/" element={<CommandCenter />} />
            <Route path="/projects" element={<Projects />} />
            <Route path="/projects/:id" element={<ProjectCockpit />} />
            <Route path="/tasks" element={<Tasks />} />
            <Route path="/agents" element={<Agents />} />
            <Route path="/prompts" element={<PromptEnginePage />} />
            <Route path="/audits" element={<Audits />} />
            <Route path="/activity" element={<ActivityPage />} />
            <Route path="/settings" element={<Settings />} />
          </Routes>
        </AppShell>
      </BrowserRouter>
    </RegistryProvider>
  );
}
