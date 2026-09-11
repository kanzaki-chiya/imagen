# Imagen

A lightweight, secure, local-first AI image generation desktop workspace. Built with Vue 3 + TypeScript and Tauri 2 + Rust, supporting OpenAI and OpenAI-compatible image APIs for high-quality production, alongside an offline mock mode.

[English](README_EN.md) · [简体中文](README.md)

---

## Features

- **Fine-Grained Generation Control**
  - Positive and negative prompts with `Ctrl + Enter` quick submission.
  - Comprehensive aspect ratios: Landscape (16:9, 3:2, 4:3), Portrait (9:16, 2:3, 3:4), and Square (1:1).
  - Multi-tier resolutions (1K / 2K / 4K), quality settings, and batch counts (1–4 images).
  - Up to 3 reference images (PNG / JPEG / WebP, ≤10 MB each) with adjustable influence strength.
- **Multi-Provider & Adaptive Protocol**
  - Seamless support for official OpenAI endpoints and third-party compatible providers (including custom endpoints).
  - Automatic model discovery fetched directly from `{baseUrl}/models`; connection testing is decoupled from generation requests.
  - Graceful parameter degradation based on provider capabilities (unsupported fields are disabled or stored as metadata-only).
- **Background Task Queue**
  - Serial execution with a concurrency of 1 and a queue limit of 8; easily enqueue new prompts during ongoing generation.
  - Real-time queue status monitoring and individual task cancellation; unfinished tasks are automatically marked as *interrupted* after restarts.
- **History & Asset Management**
  - Generation records grouped chronologically with both waterfall grid and compact list layouts.
  - Instant keyword search, model filtering, bookmarking, and one-click prompt/parameter reuse.
  - Deletions go to a recycle bin first (restorable for 30 days); permanent deletion also removes the local image and thumbnail files.
  - Detailed inspection modal for full parameters and reference images, with copy-to-clipboard, direct native folder navigation and save-as export.
  - 2K/4K tiers request native large sizes from compatible providers and upscale locally when capped, clearly flagged in the detail view.
- **Prompt Preset Library**
  - Built-in multi-category presets; save, edit, and organize custom parameter recipes for rapid workflow reuse.
- **Immersive Desktop Experience**
  - Custom frameless title bar tailored for 1080p and 1440p displays.
  - Responsive adaptive layout: compact windows automatically collapse the sidebar into icons and transition parameter panels into slide-over drawers.
  - Clipboard-native workflow: Ctrl+V pastes a clipboard image as a reference; generated results copy straight to the clipboard.
  - Silent update check on launch with one-click download, install and restart (signed Tauri Updater artifacts).
  - Seamless bilingual switching (Simplified Chinese & English) and theme customization (Light / Dark / Follow System).

## Security & Local Storage

Imagen adheres strictly to the Local-First philosophy, ensuring full ownership and confidentiality of your credentials and data:

- **OS-Level Credential Isolation**: API keys are safeguarded inside the **Windows Credential Manager** (Service: `com.imagen.studio`) — never stored in plaintext across databases or log files.
- **Self-Contained SQLite Database**: Configurations, image metadata, presets, and task logs are managed locally via SQLite (configured with WAL mode for performance).
- **Zero External Resource Dependencies**: Generated images and 384 px thumbnails are stored in the local application directory (`%APPDATA%\com.imagen.studio\`) and served via Tauri's native asset protocol, completely free from external CDNs.
- **Built-in Offline Mock Mode**: Automatic offline mock generation for development and instant UI evaluation without external API calls.

## Keyboard Shortcuts

| Shortcut | Description |
| :--- | :--- |
| `Ctrl + Enter` | Submit / retry generation |
| `Ctrl + Alt + 1` | Switch to **Generate** workspace |
| `Ctrl + Alt + 2` | Switch to **History** workspace |
| `Ctrl + Alt + 3` | Switch to **Presets** workspace |
| `Ctrl + Alt + 4` | Switch to **Settings** workspace |
| `←` / `→` | Navigate batch images |
| `Esc` | Close modal / drawer |
| `?` | Open keyboard shortcuts help |

## Getting Started & Development

### Prerequisites

- **Node.js**: 22.12+ (Node.js 24 LTS recommended) and npm
- **Rust Toolchain**: `stable-x86_64-pc-windows-msvc` (required for desktop runtime and bundling)
- **WebView2**: Built into Windows 10 / 11

### Installation

```sh
npm install
```

### Running Locally

- **Browser Mock Mode** (lightweight frontend preview without Rust):
  ```sh
  npm run dev
  # Navigate to http://127.0.0.1:1420
  ```
- **Desktop Development Mode** (full native backend integration):
  ```sh
  npm run desktop
  ```

### Production Build

Compile the frontend and build Windows installers (MSI and NSIS):

```sh
npx tauri build
```

Installers are located at `src-tauri/target/release/bundle/`.

### Running Tests

```sh
# Run frontend parameter validation and mock scene tests
npm test

# Run Rust backend integration and error recovery tests
cd src-tauri && cargo test
```

## Tech Stack

- **Frontend**: Vue 3 + TypeScript + Vite + Lucide Icons + Custom Design Tokens
- **Desktop Shell**: Tauri 2 + Rust
- **Storage & Security**: SQLite (WAL mode) + Windows Credential Manager
- **API Protocol**: OpenAI Images API Compatible Standard (`POST {baseUrl}/images/generations`)
