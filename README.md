# Konachan Tauri

<p align="center">
  <img src="./screenshot.gif" alt="Konachan Tauri Screenshot" width="600" />
</p>

<p align="center">
  <a href="https://github.com/lf-wxp/konachan-tauri/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT" />
  </a>
  <img src="https://img.shields.io/badge/Rust-2024-orange.svg" alt="Rust" />
  <a href="https://tauri.app/">
    <img src="https://img.shields.io/badge/Tauri-2.x-24C8D8.svg" alt="Tauri 2.x" />
  </a>
  <a href="https://yew.rs/">
    <img src="https://img.shields.io/badge/Yew-0.23-green.svg" alt="Yew 0.23" />
  </a>
  <a href="https://github.com/lf-wxp/konachan-tauri/releases">
    <img src="https://img.shields.io/github/release/lf-wxp/konachan-tauri.svg" alt="Latest Release" />
  </a>
  <a href="https://github.com/lf-wxp/konachan-tauri/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/lf-wxp/konachan-tauri/release.yml" alt="CI/CD" />
  </a>
</p>

<p align="center">
  A beautiful desktop <a href="https://konachan.net/">Konachan</a> image browser built with <b>Tauri 2</b> and <b>Yew</b>, delivering a native experience across macOS, Windows, and Linux.
</p>

---

## 📋 Table of Contents

