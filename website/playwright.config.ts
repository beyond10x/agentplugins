import { defineConfig } from '@playwright/test';
import { gatePort } from './ux/port.ts';

const PORT = gatePort();
const BUILD = process.env.UX_BUILD ?? 'build';

export default defineConfig({
  testDir: 'ux',
  testMatch: '*.spec.ts',
  fullyParallel: true,
  workers: 2,
  forbidOnly: true,
  reporter: [['line'], ['./ux/slowest.ts']],
  timeout: 60_000,
  use: {
    baseURL: `http://127.0.0.1:${PORT}/agentplugins/`,
    launchOptions: process.env.UX_CHROME ? { executablePath: process.env.UX_CHROME } : {},
  },
  webServer: {
    command: `node ux/serve.ts ${BUILD} ${PORT} --base /agentplugins/ --docs`,
    url: `http://127.0.0.1:${PORT}/agentplugins/`,
    reuseExistingServer: false,
  },
});
