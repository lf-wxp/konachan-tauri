# Konachan Tauri

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2024-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2.x-24C8D8.svg)](https://tauri.app/)
[![Yew](https://img.shields.io/badge/Yew-0.23-green.svg)](https://yew.rs/)

A desktop [Konachan](https://konachan.net/) image browser built with **Tauri 2** and **Yew**, delivering a native experience across macOS, Windows, and Linux.

## Screenshot

![screenshot](./screenshot.gif)

## Tech Stack

| Layer    | Technology                                                     |
| -------- | -------------------------------------------------------------- |
| Frontend | [Yew](https://yew.rs/) (Rust → WebAssembly) + Trunk           |
| Backend  | [Tauri 2](https://tauri.app/) (Rust)                           |
| Styling  | [Stylist](https://github.com/futursolo/stylist-rs) (CSS-in-RS) |
| Icons    | [yew_icons](https://crates.io/crates/yew_icons) (Bootstrap / Lucide / Font Awesome) |

## Features

- 🖼️ Browse and search Konachan images with a smooth native UI
- 💾 Download images directly to local storage
- ⌨️ Global shortcut support (desktop platforms)
- 📋 Clipboard integration
- 🔔 System notification support
- 🎨 Splash screen on startup
- 🚀 Cross-platform — macOS, Windows, Linux

## Project Structure

```
konachan-tauri/
├── src/                  # Frontend submodule (konachan-yew)
│   ├── src/
│   │   ├── components/   # Yew UI components
│   │   ├── hook/         # Custom Yew hooks
│   │   ├── model/        # Data models
│   │   ├── store/        # State management
│   │   ├── utils/        # Utility functions
│   │   └── main.rs       # Frontend entry point
│   ├── static/           # Static assets (CSS, fonts, images)
│   ├── Cargo.toml        # Frontend dependencies
│   └── Trunk.toml        # Trunk build config
├── src-tauri/            # Tauri backend
│   ├── src/
│   │   ├── main.rs       # Backend entry point
│   │   ├── commander.rs  # Tauri command handlers
│   │   └── image.rs      # Image processing logic
│   ├── capabilities/     # Tauri permission capabilities
│   ├── icons/            # App icons
│   ├── Cargo.toml        # Backend dependencies
│   └── tauri.conf.json   # Tauri configuration
├── .github/workflows/    # CI/CD release workflow
└── README.md
```

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- [Trunk](https://trunkrs.dev/) — WASM build tool for Yew
- `wasm32-unknown-unknown` target
- Platform-specific dependencies (see [Tauri prerequisites](https://tauri.app/start/prerequisites/))

```bash
# Install Trunk
cargo install trunk

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Tauri CLI
cargo install tauri-cli
```

## Installation

```bash
# Clone with submodules
git clone --recurse-submodules https://github.com/lf-wxp/konachan-tauri.git
cd konachan-tauri

# Or initialize submodules after clone
git submodule init
git submodule update --recursive
```

### Update Submodule

```bash
git submodule update --remote
```

## Development

```bash
# Start in development mode (hot-reload enabled)
cargo tauri dev
```

## Build

```bash
# Build for production
cargo tauri build
```

The release binary will be generated in `src-tauri/target/release/`.

## Related Projects

- [konachan-yew](https://github.com/lf-wxp/konachan-yew) — Frontend submodule (Yew + WASM)
- [konachan-api](https://github.com/lf-wxp/konachan-api) — Backend API server for the web version

## License

This project is licensed under the [MIT License](./LICENSE).
