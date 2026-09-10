import React from 'react';
import '@jethac/tools-frontend-stack/ui/styles.css';
import { createRoot } from 'react-dom/client';
import { isTauri, invoke } from '@tauri-apps/api/core';
import { save, open } from '@tauri-apps/plugin-dialog';
import { createBrowserWindowHost, createTauriWindowHost } from '@jethac/tools-frontend-stack/host';
import { GroveApp } from './App';
import { GroveModel } from './model';
import { createBrowserFileHost, createTauriFileHost, type FileHost } from './files';

const PANEL_IDS = ['library', 'view-3d', 'view-2d', 'objects', 'inspector', 'bottom'];

async function bootstrap() {
  const rootElement = document.getElementById('root');
  if (!rootElement) throw new Error('Application root is missing');

  let capturedLayout = '';
  const hostOptions = {
    appId: 'grove',
    projectId: 'grove-workbench',
    panelIds: PANEL_IDS,
    captureLayout: () => capturedLayout,
  };

  const native = isTauri();
  const windowHost = native
    ? await createTauriWindowHost({ ...hostOptions, invoke })
    : await createBrowserWindowHost(hostOptions);

  const fileHost: FileHost = native
    ? createTauriFileHost({ invoke, save, open })
    : createBrowserFileHost();

  const model = await GroveModel.create();
  const root = createRoot(rootElement);
  root.render(
    <React.StrictMode>
      <GroveApp
        model={model}
        fileHost={fileHost}
        windowHost={windowHost}
        onDockLayoutChange={(layout) => {
          capturedLayout = JSON.stringify(layout);
        }}
      />
    </React.StrictMode>,
  );
  window.addEventListener('pagehide', () => windowHost.dispose(), { once: true });
}

void bootstrap().catch((error: unknown) => {
  const root = document.getElementById('root');
  if (root) {
    const message = document.createElement('p');
    message.textContent = `Grove could not start: ${error instanceof Error ? error.message : String(error)}`;
    message.setAttribute('role', 'alert');
    root.replaceChildren(message);
  }
});
