# Konachan Tauri

<p align="center">
  <img src="./screenshot.webp" alt="Konachan Tauri 截图" width="600" />
</p>

<p align="center">
  <a href="https://github.com/lf-wxp/konachan-tauri/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="许可证: MIT" />
  </a>
  <img src="https://img.shields.io/badge/Rust-1.94%2B-orange.svg" alt="Rust 1.94+" />
  <a href="https://tauri.app/">
    <img src="https://img.shields.io/badge/Tauri-2.x-24C8D8.svg" alt="Tauri 2.x" />
  </a>
  <a href="https://leptos.dev/">
    <img src="https://img.shields.io/badge/Leptos-0.8-ef3939.svg" alt="Leptos 0.8" />
  </a>
  <a href="https://github.com/lf-wxp/konachan-tauri/releases">
    <img src="https://img.shields.io/github/release/lf-wxp/konachan-tauri.svg" alt="最新版本" />
  </a>
  <a href="https://github.com/lf-wxp/konachan-tauri/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/lf-wxp/konachan-tauri/release.yml" alt="CI/CD" />
  </a>
</p>

<p align="center">
  一个使用 <b>Tauri 2</b> 和 <b>Leptos 0.8</b> 构建的精美桌面端 <a href="https://konachan.com/">Konachan</a> 图片浏览器，为 macOS、Windows 和 Linux 提供原生体验。
</p>

