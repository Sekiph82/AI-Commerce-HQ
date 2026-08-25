import { createRoot } from "react-dom/client";
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import App from "./App";
import { StartupIntro } from "./components/StartupIntro";
import "./styles.css";
import "./command-center.css";
function FrontendReady() {
  useEffect(() => {
    if ("__TAURI_INTERNALS__" in window) {
      void invoke("hiveai_frontend_ready").catch(() => undefined);
    }
  }, []);
  return (
    <>
      <App />
      <StartupIntro />
    </>
  );
}
createRoot(document.getElementById("root") as HTMLElement).render(
  <FrontendReady />,
);
