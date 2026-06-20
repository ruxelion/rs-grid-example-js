import { defineConfig, devices } from '@playwright/test';

// Library-mode demo: the host page (`index.html` + `main.js`) lives at the repo
// root and loads the wasm-pack output from `pkg/`. Run
// `wasm-pack build --target web --out-dir pkg` at the repo root before testing.
export default defineConfig({
  testDir: './tests',
  snapshotDir: './tests/snapshots',
  fullyParallel: false,
  retries: process.env.CI ? 1 : 0,
  reporter: process.env.CI ? 'github' : 'list',

  use: {
    baseURL: 'http://localhost:4183',
    trace: 'on-first-retry',
    viewport: { width: 1280, height: 800 },
  },

  projects: [{ name: 'chromium', use: { ...devices['Desktop Chrome'] } }],

  webServer: {
    command: 'npx serve .. -p 4183 --no-clipboard',
    url: 'http://localhost:4183',
    reuseExistingServer: !process.env.CI,
    timeout: 20_000,
  },
});