<p align="center">
  <a href="./README.md">English</a>
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
    - [启动桌面端](#启动桌面端)
    - [仅开发前端](#仅开发前端)
    - [特性开关](#特性开关)
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

- 🖼️ **灯箱舞台** — 全幅图片查看器，带胶片式缩略条、三档缩略条密度，支持滚轮 / 方向键 / 滑动切换
- 🔍 **标签搜索** — 基于标签的查询，支持 Shift + 点击叠加标签与分页结果
- 🌊 **瀑布流画廊** — 翻页加载时保持滚动位置的瀑布流布局
- 📥 **下载队列** — 下载由 Tauri 后端执行，带实时进度事件与逐项状态
- 🎞️ **动态壁纸** — 详情页的 Canvas 动态壁纸舞台
- 🌍 **国际化** — 中英文界面，跟随系统语言
- 🎭 **安全模式** — 可通过编译期 `safe` 特性固定，也可在运行时由 `config.js` 切换
- 🎨 **自适应主题** — 从当前图片中提取基色、主色与辅色
- 🖥️ **启动画面** — 独立的透明 Tauri 窗口，WASM 应用就绪后自动关闭
- 🚀 **跨平台** — 提供 macOS、Windows 和 Linux 原生安装包
- 🎯 **轻量级** — 得益于 Tauri 复用系统 WebView，体积小巧

---

## 📸 应用截图

![Konachan Tauri](./screenshot.webp)

---

## 🛠️ 技术栈

| 层级 | 技术 | 描述 |
| --------- | ------------------------------------------------------------------------- | ------------------------------------ |
| **前端** | [Leptos 0.8](https://leptos.dev/) + [Trunk](https://trunkrs.dev/) | Rust → WebAssembly UI 框架（CSR） |
| **后端** | [Tauri 2](https://tauri.app/) | 基于 Rust 的安全桌面框架 |
| **样式** | Trunk 编译的 SCSS | 静态样式表，无运行时样式注入 |
| **状态管理** | Leptos 信号（`RwSignal` / `Memo`） | 单一 `AppState` 上下文 + 纯函数 reducer |
| **WASM 绑定** | [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)、[web-sys](https://crates.io/crates/web-sys)、[gloo-net](https://github.com/rustwasm/gloo) | DOM 访问与 HTTP 传输 |
| **语言** | Rust 2024 edition（`rust-toolchain.toml` 固定 1.94） | 类型安全的全栈开发 |

---

## 🏗️ 架构设计

```
┌───────────────────────────────────────────┐
│            Tauri Shell (原生层)           │
│  ┌─────────────────────────────────────┐  │
│  │   WebView (前端 - Leptos WASM)      │  │
│  │  • 组件                             │  │
│  │  • AppState + 纯函数 reducer        │  │
│  │  • UI 交互                          │  │
│  └─────────────────────────────────────┘  │
│  ┌─────────────────────────────────────┐  │
│  │   Tauri 后端 (Rust 命令)            │  │
│  │  • get_post / Konachan API          │  │
│  │  • download_image + 进度事件        │  │
│  │  • close_splashscreen               │  │
│  │  • 对话框、文件系统、通知等插件     │  │
│  └─────────────────────────────────────┘  │
└───────────────────────────────────────────┘
```

桌面端构建会启用前端的 `tauri` 特性：API 请求与下载都通过 `invoke` 交给后端，而不是走 HTTP。同一份前端也可以作为普通 Web 应用对接 `konachan-api` 运行。

---

## 📦 环境要求

### 必需工具

- **[Rust](https://www.rust-lang.org/tools/install)** 1.94+ — 具体渠道由 `src/rust-toolchain.toml` 固定
- **[Trunk](https://trunkrs.dev/)** 0.21.14 — WASM 打包工具（自带编译 SCSS 的 Dart Sass）
- **`wasm32-unknown-unknown`** 编译目标
- **[Tauri CLI](https://tauri.app/start/prerequisites/)** 2.12+ 以及下述平台依赖

### 快速安装

```bash
# 安装 Rust（如果尚未安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Trunk
cargo install trunk@0.21.14

# 添加 WASM 编译目标
rustup target add wasm32-unknown-unknown

# 安装 Tauri CLI（需 2.12+；2.0.0-rc 版本会给 cargo 传 `--features tauri/native-tls`）
cargo install tauri-cli --version "^2" --locked
```

### 平台特定依赖

<details>
<summary><b>macOS</b></summary>

```bash
# 安装 Xcode 命令行工具
xcode-select --install
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

前端位于 `src/` 子模块（[konachan-leptos](https://github.com/lf-wxp/konachan-leptos)，SSH 地址），构建前必须先检出子模块。

### 方式一：克隆时包含子模块（推荐）

```bash
# 克隆仓库并同时检出前端子模块
git clone --recurse-submodules git@github.com:lf-wxp/konachan-tauri.git
cd konachan-tauri

# 拉取后端依赖
cargo fetch --manifest-path src-tauri/Cargo.toml
```

### 方式二：克隆后初始化子模块

```bash
# 克隆不包含子模块的仓库
git clone git@github.com:lf-wxp/konachan-tauri.git
cd konachan-tauri

# 初始化并更新子模块
git submodule update --init --recursive
```

> 💡 只有 HTTPS 访问权限时，可以在克隆时重写子模块地址：
> `git clone --recurse-submodules -c url."https://github.com/".insteadOf="git@github.com:" <仓库地址>`

### 更新子模块

```bash
# 将前端子模块更新到其默认分支的最新提交
git submodule update --remote src

# 然后在父仓库中提交新的 gitlink
git add src && git commit -m "chore: bump frontend submodule"
```

---

## 💻 开发指南

### 启动桌面端

```bash
cargo tauri dev
```

`tauri.conf.json` 已自动串联前端构建：

- `beforeDevCommand`：`trunk serve --features tauri --config src/Trunk.toml`（开发服务器端口 8888）
- `beforeBuildCommand`：`trunk build --release --features tauri --config src/Trunk.toml`
- `frontendDist`：`../src/dist`

修改前端代码由 Trunk 开发服务器热更新 WebView；修改 `src-tauri/` 则触发后端重新编译并重启窗口。

### 仅开发前端

子模块本身是一个可独立运行的 Web 前端，无需 Tauri 即可开发：

```bash
cd src

# 使用模拟数据，不需要后端
trunk serve --features fake

# 通过 HTTP 对接 localhost:8000 上的 konachan-api（`/api/` 会被代理）
trunk serve --features web
```

在 `src/` 内还可使用 `cargo make dev-fake`、`dev-web`、`dev-safe`、`format`、`clippy`、`check`、`test`。

### 特性开关

| 特性 | 作用 |
| ------- | --------------------------------------------------------------------------------------- |
| `fake`  | 使用 `static/mock/post.json` 模拟数据，不发起任何 API 请求 |
| `web`   | HTTP 传输（`gloo-net`），请求同源的 `/api/` |
| `safe`  | 安全模式的编译期默认值，运行时仍可被 `config.js` 覆盖 |
| `tauri` | 桌面传输：API 请求与下载均通过 Tauri 命令执行 |

每次构建只能选择一种传输方式：`web`/`safe` 走 HTTP，`tauri` 走后端命令。

### 构建生产版本

```bash
cargo tauri build
```

打包产物输出到 `src-tauri/target/release/bundle/`。

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
```

⚠️ WASM 前端与原生后端无法一步交叉编译，需要在对应平台上分别构建（`.github/workflows/release.yml` 就是这么做的）。

</details>

---

## ⌨️ 键盘快捷键

输入框获得焦点时，单键快捷键会被忽略。

| 快捷键 | 操作 |
| -------- | ------ |
| `/` | 聚焦标签搜索框 |
| `←` / `→` | 在灯箱中切换到上一张 / 下一张图片 |
| `D` | 把当前展示的图片加入下载队列 |
| `W` | 把当前展示的图片投放为动态壁纸 |
| `R` | 刷新当前结果集 |
| `Esc` | 关闭灯箱浮层 |
| `Shift` + 点击 | 把标签叠加到当前查询，而不是替换 |

---

## 📁 项目结构

```
konachan-tauri/
├── src/                          # 前端子模块 (konachan-leptos)
│   ├── src/
│   │   ├── components/           # Leptos 组件（stage、nav、search、setting 等）
│   │   ├── state/                # AppState 与 reducer（page、downloads、notices、theme、i18n）
│   │   ├── utils/                # DOM 工具、瀑布流布局、Canvas 动画、运行时配置
│   │   ├── api.rs                # 传输层（HTTP / Tauri / 模拟数据）
│   │   ├── app.rs                # 根组件（provider + 布局）
│   │   ├── model.rs              # 数据模型与类型
│   │   └── main.rs               # 前端入口
│   ├── static/                   # SCSS、字体、图片、模拟数据、启动页、脚本
│   ├── index.html                # Trunk 资源清单
│   ├── Cargo.toml                # 前端依赖与特性
│   ├── Trunk.toml                # 开发服务器（8888）与 `/api/` 代理
│   └── rust-toolchain.toml       # 固定工具链（1.94）与 wasm 目标
│
├── src-tauri/                    # Tauri 后端
│   ├── src/
│   │   ├── main.rs               # 插件与应用初始化
│   │   ├── lib.rs                # 命令注册
│   │   ├── commander.rs          # get_post / download_image / close_splashscreen
│   │   └── image.rs              # Konachan API 客户端、解析、下载与进度
│   ├── capabilities/             # Tauri 权限能力
│   ├── icons/                    # 应用图标（png、ico、icns）
│   ├── Cargo.toml                # 后端依赖
│   └── tauri.conf.json           # Tauri 配置
│
├── .github/
│   └── workflows/
│       └── release.yml           # CI/CD 发布工作流
│
├── LICENSE                       # MIT 许可证
├── README.md                     # 英文文档
├── README.zh-CN.md               # 中文文档
└── .gitmodules                   # Git 子模块配置
```

---

## 🔧 故障排除

### 常见问题

<details>
<summary><b>❌ `cargo tauri dev` 失败，显示 WebView 错误</b></summary>

**解决方案**:
- **macOS**：确保已安装 Xcode 命令行工具（`xcode-select --install`）
- **Linux**：安装 `webkit2gtk` 开发包
- **Windows**：安装 [WebView2 运行时](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

</details>

<details>
<summary><b>❌ 找不到 `trunk` 命令</b></summary>

**解决方案**:
```bash
cargo install trunk@0.21.14
```

如果仍然找不到，请确保 `~/.cargo/bin` 在 `PATH` 中。

</details>

<details>
<summary><b>❌ 子模块未初始化 / `src` 目录为空</b></summary>

**解决方案**:
```bash
git submodule update --init --recursive
```

前端迁移到 `konachan-leptos` 之前克隆的仓库，`.git/config` 里仍是旧地址，`update` 会报 `repository not found`。先按 `.gitmodules` 重新同步地址：

```bash
git submodule sync --recursive
git submodule update --init --recursive
```

</details>

<details>
<summary><b>❌ 拉取子模块时报 `Permission denied (publickey)`</b></summary>

子模块使用 SSH 地址（`git@github.com:lf-wxp/konachan-leptos.git`）。

**解决方案**:
```bash
# 确认 GitHub SSH 密钥可用
ssh -T git@github.com

# 或者在单次命令中重写地址
git -c url."https://github.com/".insteadOf="git@github.com:" submodule update --init --recursive
```

</details>

<details>
<summary><b>❌ 构建失败，显示 Rust 编译错误</b></summary>

**解决方案**:
```bash
# 更新工具链（项目通过 rust-toolchain.toml 固定 1.94）
rustup update

# 清理并重新构建
cargo clean
cargo tauri build
```

</details>

<details>
<summary><b>❌ `cargo tauri dev` 报 `failed to select a version for schannel`</b></summary>

`tauri-cli` 2.0.0-rc.\* 会以 `--features tauri/native-tls` 构建后端，从而引入 `native-tls` → `schannel ^0.1.28`，与锁定的依赖版本无法兼容。这不是项目本身的问题。

**解决方案**:
```bash
# 升级到 2.x 稳定版 CLI，它会改用 `cargo run --no-default-features`
cargo install tauri-cli --version "^2" --locked
cargo tauri --version
```

也可以改用 npm 版 CLI：`bunx @tauri-apps/cli@2 dev` / `npx @tauri-apps/cli@2 dev`。

</details>

<details>
<summary><b>❌ `cargo tauri dev` 卡在 `Waiting for your frontend dev server to start on http://localhost:8888/`</b></summary>

8888 端口已被占用，通常是上一次会话遗留的 `trunk serve` 进程。

**解决方案**:
```bash
lsof -nP -i :8888   # 找到 PID
kill <pid>
```

</details>

<details>
<summary><b>❌ 链接阶段内存不足</b></summary>

前端 release 配置使用 `lto = "fat"` 与单 codegen unit，这是链接阶段的内存峰值点。资源受限的构建机可在实际执行链接的命令上覆盖它：

```bash
# 在仓库根目录执行
CARGO_PROFILE_RELEASE_LTO=off trunk build --release --features tauri --config src/Trunk.toml
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

前端代码位于 [konachan-leptos](https://github.com/lf-wxp/konachan-leptos) 仓库（即 `src/` 子模块），前端相关的 PR 请提到该仓库。

### 开发规范

- 遵循现有的代码风格（仓库已配置 `rustfmt` 与 `clippy`）
- 编写有意义的提交信息
- 为新功能添加测试
- 根据需要更新文档

---

## 🔗 相关项目

| 项目 | 描述 | 链接 |
| ------- | ----------- | ---- |
| **konachan-leptos** | 前端子模块 (Leptos + WASM) | [GitHub](https://github.com/lf-wxp/konachan-leptos) |
| **konachan-api** | Web 版本的后端 API 服务器 | [GitHub](https://github.com/lf-wxp/konachan-api) |
| **Konachan** | 本应用基于的图片板 | [网站](https://konachan.com/) |

---

## 📄 许可证

本项目采用 **MIT 许可证** 授权。详见 [LICENSE](./LICENSE) 文件。

---

## 🙏 致谢

- [Konachan](https://konachan.com/) 提供 API
- [Tauri](https://tauri.app/) 提供出色的桌面框架
- [Leptos](https://leptos.dev/) 提供 Rust WASM 框架
- 所有帮助过这个项目的贡献者

---

<p align="center">
  由 <a href="https://github.com/lf-wxp">lf-wxp</a> ❤️ 制作
</p>

<p align="center">
  如果你觉得这个项目有用，请给个 ⭐ Star！
</p>
