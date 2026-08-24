import { createRoot } from 'react-dom/client';
import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import App from './App';
import './styles.css';
import './command-center.css';
function FrontendReady() {
  useEffect(() => {
    void invoke('hiveai_frontend_ready').catch(() => undefined);
  }, []);
  return <App />;
}
createRoot(document.getElementById('root') as HTMLElement).render(<FrontendReady />);
