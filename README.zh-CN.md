# Konachan Tauri

<p align="center">
  <img src="./screenshot.gif" alt="Konachan Tauri 截图" width="600" />
</p>

<p align="center">
  <a href="https://github.com/lf-wxp/konachan-tauri/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="许可证: MIT" />
  </a>
  <img src="https://img.shields.io/badge/Rust-2024-orange.svg" alt="Rust" />
  <a href="https://tauri.app/">
    <img src="https://img.shields.io/badge/Tauri-2.x-24C8D8.svg" alt="Tauri 2.x" />
  </a>
  <a href="https://yew.rs/">
    <img src="https://img.shields.io/badge/Yew-0.23-green.svg" alt="Yew 0.23" />
  </a>
  <a href="https://github.com/lf-wxp/konachan-tauri/releases">
    <img src="https://img.shields.io/github/release/lf-wxp/konachan-tauri.svg" alt="最新版本" />
  </a>
  <a href="https://github.com/lf-wxp/konachan-tauri/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/lf-wxp/konachan-tauri/release.yml" alt="CI/CD" />
  </a>
</p>

<p align="center">
  一个使用 <b>Tauri 2</b> 和 <b>Yew</b> 构建的精美桌面端 <a href="https://konachan.com/">Konachan</a> 图片浏览器，为 macOS、Windows 和 Linux 提供原生体验。
</p>

---

## 📋 目录

