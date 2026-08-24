# M01 Tauri 2 Foundation

Date: 2026-08-24

Git root: `C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Application root: `C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

## Scope

M01 creates a new H!veAI Tauri 2 desktop foundation under the child application
root only. It does not upgrade or repair the legacy parent `src-tauri`, does not
launch inherited commerce runtimes, and does not implement M02 dashboard UI.

## Frontend

- React 19 + TypeScript + Vite 8.
- Child dependencies and lockfile are owned by `H!veAI/package.json` and
  `H!veAI/package-lock.json`.
- Dev server: `http://127.0.0.1:5173`.
- Production build output: `H!veAI/dist`.
- M01 screen shows only `H!veAI`, `AI Development Command Center`, foundation
  status, safe native status data, and foundation buttons.

## Tauri 2 Shell

- Rust package/bin: `hiveai-desktop`.
- Product name: `H!veAI`.
- App identifier: `ai.hiveai.desktop`.
- Tauri crate: `2.11.5`.
- Tauri CLI: `2.11.4`.
- `frontendDist`: `../dist`.
- No parent Tauri 1 files were modified.

## Capabilities And Permissions

Enabled capability permissions:

- `core:default`
- `log:default`
- `notification:default`
- `allow-native-status`
- `allow-request-restart`

No shell, filesystem, dialog, process, HTTP, or unrestricted network permissions
were added.

Custom command permissions are defined in
`H!veAI/src-tauri/permissions/foundation.toml`.

## Native Commands

- `hiveai_native_status`: returns product name, identifier, version, platform,
  app-data directory, and log directory.
- `hiveai_request_restart`: requests application restart through Tauri's app API.

No arbitrary shell command is exposed.

## Logging And Notifications

Rust plugins:

- `tauri-plugin-log`
- `tauri-plugin-notification`

Log output on Windows was observed at:

`C:\Users\sekip\AppData\Local\ai.hiveai.desktop\logs\hiveai.log`

Tauri's app-data path resolves under:

`C:\Users\sekip\AppData\Roaming\ai.hiveai.desktop`

Notifications are enabled as a foundation plugin and permission only. M01 does
not send product workflow notifications. Runtime notification permission and UX
will be reviewed in a later milestone before notification workflows are added.

## App-Data Migration Policy

M01 performs no migration and deletes no old data. Future migration from old
AI-Commerce-HQ locations must be explicit, non-destructive, backed up, and
audited before any write.

Minimum future policy:

1. Detect old locations read-only.
2. Show exact source and target paths.
3. Create a timestamped backup before writing.
4. Copy or transform into H!veAI-owned storage.
5. Leave originals in place unless the user explicitly requests cleanup.
6. Record migration evidence once the event ledger exists.

## Verification

Passed:

- `npm install` from `H!veAI/`
- `npm run typecheck`
- `npm run build`
- `npx tauri --version`: `tauri-cli 2.11.4`
- `cargo check --manifest-path H!veAI/src-tauri/Cargo.toml`
- `cargo test --manifest-path H!veAI/src-tauri/Cargo.toml`
- `cargo build --manifest-path H!veAI/src-tauri/Cargo.toml`
- Bounded Windows smoke with child Vite dev server and desktop executable

Smoke evidence:

- Window title: `H!veAI`
- Native IPC: `hiveai_native_status` logged from frontend invocation
- Legacy commerce runtime: port `8765` stayed at `0` listeners before and after
- Close: desktop process stopped cleanly

Blocked/manual:

- Restart command compiles and is exposed behind a specific permission, but full
  restart-cycle verification remains manual to avoid leaving a restarted desktop
  process running during M01 automation.

## Known M01 Failures Fixed

- TypeScript initially failed on CSS side-effect import; fixed by adding
  `src/vite-env.d.ts`.
- Vite initially inherited parent PostCSS config and failed on parent
  `autoprefixer`; fixed by adding child `postcss.config.js`.
- Direct debug executable launch opened the Tauri window but did not load the
  dev frontend; fixed the smoke method by starting child Vite first.