- [Konachan Tauri](#konachan-tauri)
  - [📋 Table of Contents](#-table-of-contents)
  - [✨ Features](#-features)
  - [📸 Screenshots](#-screenshots)
  - [🛠️ Tech Stack](#️-tech-stack)
  - [🏗️ Architecture](#️-architecture)
  - [📦 Prerequisites](#-prerequisites)
    - [Required Tools](#required-tools)
    - [Quick Setup](#quick-setup)
    - [Platform-Specific Dependencies](#platform-specific-dependencies)
  - [🚀 Installation](#-installation)
    - [Option 1: Clone with Submodules (Recommended)](#option-1-clone-with-submodules-recommended)
    - [Option 2: Initialize Submodules After Clone](#option-2-initialize-submodules-after-clone)
    - [Update Submodule](#update-submodule)
  - [💻 Development](#-development)
    - [Start Development Server](#start-development-server)
    - [Build for Production](#build-for-production)
    - [Build for Specific Platforms](#build-for-specific-platforms)
  - [⌨️ Keyboard Shortcuts](#️-keyboard-shortcuts)
  - [📁 Project Structure](#-project-structure)
  - [🔧 Troubleshooting](#-troubleshooting)
    - [Common Issues](#common-issues)
    - [Getting Help](#getting-help)
  - [🤝 Contributing](#-contributing)
    - [Development Guidelines](#development-guidelines)
  - [🔗 Related Projects](#-related-projects)
  - [📄 License](#-license)
  - [� Acknowledgments](#-acknowledgments)

---

## ✨ Features

- 🖼️ **Browse & Search** — Smooth browsing and searching of Konachan images with a native UI
- 💾 **Download** — Save images directly to local storage with one click
- ⌨️ **Global Shortcuts** — Customizable keyboard shortcuts for quick access
- 📋 **Clipboard Integration** — Copy image URLs or data to clipboard instantly
- 🔔 **System Notifications** — Get notified when downloads complete
- 🎨 **Splash Screen** — Beautiful startup animation
- 🚀 **Cross-Platform** — Native performance on macOS, Windows, and Linux
- 🎯 **Lightweight** — Small bundle size thanks to Tauri's efficient architecture

---

## 📸 Screenshots

<!-- Add your screenshots here -->
<!-- Example: ![Main Window](./docs/screenshots/main.png) -->

> 💡 **Tip**: Replace `screenshot.gif` with actual screenshots of your application!

---

## 🛠️ Tech Stack

| Layer     | Technology                                                                | Description                          |
| --------- | ------------------------------------------------------------------------- | ------------------------------------ |
| **Frontend** | [Yew](https://yew.rs/) + [Trunk](https://trunkrs.dev/)                | Rust → WebAssembly UI framework      |
| **Backend**  | [Tauri 2](https://tauri.app/)                                           | Secure Rust-based desktop framework  |
| **Styling**  | [Stylist](https://github.com/futursolo/stylist-rs)                     | CSS-in-Rust for type-safe styling   |
| **Icons**     | [yew_icons](https://crates.io/crates/yew_icons)                         | Bootstrap / Lucide / Font Awesome   |
| **Language**  | Rust + TypeScript (for configuration)                                    | Type-safe full-stack development     |

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────┐
│           Tauri Shell (Native)          │
│  ┌───────────────────────────────────┐  │
│  │    WebView (Frontend - Yew WASM)  │  │
│  │  • Components                      │  │
│  │  • State Management                │  │
│  │  • UI Interactions                 │  │
│  └───────────────────────────────────┘  │
│  ┌───────────────────────────────────┐  │
│  │   Tauri Backend (Rust Commands)   │  │
│  │  • File Operations                 │  │
│  │  • System Tray                     │  │
│  │  • Global Shortcuts                │  │
│  │  • Notifications                   │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
```

---

## 📦 Prerequisites

### Required Tools

- **[Rust](https://www.rust-lang.org/tools/install)** (stable toolchain)
- **[Trunk](https://trunkrs.dev/)** — WASM build tool for Yew
- **`wasm32-unknown-unknown`** target
- Platform-specific dependencies (see [Tauri prerequisites](https://tauri.app/start/prerequisites/))

### Quick Setup

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Trunk
cargo install trunk

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Tauri CLI
cargo install tauri-cli --version "^2"
```

### Platform-Specific Dependencies

<details>
<summary><b>macOS</b></summary>

```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install required system libraries (if using Homebrew)
brew install gtk+3
```

</details>

<details>
<summary><b>Windows</b></summary>

- Install [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- Install [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (usually pre-installed on Windows 10/11)

</details>

<details>
<summary><b>Linux</b></summary>

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev

# Arch Linux
sudo pacman -S webkit2gtk-4.1 base-devel curl wget file xdotool openssl libappindicator-gtk3

# Fedora
sudo dnf install webkit2gtk4.1-devel curl wget file openssl-devel libappindicator-gtk3-devel
```

</details>

---

## 🚀 Installation

### Option 1: Clone with Submodules (Recommended)

```bash
# Clone the repository with submodules
git clone --recurse-submodules https://github.com/lf-wxp/konachan-tauri.git
cd konachan-tauri

# Install dependencies and run
cargo tauri dev
```

### Option 2: Initialize Submodules After Clone

```bash
# Clone without submodules
git clone https://github.com/lf-wxp/konachan-tauri.git
cd konachan-tauri

# Initialize and update submodules
git submodule init
git submodule update --recursive
```

### Update Submodule

```bash
# Update to the latest version of the frontend submodule
git submodule update --remote
```

---

## 💻 Development

### Start Development Server

```bash
# Start in development mode (with hot-reload)
cargo tauri dev
```

This will:
- Start the Trunk dev server for the Yew frontend
- Launch the Tauri window with hot-reload enabled
- Watch for changes in both frontend and backend code

### Build for Production

```bash
# Build optimized release binary
cargo tauri build
```

The release binary will be generated in `src-tauri/target/release/`.

### Build for Specific Platforms

<details>
<summary><b>Build Commands for Each Platform</b></summary>

```bash
# macOS (from macOS machine)
cargo tauri build

# Windows (from Windows machine)
cargo tauri build

# Linux (from Linux machine)
cargo tauri build

# Cross-compilation (advanced)
# Refer to Tauri documentation for cross-compilation setup
```

</details>

---

## ⌨️ Keyboard Shortcuts

> 💡 **Note**: Configure global shortcuts in the application settings.

| Shortcut | Action |
| -------- | ------ |
| `Ctrl/Cmd + F` | Focus search bar |
| `Ctrl/Cmd + D` | Download current image |
| `Ctrl/Cmd + C` | Copy image URL |
| `Esc` | Close dialog / Go back |
| `F5` | Refresh gallery |

> 🔧 **Customization**: You can customize these shortcuts in the app settings.

---

## 📁 Project Structure

```
konachan-tauri/
├── src/                          # Frontend submodule (konachan-yew)
│   ├── src/
│   │   ├── components/           # Yew UI components
│   │   │   ├── image_card/      # Image display components
│   │   │   ├── navbar/          # Navigation components
│   │   │   └── ...
│   │   ├── hook/                # Custom Yew hooks
│   │   ├── model/               # Data models and types
│   │   ├── store/               # State management
│   │   ├── utils/               # Utility functions
│   │   └── main.rs             # Frontend entry point
│   ├── static/                  # Static assets (CSS, fonts, images)
│   ├── Cargo.toml              # Frontend dependencies
│   └── Trunk.toml              # Trunk build configuration
│
├── src-tauri/                   # Tauri backend
│   ├── src/
│   │   ├── main.rs             # Backend entry point
│   │   ├── commander.rs        # Tauri command handlers
│   │   └── image.rs            # Image processing logic
│   ├── capabilities/            # Tauri permission capabilities
│   ├── icons/                  # App icons (png, ico, icns)
│   ├── Cargo.toml              # Backend dependencies
│   └── tauri.conf.json         # Tauri configuration
│
├── .github/
│   └── workflows/              # CI/CD release workflow
│       └── release.yml
│
├── LICENSE                      # MIT License
├── README.md                    # This file
└── .gitmodules                 # Git submodule configuration
```

---

## 🔧 Troubleshooting

### Common Issues

<details>
<summary><b>❌ `cargo tauri dev` fails with WebView error</b></summary>

**Solution**:
- **macOS**: Ensure Xcode Command Line Tools are installed (`xcode-select --install`)
- **Linux**: Install `webkit2gtk` development packages
- **Windows**: Install [WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

</details>

<details>
<summary><b>❌ `trunk` command not found</b></summary>

**Solution**:
```bash
cargo install trunk
```

If still not found, ensure `~/.cargo/bin` is in your PATH.

</details>

<details>
<summary><b>❌ Submodule not initialized</b></summary>

**Solution**:
```bash
git submodule init
git submodule update --recursive
```

</details>

<details>
<summary><b>❌ Build fails with Rust compilation errors</b></summary>

**Solution**:
```bash
# Update Rust toolchain
rustup update

# Clean and rebuild
cargo clean
cargo tauri build
```

</details>

### Getting Help

If you encounter issues not listed here:
1. Check the [Issues](https://github.com/lf-wxp/konachan-tauri/issues) page
2. Open a new issue with detailed error information

---

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add some amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Development Guidelines

- Follow the existing code style
- Write meaningful commit messages
- Add tests for new features
- Update documentation as needed

---

## 🔗 Related Projects

| Project | Description | Link |
| ------- | ----------- | ---- |
| **konachan-yew** | Frontend submodule (Yew + WASM) | [GitHub](https://github.com/lf-wxp/konachan-yew) |
| **konachan-api** | Backend API server for the web version | [GitHub](https://github.com/lf-wxp/konachan-api) |
| **Konachan** | The image board this app is based on | [Website](https://konachan.net/) |

---

## 📄 License

This project is licensed under the **MIT License**. See the [LICENSE](./LICENSE) file for details.

---

## � Acknowledgments

- [Konachan](https://konachan.net/) for providing the API
- [Tauri](https://tauri.app/) for the amazing desktop framework
- [Yew](https://yew.rs/) for the Rust WASM framework
- All contributors who have helped this project

---

<p align="center">
  Made with ❤️ by <a href="https://github.com/lf-wxp">lf-wxp</a>
</p>

<p align="center">
  ⭐ Star this repo if you find it useful!
</p>