- [Konachan Tauri](#konachan-tauri)
  - [📋 目录](#-目录)
  - [✨ 功能特性](#-功能特性)
  - [📸 应用截图](#-应用截图)
  - [🛠️ 技术栈](#️-技术栈)
  - [🏗️ 架构设计](#️-架构设计)
  - [📦 环境要求](#-环境要求)
    - [必需工具](#必需工具)
    - [快速安装](#快速安装)
    - [平台特定依赖](#平台特定依赖)
  - [🚀 安装指南](#-安装指南)
    - [方式一：克隆时包含子模块（推荐）](#方式一克隆时包含子模块推荐)
    - [方式二：克隆后初始化子模块](#方式二克隆后初始化子模块)
    - [更新子模块](#更新子模块)
  - [💻 开发指南](#-开发指南)
    - [启动开发服务器](#启动开发服务器)
    - [构建生产版本](#构建生产版本)
    - [为特定平台构建](#为特定平台构建)
  - [⌨️ 键盘快捷键](#️-键盘快捷键)
  - [📁 项目结构](#-项目结构)
  - [🔧 故障排除](#-故障排除)
    - [常见问题](#常见问题)
    - [获取帮助](#获取帮助)
  - [🤝 贡献指南](#-贡献指南)
    - [开发规范](#开发规范)
  - [🔗 相关项目](#-相关项目)
  - [📄 许可证](#-许可证)
  - [🙏 致谢](#-致谢)

---

## ✨ 功能特性

- 🖼️ **浏览与搜索** — 通过原生 UI 流畅地浏览和搜索 Konachan 图片
- 💾 **下载功能** — 一键将图片保存到本地存储
- ⌨️ **全局快捷键** — 可自定义的键盘快捷键，快速访问
- 📋 **剪贴板集成** — 即时复制图片 URL 或数据到剪贴板
- 🔔 **系统通知** — 下载完成时接收通知
- 🎨 **启动画面** — 精美的启动动画
- 🚀 **跨平台** — 在 macOS、Windows 和 Linux 上提供原生性能
- 🎯 **轻量级** — 得益于 Tauri 的高效架构，体积小巧

---

## 📸 应用截图

<!-- 在此处添加你的截图 -->
<!-- 示例: ![主窗口](./docs/screenshots/main.png) -->

> 💡 **提示**: 请将 `screenshot.gif` 替换为应用程序的实际截图！

---

## 🛠️ 技术栈

| 层级 | 技术 | 描述 |
| --------- | ------------------------------------------------------------------------- | ------------------------------------ |
| **前端** | [Yew](https://yew.rs/) + [Trunk](https://trunkrs.dev/) | Rust → WebAssembly UI 框架 |
| **后端** | [Tauri 2](https://tauri.app/) | 基于 Rust 的安全桌面框架 |
| **样式** | [Stylist](https://github.com/futursolo/stylist-rs) | 类型安全的 CSS-in-Rust |
| **图标** | [yew_icons](https://crates.io/crates/yew_icons) | Bootstrap / Lucide / Font Awesome |
| **语言** | Rust + TypeScript (用于配置) | 类型安全的全栈开发 |

---

## 🏗️ 架构设计

```
┌─────────────────────────────────────────┐
│           Tauri Shell (原生层)          │
│  ┌───────────────────────────────────┐  │
│  │    WebView (前端 - Yew WASM)      │  │
│  │  • 组件                             │  │
│  │  • 状态管理                         │  │
│  │  • UI 交互                          │  │
│  └───────────────────────────────────┘  │
│  ┌───────────────────────────────────┐  │
│  │   Tauri 后端 (Rust 命令)          │  │
│  │  • 文件操作                        │  │
│  │  • 系统托盘                        │  │
│  │  • 全局快捷键                      │  │
│  │  • 通知                            │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
```

---

## 📦 环境要求

### 必需工具

- **[Rust](https://www.rust-lang.org/tools/install)** (稳定工具链)
- **[Trunk](https://trunkrs.dev/)** — Yew 的 WASM 构建工具
- **`wasm32-unknown-unknown`** 编译目标
- 平台特定依赖（参见 [Tauri 环境要求](https://tauri.app/start/prerequisites/)）

### 快速安装

```bash
# 安装 Rust（如果尚未安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Trunk
cargo install trunk

# 添加 WASM 编译目标
rustup target add wasm32-unknown-unknown

# 安装 Tauri CLI
cargo install tauri-cli --version "^2"
```

### 平台特定依赖

<details>
<summary><b>macOS</b></summary>

```bash
# 安装 Xcode 命令行工具
xcode-select --install

# 安装必需的系统库（如果使用 Homebrew）
brew install gtk+3
```

</details>

<details>
<summary><b>Windows</b></summary>

- 安装 [Microsoft Visual Studio C++ 生成工具](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- 安装 [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)（Windows 10/11 通常已预装）

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

## 🚀 安装指南

### 方式一：克隆时包含子模块（推荐）

```bash
# 克隆包含子模块的仓库
git clone --recurse-submodules https://github.com/lf-wxp/konachan-tauri.git
cd konachan-tauri

# 安装依赖并运行
cargo tauri dev
```

### 方式二：克隆后初始化子模块

```bash
# 克隆不包含子模块的仓库
git clone https://github.com/lf-wxp/konachan-tauri.git
cd konachan-tauri

# 初始化并更新子模块
git submodule init
git submodule update --recursive
```

### 更新子模块

```bash
# 更新到前端子模块的最新版本
git submodule update --remote
```

---

## 💻 开发指南

### 启动开发服务器

```bash
# 以开发模式启动（支持热重载）
cargo tauri dev
```

这将：
- 启动 Yew 前端的 Trunk 开发服务器
- 启动启用了热重载的 Tauri 窗口
- 监视前端和后端代码的更改

### 构建生产版本

```bash
# 构建优化的发布二进制文件
cargo tauri build
```

发布的二进制文件将生成在 `src-tauri/target/release/` 目录中。

### 为特定平台构建

<details>
<summary><b>各平台的构建命令</b></summary>

```bash
# macOS（在 macOS 机器上）
cargo tauri build

# Windows（在 Windows 机器上）
cargo tauri build

# Linux（在 Linux 机器上）
cargo tauri build

# 交叉编译（高级）
# 请参阅 Tauri 文档以了解交叉编译设置
```

</details>

---

## ⌨️ 键盘快捷键

> 💡 **注意**: 在应用程序设置中配置全局快捷键。

| 快捷键 | 操作 |
| -------- | ------ |
| `Ctrl/Cmd + F` | 聚焦搜索栏 |
| `Ctrl/Cmd + D` | 下载当前图片 |
| `Ctrl/Cmd + C` | 复制图片 URL |
| `Esc` | 关闭对话框 / 返回 |
| `F5` | 刷新图库 |

> 🔧 **自定义**: 你可以在应用设置中自定义这些快捷键。

---

## 📁 项目结构

```
konachan-tauri/
├── src/                          # 前端子模块 (konachan-yew)
│   ├── src/
│   │   ├── components/           # Yew UI 组件
│   │   │   ├── image_card/      # 图片显示组件
│   │   │   ├── navbar/          # 导航组件
│   │   │   └── ...
│   │   ├── hook/                # 自定义 Yew hooks
│   │   ├── model/               # 数据模型和类型
│   │   ├── store/               # 状态管理
│   │   ├── utils/               # 工具函数
│   │   └── main.rs             # 前端入口点
│   ├── static/                  # 静态资源（CSS、字体、图片）
│   ├── Cargo.toml              # 前端依赖
│   └── Trunk.toml              # Trunk 构建配置
│
├── src-tauri/                   # Tauri 后端
│   ├── src/
│   │   ├── main.rs             # 后端入口点
│   │   ├── commander.rs        # Tauri 命令处理程序
│   │   └── image.rs            # 图片处理逻辑
│   ├── capabilities/            # Tauri 权限能力
│   ├── icons/                  # 应用图标（png、ico、icns）
│   ├── Cargo.toml              # 后端依赖
│   └── tauri.conf.json         # Tauri 配置
│
├── .github/
│   └── workflows/              # CI/CD 发布工作流
│       └── release.yml
│
├── LICENSE                      # MIT 许可证
├── README.md                    # 英文文档
├── README.zh-CN.md             # 中文文档
└── .gitmodules                 # Git 子模块配置
```

---

## 🔧 故障排除

### 常见问题

<details>
<summary><b>❌ `cargo tauri dev` 失败，显示 WebView 错误</b></summary>

**解决方案**:
- **macOS**: 确保已安装 Xcode 命令行工具（`xcode-select --install`）
- **Linux**: 安装 `webkit2gtk` 开发包
- **Windows**: 安装 [WebView2 运行时](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

</details>

<details>
<summary><b>❌ 找不到 `trunk` 命令</b></summary>

**解决方案**:
```bash
cargo install trunk
```

如果仍然找不到，请确保 `~/.cargo/bin` 在您的 PATH 中。

</details>

<details>
<summary><b>❌ 子模块未初始化</b></summary>

**解决方案**:
```bash
git submodule init
git submodule update --recursive
```

</details>

<details>
<summary><b>❌ 构建失败，显示 Rust 编译错误</b></summary>

**解决方案**:
```bash
# 更新 Rust 工具链
rustup update

# 清理并重新构建
cargo clean
cargo tauri build
```

</details>

### 获取帮助

如果你遇到此处未列出的问题：
1. 查看 [Issues](https://github.com/lf-wxp/konachan-tauri/issues) 页面
2. 打开一个新的 issue，附带详细的错误信息

---

## 🤝 贡献指南

欢迎贡献！你可以通过以下方式提供帮助：

1. **Fork** 本仓库
2. **创建** 功能分支（`git checkout -b feature/amazing-feature`）
3. **提交** 你的更改（`git commit -m 'Add some amazing feature'`）
4. **推送** 到分支（`git push origin feature/amazing-feature`）
5. **打开** 一个 Pull Request

### 开发规范

- 遵循现有的代码风格
- 编写有意义的提交信息
- 为新功能添加测试
- 根据需要更新文档

---

## 🔗 相关项目

| 项目 | 描述 | 链接 |
| ------- | ----------- | ---- |
| **konachan-yew** | 前端子模块 (Yew + WASM) | [GitHub](https://github.com/lf-wxp/konachan-yew) |
| **konachan-api** | Web 版本的后端 API 服务器 | [GitHub](https://github.com/lf-wxp/konachan-api) |
| **Konachan** | 本应用基于的图片板 | [网站](https://konachan.com/) |

---

## 📄 许可证

本项目采用 **MIT 许可证** 授权。详见 [LICENSE](./LICENSE) 文件。

---

## 🙏 致谢

- [Konachan](https://konachan.com/) 提供 API
- [Tauri](https://tauri.app/) 提供出色的桌面框架
- [Yew](https://yew.rs/) 提供 Rust WASM 框架
- 所有帮助过这个项目的贡献者

---

<p align="center">
  由 <a href="https://github.com/lf-wxp">lf-wxp</a> ❤️ 制作
</p>

<p align="center">
  如果你觉得这个项目有用，请给个 ⭐ Star！
</p>
