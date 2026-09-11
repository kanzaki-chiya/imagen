# Imagen

A local-first AI image workspace for the desktop — Vue 3 + TypeScript frontend, Tauri 2 + Rust local backend, real generation through OpenAI / OpenAI-compatible image endpoints, plus an offline mock mode.

[中文 README → README.md](README.md)

## Features

- **Generate**: prompt + negative prompt; provider / model / orientation & aspect ratio (landscape 16:9, 3:2, 4:3; portrait 9:16, 2:3, 3:4; square 1:1) / resolution (1K–4K) / quality / count (1–4). `Ctrl + Enter` to generate, with cancel and retry.
- **Task queue**: submitting while generating enqueues the task (concurrency 1, cap 8). Settings → Tasks shows status and per-task cancel; unfinished tasks are marked *interrupted* after a restart.
- **History**: search, model filter, favorites, grid / list views, date grouping; reuse prompt & params; detail dialog with full parameters and references; *Open folder* and save-as export via the native dialog.
- **Presets**: apply, save, edit, delete; deletion requires confirmation.
- **Providers**: multiple providers (OpenAI / compatible / custom), model list fetched from `{baseUrl}/models`, connection testing separate from generation; unsupported parameters (seed, guidance, negative prompt, references) are disabled or marked as recorded-only per protocol capabilities.
- **Reference images**: up to 3 × PNG / JPEG / WebP (≤10 MB each) with strength sliders; saved with the generation record.
- **Theme**: Light / Dark / System, follows the OS by default.
- **Language**: Settings → Appearance switches 中文 / English; defaults to Chinese, persisted in localStorage.

## Local data

Desktop data lives in the app data directory (`%APPDATA%\com.imagen.studio\`):

- `imagen.db` — SQLite (WAL): providers, history, presets, tasks, session state.
- `images/` — full-size generated images; `thumbs/` — 384 px thumbnails for the history grid.
- API keys go to the **Windows Credential Manager** (service `com.imagen.studio`) — never written to the database or logs.

In the browser the workspace uses IndexedDB; on first desktop launch the data is migrated to SQLite automatically.

## Run

Requires Node.js 22.12+ (24 LTS recommended) and npm.

```sh
npm install
npm run dev        # http://127.0.0.1:1420 — browser runs in mock mode
```

## Desktop app (Tauri 2 + Rust)

Requires a Rust toolchain (stable-x86_64-pc-windows-msvc) and WebView2.

```sh
npm run desktop    # tauri dev — reuses a running Vite server automatically
npx tauri build    # produces MSI + NSIS installers (src-tauri/target/release/bundle/)
```

In desktop mode the frontend calls Rust commands through a unified interface: `generate_images`, `cancel_generation`, `test_connection`, SQLite workspace persistence and keyring credential management. Rust speaks the OpenAI Images API-compatible protocol (`POST {baseUrl}/images/generations`), writes images into `images/` and serves them back via the asset protocol.

## Tests

```sh
npm test                    # frontend param validation / mock scenes (node --test)
cd src-tauri && cargo test  # mock-provider end-to-end: success, rate limits, errors, corrupt images
```

`scripts/` contains helpers: `i18n-check.mjs` (bilingual key coverage), `db-inspect.py` (inspect the SQLite database), `dev-or-attach.mjs` (reuse a running Vite dev server).

## Design

Targets 1080p / 1440p desktop windows; narrow windows collapse the sidebar to icons and the parameter panel into a drawer. Assets use relative paths (`base: "./"`) and nothing is loaded from a CDN.
